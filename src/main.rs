mod camera;
mod debug_ui;
mod orbit;
mod planet;
mod space;
mod spatial_ui;
mod time;
mod vessel;
mod world;

use bevy::prelude::*;
use big_space::prelude::*;

fn main() {
    App::new()
        // Bevy & UI
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        // Big Space
        .add_plugins(BigSpaceDefaultPlugins)
        // Eigene Feature-Plugins
        .add_plugins(time::TimePlugin)
        .add_plugins(orbit::OrbitPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(debug_ui::DebugUIPlugin)
        .add_plugins(spatial_ui::SpatialUIPlugin)
        .add_plugins(world::WorldPlugin)
        .add_plugins(vessel::VesselPlugin)
        // Setup System
        .add_systems(Update, space::check_soi_transitions)
        .run();
}
