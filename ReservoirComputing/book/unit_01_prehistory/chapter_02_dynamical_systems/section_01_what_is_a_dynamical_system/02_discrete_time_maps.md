# Section 1.2: Discrete-Time Dynamical Systems

## Definition

A **discrete-time dynamical system** (also called an **iterated map**) on a state space $X \subseteq \mathbb{R}^n$ is defined by a function $f: X \to X$ together with an iteration rule:

$$x_{t+1} = f(x_t), \qquad x_0 \in X, \qquad t = 0, 1, 2, \ldots \tag{2.1}$$

Here time $t$ is a non-negative integer. The sequence $x_0, x_1 = f(x_0), x_2 = f(f(x_0)), \ldots$ is the **orbit** or **trajectory** of the initial condition $x_0$.

We write $f^t$ for the $t$-fold composition of $f$ with itself:

$$f^0 = \text{id}, \qquad f^1 = f, \qquad f^2 = f \circ f, \qquad f^t = \underbrace{f \circ f \circ \cdots \circ f}_{t \text{ times}}$$

So $x_t = f^t(x_0)$.

The map $f$ plays the same conceptual role as the flow map $\Phi^t$ in continuous time — it moves points through the state space. But here we are choosing a fixed time step (normalized to 1) and asking only how states map to states after one step. There is no smoothness requirement in $t$: the dynamics is inherently discrete.

---

## Why Discrete Time Is Natural for Reservoir Computing

Before diving into examples, a word on why discrete-time maps deserve careful attention in this context.

Most practical reservoir computing implementations are discrete-time. The input arrives as a sequence $u_0, u_1, u_2, \ldots$ — perhaps sampled audio, financial time series, or text. The reservoir is updated once per input sample:

$$x_{t+1} = f(W^{\text{res}} x_t + W^{\text{in}} u_t)$$

This is exactly a discrete-time dynamical system, driven by an input. The map $f$ is typically a sigmoidal nonlinearity applied componentwise; $W^{\text{res}}$ and $W^{\text{in}}$ are matrices. Understanding the properties of iterated maps — fixed points, periodic orbits, sensitivity to initial conditions, and how these change as parameters vary — is directly applicable to analyzing reservoir behavior.

Moreover, many of the deepest theoretical results about reservoir computing (the echo state property, fading memory, separation property) are stated most cleanly in discrete time. Continuous-time reservoirs are important, but discrete time is where the theory is easiest to state precisely.

---

## The Logistic Map

The **logistic map** is defined by

$$x_{t+1} = r x_t (1 - x_t) \tag{2.2}$$

where $x_t \in [0, 1]$ is a normalized population density and $r \in [0, 4]$ is a growth rate parameter. When $r \leq 4$, the map sends $[0,1]$ to $[0,1]$, so the state space is the unit interval.

The logistic map is the discrete-time analogue of the continuous logistic growth equation $\dot{x} = rx(1-x)$, but it is profoundly richer. Despite being defined by a single quadratic equation with one free parameter, it displays the full spectrum of dynamical behavior: stable fixed points, stable periodic orbits of every period, and chaos. It became the canonical example of complex dynamics arising from simple rules after Feigenbaum's work on period doubling [Feigenbaum1978], and May's influential 1976 paper [May1976] introduced it to the broader scientific community as a cautionary lesson for anyone who thought discrete-time ecological models would be simple.

### Fixed Points

A fixed point satisfies $f(x^*) = x^*$, i.e., $r x^* (1 - x^*) = x^*$. This gives $x^*(r(1-x^*) - 1) = 0$, so either

$$x^* = 0 \quad \text{or} \quad x^* = 1 - \frac{1}{r}$$

The second fixed point $x^* = 1 - 1/r$ is in $[0,1]$ only when $r \geq 1$. For $1 < r < 3$, this fixed point is stable: orbits starting near it converge to it. For $r < 1$, the only fixed point in $[0,1]$ is $x^* = 0$, and the population goes extinct.

### Period Doubling

At $r = 3$, the fixed point $x^* = 1 - 1/r$ loses stability (we will see exactly why in Section 2 on eigenvalue stability: the derivative of $f$ at $x^*$ passes through $-1$). For $r$ slightly above 3, the system settles into a **period-2 orbit**: a pair of points $\{x_a, x_b\}$ such that $f(x_a) = x_b$ and $f(x_b) = x_a$.

As $r$ increases further, this period-2 orbit loses stability at $r \approx 3.449$ and gives way to a period-4 orbit. Then a period-8 orbit at $r \approx 3.544$. The period doubles again and again, at values $r_1, r_2, r_3, \ldots$ that converge geometrically:

$$\lim_{n \to \infty} \frac{r_n - r_{n-1}}{r_{n+1} - r_n} = \delta \approx 4.6692\ldots$$

This is the **Feigenbaum constant** $\delta$. The remarkable fact, discovered by Feigenbaum in 1978 [Feigenbaum1978] and proven rigorously by Lanford [Lanford1982] and others, is that *every* smooth unimodal map undergoes period doubling with the same ratio $\delta$. It is a universal constant of nonlinear dynamics, as universal as $\pi$ in a different domain.

At $r_\infty \approx 3.5699\ldots$, the period has doubled infinitely many times. Above this value, for most values of $r$, the orbit is **chaotic**: it visits each point in a Cantor-set-like subset of $[0,1]$ without ever repeating, and nearby initial conditions diverge exponentially.

