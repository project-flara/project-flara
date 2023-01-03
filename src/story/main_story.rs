use bevy::prelude::*;
use framework::states::MainStory;
use iyes_loopless::prelude::*;

use crate::{
    state::{AppState, StoryState},
    StatePlugin,
};

use super::{
    chapter_menu::CurrentChapterState, story_loading::stories::Stories,
};
pub struct MainStoryMenu;

impl Plugin for MainStoryMenu {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stories>();
        app.add_enter_system(Self::STATE, Self::on_enter);

        app.add_exit_system(Self::STATE, Self::on_exit);
        app.add_exit_system(Self::STATE, Self::on_exit);
    }
}

#[cfg(test)]
mod test {

    use framework::states::MainStory;

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
            .spawn((NodeBundle { ..default() }, MainStoryScreen))
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

        stories: Res<Stories>,
    ) {
        for (interaction, name) in query.iter() {
            if *interaction == Interaction::Clicked {
                commands.insert_resource(NextState(AppState::Story(
                    StoryState::ChapterMenu,
                )));
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
    pub fn on_exit(
        query: Query<Entity, With<MainStoryScreen>>,
        mut commands: Commands,
    ) {
        commands.entity(query.single()).despawn_recursive();
    }
}

impl StatePlugin for MainStoryMenu {
    const STATE: crate::state::AppState =
        AppState::Story(StoryState::MainStory);
}

#[derive(Component)]
pub struct MainStoryScreen;
