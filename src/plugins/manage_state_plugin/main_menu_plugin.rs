use crate::plugins::manage_state_plugin::GameModeState;
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::Menu), setup_mainmenu);
        app.add_systems(OnExit(GameModeState::Menu), cleanup_mainmenu);
        app.add_systems(
            Update,
            (button_style_system, main_menu_action_system).run_if(in_state(GameModeState::Menu)),
        );
    }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
const HOVERED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource, std::fmt::Debug)]
struct MainMenuData {
    screen_node: Entity,
}
#[derive(Component)]
struct MainMenuButtonMarker;

fn setup_mainmenu(
    camera_query: Query<Entity, With<IsDefaultUiCamera>>,
    mut commands: Commands
) {

    let camera = camera_query.single();

    // render a screen
    let screen_node = commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                     justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            TargetCamera(camera)
        ))
        .id();

    let continue_button_entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::MAX,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MainMenuButtonAction::Continue,
            MainMenuButtonMarker,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Continue",
                TextStyle {
                    font_size: 40.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        })
        .id();

    let quit_button_entity = commands
        .spawn((
            ButtonBundle {
                style: Style {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    ..default()
                },
                border_color: BorderColor(Color::BLACK),
                border_radius: BorderRadius::MAX,
                background_color: NORMAL_BUTTON.into(),
                ..default()
            },
            MainMenuButtonAction::Quit,
            MainMenuButtonMarker
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Exit",
                TextStyle {
                    font_size: 80.0,
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        })
        .id();

    commands
        .entity(screen_node)
        .push_children(&vec![continue_button_entity, quit_button_entity]);
    commands.insert_resource(MainMenuData { screen_node });
}

fn cleanup_mainmenu(mut commands: Commands, mainmenu_data: Res<MainMenuData>) {
    commands
        .entity(mainmenu_data.screen_node)
        .despawn_recursive();
}

/////////////////////////////////
// MENU ACTIONS
#[derive(Component)]
enum MainMenuButtonAction {
    Continue,
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
                MainMenuButtonAction::Continue => {
                    println!("i was here");
                    next_state.set(GameModeState::InGame);
                }

                MainMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
            }
        }
    }
}
