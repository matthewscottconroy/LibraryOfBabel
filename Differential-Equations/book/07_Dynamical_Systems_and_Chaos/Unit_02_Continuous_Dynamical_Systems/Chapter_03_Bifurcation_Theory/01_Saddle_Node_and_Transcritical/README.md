# Saddle-Node and Transcritical Bifurcations

The simplest bifurcations occur when a single eigenvalue of the Jacobian crosses zero. The behavior depends critically on the structure of the vector field near the bifurcation: whether a symmetry condition is present, and the signs of certain nonlinear terms. The saddle-node bifurcation is the generic case; the transcritical bifurcation requires an additional structure (typically the existence of a fixed point for all parameter values).

## The Implicit Function Theorem and Non-Hyperbolicity

Let $\dot{x} = F(x, \mu)$ with $F: \mathbb{R}^n \times \mathbb{R} \to \mathbb{R}^n$ smooth. Suppose $F(x^*, \mu_0) = 0$ (equilibrium) and $DF_x(x^*, \mu_0)$ (Jacobian with respect to $x$) is invertible (hyperbolic case). Then the implicit function theorem guarantees that the equilibrium persists as a smooth curve $x^*(\mu)$ for $\mu$ near $\mu_0$. Bifurcations occur precisely when $\det DF_x(x^*, \mu_0) = 0$.

When the Jacobian has a simple zero eigenvalue, the center manifold is one-dimensional, and the bifurcation analysis reduces to a one-dimensional problem.

## Saddle-Node Bifurcation

The **saddle-node bifurcation** (also called fold bifurcation) is the generic bifurcation with a single zero eigenvalue. The normal form is:

$$\dot{x} = \mu - x^2.$$

**Phase portrait analysis.** Equilibria satisfy $x^2 = \mu$:
- If $\mu < 0$: no real equilibria.
- If $\mu = 0$: one equilibrium at $x^* = 0$ (the bifurcation point).
- If $\mu > 0$: two equilibria at $x^\pm = \pm\sqrt{\mu}$.

Stability: $F'(x) = -2x$. At $x^+ = \sqrt{\mu}$, $F'(x^+) = -2\sqrt{\mu} < 0$ (stable). At $x^- = -\sqrt{\mu}$, $F'(x^-) = 2\sqrt{\mu} > 0$ (unstable). So a stable and an unstable equilibrium collide and disappear as $\mu$ decreases through 0.

**Theorem (Saddle-Node Bifurcation).** Let $F(x^*, \mu_0) = 0$, $F_x(x^*, \mu_0) = 0$ (single zero eigenvalue, with $v$ and $w$ the right and left zero eigenvectors), and:

$$w \cdot F_\mu(x^*, \mu_0) \neq 0, \quad w \cdot D^2F(x^*, \mu_0)(v, v) \neq 0.$$

Then the system undergoes a saddle-node bifurcation at $(x^*, \mu_0)$: two equilibria exist on one side of $\mu_0$ and none on the other.

The conditions are: $F_\mu \neq 0$ (the parameter actually moves the equilibrium) and the quadratic term in the center manifold reduction is nonzero.

**Example.** The equation $\dot{x} = \mu + x - x^2$ has equilibria where $\mu + x - x^2 = 0$, i.e., $x = (1 \pm \sqrt{1 + 4\mu})/2$. These collide at $\mu = -1/4$ where the discriminant vanishes. This is a saddle-node bifurcation at $x^* = 1/2$, $\mu_0 = -1/4$.

## Transcritical Bifurcation

The **transcritical bifurcation** occurs when there is an equilibrium that exists for all $\mu$ (typically $x = 0$ for all $\mu$, due to a symmetry or conservation law), and another equilibrium passes through it as $\mu$ varies. The normal form is:

$$\dot{x} = \mu x - x^2.$$

Equilibria: $x(\mu - x) = 0$, giving $x_1 = 0$ and $x_2 = \mu$ for all $\mu$.

Stability: $F'(x) = \mu - 2x$.
- At $x_1 = 0$: $F'(0) = \mu$. Stable for $\mu < 0$, unstable for $\mu > 0$.
- At $x_2 = \mu$: $F'(\mu) = -\mu$. Stable for $\mu > 0$, unstable for $\mu < 0$.

At $\mu = 0$, the two equilibria coincide and exchange stability: for $\mu < 0$, the origin is stable; for $\mu > 0$, the equilibrium at $x = \mu$ is stable. The two equilibria pass through each other, swapping stability in the process.

**Theorem (Transcritical Bifurcation).** Under conditions analogous to the saddle-node theorem but with $w \cdot F_\mu(x^*, \mu_0) = 0$ (the equilibrium persists for all $\mu$) and appropriate transversality conditions on the bilinear term $D^2F$, the system undergoes a transcritical bifurcation: two branches of equilibria cross, exchanging stability.

**Example: Logistic Equation.** The ODE $\dot{x} = rx(1-x)$ has equilibria $x = 0$ and $x = 1$ for all $r$. At $r = 0$, these coalesce at a degenerate point, but the transcritical structure is visible at $x = 0$ as $r$ varies: $F_x(0, r) = r$, which changes sign at $r = 0$. For $r > 0$, $x = 0$ is unstable and $x = 1$ is stable; for $r < 0$, the roles are reversed.

A more typical example comes from population models with immigration: $\dot{x} = (r - x)x$, where $x = 0$ loses stability at $r = 0$ as a nontrivial equilibrium $x = r$ enters from negative values.

## Normal Form Theory and Center Manifold Reduction

For $n$-dimensional systems, the reduction to the one-dimensional normal forms above is justified by center manifold theory. At a bifurcation where the Jacobian has a simple zero eigenvalue, there exists a one-dimensional center manifold $W^c$ tangent to the zero eigenvector. The dynamics on $W^c$ is governed by a scalar ODE, and after coordinate changes this ODE takes the normal form. The dynamics off $W^c$ is exponentially attracted or repelled (by the stable and unstable manifolds), so the center manifold captures all the long-term behavior near the bifurcation.

## Unfolding and Codimension

The saddle-node bifurcation is **codimension 1**: it occurs generically on a hypersurface in parameter space (requiring only one parameter to be tuned). The transcritical bifurcation is also codimension 1 but requires an additional structural assumption (the presence of a fixed point for all parameters). Without this structure, the transcritical bifurcation is unstable: generic perturbations break it into either a saddle-node or no bifurcation.

The **universal unfolding** of the saddle-node (normal form $\mu - x^2$) is parametrized by the single parameter $\mu$ and already contains all possible behaviors in its neighborhood. This is the essence of codimension 1: one parameter suffices to capture all qualitative behavior near the bifurcation point.

## Applications

Saddle-node bifurcations model **bistability** and **catastrophes**: as a control parameter is varied, a system can jump discontinuously from one stable state to another when the stable state is annihilated in a saddle-node. This is observed in:
- Laser physics (threshold behavior).
- Ecology (collapse of populations below minimum viable size).
- Electrical circuits (fold in the V-I characteristics of a tunnel diode).
- Neuroscience (threshold for action potential generation in the Hodgkin-Huxley model).

Transcritical bifurcations appear in:
- Epidemic models (the SIR model, where the disease-free equilibrium becomes unstable as the basic reproduction number $R_0$ exceeds 1).
- Laser physics (at the lasing threshold).
- Coupled oscillators (synchronization transitions).
