use crate::plugins::manage_state_plugin::GameModeState;
use bevy::prelude::*;

pub struct InGameStatePlugin;

impl Plugin for InGameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            check_enter_mainmenu_system.run_if(in_state(GameModeState::InGame)),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
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
