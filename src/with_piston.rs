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

use piston_window::*;
use world::World;

const NUM_BOIDS: u32 = 1;
const SIZE: u32 = 1000;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];
const TARGET_BOD: &'static [[f64; 2]] = &[[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];

fn main() {
    let mut environment = World::new(NUM_BOIDS, SIZE as f32);
    println!(
        "=== Flock of Seaboids with Piston ===\n {} [version {}]",
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    );
    let mut window: PistonWindow = WindowSettings::new("flock-of-boids", (SIZE, SIZE))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let i = 1.1;
    while let Some(e) = window.next() {
        let mut game_over = 0_i8;

        window.draw_2d(&e, |context, gfx, _| {
            clear([0.2, 0.2, 0.2, 1.0], gfx);
            game_over = environment.step(i);

            // Redraw the waypoints
            let waypoints = environment.get_waypoints();
            for i in 0..waypoints.len() {
                let waypoint = waypoints[i];
                let point = waypoint.get_point();
                let transform = context
                    .transform
                    .trans(point.get_x() as f64, point.get_y() as f64)
                    .rot_rad(0 as f64);

                // let width = 20.0 * (i as f64+1.0);
                let width = 1.0 * waypoint.radius;
                let rect = [-width/2.0, -width/2.0,
                            width, width];
                ellipse(waypoint.color, rect, transform, gfx);
            }

            // redraw the target
            let target = environment.get_target();
            let point = target.get_point();
            let transform = context
                    .transform
                    .trans((point.get_x() as f64) - 10.0, (point.get_y() as f64) - 10.0)
                    .rot_rad(0 as f64);
            polygon(target.color, TARGET_BOD, transform, gfx);

            // Redraw the boids
            let boids = environment.get_boids();
            for i in 0..boids.len() {
                let boid = boids[i];
                let point = boid.get_point();
                let transform = context
                    .transform
                    .trans((point.get_x() as f64) - 10.0, (point.get_y() as f64) -15.0)
                    .rot_rad(-1.57075 + boid.get_angle() as f64);

                polygon(boid.color, BOID_BOD, transform, gfx);
            }

            // Move and draw the agent
            let agent = environment.get_agent();
            //move agent
            let point = agent.get_point();
            let transform = context
                .transform
                .trans(point.get_x() as f64, point.get_y() as f64)
                .rot_rad(-1.57075 + agent.get_angle() as f64);

            polygon(agent.color, BOID_BOD, transform, gfx);
        });

        if game_over != 0_i8 {
            break;
        }
    }
}
