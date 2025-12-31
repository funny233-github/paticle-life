use bevy::prelude::*;
use bevy::sprite_render::*;
use bevy_game_test::particle::*;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn((
        Text::new("Press T to toggle update"),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

fn move_camera(
    mut camera: Query<(&mut Transform, &Camera), With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, _camera)) = camera.single_mut() else {
        return;
    };
    let mut direction = Vec3::ZERO;

    let speed = 400.0;
    let zoom_speed = 1.0;
    let current_scale = transform.scale;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        transform.translation += direction.normalize() * speed * current_scale * time.delta_secs();
    }

    // 缩放控制 - 通过改变摄像机的 scale
    if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadAdd) {
        transform.scale *= 1.0 + zoom_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadSubtract) {
        transform.scale *= 1.0 - zoom_speed * time.delta_secs();
    }

    // 限制缩放范围
    transform.scale = transform.scale.clamp(Vec3::splat(0.01), Vec3::splat(5.0));
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default(),
            ParticlePlugin {
                setup_config: ParticleSetupConfig {
                    num_particles: 5000,
                    map_width: 1000.0,
                    map_height: 1000.0,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .insert_resource(ParticleInteractionTable::new())
        .add_systems(Startup, setup)
        .add_systems(Update, move_camera)
        .run();
}
