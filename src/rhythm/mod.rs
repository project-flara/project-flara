//! Code for the startup screen that usually contains the developer's logo, etc.
//!

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::*;

use crate::{state::AppState, StatePlugin};
#[derive(Component, Inspectable)]
pub struct StartupMarker;

#[derive(Component, Deref, DerefMut, Reflect, Default)]
#[reflect(Component)]
pub struct StartupTimer(Timer);
impl NavigationMapPlugin {
    pub fn on_enter(
        mut commands: Commands,
        server: Res<AssetServer>,
        mut animations: ResMut<Assets<AnimationClip>>,
    ) {
        let font = server.load("NotoSans-Regular.ttf");
        let startup_name = Name::new("startup-screen");
        let author_name = Name::new("author-name");
        // Creating the animation
        let mut animation = AnimationClip::default();
        // A curve can modify a single part of a transform, here the translation
        animation.add_curve_to_path(
            EntityPath {
                parts: vec![author_name.clone()],
            },
            VariableCurve {
                keyframe_timestamps: vec![0.0, 0.5, 0.1],
                keyframes: Keyframes::Scale(vec![
                    Vec3::splat(0.2),
                    Vec3::splat(1.5),
                    Vec3::splat(1.0),
                ]),
            },
        );
        // Create the animation player, and set it to repeat
        let mut player = AnimationPlayer::default();
        player.play(animations.add(animation));
        let mut startup = commands.spawn((
            ButtonBundle {
                style: Style {
                    position: UiRect::all(Val::Percent(0.0)),
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    player,
                    author_name,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Produced by",
                            TextStyle {
                                font: font.clone(),
                                font_size: 36.0,
                                color: Color::BLACK,
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
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    });
                });
        });
    }
    pub fn on_update(
        mut query: Query<(&Interaction, &Name, &mut StartupTimer)>,

        time: Res<Time>,
        mut commands: Commands,
    ) {
        let (interaction, _, mut timer) = query
            .iter_mut()
            .find(|(_, name, _)| name.as_str() == "startup-screen")
            .unwrap();

        if !timer.finished() && !(*interaction == Interaction::Clicked) {
            timer.tick(time.delta());
        } else {
            commands.insert_resource(NextState(AppState::TitleScreen));
        }
    }
    pub fn on_exit(query: Query<(Entity, &Name)>, mut commands: Commands) {
        let entity = query
            .iter()
            .find(|(_, name)| name.as_str() == "startup-screen")
            .unwrap()
            .0;
        commands.entity(entity).despawn_recursive();
    }
}
pub struct NavigationMapPlugin;

impl Plugin for NavigationMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartupTimer>();
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

impl StatePlugin for NavigationMapPlugin {
    const STATE: AppState = AppState::StartupScreen;
}
