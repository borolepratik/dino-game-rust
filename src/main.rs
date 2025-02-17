mod components;
mod constants;
mod resources;
mod states;
mod systems {
    pub mod game {
        pub mod game_over;
        pub mod restart_game;
        pub mod setup;
    }
    pub mod obstacles {
        pub mod movement;
        pub mod spawn;
    }
    pub mod player {
        pub mod gravity;
        pub mod health;
        pub mod jump;
        pub mod movement;
        pub mod points;
    }
    pub mod ui {
        pub mod health;
        pub mod points;
    }
    pub mod collision;
}

use crate::constants::SPAWN_INTERVAL;
use crate::resources::ObstacleSpawningTimer;
use crate::states::GameState::{GameOver, InGame};
use crate::systems::collision::detect_collision_player_obstacle;
use crate::systems::game::game_over::game_over;
use crate::systems::game::restart_game::restart_game;
use crate::systems::game::setup::setup;
use crate::systems::obstacles::movement::move_obstacles;
use crate::systems::obstacles::spawn::spawn_obstacles;
use crate::systems::player::gravity::apply_gravity;
use crate::systems::player::health::check_health;
use crate::systems::player::jump::jump;
use crate::systems::player::movement::player_movement;
use crate::systems::player::points::update_points;
use crate::systems::ui::health::render_health_info;
use crate::systems::ui::points::render_points_info;
use bevy::prelude::{
    in_state, App, AppExtStates, IntoSystemConfigs, OnEnter, Startup, Timer, TimerMode, Update,
};
use bevy::DefaultPlugins;
use bevy_prng::WyRand;
use bevy_rand::prelude::EntropyPlugin;

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
                detect_collision_player_obstacle,
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
