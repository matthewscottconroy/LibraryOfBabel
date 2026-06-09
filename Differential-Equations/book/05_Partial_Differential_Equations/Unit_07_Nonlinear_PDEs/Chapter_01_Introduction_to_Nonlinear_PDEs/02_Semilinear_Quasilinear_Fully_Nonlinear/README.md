# Semilinear, Quasilinear, and Fully Nonlinear Equations: Examples and Analysis

Abstract classification is made concrete through worked examples. This section examines one representative PDE from each level of the nonlinearity hierarchy, analyzing it with the appropriate tools: energy methods for semilinear equations, the implicit function theorem and Schauder estimates for quasilinear equations, and the viscosity solution framework for fully nonlinear equations. The goal is not a comprehensive survey but a clear illustration of how the type of nonlinearity determines the analytical approach.

## Semilinear Example: The Allen-Cahn Equation

The **Allen-Cahn equation** is:

$$u_t = \varepsilon^2\Delta u + u - u^3 = \varepsilon^2\Delta u - W'(u), \qquad W(u) = \frac{1}{4}(1-u^2)^2. \tag{AC}$$

Here $W(u)$ is the double-well potential with wells at $u = \pm 1$. The equation models phase separation (e.g., the composition of a binary alloy), where $u \approx 1$ is one phase and $u \approx -1$ is the other. The parameter $\varepsilon$ is the interface width.

**Gradient flow structure.** The Allen-Cahn equation is the $L^2$ gradient flow of the Ginzburg-Landau energy:

$$\mathcal{E}_\varepsilon[u] = \int_\Omega\left[\frac{\varepsilon^2}{2}|\nabla u|^2 + W(u)\right]dx.$$

Indeed, $u_t = -\delta\mathcal{E}_\varepsilon/\delta u = \varepsilon^2\Delta u - W'(u)$. Since $\frac{d}{dt}\mathcal{E}_\varepsilon[u(t)] = -\int_\Omega |u_t|^2\,dx \leq 0$, the energy decreases along solutions.

**1D steady states.** Setting $u_t = 0$: $\varepsilon^2 u_{xx} = W'(u) = u^3 - u$. This ODE has the exact solution:

$$u_0(x) = \tanh\!\left(\frac{x}{\varepsilon\sqrt{2}}\right),$$

a **phase transition layer** (diffuse interface) interpolating between $u = -1$ (as $x \to -\infty$) and $u = +1$ (as $x \to +\infty$) over a width $\sim\varepsilon$.

**As $\varepsilon\to 0$:** The interface becomes sharp, and the limiting problem is **mean curvature flow**: the interface between the two phases moves with velocity proportional to its mean curvature. This is the $\Gamma$-convergence result for the Allen-Cahn equation.

**Stability analysis.** Linearizing around $u_0 = \tanh(x/(\varepsilon\sqrt{2}))$: set $u = u_0 + v$ with $v$ small. The linearized equation is $v_t = \varepsilon^2 v_{xx} + (1-3u_0^2)v$. The coefficient $1-3u_0^2$ ranges from $1-3\tanh^2(\cdot) < 0$ (in the interface layer, where $|u_0| < 1/\sqrt{3}$) to $1-3(-1)^2 = -2 < 0$ and $1-3(1)^2 = -2 < 0$ (far from the interface). So the linearized operator is negative definite (for appropriate boundary conditions), and the steady state is linearly stable.

## Quasilinear Example: The Minimal Surface Equation

The minimal surface equation is:

$$\text{div}\!\left(\frac{\nabla u}{\sqrt{1+|\nabla u|^2}}\right) = 0, \quad \mathbf{x}\in\Omega\subset\mathbb{R}^2. \tag{MSE}$$

Writing out the divergence:

$$(1+u_y^2)u_{xx} - 2u_xu_yu_{xy} + (1+u_x^2)u_{yy} = 0.$$

This is the Euler-Lagrange equation for the area functional $\mathcal{A}[u] = \int_\Omega\sqrt{1+|\nabla u|^2}\,dx\,dy$. A function $u$ satisfying (MSE) is called a **minimal surface** (or, more precisely, the graph $\{(x,y,u(x,y))\}$ has zero mean curvature).

**Ellipticity.** The coefficient matrix of the second derivatives is:

$$A(p) = \frac{1}{(1+|p|^2)^{3/2}}\begin{pmatrix}1+p_2^2 & -p_1p_2 \\ -p_1p_2 & 1+p_1^2\end{pmatrix}, \quad p = (p_1,p_2) = (u_x,u_y).$$

The eigenvalues of $A$ are $(1+|p|^2)^{-3/2}$ and $(1+|p|^2)^{-1/2}$, both strictly positive. The minimal surface equation is **uniformly elliptic** for bounded $|\nabla u|$, but the ellipticity constants deteriorate as $|\nabla u| \to\infty$.

**Bernstein's theorem.** A classical result: if $u:\mathbb{R}^2\to\mathbb{R}$ is a smooth solution of (MSE) (a complete minimal graph), then $u$ is affine (a plane). This fails in higher dimensions: for $n \geq 8$, there exist non-affine entire minimal graphs (Bombieri-De Giorgi-Giusti, 1969).

