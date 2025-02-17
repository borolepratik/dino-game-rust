use crate::GameState::{GameOver, InGame};
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
const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const SPAWN_INTERVAL: f32 = 1.0;
const GROUND_LEVEL: f32 = -100.0;
const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
const HEALTH_INFO_POSITION: (Val, Val) = (Val::Px(5.0), Val::Px(5.0));
const POINTS_INFO_POSITION: (Val, Val) = (Val::Px(30.0), Val::Px(5.0));
const INITIAL_HEALTH: usize = 3;
const INITIAL_POINTS: usize = 0;
//endregion

//region Components, resources, and states
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Obstacle;

#[derive(Component)]
struct Health(usize);

#[derive(Component)]
struct HealthInfo;

#[derive(Component)]
struct Points(usize);

#[derive(Component)]
struct PointsInfo;

#[derive(Component)]
struct GameOverText;

#[derive(Resource)]
struct ObstacleSpawningTimer(Timer);

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    InGame,
    GameOver,
}
//endregion

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .add_systems(Startup, setup)
        .insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
            SPAWN_INTERVAL,
            TimerMode::Repeating,
        )))
        .insert_state(InGame)
        .add_systems(
            Update,
            (jump, apply_gravity, player_movement).run_if(in_state(InGame)),
        )
        .add_systems(
            Update,
            (
                spawn_obstacles,
                move_obstacles,
                detect_collision,
                render_health_info,
                check_health,
                render_points_info,
                update_points,
            )
                .run_if(in_state(InGame)),
        )
        .add_systems(OnEnter(GameOver), game_over)
        .add_systems(Update, restart_game.run_if(in_state(GameOver)))
        .run();
}

fn setup(mut commands: Commands) {
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

fn move_obstacles(time: Res<Time>, mut query: Query<&mut Transform, With<Obstacle>>) {
    for mut transform in query.iter_mut() {
        transform.translation.x -= GAME_SPEED * time.delta_secs();
    }
}

fn detect_collision(
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

fn check_health(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}

fn game_over(mut commands: Commands) {
    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(10.),
            right: Val::Percent(10.),
            top: Val::Percent(15.),
            bottom: Val::Percent(15.),
            justify_content: JustifyContent::Center,
            ..default()
        },))
        .with_children(|builder| {
            builder.spawn((
                Text("GAME OVER".to_string()),
                TextFont::from_font_size(160.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                GameOverText,
            ));
            builder.spawn((
                Button,
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(20.0),
                    ..default()
                },
                Text("Restart".to_string()),
                TextFont::from_font_size(40.0),
                TextLayout::new_with_justify(JustifyText::Center).with_no_wrap(),
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
            ));
        });
}

fn render_health_info(
    player_query: Query<&mut Health, With<Player>>,
    mut health_info_query: Query<&mut Text, With<HealthInfo>>,
) {
    if let Ok(mut health_info) = health_info_query.get_single_mut() {
        if let Ok(health) = player_query.get_single() {
            health_info.0 = format!("Health: {}", health.0);
        }
    }
}

fn render_points_info(
    player_query: Query<&mut Points, With<Player>>,
    mut points_info_query: Query<&mut Text, With<PointsInfo>>,
) {
    if let Ok(mut points_info) = points_info_query.get_single_mut() {
        if let Ok(points) = player_query.get_single() {
            points_info.0 = format!("Points: {}", points.0);
        }
    }
}

fn update_points(
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

fn restart_game(
    mut commands: Commands,
    button_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut player_query: Query<(&mut Transform, &mut Health, &mut Points), With<Player>>,
    obstacle_query: Query<Entity, With<Obstacle>>,
    game_over_text_query: Query<Entity, With<GameOverText>>,
    restart_button_query: Query<Entity, With<Button>>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Reset game state
            game_state.set(InGame);

            // Reset player state
            if let Ok((mut transform, mut health, mut points)) = player_query.get_single_mut() {
                transform.translation = Vec3::new(PLAYER_X, GROUND_LEVEL, 0.0);
                health.0 = INITIAL_HEALTH; // Reset health to initial value
                points.0 = INITIAL_POINTS; // Reset points to initial value
            }

            // De-spawn all obstacles
            for entity in obstacle_query.iter() {
                commands.entity(entity).despawn();
            }

            // De-spawn game over text
            for entity in game_over_text_query.iter() {
                commands.entity(entity).despawn();
            }

            // De-spawn restart button
            for entity in restart_button_query.iter() {
                commands.entity(entity).despawn();
            }

            // Reset obstacle spawning timer
            commands.insert_resource(ObstacleSpawningTimer(Timer::from_seconds(
                SPAWN_INTERVAL,
                TimerMode::Repeating,
            )));
        }
    }
}
