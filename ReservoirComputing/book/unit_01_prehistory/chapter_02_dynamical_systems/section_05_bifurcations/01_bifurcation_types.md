# 2.5 Bifurcations: How Systems Change Character

## The Qualitative Change

A bifurcation is a moment of qualitative change in a dynamical system's behavior as a parameter is varied. Before the bifurcation, the system has one character — perhaps a single stable fixed point. After, it has another — perhaps two fixed points, or a limit cycle, or chaos. At the bifurcation value itself, the system stands at a threshold, structurally unstable, on the boundary between two regimes.

Bifurcations matter for reservoir computing because the spectral radius $\rho$ and input scaling $\sigma_{\text{in}}$ are parameters that control the reservoir's qualitative dynamics. Choosing $\rho = 0.9$ versus $\rho = 1.1$ is not just a quantitative change in performance — it can be a qualitative change in the reservoir's dynamical regime. Understanding bifurcations is what allows you to understand *why* a small change in $\rho$ can dramatically alter reservoir behavior.

## The One-Parameter Family

We study **one-parameter families** of dynamical systems:

$$\dot{\mathbf{x}} = f(\mathbf{x}; \mu) \quad \text{or} \quad \mathbf{x}_{t+1} = f(\mathbf{x}_t; \mu)$$

where $\mu \in \mathbb{R}$ is a control parameter. For each fixed $\mu$, we have a specific dynamical system. As $\mu$ varies, the qualitative behavior changes.

**Definition.** A **bifurcation** occurs at $\mu = \mu_0$ if the qualitative structure of the phase portrait (the topology of the attractor) changes as $\mu$ passes through $\mu_0$.

The formal theory of bifurcations classifies these qualitative changes and shows that, for generic one-parameter families, only a small number of "normal forms" occur. We study the four most important.

## Saddle-Node Bifurcation

The **saddle-node** (or fold) bifurcation is the generic mechanism for the creation and destruction of fixed points.

**Normal form (1D continuous):**
$$\dot{x} = \mu - x^2$$

**Analysis:**
- Fixed points: $x^* = \pm\sqrt{\mu}$, which exist only for $\mu \geq 0$.
- For $\mu > 0$: two fixed points, $x^* = +\sqrt{\mu}$ (stable) and $x^* = -\sqrt{\mu}$ (unstable).
- For $\mu = 0$: one fixed point at $x^* = 0$ (half-stable).
- For $\mu < 0$: no fixed points. Trajectories flow to $-\infty$.

The bifurcation at $\mu = 0$ creates a pair of fixed points (one stable, one unstable) from nothing. This is the **saddle-node bifurcation** — the stable node and the saddle (in 2D generalizations) collide and annihilate as $\mu$ decreases through 0, or are born as $\mu$ increases through 0.

**Bifurcation diagram:** Plot $x^*$ versus $\mu$. The curve $x^* = \pm\sqrt{\mu}$ forms a parabola opening to the right. The upper branch is stable (solid line), the lower branch is unstable (dashed line). The two branches meet at the bifurcation point $(\mu, x^*) = (0, 0)$.

## Transcritical Bifurcation

The **transcritical bifurcation** occurs when two fixed points exchange stability rather than collide and annihilate.

**Normal form:**
$$\dot{x} = \mu x - x^2 = x(\mu - x)$$

**Analysis:**
- Fixed points: $x^* = 0$ (always) and $x^* = \mu$ (always).
- For $\mu < 0$: $x^* = 0$ is stable; $x^* = \mu < 0$ is unstable.
- For $\mu > 0$: $x^* = 0$ is unstable; $x^* = \mu > 0$ is stable.
- At $\mu = 0$: the two fixed points coincide at $x^* = 0$, and stability exchanges.

This bifurcation often appears in population dynamics (where $x^* = 0$ is the extinction equilibrium) and in systems with a symmetry that prevents fixed point collision.

## Pitchfork Bifurcation

The **pitchfork bifurcation** arises in systems with a reflection symmetry $x \to -x$. It is the mechanism by which symmetry is broken.

**Normal form (supercritical):**
$$\dot{x} = \mu x - x^3$$

**Analysis:**
- Fixed points: $x^* = 0$ (always) and $x^* = \pm\sqrt{\mu}$ (for $\mu > 0$).
- For $\mu < 0$: only $x^* = 0$, which is stable.
- For $\mu > 0$: $x^* = 0$ is unstable; $x^* = \pm\sqrt{\mu}$ are stable.

At $\mu = 0$, a single stable fixed point bifurcates into an unstable fixed point and two stable ones — the "tines" of the pitchfork. The system has broken its reflection symmetry: it must choose $+\sqrt{\mu}$ or $-\sqrt{\mu}$.

