// This file contains code with Bevy's Reflect trait. This can be read about here:
// https://taintedcoders.com/bevy/reflection

use std::{ 
    fs, fs::OpenOptions, 
    collections::HashMap,
    io::Write,
    any::TypeId,
};
use serde::{ de::DeserializeSeed };
use bevy::{
    reflect::{ FromReflect, PartialReflect, TypeRegistry,
        serde::{ TypedReflectDeserializer, TypedReflectSerializer }
    },
    prelude::{ 
        Commands, Resource, Res, Startup, App, Plugin, Reflect,
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

// Resources with the Reflect trait derived will have their type registered in the world's
// AppTypeRegistry resource, enabling automatic extraction when we serialize.
// Fields can then be accessed dynamically from the string value of the property.
#[derive(Reflect, Debug, Resource)]
pub struct ExposedConfig {
    pub keyboard_bindings: KeyboardBindings,
    pub controller_bindings: ControllerBindings
}

#[derive(Reflect, Debug)]
pub struct KeyboardBindings { 
    pub exploration_controls: HashMap<String, KeyCode>
}
#[derive(Reflect, Debug)]
pub struct ControllerBindings {
    pub exploration_controls: HashMap<String, GamepadButton>
}

impl ExposedConfig {
    // fn pack(&self) -> SerializedExposedConfig {
    //     SerializedExposedConfig {
    //         keyboard_bindings: SerializedBindings::from_keyboard(&self.keyboard_bindings),
    //         controller_bindings: SerializedBindings::from_controller(&self.controller_bindings),
    //     }
    // }
}

fn load_exposed_config_file(mut commands: Commands) {
    let config_ron_str = fs::read_to_string(CONFIG_FILEPATH).unwrap_or_else(|err|panic!("{}", err));
    
    // make sure registry has the type
    let mut type_registry = TypeRegistry::default();
    type_registry.register::<ExposedConfig>();

    // deserialize the RON string
    let registration = type_registry.get(TypeId::of::<ExposedConfig>()).unwrap();
    let mut deserializer = ron::de::Deserializer::from_str(&config_ron_str).unwrap_or_else(|e|panic!("{}",e));
    let reflect_deserializer = TypedReflectDeserializer::new(registration, &type_registry);
    let config_reflect_box: Box<dyn PartialReflect> = reflect_deserializer.deserialize(&mut deserializer).unwrap();
    let config = ExposedConfig::from_reflect(&*config_reflect_box).unwrap();
    commands.insert_resource(config);
}

pub fn update_exposed_config_file(config: Res<ExposedConfig>) -> Result<String, String>{
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CONFIG_FILEPATH).unwrap_or_else(|err| panic!("{}", err));
    
    // let type_registry = type_registry.read();
    let type_registry = TypeRegistry::default();
    let reflect_serializer = TypedReflectSerializer::new(config.as_partial_reflect(), &type_registry);
    let new_config = ron::ser::to_string_pretty(
        &reflect_serializer,
        ron::ser::PrettyConfig::default()
    ).unwrap_or_else(|err| panic!("{}", err));

    if file.write_all(new_config.as_bytes()).is_ok() {
        Ok(String::from("file write successful"))
    } else {
        Err(String::from("error updating CONFIG_FILEPATH"))
    }
}

