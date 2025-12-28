use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::sprite_render::*;

const fn sqrt_const(x: f32) -> f32 {
    if x >= 0.0 {
        let mut result = x;
        let mut i = 0;
        while i < 1000000 {
            result = 0.5 * (result + x / result);
            i += 1;
        }

        result
    } else {
        0.0
    }
}

const RED: Color = Color::hsl(360. * 2.0, 0.95, 0.7);
const GRAVITATION: f32 = 6.674e-11;
const MASS: f32 = 1000000000000000.0 * 100.0;
const RADIUS: f32 = 50.0;
const VELOCITY: f32 = sqrt_const(GRAVITATION * MASS / (4.0 * RADIUS));

#[derive(Component, Default, Clone)]
struct Planet {
    id: usize,
    velocity: Vec3,
    accelerate: Vec3,
    mass: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Planet {
            id: 0,
            velocity: Vec3::new(0.0, VELOCITY, 0.0),
            accelerate: Vec3::default(),
            mass: MASS,
        },
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(material.add(RED)),
        Transform::from_xyz(-RADIUS, 0.0, 0.0),
    ));

    commands.spawn((
        Planet {
            id: 1,
            velocity: Vec3::new(0.0, -VELOCITY, 0.0),
            accelerate: Vec3::default(),
            mass: MASS,
        },
        Mesh2d(meshes.add(Circle::new(10.0))),
        MeshMaterial2d(material.add(RED)),
        Transform::from_xyz(RADIUS, 0.0, 0.0),
    ));
}

fn update_planet(query: Query<(&mut Planet, &mut Transform), With<(Mesh2d)>>, time: Res<Time>) {
    let components_copy = query
        .iter()
        .map(|(p, t)| (p.to_owned(), t.to_owned()))
        .collect::<Vec<(Planet, Transform)>>();

    for (mut planet, mut transform) in query {
        let accelerate_res = components_copy
            .iter()
            .filter(|(p, _)| planet.id != p.id)
            .fold(Vec3::default(), |accelerate_res, (p, t)| {
                let distance = transform.translation.distance(t.translation);
                let d = t.translation - transform.translation;
                let accelerate_direction = d / d.length();
                println!(
                    "id:{},distance:{}",
                    planet.id,
                    transform.translation.distance(t.translation)
                );
                accelerate_res + accelerate_direction * GRAVITATION * p.mass / (distance * distance)
            });

        planet.accelerate = accelerate_res;

        let a = planet.accelerate.to_owned();
        planet.velocity += a * time.delta_secs();
        let v = planet.velocity.to_owned();
        transform.translation += v * time.delta_secs();
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            update_planet.run_if(input_toggle_active(true, KeyCode::KeyT)),
        )
        .run();
}
