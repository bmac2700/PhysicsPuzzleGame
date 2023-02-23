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
pub struct MapHitbox;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Collider::cuboid(4.0, 0.001, 2.5));

    commands.spawn(Collider::cuboid(4.0, 0.001, 2.5))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, -4.0).with_rotation(Quat::from_rotation_x(0.279253))));

    //commands.spawn(Transform::from_translation(Vec3::new(0.0, 0.25, 0.0))).insert(Collider::cuboid(2.0, 0.001, 1.0));

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

fn generate_hitbox(
    mut query: Query<&Handle<Scene>, With<MapHitbox>>,
    scenes: ResMut<Assets<Scene>>,
    meshes: ResMut<Assets<Mesh>>,
) {
    for scene in query.iter() {
        let scene = scenes.get(scene).unwrap();
    }
}
