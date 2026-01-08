//! Particle interaction table
//!
//! Stores interaction forces between all pairs of particle types.

use crate::components::ParticleType;
use bevy::ecs::resource::Resource;
use std::str::FromStr;

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
