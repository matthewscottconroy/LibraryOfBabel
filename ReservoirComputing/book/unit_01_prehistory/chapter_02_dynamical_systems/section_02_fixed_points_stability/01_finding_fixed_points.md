# Section 2.1: Finding Fixed Points

## What Is a Fixed Point?

A **fixed point** of a dynamical system is a state that does not change over time. It is a state of perfect equilibrium — if the system begins there, it stays there forever.

For a continuous-time system $\dot{\mathbf{x}} = f(\mathbf{x})$, the fixed points are the solutions of

$$f(\mathbf{x}^*) = \mathbf{0} \tag{2.1}$$

The velocity at $\mathbf{x}^*$ is zero, so nothing moves. These points are also called **equilibria** or **stationary points**.

For a discrete-time map $x_{t+1} = f(x_t)$, a fixed point satisfies

$$f(x^*) = x^* \tag{2.2}$$

After one application of the map, you are back where you started.

Fixed points are the simplest possible long-term behavior of a dynamical system. More complex behaviors — limit cycles, quasiperiodic orbits, chaos — can be understood as departures from the simplest case. Before we can understand what happens near fixed points, we need to find them.

---

## Finding Fixed Points in 1D Systems

For a 1D continuous-time system $\dot{x} = f(x)$, the fixed points are the zeros of $f$: the values $x^*$ where $f(x^*) = 0$.

