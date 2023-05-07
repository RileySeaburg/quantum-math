# Schrödinger Equation Simulation

This is a Rust crate named `schroidnger_equation` that simulates the evolution of a quantum wave function based on the Schrödinger equation. The crate contains a `WaveFunction` struct that represents a quantum state. This struct includes properties for the wave function itself (`psi`), its first and second derivatives (`psi_prime` and `psi_prime_prime`), the energy of the system (`energy`), the potential energy function (`potential`), the position (`x`), the change in position (`dx`), the mass of the particle (`mass`), and the spring constant for the harmonic oscillator potential (`k`).

## Features

- `WaveFunction` struct: Represents a quantum state and includes properties for various physical parameters and wave function derivatives.
- `new()` function: Creates a new `WaveFunction` with given initial values.
- `potential_energy()` function: Calculates the potential energy based on the harmonic oscillator potential.
- `update()` function: Updates the `WaveFunction` based on the Schrödinger equation.

## Usage

Add `schroidnger_equation` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
schroidnger_equation = "0.1.0"
```

Then import `WaveFunction` in your Rust file:

```rust
use schroidnger_equation::WaveFunction;
```

Create a new `WaveFunction` and update it as needed:

```rust
let mut wave_fn = WaveFunction::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.01, 1.0, 0.0);
wave_fn.update();
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## Support

For any questions or issues, please submit a GitHub issue.
