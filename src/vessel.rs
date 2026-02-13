use crate::{
    orbit::OrbitConfig,
    planet::{Planet, PlanetSOI},
    space::ReferenceFrame,
    time::{GameTime, UniversalTime},
    world::GravityConstant,
};
use bevy::{math::DVec3, prelude::*};
use big_space::{
    grid::Grid,
    prelude::{CellCoord, GridCommands},
};

#[derive(Component)]
pub struct Vessel;

#[derive(Component)]
pub struct KinematicState {
    pub velocity: DVec3,
}

#[derive(Bundle)]
pub struct VesselBundle {
    pub vessel: Vessel,
    pub kinematic_state: KinematicState,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub cell: CellCoord,
    pub ref_frame: ReferenceFrame,
}

pub struct VesselPlugin;

impl Plugin for VesselPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, move_vessel);
    }
}

pub fn spawn_test_vessel(
    parent: &mut GridCommands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let start = DVec3::new(-50.0, 0.0, 0.0); // Position

    parent
        .spawn(VesselBundle {
            vessel: Vessel,
            kinematic_state: KinematicState {
                velocity: DVec3::new(0.0, 0.0, 43.0), //Speed
            },
            transform: Transform::from_translation(start.as_vec3()),
            global_transform: GlobalTransform::default(),
            cell: CellCoord::default(),
            ref_frame: ReferenceFrame::SolarSystemBarycentric,
        })
        .insert((
            Mesh3d(meshes.add(Cuboid::new(5.0, 1.0, 5.0))),
            MeshMaterial3d(
                materials.add(Color::from(bevy::color::palettes::basic::RED)),
            ),
        ))
        .insert(Name::new("Test Vessel"));
}

pub fn move_vessel(
    grid: Single<&Grid>,
    planet_query: Query<
        (Entity, &Transform, &CellCoord, &PlanetSOI, Option<&OrbitConfig>),
        (With<Planet>, Without<Vessel>),
    >,
    gt: Res<GameTime>,
    ut: Res<UniversalTime>,
    g: Res<GravityConstant>,
    mut vessel_query: Query<
        (&mut Transform, &mut CellCoord, &mut KinematicState, &ReferenceFrame),
        (With<Vessel>, Without<Planet>),
    >,
) {
    let dt = gt.0;
    if dt <= 0.0 {
        return;
    }

    for (mut transform, mut cell, mut kinematic_state, ref_frame) in
        vessel_query.iter_mut()
    {
        let current_pos = grid.grid_position_double(&cell, &transform);

        let mut acceleration = DVec3::ZERO;
        let mut frame_velocity_offset = DVec3::ZERO;

        let boss_data = match ref_frame {
            ReferenceFrame::PlanetCentered(entity) => {
                planet_query.get(*entity).ok()
            }
            ReferenceFrame::SolarSystemBarycentric => planet_query
                .iter()
                .max_by(|a, b| a.3.mass.partial_cmp(&b.3.mass).unwrap()),
        };

        if let Some((_, p_trans, p_cell, p_soi, p_orbit)) = boss_data {
            let planet_pos = grid.grid_position_double(p_cell, p_trans);
            let delta = planet_pos - current_pos;
            let dist_sq = delta.length_squared();

            if dist_sq > 1.0 {
                let force_mag = (g.0 as f64 * p_soi.mass) / dist_sq;
                acceleration = (delta / dist_sq.sqrt()) * force_mag;
            }

            if matches!(ref_frame, ReferenceFrame::PlanetCentered(_)) {
                if let Some(orbit) = p_orbit {
                    let phase =
                        (ut.0 * orbit.completion_speed) + orbit.start_phase;
                    let (sin, cos) = phase.sin_cos();
                    let vx = -orbit.radius * orbit.completion_speed * sin;
                    let vz = orbit.radius * orbit.completion_speed * cos;
                    frame_velocity_offset = DVec3::new(vx, 0.0, vz);
                }
            }
        }

        kinematic_state.velocity += acceleration * dt;

        let total_velocity = kinematic_state.velocity + frame_velocity_offset;
        let new_pos = current_pos + (total_velocity * dt);

        let (new_cell, new_translation) = grid.translation_to_grid(new_pos);
        *cell = new_cell;
        transform.translation = new_translation;
    }
}
