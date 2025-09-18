use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::core::{
    math::constants,
    shapes::{edge::Edge, point::Point, triangle::Triangle},
    triangulation::circumable::Circumable,
};

pub struct BowyerWatsonTriangulator {}

impl BowyerWatsonTriangulator {
    pub fn compute(points: &[Point], need_shuffle: bool) -> Vec<Triangle> {
        let super_triangle = Circumable::super_triangle(points);

        let mut shuffled_points = points.to_vec();
        Self::shuffle_if_need(&mut shuffled_points, need_shuffle);

        let mut triangles: Vec<Triangle> = Vec::with_capacity(points.len() / 3 + 1);
        triangles.push(super_triangle.clone());

        for point in &shuffled_points {
            let mut bad_triangles: Vec<Triangle> = Vec::new();

            // 2) find all triangles whose circumcircle contains p
            for triangle in &triangles {
                if Circumable::point_in(triangle, &point) {
                    bad_triangles.push(triangle.clone());
                }
            }

            // 3) find the polygon (boundary) of the hole by counting edges
            let mut polygon: Vec<Edge> = Vec::new();
            for bad in &bad_triangles {
                for edge in bad.edges() {
                    Edge::toggle_polygon_edge(&mut polygon, edge, constants::EPSILON);
                }
            }

            // 4) remove bad triangles from triangulation
            triangles.retain(|t| !bad_triangles.contains(t));

            // 5) re-triangulate the hole with triangles from polygon edges to p
            for edge in &polygon {
                triangles.push(Triangle {
                    a: edge.start.clone(),
                    b: edge.end.clone(),
                    c: point.clone(),
                });
            }
        }

        // 6) remove triangles that use the super-triangle's vertices
        let super_triangle_points = &super_triangle.vertexes();

        triangles.retain(|t| !t.has_any_vertex(super_triangle_points));

        return triangles;
    }

    fn shuffle_if_need(points: &mut [Point], need_shuffle: bool) {
        if need_shuffle {
            let mut rng = rand::rng();
            points.shuffle(&mut rng);
        }
    }
}
