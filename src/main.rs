use bevy::prelude::*;

mod plugins;
use crate::plugins::{
    game_runtime_plugin::GameRuntimePlugin, 
    manage_state_plugin::ManageStatePlugin,
    camera_plugin::CameraPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, ManageStatePlugin, GameRuntimePlugin))
        .run();
}

