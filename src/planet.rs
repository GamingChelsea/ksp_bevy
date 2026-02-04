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

#[allow(dead_code)]
#[derive(Component)]
pub struct PlanetSOI {
    pub soi_radius: f64,
    pub mass: f64,
}

pub fn setup_planets(
    meshes: &mut ResMut<Assets<Mesh>>, // Assets brauchen ResMut
    materials: &mut Assets<StandardMaterial>,
    parent: &mut GridCommands,
) {
    // Liste der Planeten
    let planet_specs = vec![
        PlanetSpec {
            name: "Sonne",
            radius: 3.0,
            color: Color::srgb(1.0, 0.0, 0.5),
            orbit: OrbitConfig {
                radius: 0.0,
                completion_speed: 0.0,
                start_phase: 0.0,
            },
            mass: 5.972e24,
            soi_radius: 20.0,
            central_b: true,
        },
        PlanetSpec {
            name: "Not Kerbin",
            radius: 1.0,
            color: Color::srgb(0.2, 0.5, 1.0),
            orbit: OrbitConfig {
                radius: 50.0,
                completion_speed: 0.5,
                start_phase: 0.0,
            },
            mass: 5.972e24,
            soi_radius: 20.0,
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
        ));

        if spec.central_b {
            entity.insert(CentralBody);
        }
    }
}
