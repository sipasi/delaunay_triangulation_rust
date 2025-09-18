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
}
