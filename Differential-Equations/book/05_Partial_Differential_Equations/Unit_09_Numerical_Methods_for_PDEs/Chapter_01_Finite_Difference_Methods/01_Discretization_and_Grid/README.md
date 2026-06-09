# Discretization and Grid

The first step in any finite difference method is the construction of a computational grid and the replacement of continuous derivatives by discrete approximations. This section develops the Taylor expansion analysis of difference operators, quantifies the truncation error, and illustrates the key principle: higher-order accuracy requires wider stencils.

## Taylor Expansion and Difference Quotients

Let $u(x)$ be a smooth function. By Taylor's theorem:

$$u(x+h) = u(x) + hu'(x) + \frac{h^2}{2}u''(x) + \frac{h^3}{6}u'''(x) + \frac{h^4}{24}u^{(4)}(x) + O(h^5).$$

$$u(x-h) = u(x) - hu'(x) + \frac{h^2}{2}u''(x) - \frac{h^3}{6}u'''(x) + \frac{h^4}{24}u^{(4)}(x) + O(h^5).$$

**First derivative approximations:**

- **Forward difference:** $\frac{u(x+h)-u(x)}{h} = u'(x) + \frac{h}{2}u''(x) + O(h^2)$. Error: $O(h)$.
- **Backward difference:** $\frac{u(x)-u(x-h)}{h} = u'(x) - \frac{h}{2}u''(x) + O(h^2)$. Error: $O(h)$.
- **Centered difference:** $\frac{u(x+h)-u(x-h)}{2h} = u'(x) + \frac{h^2}{6}u'''(x) + O(h^4)$. Error: $O(h^2)$.

The centered difference is second-order accurate because the first-order error terms cancel when combining the two Taylor expansions.

**Second derivative:**

$$\frac{u(x+h)-2u(x)+u(x-h)}{h^2} = u''(x) + \frac{h^2}{12}u^{(4)}(x) + O(h^4). \tag{2nd-order centered}$$

Error: $O(h^2)$ — standard for finite difference discretization of $u_{xx}$.

**Fourth-order approximation for $u''$:**

$$\frac{-u(x+2h)+16u(x+h)-30u(x)+16u(x-h)-u(x-2h)}{12h^2} = u''(x) + O(h^4).$$

This uses a 5-point stencil and achieves fourth-order accuracy at the cost of a wider stencil (boundary treatment becomes more complex).

## Local Truncation Error

**Definition.** For a finite difference scheme approximating a PDE $Lu = 0$, the **local truncation error (LTE)** at grid point $(x_j,t_n)$ is the residual obtained by substituting the exact solution $u(x_j,t_n)$ into the difference scheme:

$$\tau_j^n = \frac{1}{\Delta t}\left[u_j^{n+1} - u_j^n\right] - \kappa\frac{u_{j+1}^n - 2u_j^n + u_{j-1}^n}{(\Delta x)^2},$$

where $u_j^n = u(x_j,t_n)$. For the heat equation $u_t = \kappa u_{xx}$, the LTE of FTCS is:

$$\tau_j^n = \frac{u_t\Delta t}{2} + O(\Delta t^2) - \kappa\frac{u_{xxxx}(\Delta x)^2}{12} + O((\Delta x)^4) = O(\Delta t) + O((\Delta x)^2).$$

The scheme is **first-order accurate in time** and **second-order accurate in space**.

**Notation:** A scheme with LTE $= O(\Delta t^p + (\Delta x)^q)$ is said to be **order $p$ in time and order $q$ in space**.

## Grid Generation for Rectangular Domains

For $\Omega = [0,L_x]\times[0,L_y]$ with $M_x$ and $M_y$ intervals:

- $\Delta x = L_x/M_x$, $\Delta y = L_y/M_y$.
- Grid points: $(x_j, y_k) = (j\Delta x, k\Delta y)$ for $j = 0,\ldots,M_x$, $k = 0,\ldots,M_y$.
- Interior points: $j = 1,\ldots,M_x-1$, $k = 1,\ldots,M_y-1$ ($(M_x-1)(M_y-1)$ unknowns).

**Ordering of unknowns.** For the 2D Laplace equation, the interior unknowns $U_{jk} = U(x_j,y_k)$ are ordered (row by row or column by column) into a vector of length $(M_x-1)(M_y-1)$. The resulting linear system has a banded structure with bandwidth $\max(M_x, M_y)$.

**Five-point stencil for $\Delta u$:** 

$$\Delta_h U_{jk} = \frac{U_{j+1,k}+U_{j-1,k}-2U_{jk}}{(\Delta x)^2} + \frac{U_{j,k+1}+U_{j,k-1}-2U_{jk}}{(\Delta y)^2}.$$

For $\Delta x = \Delta y = h$: $\Delta_h U_{jk} = (U_{j+1,k}+U_{j-1,k}+U_{j,k+1}+U_{j,k-1}-4U_{jk})/h^2$.

LTE: $\tau_{jk} = \frac{h^2}{12}(\partial^4_{xxxx}u + \partial^4_{yyyy}u) + O(h^4)$ — second-order accurate.

## Irregular Grids and Boundary Fitting

For non-rectangular domains, the standard approach is to use one of:

1. **Immersed boundary method:** Use a uniform Cartesian grid, but modify the stencil near the boundary to impose boundary conditions.

2. **Structured curvilinear grids:** Map the domain $\Omega$ to a rectangle $[0,1]^2$ via a coordinate transformation $(\xi,\eta)\to(x,y)$; solve the PDE in the transformed coordinates (where the grid is uniform).

3. **Finite element method (Unit 8):** Triangulate $\Omega$ directly; the FEM handles irregular domains naturally.

For this unit, we restrict to rectangular domains and structured grids.

## Consistency: Taylor Error Table

A systematic way to derive high-order schemes: unknown coefficients $\{c_k\}$ in the stencil $\sum_k c_k u(x+k\Delta x)$ are determined by requiring the LTE to vanish to high order. This is a linear system for $\{c_k\}$ obtained by expanding each $u(x+k\Delta x)$ in Taylor series and matching powers of $\Delta x$.

| Order | Stencil for $u''$ | Points |
|---|---|---|
| 2 | $\frac{1}{h^2}[1,-2,1]$ | 3 (centered) |
| 4 | $\frac{1}{12h^2}[-1,16,-30,16,-1]$ | 5 |
| 6 | $\frac{1}{180h^2}[2,-27,270,-490,270,-27,2]$ | 7 |

Higher-order stencils improve accuracy but require more boundary treatment and communication in parallel implementations.

## Stability: A Preview

Consistency alone does not guarantee accuracy. The scheme's stability — the question of whether numerical errors grow or remain bounded — is equally essential. The classic example is FTCS for the heat equation with $r = \kappa\Delta t/(\Delta x)^2 > 1/2$: the scheme is consistent (LTE $\to 0$ as $h\to 0$) but unstable — numerical errors grow without bound.

The condition $r \leq 1/2$ is the **stability condition** for FTCS, derived in Chapter 2 via von Neumann analysis. It provides the time step constraint: to halve $\Delta x$ (double the spatial resolution), the time step must be reduced by a factor of 4. This quadratic relationship between $\Delta t$ and $\Delta x$ is the defining characteristic of explicit parabolic schemes and the main motivation for implicit methods.
