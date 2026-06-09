# 11.2 The Lorenz System

In 1963, Edward Lorenz — a meteorologist, not a mathematician — was running a simplified numerical model of atmospheric convection on an early computer. He noticed something strange: two runs of the same model, starting from initial conditions that differed only in the sixth decimal place, diverged rapidly and produced completely different long-term behavior. He had discovered, or rather rediscovered, sensitive dependence on initial conditions. His paper, *Deterministic Nonperiodic Flow*, became one of the most cited scientific papers ever written and launched the modern study of chaos.

The system he studied is now called the Lorenz system, and it is our first and most important example.

## The Equations

The Lorenz system is a three-dimensional ODE, derived from the Navier-Stokes equations via a Galerkin truncation (projecting onto three Fourier modes):
$$\dot{x} = \sigma(y - x), \quad \dot{y} = x(\rho - z) - y, \quad \dot{z} = xy - \beta z.$$

The variables $x$, $y$, $z$ represent (roughly) convection rate, temperature difference, and distortion of the vertical temperature profile. The standard parameters are $\sigma = 10$ (Prandtl number), $\rho = 28$ (Rayleigh number ratio), and $\beta = 8/3$ (geometric factor). These are not arbitrary; they are the values Lorenz used, and they are where the interesting behavior lives.

**Properties of the System:**

The geometry of the Lorenz system at standard parameters is controlled by several key facts:

- The system has three equilibria: the origin $(0,0,0)$ and two symmetric points $C^\pm = (\pm\sqrt{\beta(\rho-1)}, \pm\sqrt{\beta(\rho-1)}, \rho-1)$.
- For standard parameters, all three equilibria are unstable saddles — no orbit settles down to a fixed point.
- The system is dissipative: the divergence of the vector field is $\nabla \cdot F = -\sigma - 1 - \beta = -41/3 < 0$. Phase volume shrinks at a constant exponential rate. The system contracts, but it has nowhere nice to contract *to*.
- Solutions are globally bounded: the function $V = x^2 + y^2 + (z - \rho - \sigma)^2$ decreases outside a large ellipsoid. All orbits are eventually trapped.

Combining dissipation (volume shrinks) with trapping (orbits are bounded) forces the attractor to have zero volume but be genuinely complicated. This is the geometric seed of the strange attractor.

## The Lorenz Attractor

**The Lorenz Attractor:** The omega-limit set of Lebesgue-a.e. initial condition in the bounding ellipsoid is a *strange attractor* — a compact invariant set that is neither a fixed point nor a periodic orbit, on which the dynamics is chaotic.

Lorenz observed this numerically. For decades, the rigorous mathematical picture was incomplete. Guckenheimer and Williams (1979) constructed a "geometric Lorenz model" with the right topological properties, but the gap between the model and the actual equations remained.

**Theorem 11.2.1 (Tucker, 2002 — Computer-Assisted Proof).** The Lorenz system with standard parameters has a robust chaotic attractor — a uniformly hyperbolic attractor — confirming Lorenz's numerical observations rigorously.

*(The proof uses rigorous interval arithmetic to construct a Poincaré map on a cross-section and verify its hyperbolic properties.)*

What this is saying is: Tucker's proof is not just a numerical simulation. It uses interval arithmetic — computing with guaranteed error bounds — to rigorously verify that a certain geometric structure (hyperbolicity of the Poincaré map) holds. It took nearly 40 years from Lorenz's observation to a complete rigorous proof. This is not unusual in dynamics: numerical evidence can precede rigorous proofs by decades. It is a feature of the subject, not a bug.

## 11.2.1 The Lorenz Map

A key tool for analyzing the Lorenz system is dimensional reduction via a Poincaré section.

The Poincaré map of the Lorenz system on the section $\{z = \rho - 1\}$ (the height of the two non-origin equilibria) produces a one-dimensional map $F: [0,1] \to [0,1]$ — the *Lorenz map* — that captures the essential dynamics of the return map:

- $F$ is monotone increasing on $(0, 1/2)$ and $(1/2, 1)$
- $F$ has a discontinuity at $x = 1/2$ with $F(1/2^-) = 1$ and $F(1/2^+) = 0$
- $F$ has slopes $> 1$ everywhere (it is uniformly expanding)

This map is chaotic: it has sensitive dependence, dense periodic orbits, and positive entropy $\approx \log 2$. The fact that a three-dimensional flow can be reduced to a one-dimensional map with these properties is both the power and the magic of the Poincaré section technique.

In the next section, we abstract away from the Lorenz system to study strange attractors in general — but the Lorenz attractor remains the paradigm case throughout.
