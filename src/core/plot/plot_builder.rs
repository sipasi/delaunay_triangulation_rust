use plotters::{
    chart::ChartBuilder,
    prelude::{BitMapBackend, IntoDrawingArea},
    series::LineSeries,
    style::{RED, WHITE},
};

use crate::core::shapes::triangle::Triangle;

pub struct PlotBuilder {}

impl PlotBuilder {
    pub fn triangles(triangles: &Vec<Triangle>) -> Result<(), Box<dyn std::error::Error>> {
        // Create a 600x600 PNG
        let root =
            BitMapBackend::new("output/assets/triangles.png", (600, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .build_cartesian_2d(0.0..3.0, 0.0..3.0)?;

        chart.configure_mesh().draw()?;

        for t in triangles {
            chart.draw_series(LineSeries::new(
                vec![
                    (t.a.x, t.a.y),
                    (t.b.x, t.b.y),
                    (t.c.x, t.c.y),
                    (t.a.x, t.a.y), // close the triangle
                ],
                &RED,
            ))?;
        }

        println!("Saved triangles.png!");
        Ok(())
    }
}
