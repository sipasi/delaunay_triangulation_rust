use std::{env, fs::File, io::Write, path::PathBuf};

use crate::core::{shapes::triangle::Triangle, triangulation::circumable::Circumable};

use plotly::{
    Layout, Plot, Scatter,
    color::Rgb,
    common::{Fill, Mode, Title},
    layout::{Axis, ColorAxis, themes::BuiltinTheme},
};
use plotters::style::Color;

pub struct PlotlyBuilder {}

impl PlotlyBuilder {
    pub fn triangles(triangles: &Vec<Triangle>) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize a new Plot
        let mut plot = Plot::new();

        // Define a color palette for the triangles
        let colors = vec![
            Rgb::new(255, 99, 132),  // Red
            Rgb::new(54, 162, 235),  // Blue
            Rgb::new(255, 159, 64),  // Orange
            Rgb::new(75, 192, 192),  // Teal
            Rgb::new(153, 102, 255), // Purple
            Rgb::new(255, 159, 64),  // Yellow
        ];

        // Iterate over each triangle and add it to the plot
        for (i, t) in triangles.iter().enumerate() {
            // Define the vertices of the triangle
            let x = vec![t.a.x, t.b.x, t.c.x, t.a.x];
            let y = vec![t.a.y, t.b.y, t.c.y, t.a.y];

            // Create a Scatter trace for the triangle
            let trace = Scatter::new(x, y)
                .mode(Mode::Lines)
                .name(format!("T {}", i + 1))
                .line(
                    plotly::common::Line::new()
                        .color(colors[i % colors.len()])
                        .width(2.0),
                )
                .fill(Fill::ToSelf);

            // Add the trace to the plot
            plot.add_trace(trace);

            // === CIRCUMCIRCLE ===
            let circle = Circumable::circle(t);
            if !circle.center.x.is_nan() && !circle.radius.is_infinite() {
                let steps = 100;
                let mut cx = Vec::with_capacity(steps + 1);
                let mut cy = Vec::with_capacity(steps + 1);

                for k in 0..=steps {
                    let theta = 2.0 * std::f64::consts::PI * k as f64 / steps as f64;
                    cx.push(circle.center.x + circle.radius * theta.cos());
                    cy.push(circle.center.y + circle.radius * theta.sin());
                }

                let circ_trace = Scatter::new(cx, cy)
                    .mode(Mode::Lines)
                    .name(format!("Circ {}", i + 1))
                    .line(
                        plotly::common::Line::new()
                            .color("rgba(200,200,200,0.5)") // light gray, semi-transparent
                            .width(1.5),
                    );

                plot.add_trace(circ_trace);
            }
        }

        // Define the layout of the plot
        let layout = Layout::new()
            .title("Delaunay Triangulation")
            .template(BuiltinTheme::PlotlyDark.build())
            .auto_size(true)
            .x_axis(
                Axis::new()
                    .title("X Axis")
                    .show_grid(false)
                    .scale_anchor("y"),
            )
            .y_axis(Axis::new().title("Y Axis").show_grid(false))
            .show_legend(true);

        // Set the layout for the plot
        plot.set_layout(layout);

        // Get output path
        let file_name = Self::get_output_path();

        // Create html file
        let _ = Self::create_html_file(&plot, &file_name);

        // Display the plot in the default web browser
        plot.show_html(&file_name);

        Ok(())
    }

    fn create_html_file(plot: &Plot, file_name: &str) -> std::io::Result<()> {
        // Generate HTML
        let html = plot.to_html();

        // Save HTML to file
        let mut file = File::create(file_name)?;
        file.write_all(html.as_bytes())?;

        // println!("Saved plot to {}", file_name);

        Ok(())
    }

    fn get_output_path() -> String {
        // Full path
        let full_path: PathBuf = PathBuf::from("output\\assets\\plotly_triangles.html");

        // Get current working directory
        let current_dir = env::current_dir().expect("Failed to get current dir");

        // Compute relative path
        let relative_path = full_path.strip_prefix(&current_dir).unwrap_or(&full_path);

        // println!("Relative path: {}", relative_path.display());

        // Convert Path to String
        relative_path.to_string_lossy().to_string()
    }
}
