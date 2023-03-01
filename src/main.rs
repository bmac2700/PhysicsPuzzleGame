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
        .add_system(generate_hitbox)
        .run();
}

#[derive(Component)]
pub struct MapCollider;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    //Spawn map
    {
        commands.spawn(SceneBundle {
            scene: asset_server.load("test_map.glb#Scene0"),
            ..default()
        });

        /*commands
        .spawn(SceneBundle {
            scene: asset_server.load("test_map_bounds.glb#Scene0"),
            visibility: Visibility::VISIBLE,
            ..default()
        })
        .insert(MapHitbox);*/
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
pub struct MapBounds {
    cube_colliders: Vec<CubeCollider>,
}

fn generate_hitbox(mut commands: Commands) {
    let map_bounds = MapBounds {
        cube_colliders: vec![CubeCollider {
            origin: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0],
            size_x: 4.0,
            size_y: 0.001,
            size_z: 2.5,
        }],
    };

    for cube_collider in map_bounds.cube_colliders {
        commands
        .spawn(Collider::cuboid(cube_collider.size_x, cube_collider.size_y, cube_collider.size_z))
        .insert(TransformBundle::from(
            Transform::from_xyz(cube_collider.origin[0], cube_collider.origin[1], cube_collider.origin[2]).with_rotation(Quat::from_scaled_axis(Vec3::new(cube_collider.rotation[0], cube_collider.rotation[1], cube_collider.rotation[2]))),
        )).insert(MapCollider);
    }
}
