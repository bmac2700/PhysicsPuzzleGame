use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::RenderTarget,
};

use bevy_rapier3d::prelude::*;

use crate::components::player::{PickableObject, PlayerBody, PlayerCamera, PlayerModel};

use super::{InGameEntity, MainCameraData};

pub fn create_camera(
    mut commands: Commands,
    main_camera_data: Res<MainCameraData>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            Vec3::new(0.0, 10.0, 0.0),
        )))
        .insert((InGameEntity, PlayerBody))
        .with_children(|parent| {
            parent
                .spawn(Collider::cuboid(0.35, 1.5, 0.35))
                .insert(Transform::from_translation(Vec3::new(-1.0, -6.0, 0.0)));
            parent
                .spawn(Collider::cuboid(0.35, 1.5, 0.35))
                .insert(Transform::from_translation(Vec3::new(1.0, -6.0, 0.0)));

            parent
                .spawn((
                    Camera3dBundle {
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::Custom(Color::WHITE),
                            ..default()
                        },
                        camera: Camera {
                            target: RenderTarget::Image(main_camera_data.image_handle.clone()),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
                        ..default()
                    },
                    // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
                    // the cost of rendering the UI without any post processing effects.
                    UiCameraConfig { show_ui: false },
                ))
                .insert(PlayerCamera);

            parent
                .spawn(SceneBundle {
                    scene: asset_server.load("models/Characters/CameraMan/CameraMan.glb#Scene0"),
                    transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0))
                        .with_rotation(Quat::from_rotation_y(3.141593)),
                    visibility: Visibility::Hidden,
                    ..default()
                })
                .insert(PlayerModel);
        });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 5.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(2.5, 2.5, 2.5))
        .insert(Damping::default())
        .insert(Velocity::default())
        .insert(PickableObject);

    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_translation(Vec3::new(0.0, -2.5, 0.0)));
}
