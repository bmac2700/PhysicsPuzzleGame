use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::{game_states::in_game::InGameState, keymap_manager::KeymapState};

use super::{PickableObject, PlayerBody, PlayerCamera, MAX_PICKUP_TOI};

#[derive(Default, Clone, Debug)]
pub enum PlayerInventoryItem {
    #[default]
    Empty,
}

#[derive(Resource, Default, Debug)]
pub struct PlayerInventory {
    pub inventory: Vec<PlayerInventoryItem>,
}

#[derive(Component, Clone, Debug)]
pub struct DroppedInventoryItem {
    pub data: PlayerInventoryItem,
}

/* A system that displays the events. */
pub fn handle_item_pickup(
    rapier_context: Res<RapierContext>,
    windows: Query<&mut Window>,
    keymap_state: Res<KeymapState>,

    player_camera_query: Query<&Transform, With<PlayerCamera>>,
    player_body_query: Query<&Transform, With<PlayerBody>>,
    pickable_items: Query<
        (Entity, &DroppedInventoryItem),
        (With<DroppedInventoryItem>, Without<PickableObject>),
    >,
    ingame_state: Res<InGameState>,

    mut player_inventory: ResMut<PlayerInventory>,
    mut commands: Commands,
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

    let ray_cast = rapier_context.cast_ray(
        start,
        dir,
        MAX_PICKUP_TOI,
        false,
        QueryFilter::new().exclude_sensors(),
    );

    if let Some((hit_entity, _)) = ray_cast {
        if keymap_state.interact_use_just_pressed {
            for (entity, item_data) in pickable_items.iter() {
                if entity != hit_entity {
                    continue;
                }

                player_inventory.inventory.push(item_data.data.clone());

                commands.entity(entity).despawn_recursive();
                break;
            }
        }
    }
}
