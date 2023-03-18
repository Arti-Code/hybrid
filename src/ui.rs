use bevy_egui::egui::RichText;
use bevy_egui::egui::Color32;
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
            ui.heading(RichText::new( "HYBRID").color(Color32::GREEN).strong().heading());
            ui.add_space(5.0);
            ui.separator();
            ui.add_space(5.0);
            egui::menu::menu_button(ui, "Simulation", |ui| {
                if ui.button("New Simulation").clicked() {
                }
                if ui.button("Load Simulation").clicked() {
                }
                if ui.button("Save Simulation").clicked() {
                }
                if ui.button(RichText::new("Quit").color(Color32::RED)).clicked() {
                    std::process::exit(0);
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, "Tools", |ui| {
                if ui.button("Hybrid Library").clicked() {
                }
                if ui.button("Inspector").clicked() {
                }
                if ui.button("Hybrydizer").clicked() {
                }
                if ui.button("Creator").clicked() {
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, "View", |ui| {
                if ui.button("Real-Time Statistics").clicked() {
                }
                if ui.button("Zoom In").clicked() {
                }
                if ui.button("Zoom Out").clicked() {
                }
                if ui.button("MiniMap").clicked() {
                }
            });
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);
            egui::menu::menu_button(ui, "About", |ui| {
                if ui.button("Documentation").clicked() {
                }
                if ui.button("Changes Log").clicked() {
                }
                if ui.button("About").clicked() {
                }
            });
        });
    });
}