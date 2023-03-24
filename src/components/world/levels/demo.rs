use bevy::prelude::*;

use crate::{
    components::world::{
        entity_spawner::{
            EntitySpawnEvent, PlayerSpawnData, PostCameraSpawnData, RatSpawnData, TestCubeSpawnData,
        },
        hitbox_loader::{load_hitboxes, MapCollision},
    },
    game_states::in_game::InGameEntity,
};

pub fn load_world(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    entity_spawner: &mut EventWriter<EntitySpawnEvent>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.318242, 0.318466, 0.567203),
        brightness: 100.0,
    });

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("levels/demo/world.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..default()
        })
        .insert(InGameEntity);

    let map_collision_contents = std::fs::read_to_string("assets/levels/demo/map_collision.json")
        .expect("Unable to find demo map data");

    let map_collision: MapCollision = match serde_json::from_str(&map_collision_contents) {
        Ok(v) => v,
        Err(_) => {
            println!("Unable to parse map collision data");
            return;
        }
    };

    load_hitboxes(commands, map_collision);

    entity_spawner.send(EntitySpawnEvent::SpawnPostCamera(PostCameraSpawnData {
        location: Vec3::new(0.0, 0.0, 1.5),
    }));

    entity_spawner.send(EntitySpawnEvent::SpawnPlayer(PlayerSpawnData {
        location: Vec3::new(0.0, 15.0, -35.0),
        rotation: Vec3::new(0.0, -1.570796, 0.0),
    }));

    entity_spawner.send(EntitySpawnEvent::SpawnTestCube(TestCubeSpawnData {
        location: Vec3::new(25.0, 5.0, -35.0),
        rotation: Vec3::new(0.0, 0.0, 0.0),
        size: 5.0,
    }));

    entity_spawner.send(EntitySpawnEvent::SpawnRat(RatSpawnData {
        location: Vec3::new(25.0, 7.0, -35.0),
        rotation: Vec3::new(0.0, 0.0, 0.0),
        size: 1.0,
    }));
}
