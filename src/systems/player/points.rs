use crate::components::{Obstacle, Player, Points};
use crate::constants::OBSTACLE_SIZE;
use bevy::prelude::{Commands, Entity, Query, Transform, With};

pub fn update_points(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Points), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    if let Ok((player_transform, mut points)) = player_query.get_single_mut() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let win = player_transform.translation.x
                > (obstacle_transform.translation.x + OBSTACLE_SIZE.x);
            if win {
                points.0 += 1;
                commands.entity(entity).despawn(); // Remove obstacle
            }
        }
    }
}
