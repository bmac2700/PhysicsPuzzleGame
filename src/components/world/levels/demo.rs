use bevy::prelude::*;

use crate::{
    components::world::entity_spawner::{EntitySpawnEvent, PlayerSpawnData},
    game_states::in_game::InGameEntity,
};

pub fn load_world(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_spawner: &mut EventWriter<EntitySpawnEvent>,
) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("levels/demo/world.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        })
        .insert(InGameEntity);

    /*entity_spawner.send(EntitySpawnEvent::SpawnPlayer(PlayerSpawnData {
        location: Vec3::new(0.0, 10.0, 0.0),
        rotation: Vec3::new(0.0, 0.0, 0.0),
        scale: Vec3::new(5.0, 5.0, 5.0),
    }))*/
}
