# Fisher's Equation

In 1937, Ronald Fisher published "The wave of advance of advantageous genes," modeling the spatial spread of a beneficial mutation through a population. The equation he proposed:

$$u_t = Du_{xx} + ru(1-u), \quad x\in\mathbb{R}, \; t > 0, \tag{Fisher-KPP}$$

where $u(x,t) \in [0,1]$ is the frequency of the advantageous allele at position $x$ and time $t$, $D > 0$ is the dispersal rate, and $r > 0$ is the growth rate advantage. In the same year, Kolmogorov, Petrovskii, and Piskunov (KPP) proved the main mathematical results for this equation; it is now known as the **Fisher-KPP equation**.

The equation is a paradigm for reaction-diffusion dynamics: the reaction term $f(u) = ru(1-u)$ has zeros at $u=0$ (unstable equilibrium) and $u=1$ (stable equilibrium), and the diffusion term $Du_{xx}$ couples the evolution at different spatial locations. The main mathematical phenomenon is the propagation of the stable state $u=1$ into the unstable state $u=0$ in the form of a **traveling wave front**.

## Phase Plane Analysis of the Steady States

Before looking for traveling waves, examine the spatially homogeneous ODE $\dot u = ru(1-u)$ (the logistic equation): $u=0$ is an unstable equilibrium and $u=1$ is a stable equilibrium. All solutions with $u_0 \in (0,1]$ satisfy $u(t) \to 1$ as $t\to\infty$.

For the PDE, the maximum principle gives: if $0 \leq u(x,0) \leq 1$, then $0 \leq u(x,t) \leq 1$ for all $t > 0$. This follows because $u=0$ and $u=1$ are sub- and supersolutions respectively (or by comparison with the ODE).

## Traveling Wave Solutions

Seek solutions of the form $u(x,t) = U(x-ct)$, a wave profile $U$ traveling to the right at speed $c > 0$. Setting $\xi = x-ct$:

$$-cU' = DU'' + rU(1-U),$$

or equivalently:

$$DU'' + cU' + rU(1-U) = 0. \tag{ODE}$$

The boundary conditions express that the wave connects the two equilibria:

$$U(-\infty) = 1, \quad U(+\infty) = 0. \tag{BC}$$

(The wave front advances into the $u=0$ region as time progresses.)

**Phase plane.** Set $V = U'$. The system (ODE) becomes the first-order system:

$$U' = V, \qquad V' = \frac{1}{D}[-cV - rU(1-U)].$$

The equilibria of this system are $(U,V) = (0,0)$ and $(U,V) = (1,0)$.

**Linearization at $(0,0)$:** The Jacobian is $\begin{pmatrix}0 & 1 \\ -r/D & -c/D\end{pmatrix}$, with eigenvalues:

$$\lambda_{1,2} = \frac{-c \pm \sqrt{c^2 - 4rD}}{2D}.$$

- If $c^2 > 4rD$ (i.e., $c > c^* = 2\sqrt{rD}$): both eigenvalues are real and negative ($(0,0)$ is a stable node). Solutions approaching $(0,0)$ from the left (i.e., $U \to 0^+$ as $\xi\to+\infty$) can do so monotonically.
- If $c^2 < 4rD$ (i.e., $c < c^* = 2\sqrt{rD}$): eigenvalues are complex with negative real part (stable spiral). Solutions approach $(0,0)$ while oscillating — $U$ takes negative values, violating $U \geq 0$.

**Linearization at $(1,0)$:** The Jacobian is $\begin{pmatrix}0 & 1 \\ r/D & -c/D\end{pmatrix}$, with eigenvalues $\frac{-c \pm \sqrt{c^2+4rD}}{2D}$. Since $\sqrt{c^2+4rD} > c$ for all $c > 0$, one eigenvalue is positive and one is negative — $(1,0)$ is a saddle point.

**Existence of heteroclinic orbit.** For each $c \geq c^* = 2\sqrt{rD}$, there exists a monotone decreasing solution $U(\xi)$ connecting $(1,0)$ to $(0,0)$ in the phase plane (a heteroclinic orbit from the saddle to the node). This is proved by showing the unstable manifold of $(1,0)$ enters the region $\{0 < U < 1, V < 0\}$ and, for $c \geq c^*$, reaches $(0,0)$ monotonically.