**Subcritical pitchfork:**
$$\dot{x} = \mu x + x^3$$

Here the bifurcation is destabilizing: two unstable fixed points $x^* = \pm\sqrt{-\mu}$ exist for $\mu < 0$ and disappear at $\mu = 0$, leaving only the unstable origin. The system then jumps to a distant attractor — a subcritical transition associated with hysteresis.

## Period-Doubling Route to Chaos

The period-doubling cascade is one of the most beautiful phenomena in nonlinear dynamics, and it is the route to chaos most relevant to reservoir computing (via the logistic map).

Consider the logistic map $x_{t+1} = r x_t(1 - x_t)$ with parameter $r$:

| $r$ range | Behavior |
|-----------|----------|
| $0 < r < 1$ | Stable fixed point $x^* = 0$ |
| $1 < r < 3$ | Stable fixed point $x^* = 1 - 1/r$ |
| $3 < r < 3.449...$ | Stable period-2 cycle |
| $3.449... < r < 3.544...$ | Stable period-4 cycle |
| $\vdots$ | $\vdots$ |
| $r < r_\infty \approx 3.5699...$ | Period $2^n$ cycles for all $n$ |
| $r > r_\infty$ | Chaos (with periodic windows) |

Each period-doubling occurs when a periodic orbit loses stability and a new orbit of twice the period is born. This happens through a **period-doubling bifurcation** (also called a **flip bifurcation**): one of the Floquet multipliers of the orbit crosses $-1$ (not $+1$ as in the saddle-node).

**The Feigenbaum constant:** Let $r_n$ be the value of $r$ at which the period-$2^n$ cycle appears. Then the successive bifurcations accumulate geometrically:

$$\delta = \lim_{n \to \infty} \frac{r_n - r_{n-1}}{r_{n+1} - r_n} \approx 4.6692...$$

This number — the **Feigenbaum constant** $\delta$ — is universal [Feigenbaum1978]: it appears in the period-doubling cascade of any smooth unimodal map, not just the logistic map. It is one of the deep universal constants of nonlinear dynamics.

**Bifurcation diagram:** The bifurcation diagram of the logistic map plots the attractor as a function of $r$. For small $r$, it is a single point (fixed point). It splits at $r = 3$ (first period-doubling), splits again, and again, until the chaotic band appears at $r_\infty$. The fractal self-similarity of the chaotic region reflects the universality of the Feigenbaum constant.

## Bifurcations in Reservoir Computing

The spectral radius $\rho$ of a reservoir plays the role of the bifurcation parameter:

- **$\rho \ll 1$:** All reservoir trajectories converge rapidly to the fixed point $\mathbf{x}^* = 0$. Minimal memory, minimal dynamics. The reservoir is too stable to be useful.

- **$\rho \lesssim 1$:** The reservoir has a unique stable fixed point for zero input, but responds richly to input perturbations. This is the most useful regime for most tasks.

- **$\rho = 1$ (without input):** The reservoir is at the boundary of stability — a Neimark-Sacker bifurcation (the discrete-time analog of Hopf) is imminent.

- **$\rho > 1$ (without input):** The reservoir is unstable — trajectories diverge from the fixed point. With bounded input, the nonlinearity (tanh saturation) can stabilize the dynamics, but the echo state property may not hold.

- **$\rho \gg 1$:** The reservoir dynamics are chaotic even without input. Maximum expressiveness, but the echo state property is violated: two different initial conditions produce different states even after long input sequences, undermining the training procedure.

The edge of stability at $\rho \approx 1$ is the reservoir's critical point — the analog of the phase transition between order and chaos that Bertschinger and Natschläger [Bertschinger2004] showed maximizes information processing capacity.

---

## References

- [Feigenbaum1978] Feigenbaum, M.J. (1978). Quantitative universality for a class of nonlinear transformations. *Journal of Statistical Physics*, 19(1), 25–52. **[The discovery of the Feigenbaum constant.]**
- [Strogatz2018] Strogatz, S.H. (2018). *Nonlinear Dynamics and Chaos*, 2nd ed. CRC Press. Chapters 3 and 8 cover all bifurcation types with extensive worked examples.
- [Guckenheimer1983] Guckenheimer, J. & Holmes, P. (1983). *Nonlinear Oscillations, Dynamical Systems, and Bifurcations of Vector Fields*. Springer. The rigorous reference.
- [Bertschinger2004] Bertschinger, N. & Natschläger, T. (2004). Real-time computation at the edge of chaos in recurrent neural networks. *Neural Computation*, 16(7), 1413–1436.
- [Kuznetsov1995] Kuznetsov, Y.A. (1995). *Elements of Applied Bifurcation Theory*. Springer. The comprehensive modern reference.
