//! Camera movement control parameters

use bevy::ecs::resource::Resource;

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
