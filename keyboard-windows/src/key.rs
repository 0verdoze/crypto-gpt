use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    BackspaceKey,
    TabKey,
    EnterKey,
    EscapeKey,
    SpaceKey,
    PageUpKey,
    PageDownKey,
    EndKey,
    HomeKey,
    LeftKey,
    UpKey,
    RightKey,
    DownKey,
    InsertKey,
    DeleteKey,
    Numrow0Key,
    Numrow1Key,
    Numrow2Key,
    Numrow3Key,
    Numrow4Key,
    Numrow5Key,
    Numrow6Key,
    Numrow7Key,
    Numrow8Key,
    Numrow9Key,
    AKey,
    BKey,
    CKey,
    DKey,
    EKey,
    FKey,
    GKey,
    HKey,
    IKey,
    JKey,
    KKey,
    LKey,
    MKey,
    NKey,
    OKey,
    PKey,
    QKey,
    RKey,
    SKey,
    TKey,
    UKey,
    VKey,
    WKey,
    XKey,
    YKey,
    ZKey,
    LSuper,
    RSuper,
    Numpad0Key,
    Numpad1Key,
    Numpad2Key,
    Numpad3Key,
    Numpad4Key,
    Numpad5Key,
    Numpad6Key,
    Numpad7Key,
    Numpad8Key,
    Numpad9Key,
    F1Key,
    F2Key,
    F3Key,
    F4Key,
    F5Key,
    F6Key,
    F7Key,
    F8Key,
    F9Key,
    F10Key,
    F11Key,
    F12Key,
    F13Key,
    F14Key,
    F15Key,
    F16Key,
    F17Key,
    F18Key,
    F19Key,
    F20Key,
    F21Key,
    F22Key,
    F23Key,
    F24Key,
    NumLockKey,
    ScrollLockKey,
    CapsLockKey,
    LShiftKey,
    RShiftKey,
    LControlKey,
    RControlKey,
    LAltKey,
    RAltKey,
    BrowserBackKey,
    BrowserForwardKey,
    BrowserRefreshKey,
    VolumeMuteKey,
    VolumeDownKey,
    VolumeUpKey,
    MediaNextTrackKey,
    MediaPrevTrackKey,
    MediaStopKey,
    MediaPlayPauseKey,
    BackquoteKey,
    SlashKey,
    BackslashKey,
    CommaKey,
    PeriodKey,
    MinusKey,
    QuoteKey,
    SemicolonKey,
    LBracketKey,
    RBracketKey,
    EqualKey,
    OtherKey(u16),
}

