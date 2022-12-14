//! Title screen where a button will say "PRESS TO CONTINUE"
//!
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{state::AppState, StatePlugin};

impl TitlePlugin {
    pub fn on_update(
        mut query: Query<(&Interaction, &Name)>,

        mut commands: Commands,
    ) {
        let (interaction, _) = query
            .iter_mut()
            .find(|(_, name)| name.as_str() == "play-button")
            .unwrap();

        if *interaction == Interaction::Clicked {
            commands.insert_resource(NextState(AppState::MainScreen));
        }
    }
    pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
        let font = server.load("NotoSans-Regular.ttf");

        let mut entity = commands.spawn((
            NodeBundle {
                style: Style {
                    position: UiRect::all(Val::Percent(0.0)),
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("title-screen"),
        ));
        entity.with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    "Project Flara",
                    TextStyle {
                        font: font.clone(),
                        font_size: 64.0,
                        ..default()
                    },
                ),
                ..default()
            });

            parent
                .spawn((
                    ButtonBundle {
                        background_color: BackgroundColor(Color::PINK),
                        style: Style {
                            padding: UiRect::all(Val::Px(15.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("play-button"),
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Play now!",
                            TextStyle {
                                font: font.clone(),
                                font_size: 36.0,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
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
pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
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

impl StatePlugin for TitlePlugin {
    const STATE: AppState = AppState::TitleScreen;
}
