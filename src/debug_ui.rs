use crate::time::WarpFactor;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
use strum::IntoEnumIterator;

pub struct DebugUIPlugin;

impl Plugin for DebugUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin::default())
            .add_systems(EguiPrimaryContextPass, ui_example_system);
    }
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut warp_factor: ResMut<WarpFactor>,
) -> Result {
    egui::Window::new("Time Warp").show(contexts.ctx_mut()?, |ui| {
        // Die ComboBox zeigt den aktuell ausgewählten Text an
        egui::ComboBox::from_label("Warp Factor")
            .selected_text(format!("{:?}", *warp_factor))
            .show_ui(ui, |ui| {
                // Wir iterieren über alle Varianten
                for factor in WarpFactor::iter() {
                    // selectable_value setzt warp_factor automatisch, wenn geklickt wird
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
