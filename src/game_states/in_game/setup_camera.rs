use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
    },
};

#[derive(Resource)]
pub struct MainCameraData {
    pub image_handle: Handle<Image>,
    pub mesh_handle: Option<Handle<Mesh>>,
}

pub fn update_main_camera_data(
    windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    main_camera_data: Res<MainCameraData>,
) {
    let window = windows.single();

    let size = Extent3d {
        width: window.resolution.physical_width(),
        height: window.resolution.physical_height(),
        ..default()
    };

    if let Some(image) = images.get(&main_camera_data.image_handle) {
        if image.size() != Vec2::new(size.width as f32, size.height as f32) {
            if let Some(mut_image) = images.get_mut(&main_camera_data.image_handle) {
                mut_image.resize(size);
                if let Some(mesh_handle) = &main_camera_data.mesh_handle {
                    if let Some(mesh) = meshes.get_mut(mesh_handle) {
                        *mesh = Mesh::from(shape::Quad::new(Vec2::new(
                            size.width as f32,
                            size.height as f32,
                        )));
                    }
                }
            }
        }
    }
}

pub fn setup_camera_resources(
    mut commands: Commands,
    windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
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

    commands.insert_resource(MainCameraData {
        image_handle,
        mesh_handle: None,
    });
}
