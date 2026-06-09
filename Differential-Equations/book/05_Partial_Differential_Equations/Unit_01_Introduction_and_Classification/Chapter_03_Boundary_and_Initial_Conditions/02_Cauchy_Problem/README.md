# The Cauchy Problem for PDEs

The Cauchy problem is the PDE analogue of the initial value problem for ODEs. Just as an ODE of order $k$ requires $k$ initial conditions (the value of the unknown and its first $k-1$ derivatives at a single point), a PDE of order $k$ in time requires $k$ initial conditions — the value of the solution and its first $k-1$ time derivatives at the initial time $t = 0$. The Cauchy problem asks: given this initial data, does a unique solution exist, and how does it depend on the data?

## Definition and Setting

Let $\Omega = \mathbb{R}^n \times (0,\infty)$ be the upper half-space in spacetime, with spatial variable $\mathbf{x} \in \mathbb{R}^n$ and time $t > 0$. The initial surface (or Cauchy surface) is $\Sigma = \mathbb{R}^n \times \{0\}$, i.e., $t = 0$.

The **Cauchy problem** for a second-order PDE consists of:
1. The PDE $L[u] = f$ in $\Omega$.
2. Initial conditions $u(\mathbf{x},0) = \phi(\mathbf{x})$ and $u_t(\mathbf{x},0) = \psi(\mathbf{x})$ on $\Sigma$.

The functions $\phi$ and $\psi$ are the **Cauchy data**. For a first-order equation, only $u(\mathbf{x},0) = \phi(\mathbf{x})$ is prescribed. For a parabolic equation such as $u_t = k\Delta u$, which is first-order in $t$, only $u(\mathbf{x},0) = \phi(\mathbf{x})$ is required.

## Cauchy Problem for the Wave Equation

The classical Cauchy problem for the one-dimensional wave equation is:

$$u_{tt} = c^2 u_{xx}, \qquad x \in \mathbb{R},\ t > 0,$$
$$u(x,0) = \phi(x), \qquad u_t(x,0) = \psi(x).$$

**d'Alembert's Formula.** The unique solution is

$$u(x,t) = \frac{\phi(x+ct) + \phi(x-ct)}{2} + \frac{1}{2c}\int_{x-ct}^{x+ct}\psi(s)\,ds.$$

This formula shows that the solution at $(x,t)$ depends only on the initial data in the interval $[x-ct, x+ct]$ — the **domain of dependence** of the point $(x,t)$. Correspondingly, the initial data at a point $x_0$ influences the solution only in the wedge $|x - x_0| \leq ct$ — the **domain of influence** of $x_0$.

The well-posedness of this Cauchy problem is immediate from d'Alembert's formula:
- **Existence:** the formula gives an explicit solution.
- **Uniqueness:** the formula is derived from the general solution $u = f(x+ct) + g(x-ct)$, which is uniquely determined by the two initial conditions.
- **Continuous dependence:** $\|u(\cdot,t)\|_{L^\infty}$ is controlled by $\|\phi\|_{L^\infty}$, $\|\phi'\|_{L^\infty}$, and $\|\psi\|_{L^\infty}$ uniformly in $t$.

## Cauchy Problem for the Heat Equation

The Cauchy problem for the heat equation on $\mathbb{R}$:

$$u_t = k u_{xx}, \qquad x \in \mathbb{R},\ t > 0,$$
$$u(x,0) = \phi(x).$$

**Solution via the heat kernel.** For $\phi \in L^\infty(\mathbb{R})$ (or more generally for $\phi$ with at most polynomial growth), the unique solution is given by convolution with the heat kernel:

$$u(x,t) = \int_{-\infty}^\infty K(x-y,t)\,\phi(y)\,dy, \qquad K(x,t) = \frac{1}{\sqrt{4\pi kt}}\,e^{-x^2/(4kt)}.$$

Unlike the wave equation, there is no finite domain of dependence: the solution at $(x,t)$ depends on $\phi(y)$ for all $y \in \mathbb{R}$, with weights that are Gaussian. The heat kernel spreads information instantaneously across all of space, reflecting the infinite propagation speed of diffusion.

## The Cauchy-Kovalevskaya Theorem

For general PDEs with analytic data, the Cauchy-Kovalevskaya theorem provides the fundamental existence and uniqueness result.

**Theorem (Cauchy-Kovalevskaya).** Consider the Cauchy problem for a $k$-th order PDE:

$$\frac{\partial^k u}{\partial t^k} = F\!\left(t, \mathbf{x}, \left\{D^\alpha_\mathbf{x}\,\partial^j_t u : |\alpha| + j \leq k,\, j < k\right\}\right),$$

with Cauchy data $\partial^j u/\partial t^j\big|_{t=0} = \phi_j(\mathbf{x})$ for $j = 0, 1, \ldots, k-1$. If $F$ and the functions $\phi_0, \ldots, \phi_{k-1}$ are real-analytic in a neighborhood of the initial point, and the Cauchy surface $\Sigma = \{t=0\}$ is non-characteristic (which here means the equation is solved for the leading $t$-derivative), then the Cauchy problem has a unique real-analytic solution in some neighborhood of $\Sigma$.

The proof uses the Picard iteration for the equivalent power series, showing the series converges by a majorant (comparison with a simpler converging series). The key assumption is real-analyticity; the theorem fails for $C^\infty$ data in general (Lewy's example of a first-order PDE with smooth coefficients and no $C^1$ local solution).

**Limitations.** The Cauchy-Kovalevskaya theorem is purely local (it gives a solution in a neighborhood, not on the whole domain) and applies only to analytic data. For global solutions, for non-analytic data, and for equations with non-characteristic degenerate behavior, one needs the theory specific to each PDE type.

## When the Cauchy Problem Fails

The Cauchy problem is not always well-posed. For elliptic equations, posing a Cauchy problem (data on a surface of dimension $n-1$ inside $\mathbb{R}^n$, specifying $u$ and $\partial u/\partial\nu$) is generically ill-posed, as Hadamard's example demonstrates. The issue is that elliptic equations have no real characteristics — the surface $\Sigma$ is always non-characteristic — but the solution depends on boundary data everywhere on $\partial\Omega$, not just on a Cauchy surface of lower dimension.

A more subtle failure occurs even for hyperbolic equations when the Cauchy surface is characteristic. If $\Sigma$ is tangent to a characteristic at some point, the Cauchy problem may have no solution, infinitely many solutions, or may require additional data that the classical Cauchy formulation does not include.

## Cauchy Problem vs. Boundary Value Problem

The Cauchy problem and boundary value problems are conceptually complementary:

- The **Cauchy problem** specifies all data on an initial surface of dimension $n-1$ (less than the full boundary $\partial\Omega$) but specifies both $u$ and its normal derivative. It is appropriate for evolution equations (hyperbolic and parabolic).
- The **boundary value problem** specifies data only on $u$ (or its normal derivative, or a combination) but on the entire boundary $\partial\Omega$. It is appropriate for elliptic equations.

The well-posedness of each formulation depends on the equation type, which is why classification (Chapter 2) is a prerequisite for understanding auxiliary conditions.
