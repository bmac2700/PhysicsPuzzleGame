use crate::{
    components::player::{PlayerBody, PlayerCamera, PlayerModel},
    game_states::in_game::{InGameEntity, MainCameraData},
};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, render::camera::RenderTarget,
};
use bevy_rapier3d::prelude::*;

use super::EntitySpawnEvent;

#[derive(Clone, Copy)]
pub struct PlayerSpawnData {
    pub location: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
}

pub fn handle_player_spawn(
    mut commands: Commands,
    mut events: EventReader<EntitySpawnEvent>,
    main_camera_data: Res<MainCameraData>,
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        if let EntitySpawnEvent::SpawnPlayer(data) = event {
            commands
                .spawn(SpatialBundle::from_transform(
                    Transform::from_translation(data.location)
                        /*.with_rotation(Quat::from_scaled_axis(data.rotation))*/,
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
                                    target: RenderTarget::Image(
                                        main_camera_data.image_handle.clone(),
                                    ),
                                    ..default()
                                },
                                transform: Transform::from_translation(Vec3::new(
                                    0.0,
                                    data.scale.y,
                                    0.0,
                                )),
                                ..default()
                            },
                            // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
                            // the cost of rendering the UI without any post processing effects.
                            UiCameraConfig { show_ui: false },
                        ))
                        .insert(PlayerCamera);

                    parent
                        .spawn(SceneBundle {
                            scene: asset_server
                                .load("models/Characters/CameraMan/CameraMan.glb#Scene0"),
                            transform: Transform::from_scale(data.scale)
                                .with_rotation(Quat::from_rotation_y(3.141593)),
                            visibility: Visibility::Hidden,
                            ..default()
                        })
                        .insert(PlayerModel);
                });
        }
    }
}
