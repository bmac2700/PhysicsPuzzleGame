use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::game_states::in_game::InGameState;

use super::{PlayerBody, PlayerCamera};

#[derive(Resource, Default)]
pub struct PlayerItemPickupState {
    pub current_item_at_hand: Option<Entity>,
}

#[derive(Component)]
pub struct PickableObject;

pub fn handle_object_pickup(
    rapier_context: Res<RapierContext>,
    windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,

    player_camera_query: Query<&Transform, With<PlayerCamera>>,
    player_body_query: Query<&Transform, With<PlayerBody>>,

    mut item_pickup_state: ResMut<PlayerItemPickupState>,
    pickable_objects: Query<Entity, With<PickableObject>>,

    ingame_state: Res<InGameState>,
) {
    if *ingame_state != InGameState::Running {
        return;
    }

    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::E) && item_pickup_state.current_item_at_hand.is_some() {
        item_pickup_state.current_item_at_hand = None;
        return;
    }

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
    let mut dir = player_body_transform.rotation * Vec3::NEG_Z;
    let start = player_body_transform.translation + player_camera_transform.translation;
    dir.y = player_camera_transform.forward().y;

    let ray_cast =
        rapier_context.cast_ray(start, dir, Real::MAX, false, QueryFilter::only_dynamic());

    if let Some((entity, _)) = ray_cast {
        if keyboard_input.just_pressed(KeyCode::E)
            && item_pickup_state.current_item_at_hand.is_none()
        {
            if pickable_objects.contains(entity) {
                item_pickup_state.current_item_at_hand = Some(entity);
            }
        }
    }
}

pub fn move_object(
    item_pickup_state: Res<PlayerItemPickupState>,
    mut pickable_objects: Query<(Entity, &mut Transform), With<PickableObject>>,
    player_camera_query: Query<&Transform, (With<PlayerCamera>, Without<PickableObject>)>,
    player_body_query: Query<&Transform, (With<PlayerBody>, Without<PickableObject>)>,
) {
    if item_pickup_state.current_item_at_hand.is_none() {
        return;
    }

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

    for (entity, mut transform) in pickable_objects.iter_mut() {
        if entity != item_pickup_state.current_item_at_hand.unwrap() {
            continue;
        }

        let mut dir = player_body_transform.rotation * Vec3::NEG_Z;
        let start = player_body_transform.translation + player_camera_transform.translation;
        dir.y = player_camera_transform.forward().y;

        transform.translation = start + (dir * 15.0);
    }
}
