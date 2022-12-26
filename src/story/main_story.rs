use bevy::prelude::*;

use crate::{
    state::{AppState, Story, StoryState},
    StatePlugin,
};

use super::{
    chapter_menu::CurrentChapterState,
    story_loading::{stories::Stories, MainStory},
};
pub struct MainStoryMenu;

impl Plugin for MainStoryMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stories>();
        app.add_system_set(
            SystemSet::on_enter(Self::STATE).with_system(Self::on_enter),
        );

        app.add_system_set(
            SystemSet::on_exit(Self::STATE).with_system(Self::on_update),
        );

        app.add_system_set(
            SystemSet::on_exit(Self::STATE).with_system(Self::on_exit),
        );
    }
}

#[cfg(test)]
mod test {

    use crate::story::story_loading::MainStory;

    use super::Stories;

    #[test]
    fn test_resource() {
        let mut stories = Stories::default();

        stories.load_mainline();
        let storiess = stories
            .mainline
            .errors
            .iter()
            .collect::<Vec<(&MainStory, &dlopen::Error)>>();
        if storiess.len() != 0 {
            panic!("{storiess:#?}")
        }
        let first = stories.mainline.stories.values().next().unwrap().chapter();
        assert_eq!(first.name(), String::from("Sakura Blossoms Everyday!"));
        assert_eq!(first.author(), String::from("Fiana Fortressia"));
        assert_eq!(first.license(), String::from("CC-BY-SA-4.0"));

        assert_eq!(
            *stories.mainline.stories.keys().next().unwrap(),
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
                if !stories.mainline.loaded {
                    stories.load_mainline()
                }

                for (id, story) in &stories.mainline.stories {
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
        mut commands: Commands,
        mut state: ResMut<State<AppState>>,
        stories: Res<Stories>,
    ) {
        for (interaction, name) in query.iter() {
            if *interaction == Interaction::Clicked {
                state.set(AppState::Story(StoryState::ChapterMenu)).unwrap();
                commands.insert_resource(CurrentChapterState {
                    chapter: stories
                        .mainline
                        .stories
                        .get(name)
                        .unwrap()
                        .clone()
                        .chapter(),
                });
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

impl StatePlugin for MainStoryMenu {
    const STATE: crate::state::AppState =
        AppState::Story(StoryState::MainStory);
}
