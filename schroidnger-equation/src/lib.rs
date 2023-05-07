
/// `WaveFunction` struct represents a quantum state described by the Schrödinger equation.
///
/// It includes properties for the wave function itself (`psi`), its first and second
/// derivatives (`psi_prime` and `psi_prime_prime`), the energy of the system (`energy`),
/// the potential energy function (`potential`), the position (`x`), the change in position (`dx`),
/// the mass of the particle (`mass`), and the spring constant for the harmonic oscillator potential (`k`).
pub struct WaveFunction {
    pub psi: f64,
    pub psi_prime: f64,
    pub psi_prime_prime: f64,
    pub energy: f64,
    pub potential: f64,
    pub x: f64,
    pub dx: f64,
    pub mass: f64,
    pub k: f64,
}

impl WaveFunction {
    /// Creates a new `WaveFunction` with given initial values.
    ///
    /// # Examples
    ///
    /// ```
    /// let wave_fn = WaveFunction::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.01, 1.0, 0.0);
    /// ```
    pub fn new(
        psi: f64,
        psi_prime: f64,
        energy: f64,
        potential: f64,
        x: f64,
        dx: f64,
        mass: f64,
        k: f64,
    ) -> Self {
        WaveFunction {
            psi,
            psi_prime,
            psi_prime_prime: 0.0,
            energy,
            potential,
            x,
            dx,
            mass,
            k,
        }
    }

    /// Calculate the potential energy based on the harmonic oscillator potential
    pub fn potential_energy(&self) -> f64 {
        0.5 * self.k * self.x * self.x
    }

    /// Updates the `WaveFunction` based on the Schrödinger equation.
    ///
    /// This method updates the wave function and its first derivative at the next position.
    /// The next position is calculated by adding `dx` to the current position `x`.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut wave_fn = WaveFunction::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.01, 1.0, 0.0);
    /// wave_fn.update();
    /// ```
    pub fn update(&mut self) {
        let hbar = 1.0; // Planck's constant
        let k1 = 2.0 * self.mass / (hbar * hbar) * (self.energy - self.potential);
        let dx_sq_k1_12 = self.dx * self.dx / 12.0 * k1;
        
        let psi_next = 2.0 * (1.0 - 5.0 / 12.0 * self.dx * self.dx * k1) * self.psi
            - (1.0 + dx_sq_k1_12) * self.psi_prime
                / (1.0 + dx_sq_k1_12);

        self.psi_prime = self.psi;
        self.psi = psi_next;

        self.x += self.dx;
    }
}