impl Key {
    pub fn to_vk(self) -> VIRTUAL_KEY {
        let raw_vk = match self {
            Self::BackspaceKey => 0x08,
            Self::TabKey => 0x09,
            Self::EnterKey => 0x0D,
            Self::EscapeKey => 0x1B,
            Self::SpaceKey => 0x20,
            Self::PageUpKey => 0x21,
            Self::PageDownKey => 0x22,
            Self::EndKey => 0x23,
            Self::HomeKey => 0x24,
            Self::LeftKey => 0x25,
            Self::UpKey => 0x26,
            Self::RightKey => 0x27,
            Self::DownKey => 0x28,
            Self::InsertKey => 0x2D,
            Self::DeleteKey => 0x2E,
            Self::Numrow0Key => 0x30,
            Self::Numrow1Key => 0x31,
            Self::Numrow2Key => 0x32,
            Self::Numrow3Key => 0x33,
            Self::Numrow4Key => 0x34,
            Self::Numrow5Key => 0x35,
            Self::Numrow6Key => 0x36,
            Self::Numrow7Key => 0x37,
            Self::Numrow8Key => 0x38,
            Self::Numrow9Key => 0x39,
            Self::AKey => 0x41,
            Self::BKey => 0x42,
            Self::CKey => 0x43,
            Self::DKey => 0x44,
            Self::EKey => 0x45,
            Self::FKey => 0x46,
            Self::GKey => 0x47,
            Self::HKey => 0x48,
            Self::IKey => 0x49,
            Self::JKey => 0x4A,
            Self::KKey => 0x4B,
            Self::LKey => 0x4C,
            Self::MKey => 0x4D,
            Self::NKey => 0x4E,
            Self::OKey => 0x4F,
            Self::PKey => 0x50,
            Self::QKey => 0x51,
            Self::RKey => 0x52,
            Self::SKey => 0x53,
            Self::TKey => 0x54,
            Self::UKey => 0x55,
            Self::VKey => 0x56,
            Self::WKey => 0x57,
            Self::XKey => 0x58,
            Self::YKey => 0x59,
            Self::ZKey => 0x5A,
            Self::LSuper => 0x5B,
            Self::RSuper => 0x5C,
            Self::Numpad0Key => 0x60,
            Self::Numpad1Key => 0x61,
            Self::Numpad2Key => 0x62,
            Self::Numpad3Key => 0x63,
            Self::Numpad4Key => 0x64,
            Self::Numpad5Key => 0x65,
            Self::Numpad6Key => 0x66,
            Self::Numpad7Key => 0x67,
            Self::Numpad8Key => 0x68,
            Self::Numpad9Key => 0x69,
            Self::F1Key => 0x70,
            Self::F2Key => 0x71,
            Self::F3Key => 0x72,
            Self::F4Key => 0x73,
            Self::F5Key => 0x74,
            Self::F6Key => 0x75,
            Self::F7Key => 0x76,
            Self::F8Key => 0x77,
            Self::F9Key => 0x78,
            Self::F10Key => 0x79,
            Self::F11Key => 0x7A,
            Self::F12Key => 0x7B,
            Self::F13Key => 0x7C,
            Self::F14Key => 0x7D,
            Self::F15Key => 0x7E,
            Self::F16Key => 0x7F,
            Self::F17Key => 0x80,
            Self::F18Key => 0x81,
            Self::F19Key => 0x82,
            Self::F20Key => 0x83,
            Self::F21Key => 0x84,
            Self::F22Key => 0x85,
            Self::F23Key => 0x86,
            Self::F24Key => 0x87,
            Self::NumLockKey => 0x90,
            Self::ScrollLockKey => 0x91,
            Self::CapsLockKey => 0x14,
            Self::LShiftKey => 0xA0,
            Self::RShiftKey => 0xA1,
            Self::LControlKey => 0xA2,
            Self::RControlKey => 0xA3,
            Self::LAltKey => 0xA4,
            Self::RAltKey => 0xA5,
            Self::BrowserBackKey => 0xA6,
            Self::BrowserForwardKey => 0xA7,
            Self::BrowserRefreshKey => 0xA8,
            Self::VolumeMuteKey => 0xAD,
            Self::VolumeDownKey => 0xAE,
            Self::VolumeUpKey => 0xAF,
            Self::MediaNextTrackKey => 0xB0,
            Self::MediaPrevTrackKey => 0xB1,
            Self::MediaStopKey => 0xB2,
            Self::MediaPlayPauseKey => 0xB3,
            Self::BackquoteKey => 0xC0,
            Self::SlashKey => 0xBF,
            Self::BackslashKey => 0xDC,
            Self::CommaKey => 0xBC,
            Self::PeriodKey => 0xBE,
            Self::MinusKey => 0xBD,
            Self::QuoteKey => 0xDE,
            Self::SemicolonKey => 0xBA,
            Self::LBracketKey => 0xDB,
            Self::RBracketKey => 0xDD,
            Self::EqualKey => 0xBB,
            Self::OtherKey(code) => code,
        };

        VIRTUAL_KEY(raw_vk)
    }

