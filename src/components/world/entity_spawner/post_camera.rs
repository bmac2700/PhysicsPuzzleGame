use bevy::{
    prelude::*,
    render::{render_resource::Extent3d, view::RenderLayers},
    sprite::MaterialMesh2dBundle,
};

use crate::{
    game_states::in_game::{InGameEntity, MainCameraData},
    post_processing::PostProcessingMaterial,
};

use super::EntitySpawnEvent;

#[derive(Clone, Copy)]
pub struct PostCameraSpawnData {
    pub location: Vec3,
}
pub fn handle_camera_spawn(
    mut commands: Commands,
    mut events: EventReader<EntitySpawnEvent>,
    mut main_camera_data: ResMut<MainCameraData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    windows: Query<&Window>,
) {
    for event in events.iter() {
        if let EntitySpawnEvent::SpawnPostCamera(data) = event {
            let window = windows.single();

            let size = Extent3d {
                width: window.resolution.physical_width(),
                height: window.resolution.physical_height(),
                ..default()
            };

            // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
            let post_processing_pass_layer =
                RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

            let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                size.width as f32,
                size.height as f32,
            ))));

            main_camera_data.mesh_handle = Some(quad_handle.clone());

            // This material has the texture that has been rendered.
            let material_handle = post_processing_materials.add(PostProcessingMaterial {
                source_image: main_camera_data.image_handle.clone(),
                wash_color: 3.0,
            });

            // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
            commands
                .spawn((
                    MaterialMesh2dBundle {
                        mesh: quad_handle.into(),
                        material: material_handle,
                        transform: Transform {
                            translation: data.location,
                            ..default()
                        },
                        ..default()
                    },
                    post_processing_pass_layer,
                ))
                .insert(InGameEntity);

            // The post-processing pass camera.
            commands
                .spawn((
                    Camera2dBundle {
                        camera: Camera {
                            // renders after the first main camera which has default value: 0.
                            order: 1,
                            ..default()
                        },
                        ..Camera2dBundle::default()
                    },
                    post_processing_pass_layer,
                ))
                .insert(InGameEntity);
        }
    }
}