**Example.** Consider $\dot{x} = \sin(x)$. The fixed points are $x^* = k\pi$ for $k \in \mathbb{Z}$. These alternate between stable ($k$ even: $\sin'(k\pi) = \cos(k\pi) = -1 < 0$ when $k$ is even) and unstable ($k$ odd: $\cos(k\pi) = 1 > 0$). Wait — we are getting ahead of ourselves. The stability question comes in Section 2.2. For now: the fixed points are at $x^* = 0, \pm\pi, \pm 2\pi, \ldots$, and we find them by solving $\sin(x^*) = 0$.

**Example.** Consider $\dot{x} = r - x^2$ (a 1D normal form that will appear again in bifurcation theory). Fixed points satisfy $x^{*2} = r$, so $x^* = \pm\sqrt{r}$ (real only when $r \geq 0$). When $r > 0$, there are two fixed points; when $r = 0$, one (a degenerate double fixed point); when $r < 0$, none. The disappearance of fixed points as $r$ passes through zero is a **saddle-node bifurcation** (Section 5).

For 1D maps $x_{t+1} = f(x_t)$, the fixed points are the intersections of the graph $y = f(x)$ with the diagonal $y = x$.

**Example.** For the logistic map $f(x) = rx(1-x)$, the fixed-point equation is $rx(1-x) = x$. Factoring: $x(r(1-x) - 1) = 0$. Solutions: $x^* = 0$ (always) and $x^* = 1 - 1/r$ (when $r \geq 1$). The graphical interpretation is clear from the cobweb: you see the parabola $y = rx(1-x)$ intersecting the diagonal $y = x$ at these two points.

---

## Finding Fixed Points in 2D Systems

For a 2D continuous-time system $\dot{x} = f_1(x, y)$, $\dot{y} = f_2(x, y)$, the fixed points lie at the *intersections of the nullclines*.

The **$x$-nullcline** is the curve $\{(x,y) : f_1(x,y) = 0\}$. On this curve, $\dot{x} = 0$: the $x$-component of velocity is zero, so trajectories cross this curve moving purely in the $y$-direction.

The **$y$-nullcline** is the curve $\{(x,y) : f_2(x,y) = 0\}$. On this curve, $\dot{y} = 0$: trajectories cross it moving purely in the $x$-direction.

Fixed points occur where *both* $\dot{x} = 0$ and $\dot{y} = 0$: that is, at the intersections of the $x$-nullcline and the $y$-nullcline.

**Example: The Lotka-Volterra system.**

Recall the predator-prey equations:

$$\dot{x} = \alpha x - \beta x y, \qquad \dot{y} = \delta x y - \gamma y$$

The $x$-nullclines: $\alpha x - \beta xy = x(\alpha - \beta y) = 0$, so $x = 0$ or $y = \alpha/\beta$.
The $y$-nullclines: $\delta xy - \gamma y = y(\delta x - \gamma) = 0$, so $y = 0$ or $x = \gamma/\delta$.

Intersections: $(0, 0)$ and $(\gamma/\delta,\ \alpha/\beta)$. These are the two fixed points we found algebraically in Section 1.

The nullcline method is powerful because it works even when the algebraic equations are hard to solve. Sketching the nullclines on the phase plane immediately reveals where trajectories can stop, and which regions have trajectories flowing predominantly in each direction.

**Example: The van der Pol oscillator.**

$$\dot{x} = y, \qquad \dot{y} = \mu(1 - x^2)y - x$$

$x$-nullcline: $y = 0$ (the $x$-axis).
$y$-nullcline: $\mu(1-x^2)y - x = 0$, so (when $y \neq 0$) $y = x / (\mu(1-x^2))$. This is an $S$-shaped curve.

Intersection: only at the origin $(0, 0)$. So the van der Pol oscillator has a single fixed point at the origin. Yet for $\mu > 0$, the origin is unstable (as we'll show in Section 2.2), and nearby trajectories spiral outward toward a stable limit cycle. The single fixed point is surrounded by persistent oscillation.

---

## Finding Fixed Points Numerically

Except in the simplest cases, fixed-point equations must be solved numerically. Newton's method is the standard tool. Starting from an initial guess $\mathbf{x}_0$, the Newton iteration

$$\mathbf{x}_{k+1} = \mathbf{x}_k - [Df(\mathbf{x}_k)]^{-1} f(\mathbf{x}_k)$$

converges quadratically to a fixed point of the continuous-time system (a zero of $f$), provided the initial guess is close enough and $Df(\mathbf{x}^*)$ is invertible.

For finding fixed points of maps, Newton's method is applied to $g(x) = f(x) - x$: find $x^*$ such that $g(x^*) = 0$.

For finding **period-$k$ orbits** (orbits of period exactly $k$, which are fixed points of the $k$-fold composition $f^k$), Newton's method is applied to $f^k(x) - x = 0$. The complexity grows exponentially in $k$, which is one reason high-period orbits are hard to find analytically but accessible numerically.

---

## Graphical Methods: The Cobweb Diagram Revisited

For 1D maps, the cobweb diagram (introduced in Section 1.2) is both a method for finding fixed points graphically and a method for understanding their stability.

Recall the procedure: plot $y = f(x)$ and $y = x$ on the same axes, then draw the staircase pattern. Fixed points are where these curves intersect. Near the intersection:

- If the slope $|f'(x^*)| < 1$, the cobweb converges to $x^*$ (the curve $y = f(x)$ is less steep than the diagonal, so each iteration moves you closer). These are **stable** fixed points.
- If $|f'(x^*)| > 1$, the cobweb diverges away from $x^*$. These are **unstable** fixed points.
- If $f'(x^*) < 0$ and $|f'(x^*)| < 1$, the cobweb spirals inward from alternating sides (the orbit alternates above and below $x^*$).
- If $f'(x^*) < 0$ and $|f'(x^*)| > 1$, the cobweb spirals outward: alternating sides, diverging.

This is the beginning of stability analysis. The slope of the map at the fixed point — the derivative $f'(x^*)$ — determines stability in 1D. In Section 2.2, we generalize this to $n$ dimensions using the Jacobian matrix.

---

## Period-2 Points and Higher-Period Orbits

A **period-2 orbit** of the map $f$ is a pair $\{x_a, x_b\}$ with $x_a \neq x_b$ such that $f(x_a) = x_b$ and $f(x_b) = x_a$. Note that $x_a$ and $x_b$ are not fixed points of $f$ — but they *are* fixed points of $f^2 = f \circ f$.

So the period-2 points of $f$ are the solutions of $f^2(x) = x$ that are *not* solutions of $f(x) = x$.

For the logistic map $f(x) = rx(1-x)$, let's find the period-2 orbit for $r = 3.2$.

We need $f(f(x)) = x$:

$$f(f(x)) = r f(x)(1 - f(x)) = r[rx(1-x)]\bigl(1 - rx(1-x)\bigr) = x$$

This is a degree-4 polynomial equation. Its four roots are the two fixed points $x^* = 0$ and $x^* = 1 - 1/r$ (which satisfy $f(x) = x$) plus the two period-2 points. Factoring out the fixed points:

$$f^2(x) - x = (f(x) - x) \cdot Q(x)$$

where $Q(x)$ is a degree-2 polynomial whose roots are the period-2 orbit. Setting $Q(x) = 0$ and solving with the quadratic formula gives explicit expressions for $x_a$ and $x_b$:

$$x_{a,b} = \frac{(r+1) \pm \sqrt{r^2 - 2r - 3}}{2r} = \frac{(r+1) \pm \sqrt{(r-3)(r+1)}}{2r}$$

This is real only when $r \geq 3$ (confirming that the period-2 orbit appears at $r = 3$, as we noted from the bifurcation diagram).

---

## Fixed Points in High-Dimensional Reservoir States

For reservoir computing, the "fixed points" of interest are not just mathematical curiosities — they are the rest states of the network. In the absence of input, a reservoir with a single stable fixed point will relax to that fixed point regardless of its initial state. This has direct consequences for memory: a reservoir that forgets its initial condition exponentially fast may also forget the input it received.

The **echo state property** (Section 7, and the central topic of Chapter 4) requires that the reservoir eventually forgets its initial state — which is precisely the statement that the autonomous dynamics contracts toward the input-driven trajectory. Understanding this contraction requires knowing the stability of the fixed points, and in higher-dimensional cases, the contraction of the entire phase space volume (Lyapunov exponents again).

The search for fixed points, and the analysis of what happens near them, is thus not just theoretical housekeeping. It is a practical diagnostic tool for reservoir design.

---

## Summary

Fixed points of a dynamical system are states where time evolution halts. In continuous time, they are zeros of the vector field $f(\mathbf{x}^*) = 0$; in discrete time, solutions of $f(x^*) = x^*$. We find them analytically (solving the equations), graphically (nullclines for continuous 2D systems, cobweb diagrams for 1D maps), and numerically (Newton's method). Period-$k$ orbits are fixed points of the $k$-fold iterate $f^k$. In the next section, we ask what happens to trajectories that start near a fixed point: do they converge to it, diverge from it, or oscillate? This is the stability question, and it is answered by the Jacobian matrix.
