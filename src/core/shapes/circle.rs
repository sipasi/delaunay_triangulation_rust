use crate::core::{math::constants, shapes::point::Point};

#[derive(Debug, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn contains(&self, point: &Point) -> bool {
        let distance: f64 = self.center.distance_between(&point);

        return distance <= self.radius + constants::EPSILON;
    }
}
