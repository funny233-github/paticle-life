//! Particle type component

use bevy::color::Color;
use bevy::color::palettes::tailwind::{
    AMBER_500, BLUE_500, CYAN_500, EMERALD_500, FUCHSIA_500, GREEN_500, INDIGO_500, LIME_500,
    ORANGE_500, PINK_500, PURPLE_500, RED_500, ROSE_500, SKY_500, TEAL_500, VIOLET_500, YELLOW_500,
};
use bevy::ecs::component::Component;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// Type of particle in the simulation
///
/// Each particle type can have different interaction forces with
/// every other particle type.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum ParticleType {
    /// Amber particle type
    #[default]
    Amber = 0,
    /// Blue particle type
    Blue = 1,
    /// Cyan particle type
    Cyan = 2,
    /// Emerald particle type
    Emerald = 3,
    /// Fuchsia particle type
    Fuchsia = 4,
    /// Green particle type
    Green = 5,
    /// Indigo particle type
    Indigo = 6,
    /// Lime particle type
    Lime = 7,
    /// Orange particle type
    Orange = 8,
    /// Pink particle type
    Pink = 9,
    /// Purple particle type
    Purple = 10,
    /// Red particle type
    Red = 11,
    /// Rose particle type
    Rose = 12,
    /// Sky particle type
    Sky = 13,
    /// Teal particle type
    Teal = 14,
    /// Violet particle type
    Violet = 15,
    /// Yellow particle type
    Yellow = 16,
}

impl ParticleType {
    /// Total number of particle types
    pub const COUNT: usize = 17;

    /// Returns an array containing all particle types
    #[must_use]
    pub const fn all_types() -> [Self; Self::COUNT] {
        [
            Self::Amber,
            Self::Blue,
            Self::Cyan,
            Self::Emerald,
            Self::Fuchsia,
            Self::Green,
            Self::Indigo,
            Self::Lime,
            Self::Orange,
            Self::Pink,
            Self::Purple,
            Self::Red,
            Self::Rose,
            Self::Sky,
            Self::Teal,
            Self::Violet,
            Self::Yellow,
        ]
    }

    /// Returns string representation of this particle type
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Amber => "Amber",
            Self::Blue => "Blue",
            Self::Cyan => "Cyan",
            Self::Emerald => "Emerald",
            Self::Fuchsia => "Fuchsia",
            Self::Green => "Green",
            Self::Indigo => "Indigo",
            Self::Lime => "Lime",
            Self::Orange => "Orange",
            Self::Pink => "Pink",
            Self::Purple => "Purple",
            Self::Red => "Red",
            Self::Rose => "Rose",
            Self::Sky => "Sky",
            Self::Teal => "Teal",
            Self::Violet => "Violet",
            Self::Yellow => "Yellow",
        }
    }

    /// Returns the color associated with this particle type
    #[must_use]
    pub const fn to_color(&self) -> Color {
        match self {
            Self::Amber => Color::Srgba(AMBER_500),
            Self::Blue => Color::Srgba(BLUE_500),
            Self::Cyan => Color::Srgba(CYAN_500),
            Self::Emerald => Color::Srgba(EMERALD_500),
            Self::Fuchsia => Color::Srgba(FUCHSIA_500),
            Self::Green => Color::Srgba(GREEN_500),
            Self::Indigo => Color::Srgba(INDIGO_500),
            Self::Lime => Color::Srgba(LIME_500),
            Self::Orange => Color::Srgba(ORANGE_500),
            Self::Pink => Color::Srgba(PINK_500),
            Self::Purple => Color::Srgba(PURPLE_500),
            Self::Red => Color::Srgba(RED_500),
            Self::Rose => Color::Srgba(ROSE_500),
            Self::Sky => Color::Srgba(SKY_500),
            Self::Teal => Color::Srgba(TEAL_500),
            Self::Violet => Color::Srgba(VIOLET_500),
            Self::Yellow => Color::Srgba(YELLOW_500),
        }
    }
}

/// Error returned when parsing an invalid particle type string
#[derive(Debug)]
pub struct ParticleTypeError;

impl Display for ParticleTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid particle type. Expected one of: amber, blue, cyan, emerald, fuchsia, green, indigo, lime, orange, pink, purple, red, rose, sky, teal, violet, yellow"
        )
    }
}

impl Error for ParticleTypeError {}

impl FromStr for ParticleType {
    type Err = ParticleTypeError;

    fn from_str(s: &str) -> Result<Self, ParticleTypeError> {
        match s.to_lowercase().as_str() {
            "amber" => Ok(Self::Amber),
            "blue" => Ok(Self::Blue),
            "cyan" => Ok(Self::Cyan),
            "emerald" => Ok(Self::Emerald),
            "fuchsia" => Ok(Self::Fuchsia),
            "green" => Ok(Self::Green),
            "indigo" => Ok(Self::Indigo),
            "lime" => Ok(Self::Lime),
            "orange" => Ok(Self::Orange),
            "pink" => Ok(Self::Pink),
            "purple" => Ok(Self::Purple),
            "red" => Ok(Self::Red),
            "rose" => Ok(Self::Rose),
            "sky" => Ok(Self::Sky),
            "teal" => Ok(Self::Teal),
            "violet" => Ok(Self::Violet),
            "yellow" => Ok(Self::Yellow),
            _ => Err(ParticleTypeError),
        }
    }
}
