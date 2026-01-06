use crate::particle::*;
use bevy::prelude::*;
use bevy_console::*;
use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, PartialEq)]
enum SetSubcommand {
    /// Set map boundary dimensions
    Boundary { width: f32, height: f32 },
    /// Set d1 distance value (collision distance)
    D1 { value: f32 },
    /// Set d2 distance value (interaction transition start)
    D2 { value: f32 },
    /// Set d3 distance value (interaction max distance and chunk size)
    D3 { value: f32 },
    /// Set the repel force magnitude for collision
    RepelForce { value: f32 },
    /// Set temperature coefficient for velocity damping
    Temperature { value: f32 },
    /// Set the time step for particle updates
    Dt { value: f32 },
    /// Set the initial number of particles to spawn
    InitParticleNum { value: usize },
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set")]
struct SetCommand {
    #[command(subcommand)]
    subcommand: SetSubcommand,
}

fn set(mut log: ConsoleCommand<SetCommand>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetCommand { subcommand })) = log.take() {
        match subcommand {
            SetSubcommand::Boundary { width, height } => {
                config.map_width = width;
                config.map_height = height;
                reply!(log, "set map width: {:.2}, height: {:.2} successfully", width, height);
            }
            SetSubcommand::D1 { value } => {
                config.d1 = value;
                reply!(log, "set d1 to {:.2} successfully", value);
            }
            SetSubcommand::D2 { value } => {
                config.d2 = value;
                reply!(log, "set d2 to {:.2} successfully", value);
            }
            SetSubcommand::D3 { value } => {
                config.d3 = value;
                reply!(log, "set d3 to {:.2} successfully", value);
            }
            SetSubcommand::RepelForce { value } => {
                config.repel_force = value;
                reply!(log, "set repel_force to {:.2} successfully", value);
            }
            SetSubcommand::Temperature { value } => {
                config.temperature = value;
                reply!(log, "set temperature to {:.3} successfully", value);
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrintTarget {
    Boundary,
    D,
    RepelForce,
    Temperature,
    Dt,
    Config,
}

impl std::str::FromStr for PrintTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "boundary" => Ok(PrintTarget::Boundary),
            "d" => Ok(PrintTarget::D),
            "repel_force" => Ok(PrintTarget::RepelForce),
            "temperature" => Ok(PrintTarget::Temperature),
            "dt" => Ok(PrintTarget::Dt),
            "config" => Ok(PrintTarget::Config),
            _ => Err(format!(
                "Invalid print target. Valid options: boundary, d, repel_force, temperature, dt, config"
            )),
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "print")]
struct PrintCommand {
    /// What to print: boundary, d, repel_force, temperature, dt, or config
    target: PrintTarget,
}

fn print(mut log: ConsoleCommand<PrintCommand>, config: Res<ParticleConfig>) {
    if let Some(Ok(PrintCommand { target })) = log.take() {
        match target {
            PrintTarget::Boundary => {
                reply!(log, "map width: {:.2}, height: {:.2}", config.map_width, config.map_height);
            }
            PrintTarget::D => {
                reply!(log, "d1: {:.2}, d2: {:.2}, d3: {:.2}", config.d1, config.d2, config.d3);
            }
            PrintTarget::RepelForce => {
                reply!(log, "repel_force: {:.2}", config.repel_force);
            }
            PrintTarget::Temperature => {
                reply!(log, "temperature: {:.3}", config.temperature);
            }
            PrintTarget::Dt => {
                reply!(log, "dt: {:.3}", config.dt);
            }
            PrintTarget::Config => {
                reply!(
                    log,
                    "ParticleConfig:\n\
                     - init_particle_num: {}\n\
                     - map_width: {:.2}\n\
                     - map_height: {:.2}\n\
                     - d1 (collision): {:.2}\n\
                     - d2 (transition): {:.2}\n\
                     - d3 (max_distance): {:.2}\n\
                     - repel_force: {:.2}\n\
                     - temperature: {:.3}\n\
                     - dt: {:.3}",
                    config.init_particle_num,
                    config.map_width,
                    config.map_height,
                    config.d1,
                    config.d2,
                    config.d3,
                    config.repel_force,
                    config.temperature,
                    config.dt
                );
            }
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "respawn_particle")]
struct RespawnParticle;

fn respawn_particle(
    mut log: ConsoleCommand<RespawnParticle>,
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    query: Query<Entity, With<ParticleMarker>>,
    config: Res<ParticleConfig>,
) {
    if let Some(Ok(RespawnParticle)) = log.take() {
        clean_particle(commands.reborrow(), query);
        spawn_particle(commands, meshes, material, config);
        reply!(log, "Respawned all particles");
    }
}

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SetCommand, _>(set);
        app.add_console_command::<PrintCommand, _>(print);
        app.add_console_command::<RespawnParticle, _>(respawn_particle);
    }
}
