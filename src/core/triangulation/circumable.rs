use crate::core::{
    math::constants,
    shapes::{circle::Circle, point::Point, triangle::Triangle},
    triangulation::extreme_points::ExtremePoints,
};

pub struct Circumable {}

impl Circumable {
    pub fn circle(t: &Triangle) -> Circle {
        let ax = t.a.x;
        let ay = t.a.y;

        let bx = t.b.x;
        let by = t.b.y;

        let cx = t.c.x;
        let cy = t.c.y;

        let d = 2f64 * (ax * (by - cy) + bx * (cy - ay) + cx * (ay - by));

        if d.abs() < constants::EPSILON {
            return Circle {
                center: Point {
                    x: f64::NAN,
                    y: f64::NAN,
                },
                radius: f64::INFINITY,
            };
        }

        let a2 = ax * ax + ay * ay;
        let b2 = bx * bx + by * by;
        let c2 = cx * cx + cy * cy;

        let ux = (a2 * (by - cy) + b2 * (cy - ay) + c2 * (ay - by)) / d;
        let uy = (a2 * (cx - bx) + b2 * (ax - cx) + c2 * (bx - ax)) / d;

        let center: Point = Point { x: ux, y: uy };

        return Circle {
            center: center.clone(),
            radius: center.distance_between(&t.a),
        };
    }

    pub fn super_triangle(points: &[Point]) -> Triangle {
        let extreme = ExtremePoints::find(points);

        let width = extreme.horizontal_distance();
        let height = extreme.vertical_distance();

        let center_x = (extreme.leftmost.x + extreme.rightmost.x) / 2.0;
        let center_y = (extreme.topmost.y + extreme.bottommost.y) / 2.0;

        let margin = 10.0 * f64::max(width, height);

        return Triangle {
            a: Point::new(center_x - margin, center_y - margin),
            b: Point::new(center_x, center_y + margin),
            c: Point::new(center_x + margin, center_y - margin),
        };
    }

    pub fn point_in(triangle: &Triangle, point: &Point) -> bool {
        let circle: Circle = Self::circle(&triangle);

        if f64::is_infinite(circle.radius) {
            return false;
        }

        return circle.contains(&point);
    }
}
