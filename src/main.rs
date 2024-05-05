use bevy::app::{App, Startup, Update};
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Without};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::*;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3}; use bevy::time::Time;
use bevy::transform::components::Transform;
// For easier 2D vector operations

use bevy::DefaultPlugins;

use bevy_xpbd_2d::prelude::*;

mod structs;
mod startup;
mod player_movement;

use startup::*;
use structs::*;

pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Gravity(Vec2::NEG_Y * 100.0))
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_pipes)
        .add_systems(Startup, spawn_ledges) // Spawn pipes every 2 seconds
        .add_systems(
            Update,
            (
                player_movement_detection,
                player_ledge_edging,
                sync_player_camera,
                update_ledges_information,
            )
                .chain(),
        )
        .add_systems(Update, player_movement_moving)
        .run();
}



pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Camera2d, &mut Transform), Without<Player>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    let Ok((mut _camera, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    let delta = player.translation - camera_transform.translation;

    if delta != Vec3::ZERO {
        camera_transform.translation += delta;
    }
}
pub fn update_ledges_information(
    player: Query<&Transform, With<Player>>,
    mut ledges: Query<(&Transform, &mut Ledge), With<Ledge>>,
) {
    if let Ok(player_transform) = player.get_single() {
        for (ledge_transform, mut ledge) in ledges.iter_mut() {
            ledge.distance_from_player = player_transform
                .translation
                .distance(ledge_transform.translation);
        }
    }
}



pub fn player_movement_detection(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(Query<(&mut Transform, &mut Player), With<Player>>,)>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut player)) = set.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;
        direction += Vec3::new(0.0, -1.0, 0.0);
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.is_attatched_to_ledge = true;
        } else {
            player.is_attatched_to_ledge = false;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_ledge_edging(
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    ledges: Query<(Entity, &Ledge, &Transform), With<Ledge>>,
    _time: Res<Time>,
) {
    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        let mut closest_ledge_entity: Option<Entity> = None;
        let mut closest_distance = f32::MAX;
        let mut ledge_position = Vec3::ZERO;

        // Calculate distance based on player's position
        for (ledge_entity, _ledge, ledge_transform) in ledges.iter() {
            let distance = player_transform.translation.distance(ledge_transform.translation);
            if distance < closest_distance {
                println!("Distance: {}", distance);
                closest_distance = distance;
                closest_ledge_entity = Some(ledge_entity);
                ledge_position = ledge_transform.translation;
            }
        }

        // If a closer ledge is found, update the player's attached ledge info
        if !player.is_attatched_to_ledge {
            if let Some(ledge_entity) = closest_ledge_entity {
                player.ledge_attatched_to = Some(ledge_entity);
                player.ledge_x = ledge_position.x;
                player.ledge_y = ledge_position.y;
                player.closest_distance = closest_distance;
            }
        }
    }
}
pub fn player_movement_moving(
    mut set: ParamSet<(Query<(&mut Transform, &mut Player, &mut BezierState), With<Player>>,)>,
    time: Res<Time>,
) {
    let mut binding = set.p0();
    let (mut player_transform, mut player, mut bezier_state) = binding.get_single_mut().unwrap();

    if player.is_attatched_to_ledge {
        if bezier_state.t < 1.0 {
            bezier_state.t += time.delta_seconds() * 0.5; // Control the speed of the transition
            bezier_state.t = bezier_state.t.min(1.0); // Ensure t does not exceed 1

            let new_position = bezier_state.bezier_point();
            player_transform.translation = new_position;
        } else {
            // Update target if needed or finalize movement
            player.is_attatched_to_ledge = false; // Optionally detach player once movement is complete
            player.ledge_attatched_to = None; // Clear the attached ledge entity reference
        }
    } else {
        // Initialize Bezier state when player attaches to a ledge
        let current_position = player_transform.translation;
        let target_position = Vec3::new(player.ledge_x + 30.0, player.ledge_y, 0.0); // Adjusting according to new position
        player.is_attatched_to_ledge = true;
        *bezier_state = BezierState::new(current_position, target_position);
    }
}
