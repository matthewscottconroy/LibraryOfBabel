# Stability Analysis

An equilibrium tells us where a dynamical system can rest. Stability analysis tells us whether the system will actually stay near that equilibrium when perturbed. This distinction between existence of equilibria and their stability is fundamental: in any physical system, only stable equilibria are observed in practice, because any slight disturbance from an unstable equilibrium drives the system away.

## Definitions of Stability

Let $y^*$ be an equilibrium of $y' = f(y)$. The equilibrium is **stable** (in the sense of Lyapunov) if for every $\epsilon > 0$ there exists $\delta > 0$ such that $|y(x_0) - y^*| < \delta$ implies $|y(x) - y^*| < \epsilon$ for all $x > x_0$. In words: solutions that start close to $y^*$ remain close.

The equilibrium is **asymptotically stable** if it is stable and additionally $y(x) \to y^*$ as $x \to \infty$ for all initial conditions sufficiently close to $y^*$. Asymptotic stability is the stronger condition, requiring not merely that solutions stay near $y^*$ but that they are attracted to it.

The equilibrium is **unstable** if it is not stable: there exist solutions starting arbitrarily close to $y^*$ that eventually leave a fixed neighborhood.

For one-dimensional autonomous systems, the definition simplifies greatly via the phase line analysis: an equilibrium is asymptotically stable if and only if the arrows on the phase line both point toward it, which happens precisely when $f$ changes sign from positive to negative at $y^*$.

## Linearization and the Exponential Decay Rate

The rate at which solutions approach a stable equilibrium is determined by the linearization. Writing $y(x) = y^* + u(x)$ and expanding $f(y^* + u) = f(y^*) + f'(y^*)u + O(u^2) = f'(y^*)u + O(u^2)$, the linearized equation is

$$u' = f'(y^*)u,$$

with solution $u(x) = u(0)e^{f'(y^*)x}$. When $f'(y^*) < 0$, perturbations decay at the exponential rate $\lambda = f'(y^*) < 0$. The quantity $\tau = -1/f'(y^*)$ is the **characteristic time** or relaxation time: it measures how long it takes perturbations to decay by a factor of $e^{-1}$.

**Example.** For the logistic equation $y' = ry(1 - y/K)$ near the stable equilibrium $y^* = K$:

$$f'(y) = r - \frac{2ry}{K}, \qquad f'(K) = r - 2r = -r.$$

Perturbations decay as $e^{-rx}$, with characteristic time $\tau = 1/r$. A larger growth rate $r$ means faster relaxation to the carrying capacity.

## Basin of Attraction

The **basin of attraction** of an asymptotically stable equilibrium $y^*$ is the set of initial values $y(x_0) = y_0$ from which the solution converges to $y^*$ as $x \to \infty$. For a one-dimensional system, the basin of attraction is an open interval: the largest interval containing $y^*$ on which $f$ maintains the sign pattern corresponding to attraction (positive below $y^*$, negative above).

For $y' = y(y-1)(y+2)$, the stable equilibrium is $y^* = 0$. The basin of attraction is $(-2, 1)$: starting anywhere in this interval, the solution converges to 0. Starting outside this interval, the solution moves away from 0 toward $\pm\infty$ or toward $y = 1$ (from the right side of $1$ it blows up upward).

## Global versus Local Stability

Asymptotic stability as defined above is a **local** concept: it concerns solutions starting in a neighborhood of $y^*$. **Global** asymptotic stability means all solutions (regardless of initial condition) converge to $y^*$. For one-dimensional systems, global asymptotic stability of $y^*$ requires that $y^*$ be the only equilibrium, and that $f(y) > 0$ for all $y < y^*$ and $f(y) < 0$ for all $y > y^*$.

## Bifurcations: Stability Changes Under Parameter Variation

When an ODE depends on a parameter $\mu$, equilibria can appear, disappear, or change stability as $\mu$ varies. A **bifurcation** occurs at a parameter value $\mu_0$ where the number or stability type of equilibria changes.

The simplest example is the **saddle-node bifurcation**: $y' = \mu - y^2$. For $\mu > 0$, there are two equilibria $y^* = \pm\sqrt{\mu}$; the positive one is unstable ($f'(\sqrt{\mu}) = -2\sqrt{\mu} < 0$... wait: $f(y) = \mu - y^2$, $f'(y) = -2y$, so $f'(\sqrt{\mu}) = -2\sqrt{\mu} < 0$, stable; $f'(-\sqrt{\mu}) = 2\sqrt{\mu} > 0$, unstable). For $\mu = 0$, the two equilibria merge into a single semi-stable equilibrium at $y^* = 0$. For $\mu < 0$, there are no real equilibria, and all solutions diverge to $-\infty$.

The **transcritical bifurcation** occurs in $y' = \mu y - y^2 = y(\mu - y)$: the equilibria $y^* = 0$ and $y^* = \mu$ exchange stability as $\mu$ passes through 0. This structure arises in population models where $\mu$ represents a growth rate minus a mortality rate.

## Stability Without Explicit Solutions

The strength of stability analysis is that it gives definitive conclusions without requiring explicit solution formulas. For complex or nonlinear equations where no closed form exists, analyzing $f'(y^*)$ or constructing a phase line provides the essential information about long-term behavior. This philosophy, extracting qualitative conclusions from the structure of the equation, extends to higher-dimensional systems through Lyapunov functions and linearization at equilibria in the phase plane, topics developed in the systems unit.
