use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{components::player::PickableObject, game_states::in_game::InGameEntity};

use super::EntitySpawnEvent;

#[derive(Clone, Copy)]
pub struct TestCubeSpawnData {
    pub location: Vec3,
    pub rotation: Vec3,
    pub size: f32,
}

pub fn handle_test_cube_spawn(
    mut commands: Commands,
    mut events: EventReader<EntitySpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.iter() {
        if let EntitySpawnEvent::SpawnTestCube(data) = event {
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: data.size })),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_translation(data.location)
                        .with_rotation(Quat::from_scaled_axis(data.rotation)),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(
                    data.size / 2.0,
                    data.size / 2.0,
                    data.size / 2.0,
                ))
                .insert(Damping::default())
                .insert(GravityScale(data.size))
                .insert(Velocity::default())
                .insert(PickableObject)
                .insert(InGameEntity);
        }
    }
}
