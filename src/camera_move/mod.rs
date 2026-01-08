//! Camera movement and control systems
//!
//! This module provides camera movement controls for 2D games, including
//! panning (WASD keys) and zooming (+/- keys). The camera only responds to
//! input when the game has focus (as opposed to console focus).

use crate::input_focus::InputFocus;
use bevy::prelude::*;

/// Camera movement control parameters
///
/// Configuration for camera movement speed and zoom limits.
#[derive(Resource, Clone, Copy)]
pub struct CameraMoveConfig {
    /// Camera movement speed in units per second
    pub speed: f32,
    /// Zoom speed multiplier
    pub zoom_speed: f32,
    /// Minimum zoom scale (zoomed out)
    pub min_scale: f32,
    /// Maximum zoom scale (zoomed in)
    pub max_scale: f32,
}

impl Default for CameraMoveConfig {
    fn default() -> Self {
        Self {
            speed: 400.0,
            zoom_speed: 1.0,
            min_scale: 0.01,
            max_scale: 50.0,
        }
    }
}

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

/// Plugin that registers camera movement system
///
/// This plugin:
/// - Inserts the default [`CameraMoveConfig`] resource
/// - Registers the [`move_camera`] system to run in the `Update` schedule
pub struct CameraMovePlugin;

impl Plugin for CameraMovePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraMoveConfig::default());

        app.add_systems(Update, move_camera);
    }
}
