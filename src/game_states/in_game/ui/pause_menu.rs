use bevy::prelude::*;

use crate::game_states::{
    in_game::{GamePauseEvent, InGameEntity, InGameState},
    menu::{ButtonData, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON},
    AppState,
};

#[derive(Resource)]
pub struct PauseMenuData {
    menu_area: Option<Entity>,
    back_to_menu_button: Option<Entity>,
    resume_button: Option<Entity>,
}

impl Default for PauseMenuData {
    fn default() -> Self {
        Self {
            menu_area: None,
            back_to_menu_button: None,
            resume_button: None,
        }
    }
}

pub fn setup_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut pause_menu_data: ResMut<PauseMenuData>,
    mut pause_events: EventReader<GamePauseEvent>,
) {
    for event in pause_events.iter() {
        if *event == GamePauseEvent::Pause {
            let menu_area = commands
                .spawn(NodeBundle {
                    style: Style {
                        // center button
                        size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        //align_items: AlignItems::Center,
                        position: UiRect {
                            left: Val::Px(0.0),
                            ..Default::default()
                        },
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    background_color: Color::rgba(0.15, 0.15, 0.15, 0.85).into(),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Pause menu",
                        TextStyle {
                            font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .id();

            let back_to_menu_button = commands
                .spawn(NodeBundle {
                    style: Style {
                        // center button
                        size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        //align_items: AlignItems::Center,
                        position: UiRect {
                            left: Val::Px(0.0),
                            top: Val::Percent(85.0),
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
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
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
                                "To main menu",
                                TextStyle {
                                    font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                })
                .id();

            let resume_button = commands
                .spawn(NodeBundle {
                    style: Style {
                        // center button
                        size: Size::new(Val::Percent(40.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::Center,
                        //align_items: AlignItems::Center,
                        position: UiRect {
                            left: Val::Px(0.0),
                            top: Val::Percent(10.0),
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
                                size: Size::new(Val::Px(350.0), Val::Px(65.0)),
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
                                "Resume",
                                TextStyle {
                                    font: asset_server.load("fonts/VCR_OSD_MONO_1.001.ttf"),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        });
                })
                .id();

            (*pause_menu_data).back_to_menu_button = Some(back_to_menu_button);
            (*pause_menu_data).menu_area = Some(menu_area);
            (*pause_menu_data).resume_button = Some(resume_button);
        }

        if *event == GamePauseEvent::Unpause {
            despawn_pause_menu(&mut commands, &mut pause_menu_data);
        }
    }
}

fn despawn_pause_menu(commands: &mut Commands, pause_menu_data: &mut PauseMenuData) {
    if let Some(ent) = pause_menu_data.menu_area {
        commands.entity(ent).despawn_recursive();
    }

    if let Some(ent) = pause_menu_data.back_to_menu_button {
        commands.entity(ent).despawn_recursive();
    }

    if let Some(ent) = pause_menu_data.resume_button {
        commands.entity(ent).despawn_recursive();
    }

    pause_menu_data.menu_area = None;
    pause_menu_data.back_to_menu_button = None;
    pause_menu_data.resume_button = None;
}

pub fn handle_pause_menu_input(
    mut commands: Commands,
    mut ingame_state: ResMut<InGameState>,
    mut game_pause: EventWriter<GamePauseEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonData),
        (Changed<Interaction>, With<Button>, With<ButtonData>),
    >,
    entity_query: Query<Entity, With<InGameEntity>>,
    mut pause_menu_data: ResMut<PauseMenuData>,
) {
    if *ingame_state == InGameState::Paused {
        for (interaction, mut color, button_data) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                    if button_data.button_id == 0 {
                        game_pause.send(GamePauseEvent::Unpause);
                        for entity in entity_query.iter() {
                            commands.entity(entity).despawn_recursive();
                        }
                        despawn_pause_menu(&mut commands, &mut pause_menu_data);
                        *ingame_state = InGameState::Running;
                        next_state.set(AppState::MainMenu);
                    }

                    if button_data.button_id == 1 {
                        *ingame_state = InGameState::Running;
                        game_pause.send(GamePauseEvent::Unpause);
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
}
