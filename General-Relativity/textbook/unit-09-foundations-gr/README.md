# Unit IX: Foundations of General Relativity

---

## Unit Introduction

We have everything we need. Three years of mathematics — calculus, linear algebra, differential geometry — and three years of physics — Newtonian mechanics, electromagnetism, special relativity — have prepared us for the final step. This unit takes that step.

General relativity is Einstein's theory of gravity. It is not a modification of Newtonian gravity with relativistic corrections. It is a completely new conception of what gravity *is*: not a force, but the curvature of spacetime. A planet orbiting the Sun is not being pulled by a gravitational force; it is following a straight line — a geodesic — through curved spacetime. The Sun's mass warps spacetime, and the planet moves as freely and inertially as it can through that warped geometry.

This sounds like metaphor. It is not. The statement "gravity is geometry" is mathematically precise and experimentally confirmed to extraordinary precision. The same Einstein equations that describe the precession of Mercury's perihelion (1915) describe gravitational waves detected a century later by LIGO (2015) and the shadows of black holes observed by the Event Horizon Telescope (2019). The theory has no adjustable parameters beyond Newton's gravitational constant $G$ and the cosmological constant $\Lambda$. It has been tested in every conceivable arena and found correct.

How do we get there? The route runs through four foundational ideas:

**The Equivalence Principle** (Chapter 34): Inertial mass equals gravitational mass — a fact known since Galileo, puzzling for 250 years, and finally explained by Einstein as a deep geometric truth. In the frame of a freely falling elevator, gravity disappears. This is not an approximation; locally, a gravitational field is *exactly* equivalent to an accelerated reference frame. This means that the locally observed laws of physics must be those of special relativity — gravity is absent in any sufficiently small freely-falling region. The equivalence principle is the cornerstone of GR. It tells us that the theory must be generally covariant (the same in any coordinates), that light must be deflected by gravity, and that clocks run slower in a gravitational field.

**The Einstein Field Equations** (Chapter 35): $G_{\mu\nu} = 8\pi G T_{\mu\nu}$. Ten coupled nonlinear partial differential equations relating the Einstein tensor (built from the metric and its first two derivatives) to the stress-energy tensor of matter and energy. These equations replace Poisson's equation $\nabla^2\Phi = 4\pi G\rho$ as the governing equation of gravity. They are more than a generalization: they describe not just how matter tells spacetime to curve, but how the curvature propagates, how spacetime itself can carry energy (gravitational waves), and how the geometry of the universe evolves.

**The Stress-Energy Tensor in GR** (Chapter 36): In SR, $T^{\mu\nu}$ was the source of gravity in a flat background. In GR, $T^{\mu\nu}$ is the source of curvature, and the equations of motion for matter ($\nabla_\mu T^{\mu\nu} = 0$) follow from the Bianchi identity — they are not an independent postulate. Matter tells spacetime how to curve; curved spacetime tells matter how to move. The geometry and the matter are coupled.

**The Geodesic Equation as Law of Motion** (Chapter 37): Free particles and light move on geodesics of spacetime. The geodesic equation $\ddot{x}^\mu + \Gamma^\mu_{\nu\rho}\dot{x}^\nu\dot{x}^\rho = 0$ is both the definition of a "straight line" in curved spacetime and the GR equation of motion for a freely falling body. It replaces Newton's second law for gravitational motion. In the Newtonian limit ($v \ll c$, weak fields, slow time variation), the geodesic equation reduces to $\ddot{\mathbf{x}} = -\nabla\Phi$ — Newton's second law with $\Phi = g_{00}c^2/2$ (roughly). The full geodesic equation captures all of GR's predictions for the motion of particles and light.

Together, these four chapters constitute the core of GR. Everything else — exact solutions, gravitational waves, cosmology, black holes — is an application of the framework built here.

---

## Chapters in This Unit

- [Chapter 34: The Equivalence Principle](chapter-34-equivalence-principle/README.md)
- [Chapter 35: The Einstein Field Equations](chapter-35-einstein-field-equations/README.md)
- [Chapter 36: The Stress-Energy Tensor in GR](chapter-36-stress-energy-tensor/README.md)
- [Chapter 37: The Geodesic Equation](chapter-37-geodesic-equation/README.md)
