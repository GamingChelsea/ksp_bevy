use crate::planet::PlanetSOI;
use crate::world::GridRes;
use bevy::prelude::*;
use big_space::prelude::CellCoord;

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ReferenceFrame {
    SolarSystemBarycentric,
    PlanetCentered(Entity),
}

pub fn check_soi_transitions(
    camera_query: Query<(Entity, &CellCoord, &Transform), With<Camera>>,
    planets_query: Query<(Entity, &CellCoord, &Transform, &PlanetSOI)>,
    grid: Res<GridRes>,
) {
    let Ok((_cam_ent, cam_cellcoord, cam_transform)) = camera_query.single()
    else {
        return;
    };

    // 1. Absolute Position der Kamera in f64 berechnen
    let cam_pos = grid.0.grid_position_double(&cam_cellcoord, cam_transform);

    for (planet_ent, planet_cell, planet_transform, soi) in planets_query.iter()
    {
        // 2. Absolute Position des Planeten berechnen
        let planet_pos =
            grid.0.grid_position_double(planet_cell, planet_transform);

        // 3. Distanz im DVec3 Raum (f64 Präzision)
        // Optimization: Use squared distance to avoid expensive square root.
        let distance_sq = cam_pos.distance_squared(planet_pos);
        let soi_radius = soi.soi_radius as f64;
        let soi_radius_sq = soi_radius * soi_radius;

        if distance_sq < soi_radius_sq {
            info!("Kamera ist im Einflussbereich von Entity {:?}", planet_ent);
        }
    }
}
