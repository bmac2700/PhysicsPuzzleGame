use bevy::prelude::*;

mod keymap_state;

pub use keymap_state::KeymapState;

use crate::game_states::AppState;

#[allow(dead_code)]
pub enum Input {
    NoBind,
    Keycode(KeyCode),
}

#[derive(Resource)]
pub struct KeymapResource {
    pub move_forward_bind: Input,
    pub move_backward_bind: Input,
    pub move_left_bind: Input,
    pub move_right_bind: Input,
    pub move_sprint_bind: Input,
    pub move_crouch_bind: Input,
    pub move_jump_bind: Input,

    pub interact_use_bind: Input,
}

impl Default for KeymapResource {
    fn default() -> Self {
        Self {
            move_forward_bind: Input::Keycode(KeyCode::W),
            move_backward_bind: Input::Keycode(KeyCode::S),
            move_left_bind: Input::Keycode(KeyCode::A),
            move_right_bind: Input::Keycode(KeyCode::D),
            move_sprint_bind: Input::Keycode(KeyCode::LShift),
            move_crouch_bind: Input::Keycode(KeyCode::LControl),
            move_jump_bind: Input::Keycode(KeyCode::Space),

            interact_use_bind: Input::Keycode(KeyCode::E),
        }
    }
}

pub struct KeymapPlugin;

impl Plugin for KeymapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeymapResource>()
            .init_resource::<keymap_state::KeymapState>()
            .add_system(keymap_state::update_keymap_state.in_set(OnUpdate(AppState::InGame)));
    }
}
