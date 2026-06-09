# Conservation Laws

A conservation law is a PDE that expresses the conservation of some physical quantity — mass, momentum, energy, charge — within a spatial region. The prototypical scalar conservation law in one space dimension is

$$u_t + f(u)_x = 0, \qquad x \in \mathbb{R},\; t > 0, \tag{1}$$

where $u(x,t)$ is the density of the conserved quantity and $f(u)$ is its flux (the rate at which the quantity crosses a unit area per unit time, in the positive $x$-direction). The equation says that any change in the total amount of $u$ in an interval $[a,b]$ must be due to flux through the endpoints:

$$\frac{d}{dt}\int_a^b u(x,t)\,dx = f(u(a,t)) - f(u(b,t)).$$

This integral form is the true statement of conservation and is meaningful even for discontinuous $u$.

## Derivation from Physical Principles

Consider a fluid with density $\rho(x,t)$ and velocity $v(x,t)$ flowing in a pipe of unit cross-section. The mass in $[a,b]$ at time $t$ is $\int_a^b \rho\,dx$. Conservation of mass requires:

$$\frac{d}{dt}\int_a^b \rho\,dx = \rho(a,t)v(a,t) - \rho(b,t)v(b,t) = -[(\rho v)_x]_{x=a}^{x=b}\cdot(b-a).$$

Since $[a,b]$ is arbitrary, differentiating gives the continuity equation $\rho_t + (\rho v)_x = 0$. This is a conservation law with $u = \rho$ and $f(u) = \rho v$.

**Traffic flow.** A classical model: let $u(x,t)$ be the density of cars (cars per kilometer) on a highway. The flux is $f(u) = uV(u)$, where $V(u)$ is the average speed as a function of density. A simple model is $V(u) = v_{\max}(1 - u/u_{\max})$ (speed decreases linearly with density). Then $f(u) = v_{\max}u(1-u/u_{\max})$, which is the Lighthill-Whitham-Richards (LWR) model. The PDE $u_t + f(u)_x = 0$ governs the evolution of traffic density.

## The Riemann Problem

The **Riemann problem** is the initial value problem for (1) with piecewise constant initial data:

$$u(x,0) = \begin{cases} u_L & x < 0 \\ u_R & x > 0 \end{cases}.$$

For Burgers' equation $f(u) = u^2/2$ (so $f'(u) = u$), there are two cases:

**Shock ($u_L > u_R$):** The characteristics from the left have speed $u_L >$ characteristics from the right with speed $u_R$. They converge and cross immediately at $t=0^+$, forming an instantaneous shock. The shock speed from the Rankine-Hugoniot condition is

$$s = \frac{f(u_R) - f(u_L)}{u_R - u_L} = \frac{u_R^2/2 - u_L^2/2}{u_R - u_L} = \frac{u_L + u_R}{2}.$$

The solution is: $u = u_L$ for $x < st$ and $u = u_R$ for $x > st$.

**Rarefaction ($u_L < u_R$):** The characteristics diverge, leaving a "vacuum" in between. The solution fills this gap with a smooth rarefaction wave:

$$u(x,t) = \begin{cases} u_L & x < u_L t \\ x/t & u_L t \leq x \leq u_R t \\ u_R & x > u_R t \end{cases}.$$

Check: in the fan region $u = x/t$, so $u_t = -x/t^2$ and $u_x = 1/t$. Then $u_t + u u_x = -x/t^2 + (x/t)(1/t) = 0$. Correct.

## Systems of Conservation Laws

Many important physical systems are described by systems of conservation laws:

$$\mathbf{u}_t + \mathbf{f}(\mathbf{u})_x = 0, \qquad \mathbf{u}, \mathbf{f} \in \mathbb{R}^n.$$

**The Euler equations** for compressible gas dynamics in one dimension form a $3\times 3$ system with $\mathbf{u} = (\rho, \rho v, E)^T$ (density, momentum density, total energy density) and flux $\mathbf{f} = (\rho v, \rho v^2 + p, v(E+p))^T$, closed by the equation of state $p = p(\rho, e)$ where $e = E/\rho - v^2/2$ is the specific internal energy.

The eigenvalues of the Jacobian matrix $D\mathbf{f}(\mathbf{u})$ are the characteristic speeds $v-c$, $v$, $v+c$ where $c = \sqrt{\partial p/\partial\rho|_s}$ is the sound speed. Shock waves correspond to discontinuities propagating at speeds satisfying the Rankine-Hugoniot conditions for the full system.

## Convex Flux and Genuine Nonlinearity

A scalar conservation law with **convex** flux ($f''(u) > 0$ everywhere) is the cleanest case. The characteristic speed $f'(u)$ is strictly increasing with $u$: faster-moving parts of the wave have larger $u$-values. The Riemann problem has a unique entropy solution (either a shock or a rarefaction, but not both). The well-posedness theory for $L^1 \cap L^\infty$ initial data was worked out by Kruzkov and others in the 1960s-70s and provides a complete global existence and uniqueness theory.

For non-convex fluxes (e.g., $f(u) = u^3$ or the cubic Buckley-Leverett flux in porous media flow), the Riemann problem can have composite waves (compound shocks and rarefactions), and the admissibility theory is more subtle.

## Self-Similar Solutions and Scaling

The Riemann problem has a self-similar structure: the solution depends only on $x/t$ (not on $x$ and $t$ separately). This is because the problem is invariant under the scaling $(x,t)\mapsto(\lambda x, \lambda t)$ for any $\lambda > 0$. More generally, the conservation law (1) is scale-invariant when $f(u) = cu^k$ for some constants $c$ and $k$.

Self-similar solutions are among the most important exact solutions in PDE theory, providing insights into the structure of solutions near singularities and at long times, and serving as building blocks for more complex wave patterns in systems.