**Schauder estimates.** For solutions of the Dirichlet problem (MSE) on $\Omega$ with $u = g$ on $\partial\Omega$: if $g\in C^{2,\alpha}(\partial\Omega)$ and $\Omega$ is uniformly convex, the problem has a unique solution $u\in C^{2,\alpha}(\overline{\Omega})$. The proof uses: (1) the implicit function theorem in Banach spaces (small data), (2) the continuity method ($\lambda$-parametric deformation from Laplace to MSE), and (3) a priori $C^{1,\alpha}$ estimates from DeGiorgi-Nash-Moser theory (the equation is quasilinear elliptic in divergence form).

**Non-solvability for mean-convex domains.** For the Dirichlet problem on a general $\Omega$, a solution exists if and only if the boundary data $g$ satisfies a bounded slope condition. For non-convex domains, the mean curvature of $\partial\Omega$ may prevent the Dirichlet problem from being solvable for large boundary data — a feature with no linear analogue.

## Fully Nonlinear Example: The Monge-Ampere Equation

The **Monge-Ampere equation** is:

$$\det(D^2 u) = f(x), \quad x\in\Omega\subset\mathbb{R}^n, \tag{MA}$$

with $D^2u = (\partial^2 u/\partial x_i\partial x_j)$ the Hessian matrix. For (MA) to be elliptic, $D^2 u$ must be positive definite (i.e., $u$ must be convex). This ties the ellipticity to the solution.

**Physical and geometric origin.** In optimal transport (Brenier-McCann-Villani): the optimal transport map $T$ from density $\rho_0$ to $\rho_1$ (minimizing the $L^2$ cost $\int|x-T(x)|^2\rho_0\,dx$) has the form $T = \nabla u$ where $u$ is convex and satisfies $\det(D^2 u) = \rho_0/(\rho_1\circ\nabla u)$. In differential geometry: if $u:\Omega\to\mathbb{R}$ is the convex function whose graph has prescribed Gaussian curvature $K(x,u(x))$, then $\det(D^2 u) = K(x,u)(1+|\nabla u|^2)^{(n+2)/2}$.

**Example in 2D.** $\det(D^2 u) = u_{xx}u_{yy} - u_{xy}^2 = f(x,y)$. The exact solution for $f = 1$ on the unit disk with $u = 0$ on the boundary is $u(x,y) = (1-x^2-y^2)/2$ — the paraboloid. Here $D^2u = -\text{Id}$ is negative definite... correction: $u = \frac{1}{2}(x^2+y^2) - C$ gives $D^2u = \text{Id}$ (positive definite) with $\det = 1$. Taking $u = \frac{1}{2}(x^2+y^2)$ satisfies $\det(D^2 u) = 1$ but not the boundary condition; adding boundary corrections gives the full solution.

**The viscosity solution approach.** Caffarelli's regularity theory (1990s) shows that if $\lambda \leq f \leq \Lambda$ (bounded between positive constants), then any convex viscosity solution of (MA) is in $C^{1,\alpha}$ (Hölder). If additionally $f\in C^{k,\alpha}$, then $u\in C^{k+2,\alpha}$ by Schauder estimates for the linearized equation (which is a uniformly elliptic operator $\sum a_{ij}\partial_{ij}$ with $a_{ij} = \text{cofactors of }D^2u$).

## Comparison: Analytical Tools by Type

| Type | Tools | Existence method | Uniqueness |
|---|---|---|---|
| Semilinear | Banach fixed point, energy, Sobolev | Contraction in $H^1$; subcritical Sobolev | Via energy or comparison |
| Quasilinear | Schauder estimates, continuity method | DeGiorgi-Nash-Moser + degree theory | Comparison (if monotone in $u$) |
| Fully nonlinear | Viscosity solutions, Evans-Krylov theory | Perron's method with barriers | Comparison principle for viscosity solutions |

The key unifying theme: in all three cases, the **comparison principle** (if it holds) provides both uniqueness and stability. The challenge increases with the level of nonlinearity because establishing the comparison principle for viscosity solutions requires careful handling of points where two test functions touch the solution.

## Local vs. Global Behavior

An important distinction absent in linear theory: for nonlinear PDEs, **local** and **global** behavior can differ dramatically.

**Local existence.** For most nonlinear evolution equations (semilinear or quasilinear), local-in-time existence follows from a contraction mapping argument in a suitable Sobolev space. The solution exists on a time interval $[0,T^*)$ where $T^*$ depends on the initial data.

**Global existence vs. blow-up.** Whether $T^* = \infty$ (global existence) or $T^* < \infty$ (blow-up) is determined by the competition between:
- **Dissipation** (negative terms, e.g., $-\|\nabla u\|^2$ from the diffusive part).
- **Growth** (positive terms, e.g., $\|u\|^p$ from the reaction part).

When the growth term dominates (supercritical regime), blow-up occurs. When dissipation dominates (subcritical regime), global existence holds. At the critical exponent, the behavior is delicate and may depend on the size of the initial data.
