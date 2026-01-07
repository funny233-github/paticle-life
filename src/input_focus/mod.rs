use bevy::prelude::*;
use bevy_console::ConsoleOpen;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum InputFocus {
    #[default]
    Game,
    Console,
}

impl InputFocus {
    pub fn toggle(&mut self) {
        *self = match *self {
            InputFocus::Game => InputFocus::Console,
            InputFocus::Console => InputFocus::Game,
        };
    }

    pub fn is_game(&self) -> bool {
        *self == InputFocus::Game
    }

    pub fn is_console(&self) -> bool {
        *self == InputFocus::Console
    }

    pub fn set_game(&mut self) {
        *self = InputFocus::Game;
    }

    pub fn set_console(&mut self) {
        *self = InputFocus::Console;
    }
}

pub fn update_input_focus(console_open: Res<ConsoleOpen>, mut input_focus: ResMut<InputFocus>) {
    if console_open.open {
        input_focus.set_console();
    } else {
        input_focus.set_game();
    }
}

pub struct InputFocusPlugin;

impl Plugin for InputFocusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputFocus::default());

        app.add_systems(Update, update_input_focus);
    }
}
