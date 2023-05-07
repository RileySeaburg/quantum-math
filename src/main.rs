extern crate plotters;
extern crate schroidnger_equation;

use plotters::prelude::*;
use schroidnger_equation::WaveFunction;
use std::fs::create_dir_all;


/// The main function generates frames for a video of the wave function evolution,
/// then uses FFmpeg to compile these frames into a
/// video.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the wave function with initial values
    let mut wave_fn = WaveFunction::new(0.0, 1.0, 0.5, -5.0, 0.1, 1.0, 0.0, 0.0);

    // Create a directory to store the frames
    create_dir_all("frames")?;

    // Generate frames
    for i in 0..100 {
        // Create a frame as a PNG image
        let filename = format!("frames/frame_{:04}.png", i);
        let root = BitMapBackend::new(&filename, (640, 480)).into_drawing_area();
        root.fill(&WHITE)?;

        // Adjust the range of the axes based on the expected values of the wave function and position
        let mut chart = ChartBuilder::on(&root)
            .caption("Wave Function", ("Arial", 20))
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-6.0..6.0, -2.0..2.0)?;

        chart.configure_mesh().draw()?;

        // Generate points for the wave function
        let mut points = Vec::with_capacity(100);
        for _ in 0..100 {
            points.push((wave_fn.x, wave_fn.psi));
            wave_fn.update(); // Update the wave function after collecting the point
        }

        // Plot the wave function
        chart.draw_series(LineSeries::new(points, &RED))?;

        root.present()?;
    }

    // Use FFmpeg to convert the frames into an MP4 video
    // (This assumes that FFmpeg is installed and available in your PATH)
    std::process::Command::new("ffmpeg")
        .arg("-framerate")
        .arg("10") // Frame rate (e.g., 10 frames per second)
        .arg("-i")
        .arg("frames/frame_%04d.png") // Input file pattern (e.g., all PNG files in the frames directory)
        .arg("-c:v")
        .arg("libx264")
        .arg("-profile:v")
        .arg("high")
        .arg("-crf")
        .arg("20")
        .arg("-pix_fmt")
        .arg("yuv420p")
        .arg("output.mp4") // Output file name
        .status()?
        .success()
        .then(|| println!("Successfully created output.mp4"))
        .ok_or("Failed to create output.mp4")?;

    Ok(())
}
