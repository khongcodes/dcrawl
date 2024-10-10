mod ingame_state_plugin;
mod intro_screen_plugin;
mod main_menu_plugin;
use crate::plugins::manage_state_plugin::{
    ingame_state_plugin::InGameStatePlugin, 
    intro_screen_plugin::IntroScreenPlugin,
    main_menu_plugin::MainMenuPlugin,
};

use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    IntroScreen,
    Menu,
    InGame,
}

// TODO: system that changes game mode

pub struct ManageStatePlugin;

impl Plugin for ManageStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameModeState::IntroScreen);
        app.add_plugins((IntroScreenPlugin, MainMenuPlugin, InGameStatePlugin));
    }
}
