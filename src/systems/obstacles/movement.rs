use crate::components::Obstacle;
use crate::constants::GAME_SPEED;
use bevy::prelude::{Query, Res, Time, Transform, With};

pub fn move_obstacles(time: Res<Time>, mut query: Query<&mut Transform, With<Obstacle>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
    }
}
