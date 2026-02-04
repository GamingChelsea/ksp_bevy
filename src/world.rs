use crate::camera::DebugFlyCam;
use crate::planet::setup_planets;
use crate::vessel::spawn_test_vessel;
use bevy::prelude::*;
use big_space::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_solar_system);
    }
}

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

        setup_planets(&mut meshes, &mut materials, parent);

        spawn_test_vessel(parent, &mut meshes, &mut materials);

        // Licht
        parent.spawn((
            DirectionalLight::default(),
            Transform::from_xyz(1.0, 1.0, 1.0).looking_at(Vec3::ZERO, Vec3::Y),
            CellCoord::default(),
        ));
    });
}
