/// Code that runs the system exported by [framework::Story::run()]
use crate::{
    state::{AppState, StoryState},
    StatePlugin,
};
use bevy::prelude::*;
use bevy_rpg::ActiveState;
use iyes_loopless::prelude::*;

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

        world.insert_resource(NextState(ActiveState::Active));
    }
}

impl Plugin for DialogRunPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Self::STATE, Self::on_enter);
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
