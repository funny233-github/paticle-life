//! Update input focus based on console state
//!
//! This system automatically updates the input focus resource
//! based on whether the console is currently open or closed.

use crate::resources::InputFocus;
use bevy::prelude::*;
use bevy_console::ConsoleOpen;

/// Update input focus based on console state
///
/// This system automatically updates the input focus resource
/// based on whether the console is currently open or closed.
///
/// When the console is open, input focus is set to console.
/// When the console is closed, input focus is set to game.
///
/// # Arguments
/// - `Res<ConsoleOpen>`: Current console state
/// - `ResMut<InputFocus>`: Focus resource to update
#[allow(clippy::needless_pass_by_value)]
pub fn update_input_focus(console_open: Res<ConsoleOpen>, mut input_focus: ResMut<InputFocus>) {
    if console_open.open {
        input_focus.set_console();
    } else {
        input_focus.set_game();
    }
}
