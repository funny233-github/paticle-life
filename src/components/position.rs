//! Position component for particles
//!
//! Stores the position vector for physics calculations.
//! This is separate from `Transform` to allow the physics system
//! to update positions independently from rendering.

use bevy::ecs::component::Component;
use bevy::math::Vec3;

/// Position component for particles
///
/// Stores the position vector for physics calculations.
/// This is separate from `Transform` to allow the physics system
/// to update positions independently from rendering.
///
/// The physics system updates `Position`, while `sync_transform`
/// copies it to `Transform` for rendering.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Position {
    /// Position vector in world space
    pub value: Vec3,
}

impl Position {
    /// Creates a new position from a vector
    #[must_use]
    pub const fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Creates a new position from x, y, z components
    #[must_use]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}
