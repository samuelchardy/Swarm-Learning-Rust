use crate::{point::Point, waypoint::Waypoint, target::Target};

pub fn create_target_waypoints(difficulty: &String) -> (Target, Vec<Waypoint>) {
    if difficulty == "hard" {
        return create_hard_difficulty();
    } else if difficulty == "medium" {
        return create_medium_difficulty();
    } else if difficulty == "easy" {
        return create_easy_difficulty();
    } else {
        return create_medium_difficulty();
    }
}

#[allow(dead_code)]
pub fn create_hard_difficulty() -> (Target, Vec<Waypoint>) {
    // Create target
    let point = Point::new(
        400_f32,
        600_f32,
    );
    let target = Target::new(point);
    
    let mut waypoints = Vec::new();
    let xs: [f32; 2] = [400.0, 400.0];
    let ys: [f32; 2] = [250.0, 600.0];

    for i in 0..xs.len() {
        let point = Point::new(
            xs[i],
            ys[i],
        );
        waypoints.push(Waypoint::new(point, 160_f64, i as f32));
    }
    (target, waypoints)
}

#[allow(dead_code)]
pub fn create_medium_difficulty() -> (Target, Vec<Waypoint>) {
    // Create target
    let point = Point::new(
        275_f32,
        550_f32,
    );
    let target = Target::new(point);
    
    let mut waypoints = Vec::new();
    let xs: [f32; 3] = [250.0, 600.0, 250.0];
    let ys: [f32; 3] = [250.0, 400.0, 550.0];

    for i in 0..xs.len() {
        let point = Point::new(
            xs[i],
            ys[i],
        );
        waypoints.push(Waypoint::new(point, 160_f64, i as f32));
    }
    (target, waypoints)
}

#[allow(dead_code)]
pub fn create_easy_difficulty() -> (Target, Vec<Waypoint>) {
    // Create target
    let point = Point::new(
        400_f32,
        400_f32,
    );
    let target = Target::new(point);
    
    let mut waypoints = Vec::new();
    let xs: [f32; 4] = [250.0, 550.0, 250.0, 600.0];
    let ys: [f32; 4] = [250.0, 150.0, 600.0, 400.0];

    for i in 0..xs.len() {
        let point = Point::new(
            xs[i],
            ys[i],
        );
        waypoints.push(Waypoint::new(point, 160_f64, i as f32));
    }
    (target, waypoints)
}