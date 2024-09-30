use crate::{constants::PI_X_2, point::Point, vector::Vector};
use piston_window::*;

#[derive(Clone, Copy)]
pub struct Agent {
    pub point: Point,
    pub vector: Vector,
    pub color: [f32; 4],
}

const AGENT_COLOR: [f32; 4] = [0.82, 0.20, 0.51, 0.95];

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

    pub fn step_forward_bound(&mut self, percent: f32) {
        let mut x = self.vector.dx * percent;
        let mut y = self.vector.dy * percent;

        let xy_mult = self.point.bound(x, y);
        
        if (xy_mult[0] != 1.0) || (xy_mult[1] != 1.0) {
            x = x * xy_mult[0];
            y = y * xy_mult[1];
            let new_vector = Vector {
                dx: x,
                dy: y,
            };
            self.turn_to(new_vector.get_angle(), 1.0f32);

            x = self.vector.dx * percent;
            y = self.vector.dy * percent;
        }

        self.point.move_forward(x, y);
    }

    pub fn step_plan(&mut self, seconds: f32, new_angle: f32) {     
        self.turn_to(new_angle, 0.04f32);
        self.step_forward_bound(seconds);
    }

    pub fn draw(self, boid_bod: &'static [[f64; 2]], gfx: &mut G2d, context: &Context) {
        let transform = context
        .transform
        .trans(self.point.get_x() as f64, self.point.get_y() as f64)
        .rot_rad(-1.57075 + self.get_angle() as f64);

        polygon(self.color, boid_bod, transform, gfx);
    }
}