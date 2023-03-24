use bevy::prelude::*;

use crate::game_states::AppState;

mod player;
pub use player::PlayerSpawnData;

mod post_camera;
pub use post_camera::PostCameraSpawnData;

mod test_cube;
pub use test_cube::TestCubeSpawnData;

mod rat;
pub use rat::RatSpawnData;

pub enum EntitySpawnEvent {
    SpawnPlayer(player::PlayerSpawnData),
    SpawnPostCamera(post_camera::PostCameraSpawnData),
    SpawnTestCube(test_cube::TestCubeSpawnData),
    SpawnRat(rat::RatSpawnData),
}

pub struct EntitySpawnPlugin;

impl Plugin for EntitySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EntitySpawnEvent>()
            .add_system(post_camera::handle_camera_spawn.in_set(OnUpdate(AppState::InGame)))
            .add_system(player::handle_player_spawn.in_set(OnUpdate(AppState::InGame)))
            .add_system(test_cube::handle_test_cube_spawn.in_set(OnUpdate(AppState::InGame)))
            .add_system(rat::handle_rat_spawn.in_set(OnUpdate(AppState::InGame)));
    }
}
