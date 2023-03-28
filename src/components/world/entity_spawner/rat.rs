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
    asset_server: Res<AssetServer>,
) {
    for event in events.iter() {
        if let EntitySpawnEvent::SpawnRat(data) = event {
            commands
                .spawn(SpatialBundle::from_transform(
                    Transform::from_translation(data.location)
                        .with_rotation(Quat::from_scaled_axis(data.rotation)),
                ))
                .insert(RigidBody::Dynamic)
                .insert(Collider::cuboid(
                    data.size,
                    data.size / 2.0,
                    data.size * 2.0,
                ))
                .insert(Damping {
                    angular_damping: 6.0,
                    linear_damping: 1.0,
                    ..default()
                })
                .insert(Velocity::default())
                .insert(GravityScale(data.size * 6.0))
                .insert(PickableObject)
                .insert(RatMovementBehaviour)
                .insert(InGameEntity)
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::new(0.0, -50.0, 0.0),
                    mass: 100.0,
                    principal_inertia_local_frame: Quat::IDENTITY,
                    principal_inertia: Vec3::new(0.0, 0.0, 0.0),
                }))
                .with_children(|parent| {
                    parent.spawn(SceneBundle {
                        scene: asset_server.load("models/rat.glb#Scene0"),
                        transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0))
                            .with_translation(Vec3::new(0.0, -0.5, 0.0))
                            .with_scale(Vec3::new(data.size, data.size, data.size)),
                        visibility: Visibility::Visible,
                        ..default()
                    });
                });
        }
    }
}
