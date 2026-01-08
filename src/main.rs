use bevy::app::{App, Startup, Update};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::sprite_render::Wireframe2dPlugin;
use bevy_console::{ConsoleConfiguration, ConsolePlugin};
use bevy_game_test::{
    CameraMovePlugin, CommandPlugin, InputFocusPlugin, ParticlePlugin, resources::*,
};

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new(
            "Press ` (backtick) to toggle console\nGame: T=toggle update, WASD=move, -/+=zoom\nConsole: Type commands and press Enter",
        ),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn update_fps(mut query: Query<&mut Text>, diagnostics: Res<DiagnosticsStore>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            text.0 = format!(
                "FPS: {:.1}\nPress ` (backtick) to toggle console\nGame: T=toggle update, R=respawn particle, WASD=move, -/+=zoom\nConsole: Type commands and press Enter",
                value
            );
        }
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
            ConsolePlugin,
            InputFocusPlugin,
            CommandPlugin,
            CameraMovePlugin,
            ParticlePlugin {
                config: ParticleConfig {
                    init_particle_num: 6000,
                    map_width: 7608.0,
                    map_height: 3909.0,

                    ..Default::default()
                },
            },
        ))
        .insert_resource(ParticleInteractionTable::new())
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update_fps)
        .run();
}
