use bevy::{prelude::*, sprite::Material2dPlugin};
use game_states::{
    in_game::{ui::pause_menu::PauseMenuData, GamePauseEvent, InGameState},
    AppState,
};
use post_processing::PostProcessingMaterial;

mod game_states;
mod post_processing;

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bowens' handyman service".into(),
                // Tells wasm to resize the window according to the available canvas
                fit_canvas_to_parent: true,
                // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_event::<GamePauseEvent>()
        .insert_resource(InGameState::Running)
        .insert_resource(PauseMenuData::default())
        .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        .add_system(game_states::menu::setup_camera.on_startup())
        //MainMenu
        .add_system(
            game_states::menu::main_menu::setup_menu.in_schedule(OnEnter(AppState::MainMenu)),
        )
        .add_system(game_states::menu::main_menu::menu_input.in_set(OnUpdate(AppState::MainMenu)))
        .add_system(game_states::menu::setup_camera.in_schedule(OnExit(AppState::InGame)))
        .add_system(
            game_states::menu::main_menu::cleanup_menu.in_schedule(OnExit(AppState::MainMenu)),
        )
        //Options menu
        .add_system(
            game_states::menu::options_menu::setup_menu.in_schedule(OnEnter(AppState::OptionsMenu)),
        )
        .add_system(
            game_states::menu::options_menu::menu_input.in_set(OnUpdate(AppState::OptionsMenu)),
        )
        .add_system(
            game_states::menu::options_menu::cleanup_menu
                .in_schedule(OnExit(AppState::OptionsMenu)),
        )
        //InGame
        .add_system(game_states::menu::remove_camera.in_schedule(OnEnter(AppState::InGame)))
        .add_system(game_states::in_game::setup_view.in_schedule(OnEnter(AppState::InGame)))
        .add_system(
            game_states::in_game::main_camera_cube_rotator_system
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_system(game_states::in_game::game_pause.in_set(OnUpdate(AppState::InGame)))
        .add_system(
            game_states::in_game::ui::pause_menu::setup_pause_menu
                .in_set(OnUpdate(AppState::InGame)),
        )
        .add_system(
            game_states::in_game::ui::pause_menu::handle_pause_menu_input
                .in_set(OnUpdate(AppState::InGame)),
        )
        .run();
}
