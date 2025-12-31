use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use serde::Deserialize;
use std::collections::BTreeMap;
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

#[derive(Component, Default, Clone)]
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

            let target_type = ParticleType::from_str(&record.target).unwrap();

            let source_type = ParticleType::from_str(&record.source).unwrap();

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

pub struct ParticlePlugin;

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

                let d1 = 20.0;
                let d2 = 50.0;
                let d3 = 100.0;

                let distance_factor = if distance >= d3 {
                    0.0
                } else if distance >= d2 {
                    (d3 - distance) / (d3 - d2)
                } else if distance > d1 {
                    (distance - d1) / d1
                } else {
                    0.0
                };

                let actual_acceleration = if distance > d1 {
                    direction * strength * distance_factor
                } else {
                    direction * -0.5 * (d1 - distance)
                };

                acc + actual_acceleration
            });

        particle.velocity += acceleration * time.delta_secs();
        particle.velocity *= 0.995;

        transform.translation += particle.velocity * time.delta_secs();
    }
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_particle.run_if(input_toggle_active(true, KeyCode::KeyT)),
        );
    }
}
