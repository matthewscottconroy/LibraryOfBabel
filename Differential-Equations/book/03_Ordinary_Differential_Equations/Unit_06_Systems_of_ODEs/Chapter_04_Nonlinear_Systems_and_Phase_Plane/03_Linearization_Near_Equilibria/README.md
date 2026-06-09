# Linearization Near Equilibria

Near an equilibrium point, a nonlinear autonomous system behaves approximately like a linear system. The linearization replaces the nonlinear vector field by its best linear approximation at the equilibrium — the Jacobian matrix — and the behavior of the resulting linear system often faithfully reflects the local behavior of the nonlinear one. This principle, made precise by the Hartman-Grobman theorem, is the foundation of local stability analysis for nonlinear systems.

## Setting Up the Linearization

Let $(x^*, y^*)$ be an equilibrium of the autonomous system $x' = f(x,y)$, $y' = g(x,y)$, so that $f(x^*,y^*) = 0$ and $g(x^*,y^*) = 0$. Introduce small displacements $u = x - x^*$ and $v = y - y^*$. Then:

$$u' = x' = f(x^* + u, y^* + v).$$

Expanding in a Taylor series about $(x^*, y^*)$:

$$f(x^*+u, y^*+v) = f(x^*,y^*) + \frac{\partial f}{\partial x}\bigg|_{*}u + \frac{\partial f}{\partial y}\bigg|_{*}v + O(u^2 + v^2),$$

where $|_*$ denotes evaluation at $(x^*,y^*)$. Since $f(x^*,y^*) = 0$, the leading terms are linear in $(u,v)$. The **Jacobian matrix** of the system at the equilibrium is:

$$J = \begin{pmatrix}\partial f/\partial x & \partial f/\partial y \\ \partial g/\partial x & \partial g/\partial y\end{pmatrix}\bigg|_{(x^*,y^*)}.$$

The **linearized system** at $(x^*,y^*)$ is:

$$\begin{pmatrix}u' \\ v'\end{pmatrix} = J\begin{pmatrix}u \\ v\end{pmatrix}.$$

The higher-order terms $O(u^2+v^2)$ are negligible when $(u,v)$ is small, so the linearization is expected to be a good approximation near the equilibrium. The question is how faithfully the linear system represents the nonlinear one.

## The Hartman-Grobman Theorem

The Hartman-Grobman theorem gives precise conditions under which the linearization is qualitatively accurate:

**Theorem (Hartman-Grobman).** Let $(x^*,y^*)$ be an equilibrium of $\mathbf{x}' = \mathbf{F}(\mathbf{x})$ where $\mathbf{F}$ is $C^1$, and let $J$ be the Jacobian at the equilibrium. If $J$ is **hyperbolic** — meaning all eigenvalues of $J$ have nonzero real part — then the nonlinear system near $(x^*,y^*)$ is topologically conjugate to the linearization near the origin. In particular, the qualitative local behavior (node, saddle, spiral — stable or unstable) of the nonlinear system matches that of the linear one.

The theorem is a homeomorphism result: there exists a continuous bijection (with continuous inverse) mapping orbits of the nonlinear system to orbits of the linear system in a neighborhood of the equilibrium, preserving the direction of time. The homeomorphism is generally not smooth (not a diffeomorphism), but it preserves the topological type of the equilibrium.

**Non-hyperbolic equilibria** — those where $J$ has at least one purely imaginary eigenvalue (or a zero eigenvalue) — are excluded from the theorem. For these, the linearization can give qualitatively wrong information about the nonlinear system. The most common non-hyperbolic case is the center (purely imaginary eigenvalues), which may correspond to a true center or to a stable/unstable spiral in the nonlinear system.

## Worked Example: Competing Species

Consider the competing species model:

$$x' = x(2 - x - y), \qquad y' = y(3 - 2x - y).$$

The equilibria are the solutions of $x(2-x-y) = 0$ and $y(3-2x-y) = 0$. This gives four equilibria: $(0,0)$, $(2,0)$, $(0,3)$, and the interior equilibrium where $2-x-y=0$ and $3-2x-y=0$. Subtracting: $3-2x-y-(2-x-y) = 1-x = 0$, so $x = 1$ and $y = 1$: the interior equilibrium is $(1,1)$.

