use crate::input_focus::*;
use bevy::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

const RED: Color = Color::hsl(360. * 0.0, 0.95, 0.7);
const BLUE: Color = Color::hsl(360. * 0.66, 0.95, 0.7);
const GREEN: Color = Color::hsl(360. * 0.33, 0.95, 0.7);
const DT: f32 = 0.01;

/// 粒子更新切换状态资源
#[derive(Resource, Default)]
struct ParticleUpdateToggle {
    enabled: bool,
}

/// 切换粒子更新状态（只在游戏焦点时响应）
fn toggle_particle_update(
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<ParticleUpdateToggle>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.is_game() && keys.just_pressed(KeyCode::KeyT) {
        toggle.enabled = !toggle.enabled;
        bevy::log::info!(
            "Particle update: {}",
            if toggle.enabled {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum ParticleType {
    #[default]
    Red = 0,
    Blue = 1,
    Green = 2,
}

impl ParticleType {
    pub const COUNT: usize = 3;

    pub fn all_types() -> [ParticleType; Self::COUNT] {
        [ParticleType::Red, ParticleType::Blue, ParticleType::Green]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ParticleType::Red => "Red",
            ParticleType::Blue => "Blue",
            ParticleType::Green => "Green",
        }
    }
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
                particle_type,
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

#[derive(Debug, Resource, Clone, Default)]
pub struct ParticleInteractionTable {
    interactions: [[f32; ParticleType::COUNT]; ParticleType::COUNT],
}

impl ParticleInteractionTable {
    pub fn new() -> Self {
        Self {
            interactions: [[0.0; ParticleType::COUNT]; ParticleType::COUNT],
        }
    }

    pub fn get_interaction(&self, target: ParticleType, source: ParticleType) -> f32 {
        self.interactions[target as usize][source as usize]
    }

    pub fn set_interaction(
        &mut self,
        target: ParticleType,
        source: ParticleType,
        acceleration: f32,
    ) {
        self.interactions[target as usize][source as usize] = acceleration;
    }

    pub fn from_csv_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut table = Self::new();

        let file = std::fs::File::open(path)?;
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);

        let mut headers: Vec<String> = Vec::new();
        let mut row_idx = 0;

        for result in rdr.records() {
            let record = result?;

            if headers.is_empty() {
                // First row: header row (,target,Red,Blue,Green)
                headers = record.iter().map(|s| s.to_string()).collect();
                println!("CSV Headers: {:?}", headers);
                println!("Header length: {}", headers.len());
                continue;
            }

            // Parse subsequent rows
            // Format: source,target_val0,target_val1,target_val2
            println!(
                "Row {}: {} columns, expected {}",
                row_idx,
                record.len(),
                headers.len()
            );

            if record.len() < headers.len() {
                eprintln!("Warning: Row {} has fewer columns than expected", row_idx);
                row_idx += 1;
                continue;
            }

            // First column is the source particle type
            let source_str = record.get(0).ok_or("Missing source column")?;
            let source_type = ParticleType::from_str(source_str)?;

            // Remaining columns are the target values
            for (col_idx, target_str) in headers.iter().skip(1).enumerate() {
                if col_idx >= ParticleType::COUNT {
                    break;
                }

                let value_str = record.get(col_idx + 1).ok_or("Missing value")?;
                let value: f32 = value_str.parse()?;

                let target_type = ParticleType::from_str(target_str)?;

                let target_idx = target_type as usize;
                let source_idx = source_type as usize;

                table.interactions[target_idx][source_idx] = value;

                println!(
                    "Loaded: {}[{}] <- {}[{}] = {:.1}",
                    target_str, target_idx, source_str, source_idx, value
                );
            }

            row_idx += 1;
        }

        println!("\nLoaded interaction table from {}:", path);
        table.print_table();

        Ok(table)
    }

    pub fn to_csv_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(path)?;

        // Write header row: ,target,Red,Blue,Green
        let mut header: Vec<String> = vec![String::new(), String::from("target")];
        for particle_type in ParticleType::all_types() {
            header.push(particle_type.as_str().to_string());
        }
        wtr.write_record(&header)?;

        // Write data rows: source,value0,value1,value2
        for source in ParticleType::all_types() {
            let mut row: Vec<String> = vec![source.as_str().to_string()];
            for target in ParticleType::all_types() {
                let value = self.get_interaction(target, source);
                row.push(value.to_string());
            }
            wtr.write_record(&row)?;
        }

        wtr.flush()?;
        println!("Saved interaction table to {}", path);
        Ok(())
    }

    pub fn print_table(&self) {
        println!(
            "       {:>8} {:>8} {:>8}",
            ParticleType::Red.as_str(),
            ParticleType::Blue.as_str(),
            ParticleType::Green.as_str()
        );
        println!(
            "       {:>8} {:>8} {:>8}",
            "--------", "--------", "--------"
        );

        for target in ParticleType::all_types() {
            print!("{:<6} |", target.as_str());
            for source in ParticleType::all_types() {
                let value = self.get_interaction(target, source);
                print!(" {:>8.1}", value);
            }
            println!();
        }
        println!();
    }

    pub fn as_matrix(&self) -> &[[f32; ParticleType::COUNT]; ParticleType::COUNT] {
        &self.interactions
    }

    pub fn as_matrix_mut(&mut self) -> &mut [[f32; ParticleType::COUNT]; ParticleType::COUNT] {
        &mut self.interactions
    }
}

#[derive(Debug, Resource, Clone)]
pub struct ParticleConfig {
    pub init_particle_num: usize,
    pub map_width: f32,
    pub map_height: f32,

    // ParticleInteractionDistanceLayer
    pub d1: f32,
    pub d2: f32,
    pub d3: f32,

    pub repel_force: f32,
    pub friction: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            init_particle_num: 1000,
            map_width: 1000.0,
            map_height: 1000.0,

            d1: 30.0,
            d2: 65.0,
            d3: 100.0,

            repel_force: -100.0,
            friction: 0.1,
        }
    }
}

