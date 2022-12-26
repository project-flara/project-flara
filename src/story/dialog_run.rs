use crate::{
    state::{AppState, StoryState},
    StatePlugin,
};
use bevy::prelude::*;
use bevy_rpg::ActiveState;

use super::chapter_menu::CurrentChapterState;

pub struct DialogRunPlugin;

impl DialogRunPlugin {
    pub fn on_enter(world: &mut World) {
        let chapter: &CurrentChapterState = world.get_resource().unwrap();
        let current: &CurrentDialog = world.get_resource().unwrap();
        let stories = chapter.chapter.stories();
        let chapter = stories
            .iter()
            .find(|story| story.id() == current.id)
            .unwrap();

        chapter.run().run((), world);
        let mut active_state: Mut<State<ActiveState>> =
            world.get_resource_mut().unwrap();

        active_state.set(ActiveState::Active).unwrap();
    }
}

impl Plugin for DialogRunPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Self::STATE).with_system(Self::on_enter),
        );
    }
}

impl StatePlugin for DialogRunPlugin {
    const STATE: crate::state::AppState = AppState::Story(StoryState::Running);
}
#[derive(Resource)]
pub struct CurrentDialog {
    /// This is the dialog in [framework::Story::id()]
    pub id: String,
}
