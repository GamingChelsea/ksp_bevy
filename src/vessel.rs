use bevy::{math::DVec3, prelude::*};
use big_space::prelude::{CellCoord, GridCommands};

use crate::time::UniversalTime;

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
    parent
        .spawn(VesselBundle {
            vessel: Vessel,
            kinematic_state: KinematicState {
                velocity: DVec3::new(-1.0, 0.0, 0.0), //Speed
            },
            transform: Transform::from_xyz(10.0, 0.0, 0.0), //Position
            global_transform: GlobalTransform::default(),
            cell: CellCoord::default(),
        })
        .insert((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(
                materials.add(Color::from(bevy::color::palettes::basic::RED)),
            ),
        ))
        .insert(Name::new("Test Vessel"));
}

fn move_vessel(
    ut: Res<UniversalTime>,
    mut query: Query<(&mut Transform, &KinematicState), With<Vessel>>,
) {
    for (mut transform, kinematic_state) in query.iter_mut() {
        let new_translation: DVec3 = kinematic_state.velocity * ut.0;

        transform.translation = new_translation.as_vec3();
    }
}
