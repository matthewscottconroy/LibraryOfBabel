# Stability via Lyapunov Methods

Lyapunov's direct method (or Lyapunov's second method) is the most powerful general technique for analyzing stability of equilibria in nonlinear systems. Rather than solving the differential equation, it constructs an auxiliary scalar function — a Lyapunov function — that behaves like a generalized energy: positive away from the equilibrium and decreasing along solution trajectories. If such a function can be found, stability or instability follows immediately, without knowing the solutions explicitly. The method applies globally or locally, handles non-hyperbolic equilibria that linearization cannot classify, and provides quantitative estimates of basins of attraction.

## Stability Definitions

Let $\mathbf{x}^*$ be an equilibrium of $\mathbf{x}' = \mathbf{F}(\mathbf{x})$. Without loss of generality, translate so that $\mathbf{x}^* = \mathbf{0}$.

The equilibrium is **Lyapunov stable** (or simply stable) if for every $\varepsilon > 0$ there exists $\delta > 0$ such that $|\mathbf{x}(0)| < \delta$ implies $|\mathbf{x}(t)| < \varepsilon$ for all $t \geq 0$. Solutions starting near the equilibrium remain near it for all future time.

The equilibrium is **asymptotically stable** if it is Lyapunov stable and there exists $\delta > 0$ such that $|\mathbf{x}(0)| < \delta$ implies $|\mathbf{x}(t)| \to 0$ as $t \to +\infty$. Solutions starting sufficiently near the equilibrium not only remain near it but actually approach it.

The equilibrium is **globally asymptotically stable** if it is asymptotically stable and the condition $|\mathbf{x}(t)| \to 0$ holds for every initial condition, not just those starting near the equilibrium.

An equilibrium that is not Lyapunov stable is **unstable**: there exist initial conditions arbitrarily close to the equilibrium from which solutions eventually leave every fixed neighborhood.

## Lyapunov's Stability Theorem

**Theorem (Lyapunov, 1892).** Let $\mathbf{0}$ be an equilibrium of $\mathbf{x}' = \mathbf{F}(\mathbf{x})$, and let $D$ be an open set containing $\mathbf{0}$. Suppose there exists a continuously differentiable function $V: D \to \mathbb{R}$ such that:

(i) $V(\mathbf{0}) = 0$ and $V(\mathbf{x}) > 0$ for all $\mathbf{x} \in D \setminus \{\mathbf{0}\}$ (**positive definite**).

(ii) $\dot{V}(\mathbf{x}) = \nabla V(\mathbf{x}) \cdot \mathbf{F}(\mathbf{x}) \leq 0$ for all $\mathbf{x} \in D$ (**negative semidefinite** along trajectories).

Then $\mathbf{0}$ is Lyapunov stable. If in addition:

(iii) $\dot{V}(\mathbf{x}) < 0$ for all $\mathbf{x} \in D \setminus \{\mathbf{0}\}$ (**negative definite**),

then $\mathbf{0}$ is asymptotically stable.

**Proof sketch.** The function $V$ decreases along trajectories (by condition (ii)). The level sets $\{V(\mathbf{x}) \leq c\}$ are positively invariant: once a trajectory enters such a set, it stays in it (since $\dot{V} \leq 0$ prevents $V$ from increasing). For any $\varepsilon > 0$, let $B_\varepsilon$ be a ball of radius $\varepsilon$ around the origin. Since $V$ is continuous and positive definite, there exists $c > 0$ such that $\{V \leq c\} \subset B_\varepsilon$. Choose $\delta$ so that $B_\delta \subset \{V \leq c\}$. Starting in $B_\delta$ keeps $V \leq c$, hence keeps the trajectory in $B_\varepsilon$ — this is Lyapunov stability.

For asymptotic stability with $\dot{V} < 0$: since $V$ is decreasing and bounded below by zero, it converges to some limit $L \geq 0$. If $L > 0$, then on the set $\{V \geq L/2\}$, $\dot{V}$ is bounded away from zero by some $-\alpha < 0$ (compactness), so $V(t) \leq V(0) - \alpha t \to -\infty$ — contradiction. Hence $L = 0$ and $V(\mathbf{x}(t)) \to 0$, which (by positive definiteness of $V$) forces $\mathbf{x}(t) \to \mathbf{0}$.

## The Time Derivative Along Trajectories

The key quantity is $\dot{V}(\mathbf{x}) = \frac{d}{dt}V(\mathbf{x}(t)) = \nabla V(\mathbf{x}) \cdot \mathbf{x}' = \nabla V(\mathbf{x}) \cdot \mathbf{F}(\mathbf{x})$.

This can be computed directly from $V$ and $\mathbf{F}$ without knowing the trajectory $\mathbf{x}(t)$. For a two-dimensional system $x' = f(x,y)$, $y' = g(x,y)$:

$$\dot{V} = \frac{\partial V}{\partial x}f(x,y) + \frac{\partial V}{\partial y}g(x,y).$$

