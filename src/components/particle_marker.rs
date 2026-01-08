//! Marker component for particles

use bevy::ecs::component::Component;

/// Marker component for particles
///
/// Used in queries to identify entities that are particles.
#[derive(Component, Debug, Clone, Copy)]
pub struct ParticleMarker;
