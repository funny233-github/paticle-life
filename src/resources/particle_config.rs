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
    /// Collision distance (particles closer than this repel)
    pub d1: f32,
    /// Interaction transition start distance
    pub d2: f32,
    /// Maximum interaction distance and spatial partition chunk size
    pub d3: f32,
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
            init_particle_num: 1000,
            map_width: 7608.0,
            map_height: 3909.0,

            d1: 30.0 * 3.0,
            d2: 65.0 * 3.0,
            d3: 100.0 * 3.0,

            repel_force: -100.0,

            dt: 0.005,
            dt_half: 5.0,
        }
    }
}
