use std::f64::consts::PI;

use rand::{Rng, RngCore};

use crate::core::shapes::point::Point;

pub struct PointGenerator {}

impl PointGenerator {
    pub fn square(width: usize) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(width * width);

        for i in 0..width {
            for j in 0..width {
                points.push(Point {
                    x: i as f64,
                    y: j as f64,
                });
            }
        }

        return points;
    }

    pub fn random(width: usize) -> Vec<Point> {
        let mut rng = rand::rng();

        let mut point_list: Vec<Point> = Vec::with_capacity(width);

        let w = 600.0;
        let pad = w / 12.0;

        for _ in 0..width {
            // випадкова точка в межах padding
            let rand_x = rng.random_range(pad..(w - pad));
            let rand_y = rng.random_range(pad..(w - pad));

            // додаємо в обидва вектори
            point_list.push(Point {
                x: rand_x,
                y: rand_y,
            });
        }

        return point_list;
    }
}
