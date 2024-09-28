use crate::{constants::PI_X_2, point::Point, target::Target, boid::Boid, agent::Agent};
// use rand::Rng;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

#[derive(Clone, Copy)]
pub struct Simulation {
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {}
    }
   
    pub fn sim_reward(&mut self, seconds: f32, mut agent: Agent, target: Target,
                        swarm_com: Point) -> i8 {
            for _i in 0..500 {
                agent.step_forward(seconds);
                if agent.point.distance_to(&swarm_com) < 100.0 {
                    return -1_i8;
                }

                if agent.point.distance_to(&target.point) < 10.0 {
                    return 1_i8;
                }
            }
            return 0_i8;
        }

    pub fn find_move(&mut self, seconds: f32, mut agent: Agent, target:
                        Target, swarm: Vec<Boid>) -> f32 { 
        let mut swarm_com = Point::mean(
            swarm
                .iter()
                .map(|b| b.point)
                .collect::<Vec<Point>>(),
        );

        let mut pi = 3.14;
        let mut angles = vec![-pi/2.0, -pi/2.11, -pi/2.22, -pi/2.33, -pi/2.44, -pi/2.55,
                            -pi/2.66, -pi/2.77, -pi/2.88, -pi/2.99, -pi/3.10,
                            pi/3.10, pi/2.99, pi/2.88, pi/2.77, pi/2.66, pi/2.55,
                            pi/2.44, pi/2.33, pi/2.22, pi/2.11, pi/2.0];
        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();

        irs.shuffle(&mut angles, &mut rng);
        let mut rewards = Vec::new();
        let mut largest_ind = 0;


        for i in 0..angles.len() {
            let mut agent_sim = agent;
            agent_sim.turn_to(agent_sim.get_angle()-angles[i], 0.04f32);
            let sim_reward_val = self.sim_reward(seconds, agent_sim, target, swarm_com);
            rewards.push(sim_reward_val);

            if sim_reward_val > rewards[largest_ind] {
                largest_ind = i;
            }
        }

        let reward_big = rewards[largest_ind];
        println!("{reward_big}");
        return angles[largest_ind];
    }
}