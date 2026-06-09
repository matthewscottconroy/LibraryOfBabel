# The Logistic Map

The logistic map $f_r(x) = rx(1-x)$ is simultaneously one of the simplest and one of the most instructive examples in all of dynamical systems. Introduced as a population model by Robert May in 1976, it depends on a single parameter $r$ and is defined on the unit interval $[0,1]$. Despite its quadratic simplicity, it exhibits the full range of dynamical behavior: stable equilibria, stable oscillations of every period, chaos, and the transition between them governed by universal constants. It is the canonical example through which virtually every concept of one-dimensional discrete dynamics is illustrated.

## Population Ecology Motivation

Suppose $x_n \in [0,1]$ represents the population of a species in year $n$, normalized so that $x = 1$ is the carrying capacity of the environment. The simplest model of unrestricted growth gives $x_{n+1} = rx_n$. With resource limitation, the growth rate is penalized when the population is large: $x_{n+1} = rx_n(1-x_n)$. The factor $(1-x_n)$ represents the fraction of unused resources. The parameter $r > 0$ is the intrinsic growth rate.

For this model to make biological sense, we need $f_r(x) \in [0,1]$ whenever $x \in [0,1]$. The maximum of $f_r$ on $[0,1]$ is attained at $x = 1/2$ and equals $r/4$. Thus $f_r: [0,1] \to [0,1]$ is well defined precisely when $r \leq 4$.

## Fixed Points and Stability Analysis

The fixed point equation $rx(1-x) = x$ gives $x = 0$ and $x^* = 1 - 1/r$ (defined in $[0,1]$ for $r \geq 1$). Computing the derivative: $f_r'(x) = r - 2rx = r(1 - 2x)$.

At $x = 0$: $f_r'(0) = r$. So $x = 0$ is stable for $r < 1$ and unstable for $r > 1$.

At $x^* = 1 - 1/r$ (for $r > 1$): $f_r'(x^*) = r(1 - 2(1-1/r)) = r(2/r - 1) = 2 - r$. So $|f_r'(x^*)| = |2-r|$, which is less than 1 if and only if $1 < r < 3$.

The full picture for small $r$:

| Parameter range | Attractor |
|---|---|
| $0 < r < 1$ | $x = 0$ |
| $1 < r < 3$ | $x^* = 1 - 1/r$ |
| $r = 3$ | bifurcation point |
| $r > 3$ | period-doubling cascade and chaos |

## The Period-Doubling Cascade

At $r = 3$, the fixed point $x^*$ has multiplier $f_r'(x^*) = -1$, and a period-2 orbit is born. The two period-2 points are

$$p, q = \frac{r + 1 \pm \sqrt{(r+1)(r-3)}}{2r}.$$

The period-2 orbit $\{p, q\}$ is stable for $3 < r < r_2 \approx 3.4495$ (specifically $r_2 = 1 + \sqrt{6}$), where it loses stability to a period-4 orbit. Subsequent period doublings occur at parameter values $r_n$ satisfying

$$\lim_{n \to \infty} \frac{r_n - r_{n-1}}{r_{n+1} - r_n} = \delta \approx 4.6692.$$

This convergence ratio is the first **Feigenbaum constant** and is universal across all unimodal maps (explained in Chapter 2).

The sequence $r_1 = 3 < r_2 \approx 3.4495 < r_3 \approx 3.5441 < \cdots$ accumulates at

$$r_\infty \approx 3.56995.$$

Beyond $r_\infty$, the dynamics are predominantly chaotic, though periodic windows persist.

## Dynamics at $r = 4$: Full Chaos

At $r = 4$, the logistic map is topologically conjugate to the tent map and to angle doubling on the circle. Specifically, the conjugacy $h(x) = \frac{2}{\pi}\arcsin(\sqrt{x})$ satisfies $h \circ f_4 = g \circ h$, where $g(\theta) = 2\theta \pmod{1}$ is angle doubling on $[0,1]$.

Using this conjugacy, one can prove:

- **Dense periodic orbits.** The periodic points of $f_4$ are dense in $[0,1]$.
- **Topological transitivity.** There exists an orbit that is dense in $[0,1]$, meaning for any open subinterval $U \subset [0,1]$, eventually some iterate of the dense orbit enters $U$.
- **Sensitive dependence.** There exists $\Delta > 0$ such that for every $x \in [0,1]$ and every $\varepsilon > 0$, there exists $y$ with $|x - y| < \varepsilon$ and $n$ such that $|f_4^n(x) - f_4^n(y)| > \Delta$.

A map satisfying all three conditions is called **chaotic** in the sense of Devaney. The logistic map at $r = 4$ is thus the canonical example of chaos on a bounded interval.

## Invariant Measure and Ergodicity

At $r = 4$, the logistic map preserves the probability measure $d\mu = \frac{dx}{\pi\sqrt{x(1-x)}}$, the arcsine distribution on $[0,1]$. Moreover, $f_4$ is **ergodic** with respect to this measure: for any measurable $A \subset [0,1]$,

$$\lim_{N \to \infty} \frac{1}{N} \sum_{n=0}^{N-1} \mathbf{1}_A(f_4^n(x)) = \mu(A)$$

for $\mu$-almost every $x$. This means that the proportion of time a typical orbit spends in any set $A$ equals the $\mu$-measure of $A$. The invariant measure $\mu$ is called the **natural measure** or **SRB measure** of the system.

The Lyapunov exponent at $r = 4$ can be computed exactly:

$$\lambda = \int_0^1 \log|f_4'(x)| \, d\mu(x) = \int_0^1 \log|4 - 8x| \cdot \frac{dx}{\pi\sqrt{x(1-x)}} = \log 2.$$

The positive Lyapunov exponent $\log 2 > 0$ confirms exponential sensitivity to initial conditions: nearby orbits diverge on average at rate $2^n$.

## The Bifurcation Diagram

The bifurcation diagram of the logistic map plots, for each $r$, the long-term attracting set (obtained by discarding transient behavior and plotting the subsequent orbit). The result is one of the most iconic images in mathematics:

- For $r < 3$: a single curve converging to $x^*(r)$.
- For $3 < r < r_2$: two branches, the period-2 orbit.
- For $r_2 < r < r_3$: four branches.
- Near $r_\infty$: a Cantor-like set of branches.
- For $r > r_\infty$: a dense cloud (chaos) interspersed with periodic windows.

The self-similarity of the diagram—zooming into any periodic window reveals a miniature copy of the entire diagram—is a visual manifestation of the universality theory.

## Connections to Other Systems

The logistic map's behavior is not an isolated curiosity. May's 1976 Nature paper showed that ecological models, even deterministic ones, can produce chaotic time series indistinguishable from noise. The Feigenbaum constants appear in experiments on driven Rayleigh-Benard convection cells, dripping faucets, and nonlinear electronic circuits. The universality of the period-doubling cascade means that the logistic map, despite its simplicity, genuinely captures the behavior of a broad class of physical systems.
