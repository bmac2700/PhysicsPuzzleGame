mod setup_camera;
use bevy_rapier3d::prelude::RapierConfiguration;
pub use setup_camera::*;

pub mod ui;

use bevy::{prelude::*, window::CursorGrabMode};

#[derive(Resource, Clone, Copy, PartialEq, PartialOrd)]
pub enum InGameState {
    Running,
    Paused,
}

#[derive(Component)]
pub struct InGameEntity;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum GamePauseEvent {
    Pause,
    Unpause,
}

pub fn game_pause(
    mut ingame_state: ResMut<InGameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut game_pause: EventWriter<GamePauseEvent>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut windows: Query<&mut Window>,
) {
    let mut window = match windows.get_single_mut() {
        Ok(v) => v,
        Err(_) => {
            return;
        }
    };

    if !window.focused && *ingame_state == InGameState::Running {
        pause_game(
            &mut ingame_state,
            &mut game_pause,
            &mut rapier_configuration,
            &mut window,
        );
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *ingame_state == InGameState::Paused {
            unpause_game(
                &mut ingame_state,
                &mut game_pause,
                &mut rapier_configuration,
                &mut window,
            );
        } else {
            pause_game(
                &mut ingame_state,
                &mut game_pause,
                &mut rapier_configuration,
                &mut window,
            );
        }
    }
}

pub fn pause_game(
    ingame_state: &mut InGameState,
    game_pause: &mut EventWriter<GamePauseEvent>,
    rapier_configuration: &mut RapierConfiguration,
    window: &mut Window,
) {
    *ingame_state = InGameState::Paused;
    game_pause.send(GamePauseEvent::Pause);
    rapier_configuration.physics_pipeline_active = false;
    window.cursor.grab_mode = CursorGrabMode::None;
    window.cursor.visible = true;
}

pub fn unpause_game(
    ingame_state: &mut InGameState,
    game_pause: &mut EventWriter<GamePauseEvent>,
    rapier_configuration: &mut RapierConfiguration,
    window: &mut Window,
) {
    *ingame_state = InGameState::Running;
    game_pause.send(GamePauseEvent::Unpause);
    rapier_configuration.physics_pipeline_active = true;
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}
