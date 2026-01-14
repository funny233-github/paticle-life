use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use std::sync::Arc;

/// TODO doc
#[derive(Component, Debug, Clone, Default)]
pub struct Collision {
    /// TODO doc
    pub collision_entitys: Arc<Vec<Entity>>,
}

impl Collision {
    /// TODO doc
    #[must_use]
    pub fn new() -> Self {
        Self {
            collision_entitys: Arc::new(Vec::with_capacity(1000)),
        }
    }
}
