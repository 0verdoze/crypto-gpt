
use std::{mem, thread};

use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::Foundation::*;

use crate::{Key, Lazy, Mutex};

pub type HotkeyCallback = Box<dyn FnMut(&[Key]) -> bool + Send>;
pub type KeypressCallback = Box<dyn FnMut(Key, bool) -> bool + Send>;

static HANDLER_CTX: Lazy<Mutex<HandlerCtx>> = Lazy::new(|| {
    // initalize thread
    thread::spawn(worker);

    Default::default()
});

pub(crate) const MAGIC_CONST: usize = 0x8007f00000000000;

#[derive(Default)]
struct HandlerCtx {
    handlers: Vec<(Vec<Key>, HotkeyCallback)>,
    keypressed_callback: Option<KeypressCallback>,
}

pub struct KeypressHandler;

unsafe extern "system" fn windows_callback(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let info: &mut KBDLLHOOKSTRUCT = unsafe {
        if (code as u32 != HC_ACTION) | (lparam.0 == 0) {
            return CallNextHookEx(None, code, wparam, lparam);
        }

        // using transmute here is UB, as of rust 1.78 it fails
        // https://github.com/rust-lang/rust/pull/121282#issuecomment-2129895020
        &mut *(lparam.0 as *mut KBDLLHOOKSTRUCT)
    };

    if info.dwExtraInfo == MAGIC_CONST {
        return CallNextHookEx(None, code, wparam, lparam);
    }

    match wparam.0 as u32 {
        WM_KEYDOWN => {
            if let Ok(vk_code) = info.vkCode.try_into() {
                let pressed_key = Key::from_vk(VIRTUAL_KEY(vk_code));
    
                let mut ignore_next_hook = false;
                let mut was_hook_called = false;
                let mut guard = HANDLER_CTX.lock();

                for (keys, handler) in guard.handlers.iter_mut() {
                    if keys.last().map(|key| key == &pressed_key).unwrap_or_default() {
                        let call = keys[..keys.len() - 1].iter()
                            .all(Key::is_pressed);

                        if call {
                            ignore_next_hook |= (handler)(&keys);
                            was_hook_called = true;
                        }
                    }
                }

                if let Some(handler) = guard.keypressed_callback.as_mut() {
                    ignore_next_hook |= (handler)(pressed_key, was_hook_called);
                }

                if ignore_next_hook {
                    return LRESULT(1);
                }
            }
        },
        _ => {
            // ignore...
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

fn worker() {
    // setup hook
    let keyboard_hook = unsafe {
        SetWindowsHookExW(WH_KEYBOARD_LL, Some(windows_callback), None, 0)
    };

    if keyboard_hook.is_err() {
        // TODO: error reporting
        return;
    }

    unsafe {
        let mut msg = MSG::default();

        loop {
            let result = GetMessageW(&mut msg, None, 0, 0);

            match result {
                BOOL(-1) => {
                    // err
                    log::error!("{}", windows::core::Error::from_win32());
                },
                BOOL(0) => {
                    // WM_QUIT
                    break;
                },
                BOOL(_) => {
                    // real event
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }

    unsafe {
        let _result = UnhookWindowsHookEx(keyboard_hook.unwrap());
    }
}

impl KeypressHandler {
    pub fn init() {
        Lazy::force(&HANDLER_CTX);
    }

    pub fn add_hotkey<F: FnMut(&[Key]) -> bool + Send + 'static>(hotkey: &[Key], callback: F) {
        if hotkey.len() > 0 {
            HANDLER_CTX.lock()
                .handlers
                .push((hotkey.to_vec(), Box::new(callback)))
        }
    }

    pub fn set_keypress_callback<F: FnMut(Key, bool) -> bool + Send + 'static>(callback: F) {
        HANDLER_CTX.lock()
            .keypressed_callback = Some(Box::new(callback));
    }

    pub fn clear_keypress_callback() {
        HANDLER_CTX.lock()
            .keypressed_callback = None;
    }
}
