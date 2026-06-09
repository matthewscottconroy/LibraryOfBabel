# Unit 2: Distributions and Generalized Functions

Classical analysis deals with functions: mappings that assign a numerical value to each point in their domain. Differential equations, however, routinely involve objects that do not fit this classical description. The Dirac delta function $\delta(x)$, which is "zero everywhere except at zero" and satisfies $\int_{-\infty}^\infty \delta(x) \, dx = 1$, is the canonical example: no ordinary function has these properties simultaneously. Yet $\delta$ appears throughout physics (as a point mass, a point charge, an impulsive force) and engineering (as an impulse response). The theory of distributions, developed by Laurent Schwartz in the 1940s, provides the mathematical framework that makes $\delta$ rigorous and extends the notion of differentiation far beyond the classical setting.

## The Core Idea

The key insight is to replace functions with linear functionals on a space of test functions. Rather than asking "what value does $\delta$ take at the point $x$?", one asks "how does $\delta$ act when paired with a test function $\phi$?" The answer is $\langle \delta, \phi \rangle = \phi(0)$—evaluation at zero. This reformulation is well-defined even though $\delta$ is not a function in the classical sense.

More generally, a **distribution** is a continuous linear functional on the space $\mathcal{D}(\mathbb{R}^n)$ of compactly supported smooth test functions. Ordinary functions (and much more general objects like locally integrable functions) define distributions by integration: $\langle T_f, \phi \rangle = \int f \phi \, dx$. But the class of distributions is much larger and includes objects without classical function representatives.

## Power of the Framework

Two properties of distributions are particularly important for PDE theory:

**Differentiation.** Every distribution can be differentiated arbitrarily many times. The derivative $T'$ of a distribution $T$ is defined by duality: $\langle T', \phi \rangle = -\langle T, \phi' \rangle$ (integration by parts). This agrees with the classical derivative whenever $T$ is a smooth function, and extends it to non-smooth objects. The absolute value function $|x|$ is not differentiable at 0 in the classical sense, but its distributional derivative is the sign function, and the distributional derivative of the sign function is $2\delta$.

**Fourier transform.** The Fourier transform extends naturally from functions to distributions, and the result is again a distribution. This extension is essential for solving PDEs with distributional sources and for analyzing fundamental solutions.

## Unit Structure

**Chapter 1: Motivation and Definitions** introduces the Dirac delta, the space $\mathcal{D}$ of test functions, and the precise definition of distributions. Key examples are developed.

**Chapter 2: Operations on Distributions** develops the algebra of distributions: differentiation (the crucial extension), multiplication by smooth functions, pullback under smooth maps, and the Fourier transform. These operations obey the same formal rules as their classical counterparts, making distributions a natural extension of function theory.

**Chapter 3: Fundamental Solutions** applies the theory to PDEs. A fundamental solution of a differential operator $P(D)$ is a distribution $E$ satisfying $P(D)E = \delta$. Convolution with $E$ then solves the inhomogeneous equation $P(D)u = f$ for arbitrary (distributional) right-hand sides. This approach unifies and generalizes the Green's function methods developed earlier in the course.
