use bevy::prelude::*;

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

pub fn toggle_input_focus(keys: Res<ButtonInput<KeyCode>>, mut input_focus: ResMut<InputFocus>) {
    if keys.just_pressed(KeyCode::Backquote) {
        input_focus.toggle();
        bevy::log::info!(
            "Input focus switched to: {}",
            if input_focus.is_game() {
                "Game"
            } else {
                "Console"
            }
        );
    }
}

pub struct InputFocusPlugin;

impl Plugin for InputFocusPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InputFocus::default());

        app.add_systems(Update, toggle_input_focus);
    }
}
