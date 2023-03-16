use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::MaterialMesh2dBundle,
};

use crate::post_processing::PostProcessingMaterial;

use super::{InGameEntity, InGameState};

#[derive(Component)]
pub struct MainCube;

pub fn setup_view(
    mut commands: Commands,
    windows: Query<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
) {
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/Characters/CameraMan/CameraMan.glb#Scene0"),
            transform: Transform::from_scale(Vec3::new(5.0, 5.0, 5.0)),
            ..default()
        })
        .insert(MainCube)
        .insert(InGameEntity);

    // Light
    // NOTE: Currently lights are ignoring render layers - see https://github.com/bevyengine/bevy/issues/3462
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 10.0)),
            ..default()
        })
        .insert(InGameEntity);

    // Main camera, first to render
    commands
        .spawn((
            Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::WHITE),
                    ..default()
                },
                camera: Camera {
                    target: RenderTarget::Image(image_handle.clone()),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                    .looking_at(Vec3::default(), Vec3::Y),
                ..default()
            },
            // Disable UI rendering for the first pass camera. This prevents double rendering of UI at
            // the cost of rendering the UI without any post processing effects.
            UiCameraConfig { show_ui: false },
        ))
        .insert(InGameEntity);

    // This specifies the layer used for the post processing camera, which will be attached to the post processing camera and 2d quad.
    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    // This material has the texture that has been rendered.
    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        source_image: image_handle,
        wash_color: 3.0,
    });

    // Post processing 2d quad, with material using the render texture done by the main camera, with a custom shader.
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: quad_handle.into(),
                material: material_handle,
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.5),
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

pub fn main_camera_cube_rotator_system(
    ingame_state: Res<InGameState>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCube>>,
) {
    if *ingame_state != InGameState::Running {
        return;
    }

    for mut transform in &mut query {
        transform.rotate_x(0.55 * time.delta_seconds());
        transform.rotate_z(0.15 * time.delta_seconds());
    }
}
