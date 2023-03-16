use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::game_states::AppState;

mod movement;

pub use movement::PlayerBody;

#[derive(Resource, Default)]
pub struct PlayerInputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInputState>()
            .add_system(movement::player_movement.in_set(OnUpdate(AppState::InGame)))
            .add_system(movement::initialize_player_body.in_set(OnUpdate(AppState::InGame)));
    }
}
