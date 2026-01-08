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
        let x = (pos.value.x / config.d3) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let y = (pos.value.y / config.d3) as i32;
        chunk
            .entry((x, y))
            .and_modify(|inner| inner.push((entity, ptype.to_owned(), pos.to_owned())))
            .or_insert_with(|| [(entity, ptype.to_owned(), pos.to_owned())].into());
    }

    for (entity, ptype, mut velocity, mut position) in query {
        let my_type = *ptype;
        let my_index = entity.index();

        #[allow(clippy::cast_possible_truncation)]
        let chunk_x = (position.value.x / config.d3) as i32;
        #[allow(clippy::cast_possible_truncation)]
        let chunk_y = (position.value.y / config.d3) as i32;

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
                let distance = position.value.distance(pos.value);
                let direction = (pos.value - position.value) / distance;

                if distance < config.d1 {
                    let actual_acceleration =
                        direction * config.repel_force * (config.d1 - distance);
                    return acc + actual_acceleration;
                } else if distance >= config.d3 {
                    return acc;
                }
                let distance_factor = if distance >= config.d2 {
                    (config.d3 - distance) / (config.d3 - config.d2)
                } else {
                    (distance - config.d1) / config.d1
                };

                let other_type = *p;
                let strength = interaction_table.get_interaction(my_type, other_type);
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            });

        velocity.value += acceleration * config.dt;
        velocity.value *= config.temperature.powf(config.dt);

        position.value += velocity.value * config.dt;

        let half_width = config.map_width / 2.0;
        let half_height = config.map_height / 2.0;

        if position.value.x < -half_width {
            position.value.x = -half_width;
            velocity.value.x *= -1.0;
        } else if position.value.x > half_width {
            position.value.x = half_width;
            velocity.value.x *= -1.0;
        } else if position.value.y < -half_height {
            position.value.y = -half_height;
            velocity.value.y *= -1.0;
        } else if position.value.y > half_height {
            position.value.y = half_height;
            velocity.value.y *= -1.0;
        }
    }
}
