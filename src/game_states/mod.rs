use bevy::prelude::*;

pub mod in_game;
pub mod menu;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    MainMenu,
    OptionsMenu,

    InGame,
    Paused,
}
