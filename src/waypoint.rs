use crate::{point::Point};
use piston_window::*;

#[derive(Clone, Copy)]
pub struct Waypoint {
    pub point: Point,
    pub radius: f64,
    pub id: f32,
    pub color: [f32; 4],
}

const WAYPOINT_COLOR: [f32; 4] = [0.05, 0.05, 0.85, 0.20];

impl Waypoint {
    pub fn new(point: Point, radius: f64, id: f32) -> Waypoint {
        Waypoint {
            point,
            radius,
            id,
            color: WAYPOINT_COLOR
        }
    }

    pub fn draw(self, gfx: &mut G2d, context: &Context) {
        let transform = context
            .transform
            .trans(self.point.get_x() as f64, self.point.get_y() as f64)
            .rot_rad(0 as f64);

        let width = 1.0 * self.radius;
        let rect = [-width/2.0, -width/2.0,
                    width, width];
        ellipse(self.color, rect, transform, gfx);
    }


    pub fn get_point(self) -> Point {
        self.point.clone()
    }
}