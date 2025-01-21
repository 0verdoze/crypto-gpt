use std::{mem, slice, sync::mpsc, thread};

use log::Level;
use tray_icon::Icon;

/// wrapper aroung env_logger
///
/// all calls are redirected as is to env_logger
///
/// in addition to that a copy of log message is available on tray
/// icon color indicating message type
pub struct TrayLogger {
    env_log: env_logger::Logger,
    tray: mpsc::SyncSender<(Option<String>, Option<Icon>)>,
}

impl TrayLogger {
    pub fn new(env_log: env_logger::Logger) -> Self {
        // setup separate thread to handle icon
        let (tx, rx) = mpsc::sync_channel(8);
        thread::spawn(move || Self::worker(rx));

        Self { env_log, tray: tx }
    }

    fn level_to_rgba(level: Level) -> [u8; 4] {
        match level {
            Level::Error => [255, 0, 0, 255],
            Level::Warn => [255, 0xAE, 0x42, 255],
            Level::Info => [0, 255, 0, 255],
            Level::Debug => [105, 105, 105, 255],
            Level::Trace => [255, 255, 255, 255],
        }
    }

    pub fn set_icon(&self, color: [u8; 4]) {
        let icon = Self::gen_icon(color);

        let _result = self.tray.try_send((None, Some(icon)));
    }

    // pub fn set_tooltip(&self, s: String) {
    //     let _result = self.tray.send((Some(s), None));
    // }

    fn worker(rx: mpsc::Receiver<(Option<String>, Option<Icon>)>) {
        let tray = tray_icon::TrayIconBuilder::new()
            .with_title("Magic helper")
            .with_icon(Self::gen_icon([200, 200, 200, 255]))
            .with_tooltip("Initialization OK")
            .build()
            .unwrap();

        while let Ok((msg, icon)) = rx.recv() {
            if let Some(msg) = msg {
                let _result = tray.set_tooltip(Some(msg));
            }

            if let Some(icon) = icon {
                let _result = tray.set_icon(Some(icon));
            }
        }
    }

    fn gen_icon(color: [u8; 4]) -> Icon {
        const X_DIMMENSION: usize = 32;
        const Y_DIMMENSION: usize = 32;

        let mut img = [0u32; X_DIMMENSION * Y_DIMMENSION];

        let center_x = 16;
        let center_y = 16;
        let radius = 15;

        for y in 0..Y_DIMMENSION {
            for x in 0..X_DIMMENSION {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                let distance_squared = dx * dx + dy * dy;

                if distance_squared <= radius * radius {
                    *Self::as_bytes_u32_mut(&mut img[y * Y_DIMMENSION + x]) = color;
                }
            }
        }

        let rgba = Self::as_bytes_u32_slice(&img).to_vec();
        Icon::from_rgba(rgba, X_DIMMENSION as u32, Y_DIMMENSION as u32).unwrap()
    }

    fn as_bytes_u32_mut<'a>(n: &'a mut u32) -> &'a mut [u8; 4] {
        unsafe { mem::transmute::<&'a mut u32, &'a mut [u8; 4]>(n) }
    }

    fn as_bytes_u32_slice(b: &[u32]) -> &[u8] {
        unsafe { slice::from_raw_parts(b.as_ptr() as *const u8, b.len() * 4) }
    }
}

impl log::Log for TrayLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let color = Self::level_to_rgba(record.level());

        let msg = format!("{}", record.args());
        let icon = Self::gen_icon(color);

        let _result = self.tray.send((Some(msg), Some(icon)));

        self.env_log.log(record)
    }

    fn flush(&self) {
        self.env_log.flush()
    }
}
