#![doc = include_str!("./README.md")]
pub mod invisible_to_focus;
pub mod main_screen;
pub mod startup;
use bevy_rapier2d::prelude::*;
use cursor::{CustomCursor, MainCamera};
pub use framework::states as state;
pub mod cursor;
pub mod story;
use bevy::{prelude::*, window::WindowMode};
use bevy_inspector_egui::WorldInspectorPlugin;

use invisible_to_focus::InvisibleToFocusPlugin;
use main_screen::MainScreenPlugin;
use startup::{startup_screen::StartupPlugin, title::TitlePlugin};
use story::{
    chapter_menu::ChapterMenuPlugin, main_story::MainStoryMenu,
    menu::StoryMenuPlugin,
};
pub const LAUNCHER_TITLE: &str = "Project Flara";

pub trait StatePlugin {
    const STATE: crate::state::AppState;
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
    .add_startup_system(camera)
    // add the app state type
    .add_state(state::AppState::StartupScreen)
    .add_plugin(InvisibleToFocusPlugin)
    .add_plugin(CustomCursor)
    .add_plugin(StartupPlugin)
    .add_plugin(TitlePlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
    .add_plugin(MainScreenPlugin)
    .add_plugin(StoryMenuPlugin)
    .add_plugin(MainStoryMenu)
    .add_plugin(ChapterMenuPlugin)
    .add_plugin(WorldInspectorPlugin::new())
    .register_type::<Interaction>();
    app
}

fn camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}
