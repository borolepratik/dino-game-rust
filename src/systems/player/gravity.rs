use crate::components::{Player, Velocity};
use crate::constants::GRAVITY;
use bevy::prelude::{Query, Res, Time, With};

pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}
