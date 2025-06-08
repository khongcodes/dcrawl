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

#[derive(Component)]
#[require(Camera2d)]
pub struct UiCamera;

#[derive(Component)]
#[require(Camera3d)]
pub struct NavigateCamera;

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn(UiCamera);
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
