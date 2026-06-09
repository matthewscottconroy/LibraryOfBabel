# Convergence and Radius of Convergence

A power series $\sum_{n=0}^\infty a_n(x-x_0)^n$ represents a function defined wherever the series converges. The set of $x$ values for which convergence holds is always an interval (or a single point, or the entire real line), centered at $x_0$. Understanding this interval and how to determine its size is the first technical prerequisite for the series method in ODEs.

## Absolute Convergence

The series $\sum a_n(x-x_0)^n$ **converges absolutely** at $x$ if $\sum_{n=0}^\infty |a_n||x-x_0|^n < \infty$. Absolute convergence implies ordinary convergence. The ratio and root tests give sufficient conditions.

**Ratio test.** If $\lim_{n\to\infty}|a_{n+1}(x-x_0)^{n+1}| / |a_n(x-x_0)^n| = |x-x_0|\lim|a_{n+1}/a_n| < 1$, the series converges absolutely. This gives convergence for $|x - x_0| < R$ where $R = \lim|a_n/a_{n+1}|$ (when the limit exists).

**Cauchy-Hadamard formula.** The radius of convergence is always $R = 1/\limsup_{n\to\infty}|a_n|^{1/n}$, with the convention $R = \infty$ if the limsup is 0 and $R = 0$ if it is $\infty$. This formula always gives $R$, even when the ratio test limit does not exist.

## Examples

For $\sum_{n=0}^\infty x^n/n!$ (exponential series): $|a_{n+1}/a_n| = 1/(n+1) \to 0$, so $R = \infty$. Converges everywhere.

For $\sum_{n=0}^\infty n! x^n$: $|a_{n+1}/a_n| = n+1 \to \infty$, so $R = 0$. Converges only at $x = 0$.

For $\sum_{n=0}^\infty x^n/n^2$: $|a_{n+1}/a_n| = n^2/(n+1)^2 \to 1$, so $R = 1$. Converges absolutely for $|x| < 1$, converges at $x = \pm 1$ (conditionally at $x = -1$, absolutely at $x = 1$).

## Uniform Convergence

On any closed subinterval $[x_0 - r, x_0 + r]$ with $r < R$, the power series converges uniformly. Uniform convergence justifies term-by-term differentiation and integration, which are the operations used in the series method for ODEs:

$$\frac{d}{dx}\sum_{n=0}^\infty a_n(x-x_0)^n = \sum_{n=1}^\infty na_n(x-x_0)^{n-1}, \qquad |x-x_0| < R.$$

The differentiated series has the same radius of convergence $R$.

## The Radius in the ODE Context

For the ODE $y'' + p(x)y' + q(x)y = 0$, if $p$ and $q$ have power series representations with radii of convergence $R_p$ and $R_q$ around $x_0$, then the solution series around $x_0$ has radius of convergence at least $\min(R_p, R_q)$. More precisely, the radius is at least the distance from $x_0$ to the nearest singularity of $p$ or $q$ in the complex plane.

This guarantees, for example, that the series solution of $(1 - x^2)y'' - 2xy' + n(n+1)y = 0$ (Legendre's equation) around $x_0 = 0$ converges for $|x| < 1$, since the coefficients become singular at $x = \pm 1$ (distance 1 from the origin). The solution series has radius of convergence exactly 1.
