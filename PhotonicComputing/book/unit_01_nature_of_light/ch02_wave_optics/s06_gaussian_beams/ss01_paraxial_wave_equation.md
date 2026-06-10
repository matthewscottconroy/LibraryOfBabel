# 2.6.1 — The Paraxial Wave Equation

## From Helmholtz to Paraxial

The Helmholtz equation for a monochromatic field in a medium of index $n$:

$$\nabla^2 E + k^2 E = 0, \qquad k = nk_0 = \frac{n\omega}{c}$$

For a wave propagating primarily in the $+z$ direction, we write:

$$E(x, y, z) = u(x, y, z) \, e^{ikz}$$

where $u(x, y, z)$ is a slowly varying envelope: $|\partial u/\partial z| \ll k|u|$ (the amplitude changes slowly over one wavelength in $z$). This is the *slowly varying envelope approximation* (SVEA).

Substituting into the Helmholtz equation:

$$\nabla_\perp^2 u + \frac{\partial^2 u}{\partial z^2} + 2ik\frac{\partial u}{\partial z} = 0$$

where $\nabla_\perp^2 = \partial^2/\partial x^2 + \partial^2/\partial y^2$ is the transverse Laplacian. The SVEA says $|\partial^2 u/\partial z^2| \ll |2ik\partial u/\partial z|$ (the second derivative in $z$ is negligible compared to the first derivative scaled by $2k$). Dropping the $\partial^2 u/\partial z^2$ term:

$$\nabla_\perp^2 u + 2ik\frac{\partial u}{\partial z} = 0$$

This is the **paraxial wave equation** (also called the paraxial Helmholtz equation or, in the quantum optics context, the Schrödinger equation for a free particle in 2D, with $z$ playing the role of time).

## Physical Content

The paraxial wave equation says: the transverse diffraction of the beam (described by $\nabla_\perp^2 u$) drives the $z$-evolution of the envelope. Without the $\nabla_\perp^2$ term (no diffraction), $u$ would be constant — a plane wave. With diffraction, a spatially confined beam spreads transversely as it propagates.

The equation is linear: superpositions of solutions are solutions. Its solutions form a complete basis for paraxial beams propagating in the $+z$ direction. The simplest solution (lowest-order, axially symmetric) is the Gaussian beam. Higher-order solutions (Hermite-Gaussian and Laguerre-Gaussian modes) form a complete set of transverse modes.

## Connection to the Schrödinger Equation

The paraxial wave equation $\nabla_\perp^2 u + 2ik\partial u/\partial z = 0$ is mathematically identical to the 2D Schrödinger equation for a free particle:

$$-\frac{\hbar^2}{2m}\nabla_\perp^2 \psi = i\hbar\frac{\partial\psi}{\partial t}$$

with the identification $z \leftrightarrow t$ (propagation distance plays the role of time) and $k \leftrightarrow m/\hbar$ (wavevector plays the role of mass divided by $\hbar$). This analogy is not merely formal — it means that all the mathematical results of 2D quantum mechanics (free-particle propagators, coherent states, wavepacket spreading) have direct optical analogs:

| Quantum mechanics | Paraxial optics |
|-------------------|-----------------|
| Free particle wavepacket spreading | Gaussian beam divergence |
| Harmonic oscillator eigenstates (Hermite-Gaussian quantum states) | Hermite-Gaussian beam modes |
| Coherent state (minimum-uncertainty Gaussian wavepacket) | Gaussian beam (minimum spread × divergence angle product) |
| Wigner function | Wigner distribution of beam |

This mapping makes paraxial optics a useful analog simulator for certain quantum mechanics calculations, and vice versa. It is exploited in some proposals for optical analog computing of quantum problems.

## The Validity of the Paraxial Approximation

The SVEA requires $|\partial^2 u/\partial z^2| \ll |2k\partial u/\partial z|$. For a beam with transverse size $w$ and angular divergence $\theta \approx \lambda/(2\pi w)$, the $z$-variation scale is the *Rayleigh range* $z_R = \pi w^2/\lambda$ (derived in Section 2.6.2). The condition $|\partial^2 u/\partial z^2| \ll 2k|\partial u/\partial z|$ then requires $1/z_R \ll 2k$, i.e., $\lambda \ll 2\pi w$ — the beam must be much wider than a wavelength.

For a 1550 nm Gaussian beam with waist $w_0 = 5$ μm: $w_0/\lambda = 3.2$ — marginally paraxial (beam divergence $\approx 18°$). For $w_0 = 50$ μm: $w_0/\lambda = 32$ — well paraxial (divergence $\approx 1.8°$).

**Silicon nanowire waveguide mode** ($w_0 \approx 0.3$ μm at 1550 nm): $w_0/\lambda \approx 0.2$ — deeply non-paraxial. The guided mode of a silicon wire waveguide cannot be described by the paraxial wave equation; it requires the full vector wave equation. This is why silicon photonic waveguide modes are calculated numerically (using finite element methods or finite difference mode solvers), not analytically.

The paraxial approximation is valid for:
- Free-space beams with divergence angles $< 20°$
- Weakly guided fiber modes (large mode area fiber, standard single-mode fiber)
- Larger integrated waveguides (silicon nitride, polymer)

It fails for:
- Silicon nanowire waveguides (sub-wavelength mode confinement)
- Photonic crystal waveguides (sub-wavelength periodicity)
- Near-field optics (evanescent coupling, sub-wavelength apertures)

## Summary

- Paraxial wave equation: $\nabla_\perp^2 u + 2ik\partial u/\partial z = 0$, derived from Helmholtz equation by SVEA.
- Physically: transverse diffraction drives the $z$-evolution of the beam envelope.
- Mathematically equivalent to the 2D Schrödinger equation (free particle), $z \leftrightarrow t$.
- Valid for $w_0 \gg \lambda/2\pi$ (beam width much larger than wavelength/2π).
- Fails for sub-wavelength silicon photonic waveguides — full vector numerics required.
