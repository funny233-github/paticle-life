//! Camera movement and zoom control system
//!
//! Controls:
//! - **WASD**: Move camera up/left/down/right
//! - **+/-**: Zoom in/out
//!
//! This system only responds to input when the game has focus
//! (as opposed to the console focus).

use crate::resources::{CameraMoveConfig, InputFocus};
use bevy::prelude::*;

/// Type alias for particle chunk data in spatial partitioning
pub type ParticleChunk = Vec<(
    Entity,
    crate::components::ParticleType,
    crate::components::Position,
)>;

/// Camera movement and zoom control system
///
/// Controls:
/// - **WASD**: Move camera up/left/down/right
/// - **+/-**: Zoom in/out
///
/// This system only responds to input when the game has focus
/// (as opposed to the console focus).
///
/// # System Parameters
/// - `Query<(&mut Transform, &Camera), With<Camera2d>>`: Camera transform and projection
/// - `Res<ButtonInput<KeyCode>>`: Keyboard input
/// - `Res<Time>`: Time delta for frame-independent movement
/// - `Res<InputFocus>`: Current focus state (game vs console)
/// - `Res<CameraMoveConfig>`: Movement configuration
#[allow(clippy::needless_pass_by_value)]
pub fn move_camera(
    mut camera: Query<(&mut Transform, &Camera), With<Camera2d>>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    input_focus: Res<InputFocus>,
    config: Res<CameraMoveConfig>,
) {
    if !input_focus.is_game() {
        return;
    }

    let Ok((mut transform, _camera)) = camera.single_mut() else {
        return;
    };

    let mut direction = Vec3::ZERO;
    let current_scale = transform.scale;

    if keys.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    if direction != Vec3::ZERO {
        transform.translation +=
            direction.normalize() * config.speed * current_scale * time.delta_secs();
    }

    if keys.pressed(KeyCode::Minus) || keys.pressed(KeyCode::NumpadAdd) {
        transform.scale *= config.zoom_speed.mul_add(time.delta_secs(), 1.0);
    }
    if keys.pressed(KeyCode::Equal) || keys.pressed(KeyCode::NumpadSubtract) {
        transform.scale *= config.zoom_speed.mul_add(-time.delta_secs(), 1.0);
    }

    transform.scale = transform
        .scale
        .clamp(Vec3::splat(config.min_scale), Vec3::splat(config.max_scale));
}
