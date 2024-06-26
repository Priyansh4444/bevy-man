use bevy::{
    ecs::{component::Component, entity::Entity, event::Event},
    math::Vec3,
};

#[derive(Component)]
pub struct Player {
    pub is_attatched_to_ledge: bool,
    pub ledge_attatched_to: Option<Entity>,
    pub ledge_x: f32,
    pub closest_distance: f32,
    pub ledge_y: f32,
    pub velocity: Vec3,
    pub swinging: bool, // Flag to indicate if the player is currently swinging
    pub angular_velocity: f32, // Initial push when starting to swing
}

#[derive(Component)]
pub struct Pipe {
    pub top: bool,
    pub position: Vec3,
}

#[derive(Component, Clone)]
pub struct Ledge {
    pub distance_from_player: f32,
    pub id: u32,
    pub position: Vec3,
}

#[derive(Component)]
pub struct Rope {
    pub start: Vec3,
    pub end: Vec3,
}

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;
