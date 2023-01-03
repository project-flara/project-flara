//! This section is where a menu is
//!

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    state::{AppState, StoryState},
    StatePlugin,
};

pub struct StoryMenuPlugin;

impl Plugin for StoryMenuPlugin {
    fn build(&self, app: &mut App) {
        // systems to run only in the startup screen
        app.add_enter_system(Self::STATE, Self::on_enter);

        app.add_exit_system(Self::STATE, Self::on_exit);
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(Self::STATE)
                .with_system(Self::on_update)
                .into(),
        );
    }
}

impl StoryMenuPlugin {
    pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
        let font = server.load("NotoSans-Regular.ttf");
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("story-menu"),
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(Val::Px(5.0)),

                                ..default()
                            },
                            ..default()
                        },
                        Name::new("main-story-button"),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                "Main Story",
                                TextStyle {
                                    font_size: 36.0,
                                    font: font.clone(),
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    });

                parent.spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    Name::new(""),
                ));
            });
    }

    pub fn on_update(
        query: Query<(&Interaction, &Name)>,

        mut commands: Commands,
    ) {
        let interaction = query
            .iter()
            .find(|(_, name)| name.as_str() == "main-story-button")
            .unwrap()
            .0;

        if *interaction == Interaction::Clicked {
            commands.insert_resource(NextState(AppState::Story(
                StoryState::MainStory,
            )));
        }
    }

    pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
        let entity = query
            .iter()
            .find(|(_, name)| name.as_str() == "story-menu")
            .unwrap()
            .0;
        commands.entity(entity).despawn_recursive();
    }
}

impl StatePlugin for StoryMenuPlugin {
    const STATE: AppState = AppState::Story(StoryState::Menu);
}
