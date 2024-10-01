use crate::{boid::Boid, point::Point, vector::Vector, waypoint::Waypoint, constants::*,
            target::Target, agent::Agent, simulation::Simulation, scenarios::create_target_waypoints};
use rand::Rng;

#[allow(dead_code)]
#[derive(Clone)]
pub struct World {
    width: f32,
    height: f32,
    boids: Vec<Boid>,
    target: Target,
    waypoints: Vec<Waypoint>,
    waypoint_index: usize,
    agent: Agent,
    simulation: Simulation,
}

struct Grid {
    x: f32,
    y: f32,
}

const MAX_VELOCITY: f32 = 2.0;
const MIN_VELOCITY: f32 = 1.5;

const SIGHT: f32 = 10.0;
const GRID_GAP: f32 = 50.0;


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

fn create_agent() -> Agent {
    // let rng = rand::thread_rng();
    let point = Point::new(1350.0, 500.0);
    let vector = Vector {
        dx: -2.0,
        dy: -2.0,
    };
    let agent = Agent::new(point, vector);
    agent
}

impl World {
    pub fn new(total_boids: u32, difficulty: &String) -> World {

        // Create target & waypoints based on difficulty
        let (target, waypoints) = create_target_waypoints(difficulty);
        let waypoint_index = 0;

        // Create boids
        let boids = create_boids(total_boids);

        // Create agent
        let agent = create_agent();

        // Create simulation
        let sim = Simulation::new();

        World {
            width: ENV_WIDTH,
            height: ENV_HEIGHT,
            boids: boids,
            waypoints: waypoints,
            waypoint_index: waypoint_index,
            target: target,
            agent: agent,
            simulation: sim,
        }
    }

    pub fn check_win_conditions(&mut self) -> i8 {
        // Check if agent is to close to a boid
        for i in 0..self.boids.len() {
            let boid = self.boids[i];
            if boid.get_point().distance_to(&self.agent.get_point()) < 12_f32 {
                println!("FAIL: AGENT HIT SWARM!");
                return -1_i8;
            }
        }

        // // Check if agent is at the target
        if self.agent.get_point().distance_to(&self.target.get_point()) <= 10_f32 {
            println!("SUCCESS: AGENT REACHED THE TARGET!");
            return 1_i8;
        }
        return 0_i8;
    }

    pub fn move_swarm(&mut self, seconds: f32) {
        for i in 0..self.boids.len() {
            let mut boid = self.boids[i];
            let neighbors = self.clone().get_visible_neighbors(&boid);

            boid.step(seconds, neighbors, self.waypoints[self.waypoint_index]);
            self.boids[i] = boid;
        }

        // Calculate the swarms centre of mass
        let swarm_com = Point::mean(
            self.boids
                .iter()
                .map(|b| b.point)
                .collect::<Vec<Point>>(),
        );

        // Determine which waypoint boids should target
        if swarm_com.distance_to(&self.waypoints[self.waypoint_index].get_point()) < 40_f32 {
            if self.waypoint_index == self.waypoints.len()-1 {
                self.waypoint_index = 0;
            } else {
                self.waypoint_index = self.waypoint_index + 1;
            }
        }
    }

    pub fn step(&mut self, seconds: f32) -> i8 {
        // Check win conditions
        let win = self.check_win_conditions();
        if win != 0 {
            return win;
        }

        // Move the boids
        self.move_swarm(seconds);

        // Create simulation
        let new_angle = self.simulation.find_move(seconds, self.agent.clone(), self.target.clone(),
                                        self.boids.clone());

        // Move the agent
        self.agent.step_plan(seconds, new_angle);

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
