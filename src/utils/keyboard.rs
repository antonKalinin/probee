use std::convert::TryFrom;
use std::fmt;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    A = 0,
    B = 11,
    C = 8,
    D = 2,
    E = 14,
    F = 3,
    G = 5,
    H = 4,
    I = 34,
    J = 38,
    K = 40,
    L = 37,
    M = 46,
    N = 45,
    O = 31,
    P = 35,
    Q = 12,
    R = 15,
    S = 1,
    T = 17,
    U = 32,
    V = 9,
    W = 13,
    X = 7,
    Y = 16,
    Z = 6,

    // Numbers
    Zero = 29,
    One = 18,
    Two = 19,
    Three = 20,
    Four = 21,
    Five = 23,
    Six = 22,
    Seven = 26,
    Eight = 28,
    Nine = 25,

    // Symbols & Punctuation
    Equal = 24,
    Minus = 27,
    Backtick = 50,
    Backslash = 42,
    Comma = 43,
    Period = 47,
    Slash = 44,
    Semicolon = 41,
    Quote = 39,
    LeftBracket = 33,
    RightBracket = 30,

    // Modifiers
    Command = 55,
    Shift = 56,
    CapsLock = 57,
    Option = 58,
    Control = 59,
    RightShift = 60,
    RightOption = 61,
    RightControl = 62,
    Function = 63,

    // Navigation & Editing
    Return = 36,
    Tab = 48,
    Space = 49,
    Delete = 51,
    Escape = 53,
    ForwardDelete = 117,
    Home = 115,
    End = 119,
    PageUp = 116,
    PageDown = 121,
    Help = 114,

    // Arrow Keys
    LeftArrow = 123,
    RightArrow = 124,
    DownArrow = 125,
    UpArrow = 126,

    // Function Keys
    F1 = 122,
    F2 = 120,
    F3 = 99,
    F4 = 118,
    F5 = 96,
    F6 = 97,
    F7 = 98,
    F8 = 100,
    F9 = 101,
    F10 = 109,
    F11 = 103,
    F12 = 111,
    F13 = 105,
    F14 = 107,
    F15 = 113,
    F16 = 106,
    F17 = 64,
    F18 = 79,
    F19 = 80,
    F20 = 90,

    // Media Keys
    VolumeUp = 72,
    VolumeDown = 73,
    Mute = 74,

    Unknown = 0xFFFF, // Use a sentinel value for unknown keys
}

impl KeyCode {
    pub fn is_modifier(&self) -> bool {
        matches!(
            self,
            KeyCode::Command
                | KeyCode::Shift
                | KeyCode::CapsLock
                | KeyCode::Option
                | KeyCode::Control
                | KeyCode::RightShift
                | KeyCode::RightOption
                | KeyCode::RightControl
                | KeyCode::Function
        )
    }
}

impl TryFrom<u64> for KeyCode {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyCode::A),
            1 => Ok(KeyCode::S),
            2 => Ok(KeyCode::D),
            3 => Ok(KeyCode::F),
            4 => Ok(KeyCode::H),
            5 => Ok(KeyCode::G),
            6 => Ok(KeyCode::Z),
            7 => Ok(KeyCode::X),
            8 => Ok(KeyCode::C),
            9 => Ok(KeyCode::V),
            11 => Ok(KeyCode::B),
            12 => Ok(KeyCode::Q),
            13 => Ok(KeyCode::W),
            14 => Ok(KeyCode::E),
            15 => Ok(KeyCode::R),
            16 => Ok(KeyCode::Y),
            17 => Ok(KeyCode::T),
            18 => Ok(KeyCode::One),
            19 => Ok(KeyCode::Two),
            20 => Ok(KeyCode::Three),
            21 => Ok(KeyCode::Four),
            22 => Ok(KeyCode::Six),
            23 => Ok(KeyCode::Five),
            24 => Ok(KeyCode::Equal),
            25 => Ok(KeyCode::Nine),
            26 => Ok(KeyCode::Seven),
            27 => Ok(KeyCode::Minus),
            28 => Ok(KeyCode::Eight),
            29 => Ok(KeyCode::Zero),
            30 => Ok(KeyCode::RightBracket),
            31 => Ok(KeyCode::O),
            32 => Ok(KeyCode::U),
            33 => Ok(KeyCode::LeftBracket),
            34 => Ok(KeyCode::I),
            35 => Ok(KeyCode::P),
            36 => Ok(KeyCode::Return),
            37 => Ok(KeyCode::L),
            38 => Ok(KeyCode::J),
            39 => Ok(KeyCode::Quote),
            40 => Ok(KeyCode::K),
            41 => Ok(KeyCode::Semicolon),
            42 => Ok(KeyCode::Backslash),
            43 => Ok(KeyCode::Comma),
            44 => Ok(KeyCode::Slash),
            45 => Ok(KeyCode::N),
            46 => Ok(KeyCode::M),
            47 => Ok(KeyCode::Period),
            48 => Ok(KeyCode::Tab),
            49 => Ok(KeyCode::Space),
            50 => Ok(KeyCode::Backtick),
            51 => Ok(KeyCode::Delete),
            53 => Ok(KeyCode::Escape),
            55 => Ok(KeyCode::Command),
            56 => Ok(KeyCode::Shift),
            57 => Ok(KeyCode::CapsLock),
            58 => Ok(KeyCode::Option),
            59 => Ok(KeyCode::Control),
            60 => Ok(KeyCode::RightShift),
            61 => Ok(KeyCode::RightOption),
            62 => Ok(KeyCode::RightControl),
            63 => Ok(KeyCode::Function),
            64 => Ok(KeyCode::F17),
            72 => Ok(KeyCode::VolumeUp),
            73 => Ok(KeyCode::VolumeDown),
            74 => Ok(KeyCode::Mute),
            79 => Ok(KeyCode::F18),
            80 => Ok(KeyCode::F19),
            90 => Ok(KeyCode::F20),
            96 => Ok(KeyCode::F5),
            97 => Ok(KeyCode::F6),
            98 => Ok(KeyCode::F7),
            99 => Ok(KeyCode::F3),
            100 => Ok(KeyCode::F8),
            101 => Ok(KeyCode::F9),
            103 => Ok(KeyCode::F11),
            105 => Ok(KeyCode::F13),
            106 => Ok(KeyCode::F16),
            107 => Ok(KeyCode::F14),
            109 => Ok(KeyCode::F10),
            111 => Ok(KeyCode::F12),
            113 => Ok(KeyCode::F15),
            114 => Ok(KeyCode::Help),
            115 => Ok(KeyCode::Home),
            116 => Ok(KeyCode::PageUp),
            117 => Ok(KeyCode::ForwardDelete),
            118 => Ok(KeyCode::F4),
            119 => Ok(KeyCode::End),
            120 => Ok(KeyCode::F2),
            121 => Ok(KeyCode::PageDown),
            122 => Ok(KeyCode::F1),
            123 => Ok(KeyCode::LeftArrow),
            124 => Ok(KeyCode::RightArrow),
            125 => Ok(KeyCode::DownArrow),
            126 => Ok(KeyCode::UpArrow),
            _ => Err(()),
        }
    }
}

