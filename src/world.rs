use crate::{boid::Boid, point::Point, vector::Vector, waypoint::Waypoint};
use rand::Rng;

#[derive(Clone)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
    waypoints: Vec<Waypoint>,
    waypoint_index: usize,
}

struct Grid {
    x: f32,
    y: f32,
}

const MAX_VELOCITY: f32 = 2.0;
const MIN_VELOCITY: f32 = 1.9;

const SIGHT: f32 = 25.0;
const GRID_GAP: f32 = 8.0;
const FIELD_OF_VIEW: f32 = std::f32::consts::PI * 3.0 / 4.0;

impl World {
    pub fn new(total_boids: u32, size: f32) -> World {
        // Create boids
        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..total_boids {
            let point = Point::new(
                rng.gen_range(5_f32..125_f32),
                rng.gen_range(5_f32..125_f32),
            );
            let vector = Vector {
                dx: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
                dy: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
            };
            boids.push(Boid::new(point, vector, i));
        }

        // Create waypoints
        let mut waypoints = Vec::new();
        let xs: [f32; 3] = [300.0, 500.0, 400.0];
        let ys: [f32; 3] = [300.0, 300.0, 550.0];

        for i in 0..xs.len() {
            let point = Point::new(
                // rng.gen_range(20_f32..400_f32),
                // rng.gen_range(20_f32..400_f32),
                xs[i],
                ys[i],
            );
            waypoints.push(Waypoint::new(point, 160_f64, i as f32));
        }
        let waypoint_index = 0;

        World {
            width: size,
            height: size,
            boids: boids,
            waypoints: waypoints,
            waypoint_index: waypoint_index,
        }
    }

    pub fn step(&mut self, seconds: f32) {

        // Move the boids
        for i in 0..self.boids.len() {
            let mut boid = self.boids[i];
            let neighbors = self.clone().get_visible_neighbors(&boid);

            // Determine which waypoint boids should target
            if boid.get_point().distance_to(&self.waypoints[self.waypoint_index].get_point()) < 10_f32 {
                // println!("{}", self.waypoint_index);
                if self.waypoint_index == self.waypoints.len()-1 {
                    self.waypoint_index = 0;
                } else {
                    self.waypoint_index = self.waypoint_index + 1;
                }
            }
            boid.step(seconds, neighbors, self.waypoints[self.waypoint_index]);
            boid.bound(self.width, self.height);
            self.boids[i] = boid;
        }
    }

    pub fn get_visible_neighbors(&self, boid: &Boid) -> Vec<Boid> {
        let grid = Grid {
            x: (boid.point.get_x() / SIGHT).floor(),
            y: (boid.point.get_y() / SIGHT).floor(),
        };
        self.boids
            .iter()
            .cloned()
            .filter(|b| {
                if b.id == boid.id {
                    return false;
                }

                let other_grid = Grid {
                    x: (b.point.get_x() / SIGHT).floor(),
                    y: (b.point.get_y() / SIGHT).floor(),
                };

                if (grid.x - other_grid.x).abs() + (grid.y - other_grid.y).abs() > GRID_GAP {
                    return false;
                }

                let vector: Vector = boid.point.vector_to(&b.point);
                if vector.get_length() > SIGHT {
                    return false;
                }

                if vector.radial_distance(boid.vector) > FIELD_OF_VIEW {
                    return false;
                }

                true
            })
            .collect::<Vec<Boid>>()
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn get_waypoints(&self) -> Vec<Waypoint> {
        self.waypoints.clone()
    }
}
