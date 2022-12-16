use bevy::prelude::*;

use super::startup::{StartupMarker, StartupTimer};

pub fn main_screen(
    mut commands: Commands,
    server: Res<AssetServer>,

    mut query: Query<(Entity, &StartupMarker, &mut StartupTimer, &Interaction)>,
    time: Res<Time>,
) {
    if let Some((entity, _, mut timer, interaction)) = query.iter_mut().next() {
        if !timer.finished() && !(*interaction == Interaction::Clicked) {
            timer.tick(time.delta());
            return;
        };
        commands.entity(entity).despawn_recursive();

        let font = server.load("NotoSans-Regular.ttf");

        let mut entity = commands.spawn(NodeBundle {
            style: Style {
                position: UiRect::all(Val::Percent(0.0)),
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        });
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
                .spawn(ButtonBundle {
                    background_color: BackgroundColor(Color::PINK),
                    style: Style {
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                })
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
}
