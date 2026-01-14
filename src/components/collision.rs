use crate::components::{ParticleType, Position};
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;

/// TODO doc
#[derive(Component, Debug, Clone, Default)]
pub struct Collision {
    /// TODO doc
    pub collision_entities: Vec<(Entity, ParticleType, Position)>,
}

impl Collision {
    /// TODO doc
    #[must_use]
    pub fn new() -> Self {
        Self {
            collision_entities: Vec::with_capacity(1000),
        }
    }
}
