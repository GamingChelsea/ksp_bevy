use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use big_space::prelude::FloatingOrigin;

#[derive(Component)]
pub struct DebugFlyCam {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for DebugFlyCam {
    fn default() -> Self {
        Self { speed: 20.0, sensitivity: 0.15 }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_controller);
    }
}

fn camera_controller(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut Transform, &DebugFlyCam), With<FloatingOrigin>>,
) {
    let Ok((mut transform, config)) = query.single_mut() else { return };

    let activate_camera_control = keyboard.pressed(KeyCode::AltLeft);
    // --- ROTATION (Maus) ---
    if activate_camera_control {
        let mut rotation_move = Vec2::ZERO;
        rotation_move += mouse_motion.delta;

        if rotation_move.length_squared() > 0.0 {
            let delta_x =
                rotation_move.x * config.sensitivity * time.delta_secs();
            let delta_y =
                rotation_move.y * config.sensitivity * time.delta_secs();

            // Yaw (Links/Rechts) - Rotation um die globale Y-Achse
            let yaw = Quat::from_rotation_y(-delta_x);
            // Pitch (Hoch/Runter) - Rotation um die lokale X-Achse
            let pitch = Quat::from_rotation_x(-delta_y);

            transform.rotation = yaw * transform.rotation * pitch;
        }
    }

    // --- BEWEGUNG (WASD) ---
    let mut velocity = Vec3::ZERO;
    let local_forward = transform.forward();
    let local_right = transform.right();
    let local_up = transform.up();

    if keyboard.pressed(KeyCode::KeyW) {
        velocity += *local_forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        velocity -= *local_forward;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        velocity -= *local_right;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        velocity += *local_right;
    }
    if keyboard.pressed(KeyCode::Space) {
        velocity += *local_up;
    }
    if keyboard.pressed(KeyCode::ControlLeft) {
        velocity -= *local_up;
    }

    // "Boost" mit Shift (Space ist groß!)
    let speed_multiplier =
        if keyboard.pressed(KeyCode::ShiftLeft) { 50.0 } else { 1.0 };

    if velocity.length_squared() > 0.0 {
        transform.translation += velocity.normalize()
            * config.speed
            * speed_multiplier
            * time.delta_secs();
    }
}
