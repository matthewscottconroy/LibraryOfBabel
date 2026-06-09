# The Indicial Equation

The indicial equation $F(r) = r(r-1) + P_0 r + Q_0 = 0$ is the quadratic in $r$ obtained by substituting the Frobenius ansatz $y = \sum a_n x^{n+r}$ into the ODE and extracting the coefficient of $x^r$ (the lowest power). Its roots $r_1 \geq r_2$ are the **exponents at the singularity** and determine the behavior of the solutions near $x_0$.

## Deriving the Indicial Equation

For the equation with regular singular point at $x_0 = 0$:

$$x^2 y'' + xP(x)y' + Q(x)y = 0, \qquad P(x) = \sum_{n=0}^\infty p_n x^n,\; Q(x) = \sum_{n=0}^\infty q_n x^n.$$

Substituting $y = \sum_{n=0}^\infty a_n x^{n+r}$:

- $x^2 y'' = \sum (n+r)(n+r-1)a_n x^{n+r}$
- $xP(x)y' = x\left(\sum p_k x^k\right)\left(\sum (n+r)a_n x^{n+r-1}\right) = \sum_{n=0}^\infty \left(\sum_{k=0}^n p_k(n-k+r)a_{n-k}\right)x^{n+r}$
- $Q(x)y = \left(\sum q_k x^k\right)\left(\sum a_n x^{n+r}\right) = \sum_{n=0}^\infty \left(\sum_{k=0}^n q_k a_{n-k}\right)x^{n+r}$

The coefficient of $x^r$ (the $n=0$ term in each sum) gives:

$$r(r-1)a_0 + p_0 ra_0 + q_0 a_0 = 0.$$

Since $a_0 \neq 0$: $F(r) \equiv r(r-1) + p_0 r + q_0 = 0$, the indicial equation, with $p_0 = P(0)$ and $q_0 = Q(0)$.

## Solving the Indicial Equation

$F(r) = r^2 + (p_0 - 1)r + q_0 = 0$. Roots:

$$r = \frac{(1-p_0) \pm \sqrt{(p_0-1)^2 - 4q_0}}{2}.$$

The roots can be real (distinct or equal) or complex conjugates.

## The General Term Recurrence

For $n \geq 1$, the coefficient of $x^{n+r}$ gives:

$$F(n+r)a_n + \sum_{k=0}^{n-1}[(n-k+r)p_{n-k} + q_{n-k}]a_k = 0,$$

where $F(n+r) = (n+r)(n+r-1) + p_0(n+r) + q_0$. This determines $a_n$ uniquely if $F(n+r) \neq 0$.

Note that $F(n+r) = (n+r-r_1)(n+r-r_2)$ (since $F(r) = 0$ has roots $r_1, r_2$). For $r = r_1$, $F(n+r_1) = n(n+r_1-r_2)$. This is nonzero for all $n \geq 1$ as long as $n \neq r_2 - r_1$... which is impossible since $r_1 \geq r_2$ and $n \geq 1$. So the recurrence for $r = r_1$ always determines $a_n$ uniquely. The first Frobenius solution $y_1$ always exists.

For $r = r_2 < r_1$, $F(n + r_2) = n(n - (r_1 - r_2))$. If $r_1 - r_2 = N$ (a positive integer), then $F(N + r_2) = N \cdot 0 = 0$, and the $N$-th coefficient is not determined by the recurrence. This is the source of the logarithmic term in the second solution.

## Examples

**Euler equation** $x^2 y'' + \alpha xy' + \beta y = 0$: $P(x) = \alpha$, $Q(x) = \beta$, so $p_0 = \alpha$, $q_0 = \beta$. Indicial equation: $r(r-1) + \alpha r + \beta = 0$, or $r^2 + (\alpha-1)r + \beta = 0$. The solutions of the Euler equation are $x^{r_1}$ and $x^{r_2}$ (when $r_1 \neq r_2$) or $x^r$ and $x^r\ln x$ (when $r_1 = r_2$), exactly what the Frobenius method predicts.

**Bessel's equation of order $\nu$**: $P_0 = 1$, $Q_0 = -\nu^2$. Indicial equation: $r^2 - \nu^2 = 0$, roots $r = \pm\nu$. The difference $r_1 - r_2 = 2\nu$: if $2\nu$ is a positive integer, the second solution involves $\ln x$.
