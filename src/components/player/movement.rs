use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::game_states::in_game::InGameState;

#[derive(Component)]
pub struct PlayerBody;

pub fn initialize_player_body(
    mut commands: Commands,
    query: Query<
        Entity,
        (
            With<PlayerBody>,
            Without<RigidBody>,
            Without<Velocity>,
            Without<LockedAxes>,
        ),
    >,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED);
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Query<&mut Window>,
    mut query: Query<(&Transform, &mut Velocity), (With<PlayerBody>, With<Velocity>)>,

    ingame_state: Res<InGameState>,
) {
    if *ingame_state != InGameState::Running {
        return;
    }

    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let (player_transform, mut player_velocity) = match query.get_single_mut() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    let local_z = player_transform.local_z();

    let forward = -Vec3::new(local_z.x, 0.0, local_z.z);
    //let right = Vec3::new(local_z.z, 0.0, -local_z.x);

    let mut new_velocity = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        new_velocity += forward;
    }

    player_velocity.linvel += new_velocity * time.delta_seconds() * 100.0;
    //println!("moving");
}
