mod camera;
mod orbit;
mod time;
mod world;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use big_space::prelude::*;

fn main() {
    App::new()
        // Bevy & UI
        .add_plugins(DefaultPlugins.build().disable::<TransformPlugin>())
        .add_plugins(EguiPlugin::default())
        // Big Space
        .add_plugins(BigSpaceDefaultPlugins)
        // Eigene Feature-Plugins
        .add_plugins(time::TimePlugin)
        .add_plugins(orbit::OrbitPlugin)
        .add_plugins(camera::CameraPlugin)
        // Setup System
        .add_systems(Startup, world::setup_solar_system)
        .run();
}
