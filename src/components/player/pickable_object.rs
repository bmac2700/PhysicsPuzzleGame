use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::{game_states::in_game::InGameState, keymap_manager::KeymapState};

use super::{PlayerBody, PlayerCamera, MAX_PICKUP_TOI};

#[derive(Resource, Default)]
pub struct PlayerItemPickupState {
    pub current_item_at_hand: Option<Entity>,
    pub old_damping: Option<Damping>,
}

#[derive(Component)]
pub struct PickableObject;

pub fn handle_object_pickup(
    rapier_context: Res<RapierContext>,
    windows: Query<&mut Window>,
    keymap_state: Res<KeymapState>,

    player_camera_query: Query<&Transform, With<PlayerCamera>>,
    player_body_query: Query<&Transform, With<PlayerBody>>,

    mut item_pickup_state: ResMut<PlayerItemPickupState>,
    mut pickable_objects: Query<(Entity, &mut Damping), With<PickableObject>>,

    ingame_state: Res<InGameState>,
) {
    if *ingame_state != InGameState::Running {
        return;
    }

    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    if keymap_state.interact_use_just_pressed && item_pickup_state.current_item_at_hand.is_some() {
        for (entity, mut damping) in pickable_objects.iter_mut() {
            if item_pickup_state.current_item_at_hand.unwrap() != entity {
                continue;
            }

            *damping = item_pickup_state.old_damping.unwrap();
            break;
        }

        item_pickup_state.current_item_at_hand = None;
        item_pickup_state.old_damping = None;
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

    let ray_cast = rapier_context.cast_ray(
        start,
        dir,
        MAX_PICKUP_TOI,
        false,
        QueryFilter::new().exclude_sensors(),
    );

    if let Some((hit_entity, _)) = ray_cast {
        if keymap_state.interact_use_just_pressed
            && item_pickup_state.current_item_at_hand.is_none()
        {
            for (entity, mut damping) in pickable_objects.iter_mut() {
                if hit_entity != entity {
                    continue;
                }

                item_pickup_state.current_item_at_hand = Some(entity);
                item_pickup_state.old_damping = Some(*damping);

                damping.linear_damping = 10.0;
                damping.angular_damping = 10.0;
                return;
            }
        }
    }
}

pub fn move_object(
    item_pickup_state: Res<PlayerItemPickupState>,
    mut pickable_objects: Query<(Entity, &Transform, &mut Velocity), With<PickableObject>>,
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

    for (entity, transform, mut velocity) in pickable_objects.iter_mut() {
        if entity != item_pickup_state.current_item_at_hand.unwrap() {
            continue;
        }

        let mut dir = player_body_transform.rotation * Vec3::NEG_Z;
        let start = player_body_transform.translation + player_camera_transform.translation;
        dir.y = player_camera_transform.forward().y;

        let target_object_position = start + (dir * 15.0);

        let dir = target_object_position - transform.translation;

        velocity.linvel += dir;
    }
}
