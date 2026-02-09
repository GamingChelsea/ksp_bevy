use crate::{
    orbit::OrbitConfig,
    planet::{Planet, PlanetSOI},
    time::UniversalTime,
    vessel::{KinematicState, Vessel},
    world::CentralBody,
};
use bevy::{math::DVec3, prelude::*};
use big_space::{grid::Grid, prelude::CellCoord};
use strum_macros::Display;

#[derive(Component, Debug, Clone, Copy, PartialEq, Display)]
pub enum ReferenceFrame {
    SolarSystemBarycentric,
    PlanetCentered(Entity),
}

pub fn check_soi_transitions(
    grid: Single<&Grid>,
    mut vessel_query: Query<
        (&CellCoord, &Transform, &mut ReferenceFrame, &mut KinematicState),
        With<Vessel>,
    >,
    ut: Res<UniversalTime>,
    planets_query: Query<
        (Entity, &CellCoord, &Transform, &PlanetSOI, &OrbitConfig),
        (With<Planet>, Without<CentralBody>),
    >,
) {
    let active_planets: Vec<(Entity, DVec3, f64)> = planets_query
        .iter()
        .map(|(ent, cell, trans, soi, _)| {
            let pos = grid.grid_position_double(cell, trans);
            let soi_sq = (soi.soi_radius as f64).powi(2);
            (ent, pos, soi_sq)
        })
        .collect();

    vessel_query.par_iter_mut().for_each(
        |(ves_cell, ves_trans, mut ves_frame, mut ves_kin)| {
            let ves_pos = grid.grid_position_double(ves_cell, ves_trans);

            let candidates: Vec<&(Entity, DVec3, f64)> = active_planets
                .iter()
                .filter(|(_, planet_pos, soi_sq)| {
                    ves_pos.distance_squared(*planet_pos) < *soi_sq
                })
                .collect();

            let matching_planet = candidates.iter().min_by(|a, b| {
                a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal)
            });

            match matching_planet {
                Some((planet_ent, _, _)) => {
                    let new_frame = ReferenceFrame::PlanetCentered(*planet_ent);
                    if *ves_frame != new_frame {
                        if let Ok((_, _, _, _, orbit)) = planets_query.get(*planet_ent) {
                            let planet_vel = orbit.get_velocity_at_time(ut.0);
                            ves_kin.velocity -= planet_vel;
                            info!("Entered SOI: Velocity adjusted (relative to planet)");
                        }
                        *ves_frame = new_frame;
                    }
                }
                None => {
                    let barycentric = ReferenceFrame::SolarSystemBarycentric;
                    if *ves_frame != barycentric {
                        if let ReferenceFrame::PlanetCentered(old_planet_ent) = *ves_frame {
                            if let Ok((_, _, _, _, orbit)) = planets_query.get(old_planet_ent) {
                                let planet_vel = orbit.get_velocity_at_time(ut.0);
                                ves_kin.velocity += planet_vel;
                                info!("Left SOI: Velocity adjusted (back to sun-relative)");
                            }
                        }
                        *ves_frame = barycentric;
                    }
                }
            }
        },
    );
}
