use crate::plugins::manage_state_plugin::GameModeState;
use bevy::prelude::*;

pub struct GameRuntimePlugin;

impl Plugin for GameRuntimePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameModeState::IntroScreen), setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    commands.spawn(
        (Camera3dBundle {
            transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }),
    );
}
