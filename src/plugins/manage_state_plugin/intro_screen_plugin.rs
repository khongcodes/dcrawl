///// SPECS
// - button StartButton nextStates to MainMenu

use crate::plugins::{ manage_state_plugin::GameModeState, camera_plugin::UiCamera };
use bevy::prelude::*;


pub struct IntroScreenPlugin;

impl Plugin for IntroScreenPlugin {
    fn build(&self, app: &mut App) {
        info!("Running IntroScreenPlugin app::build");
        app.add_systems(OnEnter(GameModeState::IntroScreen), setup_intro_screen);
        app.add_systems(OnExit(GameModeState::IntroScreen), cleanup_intro_screen);
        app.add_systems(
            Update,
            run_intro_screen.run_if(in_state(GameModeState::IntroScreen)),
        );
    }
}


#[derive(Component)]
pub struct IntroScreenRootNode;

// TEST: CALLING THIS IN STARTUP IN MAIN.RS
pub fn setup_intro_screen(
    camera_query: Query<Entity, With<UiCamera>>,
    root_query: Query<Entity, With<IntroScreenRootNode>>,
    mut commands: Commands
) {

    if root_query.single().is_ok() {
        return;
    }

    let ui_camera = match camera_query.single() {
        Ok(c) => c,
        Err(_) => return
    };
    
    commands.spawn((
        // root container to center the button within
        IntroScreenRootNode,
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
        UiTargetCamera(ui_camera),
        children![(
            Button, 
            Node {
                width: Val::Px(150.),
                height: Val::Px(65.),
                // horizontally + vertically center child text
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(NORMAL_BUTTON.into()),
            children![(
                Text::new("Start"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9))
            )]
        )]
    ));

}


fn cleanup_intro_screen(
    query: Query<Entity, With<IntroScreenRootNode>>,
    mut commands: Commands, 
) {
    let introscreen_root_node = match query.single() {
        Ok(n) => n,
        Err(_) => return,
    };

    commands
        .entity(introscreen_root_node)
        .despawn();
}


/////////////////////////////////////////
// BUTTON STYLES

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

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
                next_state.set(GameModeState::MainMenu);
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

