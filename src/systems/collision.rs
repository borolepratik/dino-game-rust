use crate::components::{Health, Obstacle, Player};
use bevy::prelude::{Commands, Entity, Query, Transform, With};

pub fn detect_collision_player_obstacle(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Health), With<Player>>,
    obstacle_query: Query<(Entity, &Transform), With<Obstacle>>,
) {
    if let Ok((player_transform, mut health)) = player_query.get_single_mut() {
        for (entity, obstacle_transform) in obstacle_query.iter() {
            let collision = player_transform
                .translation
                .distance(obstacle_transform.translation)
                < 50.0;
            if collision {
                health.0 -= 1;
                commands.entity(entity).despawn(); // Remove obstacle
            }
        }
    }
}
