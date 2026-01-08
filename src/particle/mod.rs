//! Particle simulation system with physics and rendering
//!
//! This module provides a complete particle simulation system with:
//! - Separated physics (`Position`) and rendering (`Transform`) updates
//! - Configurable particle interaction tables
//! - Spatial partitioning for efficient neighbor queries
//! - Boundary collision handling
//! - Console commands for runtime configuration
//!
//! # Architecture
//!
//! The particle system is designed with clear separation of concerns:
//!
//! 1. **Physics Update** (`update_particle`): Computes particle motion,
//!    interactions, and boundary collisions using `Position` component.
//!
//! 2. **Render Sync** (`sync_transform`): Updates the `Transform` component
//!    to match `Position`, ensuring the rendering system displays
//!    particles at their correct locations.
//!
//! This separation allows the physics simulation to be paused or updated
//! independently from rendering, and makes the code easier to reason about.
//!
//! # Particle Types
//!
//! Particles have three types: Red, Blue, and Green. Each type can have
//! different interaction forces with every other type (including itself).
//! These interactions are stored in a [`ParticleInteractionTable`].
//!
//! # Distance Zones
//!
//! The particle interaction system uses three distance parameters:
//!
//! - **d1**: Collision distance. Particles closer than this experience
//!   strong repulsion.
//!
//! - **d2**: Interaction transition start. Between d1 and d2, forces
//!   increase linearly from 0.
//!
//! - **d3**: Maximum interaction distance. Beyond this, particles don't
//!   interact. Also used as spatial partition chunk size.

use crate::input_focus::InputFocus;
use bevy::color::palettes::tailwind::{RED_500, BLUE_500, GREEN_500};
use bevy::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// Type alias for particle chunk data in spatial partitioning
type ParticleChunk = Vec<(Entity, ParticleType, Position)>;

#[derive(Resource, Default)]
struct ParticleUpdateToggle {
    enabled: bool,
}

#[allow(clippy::needless_pass_by_value)]
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

/// Type of particle in the simulation
///
/// Each particle type can have different interaction forces with
/// every other particle type.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(usize)]
pub enum ParticleType {
    /// Red particle type
    #[default]
    Red = 0,
    /// Blue particle type
    Blue = 1,
    /// Green particle type
    Green = 2,
}

impl ParticleType {
    /// Total number of particle types
    pub const COUNT: usize = 3;

    /// Returns an array containing all particle types
    #[must_use] 
    pub const fn all_types() -> [Self; Self::COUNT] {
        [Self::Red, Self::Blue, Self::Green]
    }

    /// Returns string representation of this particle type
    #[must_use] 
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Red => "Red",
            Self::Blue => "Blue",
            Self::Green => "Green",
        }
    }
}

/// Error returned when parsing an invalid particle type string
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
            "red" => Ok(Self::Red),
            "blue" => Ok(Self::Blue),
            "green" => Ok(Self::Green),
            _ => Err(ParticleTypeError),
        }
    }
}

/// Velocity component for particles
///
/// Stores the velocity vector for physics calculations.
/// This is separate from position to allow for clean physics updates.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Velocity {
    /// Velocity vector (units per second)
    pub value: Vec3,
}

impl Velocity {
    /// Creates a new velocity from a vector
    #[must_use] 
    pub const fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Creates a new velocity from x, y, z components
    #[must_use] 
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

/// Position component for particles
///
/// Stores the position vector for physics calculations.
/// This is separate from `Transform` to allow the physics system
/// to update positions independently from rendering.
///
/// The physics system updates `Position`, while `sync_transform`
/// copies it to `Transform` for rendering.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct Position {
    /// Position vector in world space
    pub value: Vec3,
}

impl Position {
    /// Creates a new position from a vector
    #[must_use] 
    pub const fn new(value: Vec3) -> Self {
        Self { value }
    }

    /// Creates a new position from x, y, z components
    #[must_use] 
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            value: Vec3::new(x, y, z),
        }
    }
}

/// Marker component for particles
///
/// Used in queries to identify entities that are particles.
#[derive(Component, Debug, Clone, Copy)]
pub struct ParticleMarker;

/// Bundle for spawning a particle entity
///
/// Contains all components needed for a particle:
/// - Particle type marker
/// - Particle type enum
/// - Velocity for physics
/// - Position for physics (separate from Transform)
/// - Mesh for rendering
/// - Material for rendering
/// - Transform for rendering
#[derive(Bundle, Debug, Clone)]
pub struct Particle {
    /// Marker component identifying this as a particle
    pub marker: ParticleMarker,
    /// Type of this particle
    pub particle_type: ParticleType,
    /// Velocity for physics updates
    pub velocity: Velocity,
    /// Position for physics (separate from Transform)
    pub position: Position,
    /// 2D mesh for rendering
    pub mesh: Mesh2d,
    /// Material for rendering
    pub material: MeshMaterial2d<ColorMaterial>,
    /// Transform for rendering (synced from Position)
    pub transform: Transform,
}

