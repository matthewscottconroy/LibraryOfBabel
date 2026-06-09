# The Frobenius Method

At a regular singular point $x_0$, one seeks solutions of the form

$$y = (x-x_0)^r \sum_{n=0}^\infty a_n(x-x_0)^n = \sum_{n=0}^\infty a_n(x-x_0)^{n+r}, \quad a_0 \neq 0,$$

where $r$ is a real (or complex) number to be determined. This is a **Frobenius series**. The exponent $r$ allows for solutions that are not analytic at $x_0$ (e.g., $(x-x_0)^{1/2}$ or $(x-x_0)^\pi$) while still being expressible as a series.

## Deriving the Indicial Equation

Multiply the equation $y'' + p(x)y' + q(x)y = 0$ through by $(x-x_0)^2$ and write $P(x) = (x-x_0)p(x)$ and $Q(x) = (x-x_0)^2 q(x)$ (both analytic at $x_0$). The equation becomes

$$(x-x_0)^2 y'' + (x-x_0)P(x)y' + Q(x)y = 0.$$

Substituting $y = \sum_{n=0}^\infty a_n(x-x_0)^{n+r}$:

$$(x-x_0)^2 y'' = \sum_{n=0}^\infty (n+r)(n+r-1)a_n(x-x_0)^{n+r}.$$

Expanding $P(x) = P_0 + P_1(x-x_0) + \cdots$ and $Q(x) = Q_0 + Q_1(x-x_0) + \cdots$ where $P_0 = P(x_0)$ and $Q_0 = Q(x_0)$, and looking at the coefficient of $(x-x_0)^r$ (the lowest power):

$$r(r-1)a_0 + P_0 ra_0 + Q_0 a_0 = a_0[r(r-1) + P_0 r + Q_0] = 0.$$

Since $a_0 \neq 0$, the **indicial equation** is:

$$r(r-1) + P_0 r + Q_0 = 0 \qquad \text{where } P_0 = \lim_{x\to x_0}(x-x_0)p(x),\; Q_0 = \lim_{x\to x_0}(x-x_0)^2 q(x).$$

This is a quadratic in $r$ with two roots $r_1$ and $r_2$ (the **indicial roots** or **exponents at the singularity**).

## The First Solution

For the larger root $r = r_1$, the coefficient of each $(x-x_0)^{n+r_1}$ in the substituted equation gives a recurrence for $a_n$ in terms of $a_0, \ldots, a_{n-1}$. The coefficient of $a_n$ in this recurrence is $F(n + r_1)$ where $F(r) = r(r-1) + P_0 r + Q_0$ is the left side of the indicial equation. Since $r_1$ is a root, $F(r_1) = 0$, but $F(n + r_1) \neq 0$ for $n \geq 1$ (assuming $n + r_1$ is not also a root, i.e., $r_1 - r_2$ is not a positive integer). This guarantees that the recurrence is solvable and gives a unique Frobenius series $y_1 = (x-x_0)^{r_1}\sum a_n(x-x_0)^n$.

## The Second Solution

For the smaller root $r_2$, the analysis is similar when $r_1 - r_2$ is not a non-negative integer. When $r_1 = r_2$ or $r_1 - r_2$ is a positive integer, the second solution requires more care (see the next two sections).

## Worked Example: Bessel's Equation of Order Zero

For $x^2y'' + xy' + x^2 y = 0$ (Bessel's equation with $\nu = 0$), $P_0 = 1$ and $Q_0 = 0$. Indicial equation: $r(r-1) + r + 0 = r^2 = 0$. Repeated root $r = 0$ (so $r_1 = r_2 = 0$).

The larger root $r_1 = 0$ gives a standard power series solution $J_0(x) = \sum (-1)^n x^{2n}/(2^{2n}(n!)^2)$. The repeated root signals that the second solution involves $\ln x$.
