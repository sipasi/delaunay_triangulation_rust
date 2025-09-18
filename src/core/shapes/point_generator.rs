use std::f64::consts::PI;

use crate::core::shapes::point::Point;

pub struct PointGenerator {}

impl PointGenerator {
    pub fn square(width: usize) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(width * width);

        for i in 0..width {
            for j in 0..width {
                points.push(Point {
                    x: (i + i + 10) as f64,
                    y: j as f64,
                });
            }
        }

        return points;
    }

    pub fn heart(scale: f64) -> Vec<Point> {
        let mut heart_points = Vec::new();
        let steps = 100; // more steps = smoother heart

        for i in 0..=steps {
            let t = i as f64 / steps as f64 * 2.0 * PI;
            let x = 16.0 * t.sin().powi(3) * scale;
            let y =
                (13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos())
                    * scale;
            heart_points.push(Point::new(x, y));
        }

        return heart_points;
    }

    pub fn heart_deeper(scale: f64) -> Vec<Point> {
        let mut heart_points = Vec::new();
        let steps = 100;

        for i in 0..=steps {
            let t = i as f64 / steps as f64 * 2.0 * PI;

            // Modified coefficients to make top curve inward more
            let x = 16.0 * t.sin().powi(3) * scale;
            let y = (12.0 * t.cos()
                - 6.0 * (2.0 * t).cos()
                - 2.0 * (3.0 * t).cos()
                - 1.0 * (4.0 * t).cos())
                * scale;

            heart_points.push(Point::new(x, y));
        }

        heart_points
    }

    pub fn diamond() -> Vec<Point> {
        return vec![
            Point::new(0.0, 4.0), // top tip
            Point::new(-1.0, 3.0),
            Point::new(0.0, 3.0),
            Point::new(1.0, 3.0),
            Point::new(-2.0, 2.0),
            Point::new(-1.0, 2.0),
            Point::new(0.0, 2.0),
            Point::new(1.0, 2.0),
            Point::new(2.0, 2.0),
            Point::new(-3.0, 1.0),
            Point::new(-2.0, 1.0),
            Point::new(-1.0, 1.0),
            Point::new(0.0, 1.0),
            Point::new(1.0, 1.0),
            Point::new(2.0, 1.0),
            Point::new(3.0, 1.0),
            Point::new(0.0, 0.0), // center
            Point::new(-3.0, -1.0),
            Point::new(-2.0, -1.0),
            Point::new(-1.0, -1.0),
            Point::new(0.0, -1.0),
            Point::new(1.0, -1.0),
            Point::new(2.0, -1.0),
            Point::new(3.0, -1.0),
            Point::new(-2.0, -2.0),
            Point::new(-1.0, -2.0),
            Point::new(0.0, -2.0),
            Point::new(1.0, -2.0),
            Point::new(2.0, -2.0),
            Point::new(-1.0, -3.0),
            Point::new(0.0, -3.0),
            Point::new(1.0, -3.0),
            Point::new(0.0, -4.0), // bottom tip
        ];
    }

    pub fn romb() -> Vec<Point> {
        let cx = 20.0;
        let cy = 20.0;
        let width = 20.0;
        let height = 30.0;

        // Define diamond vertices
        let top = Point::new(cx, cy - height / 2.0);
        let right = Point::new(cx + width / 2.0, cy);
        let bottom = Point::new(cx, cy + height / 2.0);
        let left = Point::new(cx - width / 2.0, cy);

        // Center point for triangles
        let center = Point::new(cx, cy);

        // Points arranged so that you can form 4 triangles
        // Triangle 1: top, right, center
        // Triangle 2: right, bottom, center
        // Triangle 3: bottom, left, center
        // Triangle 4: left, top, center
        return vec![
            top.clone(),
            right.clone(),
            center.clone(), // triangle 1
            right.clone(),
            bottom.clone(),
            center.clone(), // triangle 2
            bottom.clone(),
            left.clone(),
            center.clone(), // triangle 3
            left.clone(),
            top.clone(),
            center.clone(), // triangle 4
        ];
    }
}
