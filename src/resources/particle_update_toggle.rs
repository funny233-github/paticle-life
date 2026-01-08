//! Toggle resource for particle update

use bevy::ecs::resource::Resource;

/// Toggle resource for particle update
#[derive(Resource, Default)]
pub struct ParticleUpdateToggle {
    /// Whether particle updates are enabled
    enabled: bool,
}

impl ParticleUpdateToggle {
    /// Creates a new toggle with enabled state
    #[must_use]
    pub const fn new() -> Self {
        Self { enabled: true }
    }

    /// Returns whether particle updates are enabled
    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Toggles the particle update state
    pub const fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}
