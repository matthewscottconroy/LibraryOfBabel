# Taylor and Maclaurin Series

The Taylor series of a function $f$ centered at $x_0$ is the power series whose coefficients are determined by the derivatives of $f$ at $x_0$. For the series method in ODEs, the key concept is analyticity: a function is analytic at $x_0$ if it equals its Taylor series in some neighborhood of $x_0$.

## Taylor's Theorem and Analyticity

If $f$ is infinitely differentiable at $x_0$, its Taylor series is

$$\sum_{n=0}^\infty \frac{f^{(n)}(x_0)}{n!}(x-x_0)^n.$$

This series converges to $f(x)$ for $|x-x_0| < R$ if and only if the remainder $R_n(x) = f(x) - \sum_{k=0}^n \frac{f^{(k)}(x_0)}{k!}(x-x_0)^k \to 0$ as $n \to \infty$. A function for which this holds on some interval is **analytic** at $x_0$.

Analytic functions include: polynomials (radius of convergence $\infty$), exponentials, sines, cosines, $\sinh$, $\cosh$ (all with $R = \infty$), $\ln(1+x)$ (radius 1 centered at 0), $(1+x)^\alpha$ for any $\alpha$ (radius 1 centered at 0), and rational functions $p(x)/q(x)$ wherever $q \neq 0$ (radius = distance to nearest zero of $q$ in $\mathbb{C}$).

## Key Maclaurin Series

$$e^x = \sum_{n=0}^\infty \frac{x^n}{n!}, \quad \sin x = \sum_{n=0}^\infty \frac{(-1)^n x^{2n+1}}{(2n+1)!}, \quad \cos x = \sum_{n=0}^\infty \frac{(-1)^n x^{2n}}{(2n)!}, \quad |x| < \infty.$$

$$\frac{1}{1-x} = \sum_{n=0}^\infty x^n, \quad \ln(1+x) = \sum_{n=1}^\infty \frac{(-1)^{n-1}x^n}{n}, \quad |x| < 1.$$

$$(1+x)^\alpha = \sum_{n=0}^\infty \binom{\alpha}{n}x^n, \quad \binom{\alpha}{n} = \frac{\alpha(\alpha-1)\cdots(\alpha-n+1)}{n!}, \quad |x| < 1.$$

## Recognizing Known Series

In the series method, one often arrives at a power series whose coefficients satisfy a known pattern. Recognizing that $\sum_{n=0}^\infty (-1)^n x^{2n}/(2n)!$ is $\cos x$, or that $\sum_{n=0}^\infty x^n/n!$ is $e^x$, allows one to express the series solution in closed form. This is how the series method "rediscovers" elementary functions for equations that have them (like $y'' + y = 0$, whose series solution $y = a_0\cos x + a_1\sin x$ can be identified by recognizing the Maclaurin series).

For equations like Bessel's and Legendre's, the series do not match any elementary function pattern, defining instead the special functions discussed in Chapter 4.

## Complex Analytic Functions and the ODE Radius

In the complex plane, a function is analytic at $z_0$ if it is complex-differentiable in some neighborhood of $z_0$. The radius of convergence of the Taylor series at $z_0$ is exactly the distance to the nearest singularity in $\mathbb{C}$. This explains why the series for $1/(1+x^2)$ has radius of convergence 1 when expanded around $x_0 = 0$: the nearest singularities are at $x = \pm i$ (distance 1 from the real origin), even though $1/(1+x^2)$ is smooth for all real $x$.

For ODEs, this principle determines the radius of convergence of series solutions: the distance to the nearest singularity of the coefficient functions in the complex plane, not just the real line. A student who understands this can predict the radius of convergence of a series solution without computing a single coefficient.
