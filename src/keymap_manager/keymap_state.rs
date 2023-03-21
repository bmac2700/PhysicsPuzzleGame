use bevy::prelude::*;

use super::KeymapResource;

#[derive(Resource, Default)]
pub struct KeymapState {
    pub move_forward_just_pressed: bool,
    pub move_forward_pressed: bool,

    pub move_backward_just_pressed: bool,
    pub move_backward_pressed: bool,

    pub move_left_just_pressed: bool,
    pub move_left_pressed: bool,

    pub move_right_just_pressed: bool,
    pub move_right_pressed: bool,

    pub move_sprint_just_pressed: bool,
    pub move_sprint_pressed: bool,

    pub move_crouch_just_pressed: bool,
    pub move_crouch_pressed: bool,

    pub move_jump_just_pressed: bool,
    pub move_jump_pressed: bool,

    pub interact_use_just_pressed: bool,
    pub interact_use_pressed: bool,
}

pub fn update_keymap_state(
    keymap: Res<KeymapResource>,
    mut keymap_state: ResMut<KeymapState>,
    windows: Query<&mut Window>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let window = match windows.get_single() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    if !window.focused {
        return;
    }

    match keymap.move_forward_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_forward_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_forward_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_backward_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_backward_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_backward_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_left_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_left_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_left_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_right_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_right_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_right_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_sprint_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_sprint_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_sprint_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_crouch_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_crouch_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_crouch_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.move_jump_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.move_jump_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.move_jump_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }

    match keymap.interact_use_bind {
        super::Input::Keycode(keycode) => {
            keymap_state.interact_use_just_pressed = keyboard_input.just_pressed(keycode);
            keymap_state.interact_use_pressed = keyboard_input.pressed(keycode);
        }
        _ => {}
    }
}
