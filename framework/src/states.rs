use bevy::prelude::*;
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StartupScreen,

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

impl std::fmt::Display for MainStory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl std::fmt::Display for EventStory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum MainStory {
    // Flara Foundation
    Foundation,
    // Indicate a custom
    Custom(String),
}

impl Default for MainStory {
    fn default() -> Self {
        Self::Custom(String::new())
    }
}

impl Variation for MainStory {
    fn variations() -> Vec<Self> {
        vec![Self::Foundation]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum EventStory {
    Custom(String),
}

impl Variation for EventStory {
    fn variations() -> Vec<Self> {
        vec![]
    }
}

impl Default for EventStory {
    fn default() -> Self {
        Self::Custom(String::new())
    }
}
pub trait Variation
where
    Self: Sized,
{
    fn variations() -> Vec<Self>;
}
