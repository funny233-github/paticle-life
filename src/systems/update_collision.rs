use crate::components::{Collision, ParticleMarker, ParticleType, Position};
use crate::resources::ParticleConfig;
use bevy::prelude::*;
use std::collections::HashMap;

/// TODO doc
pub fn update_collision(
    mut query: Query<(Entity, &ParticleType, &Position, &mut Collision), With<ParticleMarker>>,
    config: Res<ParticleConfig>,
) {
    // Estimate capacity based on map size and particle radius
    let mut chunk: HashMap<(i32, i32), Vec<_>> = HashMap::with_capacity(1000);

    // First pass: build spatial partition
    for (entity, ptype, pos, _) in query.iter() {
        #[allow(clippy::cast_possible_truncation)]
        let x = (pos.value.x / config.r).floor() as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = (pos.value.y / config.r).floor() as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push((entity, *ptype, *pos)))
            .or_insert_with(|| vec![(entity, *ptype, *pos)]);
    }

    // Second pass: assign neighboring entities to each particle
    for (entity, _, pos, mut col) in &mut query {
        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (pos.value.x / config.r).floor() as i32;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_y = (pos.value.y / config.r).floor() as i32;

        col.collision_entities = Vec::with_capacity(1000);

        // Collect entities from 3x3 surrounding chunks
        for x in chunk_x - 1..=chunk_x + 1 {
            for y in chunk_y - 1..=chunk_y + 1 {
                if let Some(neighbors) = chunk.get(&(x, y)) {
                    col.collision_entities.extend(
                        neighbors
                            .iter()
                            .filter(|(e, _, _)| e.index() != entity.index()),
                    );
                }
            }
        }
    }
}
