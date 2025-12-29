use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::sprite_render::*;
use serde::Deserialize;
use std::collections::BTreeMap;

const RED: Color = Color::hsl(360. * 0.0, 0.95, 0.7);
const BLUE: Color = Color::hsl(360. * 0.66, 0.95, 0.7);
const GREEN: Color = Color::hsl(360. * 0.33, 0.95, 0.7);

#[derive(Component, Default, Clone)]
struct Particle {
    id: usize,
    velocity: Vec3,
    particle_type: ParticleType,
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum ParticleType {
    #[default]
    Red,
    Blue,
    Green,
}

impl ParticleType {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "red" => Some(ParticleType::Red),
            "blue" => Some(ParticleType::Blue),
            "green" => Some(ParticleType::Green),
            _ => None,
        }
    }
}

#[derive(Debug, Resource, Clone)]
struct ParticleInteractionTable {
    // BTreeMap<target_type, BTreeMap<source_type, acceleration>>
    interactions: BTreeMap<ParticleType, BTreeMap<ParticleType, f32>>,
}

impl ParticleInteractionTable {
    fn new() -> Self {
        let mut interactions: BTreeMap<ParticleType, BTreeMap<ParticleType, f32>> = BTreeMap::new();

        let particle_types = vec![ParticleType::Red, ParticleType::Blue, ParticleType::Green];

        for target in &particle_types {
            let mut inner_map: BTreeMap<ParticleType, f32> = BTreeMap::new();
            for source in &particle_types {
                inner_map.insert(source.clone(), 0.0);
            }
            interactions.insert(target.clone(), inner_map);
        }

        ParticleInteractionTable { interactions }
    }

    fn get_interaction(&self, target: ParticleType, source: ParticleType) -> f32 {
        self.interactions
            .get(&target)
            .and_then(|inner| inner.get(&source))
            .copied()
            .unwrap_or(0.0)
    }

    fn set_interaction(&mut self, target: ParticleType, source: ParticleType, acceleration: f32) {
        if let Some(inner) = self.interactions.get_mut(&target) {
            inner.insert(source, acceleration);
        }
    }

    fn from_csv_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut table = ParticleInteractionTable::new();

        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        #[derive(Deserialize)]
        struct InteractionRecord {
            target: String,
            source: String,
            strength: f32,
        }

        for result in rdr.deserialize() {
            let record: InteractionRecord = result?;

            let target_type = ParticleType::from_str(&record.target)
                .ok_or_else(|| format!("Invalid target particle type: {}", record.target))?;

            let source_type = ParticleType::from_str(&record.source)
                .ok_or_else(|| format!("Invalid source particle type: {}", record.source))?;

            let acceleration = record.strength;

            table.set_interaction(target_type, source_type, acceleration);
        }

        Ok(table)
    }
}

impl Particle {
    fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &mut ResMut<Assets<ColorMaterial>>,
        transform: Transform,
        particle_type: ParticleType,
        id: usize,
    ) {
        commands.spawn((
            Particle {
                id,
                velocity: Vec3::default(),
                particle_type: particle_type.clone(),
            },
            Mesh2d(meshes.add(Circle::new(5.0))),
            match particle_type {
                ParticleType::Red => MeshMaterial2d(material.add(RED)),
                ParticleType::Blue => MeshMaterial2d(material.add(BLUE)),
                ParticleType::Green => MeshMaterial2d(material.add(GREEN)),
            },
            transform,
        ));
    }
}

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
    let map_width = 800.0; // 地图宽度
    let map_height = 600.0; // 地图高度

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
}

fn update_particle(
    query: Query<(&mut Particle, &mut Transform), With<Mesh2d>>,
    interaction_table: Res<ParticleInteractionTable>,
    time: Res<Time>,
) {
    let components_copy = query
        .iter()
        .map(|(p, t)| (p.to_owned(), t.to_owned()))
        .collect::<Vec<(Particle, Transform)>>();

    for (mut particle, mut transform) in query {
        let my_type = particle.particle_type.clone();

        let acceleration = components_copy
            .iter()
            .filter(|(p, _)| p.id != particle.id)
            .fold(Vec3::default(), |acc, (p, t)| {
                let other_type = p.particle_type.clone();
                let interaction =
                    interaction_table.get_interaction(my_type.clone(), other_type.clone());

                let distance = transform.translation.distance(t.translation);
                let direction = (t.translation - transform.translation) / distance;

                let strength = interaction;

                let d = 20.0;

                let distance_factor = if distance >= 3.0 * d {
                    0.0
                } else if distance >= 2.0 * d {
                    (3.0 * d - distance) / d
                } else if distance > d {
                    (distance - d) / d
                } else {
                    0.0
                };

                let actual_acceleration = if distance > d {
                    direction * strength * distance_factor
                } else {
                    direction * -0.5 * (d - distance)
                };

                acc + actual_acceleration
            });

        // 更新速度：v = v + a * dt
        particle.velocity += acceleration * time.delta_secs();
        particle.velocity *= 0.99;

        // 更新位置：p = p + v * dt
        transform.translation += particle.velocity * time.delta_secs();
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Wireframe2dPlugin::default()))
        .insert_resource(ParticleInteractionTable::new())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (update_particle.run_if(input_toggle_active(true, KeyCode::KeyT)),),
        )
        .run();
}
