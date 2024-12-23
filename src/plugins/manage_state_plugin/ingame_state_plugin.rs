use crate::plugins::manage_state_plugin::GameModeState;
use crate::plugins::camera_plugin::setup_runtime_camera;
use bevy::prelude::*;

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameModeState::InGame), (
                setup
            )
        );
        app.add_systems(
            Update,
            check_enter_mainmenu_system.run_if(in_state(GameModeState::InGame)),
        );
    }
}

enum InGameStatus {
    Explore,
    Combat,
    Shop,
    Dialogue
}

#[derive(Resource)]
struct InGameData {
    status: InGameStatus
}

// TODO: 
// 1. check if ingame status resource exists
// 2. IF NOT: initialize a resource for ingame status
// 3. make TargetCamera the 3d camera
fn setup(mut commands: Commands) {
    
    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
}

fn initialize_ingame_data(mut commands: Commands) {
    commands.insert_resource( InGameData {
        status: InGameStatus::Explore
    });
}

fn check_enter_mainmenu_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameModeState>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        next_state.set(GameModeState::Menu);
    }
}

// TODO:
// do NOT delete the ingamestatus resource
fn cleanup() {

}
