# Pitchfork Bifurcation

The pitchfork bifurcation occurs in systems with a $\mathbb{Z}_2$ symmetry—systems that are invariant under $x \mapsto -x$. This symmetry forces the vector field to be an odd function near the equilibrium, which changes the structure of the bifurcation dramatically compared to the saddle-node. Rather than two equilibria colliding and annihilating, one equilibrium destabilizes while two new symmetric equilibria are born—or in the subcritical case, two unstable equilibria collide with the stable one, destroying it.

## Normal Form

The normal form of the pitchfork bifurcation is:

$$\dot{x} = \mu x - x^3 \quad \text{(supercritical)}$$

$$\dot{x} = \mu x + x^3 \quad \text{(subcritical)}.$$

The $\mathbb{Z}_2$ symmetry $x \mapsto -x$ is preserved in both cases (odd functions), which is why no even powers of $x$ appear.

## Supercritical Pitchfork

For $\dot{x} = \mu x - x^3$, the equilibria satisfy $x(\mu - x^2) = 0$:
- $x_0 = 0$ for all $\mu$.
- $x_\pm = \pm\sqrt{\mu}$ for $\mu > 0$.

Stability: $F'(x) = \mu - 3x^2$.
- At $x_0 = 0$: $F'(0) = \mu$. Stable for $\mu < 0$, unstable for $\mu > 0$.
- At $x_\pm = \pm\sqrt{\mu}$: $F'(\pm\sqrt{\mu}) = \mu - 3\mu = -2\mu < 0$ for $\mu > 0$. Both are stable.

The bifurcation diagram forms a pitchfork: one branch (the origin) splits into three at $\mu = 0$, with the two new branches stable and the original now unstable. The shape of this diagram—one curve splitting symmetrically into two—gives the bifurcation its name.

**Physical interpretation.** The supercritical pitchfork models the spontaneous breaking of a symmetry to one of two equivalent stable states. As $\mu$ increases through 0, the symmetric state $x = 0$ becomes unstable, and the system settles to one of the two symmetric stable states $x = \pm\sqrt{\mu}$. The choice of which state is adopted depends on initial conditions or small perturbations (symmetry breaking).

## Subcritical Pitchfork

For $\dot{x} = \mu x + x^3$, the equilibria satisfy $x(\mu + x^2) = 0$:
- $x_0 = 0$ for all $\mu$.
- $x_\pm = \pm\sqrt{-\mu}$ for $\mu < 0$ (i.e., they exist for $\mu < 0$ and are unstable).

Stability:
- At $x_0 = 0$: $F'(0) = \mu$. Stable for $\mu < 0$, unstable for $\mu > 0$.
- At $x_\pm = \pm\sqrt{-\mu}$ (for $\mu < 0$): $F'(\pm\sqrt{-\mu}) = \mu + 3(-\mu) = -2\mu > 0$. Unstable.

The subcritical pitchfork is more dramatic: for $\mu < 0$, the origin is stable and surrounded by two unstable equilibria at $\pm\sqrt{-\mu}$. As $\mu$ increases to 0, the unstable equilibria collapse onto the origin, which then becomes unstable for $\mu > 0$. There are no nearby stable states for $\mu > 0$ (in the one-dimensional normal form without higher-order terms).

**Physical consequence: Hysteresis.** In practice, a subcritical pitchfork is typically accompanied by higher-order terms that stabilize the system for larger amplitudes. Consider:

$$\dot{x} = \mu x + x^3 - x^5.$$

For small $\mu < 0$: stable origin, two unstable inner equilibria, two stable outer equilibria (beyond the unstable ones). As $\mu$ increases through 0, the origin becomes unstable, and the system jumps discontinuously to the outer stable branch. As $\mu$ is then decreased back below 0, the system remains on the outer branch until $\mu$ decreases past the saddle-node that connects the inner and outer branches. This region of parameter space admits two stable states (the origin and the outer branch), and the system exhibits hysteresis: the behavior depends on the history of how $\mu$ was varied.

## Theorem

**Theorem (Pitchfork Bifurcation).** Let $F(x, \mu)$ be smooth with $F(-x, \mu) = -F(x, \mu)$ (odd in $x$), $F(0, \mu) = 0$ for all $\mu$, $F_x(0, 0) = 0$, $F_{x\mu}(0, 0) \neq 0$, and $F_{xxx}(0, 0) \neq 0$. Then:
- If $F_{xxx}(0,0) < 0$ (supercritical): two stable nontrivial equilibria $x^\pm(\mu) = \pm\sqrt{-F_{x\mu}/F_{xxx}} \sqrt{\mu} + O(\mu)$ are born for $\mu > 0$ as the origin becomes unstable.
- If $F_{xxx}(0,0) > 0$ (subcritical): two unstable nontrivial equilibria exist for $\mu < 0$ and collide with the stable origin at $\mu = 0$, which then becomes unstable.

## Center Manifold Reduction

For $n$-dimensional systems with $\mathbb{Z}_2$ symmetry $S: x \mapsto -x$ (meaning $F(-x, \mu) = -F(x, \mu)$), the center manifold at the bifurcation is one-dimensional and $S$-invariant. By the symmetry, the restriction of $F$ to the center manifold is an odd function, eliminating all even powers. The resulting scalar ODE has the pitchfork normal form, and the theorem applies.

## Example: Buckling of an Elastic Rod

Consider a straight elastic rod under compressive end load $\lambda$. The linearized equilibrium condition is $EI y'' + \lambda y = 0$ with simply supported boundary conditions $y(0) = y(L) = 0$. The nontrivial solutions (buckled states) appear at $\lambda_n = n^2 \pi^2 EI/L^2$ (Euler buckling loads). Near $\lambda_1$, the nonlinear theory (with energy stored in stretching) gives a supercritical pitchfork: for $\lambda < \lambda_1$, the straight rod is the only stable equilibrium; for $\lambda > \lambda_1$, it becomes unstable and two buckled states $y \approx \pm A \sin(\pi x/L)$ (bowing upward or downward) are stable. The $\mathbb{Z}_2$ symmetry is $y \mapsto -y$ (bowing direction is arbitrary).

## Example: Rayleigh-Benard Convection

In a horizontal fluid layer heated from below, the conduction state (no fluid motion) is stable for Rayleigh number $Ra < Ra_c$. At $Ra = Ra_c$, a convection rolls appear via a supercritical pitchfork (or pitchfork bifurcation in amplitude, with rolls turning left or right). The symmetry $x \mapsto -x$ corresponds to the two possible roll orientations.

## Connection to Symmetry Breaking

The pitchfork bifurcation is the archetype of **spontaneous symmetry breaking**: the equations of motion have a symmetry, but the solutions below and above the bifurcation do not. This phenomenon appears throughout physics and biology—from ferromagnetism (Ising model, up/down symmetry broken by spontaneous magnetization) to the Higgs mechanism (continuous symmetry broken to give mass to gauge bosons) to left-right symmetry breaking in embryonic development.
