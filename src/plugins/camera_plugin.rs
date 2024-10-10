use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            setup_ui_camera,
            setup_runtime_camera
        )); 
    }
}

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
        IsDefaultUiCamera
    ));
}

pub fn setup_runtime_camera(mut commands: Commands) {
    commands.spawn(
        Camera3dBundle {
            camera_3d: Camera3d {
                ..default()
            },
            camera: Camera {
                order: 1,
                clear_color: ClearColorConfig::None,
                ..default()
            },
            ..default()
        }
    );
}