impl Particle {
    /// Spawns a new particle entity with given properties
    ///
    /// # Arguments
    /// - `commands`: Bevy command queue
    /// - `meshes`: Mesh assets resource
    /// - `material`: Material assets resource
    /// - `transform`: Initial transform (position will be copied to Position component)
    /// - `particle_type`: Type of particle to spawn
    pub fn spawn(
        commands: &mut Commands,
        meshes: &mut ResMut<Assets<Mesh>>,
        material: &mut ResMut<Assets<ColorMaterial>>,
        transform: Transform,
        particle_type: ParticleType,
    ) {
        commands.spawn(Self {
            marker: ParticleMarker,
            particle_type,
            velocity: Velocity::new(Vec3::default()),
            position: Position::new(transform.translation),
            mesh: Mesh2d(meshes.add(Circle::new(10.0))),
            material: match particle_type {
                ParticleType::Red => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(RED_500)))
                }
                ParticleType::Blue => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(BLUE_500)))
                }
                ParticleType::Green => {
                    MeshMaterial2d(material.add(ColorMaterial::from_color(GREEN_500)))
                }
            },
            transform,
        });
    }
}

/// Particle interaction table
///
/// Stores interaction forces between all pairs of particle types.
/// The table is indexed by [target][source], giving the force
/// that a source particle exerts on a target particle.
///
/// Positive values cause attraction, negative values cause repulsion.
#[derive(Debug, Resource, Clone, Default)]
pub struct ParticleInteractionTable {
    interactions: [[f32; ParticleType::COUNT]; ParticleType::COUNT],
}

impl ParticleInteractionTable {
    /// Creates a new interaction table with all zeros
    #[must_use] 
    pub const fn new() -> Self {
        Self {
            interactions: [[0.0; ParticleType::COUNT]; ParticleType::COUNT],
        }
    }

    /// Gets the interaction force between two particle types
    ///
    /// Returns the force that a source particle exerts on a target particle.
    #[must_use] 
    pub const fn get_interaction(&self, target: ParticleType, source: ParticleType) -> f32 {
        self.interactions[target as usize][source as usize]
    }

    /// Sets the interaction force between two particle types
    ///
    /// Sets the force that a source particle exerts on a target particle.
    pub const fn set_interaction(
        &mut self,
        target: ParticleType,
        source: ParticleType,
        acceleration: f32,
    ) {
        self.interactions[target as usize][source as usize] = acceleration;
    }

    /// Loads interaction table from a CSV file
    ///
    /// CSV format:
    /// - First row: headers (,target,Red,Blue,Green)
    /// - Subsequent rows: `source_type,red_val,blue_val,green_val`
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be opened
    /// - The CSV format is invalid
    /// - The data cannot be parsed correctly
    ///
    /// # Returns
    /// A new [`ParticleInteractionTable`] with loaded values
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
                headers = record.iter().map(|s: &str| s.to_string()).collect();
                bevy::log::info!("CSV Headers: {:?}", headers);
                bevy::log::info!("Header length: {}", headers.len());
                continue;
            }

            // Parse subsequent rows
            // Format: source,target_val0,target_val1,target_val2
            bevy::log::debug!(
                "Row {}: {} columns, expected {}",
                row_idx,
                record.len(),
                headers.len()
            );

            if record.len() < headers.len() {
                bevy::log::warn!("Warning: Row {} has fewer columns than expected", row_idx);
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

                bevy::log::debug!(
                    "Loaded: {}[{}] <- {}[{}] = {:.1}",
                    target_str,
                    target_idx,
                    source_str,
                    source_idx,
                    value
                );
            }

            row_idx += 1;
        }

        bevy::log::info!("\nLoaded interaction table from {}:", path);
        table.print_table();

        Ok(table)
    }

    /// Saves interaction table to a CSV file
    ///
    /// Writes the current interaction values to a CSV file that
    /// can be loaded later with [`from_csv_file`].
    ///
    /// # Errors
    /// Returns an error if:
    /// - The file cannot be created or written to
    /// - The data cannot be serialized to CSV
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
        bevy::log::info!("Saved interaction table to {}", path);
        Ok(())
    }

    /// Prints the interaction table to the console
    ///
    /// Outputs a formatted table showing all interaction forces
    /// between particle types.
    pub fn print_table(&self) {
        bevy::log::info!(
            "       {:>8} {:>8} {:>8}",
            ParticleType::Red.as_str(),
            ParticleType::Blue.as_str(),
            ParticleType::Green.as_str()
        );
        bevy::log::info!(
            "       {:>8} {:>8} {:>8}",
            "--------",
            "--------",
            "--------"
        );

        for target in ParticleType::all_types() {
            bevy::log::debug!("{:<6} |", target.as_str());
            for source in ParticleType::all_types() {
                let value = self.get_interaction(target, source);
                bevy::log::debug!(" {:>8.1}", value);
            }
            bevy::log::debug!("");
        }
        bevy::log::info!("");
    }

    /// Returns a reference to the underlying interaction matrix
    #[must_use] 
    pub const fn as_matrix(&self) -> &[[f32; ParticleType::COUNT]; ParticleType::COUNT] {
        &self.interactions
    }

    /// Returns a mutable reference to the underlying interaction matrix
    pub const fn as_matrix_mut(&mut self) -> &mut [[f32; ParticleType::COUNT]; ParticleType::COUNT] {
        &mut self.interactions
    }
}

