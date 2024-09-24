extern crate rand;

mod boid;
mod constants;
mod point;
mod vector;
mod world;
mod waypoint;

use piston_window::*;
use world::World;

const NUM_BOIDS: u32 = 100;
const SIZE: u32 = 1000;

const BOID_BOD: &'static [[f64; 2]] = &[[5.0, 5.0], [10.0, 0.0], [5.0, 15.0], [0.0, 0.0]];

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
        window.draw_2d(&e, |context, gfx, _| {
            clear([1.0, 1.0, 1.0, 1.0], gfx);

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

            // Redraw the boids
            environment.step(i);
            let boids = environment.get_boids();
            for i in 0..boids.len() {
                let boid = boids[i];
                let point = boid.get_point();
                let transform = context
                    .transform
                    .trans(point.get_x() as f64, point.get_y() as f64)
                    .rot_rad(-1.57075 + boid.get_angle() as f64);

                polygon(boid.color, BOID_BOD, transform, gfx);
            }
        });
    }
}
