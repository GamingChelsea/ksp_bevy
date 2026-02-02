mod camera;
mod debug_ui;
mod orbit;
mod planet;
mod spatial_ui;
mod time;
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
        // Setup System
        .add_systems(Startup, world::setup_solar_system)
        .run();
}
