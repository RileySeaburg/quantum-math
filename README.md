# Quantum Wave Function Simulation

This is a Rust project that simulates the evolution of a quantum wave function based on the Schrödinger equation. The project contains a `WaveFunction` struct that represents a quantum state, including properties for the wave function itself, its derivatives, the energy of the system, potential energy function, position, change in position, and mass of the particle.

## Features

- Defines and updates a `WaveFunction` struct based on the Schrödinger equation.
- Calculates potential energy based on the harmonic oscillator potential.
- Generates a series of frames, each displaying the wave function at a given point in time.
- Uses the Plotters library to create these frames as PNG images.
- Uses FFmpeg to convert these frames into a video, allowing for easy visualization of the wave function's evolution over time.

## Usage

### Prerequisites
This project requires that Rust and FFmpeg are installed on your machine.

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install FFmpeg](https://ffmpeg.org/download.html)

### Running the Simulation

1. Clone the repository.
2. Run `cargo build` to compile the project.
3. Run `cargo run` to execute the simulation. This will generate a series of PNG images in a `frames` directory.
4. The images in the `frames` directory are then compiled into an `output.mp4` video file.

The simulation parameters can be adjusted by modifying the `WaveFunction::new()` call in the `main()` function.

## Contributing

Please feel free to submit issues and pull requests.

## License

This project is licensed under the MIT License.
