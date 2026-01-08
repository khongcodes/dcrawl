/// This plugin organizes player movement in the Explore InGameSubstate, executing commands
/// sequentially by queue and exposing API to enqueue commands.
///
/// Resources in this plugin: ExplorationMovementData, ExplorationLocationData
///
/// Systems in this plugin are called in ExplorePlugin (src/plugins/explore_plugin)
use std::collections::VecDeque;
use std::f32::consts::{ PI, FRAC_PI_2 };
use bevy::prelude::{
    Resource, Res, ResMut, Single, With, Query, Name,
    ButtonInput, KeyCode, Gamepad, GamepadButton,
    Transform, Dir3, Time, Timer, TimerMode, 
    info
};
use bevy::math::{ Quat, Vec3 };

use crate::plugins::{
    camera_plugin::NavigateCamera,
    exposed_config_plugin::ExposedConfig
};


/////////////////////////////////////////
// CONFIGURABLES
const MOVESTEP_DURATION: f32 = 0.3;
const INPUT_BUFFER: f32 = 0.08;

// Temporarily a constant here
// in the future, let user determine grid size (as well as cardinal direction angles)
const MOVESTEP_DISTANCE: f32 = 5.0;
const CARDINAL_DIRECTION_ANGLES: [f32; 4] = [0., -FRAC_PI_2, PI, FRAC_PI_2];

// For the movements beginning with 'Face', users will never directly input them - they are
// reserved for reorienting to cardinal direction grid
#[derive(Debug)]
pub enum ExplorationMovements {
    WalkForward,
    WalkBackward,
    StrafeRight,
    StrafeLeft,
    TurnClockw,
    TurnCounterclockw,
    FaceNorth,
    FaceEast,
    FaceSouth,
    FaceWest
}

#[derive(Debug)]
pub enum CardinalDirection {
    North,  // 0
    East,   // 1
    South,  // 2
    West    // 3
}

pub enum MovementType {
    Rotation,
    Translation
}

fn cardinal_direction_rot_clockwise(current_cardinal_facing: &CardinalDirection) -> (CardinalDirection, f32) {
    match current_cardinal_facing {
        CardinalDirection::North => (CardinalDirection::East, CARDINAL_DIRECTION_ANGLES[1]),
        CardinalDirection::East => (CardinalDirection::South, CARDINAL_DIRECTION_ANGLES[2]),
        CardinalDirection::South => (CardinalDirection::West, CARDINAL_DIRECTION_ANGLES[3]),
        CardinalDirection::West => (CardinalDirection::North, CARDINAL_DIRECTION_ANGLES[0]),
    }
}

fn cardinal_direction_rot_counterclockwise(current_cardinal_facing: &CardinalDirection) -> (CardinalDirection, f32) {
    match current_cardinal_facing {
        CardinalDirection::North => (CardinalDirection::West, CARDINAL_DIRECTION_ANGLES[3]),
        CardinalDirection::East => (CardinalDirection::North, CARDINAL_DIRECTION_ANGLES[0]),
        CardinalDirection::South => (CardinalDirection::East, CARDINAL_DIRECTION_ANGLES[1]),
        CardinalDirection::West => (CardinalDirection::South, CARDINAL_DIRECTION_ANGLES[2]),
    }
}


pub struct CurrentMovementCommand {
    pub movement_type: MovementType,
    pub beginning_translation: Option<Vec3>,
    pub end_translation: Option<Vec3>,
    pub beginning_rotation: Option<Quat>,
    pub end_rotation: Option<Quat>
}

#[derive(Resource)]
pub struct ExplorationMovementData {
    pub current_movement_timer: Option<Timer>,
    pub input_queue_buffer_timer: Option<Timer>,
    pub current_movement_command: Option<CurrentMovementCommand>,
    pub command_queue: VecDeque<ExplorationMovements>,
    pub oriented_to_cardinal_directions: bool,
    pub cardinal_facing: Option<CardinalDirection>,
}

