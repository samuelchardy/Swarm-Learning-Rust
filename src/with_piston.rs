extern crate rand;
extern crate shuffle;

mod boid;
mod constants;
mod point;
mod vector;
mod world;
mod waypoint;
mod target;
mod agent;
mod simulation;
mod scenarios;

use piston_window::*;
use world::World;
use std::env;
use std::process::ExitCode;
use constants::*;

const NUM_BOIDS: u32 = 100;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];
const TARGET_BOD: &'static [[f64; 2]] = &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];

#[allow(dead_code)]
fn draw_waypoints(environment: &World, gfx: &mut G2d, context: &Context) {
    let waypoints = environment.get_waypoints();
    for i in 0..waypoints.len() {
        waypoints[i].draw(gfx, context);
    }
}

fn draw_target(environment: &World, gfx: &mut G2d, context: &Context) {
    environment.get_target().draw(TARGET_BOD, gfx, context);
}

fn draw_swarm(environment: &World, gfx: &mut G2d, context: &Context) {
    let boids = environment.get_boids();
    for i in 0..boids.len() {
        boids[i].draw(BOID_BOD, gfx, context);
    }
}

fn draw_agent(environment: &World, gfx: &mut G2d, context: &Context) {
    environment.get_agent().draw(BOID_BOD, gfx, context);
}


fn main() -> ExitCode{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("\n\ncargo run --bin with_piston DIFFICULTY[easy, medium, hard]");
        return ExitCode::from(0);
    }
    let difficulty = args[1].clone();

    let mut environment = World::new(NUM_BOIDS, &difficulty);
    let mut window: PistonWindow = WindowSettings::new("LSM Colab - Swarm Learning", (ENV_WIDTH as u32, ENV_HEIGHT as u32))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let i = 1.1;
    while let Some(e) = window.next() {
        let mut game_over = 0_i8;

        window.draw_2d(&e, |context, gfx, _| {
            // Redraw background
            clear([0.24, 0.24, 0.24, 1.0], gfx);
            game_over = environment.step(i);

            // Redraw the waypoints
            // draw_waypoints(&environment, gfx, &context);

            // Redraw the target
            draw_target(&environment, gfx, &context);

            // Redraw the boids
            draw_swarm(&environment, gfx, &context);

            // Redraw the agent
            draw_agent(&environment, gfx, &context);
        });

        if game_over != 0_i8 {
            break;
        }
    }
    return ExitCode::SUCCESS;
}
