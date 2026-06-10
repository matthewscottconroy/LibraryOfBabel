# Section 2.6: Gaussian Beams

A plane wave is infinite — it extends over all of space and carries infinite energy. Real light beams are finite. They are confined to a region of space, they diverge as they propagate, they can be focused by lenses, and they can be coupled into waveguides and fibers. The simplest, most widely used mathematical description of a real laser beam is the *Gaussian beam*.

The Gaussian beam is the lowest-order solution of the *paraxial wave equation* — the wave equation specialized to waves that are nearly plane waves, propagating predominantly in one direction. In the paraxial approximation, the complex amplitude of the beam varies slowly transversely compared to the wavelength, and the beam diverges slowly compared to its transverse width. Nearly all laser outputs in photonic computing systems are well described as Gaussian beams.

The Gaussian beam is self-similar under propagation: it remains Gaussian, with only its width, wavefront curvature, and overall phase changing with position. This makes it analytically tractable in a way that most beam profiles are not. Moreover, Gaussian beams propagate through any paraxial optical system (lenses, free space, curved mirrors) according to a simple rule — the ABCD law — which we derived in Section 2.1.3 for rays and extend here to waves.

The practical importance for photonic computing: understanding Gaussian beams is essential for

1. **Coupling between lasers and fibers**: the laser output is a Gaussian beam; the fiber accepts a Gaussian-shaped mode. Coupling efficiency depends on the overlap integral between the two Gaussians.

2. **Coupling between fibers and chips**: the fiber mode is approximately Gaussian; the silicon waveguide mode is decidedly not (it is tightly confined and highly non-Gaussian). Mode converters (inverse tapers, grating couplers) are designed to transform one to the other.

3. **Free-space optical systems**: diffractive neural networks, LiDAR systems, and free-space optical communications all involve Gaussian beam propagation through a sequence of optical elements.

4. **Laser cavity design**: the eigenmode of a stable optical cavity is a Gaussian beam (or a higher-order Hermite-Gaussian or Laguerre-Gaussian mode).

## Subsections

- **2.6.1 — The Paraxial Wave Equation**: Deriving the equation governing slowly varying paraxial beams from the Helmholtz equation.
- **2.6.2 — Gaussian Beam Parameters**: The Gaussian beam solution; beam waist, Rayleigh range, divergence angle, wavefront curvature.
- **2.6.3 — ABCD Propagation of Gaussian Beams**: The $q$-parameter and its transformation through paraxial optical systems.
- **2.6.4 — Coupling Efficiency**: Mode overlap integrals; coupling between Gaussian beams and waveguide modes; practical coupling strategies.
