use bevy::prelude::*;

use crate::game_states::AppState;

mod player;
pub use player::PlayerSpawnData;

pub enum EntitySpawnEvent {
    SpawnPlayer(player::PlayerSpawnData),
}

pub struct EntitySpawnPlugin;

impl Plugin for EntitySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntitySpawnEvent>()
            .add_system(player::handle_player_spawn.in_set(OnUpdate(AppState::InGame)));
    }
}