// currently even though ExposedConfig holds gamepad settings, we are only processing keyboard
// inputs
pub fn explore_movement_controls(
    exposed_config: Res<ExposedConfig>,
    // controller_input: Query<(&Name, &Gamepad)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut movement_data: ResMut<ExplorationMovementData>,
    camera_transform_q: Single<&mut Transform, With<NavigateCamera>>,
    time: Res<Time>
) {
    // if between_inputs timer exists and is not finished - tick and return early
    // else - do not enqueue input
    if movement_data.input_queue_buffer_timer.is_some() {
        let timer = movement_data.input_queue_buffer_timer.as_mut().unwrap();
        if !timer.is_finished() {
            timer.tick(time.delta());
            return;
        } else {
            // between_inputs timer is finished:
            // reset between_inputs in movementdata to none so in cases where movement commands were not just
            // pressed, we can skip the above checks
            movement_data.input_queue_buffer_timer = None;
        }
    }

    let k_bindings = &exposed_config.keyboard_bindings.exploration_controls;

    if keyboard_input.just_pressed(k_bindings["Walk Forward"]) {
        // if we are not oriented to cardinal directions - reorient.
        enqueue_movement(ExplorationMovements::WalkForward, movement_data, camera_transform_q);
    } 
    else if keyboard_input.just_pressed(k_bindings["Walk Backward"]) {
        enqueue_movement(ExplorationMovements::WalkBackward, movement_data, camera_transform_q);
    }
    else if keyboard_input.just_pressed(k_bindings["Strafe Left"]) {
        enqueue_movement(ExplorationMovements::StrafeLeft, movement_data, camera_transform_q);
    }
    else if keyboard_input.just_pressed(k_bindings["Strafe Right"]) {
        enqueue_movement(ExplorationMovements::StrafeRight, movement_data, camera_transform_q);
    }
    else if keyboard_input.just_pressed(k_bindings["Turn Left"]) {
        enqueue_movement(ExplorationMovements::TurnCounterclockw, movement_data, camera_transform_q);
    }
    else if keyboard_input.just_pressed(k_bindings["Turn Right"]) {
        enqueue_movement(ExplorationMovements::TurnClockw, movement_data, camera_transform_q);
    }
}


fn calc_closest_cardinal_dir(
    camera_transform_q: Single<&mut Transform, With<NavigateCamera>>,
) 
    -> CardinalDirection 
{
    let cardinal_dirs = CARDINAL_DIRECTION_ANGLES.map(Quat::from_rotation_y);
    let camera_transform_rot = camera_transform_q.into_inner().rotation;

    let rot_dist_to_cardinal_dirs: [f32; 4] = cardinal_dirs.map(|a| a.angle_between(camera_transform_rot));
    let mut closest_cardinal_dir_index = 0;

    for (index, this_rot_dist) in rot_dist_to_cardinal_dirs.iter().enumerate().skip(1) {
        if *this_rot_dist < rot_dist_to_cardinal_dirs[closest_cardinal_dir_index] {
            closest_cardinal_dir_index = index;
        }
    }
    match closest_cardinal_dir_index {
        0 => CardinalDirection::North,
        1 => CardinalDirection::East,
        2 => CardinalDirection::South,
        3 => CardinalDirection::West,
        _ => CardinalDirection::North
    }
}


