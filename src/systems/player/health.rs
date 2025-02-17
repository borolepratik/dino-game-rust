use crate::components::{Health, Player};
use crate::states::GameState;
use crate::states::GameState::GameOver;
use bevy::prelude::{NextState, Query, ResMut, With};

pub fn check_health(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(Health(health)) = player_query.get_single() {
        if *health == 0 {
            game_state.set(GameOver);
        }
    }
}
