# Section 17.2: The Quantum Harmonic Oscillator

If quantum photonics has a single load-bearing calculation, this is it. The harmonic oscillator — a mass on a spring, quantized — seems an odd centerpiece for a book about light. But when Section 17.3 expands the electromagnetic field into modes, each mode's dynamics will turn out to be *exactly* a harmonic oscillator: the mode amplitude oscillates sinusoidally, its energy is quadratic in the amplitude, and quantizing it term-for-term reproduces everything in this section. Photons, Fock states, coherent states, squeezed light, and vacuum fluctuations are all statements about quantum harmonic oscillators.

We solve the oscillator algebraically, in the style introduced by Dirac: rather than solving the Schrödinger differential equation for wavefunctions, we factor the Hamiltonian using ladder operators $\hat{a}$ and $\hat{a}^\dagger$ and extract the entire spectrum from one commutator, $[\hat{a}, \hat{a}^\dagger] = 1$. The algebraic method is not just slicker — it is the version that generalizes to field quantization, where there is no "position wavefunction" to solve for, and its operators become the photon creation and annihilation operators used on every page of the rest of this unit.

- **17.2.1** — The Hamiltonian and Energy Levels
- **17.2.2** — Creation and Annihilation Operators
- **17.2.3** — Fock States
