# Chapter 2: Laplace's Equation in Rectangular Coordinates

The simplest bounded domains for studying Laplace's equation are rectangles: geometry is aligned with the coordinate axes, boundaries are flat, and separation of variables produces Fourier series that converge to the solution. Despite this apparent simplicity, the rectangular geometry exhibits all the essential features of elliptic theory — nonhomogeneous boundary conditions on different sides, the interaction between Dirichlet and Neumann data, and the need to satisfy four boundary conditions simultaneously (two in each coordinate direction).

## Setup

On a rectangle $\Omega = (0,a)\times(0,b)$, the Dirichlet problem is:

$$\Delta u = u_{xx} + u_{yy} = 0 \text{ in } \Omega, \qquad u = g \text{ on } \partial\Omega.$$

If the boundary data $g$ is prescribed separately on each side, the problem is solved by decomposing into four sub-problems, each with nonzero data on exactly one side and zero data on the remaining three. By linearity (superposition), the total solution is the sum of the four sub-solutions.

## Structure of This Chapter

**Section 1: Dirichlet Problem on a Rectangle** solves the case of nonzero data on one side (say $u(x,b) = f(x)$ with zeros elsewhere). Separation of variables gives:

$$u(x,y) = \sum_{n=1}^\infty b_n\sin\!\left(\frac{n\pi x}{a}\right)\frac{\sinh(n\pi y/a)}{\sinh(n\pi b/a)},$$

where $b_n = \frac{2}{a}\int_0^a f(x)\sin(n\pi x/a)\,dx$. The solution decays exponentially away from the boundary where data is prescribed — a reflection of the maximum principle (the interior is controlled by the boundary).

**Section 2: Neumann and Mixed Boundary Conditions** treats the case where some sides have Neumann data ($\partial u/\partial n = h$) and others have Dirichlet data ($u = g$). For the pure Neumann problem, the compatibility condition $\oint_{\partial\Omega}h\,ds = 0$ must be verified, and the solution is unique only up to a constant.

## Key Features

The hyperbolic sine functions $\sinh(n\pi y/a)$ replace the exponential decay factors $e^{-n^2t}$ from the heat equation. This reflects a fundamental difference between parabolic and elliptic equations: in the heat equation, modes decay in time; in Laplace's equation on a rectangle, they decay in the cross-direction. The "spatial decay" of Fourier modes in the rectangle is the elliptic analogue of temporal decay.

The double Fourier series solution for data on all four sides simultaneously is constructed by superposition, combining four series of the form above. The convergence is exponential in the interior of the rectangle (reflecting the real-analyticity of harmonic functions), and the series can be truncated after a few terms for practical computation.
