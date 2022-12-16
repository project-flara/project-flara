mod startup;
use std::collections::HashMap;

use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::WorldInspectorPlugin;

use startup::{startup::StartupPlugin, startup::StartupTimer, title::{self, TitlePlugin}};
pub const LAUNCHER_TITLE: &str = "Project Flara";
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartupScreen,
    Dialog,
    MainScreen,
    TitleScreen,
}

pub trait StatePlugin {
    const STATE: AppState;
}
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
    // add the app state type
    .add_state(AppState::StartupScreen)
    .add_plugin(StartupPlugin)
    .add_plugin(TitlePlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Interaction>();
    app
}
