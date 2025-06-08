use bevy::{ prelude::{ Component, Camera3d, Camera2d, App, Startup, Plugin, Camera, ClearColorConfig, Commands, default }, 
    render::view::RenderLayers 
};

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

// Anything with this component will only be rendered on the Camera also assigned to this component
pub const UI_RL: RenderLayers = RenderLayers::layer(1);

// order: this camera should render its results "on top of" results of cameras with lower order

fn setup_ui_camera(mut commands: Commands) {
    commands.spawn((UiCamera, UI_RL, Camera {
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
