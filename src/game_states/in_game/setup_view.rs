use bevy::{
    core_pipeline::{
        bloom::{BloomCompositeMode, BloomSettings},
        clear_color::ClearColorConfig,
    },
    prelude::*,
    render::{camera::RenderTarget, view::ColorGrading},
};

use bevy_rapier3d::prelude::*;

use crate::components::player::{
    DroppedInventoryItem, PickableObject, PlayerBody, PlayerCamera, PlayerModel,
};

use super::{InGameEntity, MainCameraData};

pub fn create_camera(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    main_camera_data: Res<MainCameraData>,
    asset_server: Res<AssetServer>,
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
                            hdr: true,
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
                        color_grading: ColorGrading {
                            exposure: -7.5,
                            ..default()
                        },
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: 1.22173,
                            ..default()
                        }),
                        ..default()
                    },
                    // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
                    // the cost of rendering the UI without any post processing effects.
                    UiCameraConfig { show_ui: false },
                    BloomSettings {
                        intensity: 0.1,
                        composite_mode: BloomCompositeMode::Additive,
                        ..default()
                    },
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

    /*commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });*/

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
        .insert(GravityScale(6.0))
        .insert(Velocity::default())
        .insert(PickableObject);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.5 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 10.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(1.25, 1.25, 1.25))
        .insert(Damping::default())
        .insert(GravityScale(3.0))
        .insert(Velocity::default())
        .insert(DroppedInventoryItem {
            data: crate::components::player::PlayerInventoryItem::Empty,
        });

    commands.insert_resource(AmbientLight {
        color: Color::rgb(0.318242, 0.318466, 0.567203),
        brightness: 100.0,
    });

    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_translation(Vec3::new(0.0, -2.5, 0.0)));
}
