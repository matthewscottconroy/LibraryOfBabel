# Chapter 1: Calculus of Variations

The calculus of variations asks: among all functions satisfying given boundary conditions, which one minimizes (or extremizes) a given functional? The functional $\mathcal{E}[u] = \int_a^b L(x,u,u')\,dx$ assigns a number to each function $u$, and the goal is to find $u^*$ with $\mathcal{E}[u^*] \leq \mathcal{E}[u]$ for all competing $u$. The fundamental result — the **Euler-Lagrange equation** — is the necessary condition for such a minimizer: it is the condition that the "derivative" of $\mathcal{E}$ (in the function-space sense) vanishes at $u^*$. This condition is a second-order ODE (or PDE in higher dimensions) that $u^*$ must satisfy, and for convex $L$, it is also sufficient.

## Functionals and Their Role in PDEs

A functional $\mathcal{E}$ is a map from a function space to $\mathbb{R}$. The most common form in PDE theory:

$$\mathcal{E}[u] = \int_\Omega L(x,u(x),\nabla u(x))\,dx,$$

where $\Omega\subset\mathbb{R}^n$ and $L(x,z,p)$ is the **Lagrangian** — a function of position $x$, value $z = u(x)$, and gradient $p = \nabla u(x)$.

**Examples:**
- **Dirichlet energy:** $\mathcal{E}[u] = \frac{1}{2}\int_\Omega|\nabla u|^2\,dx$. Euler-Lagrange equation: $\Delta u = 0$ (Laplace's equation).
- **Poisson energy:** $\mathcal{E}[u] = \int_\Omega[\frac{1}{2}|\nabla u|^2 - fu]\,dx$. Euler-Lagrange equation: $-\Delta u = f$ (Poisson's equation).
- **Elastic energy:** $\mathcal{E}[u] = \int_0^L\frac{1}{2}(u'')^2\,dx$. Euler-Lagrange equation: $u'''' = 0$ (Euler-Bernoulli beam without load).
- **Arc length:** $\mathcal{E}[u] = \int_a^b\sqrt{1+(u')^2}\,dx$. Euler-Lagrange equation: $u'' = 0$ (straight line = shortest path).

## Structure of This Chapter

**Section 1: Functionals and Variation** introduces the basic setup: the function space $\mathcal{A} = \{u\in C^1([a,b]): u(a) = \alpha, u(b) = \beta\}$ (or its higher-dimensional analog), the notion of the **first variation** $\delta\mathcal{E}[u;v] = \lim_{\varepsilon\to 0}\frac{d}{d\varepsilon}\mathcal{E}[u+\varepsilon v]$, and the condition $\delta\mathcal{E}[u;v] = 0$ for all admissible variations $v$ as the stationarity condition.

**Section 2: Euler-Lagrange Equation** derives the Euler-Lagrange equation $L_u - \frac{d}{dx}L_{u'} = 0$ (in 1D) and its multi-dimensional version $L_u - \text{div}(L_p) = 0$ (where $p = \nabla u$). The derivation uses integration by parts and the fundamental lemma of the calculus of variations (if $\int_a^b f\eta\,dx = 0$ for all smooth $\eta$ with $\eta(a) = \eta(b) = 0$, then $f = 0$). Several worked examples illustrate the equation.

**Section 3: Natural Boundary Conditions** treats problems where the boundary values of $u$ are not prescribed (free boundary). In this case, the minimizer satisfies both the Euler-Lagrange equation in the interior and a **natural boundary condition** at the boundary — derived by setting the boundary terms in the integration by parts to zero. For the Dirichlet energy: the natural condition is $\partial u/\partial\nu = 0$ (Neumann condition).

**Section 4: Constraints and Lagrange Multipliers** treats optimization under constraints: isoperimetric problems (e.g., find the curve of given length enclosing maximum area — answer: circle), eigenvalue problems (minimize $\int|\nabla u|^2$ subject to $\int u^2 = 1$ — gives the principal eigenfunction), and other integral constraints handled via the Lagrange multiplier principle.

## Key Results

**Euler-Lagrange equation (1D).** If $u$ minimizes $\mathcal{E}[u] = \int_a^b L(x,u,u')\,dx$ over $u\in C^2([a,b])$ with $u(a) = \alpha$, $u(b) = \beta$, then:

$$L_u(x,u,u') - \frac{d}{dx}L_{u'}(x,u,u') = 0, \quad a < x < b. \tag{E-L}$$

**Euler-Lagrange equation (multi-D).** For $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$ with $u = g$ on $\partial\Omega$:

$$L_u(x,u,\nabla u) - \sum_{i=1}^n\frac{\partial}{\partial x_i}L_{p_i}(x,u,\nabla u) = 0, \quad x\in\Omega. \tag{E-L multi}$$

For the Dirichlet energy $L = |\nabla u|^2/2$: $L_u = 0$ and $L_{p_i} = u_{x_i}$, giving $-\Delta u = 0$.

**Convexity and minimality.** If $L(x,z,p)$ is convex in $(z,p)$ for each fixed $x$, then any critical point (solution of E-L) is a global minimizer.

## Connections to PDEs

The calculus of variations and PDE theory are two sides of the same coin: every second-order linear elliptic PDE of the form $-\sum_{ij}\partial_i(a_{ij}\partial_j u) + cu = f$ has a variational formulation as the minimizer of $\mathcal{E}[u] = \int[\frac{1}{2}\sum a_{ij}u_{x_i}u_{x_j} + \frac{c}{2}u^2 - fu]\,dx$. Conversely, the Euler-Lagrange equation of any functional with $L$ quadratic in $(u,\nabla u)$ is a linear PDE.

For nonlinear PDEs: the $p$-Laplacian $\Delta_p u = \text{div}(|\nabla u|^{p-2}\nabla u) = 0$ is the Euler-Lagrange equation for $\mathcal{E}[u] = \frac{1}{p}\int|\nabla u|^p\,dx$; the minimal surface equation is the Euler-Lagrange equation for the area functional.

The power of the variational approach becomes apparent in Chapter 2, where weak solutions are defined as minimizers (or saddle points) of the associated energy functional, allowing existence proofs via the direct method even when classical solutions do not exist.
