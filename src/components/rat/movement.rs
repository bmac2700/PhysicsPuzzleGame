use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::RatMovementBehaviour;

pub fn rat_movement(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Velocity), With<RatMovementBehaviour>>,
    rapier_context: Res<RapierContext>,
) {
    for (rat_transform, mut rat_velocity) in query.iter_mut() {
        let local_z = rat_transform.local_z();

        let forwardmove = -Vec3::new(local_z.x, 0.0, local_z.z);

        let mut new_velocity = Vec3::ZERO;

        new_velocity += forwardmove;

        let on_ground = rapier_context.cast_ray(
            rat_transform.translation,
            Vec3::NEG_Y,
            2.0,
            true,
            QueryFilter::only_fixed(),
        );

        let mut velocity_2d = rat_velocity.linvel;
        velocity_2d.y = 0.0;

        if on_ground.is_some() || velocity_2d.length() < 1.0 {
            rat_velocity.linvel += new_velocity * time.delta_seconds() * 50.0;
        }

        let mut ray_start = rat_transform.translation;

        if let Some((_, toi)) = on_ground {
            ray_start.y += -1.0 * toi + 0.2;
        }

        let in_front = rapier_context
            .cast_ray(ray_start, forwardmove, 5.0, true, QueryFilter::new())
            .is_some();

        if in_front || rat_velocity.linvel.length() < 1.0{
            if rand::random::<bool>() {
                rat_velocity.angvel.y += 2.5;
            } else {
                rat_velocity.angvel.y -= 2.5;
            }

            if rat_velocity.linvel.length() < 1.0 {
                rat_velocity.linvel.y += 2.0;
            }
        }
    }
}
