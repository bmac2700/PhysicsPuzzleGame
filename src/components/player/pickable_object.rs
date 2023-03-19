use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_states::in_game::InGameState;

use super::{PlayerBody, PlayerCamera};

#[derive(Component)]
pub struct PickableObject;

pub fn handle_object_pickup(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,

    player_camera_query: Query<&Transform, With<PlayerCamera>>,
    player_body_query: Query<&Transform, With<PlayerBody>>,

    ingame_state: Res<InGameState>,
) {
    let player_camera_transform = match player_camera_query.get_single() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    let player_body_transform = match player_body_query.get_single() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };
    let dir = player_camera_transform.forward();

    let ray_cast = rapier_context.cast_ray(
        player_body_transform.translation + player_camera_transform.translation,
        dir,
        Real::MAX,
        false,
        QueryFilter::only_dynamic(),
    );
    if let Some((entity, toi)) = ray_cast {
        println!("YEY");
        if keyboard_input.just_pressed(KeyCode::E) {
            commands.entity(entity).despawn();
        }
    }
}
