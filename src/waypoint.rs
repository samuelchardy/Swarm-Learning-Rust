use crate::{point::Point};

#[derive(Clone, Copy)]
pub struct Waypoint {
    pub point: Point,
    pub radius: f64,
    pub id: f32,
    pub color: [f32; 4],
}

const WAYPOINT_COLOR: [f32; 4] = [0.05, 0.95, 0.05, 0.20];

impl Waypoint {
    pub fn new(point: Point, radius: f64, id: f32) -> Waypoint {
        Waypoint {
            point,
            radius,
            id,
            color: WAYPOINT_COLOR
        }
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }
}