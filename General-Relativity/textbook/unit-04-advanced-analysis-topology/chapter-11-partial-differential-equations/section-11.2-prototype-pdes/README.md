# Section 11.2: Prototype PDEs

---

## Section Introduction

Three prototype second-order linear PDEs exemplify the three types of behavior classified in Section 11.1: the **Laplace equation** (elliptic), the **heat equation** (parabolic), and the **wave equation** (hyperbolic). Every student of physics and mathematics must know these equations intimately, because virtually every PDE of scientific importance is either one of these prototypes or a generalization of one.

The **Laplace equation** $\nabla^2 u = 0$ governs electrostatic potential in free space, steady-state heat distribution, and incompressible irrotational fluid flow. Its solutions — **harmonic functions** — are smooth everywhere and satisfy the mean value property: the value at any point equals the average over any surrounding sphere. There are no sources and no time evolution; the Laplace equation describes equilibrium.

The **heat equation** $\partial u/\partial t = \alpha\nabla^2 u$ governs diffusion: heat conduction, particle diffusion, Brownian motion. It is parabolic and describes irreversible evolution toward equilibrium. A sharp initial temperature distribution immediately becomes smooth — the heat equation is a "smoothing" operator. This irreversibility reflects the second law of thermodynamics.

The **wave equation** $\partial^2 u/\partial t^2 = c^2\nabla^2 u$ governs propagation of disturbances at finite speed $c$. It is hyperbolic and reversible in time. Solutions include traveling waves $u = f(x - ct) + g(x + ct)$ and standing waves. The wave equation governs sound, light, gravitational waves, and the evolution of scalar field perturbations in cosmology.

These three prototypes also organize the Einstein equations: in suitable gauge, the linearized Einstein equations become a wave equation for gravitational waves; the constraint equations are elliptic; and the full nonlinear system has a well-posed initial value formulation because of its hyperbolic character.

---

## Subsections

- [11.2.1: The Laplace and Poisson Equations](11.2.1-laplace.md)
- [11.2.2: The Heat Equation](11.2.2-heat.md)
- [11.2.3: The Wave Equation](11.2.3-wave.md)
- [11.2.4: d'Alembert's Solution and Characteristics](11.2.4-dalembert.md)
- [11.2.5: Initial and Boundary Value Problems](11.2.5-ibvp.md)
