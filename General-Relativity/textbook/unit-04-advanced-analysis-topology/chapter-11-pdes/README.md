# Chapter 11: Partial Differential Equations

---

## Chapter Introduction

Partial differential equations (PDEs) describe how fields — quantities distributed over space and time — evolve and interact. The three classical second-order linear PDEs are:

- The **Laplace equation** ∇²φ = 0 (elliptic): governs electrostatic and gravitational potentials in vacuum, steady-state heat distribution, and minimal surfaces.
- The **heat equation** ∂u/∂t = κ∇²u (parabolic): governs diffusion processes, the spread of heat, and is related to the Euclidean path integral in quantum mechanics.
- The **wave equation** ∂²u/∂t² = c²∇²u (hyperbolic): governs propagation of light, sound, and gravitational waves.

The classification (elliptic/parabolic/hyperbolic) governs: whether boundary conditions or initial conditions are appropriate; whether solutions depend continuously on data (well-posedness); and whether information propagates at finite speed (hyperbolic) or instantaneously (elliptic).

**The Einstein equations** are a second-order nonlinear hyperbolic-elliptic system for the metric tensor. In the initial value formulation (Chapter 37), they split into: **constraint equations** (elliptic, to be satisfied on an initial spacelike hypersurface) and **evolution equations** (hyperbolic, governing how the geometry evolves forward in time). The well-posedness theorem for the Einstein equations — first established by Choquet-Bruhat (1952) — applies the theory of hyperbolic PDEs to GR.

---

## Sections in This Chapter

- [Section 11.1: Classification and Well-Posedness](section-11.1-classification/README.md)
- [Section 11.2: The Laplace and Poisson Equations](section-11.2-laplace/README.md)
- [Section 11.3: The Wave Equation and Causality](section-11.3-wave-equation/README.md)
- [Section 11.4: The Heat Equation and Diffusion](section-11.4-heat-equation/README.md)
- [Exercises](exercises.md)
- [Further Reading and References](further-reading.md)
- [Important Researchers](important-researchers.md)
- [Important Concepts](important-concepts.md)
