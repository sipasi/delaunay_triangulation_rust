use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::*,
};
use rand::seq::IndexedRandom;

use crate::core::shapes::triangle::Triangle;

const OUTPUT_PATH: &str = "output/assets/plotters_triangles.png";

pub struct PlottersBuilder {}

impl PlottersBuilder {
    pub fn triangles(triangles: &[Triangle]) -> Result<(), Box<dyn std::error::Error>> {
        // Create a 600x600 PNG
        let output_path: String = OUTPUT_PATH.to_string();

        let root = BitMapBackend::new(&output_path, (600, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        // Compute bounds from triangles
        let all_x: Vec<f64> = triangles
            .iter()
            .flat_map(|t| vec![t.a.x, t.b.x, t.c.x])
            .collect();
        let all_y: Vec<f64> = triangles
            .iter()
            .flat_map(|t| vec![t.a.y, t.b.y, t.c.y])
            .collect();

        let x_min = all_x.iter().cloned().fold(f64::INFINITY, f64::min) - 1.0;
        let x_max = all_x.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1.0;
        let y_min = all_y.iter().cloned().fold(f64::INFINITY, f64::min) - 1.0;
        let y_max = all_y.iter().cloned().fold(f64::NEG_INFINITY, f64::max) + 1.0;

        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption("Triangles", ("sans-serif", 20))
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        // Configure mesh with bolder lines
        chart
            .configure_mesh()
            .x_desc("X Axis")
            .y_desc("Y Axis")
            .x_labels(10)
            .y_labels(10)
            .light_line_style(&WHITE) // hide grid lines if you want
            .axis_style(&BLACK) // bold axes
            .draw()?;

        // Predefined colors + shuffle for random uniqueness
        let colors = &[&RED, &BLUE, &GREEN, &CYAN, &MAGENTA, &YELLOW];
        let mut colors_index = 0;

        for (_, t) in triangles.iter().enumerate() {
            let color = colors[colors_index];
            colors_index = (colors_index + 1) % colors.len();

            chart.draw_series(LineSeries::new(
                vec![
                    (t.a.x, t.a.y),
                    (t.b.x, t.b.y),
                    (t.c.x, t.c.y),
                    (t.a.x, t.a.y), // close the triangle
                ],
                color.stroke_width(2), // thicker lines for triangles
            ))?;
        }

        println!("Saved triangles.png!");
        Ok(())
    }
}
