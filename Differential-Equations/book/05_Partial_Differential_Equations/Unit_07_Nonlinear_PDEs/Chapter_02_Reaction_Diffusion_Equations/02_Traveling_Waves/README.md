# Traveling Wave Solutions

A **traveling wave solution** of a reaction-diffusion equation $u_t = Du_{xx} + f(u)$ is a solution of the special form $u(x,t) = U(x - ct)$ for some wave speed $c$ and profile $U$. The profile $U(\xi)$ (with $\xi = x-ct$) satisfies an ODE, and the PDE problem reduces to finding an appropriate heteroclinic orbit of this ODE in the phase plane. Traveling waves capture the asymptotic behavior of solutions that start from step-like initial data: after long times, the solution looks like a translated version of $U$, propagating at speed $c$ without changing shape.

## General Framework

For $u_t = Du_{xx} + f(u)$ with $f(0) = f(1) = 0$, seek $u = U(\xi)$, $\xi = x-ct$:

$$DU'' + cU' + f(U) = 0, \qquad U(-\infty) = 1, \quad U(+\infty) = 0. \tag{TW}$$

This is a second-order ODE for $U(\xi)$. Setting $V = U'$:

$$U' = V, \qquad V' = -\frac{1}{D}[cV + f(U)]. \tag{System}$$

The traveling wave is a heteroclinic orbit of (System) from $(U,V) = (1,0)$ to $(U,V) = (0,0)$.

The qualitative behavior of the orbit depends critically on the type of nonlinearity:

**Monostable.** $f(0) = f(1) = 0$, $f(u) > 0$ for $u \in (0,1)$. Both equilibria $(0,0)$ and $(1,0)$ are accessible as limits of the wave profile. In the phase plane, $(0,0)$ is a stable node/spiral and $(1,0)$ is a saddle. There is a **continuum of wave speeds** $c \geq c^*$ (Fisher-KPP case).

**Bistable.** $f$ has three zeros: $f(0) = f(\alpha) = f(1) = 0$ ($\alpha \in (0,1)$), with $f < 0$ on $(0,\alpha)$ and $f > 0$ on $(\alpha,1)$. Both $u=0$ and $u=1$ are stable equilibria; $u=\alpha$ is unstable. Traveling waves exist connecting $u=1$ to $u=0$ at a **unique wave speed** $c^*$ (positive or negative, depending on which state is "more stable"). The speed $c^*$ is determined by a Rankine-Hugoniot-like condition: $c^* \int_0^1 (U')^2\,d\xi = \int_0^1 f(U)\,dU$.

**Combustion type.** $f$ has a unique zero at $u=1$ and $f(u) = 0$ for $u \in [0,\theta]$ (ignition temperature threshold $\theta \in (0,1)$). Traveling waves exist for a unique speed $c^* > 0$, determined by the threshold and the shape of $f$.

## Monostable Case: Fisher-KPP

Revisiting the Fisher-KPP wave existence argument more carefully. At $(0,0)$, the eigenvalues of the linearized system (System) are $\lambda_{1,2} = (-c \pm \sqrt{c^2-4rD})/(2D)$.

**Case $c > c^* = 2\sqrt{rD}$:** Both $\lambda_{1,2}$ are real and negative. $(0,0)$ is a stable node. The one-dimensional unstable manifold of $(1,0)$ (a saddle) enters the region $\{0 < U < 1, V < 0\}$ and must reach $(0,0)$ along one of the two eigendirections. This gives a monotone decreasing wave profile.

**Case $c = c^*$:** $\lambda_{1,2} = -c^*/(2D) < 0$ (repeated eigenvalue). $(0,0)$ is an improper node. The unstable manifold of $(1,0)$ still reaches $(0,0)$.

**Case $c < c^*$:** $\lambda_{1,2}$ are complex (stable spiral). Any orbit approaching $(0,0)$ oscillates — $U$ takes negative values near $\xi = +\infty$, violating $U \geq 0$.

**Minimum speed selection.** For compactly supported (or sufficiently rapidly decaying) initial data, the solution $u(x,t)$ converges to the wave $U_{c^*}(\xi - c^*t - \xi_0)$ for some phase $\xi_0$ as $t\to\infty$. The **minimal speed** $c^*$ is selected from the spectrum $[c^*,\infty)$ of admissible speeds.

**Proof of existence (shooting argument).** For $c \geq c^*$: the right-hand side of (TW) at $U=1$ is $cU'(1)$, so the linearized equation at the saddle $(1,0)$ has a negative eigenvalue corresponding to an eigenvector with $V < 0$, $U = 1$. Follow the unstable manifold from $(1,0)$ into $\{0 < U < 1, V < 0\}$. Claim: it reaches $(0,0)$ without crossing $V = 0$ for $U \in (0,1)$ (which would mean $U' = 0$ at some interior point, contradicting monotonicity from a careful comparison argument). By the Poincaré-Bendixson theorem, the orbit must converge to the only other equilibrium, $(0,0)$.

## Bistable Case: Unique Wave Speed

For bistable $f$ with $\int_0^1 f(u)\,du > 0$ (the stable state $u=1$ is "energetically favored"):

