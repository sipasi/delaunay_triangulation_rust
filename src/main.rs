#![allow(dead_code)]
#![allow(unused_imports)]

mod core;

use core::shapes::point::Point;

use crate::core::plot::plot_builder::PlotBuilder;
use crate::core::shapes::point_generator::PointGenerator;
use crate::core::shapes::triangle::Triangle;
use crate::core::triangulation::bowyer_watson_triangulator::BowyerWatsonTriangulator;

fn main() {
    let points: Vec<Point> = PointGenerator::square(5);

    let triangles: Vec<Triangle> = BowyerWatsonTriangulator::compute(&points, false);

    let _ = PlotBuilder::triangles(&triangles);

    for triangle in triangles {
        println!("{:?}", triangle);
    }
}
