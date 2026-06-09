# Green's Functions via Distributions

The distributional theory of fundamental solutions gives a rigorous and unified foundation for the Green's function methods developed throughout classical ODE and PDE theory. A Green's function is, precisely, a fundamental solution adapted to a domain with specific boundary conditions. This section shows how distributional thinking connects the abstract Malgrange-Ehrenpreis theorem to the concrete computational Green's functions of boundary value problems.

## From Fundamental Solutions to Green's Functions

A fundamental solution $E$ of an operator $P(D)$ in all of $\mathbb{R}^n$ solves $P(D)E = \delta$ without any boundary conditions. A **Green's function** $G(x, y)$ for the same operator on a domain $\Omega$ with boundary condition $\mathcal{B}$ solves:

$$P(D)G(x,y) = \delta(x-y) \text{ for } x \in \Omega, \quad \mathcal{B}[G(\cdot, y)] = 0 \text{ on } \partial\Omega.$$

The solution to $P(D)u = f$ in $\Omega$ with $\mathcal{B}[u] = 0$ is then $u(x) = \int_\Omega G(x,y)f(y) \, dy$.

Distributionally, $G(\cdot, y)$ is the composition of the fundamental solution $E$ with a "correction term" that accounts for the boundary conditions. For the Laplacian on a domain $\Omega$, $G(x,y) = E(x-y) + h(x,y)$, where $h$ is a smooth harmonic function in $\Omega$ (for each fixed $y$) chosen so that $G(\cdot, y) = 0$ on $\partial\Omega$.

## Green's Function for the Laplacian

**Dirichlet Green's function.** For the Dirichlet problem $-\Delta u = f$ in $\Omega$, $u = 0$ on $\partial\Omega$:

$$G(x,y) = E(x-y) + h(x,y),$$

where $E(x-y)$ is the fundamental solution of the Laplacian and $h(x,y)$ is the unique harmonic function in $\Omega$ (for each $y \in \Omega$) with boundary values $h(\cdot, y)|_{\partial\Omega} = -E(\cdot - y)|_{\partial\Omega}$.

**Properties of $G$:**
1. $G(x,y) = G(y,x)$ (symmetry, from the self-adjointness of $-\Delta$).
2. $G(x,y) < 0$ for $x, y \in \Omega$ (for domains in $\mathbb{R}^n$, $n \geq 2$, from the maximum principle and the sign of $E$). Wait—actually for the sign convention $-\Delta G = \delta$ in $\mathbb{R}^3$: $G = 1/(4\pi|x-y|) + h > 0$ typically.
3. $G(x,y) \to +\infty$ as $x \to y$ (same singularity as $E$).
4. $G(\cdot, y) = 0$ on $\partial\Omega$.

**Green's representation formula.** For a harmonic function $u$ in $\Omega$ (i.e., $\Delta u = 0$):

$$u(y) = \int_{\partial\Omega} \left[u(x) \frac{\partial G}{\partial \nu_x}(x,y) - G(x,y)\frac{\partial u}{\partial\nu}(x)\right] d\sigma(x).$$

For the Dirichlet problem ($G = 0$ on $\partial\Omega$), the second term vanishes, and the formula becomes:

$$u(y) = \int_{\partial\Omega} u(x) \frac{\partial G}{\partial \nu_x}(x,y) \, d\sigma(x) = \int_{\partial\Omega} g(x) P(x,y) \, d\sigma(x),$$

where $P(x,y) = \partial G/\partial\nu_x$ is the **Poisson kernel**. This solves the Dirichlet problem: given boundary data $g$ on $\partial\Omega$, the unique harmonic extension to $\Omega$ is given by integration against the Poisson kernel.

## Explicit Green's Functions

**The half-space $\mathbb{R}^n_+ = \{x_n > 0\}$:** For the Dirichlet Laplacian, the Green's function is given by the **method of images**: reflect the source point $y = (y', y_n)$ to $y^* = (y', -y_n)$ (across the boundary), and set:

$$G(x,y) = E(x-y) - E(x-y^*).$$

Verification: $G(x,y) = 0$ when $x_n = 0$ (since $E(x-y) = E(x-y^*)$ for $x_n = 0$), and $\Delta_x G = \delta(x-y) - 0 = \delta(x-y)$ for $x_n > 0$.

The Poisson kernel for the half-space is:

$$P(x', y) = -\frac{\partial G}{\partial x_n}\bigg|_{x_n=0} = \frac{2y_n}{\omega_n |x - y|^n} \bigg|_{x_n=0} = \frac{2y_n}{\omega_n (|x'-y'|^2 + y_n^2)^{n/2}}.$$

**The ball $B_R(0)$:** The Green's function uses the Kelvin transform (inversion in a sphere). For $y \in B_R$ and $y^* = R^2 y/|y|^2$ (the reflected point outside the ball):

$$G(x,y) = E(x-y) - \left(\frac{R}{|y|}\right)^{n-2} E(x-y^*), \quad n \geq 3.$$

## Green's Functions for ODE: The Sturm-Liouville Problem

For a Sturm-Liouville operator $Lf = -(p(x)f')' + q(x)f$ on $[a,b]$ with self-adjoint boundary conditions, the Green's function $G(x,t)$ satisfies $L_x G(x,t) = \delta(x-t)$ with the given boundary conditions.

**Explicit construction.** Let $u_1$ be the solution of $Lu = 0$ satisfying the left boundary condition, and $u_2$ the solution satisfying the right boundary condition. The Green's function is:

$$G(x,t) = \frac{1}{W(t)}\begin{cases} u_1(x)u_2(t) & x < t \\ u_1(t)u_2(x) & x > t, \end{cases}$$

where $W(t) = p(t)(u_1(t)u_2'(t) - u_1'(t)u_2(t))$ is the Wronskian (times $p$, which is constant by Abel's theorem).

**Distributional verification.** $L_x G(x,t) = 0$ for $x \neq t$ (both pieces satisfy $Lu = 0$). At $x = t$: $G$ is continuous (no jump in $G$), but $\partial_x G$ has a jump $[\partial_x G]_{x=t} = 1/p(t)$ (from the Wronskian normalization). The distributional second derivative contributes $-p(t) \cdot (1/p(t))\delta(x-t) \cdot (-1) = \delta(x-t)$. (The sign conventions here depend on the precise form of $L$; the key point is that the jump in the first derivative of $G$ at $x = t$ is exactly calibrated to produce the delta function on the right-hand side.)

## Spectral Expansion of the Green's Function

For a self-adjoint operator $L$ with eigenfunctions $\phi_n$ and eigenvalues $\lambda_n$ (assuming a discrete spectrum), the Green's function has the spectral expansion:

$$G(x,y) = \sum_{n=1}^\infty \frac{\phi_n(x)\overline{\phi_n(y)}}{\lambda_n},$$

convergent in a suitable distributional sense. This connection between the Green's function and the eigenfunction expansion is the bridge between distribution theory and spectral theory (developed in Unit 3, Chapter 3).
