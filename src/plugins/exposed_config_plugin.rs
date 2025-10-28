use std::{ 
    fs, fs::OpenOptions, 
    collections::HashMap,
    io::Write
};
use serde::{ Serialize, Deserialize };
use bevy::{
    reflect::{ Enum, FromReflect, PartialReflect },
    prelude::{ 
        Commands, Resource, Res, Startup, App, Plugin,
        GamepadButton, KeyCode, error
    },
};

// CONFIGURABLES
const CONFIG_FILEPATH: &str = "config/game_config.ron";

pub struct ExposedConfigPlugin;

impl Plugin for ExposedConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_exposed_config_file);
    }
}


#[derive(Debug, Deserialize, Serialize)]
struct SerializedExposedConfig {
    keyboard_bindings: SerializedBindings,
    controller_bindings: SerializedBindings
}

#[derive(Debug, Deserialize, Serialize)]
struct SerializedBindings {
    exploration_controls: HashMap<String, String>,
}


#[derive(Debug, Resource)]
pub struct ExposedConfig {
    pub keyboard_bindings: KeyboardBindings,
    pub controller_bindings: ControllerBindings
}

#[derive(Debug)]
pub struct KeyboardBindings { 
    pub exploration_controls: HashMap<String, KeyCode>
}
#[derive(Debug)]
pub struct ControllerBindings {
    pub exploration_controls: HashMap<String, GamepadButton>
}

impl SerializedBindings{
    fn unpack_keyboard(self) -> KeyboardBindings {
        let mut map: HashMap<String, KeyCode> = HashMap::new();
        for (action_name, keycode_str) in self.exploration_controls {
            // error!("{}, {}", action_name, keycode_str);
            map.insert(action_name, KeyCode::from_reflect(keycode_str.as_partial_reflect()).unwrap());
        }
        KeyboardBindings { exploration_controls: map }
    }
    fn unpack_controller(self) -> ControllerBindings {
        let mut map: HashMap<String, GamepadButton> = HashMap::new();
        for (action_name, button_str) in self.exploration_controls {
            map.insert(action_name, GamepadButton::from_reflect(&button_str).unwrap());
        }
        ControllerBindings { exploration_controls: map }
    }
    fn from_keyboard(k_controls: &KeyboardBindings) -> SerializedBindings {
        let mut exploration_controls: HashMap<String,String> = HashMap::new();
        for (action_name, keycode) in &k_controls.exploration_controls {
            exploration_controls.insert(String::from(action_name), keycode.variant_path());
        }
        SerializedBindings { exploration_controls }
    }
    fn from_controller(c_controls: &ControllerBindings) -> SerializedBindings {
        let mut exploration_controls: HashMap<String,String> = HashMap::new();
        for (action_name, keycode) in &c_controls.exploration_controls {
            exploration_controls.insert(String::from(action_name), keycode.variant_path());
        }
        SerializedBindings { exploration_controls }
    }
}

impl SerializedExposedConfig {
    fn unpack(self) -> ExposedConfig {
        ExposedConfig {
            keyboard_bindings: self.keyboard_bindings.unpack_keyboard(),
            controller_bindings: self.controller_bindings.unpack_controller()
        }
    }
}

impl ExposedConfig {
    fn pack(&self) -> SerializedExposedConfig {
        SerializedExposedConfig {
            keyboard_bindings: SerializedBindings::from_keyboard(&self.keyboard_bindings),
            controller_bindings: SerializedBindings::from_controller(&self.controller_bindings),
        }
    }
}

fn load_exposed_config_file(mut commands: Commands) {
    let config_str = fs::read_to_string(CONFIG_FILEPATH).unwrap_or_else(|err|panic!("{}", err));
    let ser_config: SerializedExposedConfig = ron::from_str(&config_str).unwrap_or_else(|err|panic!("{}",err));
    let config = ser_config.unpack();
    commands.insert_resource(config);
}

pub fn update_exposed_config_file(config: Res<ExposedConfig>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CONFIG_FILEPATH).unwrap_or_else(|err| panic!("{}", err));
    
    let new_config = ron::ser::to_string_pretty(
        &config.as_ref().pack(), 
        ron::ser::PrettyConfig::default()
    ).unwrap_or_else(|err| panic!("{}", err));

    file.write_all(new_config.as_bytes());
}

