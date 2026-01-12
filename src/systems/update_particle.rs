//! Update particle physics positions
//!
//! This system updates only the `Position` and `Velocity` components.
//! It performs:
//!
//! 1. Spatial partitioning for efficient neighbor queries
//! 2. Calculation of interaction forces between particles
//! 3. Collision detection and resolution
//! 4. Velocity integration and boundary checks
//!
//! The `sync_transform` system will copy updated positions to the
//! `Transform` component for rendering.

use crate::components::{ParticleMarker, ParticleType, Position, Velocity};
use crate::resources::ParticleConfig;
use crate::resources::ParticleInteractionTable;
use crate::systems::ParticleChunk;
use bevy::prelude::*;
use std::collections::HashMap;

/// Update particle physics positions
///
/// This system updates only the `Position` and `Velocity` components.
/// It performs:
///
/// 1. Spatial partitioning for efficient neighbor queries
/// 2. Calculation of interaction forces between particles
/// 3. Collision detection and resolution
/// 4. Velocity integration and boundary checks
///
/// The `sync_transform` system will copy updated positions to the
/// `Transform` component for rendering.
#[allow(clippy::needless_pass_by_value)]
pub fn update_particle(
    query: Query<(Entity, &ParticleType, &mut Velocity, &mut Position), With<ParticleMarker>>,
    interaction_table: Res<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    let mut chunk: HashMap<(i32, i32), ParticleChunk> = HashMap::with_capacity(1000);
    for (entity, ptype, _, pos) in query.iter() {
        #[allow(clippy::cast_possible_truncation)]
        let x = (pos.value.x / config.r) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = (pos.value.y / config.r) as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push((entity, ptype.to_owned(), pos.to_owned())))
            .or_insert_with(|| [(entity, ptype.to_owned(), pos.to_owned())].into());
    }

    for (entity, ptype, mut velocity, mut position) in query {
        let my_type = *ptype;
        let my_index = entity.index();

        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (position.value.x / config.r) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_y = (position.value.y / config.r) as i32;

        let mut components: ParticleChunk = Vec::with_capacity(1000);
        for x in chunk_x - 1..=chunk_x + 1 {
            for y in chunk_y - 1..=chunk_y + 1 {
                chunk
                    .entry((x, y))
                    .and_modify(|inner| components.append(inner.to_owned().as_mut()));
            }
        }

        let acceleration = components
            .iter()
            .filter(|(other_entity, _, _)| other_entity.index() != my_index)
            .fold(Vec3::default(), |acc, (_, p, pos)| {
                let b = 0.35;
                let d1 = config.r * b;
                let d2 = config.r * (1.0 - b) / 2.0;
                let d3 = config.r;
                let distance = position.value.distance(pos.value);
                let direction = (pos.value - position.value) / distance;
                let distance_factor;

                if distance < d1 {
                    distance_factor = (distance - d1) / d1;
                    let actual_acceleration = direction * distance_factor;
                    return acc + actual_acceleration;
                } else if distance >= d3 {
                    return acc;
                } else if distance >= d2 {
                    distance_factor = (d3 - distance) / (d3 - d2);
                } else {
                    distance_factor = (distance - d1) / (d2 - d1);
                }

                let other_type = *p;
                let strength = interaction_table.get_interaction(my_type, other_type);
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            });

        velocity.value *= 0.5f32.powf(config.dt / config.dt_half);
        velocity.value += acceleration * config.dt;

        position.value += velocity.value * config.dt;

        let half_width = config.map_width / 2.0;
        let half_height = config.map_height / 2.0;

        position.value.x = position.value.x.clamp(-half_width, half_width);
        position.value.y = position.value.y.clamp(-half_height, half_height);
    }
}
