use bevy::prelude::*;

use super::story_loading::{load, MainStory};

pub struct MainStoryMenu;

impl Plugin for MainStoryMenu {
    fn build(&self, _app: &mut App) {}
}

impl MainStoryMenu {
    pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
        let font = server.load("NotoSans-Regular.ttf");
        commands
            .spawn(NodeBundle { ..default() })
            .with_children(|parent| {
                for variation in MainStory::VARIATIONS {
                    match unsafe { load(&variation) } {
                        Ok(story) => {
                            parent
                                .spawn((
                                    ButtonBundle { ..default() },
                                    Name::new(variation.module_name()),
                                ))
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
                        Err(_err) => {
                            parent
                                .spawn(NodeBundle { ..default() })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Error!",
                                            TextStyle {
                                                font: font.clone(),
                                                ..default()
                                            },
                                        ),
                                        ..default()
                                    });
                                });
                        }
                    }
                }
            });
    }
    pub fn on_update() {}
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
