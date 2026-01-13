//! Configuration for particle simulation
//!
//! Contains all tunable parameters for the particle system.
//! These can be modified at runtime via console commands.

use bevy::ecs::resource::Resource;

/// Configuration for particle simulation
///
/// Contains all tunable parameters for the particle system.
/// These can be modified at runtime via console commands.
#[derive(Debug, Resource, Clone)]
pub struct ParticleConfig {
    /// Initial number of particles to spawn
    pub init_particle_num: usize,
    /// Width of the simulation map boundary
    pub map_width: f32,
    /// Height of the simulation map boundary
    pub map_height: f32,
    /// Interaction distance
    pub r: f32,
    /// Force magnitude for collision repulsion
    pub repel_force: f32,
    /// Time step for physics updates
    pub dt: f32,
    /// The half life period of velocity
    pub dt_half: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            init_particle_num: 2000,
            map_width: 2000.0,
            map_height: 2000.0,

            r: 300.0,

            repel_force: 1.0,

            dt: 1.0,
            dt_half: 1.0,
        }
    }
}
