use std::time::Duration;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
#[derive(Component, Inspectable)]
pub struct StartupMarker;

#[derive(Component, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct StartupTimer(Timer);
pub fn startup_system(mut commands: Commands, server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    let font = server.load("NotoSans-Regular.ttf");
    let startup_name = Name::new("startup-screen");
    // Creating the animation
    let mut animation = AnimationClip::default();
    // A curve can modify a single part of a transform, here the translation
    animation.add_curve_to_path(
        EntityPath {
            parts: vec![startup_name.clone()],
        },
        VariableCurve {
            keyframe_timestamps: vec![0.0, 1.5, 3.0],
            keyframes: Keyframes::Scale(vec![
                Vec3::splat(0.2),
                Vec3::splat(2.0),
                Vec3::splat(1.0),
            ]),
        },
    );
    let mut startup = commands.spawn((
        ButtonBundle {
            background_color: BackgroundColor(Color::FUCHSIA),
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
        startup_name,
        StartupMarker,
        StartupTimer(Timer::from_seconds(5.0, TimerMode::Once)),
    ));
    startup.with_children(|parent| {
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Produced by",
                TextStyle {
                    font: font.clone(),
                    font_size: 36.0,
                    ..default()
                },
            ),
            ..default()
        });

        parent.spawn(TextBundle {
            text: Text::from_section(
                "Fiana Fortressia",
                TextStyle {
                    font: font.clone(),
                    font_size: 64.0,
                    ..default()
                },
            ),
            ..default()
        });
    });
}