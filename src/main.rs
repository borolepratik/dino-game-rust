use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_prng::WyRand;
use bevy_rand::prelude::{EntropyPlugin, GlobalEntropy};
use rand_core::RngCore;

//region Constants
const GAME_SPEED: f32 = 400.0;
const JUMP_FORCE: f32 = 600.0;
const GRAVITY: f32 = -1500.0;
const PLAYER_X: f32 = -300.0;
const SPAWN_INTERVAL: f32 = 1.0;
const GROUND_LEVEL: f32 = -100.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
//endregion

//region Components, resources, and states
#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Velocity(Vec3);
//endregion

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands.spawn((
        Player,
        Sprite {
            color: Color::srgb(0.5, 1.0, 0.5),
            custom_size: Some(Vec2::new(30.0, 50.0)),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
        Velocity(Vec3::ZERO),
    ));

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(GROUND_SIZE),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-400.0, GROUND_LEVEL, 0.0),
    ));
}

fn jump(
    mut events: EventReader<KeyboardInput>,
    mut query: Query<(&mut Velocity, &Transform), With<Player>>,
) {
    for e in events.read() {
        if let Ok((mut velocity, transform)) = query.get_single_mut() {
            if e.state.is_pressed()
                && e.key_code == KeyCode::Space
                && transform.translation.y <= GROUND_LEVEL
            {
                velocity.0.y = JUMP_FORCE;
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    for (mut transform, mut velocity) in query.iter_mut() {
        transform.translation.y += velocity.0.y * time.delta_secs();
        if transform.translation.y <= GROUND_LEVEL {
            transform.translation.y = GROUND_LEVEL;
            velocity.0.y = 0.0;
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        velocity.0.y += GRAVITY * time.delta_secs();
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<ObstacleSpawningTimer>,
    mut rng: GlobalEntropy<WyRand>,
) {
    spawn_timer.0.tick(time.delta());
    if spawn_timer.0.finished() {
        let obstacle_x = GROUND_EDGE;
        let obstacle_y = GROUND_LEVEL + (rng.next_u32() % 70) as f32;
        commands.spawn((
            Obstacle,
            Sprite {
                color: OBSTACLE_COLOR,
                custom_size: Some(OBSTACLE_SIZE),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(obstacle_x, obstacle_y, 0.0),
        ));
    }
}

fn move_obstacles(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform), With<Obstacle>>,
) {
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();

        // Remove obstacles once they're off-screen
        if transform.translation.x < -GROUND_EDGE {
            commands.entity(entity).despawn();
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                jump,
                apply_gravity,
                player_movement,
                spawn_obstacles,
                move_obstacles,
            ),
        )
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .run();
}
