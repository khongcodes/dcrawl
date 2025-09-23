use bevy::prelude::*;

mod plugins;
mod exploration_movement;
use crate::plugins::{
    manage_state_plugin::{ ManageStatePlugin, intro_screen_plugin::setup_intro_screen },
    camera_plugin::{ CameraPlugin, setup_ui_camera },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(ManageStatePlugin { start_ingame: false })
        .add_systems(Startup, (setup_ui_camera, setup_intro_screen).chain())
        .run();
}

