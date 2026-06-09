# Definition and Existence Conditions

The Laplace transform converts a function of time $t \geq 0$ into a function of a complex variable $s$, via the integral

$$\mathcal{L}\{f(t)\}(s) = \int_0^\infty e^{-st}f(t)\,dt = \lim_{T\to\infty}\int_0^T e^{-st}f(t)\,dt.$$

The parameter $s$ can be complex, but for most practical purposes in ODE applications, $s$ is taken real and sufficiently large. The transform is defined wherever this improper integral converges.

## Conditions for Existence

**Definition.** A function $f: [0, \infty) \to \mathbb{R}$ is **piecewise continuous** on $[0, T]$ for each $T > 0$ if it has at most finitely many jump discontinuities on each bounded interval, with finite one-sided limits at each discontinuity. It is **of exponential order $c$** if there exist constants $M > 0$ and $T \geq 0$ such that $|f(t)| \leq Me^{ct}$ for all $t \geq T$.

**Theorem.** If $f$ is piecewise continuous on $[0, \infty)$ and of exponential order $c$, then $\mathcal{L}\{f(t)\}(s)$ exists for all $s > c$ (or $\text{Re}(s) > c$ if $s$ is complex), and

$$|\mathcal{L}\{f\}(s)| \leq \frac{M}{s - c}.$$

**Proof sketch.** For $s > c$:

$$\int_0^\infty |e^{-st}f(t)|\,dt \leq \int_0^T e^{-st}|f(t)|\,dt + M\int_T^\infty e^{-st}e^{ct}\,dt = \text{bounded} + M\cdot\frac{e^{-(s-c)T}}{s-c} < \infty.$$

The first integral is finite because $f$ is bounded on $[0, T]$ (piecewise continuous), and the second converges for $s > c$.

## Examples of Existence

$f(t) = e^{t^2}$: not of exponential order (grows faster than any $e^{ct}$), so $\mathcal{L}\{e^{t^2}\}$ does not exist.

$f(t) = t^n$: of order $c$ for any $c > 0$ (since $t^n \leq M_c e^{ct}$ for large $t$). $\mathcal{L}\{t^n\}$ exists for $s > 0$.

$f(t) = \sin(t^2)$: bounded, hence of order 0. $\mathcal{L}\{\sin(t^2)\}$ exists for $s > 0$.

## Analyticity

For functions of exponential order $c$, the Laplace transform $F(s) = \int_0^\infty e^{-st}f(t)\,dt$ is an analytic function of the complex variable $s$ for $\text{Re}(s) > c$. One can differentiate under the integral sign:

$$F'(s) = \int_0^\infty (-t)e^{-st}f(t)\,dt = -\mathcal{L}\{tf(t)\}(s).$$

This is the differentiation-of-transform property, used in Chapter 2.

## Behavior as $s \to \infty$

The Riemann-Lebesgue lemma for Laplace transforms: if $\mathcal{L}\{f\}$ exists for $s > c$, then $F(s) \to 0$ as $s \to +\infty$ along the real axis. More precisely, $|F(s)| \leq M/(s-c) \to 0$. The initial value theorem states $\lim_{s\to\infty}sF(s) = f(0^+)$ (the right-hand limit at 0), and the final value theorem states $\lim_{s\to 0^+}sF(s) = \lim_{t\to\infty}f(t)$ (when the limit exists). These are useful checks and shortcuts in IVP solving.
