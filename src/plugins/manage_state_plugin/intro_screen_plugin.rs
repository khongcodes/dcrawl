use crate::plugins::manage_state_plugin::GameModeState;
use bevy::prelude::*;

pub struct IntroScreenPlugin;

impl Plugin for IntroScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::IntroScreen), setup_intro_screen);
        app.add_systems(OnExit(GameModeState::IntroScreen), cleanup_intro_screen);
        app.add_systems(
            Update,
            run_intro_screen.run_if(in_state(GameModeState::IntroScreen)),
        );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource)]
struct IntroScreenData {
    start_button_entity: Entity,
}

fn setup_intro_screen(
    query: Query<Entity, With<IsDefaultUiCamera>>,
    mut commands: Commands
) {

    // TODO: try to query in IsDefaultUiCamera and make that target camera?
    let ui_camera = query.single();

    let start_button_entity = commands
        .spawn((
            Node {
                // center button
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            TargetCamera(ui_camera),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Button, 
                    Node {
                        width: Val::Px(150.),
                        height: Val::Px(65.),
                        // horizontally + vertically center child text
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON.into())
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start"),
                        TextFont {
                            font_size: 40.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9))
                    ));
                });
        })
        .id();
    commands.insert_resource(IntroScreenData {
        start_button_entity,
    });
}

fn cleanup_intro_screen(mut commands: Commands, introscreen_data: Res<IntroScreenData>) {
    println!("cleaning up intro screen");
    commands
        .entity(introscreen_data.start_button_entity)
        .despawn_recursive();
}

fn run_intro_screen(
    mut next_state: ResMut<NextState<GameModeState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameModeState::Menu);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