    pub fn from_vk(vk: VIRTUAL_KEY) -> Self {
        match vk.0 {
            0x08 => Self::BackspaceKey,
            0x09 => Self::TabKey,
            0x0D => Self::EnterKey,
            0x1B => Self::EscapeKey,
            0x20 => Self::SpaceKey,
            0x21 => Self::PageUpKey,
            0x22 => Self::PageDownKey,
            0x23 => Self::EndKey,
            0x24 => Self::HomeKey,
            0x25 => Self::LeftKey,
            0x26 => Self::UpKey,
            0x27 => Self::RightKey,
            0x28 => Self::DownKey,
            0x2D => Self::InsertKey,
            0x2E => Self::DeleteKey,
            0x30 => Self::Numrow0Key,
            0x31 => Self::Numrow1Key,
            0x32 => Self::Numrow2Key,
            0x33 => Self::Numrow3Key,
            0x34 => Self::Numrow4Key,
            0x35 => Self::Numrow5Key,
            0x36 => Self::Numrow6Key,
            0x37 => Self::Numrow7Key,
            0x38 => Self::Numrow8Key,
            0x39 => Self::Numrow9Key,
            0x41 => Self::AKey,
            0x42 => Self::BKey,
            0x43 => Self::CKey,
            0x44 => Self::DKey,
            0x45 => Self::EKey,
            0x46 => Self::FKey,
            0x47 => Self::GKey,
            0x48 => Self::HKey,
            0x49 => Self::IKey,
            0x4A => Self::JKey,
            0x4B => Self::KKey,
            0x4C => Self::LKey,
            0x4D => Self::MKey,
            0x4E => Self::NKey,
            0x4F => Self::OKey,
            0x50 => Self::PKey,
            0x51 => Self::QKey,
            0x52 => Self::RKey,
            0x53 => Self::SKey,
            0x54 => Self::TKey,
            0x55 => Self::UKey,
            0x56 => Self::VKey,
            0x57 => Self::WKey,
            0x58 => Self::XKey,
            0x59 => Self::YKey,
            0x5A => Self::ZKey,
            0x5B => Self::LSuper,
            0x5C => Self::RSuper,
            0x60 => Self::Numpad0Key,
            0x61 => Self::Numpad1Key,
            0x62 => Self::Numpad2Key,
            0x63 => Self::Numpad3Key,
            0x64 => Self::Numpad4Key,
            0x65 => Self::Numpad5Key,
            0x66 => Self::Numpad6Key,
            0x67 => Self::Numpad7Key,
            0x68 => Self::Numpad8Key,
            0x69 => Self::Numpad9Key,
            0x70 => Self::F1Key,
            0x71 => Self::F2Key,
            0x72 => Self::F3Key,
            0x73 => Self::F4Key,
            0x74 => Self::F5Key,
            0x75 => Self::F6Key,
            0x76 => Self::F7Key,
            0x77 => Self::F8Key,
            0x78 => Self::F9Key,
            0x79 => Self::F10Key,
            0x7A => Self::F11Key,
            0x7B => Self::F12Key,
            0x7C => Self::F13Key,
            0x7D => Self::F14Key,
            0x7E => Self::F15Key,
            0x7F => Self::F16Key,
            0x80 => Self::F17Key,
            0x81 => Self::F18Key,
            0x82 => Self::F19Key,
            0x83 => Self::F20Key,
            0x84 => Self::F21Key,
            0x85 => Self::F22Key,
            0x86 => Self::F23Key,
            0x87 => Self::F24Key,
            0x90 => Self::NumLockKey,
            0x91 => Self::ScrollLockKey,
            0x14 => Self::CapsLockKey,
            0xA0 => Self::LShiftKey,
            0xA1 => Self::RShiftKey,
            0xA2 => Self::LControlKey,
            0xA3 => Self::RControlKey,
            0xA4 => Self::LAltKey,
            0xA5 => Self::RAltKey,
            0xA6 => Self::BrowserBackKey,
            0xA7 => Self::BrowserForwardKey,
            0xA8 => Self::BrowserRefreshKey,
            0xAD => Self::VolumeMuteKey,
            0xAE => Self::VolumeDownKey,
            0xAF => Self::VolumeUpKey,
            0xB0 => Self::MediaNextTrackKey,
            0xB1 => Self::MediaPrevTrackKey,
            0xB2 => Self::MediaStopKey,
            0xB3 => Self::MediaPlayPauseKey,
            0xC0 => Self::BackquoteKey,
            0xBF => Self::SlashKey,
            0xDC => Self::BackslashKey,
            0xBC => Self::CommaKey,
            0xBE => Self::PeriodKey,
            0xBD => Self::MinusKey,
            0xDE => Self::QuoteKey,
            0xBA => Self::SemicolonKey,
            0xDB => Self::LBracketKey,
            0xDD => Self::RBracketKey,
            0xBB => Self::EqualKey,
            other => Self::OtherKey(other),
        }
    }

    pub fn is_numeric(&self) -> bool {
        [
            Key::Numrow0Key,
            Key::Numrow1Key,
            Key::Numrow2Key,
            Key::Numrow3Key,
            Key::Numrow4Key,
            Key::Numrow5Key,
            Key::Numrow6Key,
            Key::Numrow7Key,
            Key::Numrow8Key,
            Key::Numrow9Key,
            Key::Numpad0Key,
            Key::Numpad1Key,
            Key::Numpad2Key,
            Key::Numpad3Key,
            Key::Numpad4Key,
            Key::Numpad5Key,
            Key::Numpad6Key,
            Key::Numpad7Key,
            Key::Numpad8Key,
            Key::Numpad9Key,
        ]
        .contains(self)
    }

    pub fn is_alphabetic(&self) -> bool {
        [
            Key::AKey,
            Key::BKey,
            Key::CKey,
            Key::DKey,
            Key::EKey,
            Key::FKey,
            Key::GKey,
            Key::HKey,
            Key::IKey,
            Key::JKey,
            Key::KKey,
            Key::LKey,
            Key::MKey,
            Key::NKey,
            Key::OKey,
            Key::PKey,
            Key::QKey,
            Key::RKey,
            Key::SKey,
            Key::TKey,
            Key::UKey,
            Key::VKey,
            Key::WKey,
            Key::XKey,
            Key::YKey,
            Key::ZKey,
        ]
        .contains(self)
    }

    pub fn is_symbol(&self) -> bool {
        [
            Key::SpaceKey,
            Key::BackquoteKey,
            Key::SlashKey,
            Key::BackslashKey,
            Key::CommaKey,
            Key::PeriodKey,
            Key::MinusKey,
            Key::QuoteKey,
            Key::SemicolonKey,
            Key::LBracketKey,
            Key::RBracketKey,
            Key::EqualKey,
        ]
        .contains(self)
    }

    pub fn is_alphanumeric(&self) -> bool {
        self.is_alphabetic() || self.is_numeric()
    }

    pub fn is_alphanumeric_symbol(&self) -> bool {
        self.is_alphanumeric() || self.is_symbol()
    }

    pub fn is_pressed(&self) -> bool {
        (unsafe { GetAsyncKeyState(self.to_vk().0 as i32) } >> 15) != 0
    }
}
