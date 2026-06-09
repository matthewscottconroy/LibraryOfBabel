# Chapter 1: Power Series Review

Before attacking differential equations with series methods, one must be fluent with the algebra and analysis of power series: how to determine their convergence, how to manipulate them algebraically, and how to use Taylor and Maclaurin series for familiar functions. This chapter provides a focused review of these prerequisites.

## Convergence and Radius of Convergence

A power series $\sum_{n=0}^\infty a_n(x - x_0)^n$ converges absolutely at $x$ if $\sum |a_n(x-x_0)^n| < \infty$. The **radius of convergence** $R$ (possibly 0 or $\infty$) is determined by $1/R = \limsup_{n\to\infty}|a_n|^{1/n}$ (Cauchy-Hadamard formula) or by $1/R = \lim_{n\to\infty}|a_{n+1}/a_n|$ (ratio test, when the limit exists). The series converges absolutely for $|x - x_0| < R$ and diverges for $|x - x_0| > R$; convergence at the endpoints $|x - x_0| = R$ must be checked separately.

## Operations on Power Series

Within the radius of convergence, power series can be added, multiplied, differentiated, and integrated term by term. This enables algebraic manipulation of formal power series as if they were polynomials, with the results automatically valid within the radius of convergence.

## Taylor and Maclaurin Series

The Taylor series of a smooth function $f$ around $x_0$ is $\sum_{n=0}^\infty \frac{f^{(n)}(x_0)}{n!}(x-x_0)^n$. A function representable by a convergent power series on an interval containing $x_0$ is called **analytic** at $x_0$. Analyticity is the key hypothesis for the series method for ODEs near ordinary points.

## Connection to ODEs

For the ODE $y'' + p(x)y' + q(x)y = 0$, a point $x_0$ is an **ordinary point** if $p$ and $q$ are analytic at $x_0$. The power series method works at ordinary points, and the radius of convergence of the solution series is at least as large as the minimum of the radii of convergence of $p$ and $q$. This connection between the singularities of the coefficients and the radius of convergence of solutions is the key theorem of Chapter 2.
