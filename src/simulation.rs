use crate::{constants::PI_X_2, point::Point, vector::Vector, target::Target, boid::Boid, agent::Agent};

#[derive(Clone, Copy)]
pub struct Simulation {
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {}
    }

    pub fn turn_to(&mut self, angle: f32, mut heading: f32, percent: f32) -> f32 {
        if heading < angle {
            heading += PI_X_2;
        }
        let mut diff = heading - angle;

        if diff >= std::f32::consts::PI {
            diff = diff - PI_X_2;
        }

        return angle + diff * percent;
    }
}