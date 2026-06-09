# Period Doubling

Period doubling is the mechanism by which a stable periodic orbit of period $n$ is replaced by a stable periodic orbit of period $2n$ as a parameter is varied. It is the most common route to chaos in one-dimensional maps and in many physical systems. This section analyzes the bifurcation in detail, establishes the normal form, and traces the cascade of doublings from period 1 to period 2 to period 4 and beyond.

## The Period-Doubling Bifurcation

Consider a one-parameter family of maps $f_r: \mathbb{R} \to \mathbb{R}$. Suppose that for $r < r_0$, the map has a stable fixed point $x^*(r)$ with multiplier $\lambda(r) = f_r'(x^*(r))$ satisfying $-1 < \lambda(r) < 0$. As $r$ increases through $r_0$, suppose $\lambda(r_0) = -1$.

At $r = r_0$, the fixed point has multiplier exactly $-1$. The linearization $x_{n+1} \approx x^* - (x_n - x^*)$ oscillates perfectly, returning every two steps. This is the borderline between stability and instability, and it is the signature of an impending period-doubling bifurcation.

**Normal Form.** Near the bifurcation, consider the iterate $f_r^2$ in a neighborhood of $x^*$. Since $f_r^2$ has $x^*$ as a fixed point with multiplier $(f_r'(x^*))^2 = \lambda(r)^2 \approx 1$ near $r_0$, the map $f_r^2$ is near the identity. Writing $g_r = f_r^2$ and $\mu = r - r_0$, the Taylor expansion of $g_r$ near $x^*$ takes the form

$$g_r(x) = x^* + (1 + a\mu)(x - x^*) + b(x - x^*)^3 + O((x-x^*)^4, \mu(x-x^*)^2),$$

where $a$ and $b$ are coefficients that depend on $f$ and its derivatives at the bifurcation point. (The quadratic term in $x - x^*$ is absent by a symmetry argument relating to the $\lambda = -1$ condition.)

The fixed points of $g_r$ (periodic-2 points of $f_r$) satisfy $g_r(x) = x$, which gives (after canceling $x - x^*$):

$$a\mu + b(x-x^*)^2 = 0 \implies (x - x^*)^2 = -\frac{a\mu}{b}.$$

Real solutions exist when $-a\mu/b > 0$. If $a > 0$ and $b < 0$, the period-2 orbit exists for $\mu > 0$ (i.e., $r > r_0$). The two new fixed points of $g_r$ are at $x^* \pm \sqrt{-a\mu/b}$, which grow from $x^*$ as $\sqrt{r - r_0}$ — a signature square-root scaling at a bifurcation.

## Stability of the Newborn Period-2 Orbit

The stability of the period-2 orbit is determined by $g_r'$ at the new fixed points. Differentiating:

$$g_r'(x) = 1 + a\mu + 3b(x-x^*)^2 + \cdots$$

At the period-2 points where $(x-x^*)^2 = -a\mu/b$:

$$g_r'(x^* \pm \sqrt{-a\mu/b}) = 1 + a\mu + 3b \cdot \left(-\frac{a\mu}{b}\right) = 1 + a\mu - 3a\mu = 1 - 2a\mu.$$

For $\mu$ small and positive with $a > 0$, this gives $g_r' = 1 - 2a\mu \in (0,1)$, confirming that the newborn period-2 orbit is asymptotically stable. The original fixed point, meanwhile, has $g_r'(x^*) = 1 + a\mu > 1$, confirming it has become unstable.

This exchange of stability—the fixed point losing stability exactly as the period-2 orbit is born and inherits stability—is the essence of period doubling.

## The Period-Doubling Cascade

The period-2 orbit inherits stability after the bifurcation at $r_1$. As $r$ increases further, the multiplier of the period-2 orbit (as a fixed point of $f_r^2$) decreases from near $+1$ toward $-1$. When it reaches $-1$ at $r = r_2$, the period-2 orbit undergoes its own period-doubling bifurcation, spawning a period-4 orbit. This orbit remains stable until $r = r_3$, when a period-8 orbit is born.

The sequence of bifurcation values $r_1, r_2, r_3, \ldots$ converges to a finite limit $r_\infty$. Computing for the logistic map:

| $k$ | $r_k$ | $r_k - r_{k-1}$ | Ratio |
|---|---|---|---|
| 1 | 3.000000 | — | — |
| 2 | 3.449490 | 0.449490 | — |
| 3 | 3.544090 | 0.094600 | 4.752 |
| 4 | 3.564407 | 0.020317 | 4.656 |
| 5 | 3.568759 | 0.004352 | 4.668 |
| 6 | 3.569692 | 0.000933 | 4.669 |

The ratios converge to $\delta \approx 4.669$. Beyond $r_\infty$, the orbit structure is dominated by chaos, though periodic windows exist.

## Symbolic Dynamics at the Cascade

Each period-doubling step can be tracked symbolically. For the logistic map, encode each iterate as $L$ if $x_n < 1/2$ and $R$ if $x_n > 1/2$. The fixed point $x^*$ has symbol sequence $R^\infty = RRRR\ldots$ The period-2 orbit has sequence $RL^\infty = RLRLRL\ldots$ The period-4 orbit has $RLRR^\infty = RLRRRLRR\ldots$ There is a recursive pattern: the symbolic sequence for the $2^k$-cycle is obtained from the $2^{k-1}$-cycle by a substitution rule, which is the symbolic manifestation of the renormalization.

## Physical Examples

The period-doubling cascade has been observed experimentally in:

- **Driven nonlinear oscillators:** As the driving amplitude increases, the response period doubles repeatedly before becoming chaotic.
- **Rayleigh-Benard convection:** A heated fluid layer transitions from periodic oscillation to chaos via period doublings as the temperature difference increases.
- **Electronic circuits:** Nonlinear circuits with driving signals exhibit period doubling sequences; the Feigenbaum ratio has been measured to several decimal places.
- **Heart cell models:** Mathematical models of cardiac tissue can exhibit period doubling as a precursor to fibrillation.

In all cases, the ratio of successive bifurcation intervals converges to $\delta \approx 4.669$, the same universal constant.

## Transition from Period Doubling to Chaos

At $r_\infty$, infinitely many period doublings have occurred. The orbit structure is a Cantor set: the attractor is nowhere dense, has Lebesgue measure zero, but is uncountable. This is the onset of chaos. For $r$ slightly above $r_\infty$, the Lyapunov exponent is zero (at the boundary of chaos and order); as $r$ increases further into the chaotic regime, the Lyapunov exponent increases, reaching $\log 2$ at $r = 4$.

The transition at $r_\infty$ is not a sharp boundary: for parameters in $(r_\infty, 4)$, the bifurcation diagram shows a mixture of chaos (large $r$ windows with positive Lyapunov exponent) and order (periodic windows with negative Lyapunov exponent). The measure of the chaotic parameter set grows from zero at $r_\infty$ to a value approaching the full measure of the interval as $r \to 4$.
