//! Particle type component

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;
use bevy::ecs::component::Component;

/// Type of particle in the simulation
///
/// Each particle type can have different interaction forces with
/// every other particle type.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum ParticleType {
    /// Red particle type
    #[default]
    Red = 0,
    /// Blue particle type
    Blue = 1,
    /// Green particle type
    Green = 2,
}

impl ParticleType {
    /// Total number of particle types
    pub const COUNT: usize = 3;

    /// Returns an array containing all particle types
    #[must_use]
    pub const fn all_types() -> [Self; Self::COUNT] {
        [Self::Red, Self::Blue, Self::Green]
    }

    /// Returns string representation of this particle type
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
            Self::Green => "Green",
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
            "Invalid particle type. Expected one of: red, blue, or green"
        )
    }
}

impl Error for ParticleTypeError {}

impl FromStr for ParticleType {
    type Err = ParticleTypeError;

    fn from_str(s: &str) -> Result<Self, ParticleTypeError> {
        match s.to_lowercase().as_str() {
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err(ParticleTypeError),
        }
    }
}
