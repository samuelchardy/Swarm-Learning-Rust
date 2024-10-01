use crate::{constants::PI_X_2, point::Point, vector::Vector, waypoint::Waypoint};
use piston_window::*;

#[derive(Clone, Copy)]
pub struct Boid {
    pub id: u32,
    pub point: Point,
    pub vector: Vector,
    pub color: [f32; 4],
}

const BOID_COLOR: [f32; 4] = [0.75, 0.9, 0.36, 0.9];
const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];

impl Boid {
    pub fn new(point: Point, vector: Vector, id: u32) -> Boid {
        Boid {
            id,
            point,
            vector,
            color: BOID_COLOR,
        }
    }

    pub fn get_angle(&self) -> f32 {
        self.vector.get_angle()
    }

    pub fn set_angle(&mut self, angle: f32) {
        self.vector.set_angle(angle);
    }

    pub fn step_forward(&mut self, percent: f32) {
        let x = self.vector.dx * percent;
        let y = self.vector.dy * percent;
        self.point.move_forward(x, y);
    }

    pub fn get_point(self) -> Point {
        self.point.clone()
    }

    pub fn step(&mut self, seconds: f32, neighbors: Vec<Boid>, target_waypoint: Waypoint) {
        let mut vectors: Vec<Vector> = Vec::new();
        
        if neighbors.len() > 0 {
            let mut separation = Vector::mean(
                neighbors
                    .iter()
                    .map(|b| self.point.vector_to(&b.point))
                    .collect::<Vec<Vector>>(),
            );
            separation.set_length(separation.get_length() + 250f32);
            vectors.push(separation);

            let average_location =
                Point::mean(neighbors.iter().map(|b| b.point).collect::<Vec<Point>>());
            vectors.push(self.point.vector_to(&average_location).divide(100_f32));

            let average_heading = Vector::mean(
                neighbors
                    .iter()
                    .map(|b| {
                        let mut v = Vector { dx: 1f32, dy: 0f32 };
                        v.set_angle(b.vector.get_angle());
                        v.set_length(20f32);
                        v
                    })
                    .collect::<Vec<Vector>>(),
            );
            vectors.push(average_heading);
        }
        // Next waypoint vector
        vectors.push(self.point.vector_to(&target_waypoint.point));

        let final_vector = Vector::mean(vectors);
        self.turn_to(final_vector.get_angle(), 0.04f32);

        self.step_forward(seconds);
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

    pub fn draw(self, gfx: &mut G2d, context: &Context) {
        let transform = context
        .transform
        .trans(self.point.get_x() as f64, self.point.get_y() as f64)
        .rot_rad(-1.57075 + self.get_angle() as f64);

        polygon(self.color, BOID_BOD, transform, gfx);
    }
}