/// Configuration for particle simulation
///
/// Contains all tunable parameters for the particle system.
/// These can be modified at runtime via console commands.
#[derive(Debug, Resource, Clone)]
pub struct ParticleConfig {
    /// Initial number of particles to spawn
    pub init_particle_num: usize,
    /// Width of the simulation map boundary
    pub map_width: f32,
    /// Height of the simulation map boundary
    pub map_height: f32,
    /// Collision distance (particles closer than this repel)
    pub d1: f32,
    /// Interaction transition start distance
    pub d2: f32,
    /// Maximum interaction distance and spatial partition chunk size
    pub d3: f32,
    /// Force magnitude for collision repulsion
    pub repel_force: f32,
    /// Temperature coefficient for velocity damping (friction)
    pub temperature: f32,
    /// Time step for physics updates
    pub dt: f32,
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
            temperature: 0.1,

            dt: 0.1,
        }
    }
}

/// Plugin for particle simulation system
///
/// This plugin:
/// - Inserts the particle configuration resource
/// - Registers all particle simulation systems
/// - Spawns initial particles
///
/// # Systems
/// - `setup` (Startup): Loads interactions and spawns particles
/// - `toggle_particle_update` (Update): Toggles physics updates with T key
/// - `update_particle` (Update, conditional): Updates particle physics
/// - `sync_transform` (Update): Syncs Position to Transform for rendering
/// - `respawn_particle` (Update): Respawns particles when requested
#[derive(Debug)]
pub struct ParticlePlugin {
    /// Configuration for the particle system
    pub config: ParticleConfig,
}

/// Update particle physics positions
///
/// This system updates only the `Position` and `Velocity` components.
/// It performs:
///
/// 1. Spatial partitioning for efficient neighbor queries
/// 2. Calculation of interaction forces between particles
/// 3. Collision detection and resolution
/// 4. Velocity integration and boundary checks
///
/// The `sync_transform` system will copy updated positions to the
/// `Transform` component for rendering.
#[allow(clippy::needless_pass_by_value)]
fn update_particle(
    query: Query<(Entity, &ParticleType, &mut Velocity, &mut Position), With<ParticleMarker>>,
    interaction_table: Res<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    let mut chunk: HashMap<(i32, i32), ParticleChunk> = HashMap::with_capacity(1000);
    for (entity, ptype, _, pos) in query.iter() {
        #[allow(clippy::cast_possible_truncation)]
        let x = (pos.value.x / config.d3) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = (pos.value.y / config.d3) as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push((entity, ptype.to_owned(), pos.to_owned())))
            .or_insert_with(|| [(entity, ptype.to_owned(), pos.to_owned())].into());
    }

    for (entity, ptype, mut velocity, mut position) in query {
        let my_type = *ptype;
        let my_index = entity.index();

        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (position.value.x / config.d3) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_y = (position.value.y / config.d3) as i32;

        let mut components: ParticleChunk = Vec::with_capacity(1000);
        for x in chunk_x - 1..=chunk_x + 1 {
            for y in chunk_y - 1..=chunk_y + 1 {
                chunk
                    .entry((x, y))
                    .and_modify(|inner| components.append(inner.to_owned().as_mut()));
            }
        }

        let acceleration = components
            .iter()
            .filter(|(other_entity, _, _)| other_entity.index() != my_index)
            .fold(Vec3::default(), |acc, (_, p, pos)| {
                let distance = position.value.distance(pos.value);
                let direction = (pos.value - position.value) / distance;

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

                let other_type = *p;
                let strength = interaction_table.get_interaction(my_type, other_type);
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            });

        velocity.value += acceleration * config.dt;
        velocity.value *= config.temperature.powf(config.dt);

        position.value += velocity.value * config.dt;

        let half_width = config.map_width / 2.0;
        let half_height = config.map_height / 2.0;

        if position.value.x < -half_width {
            position.value.x = -half_width;
            velocity.value.x *= -1.0;
        } else if position.value.x > half_width {
            position.value.x = half_width;
            velocity.value.x *= -1.0;
        } else if position.value.y < -half_height {
            position.value.y = -half_height;
            velocity.value.y *= -1.0;
        } else if position.value.y > half_height {
            position.value.y = half_height;
            velocity.value.y *= -1.0;
        }
    }
}

