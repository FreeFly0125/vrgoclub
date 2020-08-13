use serde::{Deserialize, Serialize};

pub mod profile;

/// Enum representing the different types of moderator a user can be
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum ModLevel {
    /// User isn't a moderator
    None,

    /// User is a normal moderator
    Normal,

    /// User is an elder moderator
    Elder,

    /// Unknown or invalid value. This variant will be constructed if robtop ever adds more
    /// moderator levels and will hold the internal game value associated with the new moderator
    /// level
    Unknown(u8),
}

impl From<ModLevel> for u8 {
    fn from(level: ModLevel) -> u8 {
        match level {
            ModLevel::None => 0,
            ModLevel::Normal => 1,
            ModLevel::Elder => 2,
            ModLevel::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for ModLevel {
    fn from(i: u8) -> Self {
        match i {
            0 => ModLevel::None,
            1 => ModLevel::Normal,
            2 => ModLevel::Elder,
            i => ModLevel::Unknown(i),
        }
    }
}

// Enum representing an in-game icon color
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Color {
    /// A color whose index was known to dash-rs which could be converted to RGB values
    Known(u8, u8, u8),

    /// The index of some unknown colors. This variant will be constructed if robtop ever adds more
    /// colors and while dash-rs hasn't updated yet
    Unknown(u8),
}

impl From<u8> for Color {
    fn from(idx: u8) -> Self {
        // This match expression is listing the colors in order of the in-game selection menu!
        match idx {
            0 => Color::Known(125, 255, 0),
            1 => Color::Known(0, 255, 0),
            2 => Color::Known(0, 255, 125),
            3 => Color::Known(0, 255, 255),
            16 => Color::Known(0, 200, 255),
            4 => Color::Known(0, 125, 255),
            5 => Color::Known(0, 0, 255),
            6 => Color::Known(125, 0, 255),
            13 => Color::Known(185, 0, 255),
            7 => Color::Known(255, 0, 255),
            8 => Color::Known(255, 0, 125),
            9 => Color::Known(255, 0, 0),
            29 => Color::Known(255, 75, 0),
            10 => Color::Known(255, 125, 0),
            14 => Color::Known(255, 185, 0),
            11 => Color::Known(255, 255, 0),
            12 => Color::Known(255, 255, 255),
            17 => Color::Known(175, 175, 175),
            18 => Color::Known(80, 80, 80),
            15 => Color::Known(0, 0, 0),
            27 => Color::Known(125, 125, 0),
            32 => Color::Known(100, 150, 0),
            28 => Color::Known(75, 175, 0),
            38 => Color::Known(0, 150, 0),
            20 => Color::Known(0, 175, 75),
            33 => Color::Known(0, 150, 100),
            21 => Color::Known(0, 125, 125),
            34 => Color::Known(0, 100, 150),
            22 => Color::Known(0, 75, 175),
            39 => Color::Known(0, 0, 150),
            23 => Color::Known(75, 0, 175),
            35 => Color::Known(100, 0, 150),
            24 => Color::Known(125, 0, 125),
            36 => Color::Known(150, 0, 100),
            25 => Color::Known(175, 0, 75),
            37 => Color::Known(150, 0, 0),
            30 => Color::Known(150, 50, 0),
            26 => Color::Known(175, 75, 0),
            31 => Color::Known(150, 100, 0),
            19 => Color::Known(255, 255, 125),
            40 => Color::Known(125, 255, 175),
            41 => Color::Known(125, 125, 255),
            idx => Color::Unknown(idx),
        }
    }
}
