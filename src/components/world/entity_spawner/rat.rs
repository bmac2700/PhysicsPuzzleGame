use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    components::{player::PickableObject, rat::RatMovementBehaviour},
    game_states::in_game::InGameEntity,
};

use super::EntitySpawnEvent;

#[derive(Clone, Copy)]
pub struct RatSpawnData {
    pub location: Vec3,
    pub rotation: Vec3,
    pub size: f32,
}

pub fn handle_rat_spawn(
    mut commands: Commands,
    mut events: EventReader<EntitySpawnEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for event in events.iter() {
        if let EntitySpawnEvent::SpawnRat(data) = event {
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
                )   )
                .insert(Damping {
                    angular_damping: 6.0,
                    linear_damping: 1.0,
                    ..default()
                })
                .insert(Velocity::default())
                .insert(GravityScale(data.size))
                .insert(PickableObject)
                .insert(RatMovementBehaviour)
                .insert(InGameEntity)
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::new(0.0, -50.0, 0.0),
                    mass: 10.0,
                    principal_inertia_local_frame: Quat::IDENTITY,
                    principal_inertia: Vec3::new(0.0, 0.0, 0.0)
                }));
        }
    }
}
