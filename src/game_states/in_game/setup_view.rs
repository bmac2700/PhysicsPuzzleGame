use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::{camera::RenderTarget},
};

use bevy_rapier3d::prelude::*;

use crate::components::player::{PlayerBody, PlayerCamera};

use super::{InGameEntity, MainCameraData};

pub fn create_camera(
    mut commands: Commands,
    main_camera_data: Res<MainCameraData>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ))
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
                        transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
                        ..default()
                    },
                    // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
                    // the cost of rendering the UI without any post processing effects.
                    UiCameraConfig { show_ui: false },
                ))
                .insert(PlayerCamera);

            parent.spawn(SceneBundle {
                scene: asset_server.load("models/Characters/CameraMan/CameraMan.glb#Scene0"),
                transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0))
                    .with_translation(Vec3::new(0.0, 15.0, 0.0)),
                ..default()
            });
        });

    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(Transform::from_translation(Vec3::new(0.0, -2.5, 0.0)));
}
