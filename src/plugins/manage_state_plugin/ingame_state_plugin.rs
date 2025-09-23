///// SPECS
// - button MainMenuButton nextStates to MainMenu
//

mod explore_substate;
mod combat_substate;
mod shop_substate;

use crate::plugins::manage_state_plugin::{ GameModeState, InGameSubstate };
use crate::plugins::manage_state_plugin::ingame_state_plugin::{
    explore_substate::{ setup_exploresubstate, explore_movement_controls },
};
use crate::exploration_movement::ExplorationMovementPlugin;

use bevy::prelude::*;


pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::InGame), setup);
        app.add_systems(
            Update,
            check_enter_mainmenu_system.run_if(in_state(GameModeState::InGame)),
        );
        
        app.add_plugins(ExplorationMovementPlugin);
        app.add_systems(OnEnter(InGameSubstate::Explore), setup_exploresubstate );
        app.add_systems(
            Update,
            explore_movement_controls.run_if(in_state(InGameSubstate::Explore))
        );

        //
        //InGameSubstate::Explore
        //
        //InGameSubstate::Combat
        //
        //InGameSubstate::Shop
        //
    }
}


#[allow(dead_code)]
#[derive(Resource)]
struct InGameData {
    name: String
}

#[derive(Component)]
struct InGameRootNode;


fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(initialize_ingame_data());
}

fn initialize_ingame_data() -> InGameData {
    InGameData {
        name: String::from("placeholder character name")
    }
}

fn check_enter_mainmenu_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameModeState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(GameModeState::MainMenu);
    }
}

// fn cleanup() {
//
// }
