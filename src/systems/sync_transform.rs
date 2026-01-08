//! Sync particle positions to transform for rendering
//!
//! This system copies the physics `Position` component to the
//! rendering `Transform` component. This allows the physics system
//! to update positions independently from the rendering system.

use crate::components::{ParticleMarker, Position};
use bevy::prelude::*;

/// Sync particle positions to transform for rendering
///
/// This system copies the physics `Position` component to the
/// rendering `Transform` component. This allows the physics system
/// to update positions independently from the rendering system.
///
/// This system runs every frame to ensure particles are rendered
/// at their current physics positions.
pub fn sync_transform(mut query: Query<(&Position, &mut Transform), With<ParticleMarker>>) {
    for (position, mut transform) in &mut query {
        transform.translation = position.value;
    }
}
