use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::{ecs::system::Commands, sprite::SpriteBundle, transform::components::Transform};
use rand::{thread_rng, Rng};

use crate::structs::*;
pub const PIPE_WIDTH: f32 = 30.0; // Width of the pipe.
pub const PIPE_HEIGHT: f32 = 100.0; // Example height of the pipe.

pub fn spawn_ledges(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();
    let _gap = 150.0; // Gap size between top and bottom pipes
    let count = 4;
    for i in 0..count {
        spawn_ledge(
            &mut commands,
            &window_query,
            &asset_server,
            (i as f32) * 150.0 + 100.0,
        )
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("ball_blue_large.png"),
            ..default()
        },
        Player {
            is_attatched_to_ledge: false,
            ledge_attatched_to: None,
            ledge_x: 0.0,
            closest_distance: f32::MAX,
            ledge_y: 0.0,
            velocity: Vec3::ZERO,
            swinging: false,
            angular_velocity: 0.0,
        },
    ));
}

pub fn spawn_ledge(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    x: f32,
) {
    let window = window_query.get_single().unwrap();
    let ledge_position = Vec3::new(
        window.width() / 2.0 + x * 2.0,
        window.height() - PIPE_HEIGHT - 100.0,
        0.0,
    );

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(ledge_position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(100.0, 100.0)),
                ..default()
            },
            texture: asset_server.load("Smoking_pipe.png"),
            ..default()
        },
        Ledge {
            distance_from_player: 0.0, // This will need to be updated dynamically
            id: x as u32,
            position: ledge_position,
        },
    ));
}
pub const BORDER_THICKNESS: f32 = 10.0; // Thickness of the border walls
pub fn spawn_pipes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = thread_rng();
    let mut x_position = 150.0; // Initial offset

    // Loop to spawn multiple pipes with random gaps
    for _ in 0..5 {
        let gap = rng.gen_range(150.0..300.0); // Random gap between pipes
        x_position += gap; // Increment x position by gap

        // Top pipe
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    x_position,
                    window.height() - PIPE_HEIGHT,
                    0.0,
                )),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PIPE_WIDTH * 2.0, PIPE_HEIGHT)),
                    ..default()
                },
                texture: asset_server.load("pipe.png"),
                ..default()
            },
            Pipe {
                top: true,
                position: Vec3::new(x_position, window.height() - PIPE_HEIGHT, 0.0),
            },
        ));

        // Bottom pipe
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(x_position, 0.0, 0.0)),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(PIPE_WIDTH * 2.0, PIPE_HEIGHT)),
                    ..default()
                },
                texture: asset_server.load("pipe.png"),
                ..default()
            },
            Pipe {
                top: false,
                position: Vec3::new(x_position, 0.0, 0.0),
            },
        ));
    }
}

pub fn spawn_borders(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    // Top border
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height(), 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), BORDER_THICKNESS)),
            color: Color::BLACK,
            ..default()
        },
        ..default()
    },));

    // Bottom border
    commands.spawn((SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, 0.0, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(window.width(), BORDER_THICKNESS)),
            color: Color::BLACK,
            ..default()
        },
        ..default()
    },));
}

pub fn spawn_pipe(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    x: f32,
) {
    let window = window_query.get_single().unwrap();

    // Top pipe position
    let top_pipe_position = Vec3::new(x + window.width() / 2.0, window.height() - PIPE_HEIGHT, 0.0);
    // Bottom pipe position
    let bottom_pipe_position = Vec3::new(x + window.width() / 2.0, PIPE_HEIGHT, 0.0);

    // Top pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(top_pipe_position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIPE_WIDTH * 2.0, window.height() - 150.0)),
                ..default()
            },
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        Pipe {
            top: true,
            position: top_pipe_position,
        },
    ));

    // Bottom pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(bottom_pipe_position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(
                    PIPE_WIDTH * 2.0,
                    window.height() - PIPE_HEIGHT * 2.0 - 150.0,
                )),
                ..default()
            },
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        Pipe {
            top: false,
            position: bottom_pipe_position,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_rope(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    ledge_query: Query<&Transform, With<Ledge>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    if let Ok(player_transform) = player_query.get_single() {
        if let Some(ledge_transform) = ledge_query.iter().next() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform::from_xyz(
                        window.width() / 2.0,
                        window.height() / 2.0,
                        -1.0,
                    ),
                    sprite: Sprite {
                        color: Color::rgb(1.0, 0.01, 0.0),         // Corrected color values
                        custom_size: Some(Vec2::new(20.0, 100.0)), // Example size, adjust as needed
                        ..default()
                    },
                    visibility: Visibility::Hidden,
                    texture: asset_server.load("pipe.png"),
                    ..default()
                },
                Rope {
                    start: player_transform.translation,
                    end: ledge_transform.translation,
                },
            ));
        }
    }
}
