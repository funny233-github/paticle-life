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

use crate::components::{Collision, ParticleMarker, ParticleType, Position, Velocity};
use crate::resources::ParticleConfig;
use crate::resources::ParticleInteractionTable;
use bevy::prelude::ParamSet;
use bevy::prelude::*;

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
    mut query_set: ParamSet<(
        Query<
            (
                Entity,
                &ParticleType,
                &mut Velocity,
                &mut Position,
                &Collision,
            ),
            With<ParticleMarker>,
        >,
        Query<(Entity, &ParticleType, &Position), With<ParticleMarker>>,
    )>,
    interaction_table: Res<ParticleInteractionTable>,
    config: Res<ParticleConfig>,
) {
    use std::collections::HashMap;

    // First pass: collect all read-only data into HashMap for O(1) lookup
    let mut read_only_data: HashMap<Entity, (ParticleType, Position)> = HashMap::new();
    for (entity, ptype, position) in query_set.p1().iter() {
        read_only_data.insert(entity, (*ptype, *position));
    }

    // Second pass: update particles using the read-only data
    for (entity, ptype, mut velocity, mut position, col) in &mut query_set.p0() {
        let my_type = *ptype;
        let my_index = entity.index();

        let acceleration = col
            .collision_entitys
            .iter()
            .filter(|other_entity| other_entity.index() != my_index)
            .filter_map(|other_entity| read_only_data.get(other_entity))
            .fold(Vec3::default(), |acc, (other_type, other_pos)| {
                let b = 0.35;
                let d1 = config.r * b;
                let d2 = config.r * (1.0 - b) / 2.0;
                let d3 = config.r;
                let distance = position.value.distance(other_pos.value);
                let direction = (other_pos.value - position.value) / distance;
                let distance_factor;

                if distance < d1 {
                    distance_factor = (distance - d1) / d1;
                    let actual_acceleration = direction * distance_factor * config.repel_force;
                    return acc + actual_acceleration;
                } else if distance >= d3 {
                    return acc;
                } else if distance >= d2 {
                    distance_factor = (d3 - distance) / (d3 - d2);
                } else {
                    distance_factor = (distance - d1) / (d2 - d1);
                }

                let strength = interaction_table.get_interaction(my_type, *other_type);
                let actual_acceleration = direction * strength * distance_factor;

                acc + actual_acceleration
            });

        velocity.value *= 0.5f32.powf(config.dt / config.dt_half);
        velocity.value += acceleration * config.dt;

        let half_width = config.map_width / 2.0;
        let half_height = config.map_height / 2.0;

        if (position.value.x > half_width && velocity.value.x > 0.0)
            || (position.value.x < -half_width && velocity.value.x < 0.0)
        {
            velocity.value.x = -velocity.value.x;
        }
        if (position.value.y > half_height && velocity.value.y > 0.0)
            || (position.value.y < -half_height && velocity.value.y < 0.0)
        {
            velocity.value.y = -velocity.value.y;
        }

        position.value += velocity.value * config.dt;
    }
}
