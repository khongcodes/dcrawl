//! This code does the following:
//! 1. Declare GameModeState and InGameSubstate variants.
//! 2. Initialize app state.
//! 3. Declare other plugins that define functionality on the related game modes.
//!
//! See documentation on SubStates trait:
//!     - https://docs.rs/bevy/latest/bevy/state/state/trait.SubStates.html
//!

mod ingame_state_plugin;
pub mod intro_screen_plugin;
mod loadgame_menu_plugin;
mod main_menu_plugin;

use crate::plugins::manage_state_plugin::{
    ingame_state_plugin::InGameStatePlugin, 
    intro_screen_plugin::IntroScreenPlugin,
    loadgame_menu_plugin::LoadGameMenuPlugin,
    main_menu_plugin::MainMenuPlugin,
};

use bevy::prelude::*;


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameModeState {
    #[default]
    IntroScreen,
    MainMenu,
    LoadGameMenu,
    InGame
}

/// InGameSubstate only exists when app state is GameModeState::InGame.
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameModeState = GameModeState::InGame)]
pub enum InGameSubstate {
    #[default]
    Explore,
    Combat,
    Shop,
    Dialogue,
    PauseMenu,
}

pub struct ManageStatePlugin;

impl Plugin for ManageStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameModeState>();
        app.add_sub_state::<InGameSubstate>();
        app.add_plugins((IntroScreenPlugin, MainMenuPlugin, LoadGameMenuPlugin, InGameStatePlugin));
    }
}
