use bevy::prelude::*;
use crate::plugins::manage_state_plugin::GameModeState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
   fn build(&self, app: &mut App) {
      app.add_systems(OnEnter(GameModeState::Menu), setup_mainmenu);
      app.add_systems(OnExit(GameModeState::Menu), cleanup_mainmenu);
      app.add_systems(Update, run_mainmenu.run_if(in_state(GameModeState::Menu)));
   }
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Resource)]
struct MainMenuData {
   continue_button_entity: Entity,
   exit_button_entity: Entity
}

fn setup_mainmenu(mut commands: Commands) {
   let continue_button_entity = commands
      .spawn(NodeBundle {
            style: Style{
               width: Val::Percent(100.0),
               height: Val::Percent(50.0),
               align_items: AlignItems::Center,
               justify_content: JustifyContent::Center,
               ..default()
            },
            ..default()
      })
      .with_children(|parent| { parent.spawn(
            ButtonBundle {
               style: Style {
                  width: Val::Px(150.0),
                  height: Val::Px(65.0),
                  border: UiRect::all(Val::Px(5.0)),

                  // horizontally center child text
                  justify_content: JustifyContent::Center,
                  // vertically center child text
                  align_content: AlignContent::Center,
                  ..default()
               },
               border_color: BorderColor(Color::BLACK),
               border_radius: BorderRadius::MAX,
               background_color: NORMAL_BUTTON.into(),
               ..default()
            })
            .with_children(|parent| {
               parent.spawn(TextBundle::from_section(
                  "Continue", 
                  TextStyle { 
                        font_size: 40.0, 
                        color: Color::srgb(0.9, 0.9, 0.9), 
                        ..default() 
                  }
               ));
            });
      })
      .id();

   let exit_button_entity = commands
      .spawn(NodeBundle {
         style: Style{
            top: Val::Percent(50.0),
            width: Val::Percent(100.0),
            height: Val::Percent(50.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
         },
         ..default()
      })
      .with_children(|parent| { parent.spawn(
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
         })
         .with_children( |parent| {
            parent.spawn(TextBundle::from_section(
               String::from("Exit"),
               TextStyle {
                  font_size: 80.0,
                  color: Color::srgb(0.9, 0.9, 0.9), 
                  ..default() 
               }
            ));
         });
      })
      .id();

   commands.insert_resource(MainMenuData { exit_button_entity, continue_button_entity });
}


// TODO: when pressed, use commands.run_system_with_input() to change game state
fn run_mainmenu(
   mut interaction_query: Query<
      (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
      ),
      (Changed<Interaction>, With<Button>)
   >,
   mut text_query: Query<&mut Text>
) {
   for (interaction, mut color, mut border_color, children) in &mut interaction_query {
      let mut text = text_query.get_mut(children[0]).unwrap();
      match *interaction {
            Interaction::Pressed => {
               text.sections[0].value = "CONTINUING".to_string();
               *color = PRESSED_BUTTON.into();
               border_color.0 = bevy::color::palettes::basic::RED.into();
            }
            Interaction::Hovered => {}
            Interaction::None => {
               text.sections[0].value = String::from("Continue");
               *color = NORMAL_BUTTON.into();
               border_color.0 = Color::BLACK;
            }
      }
   }
}


fn cleanup_mainmenu() {

}
