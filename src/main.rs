#![allow(dead_code)]
#![allow(unused_imports)]

mod core;

use core::shapes::point::Point;

use crate::core::plot::plotly_builder::PlotlyBuilder;
use crate::core::plot::plotters_builder::PlottersBuilder;
use crate::core::shapes::point_generator::PointGenerator;
use crate::core::shapes::triangle::Triangle;
use crate::core::triangulation::bowyer_watson_triangulator::BowyerWatsonTriangulator;

fn main() {
    let point_count: usize = 3;

    let points: Vec<Point> = PointGenerator::square(point_count); // FPointGenerator::heart(1f64)

    let triangles: Vec<Triangle> = BowyerWatsonTriangulator::compute(&points, false);

    // let _ = PlottersBuilder::triangles(&triangles);
    let _ = PlotlyBuilder::triangles(&triangles);

    println!(
        "point count: {point_count} make triangles: {:?}",
        triangles.len()
    );
}
