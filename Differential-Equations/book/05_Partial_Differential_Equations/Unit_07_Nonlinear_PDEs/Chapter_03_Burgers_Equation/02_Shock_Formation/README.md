# Shock Formation in Burgers' Equation

The inviscid Burgers equation $u_t + uu_x = 0$ provides the cleanest model of shock formation in nonlinear hyperbolic equations. The mechanism is geometric: characteristics are straight lines along which $u$ is constant, and if the initial data has a decreasing region ($u_0' < 0$ somewhere), these characteristics converge and eventually intersect at a finite time $T^*$, the **shock time**. After $T^*$, the classical (smooth) solution ceases to exist, and the equation must be interpreted in the weak sense. The entropy condition then selects the unique physically relevant weak solution — a propagating shock. This section develops the complete theory: shock formation time, shock position, shock speed via Rankine-Hugoniot, and the entropy condition that ensures uniqueness.

## Method of Characteristics Revisited

The inviscid Burgers equation $u_t + uu_x = 0$ is equivalent to $du/dt = 0$ along characteristics $dx/dt = u$. Given $u(x,0) = u_0(x)$, the characteristic through the initial point $(x_0,0)$ is the line:

$$x = x_0 + u_0(x_0)\cdot t.$$

The solution is $u(x,t) = u_0(x_0)$ where $x_0$ is the (unique, if it exists) root of $x = x_0 + u_0(x_0)t$.

**Smooth solution existence.** The implicit function theorem gives a smooth solution as long as the map $x_0 \mapsto x_0 + u_0(x_0)t$ is invertible, i.e., $1 + u_0'(x_0)t \neq 0$ for all $x_0$. This fails when $1 + u_0'(x_0)t = 0$ for some $x_0$.

**Shock formation time:**

$$T^* = \frac{-1}{\min_{x_0} u_0'(x_0)} = \frac{1}{\max_{x_0}(-u_0'(x_0))}.$$

If $u_0' \geq 0$ everywhere, $T^* = +\infty$: no shock forms (rarefaction). If $u_0' < 0$ somewhere, $T^* < \infty$: a shock forms at the point $x_0^* = \arg\min u_0'(x_0)$.

## Example: Sinusoidal Initial Data

**Setup.** $u_0(x) = -\sin x$ (periodic, decreasing at $x=0$). Then $u_0'(x) = -\cos x$, with minimum $-1$ at $x=0$. Thus $T^* = 1$.

**Characteristics:** $x = x_0 - t\sin(x_0)$. For $t < 1$, this mapping is invertible (one-to-one). At $t = T^* = 1$: the characteristics from a neighborhood of $x_0 = 0$ (where $u_0' = -1$) first become tangent — the Jacobian $\partial x/\partial x_0 = 1 - t\cos(x_0)$ vanishes at $x_0 = 0$, $t = 1$.

**Solution at $t < T^*$:** Implicitly defined by $u = -\sin(x - ut)$. For example, at $x=0$, $t$ near $1$: $u = -\sin(-ut) = \sin(ut)$, so $u = \sin(ut)$. As $t \to T^* = 1$: $u \to u^*$ satisfying $u^* = \sin(u^*)$, with $u^* = 0$. The solution is still smooth, but its derivative $u_x = u_0'(x_0)/(1+u_0'(x_0)t)$ blows up as $t\to T^*$.

## The Equal Areas Rule (Maxwell Construction)

After $T^*$, the characteristic map is no longer injective: three different characteristics $x_0^{(1)} < x_0^{(2)} < x_0^{(3)}$ can reach the same point $x$ at time $t > T^*$. The multi-valued "solution" must be regularized by inserting a shock.

**Weak formulation.** A function $u \in L^\infty$ is a weak solution of $u_t + (u^2/2)_x = 0$ if for all test functions $\phi\in C_c^\infty(\mathbb{R}\times[0,\infty))$:

$$\int_0^\infty\int_{\mathbb{R}}\left[u\phi_t + \frac{u^2}{2}\phi_x\right]dx\,dt + \int_{\mathbb{R}}u_0(x)\phi(x,0)\,dx = 0.$$

If $u$ is piecewise smooth with a jump discontinuity at $x = s(t)$, the Rankine-Hugoniot condition (derived by applying the weak formulation with a test function supported near the shock) gives:

$$\dot{s} = \frac{u_L^2/2 - u_R^2/2}{u_L - u_R} = \frac{u_L + u_R}{2},$$

where $u_L = u(s(t)^-, t)$ and $u_R = u(s(t)^+, t)$ are the left and right limits at the shock.

