mod setup_view;
pub use setup_view::*;

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
    let mut window = windows.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *ingame_state == InGameState::Paused {
            *ingame_state = InGameState::Running;
            game_pause.send(GamePauseEvent::Unpause);
            rapier_configuration.physics_pipeline_active = true;
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        } else {
            *ingame_state = InGameState::Paused;
            game_pause.send(GamePauseEvent::Pause);
            rapier_configuration.physics_pipeline_active = false;
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}