impl fmt::Display for KeyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            // Letters
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",

            // Numbers
            KeyCode::Zero => "0",
            KeyCode::One => "1",
            KeyCode::Two => "2",
            KeyCode::Three => "3",
            KeyCode::Four => "4",
            KeyCode::Five => "5",
            KeyCode::Six => "6",
            KeyCode::Seven => "7",
            KeyCode::Eight => "8",
            KeyCode::Nine => "9",

            // Symbols
            KeyCode::Equal => "=",
            KeyCode::Minus => "-",
            KeyCode::Backtick => "`",
            KeyCode::Backslash => "\\",
            KeyCode::Comma => ",",
            KeyCode::Period => ".",
            KeyCode::Slash => "/",
            KeyCode::Semicolon => ";",
            KeyCode::Quote => "'",
            KeyCode::LeftBracket => "[",
            KeyCode::RightBracket => "]",

            // Modifiers
            KeyCode::Command => "Command",
            KeyCode::Shift => "Shift",
            KeyCode::CapsLock => "CapsLock",
            KeyCode::Option => "Option",
            KeyCode::Control => "Control",
            KeyCode::RightShift => "Right Shift",
            KeyCode::RightOption => "Right Option",
            KeyCode::RightControl => "Right Control",
            KeyCode::Function => "Function",

            // Navigation & Editing
            KeyCode::Return => "Return",
            KeyCode::Tab => "Tab",
            KeyCode::Space => "Space",
            KeyCode::Delete => "Delete",
            KeyCode::Escape => "Escape",
            KeyCode::ForwardDelete => "Forward Delete",
            KeyCode::Home => "Home",
            KeyCode::End => "End",
            KeyCode::PageUp => "Page Up",
            KeyCode::PageDown => "Page Down",
            KeyCode::Help => "Help",

            // Arrows
            KeyCode::LeftArrow => "Left Arrow",
            KeyCode::RightArrow => "Right Arrow",
            KeyCode::UpArrow => "Up Arrow",
            KeyCode::DownArrow => "Down Arrow",

            // Function Keys
            KeyCode::F1 => "F1",
            KeyCode::F2 => "F2",
            KeyCode::F3 => "F3",
            KeyCode::F4 => "F4",
            KeyCode::F5 => "F5",
            KeyCode::F6 => "F6",
            KeyCode::F7 => "F7",
            KeyCode::F8 => "F8",
            KeyCode::F9 => "F9",
            KeyCode::F10 => "F10",
            KeyCode::F11 => "F11",
            KeyCode::F12 => "F12",
            KeyCode::F13 => "F13",
            KeyCode::F14 => "F14",
            KeyCode::F15 => "F15",
            KeyCode::F16 => "F16",
            KeyCode::F17 => "F17",
            KeyCode::F18 => "F18",
            KeyCode::F19 => "F19",
            KeyCode::F20 => "F20",

            // Media
            KeyCode::VolumeUp => "Volume Up",
            KeyCode::VolumeDown => "Volume Down",
            KeyCode::Mute => "Mute",

            // Unknown
            KeyCode::Unknown => "Unknown",
        };

        write!(f, "{}", name)
    }
}
