use bevy::prelude::*;

use crate::game_states::AppState;

#[derive(Component)]
pub struct RatMovementBehaviour;

pub struct RatPlugin;

mod movement;

impl Plugin for RatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement::rat_movement.in_set(OnUpdate(AppState::InGame)));
    }
}
