//! Setup function that runs once at startup
//!
//! This system:
//! 1. Loads particle interactions from CSV file (if present)
//! 2. Spawns initial particles according to configuration

use crate::resources::{ParticleConfig, ParticleInteractionTable};
use crate::systems::spawn_particle;
use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;

/// Setup function that runs once at startup
///
/// 1. Loads particle interactions from CSV file (if present)
/// 2. Spawns initial particles according to configuration
///
/// # Arguments
/// - `commands`: Bevy command queue
/// - `meshes`: Mesh assets resource
/// - `material`: Material assets resource
/// - `interaction_table`: Interaction table resource to populate
/// - `config`: Particle configuration with spawn parameters
pub fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    let csv_path = "particle_interactions.csv";
    match ParticleInteractionTable::from_csv_file(csv_path) {
        Ok(loaded_table) => {
            *interaction_table = loaded_table;
            bevy::log::info!(
                "Successfully loaded particle interactions from {}",
                csv_path
            );
        }
        Err(e) => {
            bevy::log::warn!("Could not load {}, using default interactions", csv_path);
            bevy::log::error!("Error: {}", e);
        }
    }

    spawn_particle(commands, meshes, material, config);
}
