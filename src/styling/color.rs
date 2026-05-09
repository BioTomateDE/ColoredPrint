#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Color {
    #[default]
    Default,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    #[must_use]
    pub fn try_from_char(ch: char) -> Option<Self> {
        Some(match ch {
            '_' => Self::Default,

            // Normal colors are lowercase
            'k' => Self::Black, // Black is 'k', not 'b'
            'r' => Self::Red,
            'g' => Self::Green,
            'y' => Self::Yellow,
            'b' => Self::Blue,
            'm' => Self::Magenta,
            'c' => Self::Cyan,
            'w' => Self::White,

            // Bright colors are uppercase
            'K' => Self::BrightBlack, // Black is 'K', not 'B'
            'R' => Self::BrightRed,
            'G' => Self::BrightGreen,
            'Y' => Self::BrightYellow,
            'B' => Self::BrightBlue,
            'M' => Self::BrightMagenta,
            'C' => Self::BrightCyan,
            'W' => Self::BrightWhite,

            // True colors (RGB) are not supported.
            _ => return None,
        })
    }

    pub fn from_char(ch: char) -> Result<Self, String> {
        Self::try_from_char(ch).ok_or_else(|| format!("Invalid color character {ch:?}"))
    }

    #[must_use]
    pub fn foreground_code(self) -> &'static str {
        match self {
            Self::Default => "39;",
            Self::Black => "30;",
            Self::Red => "31;",
            Self::Green => "32;",
            Self::Yellow => "33;",
            Self::Blue => "34;",
            Self::Magenta => "35;",
            Self::Cyan => "36;",
            Self::White => "37;",
            Self::BrightBlack => "90;",
            Self::BrightRed => "91;",
            Self::BrightGreen => "92;",
            Self::BrightYellow => "93;",
            Self::BrightBlue => "94;",
            Self::BrightMagenta => "95;",
            Self::BrightCyan => "96;",
            Self::BrightWhite => "97;",
        }
    }

    #[must_use]
    pub fn background_code(self) -> &'static str {
        match self {
            Self::Default => "49;",
            Self::Black => "40;",
            Self::Red => "41;",
            Self::Green => "42;",
            Self::Yellow => "43;",
            Self::Blue => "44;",
            Self::Magenta => "45;",
            Self::Cyan => "46;",
            Self::White => "47;",
            Self::BrightBlack => "100;",
            Self::BrightRed => "101;",
            Self::BrightGreen => "102;",
            Self::BrightYellow => "103;",
            Self::BrightBlue => "104;",
            Self::BrightMagenta => "105;",
            Self::BrightCyan => "106;",
            Self::BrightWhite => "107;",
        }
    }
}
