//! Systems module
//!
//! This module contains all Bevy systems used in the game.

mod move_camera;
mod respawn_particle;
pub mod setup;
mod sync_transform;
mod toggle_particle_update;
mod update_input_focus;
mod update_particle;

pub use move_camera::{move_camera, ParticleChunk};
pub use respawn_particle::{clean_particle, respawn_particle, spawn_particle};
pub use setup::setup;
pub use sync_transform::sync_transform;
pub use toggle_particle_update::toggle_particle_update;
pub use update_input_focus::update_input_focus;
pub use update_particle::update_particle;
