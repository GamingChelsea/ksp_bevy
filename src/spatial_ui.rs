use std::f32::consts::TAU;

use crate::{orbit::OrbitConfig, world::CentralBody};
use bevy::prelude::*;

pub struct SpatialUIPlugin;

impl Plugin for SpatialUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_orbit_gizmos);
    }
}

pub fn draw_orbit_gizmos(
    mut gizmos: Gizmos,
    central_body_query: Query<&GlobalTransform, With<CentralBody>>,
    orbiting_bodies_query: Query<&OrbitConfig>,
) {
    // Finde die Transformation des zentralen Körpers (der Sonne).
    let Ok(central_body_transform) = central_body_query.single() else {
        // Wenn es keinen oder mehr als einen zentralen Körper gibt, tue nichts.
        return;
    };

    let (_scale, rotation, translation) =
        central_body_transform.to_scale_rotation_translation();

    // Die Kreise sollen auf der XY-Ebene des zentralen Körpers liegen.
    // Dafür rotieren wir sie lokal um 90 Grad um die X-Achse.
    let circle_local_rotation = Quat::from_rotation_x(TAU / 4.0);
    let final_rotation = rotation * circle_local_rotation;

    // Erzeuge die Basis-Transformation für die Gizmos.
    let gizmo_isometry = Isometry3d {
        translation: translation.into(),
        rotation: final_rotation,
    };

    for orbit_config in orbiting_bodies_query.iter() {
        gizmos
            .circle(
                gizmo_isometry,
                orbit_config.radius as f32,
                Color::from(bevy::color::palettes::basic::WHITE),
            )
            .resolution(128);
    }
}