/// Sync particle positions to transform for rendering
///
/// This system copies the physics `Position` component to the
/// rendering `Transform` component. This allows the physics system
/// to update positions independently from the rendering system.
///
/// This system runs every frame to ensure particles are rendered
/// at their current physics positions.
fn sync_transform(mut query: Query<(&Position, &mut Transform), With<ParticleMarker>>) {
    for (position, mut transform) in &mut query {
        transform.translation = position.value;
    }
}

/// Spawn initial particles according to configuration
///
/// Creates the specified number of particles with random positions
/// and types within the map boundaries.
///
/// # Arguments
/// - `commands`: Bevy command queue
/// - `meshes`: Mesh assets resource
/// - `material`: Material assets resource
/// - `config`: Particle configuration with spawn parameters
#[allow(clippy::needless_pass_by_value)]
pub fn spawn_particle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
    config: Res<ParticleConfig>,
) {
    let particle_types = [ParticleType::Red, ParticleType::Blue, ParticleType::Green];

    for _ in 0..config.init_particle_num {
        let x = rand::random_range(-config.map_width / 2.0..config.map_width / 2.0);
        let y = rand::random_range(-config.map_height / 2.0..config.map_height / 2.0);

        let particle_type = particle_types[rand::random_range(0..particle_types.len())];

        Particle::spawn(
            &mut commands,
            &mut meshes,
            &mut material,
            Transform::from_xyz(x, y, 0.0),
            particle_type,
        );
    }
}

/// Remove all particles from the simulation
///
/// Despawns all entities with the [`ParticleMarker`] component.
pub fn clean_particle(mut commands: Commands, query: Query<Entity, With<ParticleMarker>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    bevy::log::info!("Cleaned all particles");
}

/// Setup function that runs once at startup
///
/// 1. Loads particle interactions from CSV file (if present)
/// 2. Spawns initial particles according to configuration
///
/// # Arguments
/// - `commands`: Bevy command queue
/// - `meshes`: Mesh assets resource
/// - `material`: Material assets resource
/// - `interaction_table`: Interaction table resource to populate
/// - `config`: Particle configuration with spawn parameters
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
            bevy::log::info!(
                "Successfully loaded particle interactions from {}",
                csv_path
            );
        }
        Err(e) => {
            bevy::log::warn!("Could not load {}, using default interactions", csv_path);
            bevy::log::error!("Error: {}", e);
        }
    }

    spawn_particle(commands, meshes, material, config);
}

/// Respawn particles when requested
///
/// Removes all existing particles and spawns a new set
/// according to current configuration.
///
/// This is triggered by the `respawn_particle` console command.
#[allow(clippy::needless_pass_by_value)]
fn respawn_particle(
    mut commands: Commands,
    query: Query<Entity, With<ParticleMarker>>,
    meshes: ResMut<Assets<Mesh>>,
    meterial: ResMut<Assets<ColorMaterial>>,
    config: Res<ParticleConfig>,
    keys: Res<ButtonInput<KeyCode>>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.is_game() && keys.just_pressed(KeyCode::KeyR) {
        clean_particle(commands.reborrow(), query);
        spawn_particle(commands, meshes, meterial, config);
    }
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.config.clone());
        app.insert_resource(ParticleUpdateToggle::default());
        app.add_systems(Startup, setup);
        app.add_systems(Update, toggle_particle_update);
        app.add_systems(
            Update,
            update_particle.run_if(|toggle: Res<ParticleUpdateToggle>| toggle.enabled),
        );
        app.add_systems(Update, sync_transform);
        app.add_systems(Update, respawn_particle);
    }
}
