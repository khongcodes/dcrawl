// THIS FILE SHOULD ONLY CONTAIN code pertaining to transition between inGame Substates - into
// ExploreSubstate and Out of ExploreSubstate; handover of location and orientation data from other states

use bevy::{
    prelude::{
        Component, Query, Entity, With, Commands, Res, ResMut, Assets, Resource,
        AssetServer, Handle, Image,
        StandardMaterial, Plane3d, Cuboid, Vec3, Vec2, Color, Transform, 
        Mesh, Mesh3d, MeshMaterial3d,
        error
    },
    color::palettes::css::RED,
    pbr::PointLight,
    image::CompressedImageFormats,
    core_pipeline::Skybox,
};

use crate::plugins::camera_plugin::NavigateCamera;

// note - my GPU supports BC KTX2 textures - will need to design system for modular textures based
// on user GPU
const CUBEMAPS: &[(&str, CompressedImageFormats)] = &[
    (
        "textures/Ryfjallet_cubemap.png",
        CompressedImageFormats::NONE
    ),
    (
        "textures/Ryfjallet_cubemap_astc4x4.ktx2",
        CompressedImageFormats::ASTC_LDR
    ),
    (
        "textures/Ryfjallet_cubemap_bc7.ktx2",
        CompressedImageFormats::BC
    ),
    (
        "textures/Ryfjallet_cubemap_etc2.ktx2",
        CompressedImageFormats::ETC2
    )
];


#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    index: usize,
    image_handle: Handle<Image>
}


#[derive(Component)]
struct ExploreRootNode;


pub fn setup_exploresubstate(
    camera_query: Query<Entity, With<NavigateCamera>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {

    let nav_cam = match camera_query.single() {
        Ok(c) => c,
        Err(_) => {
            error!("failure getting NavigateCamera entity");
            return;
        }
    };

    // code from all of the below should be transferred to map.rs file

    let skybox_handle: Handle<Image> = asset_server.load(CUBEMAPS[2].0);

    // create flat plane?
    let plane = Plane3d::new(Vec3::Y, Vec2::splat(40.));
    let plane_color = Color::WHITE;

    // Create cuboid
    let cube = Cuboid::new(5., 5., 5.);
    let cube_color: Color = RED.into();
    let cube_position = Transform::from_xyz(0., 5., 0.);

    commands.spawn((
        Mesh3d(meshes.add(plane)),
        MeshMaterial3d(materials.add(plane_color)),
        Transform::from_xyz(0., 0., 0.)
    ));
    commands.spawn((
        Mesh3d(meshes.add(cube)),
        MeshMaterial3d(materials.add(cube_color)),
        cube_position
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..PointLight::default()
        },
        Transform::from_xyz(0., 10., 0.)
    ));
    // Place camera
    commands.entity(nav_cam).insert((
        Transform::from_xyz(0., 5., 20.,).looking_at(cube_position.translation, Vec3::Y),
        Skybox {
            image: skybox_handle.clone(),
            brightness: 1000.0,
            ..Skybox::default()
        }
    ));

}


// move this into exploration_movement
// pub fn explore_movement_controls(
//     keyboard_input: Res<ButtonInput<KeyCode>>,
//     movement_data: ResMut<ExplorationMovementData>
// ) {
//     // CONTROLS SHOULD BE REMAPPABLE
//     const KEY_FORWARD: KeyCode = KeyCode::KeyW;
//     const KEY_BACKWARD: KeyCode = KeyCode::KeyS;
//     const KEY_STRAFELEFT: KeyCode = KeyCode::KeyA;
//     const KEY_STRAFERIGHT: KeyCode = KeyCode::KeyD;
//     const KEY_ROTATELEFT: KeyCode = KeyCode::KeyQ;
//     const KEY_ROTATERIGHT: KeyCode = KeyCode::KeyE;
//
//
//     if keyboard_input.just_pressed(KEY_FORWARD) {
//        enqueue_movement(ExplorationMovements::WalkForward, movement_data);
//     } 
//     else if keyboard_input.just_pressed(KEY_BACKWARD) {
//         enqueue_movement(ExplorationMovements::WalkBackward, movement_data);
//     }
//     else if keyboard_input.just_pressed(KEY_STRAFELEFT) {
//         enqueue_movement(ExplorationMovements::StrafeLeft, movement_data);
//     }
//     else if keyboard_input.just_pressed(KEY_STRAFERIGHT) {
//         enqueue_movement(ExplorationMovements::StrafeRight, movement_data);
//     }
//     else if keyboard_input.just_pressed(KEY_ROTATELEFT) {
//         enqueue_movement(ExplorationMovements::TurnCounterclockw, movement_data);
//     }
//     else if keyboard_input.just_pressed(KEY_ROTATERIGHT) {
//         enqueue_movement(ExplorationMovements::TurnClockw, movement_data);
//     }
// }

// pub fn run_exploresubstate() {
//
// }
//
// pub fn cleanup_exploresubstate() {
//
// }
