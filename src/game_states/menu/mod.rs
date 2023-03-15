use bevy::prelude::*;

pub mod main_menu;
pub mod options_menu;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct ButtonData {
    pub button_id: u32,
}

#[derive(Component)]
pub struct MenuCamera;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default()).insert(MenuCamera);
}

pub fn remove_camera(mut commands: Commands, camera_query: Query<Entity, With<MenuCamera>>) {
    for camera_entity in camera_query.iter() {
        commands.entity(camera_entity).despawn();
    }
}
