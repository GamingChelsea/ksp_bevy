use crate::time::UniversalTime;
use bevy::{math::DVec3, prelude::*};
use big_space::prelude::*;

#[derive(Component, Clone)]
pub struct OrbitConfig {
    pub radius: f64,
    pub completion_speed: f64,
    pub start_phase: f64,
}

impl OrbitConfig {
    pub fn get_velocity_at_time(&self, ut: f64) -> DVec3 {
        let phase = (ut * self.completion_speed) + self.start_phase;
        let omega = self.completion_speed;

        let vx = -self.radius * omega * phase.sin();
        let vz = self.radius * omega * phase.cos();

        DVec3::new(vx, 0.0, vz)
    }
}

pub struct OrbitPlugin;

impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_orbit);
    }
}

fn update_orbit(
    ut: Res<UniversalTime>,
    grid: Single<&Grid>,
    mut query: Query<(&mut Transform, &mut CellCoord, &OrbitConfig)>,
) {
    // Performance optimization: skip processing if time hasn't advanced.
    if !ut.is_changed() {
        return;
    }

    for (mut transform, mut cell, orbit_config) in query.iter_mut() {
        let phase =
            (ut.0 * orbit_config.completion_speed) + orbit_config.start_phase;
        // Optimization: Compute sin and cos in a single operation.
        let (sin, cos) = phase.sin_cos();
        let x_abs = orbit_config.radius * cos;
        let z_abs = orbit_config.radius * sin;

        let (new_cell, new_translation) =
            grid.translation_to_grid([x_abs, 0.0, z_abs]);
        *cell = new_cell;
        transform.translation = new_translation;
    }
}
