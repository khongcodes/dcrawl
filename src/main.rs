use bevy::prelude::*;

mod plugins;
use crate::plugins::manage_state_plugin::ManageStatePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins((
            ManageStatePlugin
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
