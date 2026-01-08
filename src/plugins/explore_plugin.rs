// Handle data setup and run schedules once inside of Explore substate

use std::collections::VecDeque;
use bevy::prelude::{ 
    App, Plugin, Update, OnExit, FixedUpdate,
    in_state,
    IntoScheduleConfigs
};

pub mod movement;

use crate::plugins::{
    explore_plugin:: {
        movement::{ 
            ExplorationMovementData,
            explore_movement_controls, execute_movement_queue, clear_movement_queue
        },
    }, 
    manage_state_plugin::InGameSubstate
};


pub struct ExplorePlugin;

// all systems only run if in_state(InGameSubstate::Explore)
impl Plugin for ExplorePlugin {
    fn build(&self, app: &mut App) {
        
        app.insert_resource(
            ExplorationMovementData {
                current_movement_timer: None,
                input_queue_buffer_timer: None,
                current_movement_command: None,
                command_queue: VecDeque::new(),
                oriented_to_cardinal_directions: false,
                cardinal_facing: None
            }
        );

        app.add_systems(
            FixedUpdate,
            (
                execute_movement_queue,
                explore_movement_controls.before(execute_movement_queue),
            )
            .distributive_run_if(in_state(InGameSubstate::Explore))
        );

        app.add_systems(OnExit(InGameSubstate::Explore),
            clear_movement_queue
        );

    }

} 

