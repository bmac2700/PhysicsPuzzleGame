use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

use super::{PlayerBody, PlayerInputState};

#[derive(Component)]
pub struct PlayerCamera;

pub fn player_look(
    windows: Query<&mut Window>,
    mut player_input_state: ResMut<PlayerInputState>,
    motion: Res<Events<MouseMotion>>,
    mut player_body: Query<&mut Transform, With<PlayerBody>>,
    mut player_camera: Query<&mut Transform, (With<PlayerCamera>, Without<PlayerBody>)>,
) {
    let window = windows.single();

    if window.cursor.grab_mode != CursorGrabMode::Locked {
        return;
    }

    let mut player_transform = match player_body.get_single_mut() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    let mut delta_state = player_input_state.as_mut();
    for event in delta_state.reader_motion.iter(&motion) {
        let window_scale = window.height().min(window.width());
        delta_state.pitch -= (0.00005 * event.delta.y * window_scale).to_radians();
        delta_state.yaw -= (0.00005 * event.delta.x * window_scale).to_radians();
        delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

        player_transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw);

        let mut camera_transform = match player_camera.get_single_mut() {
            Ok(v) => v,
            Err(_) => {
                return;
            }
        };

        camera_transform.rotation = Quat::from_axis_angle(Vec3::X, delta_state.pitch);
    }
}
