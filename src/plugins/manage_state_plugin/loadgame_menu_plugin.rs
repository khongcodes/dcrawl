///// SPECS
// - render savefile if exists (TBD)
// - button ReturnButton nextStates to MainMenu
// - button LoadGameButton nextStates (for now) to InGame
// - button DeleteSaveButton deletes the savefile (functionality TBD)
//
// TODO: SAVELOAD SYSTEMS create and read off text files?

use crate::plugins::manage_state_plugin::GameModeState;
use bevy::{ecs::spawn::SpawnRelatedBundle, prelude::*};


/////////////////////////////////////////
// CONFIGURABLES
// - BUTTON COLORS
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);


/////////////////////////////////////////
// PLUGIN DEFINITION

pub struct LoadGameMenuPlugin;

impl Plugin for LoadGameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::LoadGameMenu), setup_loadgamemenu);
        app.add_systems(OnExit(GameModeState::LoadGameMenu), cleanup_loadgamemenu);
        app.add_systems(
            Update,
            (style_buttons, loadgamemenu_action_system).run_if(in_state(GameModeState::LoadGameMenu))
        );
    }
}


/////////////////////////////////////////
// NODE STRUCTURE

#[derive(Component)]
struct LoadGameMenuRootNode;

fn setup_loadgamemenu(
    camera_query: Query<Entity, With<IsDefaultUiCamera>>,
    mut commands: Commands
) {
    let ui_camera = match camera_query.single() {
        Ok(c) => c,
        Err(_) => return,
    };

    commands.spawn((
        LoadGameMenuRootNode,
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
                Node {
                    width: Val::Percent(50.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Start,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        Node {
                            height: Val::Percent(100.0),
                            aspect_ratio: Some(1.),
                            ..default()
                        },
                        BackgroundColor(Color::srgb(0.9, 0.05, 0.05))
                    ),
                    (
                        Node {
                            height: Val::Percent(100.0),
                            margin: UiRect {
                                left: Val::Px(20.),
                                ..default()
                            },
                            ..default()
                        },
                        children![(
                            Text::new("Placeholder save data"),
                            TextFont {
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.9, 0.9, 0.9))
                        )]

                    )
                ]
            ),
            (
                LoadGameMenuButtonAction::Load,
                generate_placeholder_button("Load")
            ),
            (
                LoadGameMenuButtonAction::Erase,
                generate_placeholder_button("Erase")
            ),
            (
                LoadGameMenuButtonAction::Return,
                Button,
                Node {
                    width: Val::Px(150.),
                    height: Val::Px(65.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.),
                    right: Val::Px(14.),
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON.into()),
                children![(
                    Text::new("Return"),
                    TextFont {
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9))
                )]
            ),
        ]
    ));
} 

fn cleanup_loadgamemenu(
    query: Query<Entity, With<LoadGameMenuRootNode>>,
    mut commands: Commands
) {
    let loadgamemenu_rootnode = match query.single() {
        Ok(n) => n,
        Err(_) => return,
    };

    commands
        .entity(loadgamemenu_rootnode)
        .despawn();
}


/////////////////////////////////////////
// BUTTON FUNCTIONALITY

#[derive(Component)]
enum LoadGameMenuButtonAction {
    Load,
    Erase,
    Return
}

fn loadgamemenu_action_system(
    interaction_query: Query<
        (&Interaction, &LoadGameMenuButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut next_state: ResMut<NextState<GameModeState>>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if interaction == &Interaction::Pressed {
            match menu_button_action {
                LoadGameMenuButtonAction::Load => {
                    next_state.set(GameModeState::InGame);
                }
                LoadGameMenuButtonAction::Erase => {
                    next_state.set(GameModeState::IntroScreen);
                }
                LoadGameMenuButtonAction::Return => {
                    next_state.set(GameModeState::MainMenu);
                }
            }
        }
    }
}


/////////////////////////////////////////
// BUTTON STYLING

fn style_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>)
    >
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


/////////////////////////////////////////
// HELPER FUNCTIONS

fn generate_placeholder_button(text: &str) -> (Button, Node, BackgroundColor, SpawnRelatedBundle<ChildOf, Spawn<(Text, TextFont, TextColor)>>) {
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
                font_size: 20.0,
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9))
        )]
    )
}

