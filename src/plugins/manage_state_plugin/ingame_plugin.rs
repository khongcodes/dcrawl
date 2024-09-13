use bevy::prelude::*;
use crate::plugins::manage_state_plugin::GameModeState;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
   fn build(&self, app: &mut App) {
      app.add_systems(Update, check_enter_mainmenu_system.run_if(in_state(GameModeState::InGame)));
   }
}

fn check_enter_mainmenu_system(
   keyboard_input: Res<ButtonInput<KeyCode>>,
   mut next_state: ResMut<NextState<GameModeState>>
) {
   if keyboard_input.pressed(KeyCode::Escape) {
      next_state.set(GameModeState::Menu);
   }
}