use crate::{point::Point};

#[derive(Clone, Copy)]
pub struct Target {
    pub point: Point,
    pub color: [f32; 4],
}

const TARGET_COLOR: [f32; 4] = [0.95, 0.05, 0.05, 0.95];

impl Target {
    pub fn new(point: Point) -> Target {
        Target {
            point,
            color: TARGET_COLOR
        }
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }
}