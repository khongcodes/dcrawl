use crate::plugins::manage_state_plugin::GameModeState;
use bevy::{ecs::spawn::SpawnRelatedBundle, prelude::*};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::MainMenu), setup_mainmenu);
        app.add_systems(OnExit(GameModeState::MainMenu), cleanup_mainmenu);
        app.add_systems(
            Update,
            (button_style_system, main_menu_action_system).run_if(in_state(GameModeState::MainMenu)),
        );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


#[derive(Component)]
struct MainMenuRootNode;

fn setup_mainmenu(
    camera_query: Query<Entity, With<IsDefaultUiCamera>>,
    mut commands: Commands
) {

    let ui_camera = match camera_query.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    // render a screen
    commands.spawn((
        MainMenuRootNode,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
             justify_content: JustifyContent::Center,
            ..default()
        },
        UiTargetCamera(ui_camera),
        children![
            (
                MainMenuButtonAction::New,
                generate_main_menu_button("New")
            ),
            (
                MainMenuButtonAction::Load, 
                generate_main_menu_button("Load")
            ),
            (
                MainMenuButtonAction::Quit,
                generate_main_menu_button("Quit")
            )
        ]
    ));

}

fn cleanup_mainmenu(
    query: Query<Entity, With<MainMenuRootNode>>,
    mut commands: Commands
) {
    let mainmenu_rootnode = match query.single() {
        Ok(n) => n,
        Err(_) => return,
    };

    commands
        .entity(mainmenu_rootnode)
        .despawn();
}

/////////////////////////////////
// MENU ACTIONS
#[derive(Component)]
enum MainMenuButtonAction {
    New,
    Load,
    Quit,
}

fn button_style_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                border_color.0 = bevy::color::palettes::basic::RED.into();
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                border_color.0 = bevy::color::palettes::basic::RED.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn main_menu_action_system(
    interaction_query: Query<
        (&Interaction, &MainMenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameModeState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if interaction == &Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::New => {
                    next_state.set(GameModeState::InGame);
                },
                MainMenuButtonAction::Load => {
                    next_state.set(GameModeState::LoadGameMenu);
                },
                MainMenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
            }
        }
    }
}


//////
// HELPER FUNCTIONS 
/////////////////////////
///

fn generate_main_menu_button(text: &str) -> (Button, Node, BackgroundColor, SpawnRelatedBundle<ChildOf, Spawn<(Text, TextFont, TextColor)>>) {
    (
        Button,
        Node {
            width: Val::Px(150.),
            height: Val::Px(65.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(NORMAL_BUTTON.into()),
        children![(
            Text::new(text),
            TextFont {
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    )
}
