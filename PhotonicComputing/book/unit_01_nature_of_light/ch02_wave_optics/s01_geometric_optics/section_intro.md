# Section 2.1: Geometric Optics and Its Limits

There is a useful tension at the heart of wave optics: the most powerful way to understand it is to begin by examining when it is *not* needed. Geometric optics — the theory of light rays, Snell's law, lenses, and mirrors — works extraordinarily well for most everyday optical devices. A camera, a telescope, a pair of eyeglasses: these are designed using ray optics, and they work. The question is *why* ray optics works, and precisely *when* it fails.

The answer, stated compactly: geometric optics is the limit of wave optics when the wavelength λ is much smaller than the characteristic length scales of the system — the sizes of apertures, the radii of curvature of surfaces, the distances traveled. In this limit, the wave nature of light is invisible to any practical measurement, and light can be treated as traveling in straight lines (in homogeneous media), bending at interfaces according to Snell's law, and focusing through lenses according to simple rules.

In photonic computing, this limit *never* holds. A 450 nm silicon waveguide carrying 1550 nm light is not in the ray optics regime — the wavelength is more than three times the waveguide width. An MZI operates by controlling path lengths to fractions of a wavelength. A photonic crystal has periodic features with periods on the order of a wavelength. Geometric optics, alone, cannot describe any of these.

But geometric optics is not useless in photonic computing. The ray transfer matrix formalism — introduced in this section — provides a rigorous description of paraxial optical systems that generalizes naturally to Gaussian beam propagation. And Fermat's principle, from which all of geometric optics can be derived, provides a deep connection between classical optics and quantum mechanics (via the path integral), and between geometric optics and the eikonal approximation in wave optics.

This section develops geometric optics not as a destination but as a foundation: a precise statement of the simpler regime from which we will depart.

## Subsections

- **2.1.1 — Fermat's Principle**: The variational principle from which Snell's law and all of geometric optics follow, with physical interpretation.
- **2.1.2 — Snell's Law: A Wave Derivation**: Snell's law derived from the phase matching condition at an interface — showing it is a consequence of wave optics, not an independent principle.
- **2.1.3 — Ray Transfer Matrices**: The ABCD matrix formalism for tracing rays through optical systems; the matrix for free propagation, thin lenses, and interfaces.
