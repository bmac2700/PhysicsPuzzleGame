use bevy::{
    core_pipeline::{
        fxaa::{Fxaa, Sensitivity},
    },
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::*,
    window::CursorGrabMode,
};
use bevy_rapier3d::prelude::*;

pub struct PlayerCharacterPlugin;
impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .add_system(player_move)
            .add_system(player_look)
            .add_system(cursor_grab);
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            match window.cursor_grab_mode() {
                CursorGrabMode::None => {
                    window.set_cursor_grab_mode(CursorGrabMode::Confined);
                    window.set_cursor_visibility(false);
                }
                _ => {
                    window.set_cursor_grab_mode(CursorGrabMode::None);
                    window.set_cursor_visibility(true);
                }
            }
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

#[derive(Component)]
pub struct PlayerCharacter;

#[derive(Component)]
pub struct PlayerCharacterCamera;

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

fn player_look(
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: Query<&mut Transform, With<PlayerCharacter>>,
    mut camera_query: Query<
        &mut Transform,
        (With<PlayerCharacterCamera>, Without<PlayerCharacter>),
    >,
) {
    if let Some(window) = windows.get_primary() {
        if window.cursor_grab_mode() == CursorGrabMode::None {
            return;
        }
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                let window_scale = window.height().min(window.width());
                delta_state.pitch -= (0.00005 * ev.delta.y * window_scale).to_radians();
                delta_state.yaw -= (0.00005 * ev.delta.x * window_scale).to_radians();
                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw);

                let (_, mut camera_transform) = camera_query.iter_mut().enumerate().last().unwrap();
                camera_transform.rotation = Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    mut query: Query<(&Transform, &mut Velocity), With<PlayerCharacter>>,
    rapier_context: Res<RapierContext>,
) {
    if let Some(window) = windows.get_primary() {
        if window.cursor_grab_mode() == CursorGrabMode::None {
            return;
        }
        let mut target_speed = 2.0;

        for (transform, mut object_velocity) in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);
            for key in keys.get_pressed() {
                match key {
                    KeyCode::W => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::A => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::Space => {
                        if let Some((_entity, _toi)) = rapier_context.cast_ray(
                            transform.translation, Vec3::new(0.0, -1.0, 0.0), 0.25, true, QueryFilter::only_fixed()
                        ) {
                            object_velocity.linvel.y = 5.0;
                        }
                    }
                    KeyCode::LShift => {
                        target_speed += 5.0;
                    }
                    _ => (),
                }
            }
            velocity = velocity.normalize_or_zero();

            let mut current_velocity = object_velocity.linvel;
            current_velocity.y *= 0.25;
            if current_velocity.length_squared() < target_speed {
                object_velocity.linvel += velocity * time.delta_seconds() * 100.0;
            }
        }
    } else {
        warn!("Primary window not found for `player_move`!");
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.25 })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.5, 0.5),
                //emissive: Color::rgb_linear(0.0, 2.5, 5.0),
                ..Default::default()
            }),
            transform: Transform::from_xyz(0.0, 1.0, 0.0),
            ..default()
        })
        .insert(Collider::cuboid(0.25, 0.25, 0.25))
        .insert(RigidBody::Dynamic)
        .insert(PlayerCharacter)
        .insert(Velocity::default())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Damping {
            linear_damping: 5.0,
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Camera3dBundle {
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: 1.22173,
                            ..Default::default()
                        }),
                        transform: Transform::from_xyz(0.0, 0.5, 0.0)
                            .looking_at(Vec3::ZERO, Vec3::Y),
                        camera: Camera {
                            hdr: true,
                            ..Default::default()
                        },
                        ..default()
                    },
                    //BloomSettings::default(),
                ))
                .insert(PlayerCharacterCamera)
                .insert(Fxaa {
                    enabled: true,
                    edge_threshold: Sensitivity::Extreme,
                    ..Default::default()
                });
        });
}
