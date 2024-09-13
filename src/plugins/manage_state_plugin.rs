mod intro_screen_plugin;
mod main_menu_plugin;
mod ingame_plugin;
use crate::plugins::manage_state_plugin::{
   intro_screen_plugin::IntroScreenPlugin,
   main_menu_plugin::MainMenuPlugin
};

use bevy::prelude::*;
use ingame_plugin::InGamePlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    IntroScreen,
    Menu,
    InGame
}


// TODO: system that changes game mode

pub struct ManageStatePlugin;

impl Plugin for ManageStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameModeState::IntroScreen);
        app.add_plugins((
         IntroScreenPlugin,
         MainMenuPlugin,
         InGamePlugin
      ));
    }
}

