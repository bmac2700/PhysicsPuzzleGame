use bevy::{prelude::*, window::CursorGrabMode};
use bevy_rapier3d::prelude::*;

use crate::game_states::in_game::InGameState;

use super::{
    GROUND_DAMPING, GROUND_TOI, JUMP_FORCE, JUMP_GRAVITY, MOVEMENT_CROUCH_SPEED_BOOST,
    MOVEMENT_RUN_SPEED_BOOST, MOVEMENT_SPEED,
};

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
            Without<Damping>,
            Without<GravityScale>,
        ),
    >,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(RigidBody::Dynamic)
            .insert(Velocity::default())
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Damping::default())
            .insert(GravityScale(JUMP_GRAVITY));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Query<&mut Window>,
    mut query: Query<(&Transform, &mut Velocity, &mut Damping), (With<PlayerBody>, With<Velocity>)>,
    rapier_context: Res<RapierContext>,

    ingame_state: Res<InGameState>,
) {
    if *ingame_state != InGameState::Running {
        return;
    }

    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let (player_transform, mut player_velocity, mut damping) = match query.get_single_mut() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    let on_ground = rapier_context
        .cast_ray(
            player_transform.translation,
            Vec3::NEG_Y,
            GROUND_TOI,
            true,
            QueryFilter::new(),
        )
        .is_some();

    if on_ground {
        damping.linear_damping = GROUND_DAMPING;
    } else {
        damping.linear_damping = 2.0;
    }

    let local_z = player_transform.local_z();

    let forwardmove = -Vec3::new(local_z.x, 0.0, local_z.z);
    let sidemove = Vec3::new(local_z.z, 0.0, -local_z.x);

    let mut new_velocity = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::W) {
        new_velocity += forwardmove;
    }

    if keyboard_input.pressed(KeyCode::S) {
        new_velocity -= forwardmove;
    }

    if keyboard_input.pressed(KeyCode::A) {
        new_velocity -= sidemove;
    }

    if keyboard_input.pressed(KeyCode::D) {
        new_velocity += sidemove;
    }

    let mut target_speed = MOVEMENT_SPEED;
    if keyboard_input.pressed(KeyCode::LShift) && on_ground && !keyboard_input.pressed(KeyCode::S) {
        //Running speed
        target_speed += MOVEMENT_RUN_SPEED_BOOST;
    }

    if keyboard_input.pressed(KeyCode::LControl) && on_ground {
        //Running speed
        target_speed += MOVEMENT_CROUCH_SPEED_BOOST;
    }

    let mut multiplier = 150.0;

    if keyboard_input.pressed(KeyCode::LShift) && on_ground {
        //Running speed
        multiplier += 150.0;
    }

    if !on_ground {
        multiplier *= 0.25;
    }

    //WASD movement
    player_velocity.linvel += new_velocity * time.delta_seconds() * multiplier;

    let flat_velocity = Vec3::new(player_velocity.linvel.x, 0.0, player_velocity.linvel.z);

    if flat_velocity.length() > target_speed {
        let mut limited_velocity = flat_velocity.normalize() * target_speed;
        limited_velocity.y = player_velocity.linvel.y;

        if on_ground {
            player_velocity.linvel = limited_velocity;
        }
    }

    //Jump movement
    if keyboard_input.just_pressed(KeyCode::Space) && on_ground {
        player_velocity.linvel = Vec3::new(
            player_velocity.linvel.x,
            JUMP_FORCE,
            player_velocity.linvel.z,
        );
    }
}