**Theorem.** There exists a unique $c^* \in \mathbb{R}$ (with sign: $c^* > 0$ if $\int_0^1 f > 0$, $c^* < 0$ if $\int_0^1 f < 0$, $c^* = 0$ if $\int_0^1 f = 0$) such that (TW) has a monotone decreasing solution for $c = c^*$ only.

**Speed formula.** Multiply (TW) by $U'$ and integrate:

$$D\int_{-\infty}^{+\infty}(U')^2V\,d\xi + c\int_{-\infty}^{+\infty}(U')^2\,d\xi + \int_{-\infty}^{+\infty}f(U)U'\,d\xi = 0.$$

The first term vanishes (integration by parts), the third term is $\int_1^0 f(U)\,dU = -\int_0^1 f(U)\,dU$:

$$c^* = \frac{\int_0^1 f(U)\,dU}{\int_{-\infty}^{+\infty}(U')^2\,d\xi}.$$

Since $(U')^2 \geq 0$ and the denominator is positive, $\text{sgn}(c^*) = \text{sgn}(\int_0^1 f)$.

## Worked Example: Nagumo's Equation (Bistable)

The **Nagumo equation** (also called the Fitzhugh-Nagumo equation in this context):

$$u_t = Du_{xx} + u(1-u)(u-\alpha), \qquad \alpha \in (0,1/2).$$

The reaction term $f(u) = u(1-u)(u-\alpha)$ has zeros at $0$, $\alpha$, $1$, with $f < 0$ on $(0,\alpha)$ and $f > 0$ on $(\alpha,1)$.

$$\int_0^1 f(u)\,du = \int_0^1 u(1-u)(u-\alpha)\,du = \frac{1}{12} - \frac{\alpha}{6} = \frac{1-2\alpha}{12}.$$

For $\alpha < 1/2$: $\int_0^1 f > 0$, so $c^* > 0$ (the wave advances in the $+x$ direction, $u=1$ invades $u=0$).

**Exact traveling wave.** The Nagumo equation has an explicit traveling wave. Seeking $U(\xi)$ of the form $U = (1+e^{k\xi})^{-1}$ (a logistic profile), substituting into (TW), and matching coefficients of $e^{k\xi}$, $e^{2k\xi}$, and constants:

$$k = \frac{1}{\sqrt{2D}}, \qquad c^* = \frac{1-2\alpha}{\sqrt{2D}}\cdot D = \sqrt{D/2}(1-2\alpha).$$

More precisely: $U = (1+e^{\xi/\sqrt{2D}})^{-1}$ satisfies (TW) with $c^* = (1-2\alpha)\sqrt{D/2}$.

**Verification:** $U' = -e^\xi/(1+e^\xi)^2\cdot(1/\sqrt{2D})$. $U'' = \cdots$. Computing $DU'' + c^*U' + f(U)$:

$U(1-U)(U-\alpha) = (1+e^\xi)^{-1}\cdot e^\xi(1+e^\xi)^{-1}\cdot[(1+e^\xi)^{-1}-\alpha]$... The verification is straightforward by direct substitution and shows the ansatz works exactly.

## Wave Stability

The traveling wave $U(x-c^*t)$ is an **asymptotically stable** solution of the reaction-diffusion equation in the following sense: perturbations of the wave profile decay as $t\to\infty$, and the perturbed solution converges to a translate $U(x-c^*t-\xi_0)$ (shift by phase $\xi_0$ determined by the initial perturbation).

For monostable waves at the minimum speed $c^*$: the convergence is only **algebraic** in time (the phase shift converges, but the convergence is $O(1/\sqrt{t})$ for the profile) — slower than for bistable waves, where the convergence is exponential.

**Proof of stability (bistable case, sketch).** The linearized operator around the wave is $\mathcal{L}v = Dv'' + c^*v' + f'(U)v$. The spectrum of $\mathcal{L}$ consists of:
- A simple eigenvalue $\lambda = 0$ with eigenfunction $v = U'$ (translation invariance).
- The essential spectrum, which lies in $\{\text{Re}\,\lambda < -\mu\}$ for some $\mu > 0$ (since $f'(0)$ and $f'(1)$ are negative — the equilibria are stable).

The absence of positive eigenvalues (proved by a Sturm-Liouville argument) shows that the wave is nonlinearly asymptotically stable modulo translation.

## Multi-Dimensional Traveling Waves

In $\mathbb{R}^n$, a **planar traveling wave** is $u = U(\mathbf{e}\cdot\mathbf{x} - ct)$ for some unit vector $\mathbf{e}$. The ODE for $U$ is the same (TW) with $D$ replaced by $D$ (isotropic diffusion), giving the same wave speeds. However, curved wave fronts also exist: for bistable equations, there are spherically expanding waves, cylindrically symmetric waves ("V-shaped" waves), and entire solutions (solutions defined for all $t\in\mathbb{R}$).

For monostable equations, the spreading is anisotropic if $D$ is anisotropic (different diffusion rates in different directions), and the asymptotic shape of the spreading front is determined by the **Wulff shape** of the reaction-diffusion system — a convex set whose support function is the spreading speed in each direction.
