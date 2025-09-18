use crate::core::shapes::{edge::Edge, point::Point};

#[derive(Debug, PartialEq, Clone)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn ab(&self) -> Edge {
        return Edge {
            start: self.a.clone(),
            end: self.b.clone(),
        };
    }
    pub fn bc(&self) -> Edge {
        return Edge {
            start: self.b.clone(),
            end: self.c.clone(),
        };
    }
    pub fn ca(&self) -> Edge {
        return Edge {
            start: self.c.clone(),
            end: self.a.clone(),
        };
    }

    pub fn edges(&self) -> [Edge; 3] {
        return [self.ab().clone(), self.bc().clone(), self.ca().clone()];
    }
    pub fn vertexes(&self) -> [Point; 3] {
        return [self.a.clone(), self.b.clone(), self.c.clone()];
    }

    pub fn has_vertex(&self, point: &Point) -> bool {
        let points = self.vertexes();

        return points.contains(&point);
    }
    pub fn has_any_vertex(&self, points: &[Point]) -> bool {
        for p in points {
            if self.has_vertex(&p) {
                return true;
            }
        }

        return false;
    }
}