The Jacobian of $(f,g) = (x(2-x-y), y(3-2x-y))$ is:

$$J = \begin{pmatrix}2-2x-y & -x \\ -2y & 3-2x-2y\end{pmatrix}.$$

At $(1,1)$: $J = \begin{pmatrix}-1 & -1 \\ -2 & -1\end{pmatrix}$. Eigenvalues: $\lambda^2 + 2\lambda + 1 - 2 = \lambda^2 + 2\lambda - 1 = 0$, so $\lambda = -1 \pm \sqrt{2}$. The eigenvalues are real with opposite signs: $\lambda_1 = -1+\sqrt{2} > 0$ and $\lambda_2 = -1-\sqrt{2} < 0$. The interior equilibrium is a **saddle point**.

At $(2,0)$: $J = \begin{pmatrix}-2&-2\\0&-1\end{pmatrix}$. Eigenvalues $-2$ and $-1$: both negative, so $(2,0)$ is a **stable node**.

At $(0,3)$: $J = \begin{pmatrix}-1&0\\-6&-3\end{pmatrix}$. Eigenvalues $-1$ and $-3$: both negative, so $(0,3)$ is a **stable node**.

At $(0,0)$: $J = \begin{pmatrix}2&0\\0&3\end{pmatrix}$. Eigenvalues $2$ and $3$: both positive, so $(0,0)$ is an **unstable node**.

The phase portrait shows that both $(2,0)$ and $(0,3)$ are stable, and the saddle at $(1,1)$ separates their basins of attraction. Depending on the initial condition, the competing species either reach a state where species $x$ wins (population $(2,0)$) or species $y$ wins (population $(0,3)$). The saddle's stable manifold forms the separatrix between these two basins — a curve dividing the first quadrant into two regions.

## Limitations of Linearization: The Center Case

The most important failure case of linearization occurs when the equilibrium is a center of the linearized system (eigenvalues $\pm i\beta$ with $\beta \neq 0$). The nonlinear system may be a true center (if a conserved quantity exists), a stable spiral, or an unstable spiral. The linearization alone cannot distinguish these cases.

**Example.** The system $x' = -y + x(x^2+y^2)$, $y' = x + y(x^2+y^2)$ has linearization at the origin with eigenvalues $\pm i$. Converting to polar coordinates: $r' = r^3 > 0$. All solutions spiral outward — the origin is an unstable spiral, not a center, even though the linearization suggests a center.

Contrast with $x' = -y - x(x^2+y^2)$, $y' = x - y(x^2+y^2)$: now $r' = -r^3 < 0$ and all solutions spiral toward the origin. The origin is a stable spiral. In both cases, the linearization is the same and gives eigenvalues $\pm i$; only the nonlinear terms determine the true behavior.

To analyze non-hyperbolic equilibria, one must use higher-order terms in the Taylor expansion, apply Lyapunov's method, use center manifold reduction, or compute the normal form of the system. These techniques go beyond what linearization alone provides.

## The Jacobian in Higher Dimensions

For an $n$-dimensional system $\mathbf{x}' = \mathbf{F}(\mathbf{x})$ with equilibrium $\mathbf{x}^*$, the Jacobian is the $n\times n$ matrix $J_{ij} = \partial F_i/\partial x_j$ evaluated at $\mathbf{x}^*$. The linearized system is $\mathbf{u}' = J\mathbf{u}$, and the Hartman-Grobman theorem applies: if all eigenvalues of $J$ have nonzero real part, the local qualitative behavior of the nonlinear system matches the linear one. An equilibrium is **locally asymptotically stable** if all eigenvalues of $J$ have strictly negative real part; it is **unstable** if any eigenvalue has positive real part. The mixed case (some eigenvalues on each side) gives a saddle-type equilibrium.

Linearization is computationally accessible for any $C^1$ system and provides the starting point for stability analysis. Its limitations at non-hyperbolic equilibria motivate the development of Lyapunov's method, which provides global stability information and applies even when linearization is inconclusive.
