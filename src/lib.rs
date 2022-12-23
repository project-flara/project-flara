pub mod main_screen;
mod startup;
pub mod story;
use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::WorldInspectorPlugin;

use main_screen::MainScreenPlugin;
use startup::{
    startup_screen::StartupPlugin,
    title::{TitlePlugin},
};
use story::menu::StoryMenuPlugin;
pub const LAUNCHER_TITLE: &str = "Project Flara";
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartupScreen,
    Dialog,
    MainScreen,
    TitleScreen,
    Story(StoryState),
    Event,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum StoryState {
    Menu,
    MainStory,
    Events,
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
    .add_plugin(MainScreenPlugin)
    .add_plugin(StoryMenuPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Interaction>();
    app
}
