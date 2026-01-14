//! Bevy game simulation library with particle systems
//!
//! This library provides a particle simulation system built on the Bevy game engine.
//! It features console commands for runtime configuration, particle interaction tables,
//! and separated physics/rendering updates.
//!
//! # Main Features
//! - Particle simulation with different types (Red, Blue, Green)
//! - Configurable interaction tables stored in CSV format
//! - Runtime console commands for tweaking simulation parameters
//! - Input focus management for game vs console input
//! - Camera movement controls

#![forbid(missing_docs, unsafe_code)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]

use crate::components::{ParticleMarker, ParticleType};
use crate::resources::{
    CameraMoveConfig, InputFocus, ParticleConfig, ParticleInteractionTable, ParticleUpdateToggle,
};
use crate::systems::{
    clean_particle, move_camera, respawn_particle, setup, spawn_particle, sync_transform,
    toggle_particle_update, update_collision, update_input_focus, update_particle,
};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::prelude::*;
use bevy::sprite_render::ColorMaterial;
use bevy_console::{AddConsoleCommand, ConsoleCommand, clap, reply};
use clap::{Parser, Subcommand};

/// Components module - all Bevy components used in the game
pub mod components;

/// Bundles module - all Bevy bundles used in the game
pub mod bundles;

/// Resources module - all Bevy resources used in the game
pub mod resources;

/// Systems module - all Bevy systems used in the game
pub mod systems;

// ============================================================================
// Camera Movement Plugin
// ============================================================================

/// Plugin that registers camera movement system
///
/// This plugin:
/// - Inserts the default [`CameraMoveConfig`] resource
/// - Registers the [`move_camera`] system to run in the `Update` schedule
pub struct CameraMovePlugin;

impl Plugin for CameraMovePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraMoveConfig::default());
        app.add_systems(Update, move_camera);
    }
}

// ============================================================================
// Input Focus Plugin
// ============================================================================

/// Plugin that registers input focus management system
///
/// This plugin:
/// - Inserts the default [`InputFocus`] resource
/// - Registers the [`update_input_focus`] system
pub struct InputFocusPlugin;

impl Plugin for InputFocusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputFocus::default());
        app.add_systems(Update, update_input_focus);
    }
}

// ============================================================================
// Console Command Plugin
// ============================================================================

/// Subcommands for the `set` console command
#[derive(Subcommand, Clone, PartialEq)]
enum SetSubcommand {
    /// Set map boundary dimensions
    Boundary { width: f32, height: f32 },
    /// Set interaction distance
    R { value: f32 },
    /// Set the repel force magnitude for collision
    RepelForce { value: f32 },
    /// Set half life period of velocity
    DTHalf { value: f32 },
    /// Set the time step for particle updates
    Dt { value: f32 },
    /// Set the initial number of particles to spawn
    InitParticleNum { value: usize },
}

/// Console command for setting simulation parameters
#[derive(Parser, ConsoleCommand)]
#[command(name = "set")]
struct SetCommand {
    #[command(subcommand)]
    subcommand: SetSubcommand,
}

/// Handle the `set` console command
///
/// Updates particle configuration with the specified parameter value.
/// Changes take effect immediately in the running simulation.
fn set(mut log: ConsoleCommand<SetCommand>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetCommand { subcommand })) = log.take() {
        match subcommand {
            SetSubcommand::Boundary { width, height } => {
                config.map_width = width;
                config.map_height = height;
                reply!(
                    log,
                    "set map width: {:.2}, height: {:.2} successfully",
                    width,
                    height
                );
            }
            SetSubcommand::R { value } => {
                config.r = value;
                reply!(log, "set r to {:.2} successfully", value);
            }
            SetSubcommand::RepelForce { value } => {
                config.repel_force = value;
                reply!(log, "set repel_force to {:.2} successfully", value);
            }
            SetSubcommand::DTHalf { value } => {
                config.dt_half = value;
                reply!(log, "set dt_half to {:.3} successfully", value);
            }
            SetSubcommand::Dt { value } => {
                config.dt = value;
                reply!(log, "set dt to {:.3} successfully", value);
            }
            SetSubcommand::InitParticleNum { value } => {
                config.init_particle_num = value;
                reply!(log, "set init_particle_num to {} successfully", value);
            }
        }
    }
}

