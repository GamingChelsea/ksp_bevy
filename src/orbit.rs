use crate::time::UniversalTime;
use bevy::prelude::*;
use big_space::prelude::*;

#[derive(Component)]
pub struct OrbitConfig {
    pub radius: f64,
    pub completion_speed: f64,
    pub start_phase: f64,
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
    for (mut transform, mut cell, orbit_config) in query.iter_mut() {
        let phase =
            (ut.0 * orbit_config.completion_speed) + orbit_config.start_phase;
        let x_abs = orbit_config.radius * phase.cos();
        let z_abs = orbit_config.radius * phase.sin();

        let (new_cell, new_translation) =
            grid.translation_to_grid([x_abs, 0.0, z_abs]);
        *cell = new_cell;
        transform.translation = new_translation;
    }
}