**Theorem (KPP, 1937).** For each $c \geq c^* = 2\sqrt{rD}$, equation (ODE)-(BC) has a monotone decreasing solution $U_c(\xi)$, unique up to translation. For $c < c^*$, no monotone solution exists.

## The Minimum Speed $c^* = 2\sqrt{rD}$

The critical speed $c^*$ has a beautiful interpretation: it is determined by the **linearization at the leading edge** ($u \approx 0$, $\xi \to +\infty$). Near $u=0$: $f(u) = ru(1-u) \approx ru$. The linearized Fisher equation is $u_t = Du_{xx} + ru$, a linear heat equation with growth. Seeking $u \sim e^{-\lambda\xi} e^{0\cdot t}$ (a wave front that moves at speed $c$ without changing shape):

$$0 = D\lambda^2 - c\lambda + r \implies c = D\lambda + r/\lambda.$$

Minimizing over $\lambda > 0$: $\frac{dc}{d\lambda} = D - r/\lambda^2 = 0 \implies \lambda^* = \sqrt{r/D}$, giving $c^* = 2\sqrt{rD}$.

This is the **linear spreading speed**: the minimum speed at which the linearized equation at $u=0$ can support exponentially decaying solutions. The nonlinear Fisher equation "locks in" to this linear speed for initial data with sufficient decay.

## The Fisher-KPP Theorem (Long-Time Behavior)

**Theorem.** Suppose $0 \leq u_0 \leq 1$, $u_0 \not\equiv 0$, and $u_0$ has compact support (or exponential decay as $|x|\to\infty$ faster than $e^{-c^*x/(2D)}$). Then the solution of Fisher-KPP satisfies:

1. For any $\sigma > c^*$: $\sup_{|x| \geq \sigma t}u(x,t) \to 0$ as $t\to\infty$.
2. For any $\sigma < c^*$: $\inf_{|x| \leq \sigma t}u(x,t) \to 1$ as $t\to\infty$.
3. The transition zone (where $u$ changes from near $1$ to near $0$) moves at speed $c^*$ and sharpens to the traveling wave profile $U_{c^*}$.

In words: the gene allele spreads at speed $c^* = 2\sqrt{rD}$ — a precise mathematical prediction for population genetics that has been confirmed experimentally.

**Proof sketch (upper bound, $\sigma > c^*$).** The function $\bar u(x,t) = \min(1, e^{-\lambda(x-ct)})$ for $c = D\lambda + r/\lambda$ is a supersolution of Fisher-KPP (since $e^{-\lambda(x-ct)}$ solves the linearized equation and the reaction term $r\bar u(1-\bar u) \leq r\bar u$). By comparison, $u(x,t) \leq \bar u(x,t)$ for all $t > 0$, so $u \to 0$ for $x > \sigma t$.

## Dimensionless Form

Setting $\tilde u = u$, $\tilde t = rt$, $\tilde x = x/\sqrt{D/r}$ (scaling time by $1/r$ and space by the diffusion length $\sqrt{D/r}$):

$$\frac{\partial\tilde u}{\partial\tilde t} = \frac{\partial^2\tilde u}{\partial\tilde x^2} + \tilde u(1-\tilde u).$$

The dimensionless Fisher-KPP equation has no free parameters; the critical wave speed in dimensionless units is $\tilde c^* = 2$. The ratio of the Fisher speed $c^* = 2\sqrt{rD}$ to the RMS displacement per unit time $\sqrt{D}$ is $c^*/\sqrt{D} = 2\sqrt{r}$ — so faster reaction (larger $r$) gives faster spread, as expected.

## Generalizations

**KPP condition.** The Fisher-KPP theory applies more generally to any $f$ satisfying: $f(0) = f(1) = 0$, $f(u) > 0$ for $u \in (0,1)$, and $f'(0) = \max_{u\in[0,1]}f(u)/u$ (the KPP condition). Under these conditions, $c^* = 2\sqrt{Df'(0)}$.

**Multidimensional Fisher-KPP.** In $\mathbb{R}^n$: $u_t = D\Delta u + ru(1-u)$. Traveling wave fronts exist in every direction, all at speed $c^*$. For radially symmetric initial data, the expanding front remains approximately radially symmetric and advances at speed $c^*$ in the radial direction.

**Heterogeneous media.** In periodic or random environments, the spreading speed $c^*$ is replaced by an effective speed $c^*_{\text{eff}}$ determined by homogenization theory or large deviation theory for random walks in random environments.
