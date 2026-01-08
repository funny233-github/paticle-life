//! Input focus state

use bevy::ecs::resource::Resource;

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