This is the directional derivative of $V$ in the direction of the vector field. The condition $\dot{V} < 0$ means $V$ decreases along every trajectory — the level sets of $V$ are barriers that trajectories cross inward.

## Worked Example: Nonlinear Oscillator

Consider the damped nonlinear oscillator $x'' + x' + x^3 = 0$, or equivalently $x' = y$, $y' = -x^3 - y$. The equilibrium is the origin. Try $V(x,y) = \frac{1}{4}x^4 + \frac{1}{2}y^2$.

Clearly $V(0,0) = 0$ and $V(x,y) > 0$ for $(x,y) \neq (0,0)$: $V$ is positive definite.

Compute $\dot{V}$:

$$\dot{V} = \frac{\partial V}{\partial x}\cdot y + \frac{\partial V}{\partial y}\cdot(-x^3 - y) = x^3 y + y(-x^3 - y) = x^3 y - x^3 y - y^2 = -y^2.$$

Here $\dot{V} = -y^2 \leq 0$, which is negative semidefinite. By the theorem, the origin is Lyapunov stable. But $\dot{V} = 0$ when $y = 0$, regardless of $x$ — so the theorem does not immediately give asymptotic stability.

To strengthen the conclusion, apply LaSalle's invariance principle: the largest invariant set within $\{\dot{V} = 0\} = \{y = 0\}$ is examined. If $y = 0$ for all $t$, then $y' = -x^3 - y = -x^3 = 0$, so $x = 0$. The largest invariant subset of $\{y = 0\}$ is just the origin. LaSalle's principle then guarantees that all trajectories in a suitable neighborhood approach the origin: the origin is asymptotically stable.

## LaSalle's Invariance Principle

LaSalle's theorem extends Lyapunov's method to handle negative semidefinite $\dot{V}$:

**Theorem (LaSalle).** Let $\Omega$ be a compact positively invariant set and $V: \Omega \to \mathbb{R}$ a $C^1$ function with $\dot{V} \leq 0$ in $\Omega$. Let $E = \{\mathbf{x} \in \Omega : \dot{V}(\mathbf{x}) = 0\}$ and $M$ be the largest invariant subset of $E$. Then every trajectory starting in $\Omega$ approaches $M$ as $t \to +\infty$.

For the asymptotic stability of an equilibrium, one applies LaSalle with $\Omega = \{V \leq c\}$ for suitably small $c$. If the largest invariant subset of $\{\dot{V} = 0\}$ within $\Omega$ is just the equilibrium, then the equilibrium is asymptotically stable.

LaSalle's principle is particularly useful for mechanical systems with velocity damping, where $V$ is an energy function and $\dot{V}$ vanishes when the velocity is zero (not just at the equilibrium).

## Instability Theorem

Chetaev's theorem provides conditions for instability from a Lyapunov-like function:

**Theorem (Chetaev).** If there exists $V: D \to \mathbb{R}$ and a region $U \subset D$ containing the equilibrium in its boundary such that $V(\mathbf{x}) > 0$ and $\dot{V}(\mathbf{x}) > 0$ for all $\mathbf{x} \in U$, and $V = 0$ on $\partial U \cap D$, then the equilibrium is unstable.

Intuitively: $V$ is increasing in $U$, so trajectories starting in $U$ near the equilibrium are pushed outward and cannot approach the equilibrium.

## Global Stability and the Barbalat Lemma

When $\Omega = \mathbb{R}^n$ (the whole state space), global asymptotic stability follows from a globally positive definite, radially unbounded Lyapunov function ($V(\mathbf{x}) \to +\infty$ as $|\mathbf{x}| \to +\infty$) with $\dot{V} < 0$ everywhere except the origin. The radial unboundedness ensures that the level sets $\{V \leq c\}$ are compact, which is needed for the stability argument.

For many physical systems — mechanical systems with damping, electrical circuits, certain ecological models — natural energy functions serve as Lyapunov functions and establish global stability with relatively little computation. The Lyapunov method thus provides a path from physical intuition (energy should decrease) to mathematical proof (rigorous stability), bridging the gap between engineering insight and mathematical rigor.

## Choosing a Lyapunov Function

There is no systematic algorithm for finding a Lyapunov function. Several heuristics guide the search. For mechanical systems, the total energy (kinetic plus potential) is the first candidate. For linear systems, the function $V = \mathbf{x}^T P \mathbf{x}$ where $P$ is a positive definite matrix satisfying the Lyapunov equation $A^T P + PA = -Q$ (for any positive definite $Q$) always works. For gradient systems $\mathbf{x}' = -\nabla \phi$, the function $V = \phi$ is a natural choice. For other systems, quadratic-plus-higher-order functions, combinations of energy and cross terms, or sum-of-squares polynomials (via computational methods) are common approaches. The difficulty of finding Lyapunov functions is a genuine challenge; it reflects the depth and generality of the method — it works for any system, but extracting its conclusions requires problem-specific insight.
