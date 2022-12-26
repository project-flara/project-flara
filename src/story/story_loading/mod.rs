use bevy::prelude::*;
pub mod dylib;
pub mod stories;

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
