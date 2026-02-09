use core::f64;

use crate::{orbit::OrbitConfig, world::CentralBody};
use bevy::prelude::*;
use big_space::prelude::*;

#[derive(Bundle)]
pub struct PlanetBundle {
    pub orbit: OrbitConfig,
    pub soi: PlanetSOI,
    pub cell: CellCoord,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

pub struct PlanetSpec {
    pub name: &'static str,
    pub radius: f32,
    pub color: Color,
    pub orbit: OrbitConfig,
    pub mass: f64,
    pub soi_radius: f64,
    pub central_b: bool,
}

#[derive(Component)]
pub struct PlanetSOI {
    pub soi_radius: f64,
    pub mass: f64,
}

#[derive(Component)]
pub struct Planet;

pub fn setup_planets(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut Assets<StandardMaterial>,
    parent: &mut GridCommands,
) {
    let planet_specs = vec![
        PlanetSpec {
            name: "Sonne",
            radius: 20.0,
            color: Color::srgb(1.0, 0.9, 0.2),
            orbit: OrbitConfig {
                radius: 0.0,
                completion_speed: 0.0,
                start_phase: 0.0,
            },
            mass: 50_000.0,
            soi_radius: f64::MAX,
            central_b: true,
        },
        PlanetSpec {
            name: "Not Kerbin",
            radius: 5.0,
            color: Color::srgb(0.2, 0.6, 1.0),
            orbit: OrbitConfig {
                radius: 250.0,
                completion_speed: 0.056,
                start_phase: 0.0,
            },
            mass: 10_000.0,
            soi_radius: 60.0,
            central_b: false,
        },
    ];

    for spec in planet_specs {
        let mut entity = parent.spawn((
            PlanetBundle {
                orbit: spec.orbit,
                soi: PlanetSOI { soi_radius: spec.soi_radius, mass: spec.mass },
                cell: CellCoord::default(),
                mesh: Mesh3d(
                    meshes.add(Sphere::new(spec.radius).mesh().ico(5).unwrap()),
                ),
                material: MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: spec.color,
                    ..default()
                })),
            },
            Name::new(spec.name),
            Planet,
        ));

        if spec.central_b {
            entity.insert(CentralBody);
        }
    }
}
