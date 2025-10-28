use bevy::{
    prelude::*,
    render::renderer::RenderDevice
};

mod plugins;

use crate::plugins::{
    manage_state_plugin::{ ManageStatePlugin, intro_screen_plugin::setup_intro_screen },
    camera_plugin::{ CameraPlugin, setup_ui_camera },
    exposed_config_plugin::ExposedConfigPlugin,
};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            CameraPlugin,
            ExposedConfigPlugin
        ))
        .add_plugins(ManageStatePlugin { start_ingame: true })
        .add_systems(Startup, (
            // log_render_device_features,

            setup_ui_camera, 
            setup_intro_screen
        ).chain())
        .run();
}

fn log_render_device_features(_render_device: Res<RenderDevice>) {
    // info!("{:?}", render_device.features());
}

