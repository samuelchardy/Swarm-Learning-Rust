use crate::{constants::*, vector::Vector};
use std::convert::TryInto;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    #[allow(dead_code)]
    pub fn bound(self, dx: f32, dy: f32) -> [f32; 2] {
        let _x = self.x + dx;
        let _y = self.y + dy;
        let mut x_mult = 1.0;
        let mut y_mult = 1.0;

        if (_x >= ENV_WIDTH) || (_x <= 0.0) {
            x_mult = -1.0;
        }
        if (_y >= ENV_HEIGHT) || (_y <= 0.0) {
            y_mult = -1.0;
        }

        [x_mult, y_mult]
    }

    pub fn mean(points: Vec<Point>) -> Point {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        for i in 0..points.len() {
            sum_x += points[i].x;
            sum_y += points[i].y;
        }

        let total: u32 = points.len().try_into().unwrap();
        Point {
            x: sum_x / total as f32,
            y: sum_y / total as f32,
        }
    }

    pub fn get_x(self) -> f32 {
        self.x
    }

    pub fn get_y(self) -> f32 {
        self.y
    }

    pub fn vector_to(&self, other: &Point) -> Vector {
        Vector {
            dx: other.x - self.x,
            dy: other.y - self.y,
        }
    }

    pub fn move_forward(&mut self, x: f32, y: f32) {
        self.x += x;
        self.y += y;
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dist = dx.powi(2) + dy.powi(2);
        dist.sqrt()
    }

    #[allow(dead_code)]
    pub fn print(self) {
        let _x = self.x;
        let _y = self.y;
        println!("{_x} {_y}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_five_points_returns_mean() {
        // arrange
        let input: Vec<Point> = vec![
            Point { x: 1.0, y: 10.0 },
            Point { x: 2.0, y: 20.0 },
            Point { x: 3.0, y: 30.0 },
            Point { x: 4.0, y: 40.0 },
            Point { x: 5.0, y: 50.0 },
        ];
        const EXPECTED: Point = Point { x: 3.0, y: 30.0 };

        // act
        let result: Point = Point::mean(input);

        // assert
        assert_eq!(result, EXPECTED);
    }
}
