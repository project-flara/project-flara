use bevy::prelude::*;

use crate::{state::AppState, state::StoryState, StatePlugin};
pub struct MainScreenPlugin;

impl Plugin for MainScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Self::STATE).with_system(Self::on_enter),
        );

        app.add_system_set(
            SystemSet::on_exit(Self::STATE).with_system(Self::on_exit),
        );

        app.add_system_set(
            SystemSet::on_update(Self::STATE).with_system(Self::on_update),
        );
    }
}

impl MainScreenPlugin {
    pub fn on_update(
        query: Query<(&Name, &Interaction)>,
        mut state: ResMut<State<AppState>>,
    ) {
        let interaction = query
            .iter()
            .find(|(name, _)| name.as_str() == "story-button")
            .unwrap()
            .1;

        if *interaction == Interaction::Clicked {
            state.set(AppState::Story(StoryState::Menu)).unwrap();
        }
    }

    pub fn on_enter(mut commands: Commands, server: Res<AssetServer>) {
        let font = server.load("NotoSans-Regular.ttf");
        commands
            .spawn((
                ButtonBundle {
                    background_color: BackgroundColor(Color::PINK),
                    style: Style {
                        position: UiRect {
                            right: Val::Px(20.0),
                            bottom: Val::Px(20.0),
                            ..default()
                        },
                        size: Size::new(Val::Px(100.0), Val::Px(190.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },

                    ..default()
                },
                Name::new("story-button"),
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Story",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                    ),
                    ..default()
                });
            });
    }

    pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
        let entity = query
            .iter()
            .find(|(_, name)| name.as_str() == "story-button")
            .unwrap()
            .0;
        commands.entity(entity).despawn_recursive();
    }
}

impl StatePlugin for MainScreenPlugin {
    const STATE: AppState = AppState::MainScreen;
}
