//! Input focus management (Game vs Console)
//!
//! This module provides a resource that tracks whether user input
//! is directed to the game world or the in-game console.
//!
//! The input focus is automatically updated based on the console state.
//! When the console is open, all input goes to the console.
//! When the console is closed, all input goes to the game.

use bevy::prelude::*;
use bevy_console::ConsoleOpen;

/// Current input focus state
///
/// Determines whether keyboard input is directed to the game
/// or to the in-game console.
#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputFocus {
    /// Input goes to the game world
    #[default]
    Game,
    /// Input goes to the console
    Console,
}

impl InputFocus {
    /// Toggle between game and console focus
    pub const fn toggle(&mut self) {
        *self = match *self {
            Self::Game => Self::Console,
            Self::Console => Self::Game,
        };
    }

    /// Returns true if focus is on the game
    #[must_use] 
    pub fn is_game(&self) -> bool {
        *self == Self::Game
    }

    /// Returns true if focus is on the console
    #[must_use] 
    pub fn is_console(&self) -> bool {
        *self == Self::Console
    }

    /// Set focus to the game
    pub const fn set_game(&mut self) {
        *self = Self::Game;
    }

    /// Set focus to the console
    pub const fn set_console(&mut self) {
        *self = Self::Console;
    }
}

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

/// Plugin that registers input focus management system
///
/// This plugin:
/// - Inserts the default [`InputFocus`] resource
/// - Registers the [`update_input_focus`] system
pub struct InputFocusPlugin;

impl Plugin for InputFocusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputFocus::default());

        app.add_systems(Update, update_input_focus);
    }
}
