use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0; // This is the player sprite size.
pub const PIPE_SPEED: f32 = 100.0;
pub const PIPE_WIDTH: f32 = 30.0; // Width of the pipe.
pub const PIPE_HEIGHT: f32 = 100.0; // Example height of the pipe.

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_pipes) // Spawn pipes every 2 seconds
        .add_systems(Update, player_movement)
        .add_systems(Update, sync_player_camera)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Pipe {
    top: bool,
}

pub struct Ledge {}

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
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn spawn_pipe(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    x: f32,
) {
    let window = window_query.get_single().unwrap();
    let gap = 150.0; // Gap size between top and bottom pipes

    // Top pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                x + window.width() / 2.0,
                window.height() - PIPE_HEIGHT,
                0.0,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)),
                ..default()
            },
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        Pipe { top: true },
    ));

    // Bottom pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x + window.width() / 2.0, PIPE_HEIGHT, 0.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)),
                ..default()
            },
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        Pipe { top: false },
    ));
}

pub fn sync_player_camera(
    player: Query<&Transform, With<Player>>,
    mut camera: Query<(&mut Camera2d, &mut Transform), Without<Player>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };
    let Ok((mut camera, mut camera_transform)) = camera.get_single_mut() else {
        return;
    };

    let delta = player.translation - camera_transform.translation;

    if delta != Vec3::ZERO {
        camera_transform.translation += delta;
    }
}

pub fn spawn_pipes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let gap = 150.0; // Gap size between top and bottom pipes
    let count = 4;
    for i in 0..count {
        spawn_pipe(
            &mut commands,
            &window_query,
            &asset_server,
            (i as f32) * 150.0,
        )
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}
