//! Respawn particles when requested
//!
//! This system removes all existing particles and spawns a new set
//! according to current configuration.

use crate::bundles::Particle;
use crate::components::ParticleMarker;
use crate::components::ParticleType;
use crate::resources::{InputFocus, ParticleConfig};
use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;

/// Respawn particles when requested
///
/// This system removes all existing particles and spawns a new set
/// according to current configuration.
///
/// This is triggered by the `respawn_particle` console command
/// or the R key when the game has focus.
#[allow(clippy::needless_pass_by_value)]
pub fn respawn_particle(
    mut commands: Commands,
    query: Query<Entity, With<ParticleMarker>>,
    meshes: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    config: Res<ParticleConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.is_game() && keys.just_pressed(KeyCode::KeyR) {
        clean_particle(commands.reborrow(), query);
        spawn_particle(commands, meshes, material, config);
    }
}

/// Remove all particles from the simulation
///
/// Despawns all entities with the [`ParticleMarker`] component.
pub fn clean_particle(mut commands: Commands, query: Query<Entity, With<ParticleMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    bevy::log::info!("Cleaned all particles");
}

/// Spawn initial particles according to configuration
///
/// Creates the specified number of particles with random positions
/// and types within the map boundaries.
///
/// # Arguments
/// - `commands`: Bevy command queue
/// - `meshes`: Mesh assets resource
/// - `material`: Material assets resource
/// - `config`: Particle configuration with spawn parameters
#[allow(clippy::needless_pass_by_value)]
pub fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    config: Res<ParticleConfig>,
) {
    let particle_types = [ParticleType::Red, ParticleType::Blue, ParticleType::Green];

    for _ in 0..config.init_particle_num {
        let x = rand::random_range(-config.map_width / 2.0..config.map_width / 2.0);
        let y = rand::random_range(-config.map_height / 2.0..config.map_height / 2.0);

        let particle_type = particle_types[rand::random_range(0..particle_types.len())];

        Particle::spawn(
            &mut commands,
            &mut meshes,
            &mut material,
            Transform::from_xyz(x, y, 0.0),
            particle_type,
        );
    }
}
