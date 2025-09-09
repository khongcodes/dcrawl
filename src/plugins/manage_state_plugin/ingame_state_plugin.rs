///// SPECS
// - button MainMenuButton nextStates to MainMenu
//

use crate::plugins::manage_state_plugin::{ GameModeState, InGameSubstate };
use crate::plugins::camera_plugin::NavigateCamera;
use bevy::prelude::*;

mod explore_substate;
mod combat_substate;
mod dialogue_substate;
mod shop_substate;
mod pausemenu_substate;

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameModeState::InGame), setup);
        app.add_systems(
            Update,
            check_enter_mainmenu_system.run_if(in_state(GameModeState::InGame)),
        );
        
        //
        //InGameSubstate::Explore
        //
        //InGameSubstate::Combat
        //
        //InGameSubstate::Shop
        //
        //InGameSubstate::Dialogue
        //
        //InGameSubstate::PauseMenu
    }
}


#[derive(Resource)]
struct InGameData {
    name: String
}

#[derive(Component)]
struct InGameRootNode;


// TODO: 
// 1. initialize game data
// 2. make TargetCamera the 3d camera
fn setup(
    mut commands: Commands,
    camera_query: Query<Entity, With<NavigateCamera>>
) {

    commands.insert_resource(initialize_ingame_data());

    commands.spawn((
        InGameRootNode,
        Node {
            ..default()
        }
    ));

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

fn initialize_ingame_data() -> InGameData {
    InGameData {
        name: String::from("placeholder character name")
    }
}

fn check_enter_mainmenu_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<InGameSubstate>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(InGameSubstate::PauseMenu);
    }
}

fn cleanup() {

}
