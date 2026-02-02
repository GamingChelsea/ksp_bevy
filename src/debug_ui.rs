use crate::time::WarpFactor;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use strum::IntoEnumIterator;

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, debug_ui)
            // World Inspector (Debugging)
            .add_plugins(WorldInspectorPlugin::new());
    }
}

fn debug_ui(
    mut contexts: EguiContexts,
    mut warp_factor: ResMut<WarpFactor>,
) -> Result {
    egui::Window::new("Time Warp").show(contexts.ctx_mut()?, |ui| {
        egui::ComboBox::from_label("Warp Factor")
            .selected_text(format!("{:?}", *warp_factor))
            .show_ui(ui, |ui| {
                for factor in WarpFactor::iter() {
                    ui.selectable_value(
                        &mut *warp_factor,
                        factor,
                        format!("{:?}", factor),
                    );
                }
            });
    });
    Ok(())
}
