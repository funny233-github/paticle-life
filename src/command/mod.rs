use crate::particle::*;
use bevy::prelude::*;
use bevy_console::*;
use clap::Parser;

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_boundary")]
struct SetBoundary {
    /// Some message
    width: f32,
    height: f32,
}

fn set_boundary(mut log: ConsoleCommand<SetBoundary>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetBoundary { width, height })) = log.take() {
        // handle command
        config.map_width = width;
        config.map_height = height;
        reply!(log, "set map width: {width}, height: {height} successfully");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_d1")]
struct SetD1 {
    /// Set the d1 distance value (collision distance)
    value: f32,
}

fn set_d1(mut log: ConsoleCommand<SetD1>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetD1 { value })) = log.take() {
        config.d1 = value;
        reply!(log, "set d1 to {value:.2} successfully");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_d2")]
struct SetD2 {
    /// Set the d2 distance value (interaction transition start)
    value: f32,
}

fn set_d2(mut log: ConsoleCommand<SetD2>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetD2 { value })) = log.take() {
        config.d2 = value;
        reply!(log, "set d2 to {value:.2} successfully");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_d3")]
struct SetD3 {
    /// Set the d3 distance value (interaction max distance and chunk size)
    value: f32,
}

fn set_d3(mut log: ConsoleCommand<SetD3>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetD3 { value })) = log.take() {
        config.d3 = value;
        reply!(log, "set d3 to {value:.2} successfully");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_repel_force")]
struct SetRepelForce {
    /// Set the repel force magnitude for collision
    value: f32,
}

fn set_repel_force(mut log: ConsoleCommand<SetRepelForce>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetRepelForce { value })) = log.take() {
        config.repel_force = value;
        reply!(log, "set repel_force to {value:.2} successfully");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_friction")]
struct SetFriction {
    /// Set the friction coefficient for velocity damping
    value: f32,
}

fn set_friction(mut log: ConsoleCommand<SetFriction>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetFriction { value })) = log.take() {
        config.friction = value;
        reply!(log, "set friction to {value:.3} successfully");
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PrintTarget {
    Boundary,
    D,
    RepelForce,
    Friction,
    Config,
}

impl std::str::FromStr for PrintTarget {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "boundary" => Ok(PrintTarget::Boundary),
            "d" => Ok(PrintTarget::D),
            "repel_force" => Ok(PrintTarget::RepelForce),
            "friction" => Ok(PrintTarget::Friction),
            "config" => Ok(PrintTarget::Config),
            _ => Err(format!(
                "Invalid print target. Valid options: boundary, d, repel_force, friction, config"
            )),
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "print")]
struct PrintCommand {
    /// What to print: boundary, d, repel_force, friction, or config
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
            PrintTarget::Friction => {
                reply!(log, "friction: {:.3}", config.friction);
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
                     - friction: {:.3}",
                    config.init_particle_num,
                    config.map_width,
                    config.map_height,
                    config.d1,
                    config.d2,
                    config.d3,
                    config.repel_force,
                    config.friction
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
    query: Query<Entity, With<Particle>>,
    config: Res<ParticleConfig>,
) {
    if let Some(Ok(RespawnParticle)) = log.take() {
        clean_particle(commands.reborrow(), query);
        spawn_particle(commands, meshes, material, config);
        reply!(log, "Respawned all particles");
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "set_init_particle_num")]
struct SetInitParticleNum {
    /// Set the initial number of particles to spawn
    value: usize,
}

fn set_init_particle_num(mut log: ConsoleCommand<SetInitParticleNum>, mut config: ResMut<ParticleConfig>) {
    if let Some(Ok(SetInitParticleNum { value })) = log.take() {
        config.init_particle_num = value;
        reply!(log, "set init_particle_num to {} successfully", value);
    }
}

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<SetBoundary, _>(set_boundary);

        app.add_console_command::<SetD1, _>(set_d1);
        app.add_console_command::<SetD2, _>(set_d2);
        app.add_console_command::<SetD3, _>(set_d3);

        app.add_console_command::<SetRepelForce, _>(set_repel_force);
        app.add_console_command::<SetFriction, _>(set_friction);

        app.add_console_command::<PrintCommand, _>(print);
        app.add_console_command::<RespawnParticle, _>(respawn_particle);
        app.add_console_command::<SetInitParticleNum, _>(set_init_particle_num);
    }
}
