use bevy::prelude::*;

mod plugins;
use crate::plugins::{
    game_runtime_plugin::GameRuntimePlugin, manage_state_plugin::ManageStatePlugin,
};
#[derive(Component)]
pub struct UiCameraMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((ManageStatePlugin, GameRuntimePlugin))
        .run();
}

