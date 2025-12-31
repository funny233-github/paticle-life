use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

const RED: Color = Color::hsl(360. * 0.0, 0.95, 0.7);
const BLUE: Color = Color::hsl(360. * 0.66, 0.95, 0.7);
const GREEN: Color = Color::hsl(360. * 0.33, 0.95, 0.7);

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum ParticleType {
    #[default]
    Red,
    Blue,
    Green,
}

#[derive(Debug)]
pub struct ParticleTypeError;

impl Display for ParticleTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid particle type. Expected one of: red, blue, or green"
        )
    }
}

impl Error for ParticleTypeError {}

impl FromStr for ParticleType {
    type Err = ParticleTypeError;

    fn from_str(s: &str) -> Result<Self, ParticleTypeError> {
        match s.to_lowercase().as_str() {
            "red" => Ok(ParticleType::Red),
            "blue" => Ok(ParticleType::Blue),
            "green" => Ok(ParticleType::Green),
            _ => Err(ParticleTypeError),
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct Particle {
    pub id: usize,
    pub velocity: Vec3,
    pub particle_type: ParticleType,
}

impl Particle {
    pub fn spawn(
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

#[derive(Debug, Resource, Clone)]
pub struct ParticleInteractionTable {
    interactions: BTreeMap<ParticleType, BTreeMap<ParticleType, f32>>,
}

impl ParticleInteractionTable {
    pub fn new() -> Self {
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

    pub fn get_interaction(&self, target: ParticleType, source: ParticleType) -> f32 {
        self.interactions
            .get(&target)
            .and_then(|inner| inner.get(&source))
            .copied()
            .unwrap_or(0.0)
    }

    pub fn set_interaction(
        &mut self,
        target: ParticleType,
        source: ParticleType,
        acceleration: f32,
    ) {
        if let Some(inner) = self.interactions.get_mut(&target) {
            inner.insert(source, acceleration);
        }
    }

    pub fn from_csv_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
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

            let target_type = ParticleType::from_str(&record.target)?;

            let source_type = ParticleType::from_str(&record.source)?;

            let acceleration = record.strength;

            table.set_interaction(target_type, source_type, acceleration);
        }

        Ok(table)
    }
}

impl Default for ParticleInteractionTable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Resource, Clone)]
pub struct ParticleSetupConfig {
    pub num_particles: usize,
    pub map_width: f32,
    pub map_height: f32,
}

impl Default for ParticleSetupConfig {
    fn default() -> Self {
        Self {
            num_particles: 1000,
            map_width: 1000.0,
            map_height: 1000.0,
        }
    }
}

#[derive(Debug, Resource, Clone)]
pub struct ParticleInteractionDistanceLayer {
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,
}

impl Default for ParticleInteractionDistanceLayer {
    fn default() -> Self {
        Self {
            d1: 30.0,
            d2: 65.0,
            d3: 100.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct ParticlePlugin {
    pub setup_config: ParticleSetupConfig,
    pub interaction_distance_layer: ParticleInteractionDistanceLayer,
}

fn update_particle(
    query: Query<(&mut Particle, &mut Transform), With<Mesh2d>>,
    interaction_table: Res<ParticleInteractionTable>,
    layer: Res<ParticleInteractionDistanceLayer>,
    time: Res<Time>,
) {
    let mut chunk: HashMap<(i32, i32), Vec<(Particle, Transform)>> = HashMap::new();
    for (p, t) in query.iter() {
        let x = (t.translation.x / layer.d3) as i32;
        let y = (t.translation.y / layer.d3) as i32;
        if let None = chunk.get(&(x, y)) {
            chunk.insert((x, y), Vec::default());
        }
        if let Some(inner) = chunk.get_mut(&(x, y)) {
            inner.push((p.to_owned(), t.to_owned()));
        }
    }

    for (mut particle, mut transform) in query {
        let my_type = particle.particle_type.clone();

        let chunk_x = (transform.translation.x / layer.d3) as i32;
        let chunk_y = (transform.translation.y / layer.d3) as i32;

        let iter_x = [chunk_x - 1, chunk_x, chunk_x + 1];
        let iter_y = [chunk_y - 1, chunk_y, chunk_y + 1];

        let mut components: Vec<(Particle, Transform)> = Vec::new();

        for x in iter_x {
            for y in iter_y {
                if let Some(inner) = chunk.get(&(x, y)) {
                    components.append(inner.to_owned().as_mut())
                }
            }
        }

        let acceleration = components.iter().filter(|(p, _)| p.id != particle.id).fold(
            Vec3::default(),
            |acc, (p, t)| {
                let other_type = p.particle_type.clone();
                let interaction =
                    interaction_table.get_interaction(my_type.clone(), other_type.clone());

                let distance = transform.translation.distance(t.translation);
                let direction = (t.translation - transform.translation) / distance;

                let strength = interaction;

                let distance_factor = if distance >= layer.d3 {
                    0.0
                } else if distance >= layer.d2 {
                    (layer.d3 - distance) / (layer.d3 - layer.d2)
                } else if distance > layer.d1 {
                    (distance - layer.d1) / layer.d1
                } else {
                    0.0
                };

                let actual_acceleration = if distance > layer.d1 {
                    direction * strength * distance_factor
                } else {
                    direction * -100.0 * (layer.d1 - distance)
                };

                acc + actual_acceleration
            },
        );

        particle.velocity += acceleration * time.delta_secs();
        particle.velocity *= 0.995;

        transform.translation += particle.velocity * time.delta_secs();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
    config: Res<ParticleSetupConfig>,
) {
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

    for i in 0..config.num_particles {
        // 随机位置
        let x = rng.gen_range(-config.map_width / 2.0..config.map_width / 2.0);
        let y = rng.gen_range(-config.map_height / 2.0..config.map_height / 2.0);

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
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.setup_config.to_owned());
        app.insert_resource(self.interaction_distance_layer.to_owned());
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            update_particle.run_if(input_toggle_active(true, KeyCode::KeyT)),
        );
    }
}
