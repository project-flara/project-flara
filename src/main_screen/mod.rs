use bevy::prelude::*;
pub struct MainScreenPlugin;

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {}
}

impl MainScreenPlugin {
    pub fn on_update() {}

    pub fn on_enter(mut commands: Commands) {}
}
