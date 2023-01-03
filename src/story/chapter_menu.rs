use bevy::prelude::*;
use iyes_loopless::prelude::*;

use framework::Chapter;

use crate::state::AppState;
use crate::StatePlugin;

use crate::state::StoryState;

use super::dialog_run::{CurrentDialog, DialogRunPlugin};

pub struct ChapterMenuPlugin;

impl Plugin for ChapterMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(Self::STATE, Self::on_enter);

        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Self::STATE)
                .with_system(Self::on_update)
                .into(),
        );
        app.add_exit_system(Self::STATE, Self::on_exit);
    }
}

impl ChapterMenuPlugin {
    pub fn on_enter(
        app_state: Res<CurrentState<AppState>>,
        server: Res<AssetServer>,
        current: Res<CurrentChapterState>,
        mut commands: Commands,
    ) {
        if AppState::Story(StoryState::ChapterMenu) == app_state.0 {
            let font = server.load("NotoSans-Regular.ttf");
            let about = server.load("app/icons/info-symbolic.png");
            commands
                .spawn((
                    NodeBundle {
                        style: Style { ..default() },
                        ..default()
                    },
                    Name::new("chapter-menu"),
                ))
                .with_children(|parent| {
                    for story in current.chapter.stories() {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        padding: UiRect::all(Val::Px(5.0)),

                                        ..default()
                                    },
                                    ..default()
                                },
                                Name::new(story.id() + ":::flarastory"),
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle {
                                    text: Text::from_section(
                                        story.name(),
                                        TextStyle {
                                            font_size: 32.0,
                                            font: font.clone(),
                                            ..default()
                                        },
                                    ),
                                    ..default()
                                });

                                parent
                                    .spawn((ButtonBundle { ..default() }))
                                    .with_children(|parent| {
                                        parent.spawn(ImageBundle {
                                            image: UiImage(about.clone()),
                                            ..default()
                                        });
                                    });
                            });
                    }
                });
        } else {
            unreachable!();
        }
    }

    pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
        let entity = query
            .iter()
            .find(|(_, name)| name.as_str() == "chapter-button")
            .unwrap()
            .0;
        commands.entity(entity).despawn_recursive();
    }

    pub fn on_update(
        query: Query<(&Interaction, &Name)>,
        mut commands: Commands,
    ) {
        if let Some((_, name)) = query.iter().find(|(interaction, name)| {
            **interaction == Interaction::Clicked
                && name.as_str().ends_with(":::flarastory")
        }) {
            commands.insert_resource(CurrentDialog {
                id: name
                    .as_str()
                    .strip_suffix(":::flarastory")
                    .unwrap()
                    .to_string(),
            });

            commands.insert_resource(NextState(DialogRunPlugin::STATE));
        }
    }
}

impl StatePlugin for ChapterMenuPlugin {
    const STATE: crate::state::AppState =
        AppState::Story(StoryState::ChapterMenu);
}

#[derive(Resource)]
pub struct CurrentChapterState {
    pub chapter: Box<dyn Chapter>,
}
