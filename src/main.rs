use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player_character;
use crate::player_character::PlayerCharacterPlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0.0, -14.0, 0.0),
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PlayerCharacterPlugin)
        .add_startup_system(setup)
        .add_system(reload_map_system)
        .run();
}

#[derive(Component)]
pub struct MapCollider;

#[derive(Component)]
pub struct MapObject;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Spawn map
    {
        commands
            .spawn(SceneBundle {
                scene: asset_server.load("Map.glb#Scene0"),
                ..default()
            })
            .insert(MapObject);
        generate_map(&mut commands, &asset_server);
    }

    player_character::spawn_player(commands, meshes, materials);
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CubeCollider {
    pub origin: [f32; 3],
    pub rotation: [f32; 3],

    pub size_x: f32,
    pub size_y: f32,
    pub size_z: f32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MapInformation {
    cube_colliders: Vec<CubeCollider>,
}

fn generate_map(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    /*commands
    .spawn(SceneBundle {
        scene: asset_server.load("Map.glb#Scene0"),
        ..default()
    })
    .insert(MapObject);*/

    let contents = std::fs::read_to_string("assets/Map_information.json")
    .expect("Should have been able to read the file");
    let map_info: MapInformation = serde_json::from_str(&contents).unwrap();

    for cube_collider in map_info.cube_colliders {
        commands
            .spawn(Collider::cuboid(
                cube_collider.size_x,
                cube_collider.size_y,
                cube_collider.size_z,
            ))
            .insert(TransformBundle::from(
                Transform::from_xyz(
                    cube_collider.origin[0],
                    cube_collider.origin[1],
                    cube_collider.origin[2],
                )
                .with_rotation(Quat::from_scaled_axis(Vec3::new(
                    cube_collider.rotation[0],
                    cube_collider.rotation[1],
                    cube_collider.rotation[2],
                ))),
            ))
            .insert(MapCollider);
    }
}

fn reload_map_system(
    mut commands: Commands,
    collider_query: Query<Entity, With<MapCollider>>,
    //object_query: Query<Entity, (With<MapObject>, Without<MapCollider>)>,
    keys: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
) {
    for key in keys.get_pressed() {
        if *key == KeyCode::R {
            for map_collider_entity in collider_query.iter() {
                commands.entity(map_collider_entity).despawn();
            }

            /*for map_object_entity in object_query.iter() {
                commands.entity(map_object_entity).despawn();
            }*/

            generate_map(&mut commands, &asset_server);

            println!("Reloaded map");
        }
    }
}
