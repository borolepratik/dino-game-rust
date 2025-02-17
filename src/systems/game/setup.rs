use crate::components::{Health, HealthInfo, Player, Points, PointsInfo, Velocity};
use crate::constants::{
    GROUND_COLOR, GROUND_EDGE, GROUND_LEVEL, GROUND_SIZE, HEALTH_INFO_POSITION, INITIAL_HEALTH,
    INITIAL_POINTS, PLAYER_COLOR, PLAYER_SIZE, PLAYER_X, POINTS_INFO_POSITION,
};
use bevy::math::Vec3;
use bevy::prelude::{default, Camera2d, Commands, Node, PositionType, Sprite, Text, Transform};
use bevy::sprite::Anchor;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    let initial_health = INITIAL_HEALTH;
    let initial_points = INITIAL_POINTS;

    // Player
    commands.spawn((
        Player,
        Sprite {
            color: PLAYER_COLOR,
            custom_size: Some(PLAYER_SIZE),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
        Health(initial_health),
        Points(initial_points),
    ));

    // Health
    commands.spawn((
        HealthInfo,
        Text::new(format!("Health: {}", initial_health)),
        Node {
            position_type: PositionType::Absolute,
            top: HEALTH_INFO_POSITION.0,
            left: HEALTH_INFO_POSITION.1,
            ..default()
        },
    ));

    // Points
    commands.spawn((
        PointsInfo,
        Text::new(format!("Points: {}", initial_points)),
        Node {
            position_type: PositionType::Absolute,
            top: POINTS_INFO_POSITION.0,
            left: POINTS_INFO_POSITION.1,
            ..default()
        },
    ));

    // Ground
    commands.spawn((
        Sprite {
            color: GROUND_COLOR,
            custom_size: Some(GROUND_SIZE),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-GROUND_EDGE, GROUND_LEVEL, 0.0),
    ));
}
