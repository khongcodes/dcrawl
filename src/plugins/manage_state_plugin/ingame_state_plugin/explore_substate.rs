// THIS FILE SHOULD ONLY CONTAIN code pertaining to transition between inGame Substates

use bevy::prelude::{ 
    Component, Query, Entity, With, Commands, Res, ResMut, Assets,
    Mesh, StandardMaterial, Plane3d, Sphere, Vec3, Vec2, Color, Transform, Mesh3d, MeshMaterial3d,
    ButtonInput, KeyCode,
    error
};
use bevy::color::palettes::css::RED;
use bevy::pbr::PointLight;

use crate::plugins::camera_plugin::NavigateCamera;
use crate::plugins::exploration_movement::{ 
    enqueue_movement, ExplorationMovements, ExplorationMovementData 
};


#[derive(Component)]
struct ExploreRootNode;

pub fn setup_exploresubstate(
    camera_query: Query<Entity, With<NavigateCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    let nav_cam = match camera_query.single() {
        Ok(c) => c,
        Err(_) => {
            error!("failure getting NavigateCamera entity");
            return;
        }
    };

    // create flat plane?
    let plane = Plane3d::new(Vec3::Y, Vec2::splat(40.));
    let plane_color = Color::WHITE;

    // Create sphere
    let sphere = Sphere::new(5.);
    let sphere_color: Color = RED.into();
    let sphere_position = Transform::from_xyz(0., 5., 0.);

    commands.spawn((
        Mesh3d(meshes.add(plane)),
        MeshMaterial3d(materials.add(plane_color)),
        Transform::from_xyz(0., 0., 0.)
    ));
    commands.spawn((
        Mesh3d(meshes.add(sphere)),
        MeshMaterial3d(materials.add(sphere_color)),
        sphere_position
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..PointLight::default()
        },
        Transform::from_xyz(0., 10., 0.)
    ));
    // Place camera
    commands.entity(nav_cam).insert(
        Transform::from_xyz(0., 5., 20.,).looking_at(sphere_position.translation, Vec3::Y)
    );

}


// move this into exploration_movement
pub fn explore_movement_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    movement_data: ResMut<ExplorationMovementData>
) {
    // CONTROLS SHOULD BE REMAPPABLE
    const KEY_FORWARD: KeyCode = KeyCode::KeyW;
    const KEY_BACKWARD: KeyCode = KeyCode::KeyS;
    const KEY_STRAFELEFT: KeyCode = KeyCode::KeyA;
    const KEY_STRAFERIGHT: KeyCode = KeyCode::KeyD;
    const KEY_ROTATELEFT: KeyCode = KeyCode::KeyQ;
    const KEY_ROTATERIGHT: KeyCode = KeyCode::KeyE;


    if keyboard_input.just_pressed(KEY_FORWARD) {
       enqueue_movement(ExplorationMovements::WalkForward, movement_data);
    } 
    else if keyboard_input.just_pressed(KEY_BACKWARD) {
        enqueue_movement(ExplorationMovements::WalkBackward, movement_data);
    }
    else if keyboard_input.just_pressed(KEY_STRAFELEFT) {
        enqueue_movement(ExplorationMovements::StrafeLeft, movement_data);
    }
    else if keyboard_input.just_pressed(KEY_STRAFERIGHT) {
        enqueue_movement(ExplorationMovements::StrafeRight, movement_data);
    }
    else if keyboard_input.just_pressed(KEY_ROTATELEFT) {
        enqueue_movement(ExplorationMovements::TurnCounterclockw, movement_data);
    }
    else if keyboard_input.just_pressed(KEY_ROTATERIGHT) {
        enqueue_movement(ExplorationMovements::TurnClockw, movement_data);
    }
}

// pub fn run_exploresubstate() {
//
// }
//
// pub fn cleanup_exploresubstate() {
//
// }