### The Bifurcation Diagram

The most striking visualization of the logistic map is its **bifurcation diagram**: for each $r$ along the horizontal axis, plot the long-term values that $x_t$ visits on the vertical axis. For $r < 3$, you see a single curve — the stable fixed point. At $r = 3$, this branches into two curves (period-2 orbit). Each branch bifurcates again at its own critical $r$. For $r > r_\infty$, you see a dense cloud of points — the chaotic attractor — punctuated by narrow windows of periodic behavior where chaos temporarily resolves into a periodic orbit before becoming chaotic again.

The bifurcation diagram is a map of the logistic map's soul. It encodes, in a single image, all the possible long-term behaviors of the system as the growth rate $r$ is varied. We return to bifurcation diagrams in detail in Section 5.

---

## Cobweb Diagrams

For 1D maps, there is a beautiful graphical method for visualizing iteration called the **cobweb diagram**.

To draw the cobweb for the map $x_{t+1} = f(x_t)$, starting from $x_0$:

1. Plot $y = f(x)$ and $y = x$ (the diagonal) on the same axes.
2. From the point $(x_0, x_0)$ on the diagonal, draw a vertical line to $(x_0, f(x_0)) = (x_0, x_1)$.
3. From $(x_0, x_1)$, draw a horizontal line to the diagonal: $(x_1, x_1)$.
4. Repeat: vertical to the curve, horizontal to the diagonal.

The resulting "cobweb" pattern traces out the orbit $x_0, x_1, x_2, \ldots$ graphically.

The cobweb diagram makes several things immediately visible:

- If $f(x^*)= x^*$ and $|f'(x^*)| < 1$, the cobweb spirals inward toward $x^*$ (stable fixed point).
- If $|f'(x^*)| > 1$, the cobweb spirals outward away from $x^*$ (unstable fixed point).
- If the orbit is periodic with period 2, the cobweb traces a rectangle bouncing between $x_a$ and $x_b$.
- For chaotic orbits, the cobweb is a tangled, non-repeating path.

The cobweb diagram is not just a pedagogical tool — it provides intuition for stability analysis that carries over to higher-dimensional systems.

---

## Beyond One Dimension: Hénon and Beyond

The logistic map is 1D, but reservoir states are high-dimensional. Let us briefly consider a 2D example.

The **Hénon map** [Henon1976] is defined by:

$$x_{t+1} = 1 - a x_t^2 + y_t \tag{2.3a}$$
$$y_{t+1} = b x_t \tag{2.3b}$$

For the canonical parameters $a = 1.4$, $b = 0.3$, the Hénon map has a **strange attractor** — a fractal set in $\mathbb{R}^2$ to which almost all initial conditions in a large region converge. The attractor is self-similar under magnification: zooming in reveals an infinite regression of parallel curves, a Cantor-set structure in the transverse direction and a smooth structure along the orbit direction.

The Hénon map is invertible (it has nonzero Jacobian determinant $-b$), so it defines a genuine homeomorphism of $\mathbb{R}^2$. Its dynamics are chaotic, with a positive Lyapunov exponent (Section 4.3). The strange attractor of the Hénon map has a Hausdorff dimension of approximately 1.261 [Russell1980], intermediate between 1 (a curve) and 2 (a surface).

This notion — an attractor with non-integer dimension — is the heart of the concept of a **strange attractor**, which we develop fully in Section 4.

---

## Connecting Continuous and Discrete Time

Discrete and continuous dynamical systems are not separate universes. There are several important connections:

**Euler discretization.** The simplest connection: given $\dot{x} = f(x)$, the forward Euler method with step size $h$ gives $x_{t+1} = x_t + h f(x_t)$, a map. This approximation can behave very differently from the ODE when $h$ is not small — the discrete system can undergo bifurcations that the continuous system does not have [Stuart1994].

**The Poincaré section.** A more principled connection: given a continuous-time system on $\mathbb{R}^n$, choose a hypersurface $\Sigma$ (a "section") that trajectories cross transversally. Each time a trajectory crosses $\Sigma$, record the crossing point. The resulting sequence of points defines a discrete-time map on $\Sigma$, called the **Poincaré map** $P: \Sigma \to \Sigma$. Fixed points of $P$ correspond to periodic orbits of the flow; period-$k$ orbits of $P$ correspond to resonant periodic orbits. This reduction from continuous to discrete is a key analytical tool.

**Sampling continuous-time reservoirs.** In practice, a continuous-time reservoir governed by an ODE (like the liquid state machine or rate-coded neural field) is read out at discrete sampling times. The sampled sequence of states $x(t_0), x(t_1), x(t_2), \ldots$ is the relevant object for the downstream readout layer. The properties of this sampled sequence — whether it faithfully encodes input history — depend on the interplay between the reservoir's continuous-time dynamics and the sampling rate.

---

## Summary

Discrete-time maps $x_{t+1} = f(x_t)$ are the natural language for reservoir computing in practice. The logistic map illustrates how extraordinary complexity — period doubling, chaos, universal constants — can emerge from a single quadratic function. Cobweb diagrams provide geometric intuition for convergence and divergence near fixed points. The Hénon map shows that 2D maps can have strange attractors with fractal geometry. Poincaré sections bridge the continuous and discrete pictures.

In Section 2, we develop the stability theory that lets us classify fixed points and determine whether nearby trajectories converge to or diverge from them.
