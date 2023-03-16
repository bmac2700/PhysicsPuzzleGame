mod setup_view;
use bevy_rapier3d::prelude::RapierConfiguration;
pub use setup_view::*;

pub mod ui;

use bevy::prelude::*;

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
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if *ingame_state == InGameState::Paused {
            *ingame_state = InGameState::Running;
            game_pause.send(GamePauseEvent::Unpause);
            rapier_configuration.physics_pipeline_active = true;
        } else {
            *ingame_state = InGameState::Paused;
            game_pause.send(GamePauseEvent::Pause);
            rapier_configuration.physics_pipeline_active = false;
        }
    }
}
