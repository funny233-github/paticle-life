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
            Mesh2d(meshes.add(Circle::new(10.0))),
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

    // 创建多个粒子用于测试
    let particle_positions = [
        (0.0, 0.0),
        (100.0, 0.0),
        (-100.0, 0.0),
        (0.0, 100.0),
        (0.0, -100.0),
    ];

    let particle_types = [ParticleType::Red, ParticleType::Blue, ParticleType::Green];

    for (i, (x, y)) in particle_positions.iter().enumerate() {
        Particle::spawn(
            &mut commands,
            &mut meshes,
            &mut material,
            Transform::from_xyz(*x, *y, 0.0),
            particle_types[i % particle_types.len()].clone(),
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

                // 计算距离和方向（指向对方）
                let distance = transform.translation.distance(t.translation);
                let direction = (t.translation - transform.translation) / distance;

                // 读取相互作用向量中的第一个分量作为作用强度
                // 正值表示吸引（沿着方向），负值表示排斥（反向）
                let strength = interaction;

                // 距离衰减因子（类似引力的平方反比或线性反比）
                let distance_factor = if distance > 200.0 {
                    0.0
                } else {
                    (200.0 - distance) / 200.0
                };

                // 计算实际加速度：强度 * 距离衰减 * 方向
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            });

        // 更新速度：v = v + a * dt
        particle.velocity += acceleration * time.delta_secs();

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
