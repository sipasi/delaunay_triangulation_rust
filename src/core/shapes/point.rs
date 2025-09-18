#[derive(Debug, PartialEq, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        return Self { x: x, y: y };
    }

    pub fn distance_between(&self, point: &Point) -> f64 {
        let square_sum = self.distance_squared_to(point);

        let square_root = square_sum.sqrt();

        return square_root;
    }

    pub fn distance_squared_to(&self, point: &Point) -> f64 {
        let dx: f64 = point.x - self.x;
        let dy: f64 = point.y - self.y;

        return dx * dx + dy * dy;
    }

    pub fn approximate_eq(&self, other: &Point, epsilon: f64) -> bool {
        (self.x - other.x).abs() < epsilon && 
        (self.y - other.y).abs() < epsilon
    }
}
