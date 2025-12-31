use bevy::prelude::*;
use bevy::sprite_render::*;
use bevy_game_test::particle::*;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
) {
    // 尝试从文件加载相互作用表，如果失败则使用默认值
    let csv_path = "particle_interactions.csv";
    if let Ok(loaded_table) = ParticleInteractionTable::from_csv_file(csv_path) {
        *interaction_table = loaded_table;
        println!(
            "Successfully loaded particle interactions from {}",
            csv_path
        );
    } else {
        println!("Could not load {}, using default interactions", csv_path);
    }

    // 随机生成粒子
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let particle_types = [ParticleType::Red, ParticleType::Blue];
    let num_particles = 1000; // 粒子数量
    let map_width = 1000.0; // 地图宽度
    let map_height = 1000.0; // 地图高度

    for i in 0..num_particles {
        // 随机位置
        let x = rng.gen_range(-map_width / 2.0..map_width / 2.0);
        let y = rng.gen_range(-map_height / 2.0..map_height / 2.0);

        // 随机粒子类型
        let particle_type = particle_types[rng.gen_range(0..particle_types.len())].clone();

        Particle::spawn(
            &mut commands,
            &mut meshes,
            &mut material,
            Transform::from_xyz(x, y, 0.0),
            particle_type,
            i,
        );
    }

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
    transform.scale = transform.scale.clamp(Vec3::splat(0.1), Vec3::splat(5.0));
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default(), ParticlePlugin))
        .insert_resource(ParticleInteractionTable::new())
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera,))
        .run();
}
