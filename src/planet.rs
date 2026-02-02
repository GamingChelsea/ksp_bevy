use crate::orbit::OrbitConfig;
use bevy::prelude::*;
use big_space::prelude::*;

// 1. Das Bundle - Hier haben wir das radius-Feld gelöscht
#[derive(Bundle)]
pub struct PlanetBundle {
    pub orbit: OrbitConfig,
    pub cell: CellCoord,
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

// 2. Der Bauplan bleibt gleich, da wir den Radius für das Mesh brauchen
pub struct PlanetSpec {
    pub radius: f32,
    pub color: Color,
    pub orbit: OrbitConfig,
}

pub fn setup_planets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    parent: &mut GridCommands,
) {
    // Liste der Planeten
    let planet_specs = vec![PlanetSpec {
        radius: 1.0,
        color: Color::srgb(0.2, 0.5, 1.0),
        orbit: OrbitConfig {
            radius: 10.0,
            completion_speed: 1.0,
            start_phase: 0.0,
        },
    }];

    for spec in planet_specs {
        // Hier wird jetzt nur noch das übergeben, was im Bundle definiert ist
        parent.spawn(PlanetBundle {
            orbit: spec.orbit,
            cell: CellCoord::default(),
            mesh: Mesh3d(
                meshes.add(Sphere::new(spec.radius).mesh().ico(5).unwrap()),
            ),
            material: MeshMaterial3d(
                materials.add(StandardMaterial {
                    base_color: spec.color,
                    ..default()
                }),
            ),
        });
    }
}
