# Unit 5: The Laplace Transform

The Laplace transform is an operational method that converts differential equations into algebraic equations. By transforming the unknown function and its derivatives into a new domain (the $s$-domain), differentiation becomes multiplication and initial conditions are automatically incorporated. After solving algebraically in the $s$-domain, one inverts the transform to recover the solution.

## The Core Idea

For a function $f(t)$ defined for $t \geq 0$, the Laplace transform is

$$F(s) = \mathcal{L}\{f\}(s) = \int_0^\infty e^{-st}f(t)\,dt.$$

The key property is that $\mathcal{L}\{f'\}(s) = sF(s) - f(0)$: differentiation in the $t$-domain becomes multiplication by $s$ in the $s$-domain (with a correction term for the initial value). This converts the ODE $y'' + py' + qy = g(t)$ into the algebraic equation $(s^2 + ps + q)Y(s) = G(s) + \text{initial data}$, solvable by simple algebra. The solution is $Y(s) = [\text{algebraic expression}]$, and $y(t) = \mathcal{L}^{-1}\{Y\}$.

## Why Laplace Transforms Are Powerful

The Laplace transform handles initial value problems for linear equations with constant coefficients effortlessly, especially when the forcing function $g(t)$ is discontinuous or impulsive. The Heaviside step function, the Dirac delta, and piecewise-defined forcing functions are all natural in the Laplace domain. The convolution theorem connects the Laplace transform to the Green's function theory of Unit 8, providing a frequency-domain perspective on what superposition means.

## Unit Organization

The unit develops in six chapters: definitions and existence conditions, operational properties (shifting, differentiation of transforms, transforms of derivatives and integrals), discontinuous and impulsive functions (Heaviside, second shifting theorem, Dirac delta, periodic functions), convolution and its applications, inverse transforms via partial fractions and table methods, and applications to IVPs, systems, and transfer functions.
