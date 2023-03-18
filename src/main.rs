#[allow(dead_code, unused)]

mod consts;
mod agent;
mod ui;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_rapier2d::prelude::*;
    pub use bevy_egui::{egui, EguiContexts, EguiPlugin};
    pub use bevy_prototype_lyon::prelude::*;
    pub use rand::{thread_rng, Rng};
    pub use crate::consts::*;
    pub use crate::agent::AgentPlugin;
    pub use crate::ui::UiPlugin;
}

use crate::prelude::*;
use bevy::window::WindowResolution;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "H Y B R I D".to_string(),
                    resolution: WindowResolution::new(WIN_SIZE.x, WIN_SIZE.y),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugin(EguiPlugin)
        //.add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.,0.,0.)))
        .add_plugin(ShapePlugin)
        .insert_resource(RapierConfiguration{
            gravity: Vec2::ZERO,
            //gravity: Vect::Y * -9.81 * 10.0,
            timestep_mode: TimestepMode::Fixed { dt: 1.0/30.0, substeps: 1 },
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_startup_system(setup_graphics_system)
        .add_plugin(UiPlugin)
        .add_plugin(AgentPlugin)
        .run();
}

fn setup_graphics_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}