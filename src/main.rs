use bevy::app::{App, Startup, Update};
use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Without};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::system::*;
use bevy::input::keyboard::KeyCode;
use bevy::input::ButtonInput;
use bevy::math::{Vec2, Vec3};
use bevy::time::Time;
use bevy::transform::components::Transform;
// For easier 2D vector operations

use bevy::DefaultPlugins;

use bevy_xpbd_2d::prelude::*;

mod player_movement;
mod startup;
mod structs;

use startup::*;
use structs::*;

pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
const GRAVITY: f32 = -50.8;
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
                apply_gravity_and_motion, // Ensure this is part of your update loop
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

pub fn apply_gravity_and_motion(
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in players.iter_mut() {
        // Apply gravity to vertical velocity if the player is not attached to a ledge
        if !player.is_attatched_to_ledge {
            player.velocity.y += GRAVITY * time.delta_seconds();
        }

        // Update the player's position based on the current velocity
        transform.translation += player.velocity * time.delta_seconds();

        // Implement damping if swinging to reduce the velocity over time
        if player.swinging {
            player.velocity *= 0.99; // Damping factor to gradually reduce the swing
        }
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
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in players.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) && !player.swinging {
            if !player.is_attatched_to_ledge {
                player.is_attatched_to_ledge = true;
                player.swinging = true;
                player.velocity += Vec3::new(50.0, 0.0, 0.0); // Initial push for swinging
            }
        } else if !keyboard_input.pressed(KeyCode::Space) {
            // Stop swinging when space is not pressed
            player.swinging = false;
            player.is_attatched_to_ledge = false;
            player.ledge_attatched_to = None;
            // Only reset x-velocity to keep gravity effect intact
        }
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
            let distance = player_transform
                .translation
                .distance(ledge_transform.translation);
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
    mut players: Query<(&mut Transform, &mut Player), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in players.iter_mut() {
        if player.swinging && player.is_attatched_to_ledge {
            let target_position = Vec3::new(player.ledge_x, player.ledge_y, 0.0);
            let direction_to_target = (target_position - transform.translation).normalize();
            let swing_speed = 100.0; // Speed of the swing can be adjusted

            // Calculate a simple harmonic motion around the ledge
            player.velocity = direction_to_target
                .cross(Vec3::new(0.0, 0.0, 1.0))
                .normalize()
                * swing_speed;

            // Apply swinging motion
            transform.translation += player.velocity * time.delta_seconds();

            // Reduce the swing over time
            player.velocity *= 0.99;
        }
    }
}
