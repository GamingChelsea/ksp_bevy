use crate::camera::DebugFlyCam;
use crate::orbit::OrbitConfig;
use bevy::prelude::*;
use big_space::prelude::*;

#[derive(Component)]
pub struct CentralBody;

pub fn setup_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_big_space(Grid::default(), |parent| {
        // Kamera
        parent.spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 50.0, 100.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            FloatingOrigin,
            CellCoord::default(),
            DebugFlyCam::default(),
        ));

        // Sonne
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(5.0).mesh().ico(5).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::YELLOW),
                emissive: LinearRgba::rgb(1.0, 1.0, 0.5),
                ..default()
            })),
            CellCoord::default(),
            CentralBody,
        ));

        // Ein Planet (Der sich bewegen wird)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(2.0).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::BLUE),
                ..default()
            })),
            CellCoord::default(),
            OrbitConfig {
                radius: 30.0,
                completion_speed: 0.5,
                start_phase: 0.0,
            },
        ));

        // Ein weiterer Planet (Der sich bewegen wird)
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(2.0).mesh().ico(4).unwrap())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::from(bevy::color::palettes::basic::GREEN),
                ..default()
            })),
            CellCoord::default(),
            OrbitConfig {
                radius: 30.0,
                completion_speed: 0.5,
                start_phase: 1.0,
            },
        ));

        // Licht
        parent.spawn((
            DirectionalLight::default(),
            Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            CellCoord::default(),
        ));
    });
}
