use std::{mem, sync::mpsc::{self, sync_channel}, thread, time::Duration};

use windows::Win32::UI::Input::{self, KeyboardAndMouse::{SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP}};

use crate::{Key, Lazy, MAGIC_CONST};

static INPUT_TX: Lazy<mpsc::SyncSender<InputRequest>> = Lazy::new(|| {
    let (tx, rx) = sync_channel(48);

    thread::spawn(move || input_sender(rx));
    tx
});

pub struct CharSender;

struct InputRequest {
    key: Key,
    release: bool,
    extra: usize,
}

const KEY_MAP: &[(char, &[Key])] = &[
    // English letters
    ('a', &[Key::AKey]),
    ('b', &[Key::BKey]),
    ('c', &[Key::CKey]),
    ('d', &[Key::DKey]),
    ('e', &[Key::EKey]),
    ('f', &[Key::FKey]),
    ('g', &[Key::GKey]),
    ('h', &[Key::HKey]),
    ('i', &[Key::IKey]),
    ('j', &[Key::JKey]),
    ('k', &[Key::KKey]),
    ('l', &[Key::LKey]),
    ('m', &[Key::MKey]),
    ('n', &[Key::NKey]),
    ('o', &[Key::OKey]),
    ('p', &[Key::PKey]),
    ('q', &[Key::QKey]),
    ('r', &[Key::RKey]),
    ('s', &[Key::SKey]),
    ('t', &[Key::TKey]),
    ('u', &[Key::UKey]),
    ('v', &[Key::VKey]),
    ('w', &[Key::WKey]),
    ('x', &[Key::XKey]),
    ('y', &[Key::YKey]),
    ('z', &[Key::ZKey]),

    // Numbers
    ('0', &[Key::Numrow0Key]),
    ('1', &[Key::Numrow1Key]),
    ('2', &[Key::Numrow2Key]),
    ('3', &[Key::Numrow3Key]),
    ('4', &[Key::Numrow4Key]),
    ('5', &[Key::Numrow5Key]),
    ('6', &[Key::Numrow6Key]),
    ('7', &[Key::Numrow7Key]),
    ('8', &[Key::Numrow8Key]),
    ('9', &[Key::Numrow9Key]),
    
    // Polish letters
    ('ą', &[Key::RAltKey, Key::AKey]),
    ('ć', &[Key::RAltKey, Key::CKey]),
    ('ę', &[Key::RAltKey, Key::EKey]),
    ('ł', &[Key::RAltKey, Key::LKey]),
    ('ń', &[Key::RAltKey, Key::NKey]),
    ('ó', &[Key::RAltKey, Key::OKey]),
    ('ś', &[Key::RAltKey, Key::SKey]),
    ('ż', &[Key::RAltKey, Key::ZKey]),
    ('ź', &[Key::RAltKey, Key::XKey]),

    // Symbols
    ('-', &[Key::MinusKey]),
    ('=', &[Key::EqualKey]),
    ('[', &[Key::LBracketKey]),
    (']', &[Key::RBracketKey]),
    (';', &[Key::SemicolonKey]),
    ('\'', &[Key::QuoteKey]),
    (',', &[Key::CommaKey]),
    ('.', &[Key::PeriodKey]),
    ('/', &[Key::SlashKey]),
    ('\\', &[Key::BackslashKey]),
    ('`', &[Key::BackquoteKey]),

    ('!', &[Key::LShiftKey, Key::Numrow1Key]),
    ('@', &[Key::LShiftKey, Key::Numrow2Key]),
    ('#', &[Key::LShiftKey, Key::Numrow3Key]),
    ('$', &[Key::LShiftKey, Key::Numrow4Key]),
    ('%', &[Key::LShiftKey, Key::Numrow5Key]),
    ('^', &[Key::LShiftKey, Key::Numrow6Key]),
    ('&', &[Key::LShiftKey, Key::Numrow7Key]),
    ('*', &[Key::LShiftKey, Key::Numrow8Key]),
    ('(', &[Key::LShiftKey, Key::Numrow9Key]),
    (')', &[Key::LShiftKey, Key::Numrow0Key]),
    ('_', &[Key::LShiftKey, Key::MinusKey]),
    ('+', &[Key::LShiftKey, Key::EqualKey]),
    ('{', &[Key::LShiftKey, Key::LBracketKey]),
    ('}', &[Key::LShiftKey, Key::RBracketKey]),
    (':', &[Key::LShiftKey, Key::SemicolonKey]),
    ('"', &[Key::LShiftKey, Key::QuoteKey]),
    ('<', &[Key::LShiftKey, Key::CommaKey]),
    ('>', &[Key::LShiftKey, Key::PeriodKey]),
    ('?', &[Key::LShiftKey, Key::SlashKey]),
    ('|', &[Key::LShiftKey, Key::BackslashKey]),

    // white spaces
    (' ', &[Key::SpaceKey]),
    ('\n', &[Key::EnterKey]),
    ('\t', &[Key::TabKey]),
    
    // ignored due to need to follow with another key when keyboard is set to "Polish (programmer)"
    // ('~', &[Key::LShiftKey, Key::BackquoteKey]),
    ('~', &[]),
];

impl CharSender {
    pub fn send_char(chr: char, ignore_handler: bool) {
        const DELAY: Duration = Duration::from_millis(20);

        let mut keystrokes = Vec::new();
        let lower_case = chr.to_lowercase().next().unwrap();

        if chr != lower_case {
            keystrokes.push(Key::LShiftKey);
        }

        let option = KEY_MAP.iter()
            .find(|(c,_)| c == &lower_case);

        if let Some((_, mapping)) = option {
            keystrokes.extend_from_slice(mapping);

            keystrokes.iter().for_each(|key| Self::press_key(*key, ignore_handler));

            thread::sleep(DELAY);

            keystrokes.iter().for_each(|key| Self::release_key(*key, ignore_handler));
        }
    }

    pub fn press_key(key: Key, ignore_handler: bool) {
        let extra = if ignore_handler { MAGIC_CONST } else { 0 };

        let _ = INPUT_TX.send(InputRequest { key, release: false, extra });
    }

    pub fn release_key(key: Key, ignore_handler: bool) {
        let extra = if ignore_handler { MAGIC_CONST } else { 0 };

        let _ = INPUT_TX.send(InputRequest { key, release: true, extra });
    }
}

fn input_sender(rx: mpsc::Receiver<InputRequest>) {
    while let Ok(input_request) = rx.recv() {
        let flags = if input_request.release {
            KEYEVENTF_KEYUP
        } else {
            KEYBD_EVENT_FLAGS(0)
        };

        let keybd = KEYBDINPUT {
            wVk: input_request.key.to_vk(),
            wScan: 0,
            dwFlags: flags,
            time: 0,
            dwExtraInfo: input_request.extra,
        };

        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: Input::KeyboardAndMouse::INPUT_0 {
                ki: keybd,
            },
        };

        unsafe {
            SendInput(&[input], mem::size_of::<INPUT>() as _);
        }
    }
}
