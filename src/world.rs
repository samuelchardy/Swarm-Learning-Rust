use crate::{boid::Boid, point::Point, vector::Vector, waypoint::Waypoint,
            target::Target, agent::Agent, simulation::Simulation};
use rand::Rng;

#[derive(Clone)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
    target: Target,
    waypoints: Vec<Waypoint>,
    waypoint_index: usize,
    agent: Agent,
}

struct Grid {
    x: f32,
    y: f32,
}

const MAX_VELOCITY: f32 = 2.0;
const MIN_VELOCITY: f32 = 1.5;

const SIGHT: f32 = 10.0;
const GRID_GAP: f32 = 50.0;
// const FIELD_OF_VIEW: f32 = std::f32::consts::PI * 3.0 / 4.0;


fn create_boids(total_boids: u32) -> Vec<Boid> {
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
    boids
}

fn create_waypoints() -> Vec<Waypoint> {
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
    waypoints
}

fn create_agent() -> Agent {
    let mut rng = rand::thread_rng();
    let point = Point::new(950.0, 450.0);
    let vector = Vector {
        dx: -2.0,
        dy: -2.0,
        // dx: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
        // dy: rng.gen_range(MIN_VELOCITY..MAX_VELOCITY),
    };
    let agent = Agent::new(point, vector);
    agent
}

impl World {
    pub fn new(total_boids: u32, size: f32) -> World {
        // Create target
        let point = Point::new(
            400_f32,
            400_f32,
        );
        let target = Target::new(point);

        // Create boids
        let boids = create_boids(total_boids);

        // Create waypoints
        let waypoints = create_waypoints();
        let waypoint_index = 0;

        // Create agent
        let agent = create_agent();

        World {
            width: size,
            height: size,
            boids: boids,
            waypoints: waypoints,
            waypoint_index: waypoint_index,
            target: target,
            agent: agent,
        }
    }

    pub fn step(&mut self, seconds: f32) -> i8 {
        // Check if agent is to close to a boid
        for i in 0..self.boids.len() {
            let boid = self.boids[i];
            if boid.get_point().distance_to(&self.agent.get_point()) < 15_f32 {
                println!("FAIL: AGENT HIT SWARM!");
                return -1_i8;
            }
        }

        // Check if agent is at the target
        if self.agent.get_point().distance_to(&self.target.get_point()) < 10_f32 {
            println!("SUCCESS: AGENT REACHED THE TARGET!");
            return 1_i8;
        }

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
            // boid.bound(self.width, self.height);
            self.boids[i] = boid;
        }
        // Create simulation
        let mut sim = Simulation::new();
        let new_angle = sim.find_move(seconds, self.agent.clone(), self.target.clone(),
                                        self.boids.clone());



        self.agent.step_plan(seconds, new_angle);

        // Move the agent
        // self.agent.step(seconds, self.target);

        return 0_i8;
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

                // if vector.radial_distance(boid.vector) > FIELD_OF_VIEW {
                //     return false;
                // }

                true
            })
            .collect::<Vec<Boid>>()
    }

    pub fn get_boids(&self) -> Vec<Boid> {
        self.boids.clone()
    }

    pub fn get_target(&self) -> Target {
        self.target.clone()
    }

    pub fn get_waypoints(&self) -> Vec<Waypoint> {
        self.waypoints.clone()
    }

    pub fn get_agent(&self) -> Agent {
        self.agent.clone()
    }
}
