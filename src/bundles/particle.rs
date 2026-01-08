//! Bundle for spawning a particle entity
//!
//! Contains all components needed for a particle:
//! - Particle type marker
//! - Particle type enum
//! - Velocity for physics
//! - Position for physics (separate from Transform)
//! - Mesh for rendering
//! - Material for rendering
//! - Transform for rendering

use crate::components::{ParticleMarker, ParticleType, Position, Velocity};
use bevy::color::palettes::tailwind::{BLUE_500, GREEN_500, RED_500};
use bevy::prelude::*;
use bevy::sprite_render::{ColorMaterial, MeshMaterial2d};

/// Bundle for spawning a particle entity
///
/// Contains all components needed for a particle:
/// - Particle type marker
/// - Particle type enum
/// - Velocity for physics
/// - Position for physics (separate from Transform)
/// - Mesh for rendering
/// - Material for rendering
/// - Transform for rendering
#[derive(Bundle, Debug, Clone)]
pub struct Particle {
    /// Marker component identifying this as a particle
    pub marker: ParticleMarker,
    /// Type of this particle
    pub particle_type: ParticleType,
    /// Velocity for physics updates
    pub velocity: Velocity,
    /// Position for physics (separate from Transform)
    pub position: Position,
    /// 2D mesh for rendering
    pub mesh: Mesh2d,
    /// Material for rendering
    pub material: MeshMaterial2d<ColorMaterial>,
    /// Transform for rendering (synced from Position)
    pub transform: Transform,
}

impl Particle {
    /// Spawns a new particle entity with given properties
    ///
    /// # Arguments
    /// - `commands`: Bevy command queue
    /// - `meshes`: Mesh assets resource
    /// - `material`: Material assets resource
    /// - `transform`: Initial transform (position will be copied to Position component)
    /// - `particle_type`: Type of particle to spawn
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &mut ResMut<Assets<ColorMaterial>>,
        transform: Transform,
        particle_type: ParticleType,
    ) {
        commands.spawn(Self {
            marker: ParticleMarker,
            particle_type,
            velocity: Velocity::new(Vec3::default()),
            position: Position::new(transform.translation),
            mesh: Mesh2d(meshes.add(Circle::new(10.0))),
            material: match particle_type {
                ParticleType::Red => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(RED_500)))
                }
                ParticleType::Blue => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(BLUE_500)))
                }
                ParticleType::Green => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(GREEN_500)))
                }
            },
            transform,
        });
    }
}
