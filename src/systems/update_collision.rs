use crate::components::{Collision, ParticleMarker, Position};
use crate::resources::ParticleConfig;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::Arc;

/// TODO doc
pub fn update_collision(
    mut query: Query<(Entity, &Position, &mut Collision), With<ParticleMarker>>,
    config: Res<ParticleConfig>,
) {
    // Estimate capacity based on map size and particle radius
    let estimated_chunks = ((config.map_width / config.r) * (config.map_height / config.r)) as usize;
    let mut chunk: HashMap<(i32, i32), Vec<Entity>> = HashMap::with_capacity(estimated_chunks.max(1000));

    // First pass: build spatial partition
    for (entity, pos, _) in query.iter() {
        #[allow(clippy::cast_possible_truncation)]
        let x = (pos.value.x / config.r).floor() as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = (pos.value.y / config.r).floor() as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push(entity))
            .or_insert_with(|| vec![entity]);
    }

    // Second pass: assign neighboring entities to each particle
    for (_, pos, mut col) in &mut query {
        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (pos.value.x / config.r).floor() as i32;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_y = (pos.value.y / config.r).floor() as i32;

        // Clear previous collision entities
        if let Some(entities) = Arc::get_mut(&mut col.collision_entitys) {
            entities.clear();
        }

        // Collect entities from 3x3 surrounding chunks
        for x in chunk_x - 1..=chunk_x + 1 {
            for y in chunk_y - 1..=chunk_y + 1 {
                if let Some(neighbors) = chunk.get(&(x, y)) {
                    if let Some(entities) = Arc::get_mut(&mut col.collision_entitys) {
                        entities.extend(neighbors.iter().copied());
                    } else {
                        // Fallback: Arc has multiple references, create new one
                        let mut new_entities = col.collision_entitys.as_ref().to_vec();
                        new_entities.extend(neighbors.iter().copied());
                        col.collision_entitys = Arc::new(new_entities);
                    }
                }
            }
        }
    }
}
