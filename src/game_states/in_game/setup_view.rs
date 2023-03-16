use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::RenderTarget,
};

use bevy_rapier3d::prelude::*;

use crate::components::player::PlayerBody;

use super::{InGameEntity, MainCameraData};

pub fn create_camera(
    mut commands: Commands,
    main_camera_data: Res<MainCameraData>,
    asset_server: Res<AssetServer>,
) {
    commands
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
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..default()
            },
            // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
            // the cost of rendering the UI without any post processing effects.
            UiCameraConfig { show_ui: false },
        ));

    /*commands
    .spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    })
    .insert(InGameEntity)
    .insert(Collider::cuboid(1.0, 1.0, 1.0))
    .insert(PlayerBody);*/

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/Characters/CameraMan/CameraMan.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0))
                .with_translation(Vec3::new(0.0, 15.0, 0.0)),
            ..default()
        })
        .insert(InGameEntity)
        .with_children(|parent| {
            parent
                .spawn(Collider::cuboid(0.35, 1.5, 0.35))
                .insert(Transform::from_translation(Vec3::new(-1.0, -6.0, 0.0)));
            parent
                .spawn(Collider::cuboid(0.35, 1.5, 0.35))
                .insert(Transform::from_translation(Vec3::new(1.0, -6.0, 0.0)));

                /*parent.spawn((
                    Camera3dBundle {
                        camera_3d: Camera3d {
                            clear_color: ClearColorConfig::Custom(Color::WHITE),
                            ..default()
                        },
                        camera: Camera {
                            target: RenderTarget::Image(main_camera_data.image_handle.clone()),
                            ..default()
                        },
                        transform: Transform::from_translation(Vec3::new(0.0, 1.0, -1.0)).with_rotation(Quat::from_rotation_y(3.141593)),
                        ..default()
                    },
                    // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
                    // the cost of rendering the UI without any post processing effects.
                    UiCameraConfig { show_ui: false },
                ));*/
        })
        .insert(PlayerBody);

    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_translation(Vec3::new(0.0, -2.5, 0.0)));
}