#[derive(Debug, Default)]
pub struct ParticlePlugin {
    pub config: ParticleConfig,
}

fn update_particle(
    query: Query<(&mut Particle, &mut Transform), With<Mesh2d>>,
    interaction_table: Res<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    let mut chunk: HashMap<(i32, i32), Vec<(Particle, Transform)>> = HashMap::with_capacity(1000);
    for (p, t) in query.iter() {
        let x = (t.translation.x / config.d3) as i32;
        let y = (t.translation.y / config.d3) as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push((p.to_owned(), t.to_owned())))
            .or_insert([(p.to_owned(), t.to_owned())].into());
    }

    for (mut particle, mut transform) in query {
        let my_type = particle.particle_type;

        let chunk_x = (transform.translation.x / config.d3) as i32;
        let chunk_y = (transform.translation.y / config.d3) as i32;

        let mut components: Vec<(Particle, Transform)> = Vec::with_capacity(1000);
        for x in chunk_x - 1..=chunk_x + 1 {
            for y in chunk_y - 1..=chunk_y + 1 {
                chunk
                    .entry((x, y))
                    .and_modify(|inner| components.append(inner.to_owned().as_mut()));
            }
        }

        let acceleration = components.iter().filter(|(p, _)| p.id != particle.id).fold(
            Vec3::default(),
            |acc, (p, t)| {
                let distance = transform.translation.distance(t.translation);
                let direction = (t.translation - transform.translation) / distance;

                if distance < config.d1 {
                    let actual_acceleration =
                        direction * config.repel_force * (config.d1 - distance);
                    return acc + actual_acceleration;
                } else if distance >= config.d3 {
                    return acc;
                }
                let distance_factor = if distance >= config.d2 {
                    (config.d3 - distance) / (config.d3 - config.d2)
                } else {
                    (distance - config.d1) / config.d1
                };

                let other_type = p.particle_type;
                let strength = interaction_table.get_interaction(my_type, other_type);
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            },
        );

        particle.velocity += acceleration * DT;
        particle.velocity *= config.friction.powf(DT);

        transform.translation += particle.velocity * DT;

        let half_width = config.map_width / 2.0;
        let half_height = config.map_height / 2.0;

        if transform.translation.x < -half_width {
            transform.translation.x = -half_width;
            particle.velocity.x *= -1.0;
        } else if transform.translation.x > half_width {
            transform.translation.x = half_width;
            particle.velocity.x *= -1.0;
        } else if transform.translation.y < -half_height {
            transform.translation.y = -half_height;
            particle.velocity.y *= -1.0;
        } else if transform.translation.y > half_height {
            transform.translation.y = half_height;
            particle.velocity.y *= -1.0;
        }
    }
}

pub fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    config: Res<ParticleConfig>,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let particle_types = [ParticleType::Red, ParticleType::Blue, ParticleType::Green];

    for i in 0..config.init_particle_num {
        // 随机位置
        let x = rng.gen_range(-config.map_width / 2.0..config.map_width / 2.0);
        let y = rng.gen_range(-config.map_height / 2.0..config.map_height / 2.0);

        // 随机粒子类型
        let particle_type = particle_types[rng.gen_range(0..particle_types.len())];

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

pub fn clean_particle(mut commands: Commands, query: Query<Entity, With<Particle>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    bevy::log::info!("Cleaned all particles");
}

fn setup(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    material: ResMut<Assets<ColorMaterial>>,
    mut interaction_table: ResMut<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    let csv_path = "particle_interactions.csv";
    match ParticleInteractionTable::from_csv_file(csv_path) {
        Ok(loaded_table) => {
            *interaction_table = loaded_table;
            println!(
                "Successfully loaded particle interactions from {}",
                csv_path
            );
        }
        Err(e) => {
            println!("Could not load {}, using default interactions", csv_path);
            println!("Error: {}", e);
        }
    }

    spawn_particle(commands, meshes, material, config)
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.to_owned());
        app.insert_resource(ParticleUpdateToggle::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_particle_update);
        app.add_systems(
            Update,
            update_particle.run_if(|toggle: Res<ParticleUpdateToggle>| toggle.enabled),
        );
    }
}
