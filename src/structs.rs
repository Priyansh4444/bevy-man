use bevy::{ecs::{component::Component, entity::Entity}, math::Vec3};

#[derive(Component)]
pub struct Player {
    pub is_attatched_to_ledge: bool,
    pub ledge_attatched_to: Option<Entity>,
    pub ledge_x: f32,
    pub closest_distance: f32,
    pub ledge_y: f32,
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
pub struct BezierState {
    pub start: Vec3,
    pub control_point_1: Vec3,
    pub control_point_2: Vec3,
    pub end: Vec3,
    pub t: f32, // Parameter ranging from 0 to 1
}
impl BezierState {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        // Initializing control points roughly, needs adjustment for better movement aesthetics
        let control_point_1 = start + (end - start) * 0.3;
        let control_point_2 = start + (end - start) * 0.7;
        BezierState {
            start,
            control_point_1,
            control_point_2,
            end,
            t: 0.0,
        }
    }

    pub fn bezier_point(&self) -> Vec3 {
        let t = self.t;
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        let p = uuu * self.start
            + 3.0 * uu * t * self.control_point_1
            + 3.0 * u * tt * self.control_point_2
            + ttt * self.end;
        p
    }
}

#[derive(Component)]
pub struct Rope {
    pub ledge_attatched_to: Entity,
    pub distance_from_ledge: f32,
}
