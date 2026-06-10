# Section 3.1: The Classical Lorentz Oscillator Model

The first thing to understand about light-matter interaction is that classical physics gets a surprising amount right. The classical model of an atom — a positive nucleus with an electron bound to it by a restoring force, free to oscillate — gives the correct form of the frequency-dependent susceptibility, the correct qualitative shape of absorption and dispersion curves, the correct asymptotic behavior far from resonance, and the foundation for the Kramers-Kronig relations. The Lorentz oscillator model is wrong in detail (real atoms are quantized) but right in structure. It is wrong the way Newtonian gravity is wrong about Mercury's perihelion: wrong by a small amount in a specific regime, but correct and predictive everywhere else.

The model is this: an electron of mass $m$ and charge $-e$ is bound to a nucleus by a spring-like restoring force with frequency $\omega_0$. When the electromagnetic field of the light wave arrives, the oscillating electric field drives the electron off equilibrium. The electron oscillates, radiates, and — crucially — loses some of its energy to radiation (radiation damping) or to collisions with other atoms (collision damping). The net response is a driven, damped harmonic oscillator: the most analyzed differential equation in physics.

The importance of this model for photonic computing is direct. Every material in a photonic system — silicon, silica, lithium niobate, III-V semiconductors, germanium — has a dielectric response that can be described by a sum of Lorentz oscillators, each representing a different resonance (electronic transition, phonon mode, free-carrier Drude response). The refractive index and absorption at any wavelength are determined by the tails and resonances of this sum. The Soref-Bennett equations for silicon, the Sellmeier dispersion formula for silica fiber, the electro-optic response of LiNbO₃ — all are consequences of the Lorentz oscillator physics developed in this section.

## Subsections

- **3.1.1 — The Driven Harmonic Oscillator**: Setting up and solving the classical equation of motion for a bound electron in an oscillating field. The steady-state solution and its physical interpretation.
- **3.1.2 — The Complex Susceptibility**: Deriving $\chi(\omega)$ from the driven oscillator; relating it to the complex refractive index $\tilde{n} = n + i\kappa$; physical meaning of real and imaginary parts.
- **3.1.3 — Dispersion and Absorption**: Normal and anomalous dispersion; the absorption spectrum; the Sellmeier equation for practical materials.
- **3.1.4 — The Kramers-Kronig Relations Revisited**: Deriving the KK relations from causality via contour integration; their physical consequences for photonic device design.
