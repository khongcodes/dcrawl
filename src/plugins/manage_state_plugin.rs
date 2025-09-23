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

// InGameSubstate only exists when app state is GameModeState::InGame.
// Combat, Shop, and Explore should be the true SubStates
// Dialogue can occur during Shop or Explore as overlay
// PauseMenu can be entered from any screen as overlay
// CharacterMenu can be entered from Explore also as overlay
#[derive(SubStates, Default, Debug, Clone, PartialEq, Eq, Hash)]
#[source(GameModeState = GameModeState::InGame)]
pub enum InGameSubstate {
    #[default]
    Explore,
    // TEMPORARILY COMMENTED OUT THE BELOW TO FOCUS ON IMPLEMENTING EXPLORE STATE
    // Combat,
    // Shop
}

pub struct ManageStatePlugin {
    pub start_ingame: bool
}

impl Plugin for ManageStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameModeState>();
        app.add_sub_state::<InGameSubstate>();
        app.add_plugins((IntroScreenPlugin, MainMenuPlugin, LoadGameMenuPlugin, InGameStatePlugin));

        if self.start_ingame {
            app.add_systems(Startup, switchstate_ingame);
        }
    }
}

fn switchstate_ingame(mut next_state: ResMut<NextState<GameModeState>>) {
    next_state.set(GameModeState::InGame);
}
