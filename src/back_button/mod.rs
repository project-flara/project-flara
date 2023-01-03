use bevy::prelude::*;
use framework::states::{AppState, StoryState};
use iyes_loopless::prelude::*;

pub struct BackButtonPlugin;

impl Plugin for BackButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::on_enter)
            .add_system(Self::on_update);
    }
}

impl BackButtonPlugin {
    pub fn on_enter(mut commands: Commands) {
        commands.insert_resource(StateHistory::<AppState>(vec![
            AppState::MainScreen,
        ]));
        Self::ui(commands);
    }
    pub fn ui(mut commands: Commands) {
        commands
            .spawn((
                ButtonBundle {
                    background_color: BackgroundColor(Color::PINK),
                    style: Style {
                        size: Size::new(Val::Px(55.), Val::Px(55.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Percent(0.0),
                            top: Val::Px(15.0),
                            ..default()
                        },

                        ..default()
                    },
                    z_index: ZIndex::Global(2),
                    ..default()
                },
                BackButton,
            ))
            .with_children(|parent| {});
    }

    pub fn on_update(
        state: Res<CurrentState<AppState>>,
        mut button: Query<&mut Visibility, With<BackButton>>,
        mut history: ResMut<StateHistory<AppState>>,
        interaction: Query<
            &Interaction,
            (With<BackButton>, Changed<Interaction>),
        >,
        mut commands: Commands,
    ) {
        if state.is_changed() {
            history.0.push(state.0.clone());
            let display: bool = match state.0 {
                AppState::TitleScreen
                | AppState::StartupScreen
                | AppState::MainScreen
                | AppState::Story(StoryState::Running) => false,

                AppState::Event
                | AppState::Story(
                    StoryState::MainStory
                    | StoryState::Menu
                    | StoryState::ChapterMenu
                    | StoryState::Events,
                ) => true,
            };

            if let Ok(mut visibility) = button.get_single_mut() {
                visibility.is_visible = display;
            }
        }

        if let Ok(interaction) = interaction.get_single() {
            if *interaction == Interaction::Clicked {
                println!("{:?}", state.0);

                if if let Some(previous) = history
                    .0
                    .iter()
                    .rfind(|prev_state| **prev_state != state.0)
                    .clone()
                {
                    commands.insert_resource(NextState(previous.clone()));
                    true
                } else {
                    false
                } {
                    history.0.pop();
                }
            }
        }
    }
}

#[derive(Resource)]
pub struct StateHistory<A>(Vec<A>);
#[derive(Component)]
pub struct BackButton;
