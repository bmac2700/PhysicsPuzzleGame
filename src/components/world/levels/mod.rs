use bevy::prelude::*;

use crate::game_states::{in_game::InGameEntity, AppState};

use super::entity_spawner::EntitySpawnEvent;
pub mod demo;

pub enum Level {
    Demo,
}

pub enum LevelChangeEvent {
    ChangeLevel(Level),
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LevelChangeEvent>()
            .add_system(level_changer.in_set(OnUpdate(AppState::InGame)))
            .add_system(level_debug.in_set(OnUpdate(AppState::InGame)));
    }
}

fn level_debug(
    keyboard_input: Res<Input<KeyCode>>,
    mut level_change_event: EventWriter<LevelChangeEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        level_change_event.send(LevelChangeEvent::ChangeLevel(Level::Demo));
        println!("NEW EVENT");
    }
}

fn level_changer(
    mut commands: Commands,
    mut level_change_events: EventReader<LevelChangeEvent>,
    entity_query: Query<Entity, With<InGameEntity>>,
    asset_server: Res<AssetServer>,
    mut entity_spawner: EventWriter<EntitySpawnEvent>,
) {
    for event in level_change_events.iter() {
        for entity in entity_query.iter() {
            //commands.entity(entity).despawn_recursive();
        }

        if let LevelChangeEvent::ChangeLevel(level) = event {
            match level {
                Level::Demo => {
                    println!("Loading world");
                    demo::load_world(&mut commands, &asset_server, &mut entity_spawner);
                }
            }
        }
    }
}
