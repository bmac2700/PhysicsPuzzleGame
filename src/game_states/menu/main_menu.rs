use bevy::{prelude::*, window::CursorGrabMode};

use crate::game_states::AppState;

use super::{ButtonData, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON};

#[derive(Resource)]
pub struct MainMenuData {
    play_button_entity: Entity,
    options_button_entity: Entity,
    exit_button_entity: Entity,

    logo_text_entity: Entity,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let play_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position: UiRect {
                    top: Val::Percent(35.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(175.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonData { button_id: 0 })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();

    let options_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                //align_items: AlignItems::Center,
                position: UiRect {
                    top: Val::Percent(50.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(175.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonData { button_id: 1 })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Options",
                        TextStyle {
                            font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();

    let exit_button_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                //align_items: AlignItems::Center,
                position: UiRect {
                    top: Val::Percent(65.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(175.0), Val::Px(65.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                })
                .insert(ButtonData { button_id: 2 })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        })
        .id();

    let logo_text_entity = commands
        .spawn(NodeBundle {
            style: Style {
                // center button
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                //align_items: AlignItems::Center,
                position: UiRect {
                    top: Val::Percent(20.0),
                    ..Default::default()
                },
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Bowens' handyman service",
                TextStyle {
                    font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .id();

    commands.insert_resource(MainMenuData {
        play_button_entity,
        options_button_entity,
        exit_button_entity,
        logo_text_entity,
    });
}

pub fn cleanup_menu(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands
        .entity(menu_data.play_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.options_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.exit_button_entity)
        .despawn_recursive();
    commands
        .entity(menu_data.logo_text_entity)
        .despawn_recursive();
}

pub fn menu_input(
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonData),
        (Changed<Interaction>, With<Button>, With<ButtonData>),
    >,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    for (interaction, mut color, button_data) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                if button_data.button_id == 0 {
                    next_state.set(AppState::InGame);
                    window.cursor.grab_mode = CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }

                if button_data.button_id == 1 {
                    next_state.set(AppState::OptionsMenu);
                }

                if button_data.button_id == 2 {
                    std::process::exit(0);
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
