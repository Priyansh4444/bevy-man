use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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
        .add_systems(Startup, spawn_pipes)
        .add_systems(Startup, spawn_ledges) // Spawn pipes every 2 seconds
        .add_systems(
            Update,
            (
                player_movement,
                player_ledge_edging,
                sync_player_camera,
                update_ledges_information,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
pub struct Player {
    is_attatched_to_ledge: bool,
    ledge_attatched_to: Option<Entity>,
}

#[derive(Component)]
pub struct Pipe {
    top: bool,
}

#[derive(Component, Clone)]
pub struct Ledge {
    pub distance_from_player: f32,
    pub id: u32,
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
                custom_size: Some(Vec2::new(
                    PIPE_WIDTH * 2.0,
                    window.height() - PIPE_HEIGHT * 2.0 - gap,
                )),
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
                custom_size: Some(Vec2::new(
                    PIPE_WIDTH * 2.0,
                    window.height() - PIPE_HEIGHT * 2.0 - gap,
                )),
                ..default()
            },
            texture: asset_server.load("pipe.png"),
            ..default()
        },
        Pipe { top: false },
    ));
}

pub fn spawn_ledge(
    commands: &mut Commands,
    window_query: &Query<&Window, With<PrimaryWindow>>,
    asset_server: &Res<AssetServer>,
    x: f32,
) {
    let window = window_query.get_single().unwrap();
    let _gap = 150.0; // Gap size between top and bottom pipes
                      // Ledge handle
    let ledge_height = 100.0;
    let ledge_width = 100.0;
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                window.width() / 2.0 + x * 2.0,
                window.height() - PIPE_HEIGHT - ledge_height,
                0.0,
            ),
            sprite: Sprite {
                custom_size: Some(Vec2::new(ledge_width, ledge_height)),
                ..default()
            },
            texture: asset_server.load("Smoking_pipe.png"),
            ..default()
        },
        Ledge {
            distance_from_player: 0.0,
            id: x as u32,
        },
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

pub fn spawn_pipes(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let _window = window_query.get_single().unwrap();
    let _gap = 150.0; // Gap size between top and bottom pipes
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
            (i as f32) * 150.0,
        )
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut set: ParamSet<(
        Query<(&mut Transform, &mut Player), With<Player>>,
        Query<(&Transform, &mut Ledge), With<Ledge>>,
    )>,
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

        if player.is_attatched_to_ledge {}

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_ledge_edging(
    mut player_query: Query<(&mut Player, &Transform), With<Player>>, // Include Transform to get player position
    ledges: Query<(Entity, &Ledge, &Transform), With<Ledge>>, // Include Transform and Entity for ledges
) {
    let mut closest_ledge: Option<Entity> = None; // Store the entity ID of the closest ledge
    let mut closest_distance = f32::MAX;

    if let Ok((mut player, player_transform)) = player_query.get_single_mut() {
        // Calculate distance based on player's position
        for (ledge_entity, _ledge, ledge_transform) in ledges.iter() {
            let distance = player_transform
                .translation
                .distance(ledge_transform.translation);
            if distance < closest_distance {
                closest_distance = distance;
                closest_ledge = Some(ledge_entity);
            }
        }
        player.ledge_attatched_to = closest_ledge;
    }
}

#[derive(Component)]
pub struct Rope {
    pub ledge_attatched_to: Entity,
    pub distance_from_ledge: f32,
}
