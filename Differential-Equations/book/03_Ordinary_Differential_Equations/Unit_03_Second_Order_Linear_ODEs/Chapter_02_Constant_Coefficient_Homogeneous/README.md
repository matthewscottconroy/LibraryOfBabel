# Chapter 2: Constant Coefficient Homogeneous Equations

When the coefficients $a, b, c$ in the second-order equation $ay'' + by' + cy = 0$ are constants, the equation has a complete algebraic solution theory. Every solution is built from exponential functions, determined by the roots of the associated **characteristic equation** $ar^2 + br + c = 0$. The three possible cases, two distinct real roots, a repeated real root, and a conjugate pair of complex roots, correspond to qualitatively different behaviors: pure exponential growth/decay, exponential with a polynomial factor, and oscillatory (sinusoidal) behavior.

## Why Exponentials?

The trial solution $y = e^{rx}$ transforms $ay'' + by' + cy = 0$ into $(ar^2 + br + c)e^{rx} = 0$. Since $e^{rx} \neq 0$, the equation holds if and only if $r$ is a root of the characteristic polynomial $ar^2 + br + c$. This algebraic reduction is possible precisely because differentiation acts on exponentials by multiplication: $(e^{rx})' = re^{rx}$, so $e^{rx}$ is an eigenvector of the differentiation operator with eigenvalue $r$. The characteristic equation is the secular equation of the differentiation operator restricted to the space of exponentials.

## Chapter Contents

The five sections of this chapter treat: the derivation of the characteristic equation; the case of two real distinct roots giving solutions $e^{r_1 x}$ and $e^{r_2 x}$; the repeated root case giving $e^{rx}$ and $xe^{rx}$; the complex conjugate root case giving oscillatory solutions $e^{\alpha x}\cos(\beta x)$ and $e^{\alpha x}\sin(\beta x)$; and the extension to higher-order constant-coefficient equations, where the characteristic polynomial may have degree $n$ and repeated or complex roots give rise to corresponding solution families.

## Physical Interpretation

The three cases correspond precisely to the three regimes of damped oscillation: overdamping (two real distinct roots with negative real part), critical damping (repeated negative real root), and underdamping (complex conjugate roots with negative real part). The undamped case (pure imaginary roots) gives perpetual sinusoidal oscillation. This physical taxonomy maps perfectly onto the algebra of the characteristic equation, one of the most satisfying correspondences in mathematical physics.