/// Subcommands for the `print` console command
#[derive(Subcommand, Clone, PartialEq)]
enum PrintSubcommand {
    /// Print map boundary dimensions
    Boundary,
    /// Print particle interaction table
    Interaction,
    /// Print interaction distance
    R,
    /// Print repel force magnitude for collisions
    RepelForce,
    /// Print half-life period of velocity
    Temperature,
    /// Print time step for particle updates
    Dt,
    /// Print all configuration values
    Config,
}

/// Console command for printing current configuration values
#[derive(Parser, ConsoleCommand)]
#[command(name = "print")]
struct PrintCommand {
    #[command(subcommand)]
    subcommand: PrintSubcommand,
}

/// Handle the `print` console command
///
/// Displays the current value of the specified configuration parameter.
#[allow(clippy::needless_pass_by_value)]
fn print(
    mut log: ConsoleCommand<PrintCommand>,
    config: Res<ParticleConfig>,
    interaction_table: Res<ParticleInteractionTable>,
) {
    use std::fmt::Write;
    if let Some(Ok(PrintCommand { subcommand })) = log.take() {
        match subcommand {
            PrintSubcommand::Boundary => {
                reply!(
                    log,
                    "map width: {:.2}, height: {:.2}",
                    config.map_width,
                    config.map_height
                );
            }
            PrintSubcommand::Interaction => {
                let types = ParticleType::all_types();
                let mut output = String::from("Particle Interaction Table:\n");
                write!(output, "{:>8} ", "target\\source").unwrap();
                for source_type in &types {
                    write!(output, "{:>6} ", source_type.as_str()).unwrap();
                }
                output.push('\n');

                for target_type in &types {
                    write!(output, "{:>8} ", target_type.as_str()).unwrap();
                    for source_type in &types {
                        let strength =
                            interaction_table.get_interaction(*target_type, *source_type);
                        write!(output, "{strength:>6.1} ").unwrap();
                    }
                    output.push('\n');
                }
                reply!(log, "{}", output);
            }
            PrintSubcommand::R => {
                reply!(log, "r: {:.2}", config.r);
            }
            PrintSubcommand::RepelForce => {
                reply!(log, "repel_force: {:.2}", config.repel_force);
            }
            PrintSubcommand::Temperature => {
                reply!(log, "dt_half: {:.3}", config.dt_half);
            }
            PrintSubcommand::Dt => {
                reply!(log, "dt: {:.3}", config.dt);
            }
            PrintSubcommand::Config => {
                reply!(
                    log,
                    "ParticleConfig:\n\
                     - init_particle_num: {}\n\
                     - map_width: {:.2}\n\
                     - map_height: {:.2}\n\
                     - r: {:.2}\n\
                     - repel_force: {:.2}\n\
                     - temperature: {:.3}\n\
                     - dt: {:.3}",
                    config.init_particle_num,
                    config.map_width,
                    config.map_height,
                    config.r,
                    config.repel_force,
                    config.dt_half,
                    config.dt
                );
            }
        }
    }
}

/// Console command to respawn all particles
#[derive(Parser, ConsoleCommand)]
#[command(name = "respawn_particle")]
struct RespawnParticle;

/// Console command to set interaction between particle types
#[derive(Parser, ConsoleCommand)]
#[command(name = "interaction")]
struct InteractionCommand {
    /// Target particle type (Red, Blue, or Green)
    target: ParticleType,
    /// Source particle type (Red, Blue, or Green)
    source: ParticleType,
    /// Interaction force value
    value: f32,
}

/// Console command to reset interactions from CSV file
#[derive(Parser, ConsoleCommand)]
#[command(name = "reset_interaction")]
struct ResetInteractionCommand;

/// Console command to set all interactions to random values
#[derive(Parser, ConsoleCommand)]
#[command(name = "random_interaction")]
struct RandomInteractionCommand;

