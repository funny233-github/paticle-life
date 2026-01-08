//! Toggle particle update system
//!
//! This system toggles particle physics updates with the T key.

use crate::resources::{InputFocus, ParticleUpdateToggle};
use bevy::prelude::*;

/// Toggle particle update system
///
/// This system toggles particle physics updates with the T key.
#[allow(clippy::needless_pass_by_value)]
pub fn toggle_particle_update(
    keys: Res<ButtonInput<KeyCode>>,
    mut toggle: ResMut<ParticleUpdateToggle>,
    input_focus: Res<InputFocus>,
) {
    if input_focus.is_game() && keys.just_pressed(KeyCode::KeyT) {
        toggle.toggle();
        bevy::log::info!(
            "Particle update: {}",
            if toggle.is_enabled() {
                "enabled"
            } else {
                "disabled"
            }
        );
    }
}