fn contextualize_current_movement(
    dequeued_movement: ExplorationMovements,
    cam_transform: &Transform,
    cardinal_facing: Option<CardinalDirection>
) -> CurrentMovementCommand {
    let (beginning_rotation, end_rotation): (Option<Quat>, Option<Quat>);
    let (beginning_translation, end_translation): (Option<Vec3>, Option<Vec3>);

    let movement_type = match &dequeued_movement {
        ExplorationMovements::WalkForward | ExplorationMovements::WalkBackward | ExplorationMovements::StrafeLeft | ExplorationMovements::StrafeRight => MovementType::Translation,
        _ => MovementType::Rotation
    };

    if let &MovementType::Translation = &movement_type {
        (beginning_rotation, end_rotation) = (None, None);
        beginning_translation = Some(cam_transform.translation);
        end_translation = Some(MOVESTEP_DISTANCE * match &dequeued_movement {
            ExplorationMovements::WalkForward => -cam_transform.local_z(),
            ExplorationMovements::WalkBackward => cam_transform.local_z(),
            ExplorationMovements::StrafeLeft => -cam_transform.local_x(),
            ExplorationMovements::StrafeRight => cam_transform.local_x(),
            _ => -cam_transform.local_z()
        });
    } else {
        (beginning_translation, end_translation) = (None, None);
        beginning_rotation = Some(cam_transform.rotation);
        end_rotation = match &dequeued_movement {
            ExplorationMovements::FaceNorth => Some(Quat::from_rotation_y(CARDINAL_DIRECTION_ANGLES[0])),
            ExplorationMovements::FaceEast  => Some(Quat::from_rotation_y(CARDINAL_DIRECTION_ANGLES[1])),
            ExplorationMovements::FaceSouth => Some(Quat::from_rotation_y(CARDINAL_DIRECTION_ANGLES[2])),
            ExplorationMovements::FaceWest  => Some(Quat::from_rotation_y(CARDINAL_DIRECTION_ANGLES[3])),
            // BELOW WILL ONLY EVER BE EXECUTED WHILE USER IS ALREADY ORIENTED TO CARDINAL
            // DIRECTIONS
            ExplorationMovements::TurnClockw => Some(Quat::from_rotation_y(cardinal_direction_rot_clockwise(&cardinal_facing.unwrap()).1)),
            ExplorationMovements::TurnCounterclockw => Some(Quat::from_rotation_y(cardinal_direction_rot_counterclockwise(&cardinal_facing.unwrap()).1)),
            _ => Some(Quat::from_rotation_y(CARDINAL_DIRECTION_ANGLES[0])),
        };
    }
    CurrentMovementCommand {
        movement_type,
        beginning_translation, end_translation,
        beginning_rotation, end_rotation
    }
}


/// This system should be called when a user has just pressed a button warranting a movement
/// Set up buffer to prevent multiple inputs from being registered from single button input
pub fn enqueue_movement(
    movement: ExplorationMovements,
    mut movement_data: ResMut<ExplorationMovementData>,
    camera_transform_q: Single<&mut Transform, With<NavigateCamera>>,
) {
    // if a movement is enqueued but we are not in cardinal - inject new movement to reorient
    // to cardinal, then enqueue the movement
    if !movement_data.oriented_to_cardinal_directions {
        let closest_cardinal = calc_closest_cardinal_dir(camera_transform_q);
        match closest_cardinal {
            CardinalDirection::North => { movement_data.command_queue.push_back(ExplorationMovements::FaceNorth); },
            CardinalDirection::East => { movement_data.command_queue.push_back(ExplorationMovements::FaceEast); },
            CardinalDirection::South => { movement_data.command_queue.push_back(ExplorationMovements::FaceSouth); },
            CardinalDirection::West => { movement_data.command_queue.push_back(ExplorationMovements::FaceWest); },
        }
        info!("Enqueueing {:?}", closest_cardinal);
        movement_data.oriented_to_cardinal_directions = true;
    }

    info!("Enqueueing {:?}", movement);
    movement_data.command_queue.push_back(movement);
    movement_data.input_queue_buffer_timer = Some(Timer::from_seconds(INPUT_BUFFER, TimerMode::Once));
}


/// When InGameSubstate shifts out of exploration (for example into combat) movement queue should
/// clear
pub fn clear_movement_queue(mut movement_data: ResMut<ExplorationMovementData>) {
    movement_data.current_movement_timer = None;
    movement_data.command_queue.clear();
}


