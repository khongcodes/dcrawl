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
    commands.spawn((UiCamera, Camera {
        order: 2,
        clear_color: ClearColorConfig::None,
        ..default()
    }));
}

pub fn setup_runtime_camera(mut commands: Commands) {
    commands.spawn((NavigateCamera, Camera {
        order: 1,
        clear_color: ClearColorConfig::None,
        ..default()
    }));
}
