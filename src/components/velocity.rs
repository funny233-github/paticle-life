//! Velocity component for particles
//!
//! Stores the velocity vector for physics calculations.
//! This is separate from position to allow for clean physics updates.

use bevy::ecs::component::Component;
use bevy::math::Vec3;

/// Velocity component for particles
///
/// Stores the velocity vector for physics calculations.
/// This is separate from position to allow for clean physics updates.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Velocity {
    /// Velocity vector (units per second)
    pub value: Vec3,
}

impl Velocity {
    /// Creates a new velocity from a vector
    #[must_use]
    pub const fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Creates a new velocity from x, y, z components
    #[must_use]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}
