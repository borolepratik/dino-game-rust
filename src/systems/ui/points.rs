use crate::components::{Player, Points, PointsInfo};
use bevy::prelude::{Query, Text, With};

pub fn render_points_info(
    player_query: Query<&mut Points, With<Player>>,
    mut points_info_query: Query<&mut Text, With<PointsInfo>>,
) {
    if let Ok(mut points_info) = points_info_query.get_single_mut() {
        if let Ok(points) = player_query.get_single() {
            points_info.0 = format!("Points: {}", points.0);
        }
    }
}
