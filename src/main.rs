use bevy::prelude::*;
use bevy::sprite::Anchor;

const GROUND_LEVEL: f32 = -100.0;
const PLAYER_X: f32 = -300.0;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Velocity(Vec3);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());

    // Player
    commands
        .spawn((
            Player,
            Sprite {
                color: Color::srgb(0.5, 1.0, 0.5),
                custom_size: Some(Vec2::new(30.0, 50.0)),
                anchor: Anchor::BottomCenter,
                ..default()
            },
            Transform::from_xyz(PLAYER_X, GROUND_LEVEL, 0.0),
            Velocity(Vec3::ZERO)
        ));

    // Ground
    commands.spawn((
        Sprite {
            color: Color::srgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(800.0, 10.0)),
            anchor: Anchor::TopLeft,
            ..default()
        },
        Transform::from_xyz(-400.0, GROUND_LEVEL, 0.0)
    ));
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