/// If command_queue is empty, do nothing
/// Else (command queue has a command in it)
/// If in_progress is None - create a new timer with Duration::from_seconds(MOVESTEP_DURATION)
/// Else tick in_progress
///     Execute current command
/// If timer is empty, set movementdata.in_progress to None
pub fn execute_movement_queue(
    camera_transform_q: Single<&mut Transform, With<NavigateCamera>>,
    mut movement_data: ResMut<ExplorationMovementData>,
    time: Res<Time>,
) {
    // FIRST: if there is no current_movement_command
    //      If command_queue.front() is none
    //          return early
    //      Else ()
    //          calculate a new current_movement_command    ABSTRACT THIS TO NEW METHOD
    //          start current_movement_timer for it
    //
    //
    // Else: there is a current_movement_command;
    //      there should be a timer already
    //      interpolation based on it and the timer
    //
    //
    // If current_movement_timer just_finished()
    //      if command_queue.front() is Some
    //          calculate a new current_movement_command    abstract to new method
    //          start current_movement_timer for it
    //      else
    //          set current_movement to none
    //          set current_movement_Timer to none
    

    // UPDATE THIS CODE: USE "CURRENT MOVEMENT" system; popping front of the command queue off and
    // putting it in ExplorationMovementData::current_movement_command after prepping it with 
    // contextualize_current_movement

    
    if movement_data.command_queue.front().is_none() { return; }
   
    // If we've made it this far - there IS movement enqueued.
    // If there is not a timer, set the timer.
    if movement_data.current_movement_timer.is_none() {
        movement_data.current_movement_timer = Some(Timer::from_seconds(MOVESTEP_DURATION, TimerMode::Once));
    } else {
        // IF we've made it this far, there is an item in command queue and there is a
        // timer- it must be manually ticked
        movement_data.current_movement_timer.as_mut().unwrap().tick(time.delta());
    }

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
            let (cd, cd_angle) = cardinal_direction_rot_clockwise(movement_data.cardinal_facing.as_ref().unwrap());
            info!("rotating to {:?}", cd);
            movement_data.cardinal_facing = Some(cd);
            rotated = cd_angle;
        },
        ExplorationMovements::TurnCounterclockw => {
            let (cd, cd_angle) = cardinal_direction_rot_counterclockwise(movement_data.cardinal_facing.as_ref().unwrap());
            info!("rotating to {:?}", cd);
            movement_data.cardinal_facing = Some(cd);
            rotated = cd_angle;
        },
        ExplorationMovements::FaceNorth => { 
            rotated = CARDINAL_DIRECTION_ANGLES[0];
            movement_data.cardinal_facing = Some(CardinalDirection::North);
        }
        ExplorationMovements::FaceEast => { 
            rotated = CARDINAL_DIRECTION_ANGLES[1]; 
            movement_data.cardinal_facing = Some(CardinalDirection::East);
        }
        ExplorationMovements::FaceSouth => { 
            rotated = CARDINAL_DIRECTION_ANGLES[2]; 
            movement_data.cardinal_facing = Some(CardinalDirection::South);
        }
        ExplorationMovements::FaceWest => { 
            rotated = CARDINAL_DIRECTION_ANGLES[3]; 
            movement_data.cardinal_facing = Some(CardinalDirection::West);
        }
    }
    
    if translated {
        camera_transform.translation += direction * MOVESTEP_DISTANCE * time.delta_secs();
    } else if rotated != 0. {
        // rotate_y needs to take in the delta of rotation in this tick, rather than the whole
        // degree of rotation; thus we need to calculate the speed required to complete rotation by
        // <rotated> over the course of <MOVESTEP_DURATION>.
        let fraction = movement_data.current_movement_timer.as_ref().unwrap().fraction();
        // let speed = 1.0 / MOVESTEP_DURATION;
        // with rotate_local_y, I keep consistently getting greater than 90 degree rotation. I am
        // going to see if working with Quaternions gets me better results.
        // Otherwise - I wonder if I have to figure out if I will have more success with
        // local/homogeneous coords.
        // camera_transform.rotate_local_y(rotated * time.delta_secs() * speed);

        let initial_rot = camera_transform.rotation;
        let new_rot = initial_rot.lerp(Quat::from_rotation_y(rotated), fraction);
        camera_transform.rotation = new_rot;
    }

    if movement_data.current_movement_timer.as_ref().unwrap().just_finished() {
        info!("movement just finished");
        movement_data.command_queue.pop_front();
        if !movement_data.command_queue.is_empty() {
            // movement_data.in_progress.as_mut().unwrap().set_duration(Duration::from_secs_f32(MOVESTEP_DURATION));
            movement_data.current_movement_timer = Some(Timer::from_seconds(MOVESTEP_DURATION, TimerMode::Once));

        } else {
            movement_data.current_movement_timer = None;
        }
    }

}

