use crate::{point::Point, target::Target, boid::Boid, agent::Agent, vector::Vector};
// use rand::Rng;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

#[derive(Clone, Copy)]
pub struct Simulation {
    pub non_zero_counter: u16,
}

impl Simulation {
    pub fn new() -> Simulation {
        let non_zero_counter = 0_u16;
        Simulation {
            non_zero_counter,
        }
    }
   
    #[allow(dead_code)]
    pub fn calc_rollout_depth(self) -> u16 {
        let mut term_2 = (250.0 * (1.2_f32.powf(self.non_zero_counter as f32)).round()) as u16;
        if term_2 > 1000 {
            term_2 = 1000;
        }
        let depth = 1250 - term_2;
        depth
    }

    pub fn sim_reward(&mut self, seconds: f32, mut agent: Agent, target: Target,
                    swarm_com: &Point, average_heading: &Vector, rollout_depth: u16) -> i8 {
        let mut swarm_com_sim = swarm_com.clone();

        for _i in 0..rollout_depth {
            // Simulate moving the swarm
            let x = average_heading.dx * 0.04;
            let y = average_heading.dy * 0.04;
            swarm_com_sim.move_forward(x, y);

            // Simulate moving the agent
            agent.step_forward(seconds);
            if agent.point.distance_to(&swarm_com_sim) < 50.0 {
                return -1_i8;
            }

            if agent.point.distance_to(&target.point) < 10.0 {
                return 1_i8;
            }
        }
        return 0_i8;
    }

    pub fn get_angle_actions(self) -> Vec<f32> {
        let mut rng = StepRng::new(2, 25);
        let mut irs = Irs::default();

        let half_fov = 4.5;
        let mut angles = Vec::<f32>::new();
        let interval = 0.1;
        let num_actions_half = (half_fov/interval) as u8;

        for i in 0..num_actions_half {
            angles.push(-half_fov+(i as f32 *interval));
        }
        
        for i in 0..num_actions_half {
            angles.push(half_fov-(i as f32 *interval));
        }
        let _ = irs.shuffle(&mut angles, &mut rng);
        angles
    }

    pub fn rollout(&mut self, seconds: f32, agent: Agent, target: Target,
        swarm_com: &Point, average_heading: &Vector, angles: Vec<f32>) -> f32 {
        let mut rewards = Vec::new();
        let mut largest_ind = 0;
        // let rollout_depth = self.calc_rollout_depth();
        let rollout_depth = 1000;

        for i in 0..angles.len() {
            let mut agent_sim = agent;
            agent_sim.turn_to(agent_sim.get_angle()-angles[i], 1.0f32);
            let sim_reward_val = self.sim_reward(seconds, agent_sim, target, &swarm_com, &average_heading, rollout_depth);
            rewards.push(sim_reward_val);

            if sim_reward_val > rewards[largest_ind] {
                largest_ind = i;
            }
        }

        let reward_big = rewards[largest_ind];
        let reward_act = agent.get_angle()-angles[largest_ind];

        if reward_big != 0_i8 {
            self.non_zero_counter = self.non_zero_counter + 1;
        } else {
            self.non_zero_counter = 0;
        }

        reward_act
    }

    pub fn find_move(&mut self, seconds: f32, agent: Agent, target:
                        Target, swarm: Vec<Boid>) -> f32 { 
        // Calculate the swarms centre of mass
        let swarm_com = Point::mean(
            swarm
                .iter()
                .map(|b| b.point)
                .collect::<Vec<Point>>(),
        );

        // Calculate the swarms average velocity
        let average_heading = Vector::mean(
            swarm
                .iter()
                .map(|b| {
                    b.vector
                })
                .collect::<Vec<Vector>>(),
        );

        let angles = self.get_angle_actions();
        let reward_act = self.rollout(seconds, agent, target, &swarm_com, &average_heading, angles);
        return reward_act;
    }
}