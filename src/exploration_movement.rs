/// This plugin organizes player movement in the Explore InGameSubstate, executing commands
/// sequentially by queue and exposing API to enqueue commands.
///
/// This plugin should be added when game is switched to Explore InGameSubstate
use std::collections::VecDeque;
use std::f32::consts::PI;
use bevy::prelude::{
    App, Plugin, Resource, Res, ResMut, Time, Timer, TimerMode, Single, With, Transform, Dir3, Update, IntoScheduleConfigs, in_state, OnExit
};
use std::time::Duration;

use crate::plugins::camera_plugin::NavigateCamera;
use crate::plugins::manage_state_plugin::InGameSubstate;


/////////////////////////////////////////
// CONFIGURABLES
const MOVESTEP_DURATION: f32 = 0.5;
// This is a temporary constant - this should be derived, in the future, from map tile size
const MOVESTEP_DISTANCE: f32 = 5.0;


pub enum ExplorationMovements {
    WalkForward,
    WalkBackward,
    StrafeRight,
    StrafeLeft,
    TurnClockw,
    TurnCounterclockw
}


pub struct ExplorationMovementPlugin;

impl Plugin for ExplorationMovementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(
            ExplorationMovementData {
                in_progress: None,
                command_queue: VecDeque::new()
            }
        );
        app.add_systems(
            Update, 
            execute_movement_queue.run_if(in_state(InGameSubstate::Explore))
        );
        app.add_systems(OnExit(InGameSubstate::Explore), clear_movement_queue);
    }
}


#[derive(Resource)]
pub struct ExplorationMovementData {
    in_progress: Option<Timer>,
    command_queue: VecDeque<ExplorationMovements>,
}


/// This system should be called when a user has just pressed a button warranting a movement
/// If we change this to button pressed instead of JUSTpressed, we should create some sort of
/// threshold for delaying registered enqueueing.
pub fn enqueue_movement(
    movement: ExplorationMovements,
    mut movement_data: ResMut<ExplorationMovementData>
) {
    movement_data.command_queue.push_back(movement);
    if movement_data.in_progress.is_none() {
        movement_data.in_progress = Some(Timer::from_seconds(MOVESTEP_DURATION, TimerMode::Once));
    }
}


/// When InGameSubstate shifts out of exploration (for example into combat) movement queue should
/// clear
fn clear_movement_queue(mut movement_data: ResMut<ExplorationMovementData>) {
    movement_data.in_progress = None;
    movement_data.command_queue.clear();
}


/// If command_queue is empty, do nothing
/// Else (command queue has a command in it)
/// If in_progress is None - create a new timer with Duration::from_seconds(MOVESTEP_DURATION)
/// Else tick in_progress
///     Execute current command
/// If timer is empty, set movementdata.in_progress to None
fn execute_movement_queue(
    camera_transform_q: Single<&mut Transform, With<NavigateCamera>>,
    mut movement_data: ResMut<ExplorationMovementData>,
    time: Res<Time>,
) {
    if movement_data.command_queue.front().is_none() { return; }
    
    // IF COMMAND QUEUE HAS A VALUE, THERE SHOULD BE A TIMER - MUST BE MANUALLY TICKED
    movement_data.in_progress.as_mut().unwrap().tick(time.delta());

    let mut camera_transform = camera_transform_q.into_inner();
    let mut direction = Dir3::from_xyz(10., 10., 10.).unwrap();
    let mut translated = false;
    let mut rotated = 0.;

    let current_command = movement_data.command_queue.front().unwrap();
    match current_command {
        ExplorationMovements::WalkForward => {
            direction = -camera_transform.local_z();
            translated = true;
        },
        ExplorationMovements::WalkBackward => {
            direction = camera_transform.local_z();
            translated = true;
        },
        ExplorationMovements::StrafeLeft => {
            direction = -camera_transform.local_x();
            translated = true;
        },
        ExplorationMovements::StrafeRight => {
            direction = camera_transform.local_x();
            translated = true;
        },
        ExplorationMovements::TurnClockw => {
            rotated = PI / 2.;
        },
        ExplorationMovements::TurnCounterclockw => {
            rotated = -PI / 2.;
        },
    }
    
    if translated {
        camera_transform.translation += direction * MOVESTEP_DISTANCE * time.delta_secs();
    } else if rotated != 0. {
        camera_transform.rotate_y(rotated);
    }

    if rotated != 0. || movement_data.in_progress.as_ref().unwrap().just_finished() {
        movement_data.command_queue.pop_front();
        if !movement_data.command_queue.is_empty() {
            movement_data.in_progress.as_mut().unwrap().set_duration(Duration::from_secs_f32(MOVESTEP_DURATION));
        } else {
            movement_data.in_progress = None;
        }
    }

}

