mod ingame_state_plugin;
mod intro_screen_plugin;
mod main_menu_plugin;
use crate::plugins::manage_state_plugin::{
    ingame_state_plugin::InGameStatePlugin, 
    intro_screen_plugin::IntroScreenPlugin,
    main_menu_plugin::MainMenuPlugin,
};

use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    #[default]
    IntroScreen,
    Menu,
    InGame,
}

pub struct ManageStatePlugin;

impl Plugin for ManageStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameModeState>();
        app.add_plugins((IntroScreenPlugin, MainMenuPlugin, InGameStatePlugin));
    }
}
