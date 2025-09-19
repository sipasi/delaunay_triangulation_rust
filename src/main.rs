#![allow(dead_code)]
#![allow(unused_imports)]

mod core;

use core::shapes::point::Point;

use crate::core::plot::plotly_builder::PlotlyBuilder;
use crate::core::shapes::point_generator::PointGenerator;
use crate::core::shapes::triangle::Triangle;
use crate::core::triangulation::bowyer_watson_triangulator::BowyerWatsonTriangulator;
use crate::core::triangulation::circumable::Circumable;

fn main() {
    let point_count: usize = 100;

    let points: Vec<Point> = PointGenerator::random(point_count);

    let triangles: Vec<Triangle> = BowyerWatsonTriangulator::compute(&points, false);

    let show_circumcircle = false;
    let skip_big_circumcircle = true;
    let _ = PlotlyBuilder::triangles(&triangles, show_circumcircle, skip_big_circumcircle);

    println!("point count: {:?}", points.len());
    println!("triangles: {:?}", triangles.len());
}
