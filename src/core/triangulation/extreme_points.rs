use crate::core::shapes::point::Point;

pub struct ExtremePoints {
    pub leftmost: Point,
    pub rightmost: Point,
    pub topmost: Point,
    pub bottommost: Point,
}

impl ExtremePoints {
    pub fn horizontal_distance(&self) -> f64 {
        return self.rightmost.x - self.leftmost.x;
    }
    pub fn vertical_distance(&self) -> f64 {
        return self.topmost.y - self.bottommost.y;
    }

    pub fn find(points: &[Point]) -> ExtremePoints {
        let mut leftmost: &Point = &points[0];
        let mut rightmost: &Point = &points[0];
        let mut topmost: &Point = &points[0];
        let mut bottommost: &Point = &points[0];

        for point in points {
            if point.x < leftmost.x {
                leftmost = point;
            }

            if point.x > rightmost.x {
                rightmost = point;
            }

            if point.y > topmost.y {
                topmost = point;
            }

            if point.y < bottommost.y {
                bottommost = point;
            }
        }

        return ExtremePoints {
            leftmost: leftmost.clone(),
            rightmost: rightmost.clone(),
            topmost: topmost.clone(),
            bottommost: bottommost.clone(),
        };
    }
}
