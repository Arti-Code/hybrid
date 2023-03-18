use crate::prelude::*;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_panel_system);
    }
}


fn main_panel_system(
    mut contexts: EguiContexts
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        egui::menu::bar(ui, |ui| {
            egui::menu::menu_button(ui, "Simulation", |ui| {
                if ui.button("New Simulation").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Load Simulation").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Save Simulation").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                }
            });
            egui::menu::menu_button(ui, "Tools", |ui| {
                if ui.button("Hybrid Library").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Inspector").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Hybrydizer").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Creator").clicked() {
                    std::process::exit(0);
                }
            });
            egui::menu::menu_button(ui, "View", |ui| {
                if ui.button("Real-Time Statistics").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Zoom In").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Zoom Out").clicked() {
                    std::process::exit(0);
                }
                if ui.button("MiniMap").clicked() {
                    std::process::exit(0);
                }
            });
            egui::menu::menu_button(ui, "About", |ui| {
                if ui.button("Documentation").clicked() {
                    std::process::exit(0);
                }
                if ui.button("Changes Log").clicked() {
                    std::process::exit(0);
                }
                if ui.button("About").clicked() {
                    std::process::exit(0);
                }
            });
        });
    });
}