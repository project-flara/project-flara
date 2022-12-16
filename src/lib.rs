mod ui;
use std::collections::HashMap;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::{widgets::InspectorQuery, WorldInspectorPlugin};

use ui::{
    startup::{self, StartupTimer},
    title,
};
pub const LAUNCHER_TITLE: &str = "Project Flara";

pub fn app(fullscreen: bool) -> App {
    let mode = if fullscreen {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            mode,
            ..Default::default()
        },
        ..default()
    }))
    .add_startup_system(startup::startup_system)
    .add_system(title::main_screen)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<StartupTimer>()
    .register_type::<Interaction>();
    app
}
