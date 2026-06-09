# Chapter 6: Applications of the Laplace Transform

The Laplace transform shines brightest in its applications. This chapter demonstrates three major application domains: solving IVPs (the primary use for ODE students), solving systems of ODEs via simultaneous Laplace transforms, and the transfer function framework of control and systems theory.

## Solving Initial Value Problems

For a single constant-coefficient ODE with any piecewise-continuous forcing, the Laplace method proceeds in three steps: transform the ODE (initial conditions are automatically incorporated), solve the resulting algebraic equation for $Y(s)$, and invert. The method handles discontinuous and impulsive forcing that would require the method of variation of parameters with case analysis in the classical approach.

## Systems via Laplace Transform

A system of linear ODEs with constant coefficients can be transformed term by term, giving a system of linear algebraic equations for the transforms of each unknown. This algebraic system is solved by elimination or Cramer's rule, and each component is then inverted separately. The Laplace method for systems is the direct analog of the single-equation method and avoids the matrix exponential computations of Unit 6.

## Transfer Functions and Impulse Response

For a linear system $L[y] = g(t)$ with zero initial conditions, the Laplace transform gives $H(s)Y(s) = G(s)$ where $H(s) = 1/(s^2 + ps + q)$ is the **transfer function** (or system function). The output $Y = H(s)G(s)$ is the product of the transfer function and the input transform. The impulse response $h(t) = \mathcal{L}^{-1}\{H(s)\}$ completely characterizes the system: the output for any input $g$ is $y = h * g$ (convolution). This framework is the foundation of classical control theory, signal processing, and linear systems analysis.
