use crate::{point::Point};
use piston_window::*;

#[derive(Clone, Copy)]
pub struct Target {
    pub point: Point,
    pub color: [f32; 4],
}

const TARGET_COLOR: [f32; 4] = [1.0, 0.02, 0.02, 0.95];

impl Target {
    pub fn new(point: Point) -> Target {
        Target {
            point,
            color: TARGET_COLOR
        }
    }

    pub fn draw(self, target_bod: &'static [[f64; 2]], gfx: &mut G2d, context: &Context) {
        let transform = context
            .transform
            .trans(self.point.get_x() as f64, self.point.get_y() as f64)
            .rot_rad(0 as f64);
        polygon(self.color, target_bod, transform, gfx);
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }
}