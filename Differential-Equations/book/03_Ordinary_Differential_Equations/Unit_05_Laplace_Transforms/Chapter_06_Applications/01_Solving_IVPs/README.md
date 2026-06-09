# Solving Initial Value Problems with the Laplace Transform

The Laplace transform is particularly effective for linear constant-coefficient IVPs. The method is: transform, solve algebraically, invert. The initial conditions enter at the transform step, eliminating the separate determination-of-constants step required by classical methods.

## The Procedure

For $ay'' + by' + cy = g(t)$, $y(0) = y_0$, $y'(0) = y_1$:

1. Apply $\mathcal{L}$: $a(s^2Y - sy_0 - y_1) + b(sY - y_0) + cY = G(s)$.
2. Collect $Y$: $(as^2 + bs + c)Y = G(s) + (as + b)y_0 + ay_1$.
3. Solve: $Y(s) = \frac{G(s) + (as+b)y_0 + ay_1}{as^2 + bs + c}$.
4. Invert: $y(t) = \mathcal{L}^{-1}\{Y\}$.

## Worked Example: Piecewise Forcing

Solve $y'' + 4y = f(t)$, $y(0) = 0$, $y'(0) = 0$, where $f(t) = \begin{cases}1 & 0 \leq t < \pi,\\ 0 & t \geq \pi.\end{cases}$

Write $f(t) = 1 - u(t - \pi)$. Transform: $(s^2 + 4)Y = 1/s - e^{-\pi s}/s$.

$$Y = \frac{1 - e^{-\pi s}}{s(s^2 + 4)}.$$

Invert $1/(s(s^2+4))$: partial fractions give $\frac{1}{4s} - \frac{s}{4(s^2+4)}$, so $\mathcal{L}^{-1}\{1/(s(s^2+4))\} = \frac{1}{4}(1 - \cos 2t)$.

By the second shifting theorem: $y(t) = \frac{1}{4}(1-\cos 2t) - \frac{1}{4}u(t-\pi)(1-\cos 2(t-\pi))$.

Since $\cos(2(t-\pi)) = \cos 2t$:

$$y(t) = \begin{cases}\frac{1}{4}(1-\cos 2t) & 0 \leq t < \pi,\\ \frac{1}{4}(1-\cos 2t) - \frac{1}{4}(1-\cos 2t) = 0 & t \geq \pi.\end{cases}$$

The system is forced for $0 \leq t < \pi$ and then released; at $t = \pi$ the force turns off and the system returns to equilibrium (the oscillation happens to be complete at exactly $t = \pi$ because $2\pi = \pi \cdot 2$: the force duration is exactly one full period).

## Advantages Over Undetermined Coefficients

The Laplace method handles piecewise and impulsive forcing effortlessly, requires no guessing of the form of the particular solution, and incorporates initial conditions automatically. For constant-coefficient equations with special forcing (polynomials, exponentials, sinusoids), undetermined coefficients is often faster. For discontinuous or impulsive forcing, the Laplace method is clearly superior.
