# Unit 02: Complex Differentiation

Differentiating a complex function looks, at first glance, like a routine extension of real-variable calculus: form the difference quotient $(f(z+h) - f(z))/h$ and take the limit as $h \to 0$. The surprise is that requiring this limit to exist in $\mathbb{C}$ — where $h$ can approach zero from any direction in the plane — is an enormously strong condition. Functions satisfying it are called analytic (or holomorphic), and they have properties that vastly exceed what real differentiability can guarantee.

## The Central Discovery

The Cauchy-Riemann equations are the algebraic signature of complex differentiability. If $f = u + iv$ is complex differentiable at $z_0$, the limits of the difference quotient taken along the real and imaginary axes must agree, and this forces the four partial derivatives of $u$ and $v$ to satisfy:
$$\frac{\partial u}{\partial x} = \frac{\partial v}{\partial y}, \qquad \frac{\partial u}{\partial y} = -\frac{\partial v}{\partial x}.$$

These two equations encode the full geometric content of complex differentiability: the Jacobian matrix of $F = (u, v)$ must be a scalar multiple of a rotation matrix at every point. Functions satisfying the Cauchy-Riemann equations on a domain — and whose partial derivatives are continuous there — are precisely the analytic functions.

## Chapter 01: Analytic Functions

The first chapter develops the theory of analytic functions: definition via the Cauchy-Riemann equations, the relationship between analyticity and harmonicity (the real and imaginary parts of an analytic function are always harmonic, satisfying Laplace's equation), and the construction of harmonic conjugates. These connections between complex analysis and the theory of elliptic PDEs are deep and consequential for mathematical physics.

## Chapter 02: Elementary Analytic Functions

The second chapter extends the familiar transcendental functions to the complex plane, where they exhibit behavior that is simultaneously more elegant and more intricate than their real counterparts. The complex exponential $e^z$ is entire (analytic on all of $\mathbb{C}$) and periodic with period $2\pi i$. The complex logarithm is the inverse of the exponential and is analytic on any simply connected domain not containing zero. Trigonometric and hyperbolic functions are defined via the exponential and reveal surprising algebraic identities connecting them. Power functions $z^\alpha$ for non-integer $\alpha$ require branch cuts.

## Learning Objectives

After completing this unit, a student should be able to:

- Apply the Cauchy-Riemann equations to determine where a given function is complex differentiable and compute its derivative.
- Prove that the real and imaginary parts of an analytic function are harmonic.
- Construct the harmonic conjugate of a given harmonic function.
- Compute with the complex exponential, logarithm, trigonometric, and hyperbolic functions, including their principal branches and domains of analyticity.
- Identify branch points and branch cuts for general power functions and inverse trigonometric functions.

## Connections Forward

The analyticity concept developed here is the hypothesis that drives every major theorem in Units 03 and 04. Cauchy's theorem requires analyticity inside a contour; the residue theorem computes residues of meromorphic functions (analytic except for poles); conformal maps are analytic functions with nonvanishing derivative. The elementary functions of Chapter 02 will appear as examples and building blocks throughout the remainder of the course.
