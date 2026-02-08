use crate::planet::PlanetSOI;
use bevy::prelude::*;
use big_space::{grid::Grid, prelude::CellCoord};

#[allow(dead_code)]
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum ReferenceFrame {
    SolarSystemBarycentric,
    PlanetCentered(Entity),
}

pub fn check_soi_transitions(
    grid: Single<&Grid>,
    camera_query: Query<(Entity, &CellCoord, &Transform), With<Camera>>,
    planets_query: Query<(Entity, &CellCoord, &Transform, &PlanetSOI, &Name)>,
) {
    let Ok((_cam_ent, cam_cellcoord, cam_transform)) =
        camera_query.get_single()
    else {
        return;
    };

    let cam_pos = grid.grid_position_double(&cam_cellcoord, cam_transform);

    for (planet_ent, planet_cell, planet_transform, soi, name) in
        planets_query.iter()
    {
        let planet_pos =
            grid.grid_position_double(planet_cell, planet_transform);

        // Optimization: Use squared distance to avoid expensive square root.
        let distance_sq = cam_pos.distance_squared(planet_pos);
        let soi_radius = soi.soi_radius as f64;
        let soi_radius_sq = soi_radius * soi_radius;

        if distance_sq < soi_radius_sq {
            ReferenceFrame::PlanetCentered(planet_ent);
            info!("Kamera ist im Einflussbereich von Entity {:?}", name);
        } else {
            #[allow(path_statements)]
            ReferenceFrame::SolarSystemBarycentric;
        }
    }
}
