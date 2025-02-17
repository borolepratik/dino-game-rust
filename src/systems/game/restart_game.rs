use crate::components::{GameOverText, Health, Obstacle, Player, Points};
use crate::constants::{GROUND_LEVEL, INITIAL_HEALTH, INITIAL_POINTS, PLAYER_X, SPAWN_INTERVAL};
use crate::resources::ObstacleSpawningTimer;
use crate::states::GameState;
use crate::states::GameState::InGame;
use bevy::math::Vec3;
use bevy::prelude::{
    Button, Changed, Commands, Entity, Interaction, NextState, Query, ResMut, Timer, TimerMode,
    Transform, With,
};

pub fn restart_game(
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
