use bevy::color::palettes::tailwind::*;
use bevy::input::keyboard::KeyCode;
use bevy::input_focus::{InputDispatchPlugin, *};
use bevy::log::*;
use bevy::prelude::*;
use bevy_console::*;
use clap::Parser;
use rand::*;

#[derive(Debug, Default, Component, Clone)]
pub struct Velocity {
    pub vec: Vec3,
}

#[derive(Debug, Default, Bundle)]
pub struct Player {
    pub velocity: Velocity,
    pub transform: Transform,
    mesh2d: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
}

impl Player {
    pub fn spawn(
        command: &mut Commands,
        meshs: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        velocity: Velocity,
        transform: Transform,
    ) {
        command.spawn(Player {
            velocity,
            transform,
            mesh2d: Mesh2d(meshs.add(Circle::new(10.0))),
            material: MeshMaterial2d(materials.add(ColorMaterial::from_color(RED_500))),
        });
    }
}

fn setup(
    mut command: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn(Camera2d);
    for _ in 0..10 {
        Player::spawn(
            &mut command,
            &mut meshs,
            &mut materials,
            Velocity::default(),
            Transform::from_xyz(
                random_range(-100.0..100.0),
                random_range(-100.0..100.0),
                random_range(-100.0..100.0),
            ),
        )
    }
}

fn print_player(query: Query<(Entity, &Transform), With<Mesh2d>>) {
    for (entity, transform) in query {
        debug!("{}: {}", entity.index(), transform.translation);
    }
    debug!("end")
}

fn despawn_player(mut command: Commands, query: Query<Entity, With<Mesh2d>>) {
    for entity in query {
        if entity.index() % 2 == 0 {
            command.entity(entity).despawn();
        }
    }
}

/// Example command
#[derive(Parser, ConsoleCommand)]
#[command(name = "example")]
struct ExampleCommand {
    /// Some message
    msg: String,
}

fn example_command(mut log: ConsoleCommand<ExampleCommand>) {
    if let Some(Ok(ExampleCommand { msg })) = log.take() {
        // handle command
        reply!(log, "{}", msg);
    }
}

fn test_input_focus(focus: ResMut<InputFocus>) {
    if let Some(entity) = focus.get() {
        info!("the focus is {}", entity);
    }
}

// 按下空格键时切换焦点到下一个玩家
fn switch_focus_on_space(
    mut focus: ResMut<InputFocus>,
    keyboard: Res<ButtonInput<KeyCode>>,
    players: Query<Entity, With<Mesh2d>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let player_entities: Vec<Entity> = players.iter().collect();
        if player_entities.is_empty() {
            return;
        }

        if let Some(current_focus) = focus.get() {
            // 找到当前焦点在玩家列表中的位置
            if let Some(current_index) = player_entities.iter().position(|&e| e == current_focus) {
                // 切换到下一个玩家
                let next_index = (current_index + 1) % player_entities.len();
                focus.set(player_entities[next_index]);
                info!("Switched focus to {}", player_entities[next_index]);
            } else {
                // 如果当前焦点不是玩家，则聚焦到第一个玩家
                focus.set(player_entities[0]);
                info!("Set focus to first player: {}", player_entities[0]);
            }
        } else {
            // 如果没有焦点，聚焦到第一个玩家
            focus.set(player_entities[0]);
            info!("Set focus to first player: {}", player_entities[0]);
        }
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ConsolePlugin, InputDispatchPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, print_player)
        .add_systems(Update, despawn_player)
        .add_systems(Update, test_input_focus)
        .add_systems(Update, switch_focus_on_space)
        .add_console_command::<ExampleCommand, _>(example_command)
        .run();
}
