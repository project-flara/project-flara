//! Main screen
//!
use crate::{state::AppState, state::StoryState, StatePlugin};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
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
            SystemSet::on_update(Self::STATE)
                .with_system(Self::on_update)
                .with_system(Self::player_movement),
        );
    }
}
#[derive(Component)]
pub struct Player(f32);
impl MainScreenPlugin {
    pub fn on_update(
        buttons: Query<(&Name, &Interaction)>,

        mut state: ResMut<State<AppState>>,
    ) {
        let interaction = buttons
            .iter()
            .find(|(name, _)| name.as_str() == "story-button")
            .unwrap()
            .1;

        if *interaction == Interaction::Clicked {
            state.set(AppState::Story(StoryState::Menu)).unwrap();
        }
    }

    pub fn on_enter(
        mut commands: Commands,
        server: Res<AssetServer>,
        mut rapier_config: ResMut<RapierConfiguration>,
    ) {
        // Set gravity to 0.0 and spawn camera.
        rapier_config.gravity = Vec2::ZERO;
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

        commands
            .spawn((
                ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Percent(40.0),
                            right: Val::Percent(40.0),
                            top: Val::Auto,
                            bottom: Val::Px(0.0),
                        },
                        size: Size::new(Val::Percent(19.0), Val::Auto),
                        padding: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    ..default()
                },
                BottomCenterMenu,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "View Map",
                        TextStyle {
                            font: font.clone(),
                            font_size: 24.0,
                            color: Color::PINK,
                        },
                    ),
                    ..default()
                });
            });
        let sprite_size = 100.0;

        // Spawn entity with `Player` struct as a component for access in movement query.
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(sprite_size, sprite_size)),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::ball(sprite_size / 2.0),
            Player(100.0),
        ));
    }

    pub fn on_exit(
        query: Query<(Entity, &Name)>,
        bottom_center_menu: Query<Entity, With<BottomCenterMenu>>,
        mut commands: Commands,
    ) {
        commands
            .entity(bottom_center_menu.single())
            .despawn_recursive();
        let entity = query
            .iter()
            .find(|(_, name)| name.as_str() == "story-button")
            .unwrap()
            .0;
        commands.entity(entity).despawn_recursive();
    }

    pub fn player_movement(
        keyboard_input: Res<Input<KeyCode>>,
        mut player_info: Query<(&Player, &mut Velocity)>,
    ) {
        for (player, mut rb_vels) in &mut player_info {
            let up = keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]);
            let down = keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]);
            let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
            let right =
                keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

            let x_axis = -(left as i8) + right as i8;
            let y_axis = -(down as i8) + up as i8;

            let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
            if move_delta != Vec2::ZERO {
                move_delta /= move_delta.length();
            }

            // Update the velocity on the rigid_body_component,
            // the bevy_rapier plugin will update the Sprite transform.
            rb_vels.linvel = move_delta * player.0;
        }
    }
}

impl StatePlugin for MainScreenPlugin {
    const STATE: AppState = AppState::MainScreen;
}

#[derive(Component)]
pub struct BottomCenterMenu;
