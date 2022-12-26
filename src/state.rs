use crate::story::story_loading::{dylib::StoryDylib, EventStory, MainStory};
use bevy::prelude::*;

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
    Running,
    ChapterMenu,
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum Story {
    MainStory(MainStory),
    EventStory(EventStory),
}
impl std::fmt::Display for Story {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Story::MainStory(story) => story.fmt(f),
            Story::EventStory(story) => story.fmt(f),
        }
    }
}
impl StoryDylib for Story {
    fn module_name(&self) -> String
where {
        match self {
            Self::MainStory(story) => story.to_string(),
            Self::EventStory(story) => story.to_string(),
        }
    }
}
