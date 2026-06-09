# Classification of Singular Points

For the equation $y'' + p(x)y' + q(x)y = 0$, a point $x_0$ is **ordinary** if $p$ and $q$ are both analytic there. If $p$ or $q$ is not analytic at $x_0$, then $x_0$ is a **singular point**. Singular points are further classified as regular or irregular, with the Frobenius method applying only to regular ones.

## Regular Singular Points

$x_0$ is a **regular singular point** if:
1. $x_0$ is a singular point (so $p$ or $q$ is not analytic at $x_0$), and
2. $(x-x_0)p(x)$ is analytic at $x_0$, and
3. $(x-x_0)^2 q(x)$ is analytic at $x_0$.

Equivalently, $p(x)$ has at most a simple pole at $x_0$ and $q(x)$ has at most a double pole. Writing $P(x) = (x-x_0)p(x)$ and $Q(x) = (x-x_0)^2 q(x)$, these are analytic at $x_0$ with values $P(x_0) = \lim_{x\to x_0}(x-x_0)p(x)$ and $Q(x_0) = \lim_{x\to x_0}(x-x_0)^2 q(x)$.

## Irregular Singular Points

If either $(x-x_0)p(x)$ or $(x-x_0)^2 q(x)$ fails to be analytic at $x_0$, then $x_0$ is an **irregular singular point**. The Frobenius method does not apply, and solutions may involve essential singularities (such as $e^{1/x}$ near $x = 0$). The analysis of irregular singular points requires asymptotic methods and Stokes phenomena, topics well beyond the scope of this unit.

## Examples

**Bessel's equation** $x^2 y'' + xy' + (x^2 - \nu^2)y = 0$, or $y'' + (1/x)y' + (1 - \nu^2/x^2)y = 0$:
- $p(x) = 1/x$ has a simple pole at $x = 0$. $(x-0)p(x) = 1$: analytic.
- $q(x) = 1 - \nu^2/x^2$ has a double pole at $x = 0$. $(x-0)^2 q(x) = x^2 - \nu^2$: analytic.
- Conclusion: $x = 0$ is a regular singular point.

**Legendre's equation** $(1-x^2)y'' - 2xy' + n(n+1)y = 0$, standard form: $y'' - \frac{2x}{1-x^2}y' + \frac{n(n+1)}{1-x^2}y = 0$:
- $p(x) = -2x/(1-x^2)$ has simple poles at $x = \pm 1$.
- $(x-1)p(x) = -2x(x-1)/((1-x)(1+x)) = 2x/(1+x)$: analytic at $x = 1$.
- $(x-1)^2 q(x) = n(n+1)(x-1)^2/((1-x)(1+x)) = -n(n+1)(x-1)/(1+x)$: analytic at $x = 1$.
- Conclusion: $x = 1$ and by symmetry $x = -1$ are regular singular points.

**Equation with irregular singular point**: $y'' + (1/x^2)y = 0$. Here $p = 0$ and $q = 1/x^2$: $(x-0)^2 q(x) = 1$, analytic. But $(x-0)p(x) = 0$: analytic. Wait, actually $p = 0$ is analytic, so $x = 0$ is a regular singular point for this equation despite appearances. Actually the singular point classification requires that $p$ or $q$ fails at $x_0$; here $p = 0$ is analytic but $q = 1/x^2$ is not. $(x-0)^2 q = 1$ is analytic, so $x = 0$ is indeed a regular singular point.

For $y'' + (1/x^3)y = 0$: $p = 0$, $q = 1/x^3$. $(x)^2 q = 1/x$ is not analytic at 0. So $x = 0$ is an irregular singular point.

## The Point at Infinity

The behavior at $x = \infty$ is analyzed by substituting $t = 1/x$. A singular point at infinity is regular if $x = 0$ is a regular singular point of the transformed equation.