/// Handle the `interaction` console command
///
/// Sets the interaction force between two particle types.
///
/// Positive values cause attraction, negative values cause repulsion.
fn interaction(
    mut log: ConsoleCommand<InteractionCommand>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
) {
    if let Some(Ok(InteractionCommand {
        target,
        source,
        value,
    })) = log.take()
    {
        interaction_table.set_interaction(target, source, value);
        reply!(
            log,
            "Set interaction {}[{}] <- {}[{}] = {:.1}",
            target.as_str(),
            target as usize,
            source.as_str(),
            source as usize,
            value
        );
    }
}

/// Handle the `reset_interaction` console command
///
/// Resets all particle interactions to the values stored in the CSV file.
fn reset_interaction(
    mut log: ConsoleCommand<ResetInteractionCommand>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
) {
    if matches!(log.take(), Some(Ok(ResetInteractionCommand))) {
        let csv_path = "particle_interactions.csv";
        match ParticleInteractionTable::from_csv_file(csv_path) {
            Ok(loaded_table) => {
                *interaction_table = loaded_table;
                reply!(log, "Reset interactions from file: {}", csv_path);
            }
            Err(e) => {
                reply!(
                    log,
                    "Warning: Could not load {}, keeping current interactions",
                    csv_path
                );
                reply!(log, "Error: {}", e);
            }
        }
    }
}

/// Handle the `random_interaction` console command
///
/// Sets all particle interactions to random values between -100.0 and 100.0.
fn random_interaction(
    mut log: ConsoleCommand<RandomInteractionCommand>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
) {
    if matches!(log.take(), Some(Ok(RandomInteractionCommand))) {
        for target in ParticleType::all_types() {
            for source in ParticleType::all_types() {
                let value = rand::random_range(-1.0..1.0);
                interaction_table.set_interaction(target, source, value);
            }
        }
        reply!(
            log,
            "Set all interactions to random values between -100.0 and 100.0"
        );
        interaction_table.print_table();
    }
}

/// Handle the `respawn_particle` console command
///
/// Removes all existing particles and spawns a new set according to the
/// current configuration.
fn respawn_particle_console(
    mut log: ConsoleCommand<RespawnParticle>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<ParticleMarker>>,
    config: Res<ParticleConfig>,
) {
    if matches!(log.take(), Some(Ok(RespawnParticle))) {
        clean_particle(commands.reborrow(), query);
        spawn_particle(commands, meshes, material, config);
        reply!(log, "Respawned all particles");
    }
}

/// Plugin that registers all console commands
///
/// This plugin registers:
/// - `set` command
/// - `print` command
/// - `interaction` command
/// - `reset_interaction` command
/// - `random_interaction` command
/// - `respawn_particle` command
pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SetCommand, _>(set);
        app.add_console_command::<PrintCommand, _>(print);
        app.add_console_command::<InteractionCommand, _>(interaction);
        app.add_console_command::<ResetInteractionCommand, _>(reset_interaction);
        app.add_console_command::<RandomInteractionCommand, _>(random_interaction);
        app.add_console_command::<RespawnParticle, _>(respawn_particle_console);
    }
}

// ============================================================================
// Particle Simulation Plugin
// ============================================================================

/// Plugin for particle simulation system
///
/// This plugin:
/// - Inserts the particle configuration resource
/// - Registers all particle simulation systems
/// - Spawns initial particles
///
/// # Systems
/// - `setup` (Startup): Loads interactions and spawns particles
/// - `toggle_particle_update` (Update): Toggles physics updates with T key
/// - `update_particle` (Update, conditional): Updates particle physics
/// - `sync_transform` (Update): Syncs Position to Transform for rendering
/// - `respawn_particle` (Update): Respawns particles when requested
#[derive(Debug, Default)]
pub struct ParticlePlugin {
    /// Configuration for the particle system
    pub config: ParticleConfig,
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone());
        app.insert_resource(ParticleUpdateToggle::new());
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_particle_update);
        app.add_systems(
            Update,
            (
                update_collision,
                update_particle.run_if(|toggle: Res<ParticleUpdateToggle>| toggle.is_enabled()),
            )
                .chain(),
        );
        app.add_systems(Update, sync_transform);
        app.add_systems(Update, respawn_particle);
    }
}
