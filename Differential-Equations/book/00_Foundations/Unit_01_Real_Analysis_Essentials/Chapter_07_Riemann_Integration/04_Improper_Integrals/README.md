# Improper Integrals

The Riemann integral $\int_a^b f(x)\,dx$ is defined for bounded functions on bounded, closed intervals. Many situations require integrating over infinite intervals ($\int_1^\infty e^{-x}\,dx$) or integrating functions with singularities within the interval ($\int_0^1 x^{-1/2}\,dx$). These **improper integrals** are defined as limits of proper Riemann integrals, and their convergence or divergence is determined by the rate at which the integrand decays or blows up.

## Type I: Infinite Intervals

**Definition.** If $f$ is integrable on $[a, R]$ for every $R > a$, define
$$\int_a^\infty f(x)\,dx = \lim_{R\to\infty} \int_a^R f(x)\,dx,$$
provided the limit exists and is finite. Similarly, $\int_{-\infty}^b f = \lim_{R\to\infty}\int_{-R}^b f$ and $\int_{-\infty}^\infty f = \int_{-\infty}^c f + \int_c^\infty f$ for any $c$ (both parts must converge separately).

**Example.** $\int_1^\infty \frac{1}{x^p}\,dx$.

For $p \neq 1$: $\int_1^R x^{-p}\,dx = \frac{R^{1-p}-1}{1-p}$. As $R\to\infty$: if $p > 1$, $R^{1-p} \to 0$, giving $\frac{1}{p-1}$; if $p < 1$, $R^{1-p}\to\infty$, diverges.

For $p = 1$: $\int_1^R \frac{1}{x}\,dx = \ln R \to \infty$.

Conclusion: $\int_1^\infty x^{-p}\,dx$ converges iff $p > 1$.

## Type II: Singularities at Endpoints

**Definition.** If $f$ is integrable on $[a+\varepsilon, b]$ for every $\varepsilon > 0$ but $f$ is not bounded near $a$, define
$$\int_a^b f(x)\,dx = \lim_{\varepsilon\to 0^+} \int_{a+\varepsilon}^b f(x)\,dx,$$
if the limit exists.

**Example.** $\int_0^1 x^{-p}\,dx$ for $p > 0$.

$\int_\varepsilon^1 x^{-p}\,dx = \frac{1-\varepsilon^{1-p}}{1-p}$ for $p\neq 1$. As $\varepsilon\to 0^+$: if $p < 1$, $\varepsilon^{1-p} \to 0$, giving $\frac{1}{1-p}$; if $p > 1$, $\varepsilon^{1-p}\to\infty$, diverges. For $p = 1$: $\int_\varepsilon^1 \frac{1}{x}\,dx = -\ln\varepsilon \to\infty$.

Conclusion: $\int_0^1 x^{-p}\,dx$ converges iff $p < 1$.

Note the complement to the Type I result: large exponent $p > 1$ is needed for convergence at infinity, while small exponent $p < 1$ is needed near a singularity at $0$.

## Convergence Tests

**Comparison Test.** If $0 \leq f(x) \leq g(x)$ for $x \geq a$ and $\int_a^\infty g$ converges, then $\int_a^\infty f$ converges. If $\int_a^\infty f$ diverges, so does $\int_a^\infty g$.

**Limit Comparison Test.** If $f, g \geq 0$ and $\lim_{x\to\infty} f(x)/g(x) = L$ with $0 < L < \infty$, then $\int_a^\infty f$ and $\int_a^\infty g$ converge or diverge together.

**Example.** Does $\int_1^\infty \frac{x}{\sqrt{x^4+1}}\,dx$ converge?

For large $x$: $\frac{x}{\sqrt{x^4+1}} \approx \frac{x}{x^2} = \frac{1}{x}$. Limit comparison with $1/x$: $\frac{x/\sqrt{x^4+1}}{1/x} = \frac{x^2}{\sqrt{x^4+1}} \to 1$. Since $\int_1^\infty 1/x\,dx$ diverges, so does the given integral.

**Absolute Convergence.** The integral $\int_a^\infty f$ converges **absolutely** if $\int_a^\infty |f|$ converges. Absolute convergence implies convergence (but not vice versa).

## The Laplace Transform

The Laplace transform of a function $f: [0,\infty) \to \mathbb{R}$ is defined by the improper integral
$$\mathcal{L}\{f\}(s) = \int_0^\infty e^{-st}f(t)\,dt,$$
for those values of $s$ for which the integral converges.

**Example.** $\mathcal{L}\{1\}(s) = \int_0^\infty e^{-st}\,dt = \lim_{R\to\infty}\left[-\frac{e^{-st}}{s}\right]_0^R = \frac{1}{s}$ for $s > 0$.

**Example.** $\mathcal{L}\{e^{at}\}(s) = \int_0^\infty e^{(a-s)t}\,dt = \frac{1}{s-a}$ for $s > a$.

**Convergence.** If $|f(t)| \leq Me^{ct}$ (exponential growth bound), then $\mathcal{L}\{f\}(s)$ converges for $s > c$ (since $\int_0^\infty M e^{(c-s)t}\,dt$ converges for $s > c$). The minimal $c$ for which this holds is the **abscissa of convergence**.

## Gamma and Beta Functions

The **Gamma function** extends the factorial:
$$\Gamma(s) = \int_0^\infty t^{s-1}e^{-t}\,dt, \quad s > 0.$$
Integration by parts gives $\Gamma(s+1) = s\Gamma(s)$, and $\Gamma(1) = 1$, so $\Gamma(n+1) = n!$ for positive integers. The Gamma function appears in the solution of Bessel's equation and other ODEs with power-law singularities.

The **Beta function** $B(p,q) = \int_0^1 t^{p-1}(1-t)^{q-1}\,dt = \Gamma(p)\Gamma(q)/\Gamma(p+q)$ arises in special function theory.

## Comparison with Series

Improper integrals and infinite series have parallel convergence theories. The integral test (Chapter 4) connects $\sum a_k$ to $\int a(x)\,dx$. Both require comparison and limit comparison tests. In both cases, the $p$-test gives: convergence iff $p > 1$ (at infinity or in a sum), or $p < 1$ (at a singularity/near $0$ in a sum of terms $1/k^p$ converging vs. $\int_0^1 x^{-p}$).

## Common Pitfalls

**Substituting the limit without verifying convergence.** $\int_0^\infty e^{-st}f(t)\,dt$ requires a check that $e^{-st}f(t)$ actually integrates — $f$ might grow too fast for small $s$.

**Cauchy principal value.** $\int_{-1}^1 \frac{1}{x}\,dx$ is sometimes assigned the "Cauchy principal value" $\lim_{\varepsilon\to 0^+}\left(\int_{-1}^{-\varepsilon} \frac{1}{x}\,dx + \int_\varepsilon^1 \frac{1}{x}\,dx\right) = 0$, but this is not the improper integral, which diverges. The distinction matters.

**Separating $\int_{-\infty}^\infty$ incorrectly.** $\int_{-\infty}^\infty f$ requires both $\int_{-\infty}^0 f$ and $\int_0^\infty f$ to converge separately. It is not valid to define it as $\lim_{R\to\infty}\int_{-R}^R f$ (which is the Cauchy principal value).
