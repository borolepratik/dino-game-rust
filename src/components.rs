use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(pub Vec3);

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Health(pub usize);

#[derive(Component)]
pub struct HealthInfo;

#[derive(Component)]
pub struct Points(pub usize);

#[derive(Component)]
pub struct PointsInfo;

#[derive(Component)]
pub struct GameOverText;