**Equal areas rule.** The shock position $s(t)$ for Burgers' equation can also be determined geometrically: the shock cuts off equal areas on the left and right of the multi-valued solution. Precisely, if the characteristics crowd up and produce a multi-valued region $[x^{(1)}, x^{(3)}]$, the shock position $s$ satisfies:

$$\int_{x_0^{(1)}}^{s_0} u_0(x)\,dx = \int_{s_0}^{x_0^{(3)}}u_0(x)\,dx,$$

where $s_0$ is the position of the shock at formation. For subsequent times, the shock position can be tracked using the Rankine-Hugoniot condition.

## Entropy Condition

The Rankine-Hugoniot condition alone does not uniquely determine the shock: for Riemann data with $u_L < u_R$, there is a non-physical shock solution $u = u_L$ for $x < \bar{s}t$, $u = u_R$ for $x > \bar{s}t$ (with speed $\bar{s} = (u_L+u_R)/2$) in addition to the physical rarefaction wave. The entropy condition selects the physical one.

**Lax entropy condition for Burgers' equation.** A shock at speed $\dot{s} = \bar{s}$ with $u_L$ on the left and $u_R$ on the right is **admissible** (physical) if:

$$u_L > \bar{s} > u_R.$$

This says characteristics from the left ($dx/dt = u_L$) enter the shock, and characteristics from the right ($dx/dt = u_R$) also enter the shock — the shock is a **compressive** wave. For the non-physical "rarefaction shock" with $u_L < u_R$, characteristics would leave the shock on both sides, violating the condition.

**Oleinik entropy condition.** A more general version: $u$ is an entropy solution if for all $a$ between $u_R$ and $u_L$:

$$\frac{F(a) - F(u_L)}{a - u_L} \geq \dot{s} \geq \frac{F(a) - F(u_R)}{a - u_R},$$

where $F(u) = u^2/2$ is the flux. For Burgers' equation with $u_L > u_R$, this reduces to $u_L > \bar{s} > u_R$.

**Vanishing viscosity criterion.** The entropy solution is the unique limit of $u^\varepsilon$ (viscous Burgers solution) as $\varepsilon\to 0^+$. This is proved using the Hopf-Cole formula: the saddle-point analysis selects the global minimizer of $G(x,\cdot,t)$, which corresponds exactly to the entropy solution.

## Complete Solution of the Riemann Problem

For the Riemann initial data $u_0(x) = u_L\mathbf{1}_{x<0} + u_R\mathbf{1}_{x>0}$:

**Case 1: $u_L > u_R$ (shock).** The entropy solution is the traveling shock:

$$u(x,t) = \begin{cases}u_L & x < \bar{s}t \\ u_R & x > \bar{s}t\end{cases}, \qquad \bar{s} = \frac{u_L+u_R}{2}.$$

**Case 2: $u_L < u_R$ (rarefaction).** The entropy solution is the continuous rarefaction wave:

$$u(x,t) = \begin{cases}u_L & x < u_L t \\ x/t & u_L t \leq x \leq u_R t \\ u_R & x > u_R t\end{cases}.$$

## Shock Interactions and Long-Time Behavior

**Two-shock interaction.** Suppose the initial data has two shocks: $u_0 = A\mathbf{1}_{x<0} + B\mathbf{1}_{0<x<L} + C\mathbf{1}_{x>L}$ with $A > B$ and $B > C$ (two entropy shocks). The left shock moves at speed $(A+B)/2$ and the right shock at $(B+C)/2$. Since $(A+B)/2 > (B+C)/2$ (as $A > C$), the left shock catches the right shock after time $T_{\text{merge}} = L/[(A+B)/2-(B+C)/2] = 2L/(A-C)$. After merging, a single shock forms with $u_L = A$, $u_R = C$, speed $(A+C)/2$.

**$N$-wave behavior.** For compactly supported initial data $u_0$ with $\int u_0\,dx = 0$ and total variation $V_0 = \|u_0\|_{\text{TV}}$, the long-time behavior is:

$$u(x,t) \approx \frac{1}{t}N\!\left(\frac{x}{t}\right), \quad t \to \infty,$$

where $N$ is the "N-wave" profile — a rarefaction fan connecting $0$ to $(x/t)$ followed by a shock back to $0$. The $1/t$ decay and the self-similar $x/t$ dependence are the large-time universality class of Burgers' equation.

**Comparison with linear advection.** The linear equation $u_t + cu_x = 0$ propagates initial data without change. Burgers' equation distorts: slower parts of the wave fall behind, faster parts advance, creating a shock. This nonlinear steepening is the fundamental mechanism of wave breaking in shallow water, sonic booms in aerodynamics, and the formation of galaxy clusters in large-scale structure cosmology (Zeldovich approximation).
