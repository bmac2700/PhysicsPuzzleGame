#[derive(serde::Serialize, serde::Deserialize)]
pub struct RectangleHitbox {
    pub size: (f32, f32, f32),     //XYZ
    pub rotation: (f32, f32, f32), //XYZ
    pub location: (f32, f32, f32), //XYZ
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum HitboxType {
    Rectangle(RectangleHitbox),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct MapCollision {
    pub hitboxes: Vec<HitboxType>,
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game_states::in_game::InGameEntity;

pub fn load_hitboxes(commands: &mut Commands, map_collision: MapCollision) {
    for hitbox in map_collision.hitboxes {
        match hitbox {
            HitboxType::Rectangle(v) => {
                commands
                    .spawn(TransformBundle::from_transform(
                        Transform::from_xyz(v.location.0, v.location.1, v.location.2)
                            .with_rotation(Quat::from_scaled_axis(Vec3::new(
                                v.rotation.0,
                                v.rotation.1,
                                v.rotation.2,
                            ))),
                    ))
                    .insert(InGameEntity)
                    .insert(Collider::cuboid(v.size.0, v.size.1, v.size.2));
            }
        }
    }
}
