use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use bevy::prelude::*;
use dlopen::wrapper::Container;
use iyes_loopless::prelude::*;

use super::dylib::StoryDylib;
use super::dylib::{load, MainStoryPluginAPI};

use framework::states::{EventStory, MainStory, Variation};

#[derive(Resource, Default)]
pub struct Stories {
    pub mainline: Storyline<MainStory>,
    pub events: Storyline<EventStory>,
}

#[derive(Default)]
pub struct Storyline<T> {
    pub stories: HashMap<T, Container<MainStoryPluginAPI>>,
    pub errors: HashMap<T, dlopen::Error>,
    pub loaded: bool,
}

impl Stories {
    fn load<T: Variation + Eq + Hash + StoryDylib + Display>(
        storyline: &mut Storyline<T>,
    ) {
        for variation in T::variations() {
            match unsafe { load(&variation) } {
                Ok(story) => {
                    storyline.stories.insert(variation, story);
                }
                Err(error) => {
                    storyline.errors.insert(variation, error);
                }
            }
        }

        storyline.loaded = true;
    }

    pub fn load_mainline(&mut self) {
        Self::load(&mut self.mainline)
    }

    pub fn load_eventsline(&mut self) {
        Self::load(&mut self.events)
    }
}
