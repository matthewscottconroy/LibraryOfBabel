# Radius of Convergence of Solutions

The most important theorem about series solutions near ordinary points gives a lower bound on the radius of convergence:

**Theorem.** Let $x_0$ be an ordinary point of $y'' + p(x)y' + q(x)y = 0$, meaning $p$ and $q$ are analytic at $x_0$ with radii of convergence $R_p$ and $R_q$. Then the power series solutions around $x_0$ converge for $|x - x_0| < R$, where $R \geq \min(R_p, R_q)$. More precisely, $R$ is at least the distance in the complex plane from $x_0$ to the nearest point where $p$ or $q$ fails to be analytic.

## Why Complex Singularities Matter

The radius of convergence of a real power series is determined by the nearest singularity of the function in the complex plane, not just the real line. This is a fundamental theorem of complex analysis with direct consequences for ODE solutions.

**Example.** The equation $y'' + y/(1 + x^2) = 0$ has $p = 0$ and $q = 1/(1+x^2)$. On the real line, $q$ is smooth everywhere. But in the complex plane, $q$ has singularities at $x = \pm i$, which are at distance 1 from the origin. Therefore series solutions around $x_0 = 0$ have radius of convergence at least 1, even though $q$ has no real singularities.

## The Theorem Versus the Ratio Test

The theorem gives a lower bound on $R$: the series solutions converge at least up to the nearest singularity. In some cases, the actual radius is larger (the theorem is not sharp), but for the equations of classical mathematical physics, the bound is typically sharp.

The ratio test applied to the recurrence relation gives the same bound in typical examples. For Legendre's equation, the recurrence gives $|a_{k+2}/a_k| \to 1$ as $k \to \infty$, so $R = 1$, matching the singularity at $x = \pm 1$.

## Implications for Physical Validity

In physical applications, the interval of validity of a series solution is often determined by the nearest singularity of the coefficient functions. For Legendre's equation in spherical coordinates (where $x = \cos\theta \in [-1, 1]$), the singularities at $x = \pm 1$ (corresponding to $\theta = 0$ and $\theta = \pi$, the poles of the sphere) are precisely at the boundary of the physical domain. The convergence of the series for $|x| < 1$ covers the interior of the sphere, which is what is physically needed.

For Bessel's equation $x^2 y'' + xy' + (x^2 - \nu^2)y = 0$, the singular point at $x = 0$ means that the origin requires the Frobenius method (Chapter 3). Series solutions around any ordinary point $x_0 \neq 0$ converge for $|x - x_0| < |x_0|$ (distance to the singular point at the origin).
