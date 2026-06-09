# Unit 4: Series Solutions of Differential Equations

The constant-coefficient techniques of Unit 3 break down when the coefficients $p(x)$ and $q(x)$ are not constants. For equations like Bessel's equation, Legendre's equation, or the Airy equation, no elementary closed-form solution exists. The remedy is to look for solutions in the form of power series, convergent in some interval. This approach, known as the method of power series or, in its extended form, the Frobenius method, gives solutions that are either elementary functions in series disguise or define important new special functions.

## Overview

The unit begins with a review of power series: convergence, radius of convergence, and basic operations. This material from calculus is recapitulated here in the context needed for the ODE applications.

The second chapter develops series solutions near **ordinary points**, where the equation's coefficients have no singularities. The method involves substituting $y = \sum a_n x^n$ into the ODE, matching powers of $x$, and deriving a **recurrence relation** for the coefficients $a_n$. The radius of convergence of the solution series is at least as large as the distance to the nearest singular point.

The third chapter treats **regular singular points**, where the coefficients have singularities of a specific moderate type. The **Frobenius method** generalizes the power series approach by seeking solutions of the form $y = x^r \sum a_n x^n$, where the exponent $r$ is determined by the **indicial equation**. The three cases of the Frobenius method, based on the difference of the indicial roots, give rise to different forms of the second solution.

The fourth chapter develops the most important special equations: Legendre's, Bessel's, Hermite's, Laguerre's, the hypergeometric, and Chebyshev. Each arises naturally in physics or mathematics, and each defines a family of special functions with rich properties used throughout science and engineering.

## Why Series?

Power series solutions serve two purposes. First, they give constructive existence proofs: rather than invoking an abstract theorem, one builds the solution term by term. Second, they define functions that have no other elementary description. The Bessel functions $J_n(x)$, for instance, are the natural eigenfunctions of the radial part of the Laplacian in cylindrical coordinates; they appear in the analysis of waves in cylindrical waveguides, the vibration of circular membranes, and the scattering of waves by spheres. Knowing their definition as solutions of Bessel's equation, their recurrence relations, and their asymptotic behavior is essential for anyone working in mathematical physics or engineering.
