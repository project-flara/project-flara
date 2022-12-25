use std::collections::HashMap;

use bevy::prelude::*;
use dlopen::wrapper::Container;

use crate::{AppState, StoryState};

use super::story_loading::{load, MainStory, MainStoryPluginAPI};

pub struct MainStoryMenu;

impl Plugin for MainStoryMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stories>();
        app.add_system_set(
            SystemSet::on_enter(AppState::Story(StoryState::MainStory))
                .with_system(Self::on_enter),
        );

        app.add_system_set(
            SystemSet::on_exit(AppState::Story(StoryState::MainStory))
                .with_system(Self::on_update),
        );

        app.add_system_set(
            SystemSet::on_exit(AppState::Story(StoryState::MainStory))
                .with_system(Self::on_exit),
        );
    }
}

#[derive(Resource, Default)]
pub struct Stories {
    pub stories: HashMap<MainStory, Container<MainStoryPluginAPI>>,
    pub errors: HashMap<MainStory, dlopen::Error>,
    pub loaded: bool,
}
impl Stories {
    pub fn load(&mut self) {
        for variation in MainStory::VARIATIONS {
            match unsafe { load(&variation) } {
                Ok(story) => {
                    self.stories.insert(variation, story);
                }
                Err(error) => {
                    self.errors.insert(variation, error);
                }
            }
        }

        self.loaded = true;
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use dlopen::wrapper::Container;

    use crate::story::story_loading::MainStory;

    use super::Stories;

    #[test]
    fn test_resource() {
        let mut stories = Stories::default();

        stories.load();
        let storiess = stories
            .errors
            .iter()
            .collect::<Vec<(&MainStory, &dlopen::Error)>>();
        if storiess.len() != 0 {
            panic!("{storiess:#?}")
        }
        let first = stories.stories.values().next().unwrap().chapter();
        assert_eq!(first.name(), String::from("Sakura Blossoms Everyday!"));
        assert_eq!(first.author(), String::from("Fiana Fortressia"));
        assert_eq!(first.license(), String::from("CC-BY-SA-4.0"));

        assert_eq!(
            *stories.stories.keys().next().unwrap(),
            MainStory::Foundation
        );
    }
}
impl MainStoryMenu {
    pub fn on_enter(
        mut commands: Commands,
        server: Res<AssetServer>,
        mut stories: ResMut<Stories>,
    ) {
        let font = server.load("NotoSans-Regular.ttf");
        commands
            .spawn(NodeBundle { ..default() })
            .with_children(|parent| {
                if !stories.loaded {
                    stories.load();
                }

                for (id, story) in &stories.stories {
                    println!("{id:?}");
                    parent
                        .spawn((ButtonBundle { ..default() }, id.clone()))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    story.chapter().name(),
                                    TextStyle {
                                        font: font.clone(),
                                        ..default()
                                    },
                                ),
                                ..default()
                            });

                            parent.spawn(ButtonBundle { ..default() });
                        });
                }
            });
    }
    pub fn on_update(
        query: Query<(&Interaction, &MainStory)>,
        _commands: Commands,
        mut state: ResMut<State<AppState>>,
    ) {
        for (interaction, name) in query.iter() {
            if *interaction == Interaction::Clicked {
                state
                    .set(AppState::Story(StoryState::Running(
                        crate::Story::MainStory(name.clone()),
                    )))
                    .unwrap();
            };
        }
    }
    pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
        commands
            .entity(
                query
                    .iter()
                    .find(|(_, name)| name.as_str() == "title-screen")
                    .unwrap()
                    .0,
            )
            .despawn_recursive();
    }
}
