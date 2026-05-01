use crate::vessel::Vessel;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::prelude::FloatExt;
use bevy::prelude::*;
use big_space::prelude::CellCoord;
use big_space::prelude::FloatingOrigin;

#[derive(Component)]
pub struct DebugFlyCam {
    speed: f32,
    sensitivity: f32,
    radius: f32,
    target_radius: f32,
    pub freemode: bool,
    pub orbit_yaw: f32,
    pub orbit_pitch: f32,
}

impl Default for DebugFlyCam {
    fn default() -> Self {
        Self {
            speed: 20.0,
            sensitivity: 0.002,
            radius: 10.0,
            target_radius: 10.0,
            freemode: false,
            orbit_pitch: -0.4,
            orbit_yaw: 0.0,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, camera_controller);
    }
}

fn camera_controller(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mut cam_query: Query<
        (&mut Transform, &mut CellCoord, &mut DebugFlyCam),
        (With<Camera>, With<FloatingOrigin>, Without<Vessel>),
    >,
    vessel_query: Query<(&Transform, &CellCoord), With<Vessel>>,
) {
    let Ok((mut c_transform, mut c_cell, mut config)) = cam_query.single_mut()
    else {
        return;
    };

    if keyboard.just_pressed(KeyCode::KeyV) {
        config.freemode = !config.freemode;
    }

    match config.freemode {
        true => {
            if keyboard.pressed(KeyCode::AltLeft) {
                let delta = mouse_motion.delta;
                config.orbit_yaw -= delta.x * config.sensitivity;
                config.orbit_pitch -= delta.y * config.sensitivity;
                config.orbit_pitch = config.orbit_pitch.clamp(-1.54, 1.54);

                c_transform.rotation = Quat::from_rotation_y(config.orbit_yaw)
                    * Quat::from_rotation_x(config.orbit_pitch);
            }

            let mut velocity = Vec3::ZERO;
            let forward = c_transform.forward();
            let right = c_transform.right();
            let up = c_transform.up();

            if keyboard.pressed(KeyCode::KeyW) {
                velocity += *forward;
            }
            if keyboard.pressed(KeyCode::KeyS) {
                velocity -= *forward;
            }
            if keyboard.pressed(KeyCode::KeyA) {
                velocity -= *right;
            }
            if keyboard.pressed(KeyCode::KeyD) {
                velocity += *right;
            }
            if keyboard.pressed(KeyCode::Space) {
                velocity += *up;
            }
            if keyboard.pressed(KeyCode::ControlLeft) {
                velocity -= *up;
            }

            if velocity.length_squared() > 0.0 {
                let speed_multiplier = if keyboard.pressed(KeyCode::ShiftLeft) {
                    50.0
                } else {
                    1.0
                };
                let move_detla = velocity.normalize()
                    * config.speed
                    * speed_multiplier
                    * time.delta_secs();
                c_transform.translation += move_detla;
            }
        }
        false => {
            let Ok((v_trans, v_cell)) = vessel_query.single() else {
                return;
            };
            if *c_cell != *v_cell {
                *c_cell = *v_cell;
            }

            if mouse_wheel.delta != Vec2::ZERO {
                config.target_radius = (config.target_radius
                    - mouse_wheel.delta.y * 2.0)
                    .clamp(2.0, 100.0);
            }

            if (config.target_radius - config.radius).abs() > 0.001 {
                let factor = 1.0 - (-10.0 * time.delta_secs()).exp();
                config.radius =
                    config.radius.lerp(config.target_radius, factor);
            }

            if keyboard.pressed(KeyCode::AltLeft) {
                let delta = mouse_motion.delta;
                config.orbit_yaw -= delta.x * config.sensitivity;
                config.orbit_pitch -= delta.y * config.sensitivity;
                config.orbit_pitch = config.orbit_pitch.clamp(-1.5, 1.5);
            }

            let rotation = Quat::from_rotation_y(config.orbit_yaw)
                * Quat::from_rotation_x(config.orbit_pitch);
            let offset = rotation.mul_vec3(Vec3::Z * config.radius);

            c_transform.translation = v_trans.translation + offset;
            c_transform.look_at(v_trans.translation, Vec3::Y);
        }
    }
}
