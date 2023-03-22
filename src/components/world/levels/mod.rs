use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
            commands.entity(entity).despawn_recursive();
        }

        commands.insert_resource(AmbientLight {
            color: Color::rgb(0.318242, 0.318466, 0.567203),
            brightness: 100.0,
        });

        commands
            .spawn(Collider::cuboid(100.0, 0.1, 100.0))
            .insert(Transform::from_translation(Vec3::new(0.0, -2.5, 0.0)))
            .insert(InGameEntity);

        let LevelChangeEvent::ChangeLevel(level) = event;

        match level {
            Level::Demo => {
                demo::load_world(&mut commands, &asset_server, &mut entity_spawner);
            }
        }
    }
}
