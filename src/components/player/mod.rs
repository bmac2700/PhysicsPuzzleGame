use bevy::{ecs::event::ManualEventReader, input::mouse::MouseMotion, prelude::*};

use crate::game_states::AppState;

mod look;
mod model;
mod movement;
mod pickable_object;

pub use look::PlayerCamera;
pub use model::PlayerModel;
pub use movement::PlayerBody;
pub use pickable_object::PickableObject;

use self::pickable_object::PlayerItemPickupState;

// Movement variables
const GROUND_DAMPING: f32 = 5.0;
const GROUND_TOI: f32 = 8.0;

const MOVEMENT_SPEED: f32 = 24.0;
const MOVEMENT_RUN_SPEED_BOOST: f32 = 8.0;
const MOVEMENT_CROUCH_SPEED_BOOST: f32 = -12.0;
const JUMP_FORCE: f32 = 40.0;
const JUMP_GRAVITY: f32 = 8.0;

//Pickup system variables
const MAX_PICKUP_TOI: f32 = 15.0;

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
            .init_resource::<PlayerItemPickupState>()
            .add_system(movement::player_movement.in_set(OnUpdate(AppState::InGame)))
            .add_system(movement::initialize_player_body.in_set(OnUpdate(AppState::InGame)))
            .add_system(look::player_look.in_set(OnUpdate(AppState::InGame)))
            .add_system(pickable_object::handle_object_pickup.in_set(OnUpdate(AppState::InGame)))
            .add_system(pickable_object::move_object.in_set(OnUpdate(AppState::InGame)));
    }
}
