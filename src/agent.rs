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

    pub fn get_point(self) -> Point {
        self.point.clone()
    }

    pub fn get_angle(&self) -> f32 {
        self.vector.get_angle()
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.vector.set_angle(angle);
    }

    pub fn turn_to(&mut self, mut heading: f32, percent: f32) {
        let angle = self.get_angle();
        if heading < angle {
            heading += PI_X_2;
        }
        let mut diff = heading - angle;

        if diff >= std::f32::consts::PI {
            diff = diff - PI_X_2;
        }

        self.set_angle(angle + diff * percent);
    }

    pub fn step_forward(&mut self, percent: f32) {
        let x = self.vector.dx * percent;
        let y = self.vector.dy * percent;
        self.point.move_forward(x, y);
    }

    pub fn step(&mut self, seconds: f32, target: Target) {
        let final_vector = self.point.vector_to(&target.point);
        self.turn_to(final_vector.get_angle(), 0.04f32);
        self.step_forward(seconds);
    }
}