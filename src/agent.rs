use crate::{constants::PI_X_2, point::Point, vector::Vector, target::Target, boid::Boid};

#[derive(Clone, Copy)]
pub struct Agent {
    pub point: Point,
    pub vector: Vector,
    pub color: [f32; 4],
}

const AGENT_COLOR: [f32; 4] = [0.05, 0.05, 0.95, 0.95];

impl Agent {
    pub fn new(point: Point, vector: Vector) -> Agent {
        Agent {
            point,
            vector,
            color: AGENT_COLOR,
        }
    }
}