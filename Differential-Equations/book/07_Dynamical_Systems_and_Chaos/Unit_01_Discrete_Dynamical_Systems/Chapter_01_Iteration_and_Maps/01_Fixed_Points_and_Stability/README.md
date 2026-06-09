# Fixed Points and Stability

Consider the iteration $x_{n+1} = f(x_n)$ with $f: I \to I$ a smooth function on an interval $I \subset \mathbb{R}$. If one starts the iteration at a fixed point $x^*$ satisfying $f(x^*) = x^*$, the orbit is trivial: $x_n = x^*$ for all $n \geq 0$. The interesting and practically important question is what happens when the initial condition $x_0$ is close to, but not exactly at, a fixed point. Does the orbit return to $x^*$, or does it drift away? The answer is one of the first and most important theorems in the subject.

## Definitions

Let $f: I \to I$ be a function. A point $x^* \in I$ is a **fixed point** of $f$ if $f(x^*) = x^*$.

A fixed point $x^*$ is called:
- **Lyapunov stable** if for every $\varepsilon > 0$ there exists $\delta > 0$ such that $|x_0 - x^*| < \delta$ implies $|f^n(x_0) - x^*| < \varepsilon$ for all $n \geq 0$.
- **Asymptotically stable** (or **attracting**) if it is Lyapunov stable and there exists $\delta > 0$ such that $|x_0 - x^*| < \delta$ implies $\lim_{n \to \infty} f^n(x_0) = x^*$.
- **Unstable** if it is not Lyapunov stable.

The **basin of attraction** of an attracting fixed point $x^*$ is the set of all $x_0$ for which $f^n(x_0) \to x^*$.

## The Linearization Theorem

**Theorem (Linearization at a Fixed Point).** Let $f: I \to I$ be $C^1$ on an open interval $I$, and let $x^*$ be a fixed point of $f$. Set $\lambda = f'(x^*)$.

1. If $|\lambda| < 1$, then $x^*$ is asymptotically stable.
2. If $|\lambda| > 1$, then $x^*$ is unstable.
3. If $|\lambda| = 1$, the theorem gives no information.

**Proof of part 1.** Let $|\lambda| < 1$. Choose $\varepsilon > 0$ such that $\lambda + \varepsilon < 1$. Since $f'$ is continuous and $f'(x^*) = \lambda$, there exists $\delta > 0$ such that $|f'(x)| \leq \lambda + \varepsilon =: \mu < 1$ for all $x \in (x^* - \delta, x^* + \delta)$.

For any $x_0$ in this interval, the mean value theorem gives

$$|x_1 - x^*| = |f(x_0) - f(x^*)| = |f'(\xi_0)| \cdot |x_0 - x^*| \leq \mu |x_0 - x^*|$$

for some $\xi_0$ between $x_0$ and $x^*$. Since $|x_1 - x^*| \leq \mu |x_0 - x^*| < |x_0 - x^*| < \delta$, the point $x_1$ lies in the same interval, and the argument repeats. By induction,

$$|x_n - x^*| \leq \mu^n |x_0 - x^*| \to 0$$

as $n \to \infty$. This establishes both Lyapunov stability and asymptotic stability. $\square$

**Proof of part 2.** Let $|\lambda| > 1$. Choose $\varepsilon > 0$ so that $|\lambda| - \varepsilon > 1$. By continuity of $f'$, there is $\delta > 0$ such that $|f'(x)| \geq |\lambda| - \varepsilon =: \nu > 1$ for $x$ near $x^*$. For $x_0 \neq x^*$ close to $x^*$,

$$|x_1 - x^*| \geq \nu |x_0 - x^*| > |x_0 - x^*|,$$

so orbits initially depart from $x^*$. A more careful argument shows that orbits leave any fixed neighborhood of $x^*$, establishing instability. $\square$

## Classification by Derivative

The behavior near a fixed point depends on the sign and magnitude of $\lambda = f'(x^*)$:

- $0 < \lambda < 1$: **Attracting, monotone.** The orbit approaches $x^*$ from one side, with each term closer than the last.
- $-1 < \lambda < 0$: **Attracting, oscillatory.** The orbit alternates sides of $x^*$ while converging.
- $\lambda = 0$: **Super-attracting.** Convergence is faster than geometric; $|x_n - x^*| = O(\mu^{2^n})$ for some $\mu < 1$.
- $\lambda > 1$: **Repelling, monotone.** Orbits diverge from $x^*$ without oscillation.
- $\lambda < -1$: **Repelling, oscillatory.** Orbits diverge while alternating sides.

## Worked Example: The Logistic Map

Consider $f_r(x) = rx(1-x)$ on $[0,1]$. The fixed points satisfy $rx(1-x) = x$, giving $x = 0$ and $x^* = 1 - 1/r$ (the latter in $[0,1]$ only when $r \geq 1$).

**Stability of $x = 0$:** We compute $f_r'(x) = r(1 - 2x)$, so $f_r'(0) = r$. Thus $x = 0$ is attracting when $r < 1$ and repelling when $r > 1$.

**Stability of $x^* = 1 - 1/r$ (for $r > 1$):** We compute

$$f_r'(x^*) = r\left(1 - 2\left(1 - \frac{1}{r}\right)\right) = r\left(\frac{2}{r} - 1\right) = 2 - r.$$

So $|f_r'(x^*)| = |2 - r|$. This is less than 1 when $1 < 2 - r$ or $2 - r > -1$, i.e., when $1 < r < 3$. Thus:
- For $1 < r < 3$: $x^*$ is asymptotically stable.
- At $r = 3$: $f_r'(x^*) = -1$, the borderline case.
- For $r > 3$: $x^*$ is unstable.

The transition at $r = 3$ is the birth of the period-doubling cascade.

## The Nonhyperbolic Case: $|\lambda| = 1$

When $\lambda = f'(x^*) = 1$ or $\lambda = -1$, stability is determined by higher-order terms. The case $\lambda = 1$ is particularly subtle. Consider $f(x) = x + ax^k + O(x^{k+1})$ near $x^* = 0$ (after translating). If $a \neq 0$ and $k$ is odd, then $x^*$ is semi-stable (attracting from one side, repelling from the other), which is itself unstable by definition. If $k$ is even, instability follows for any $a \neq 0$. The case $\lambda = -1$ arises at the onset of period doubling and is analyzed by studying the iterate $f^2$.

## The Basin of Attraction

Even when a fixed point is asymptotically stable, its basin of attraction may be complicated. For the logistic map with $1 < r < 3$, the entire interval $(0,1)$ lies in the basin of $x^* = 1 - 1/r$. But for maps with multiple fixed points or for higher-dimensional maps, basins can be fractal sets with intricate boundaries. Determining the global extent of a basin requires analysis beyond linearization: Lyapunov functions, invariant intervals, or numerical exploration.

## Connection to Differential Equations

The stability theory here mirrors exactly the theory for equilibria of autonomous ODEs $\dot{x} = g(x)$: an equilibrium $x^* = 0$ of the linearization $\dot{x} = g'(0)x$ is stable when $g'(0) < 0$ and unstable when $g'(0) > 0$. The map analogue replaces $g'(0)$ with $f'(x^*)$ and $\text{Re}(\lambda) < 0$ with $|\lambda| < 1$. This correspondence is not a coincidence: the time-1 map of the flow of $\dot{x} = g(x)$ is a discrete map whose derivative at the fixed point is $e^{g'(x^*)}$, and $|e^s| < 1 \iff \text{Re}(s) < 0$.
