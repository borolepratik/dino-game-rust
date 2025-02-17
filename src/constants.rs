use bevy::prelude::{Color, Val, Vec2};

pub const GAME_SPEED: f32 = 400.0;
pub const JUMP_FORCE: f32 = 600.0;
pub const GRAVITY: f32 = -1500.0;
pub const PLAYER_X: f32 = -300.0;
pub const PLAYER_SIZE: Vec2 = Vec2::new(30.0, 50.0);
pub const PLAYER_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
pub const SPAWN_INTERVAL: f32 = 1.0;
pub const GROUND_LEVEL: f32 = -100.0;
pub const GROUND_SIZE: Vec2 = Vec2::new(800.0, 10.0);
pub const GROUND_EDGE: f32 = GROUND_SIZE.x / 2.0;
pub const GROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);
pub const OBSTACLE_SIZE: Vec2 = Vec2::new(30.0, 30.0);
pub const OBSTACLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
pub const HEALTH_INFO_POSITION: (Val, Val) = (Val::Px(5.0), Val::Px(5.0));
pub const POINTS_INFO_POSITION: (Val, Val) = (Val::Px(30.0), Val::Px(5.0));
pub const INITIAL_HEALTH: usize = 3;
pub const INITIAL_POINTS: usize = 0;
