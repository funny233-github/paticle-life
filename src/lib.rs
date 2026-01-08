//! Bevy game simulation library with particle systems
//!
//! This library provides a particle simulation system built on the Bevy game engine.
//! It features console commands for runtime configuration, particle interaction tables,
//! and separated physics/rendering updates.
//!
//! # Main Features
//! - Particle simulation with different types (Red, Blue, Green)
//! - Configurable interaction tables stored in CSV format
//! - Runtime console commands for tweaking simulation parameters
//! - Input focus management for game vs console input
//! - Camera movement controls

#![forbid(missing_docs, unsafe_code)]
#![warn(
    clippy::all,
    clippy::nursery,
    clippy::pedantic,
    nonstandard_style,
    rustdoc::broken_intra_doc_links
)]

/// Camera movement and control systems
pub mod camera_move;

/// Console command system for runtime configuration
pub mod command;

/// Input focus management (Game vs Console)
pub mod input_focus;

/// Particle simulation system with physics and rendering
pub mod particle;
