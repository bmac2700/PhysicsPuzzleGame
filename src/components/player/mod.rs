use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::game_states::AppState;

mod movement;
pub use movement::PlayerBody;

const GROUND_DAMPING: f32 = 5.0;
const MOVEMENT_SPEED: f32 = 16.0;
const GROUND_TOI: f32 = 7.5;
const JUMP_FORCE: f32 = 13.0;

#[derive(Resource, Default)]
pub struct PlayerInputState {
    _reader_motion: ManualEventReader<MouseMotion>,
    _pitch: f32,
    _yaw: f32,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerInputState>()
            .add_system(movement::player_movement.in_set(OnUpdate(AppState::InGame)))
            .add_system(movement::initialize_player_body.in_set(OnUpdate(AppState::InGame)));
    }
}
