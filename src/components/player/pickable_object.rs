use bevy::{prelude::*, window::CursorGrabMode};
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

    if let Some((entity, toi)) = ray_cast {
        println!("{toi}");
        if keyboard_input.just_pressed(KeyCode::E) {
            if pickable_objects.contains(entity) {
                commands.entity(entity).despawn();
            }
        }
    }
}
