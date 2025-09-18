use crate::core::shapes::point::Point;

use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Edge {
    pub start: Point,
    pub end: Point,
}

impl Edge {
    pub fn normalized(&self) -> Edge {
        if Self::is_normalized(&self.start, &self.end) <= 0 {
            return self.clone();
        }

        return Edge {
            start: self.end.clone(),
            end: self.start.clone(),
        };
    }

    fn is_normalized(p1: &Point, p2: &Point) -> i32 {
        if p1.x < p2.x {
            return -1;
        }
        if p1.x > p2.x {
            return 1;
        }
        if p1.y < p2.y {
            return -1;
        }
        if p1.y > p2.y {
            return 1;
        }

        return 0;
    }

    pub fn approximate_eq(&self, other: &Self, epsilon: f64) -> bool {
        self.start.approximate_eq(&other.start, epsilon)
            && self.end.approximate_eq(&other.end, epsilon)
            || self.start.approximate_eq(&other.end, epsilon)
                && self.end.approximate_eq(&other.start, epsilon)
    }

    pub fn toggle_polygon_edge(polygon: &mut Vec<Edge>, edge: Edge, eps: f64) {
        if let Some(pos) = polygon.iter().position(|e| e.approximate_eq(&edge, eps)) {
            polygon.remove(pos);
        } else {
            polygon.push(edge);
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        let n1 = self.normalized();
        let n2 = other.normalized();

        return n1.start == n2.start && n1.end == n2.end;
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let n = self.normalized();

        n.start.x.to_bits().hash(state);
        n.start.y.to_bits().hash(state);
        n.end.x.to_bits().hash(state);
        n.end.y.to_bits().hash(state);
    }
}
