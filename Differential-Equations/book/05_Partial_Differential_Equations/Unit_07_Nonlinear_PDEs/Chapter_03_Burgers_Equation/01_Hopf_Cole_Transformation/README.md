# The Hopf-Cole Transformation

The viscous Burgers equation $u_t + uu_x = \varepsilon u_{xx}$ is nonlinear, but it is exactly equivalent to the linear heat equation via a change of dependent variable. This equivalence — the **Hopf-Cole transformation** — was discovered independently by E. Hopf (1950) and J. D. Cole (1951) and represents one of the rare cases where a nonlinear PDE can be solved completely in closed form. The transformation converts the nonlinear problem into the linear heat equation, whose solution is known explicitly via the heat kernel. The result is an exact, global formula for the viscous Burgers solution valid for all $t > 0$ and all initial data in $L^\infty$.

## Derivation of the Transformation

**Step 1: Potential.** Introduce the potential $\psi(x,t)$ by $u = \psi_x$ (possible since the equation is posed on $\mathbb{R}$, and $u\to 0$ as $|x|\to\infty$ can be assumed). Substituting into Burgers' equation:

$$\psi_{xt} + \psi_x\psi_{xx} = \varepsilon\psi_{xxx}.$$

This integrates to (integrating in $x$):

$$\psi_t + \frac{1}{2}\psi_x^2 = \varepsilon\psi_{xx}. \tag{Hamilton-Jacobi}$$

This is the **viscous Hamilton-Jacobi equation**. It is a nonlinear PDE, but the nonlinearity is only in the $(\psi_x)^2$ term.

**Step 2: Exponential substitution.** Set $\psi = -2\varepsilon\log\phi$ (equivalently, $\phi = e^{-\psi/(2\varepsilon)}$). Then:

$$\psi_t = -2\varepsilon\frac{\phi_t}{\phi}, \qquad \psi_x = -2\varepsilon\frac{\phi_x}{\phi}, \qquad \psi_{xx} = -2\varepsilon\left(\frac{\phi_{xx}}{\phi} - \frac{\phi_x^2}{\phi^2}\right).$$

Substituting into the Hamilton-Jacobi equation:

$$-2\varepsilon\frac{\phi_t}{\phi} + \frac{1}{2}\cdot 4\varepsilon^2\frac{\phi_x^2}{\phi^2} = \varepsilon\left[-2\varepsilon\frac{\phi_{xx}}{\phi} + 2\varepsilon\frac{\phi_x^2}{\phi^2}\right].$$

Simplifying:

$$-2\varepsilon\frac{\phi_t}{\phi} + 2\varepsilon^2\frac{\phi_x^2}{\phi^2} = -2\varepsilon^2\frac{\phi_{xx}}{\phi} + 2\varepsilon^2\frac{\phi_x^2}{\phi^2}.$$

The $\phi_x^2/\phi^2$ terms cancel:

$$-2\varepsilon\frac{\phi_t}{\phi} = -2\varepsilon^2\frac{\phi_{xx}}{\phi} \implies \phi_t = \varepsilon\phi_{xx}. \tag{Heat equation}$$

**Conclusion.** If $\phi > 0$ solves the heat equation $\phi_t = \varepsilon\phi_{xx}$, then:

$$u = \psi_x = -2\varepsilon\frac{\phi_x}{\phi} = -2\varepsilon\frac{\partial}{\partial x}\log\phi \tag{Hopf-Cole}$$

solves Burgers' equation $u_t + uu_x = \varepsilon u_{xx}$.

**Initial data correspondence.** Given $u(x,0) = u_0(x)$: let $\psi_0(x) = \int_0^x u_0(y)\,dy$, then $\phi_0(x) = e^{-\psi_0(x)/(2\varepsilon)}$. Solve the heat equation $\phi_t = \varepsilon\phi_{xx}$ with $\phi(x,0) = \phi_0(x)$:

$$\phi(x,t) = \frac{1}{\sqrt{4\pi\varepsilon t}}\int_{-\infty}^\infty \phi_0(\xi)e^{-(x-\xi)^2/(4\varepsilon t)}\,d\xi = \frac{1}{\sqrt{4\pi\varepsilon t}}\int_{-\infty}^\infty e^{-G(x,\xi,t)/(2\varepsilon)}\,d\xi,$$

where $G(x,\xi,t) = \psi_0(\xi) + (x-\xi)^2/(2t)$ is the **Hopf-Cole exponent**.

The Burgers solution is:

$$u(x,t) = \frac{\int_{-\infty}^\infty \frac{x-\xi}{t}\,e^{-G(x,\xi,t)/(2\varepsilon)}\,d\xi}{\int_{-\infty}^\infty e^{-G(x,\xi,t)/(2\varepsilon)}\,d\xi}. \tag{Exact solution}$$

## Worked Example: Riemann Initial Data

**Setup.** Consider the Riemann initial data:

$$u_0(x) = \begin{cases}u_L & x < 0 \\ u_R & x > 0\end{cases}, \qquad u_L > u_R \quad \text{(shock case)}.$$

**Potential:** $\psi_0(x) = \int_0^x u_0(\xi)\,d\xi = \begin{cases}u_Lx & x < 0 \\ u_Rx & x > 0\end{cases}$.

**Hopf-Cole exponent:** $G(x,\xi,t) = \psi_0(\xi) + (x-\xi)^2/(2t)$.

For large $t$ or small $\varepsilon$, the integrals in the exact formula are dominated by the saddle point — the $\xi$ minimizing $G(x,\xi,t)$. Setting $\partial G/\partial\xi = 0$: $u_0(\xi) = (x-\xi)/t$, i.e., $\xi$ is the foot of the characteristic passing through $(x,t)$.

**Saddle-point analysis (inviscid limit $\varepsilon\to 0^+$).** The integral $\int e^{-G/(2\varepsilon)}\,d\xi$ is dominated by the minimum of $G$ as $\varepsilon\to 0^+$ (Laplace's method). The minimum of $G(x,\cdot,t)$ over $\xi\in\mathbb{R}$:

- For $x > u_L t/2 + u_R t/2 = \bar{s}t$ (with $\bar{s} = (u_L+u_R)/2$): the minimizer is $\xi^* > 0$, $\xi^* = x - u_R t$. So $u \to u_R$.
- For $x < \bar{s}t$: the minimizer is $\xi^* < 0$, $\xi^* = x - u_L t$. So $u \to u_L$.
- At $x = \bar{s}t$: the two saddles have equal value, and the integral is split between them. The velocity $u \to (u_L+u_R)/2$.

This gives the inviscid shock solution: $u(x,t) \to \begin{cases}u_L & x < \bar{s}t \\ u_R & x > \bar{s}t\end{cases}$ where $\bar{s} = (u_L+u_R)/2$ — exactly the Rankine-Hugoniot shock speed for Burgers' equation (since the flux is $F(u) = u^2/2$ and $\bar{s} = (F(u_L)-F(u_R))/(u_L-u_R) = (u_L+u_R)/2$).

## Shock Layer Structure

For the Riemann problem with $u_L > u_R$, the exact viscous solution (for finite $\varepsilon > 0$) has the form of a **traveling wave**:

$$u(x,t) = U\!\left(\frac{x-\bar{s}t}{\varepsilon}\right), \qquad U(\eta) = \bar{u} - \frac{\Delta u}{2}\tanh\!\left(\frac{\Delta u}{4}\eta\right),$$

where $\bar{u} = (u_L+u_R)/2$ and $\Delta u = u_L - u_R > 0$. This is the **viscous shock profile**.

**Verification:** $U(\eta) = \bar{u} - \frac{\Delta u}{2}\tanh\!\left(\frac{\Delta u\eta}{4}\right)$.

As $\eta\to\pm\infty$: $U \to \bar{u} \mp \Delta u/2 = u_{L,R}$. $\checkmark$

The profile width (10%-90% transition) is $\sim\varepsilon/\Delta u$ — proportional to $\varepsilon$ and inversely proportional to the shock strength. In the inviscid limit $\varepsilon\to 0$: the $\tanh$ profile collapses to a step function (the shock).

**Derivation.** Seek $u = U((x-\bar{s}t)/\varepsilon)$ in Burgers' equation with $c = \bar{s}$:

$$-\bar{s}U'/\varepsilon + UU'/\varepsilon = U''/\varepsilon,$$

i.e., $(U-\bar{s})U' = U''$. Integrate: $\frac{1}{2}(U-\bar{s})^2 = U' + C$. With $U' = 0$ at $\eta = \pm\infty$ (where $U = u_{L,R}$): $C = -\frac{1}{2}(\Delta u/2)^2$. The ODE $U' = \frac{1}{2}[(U-\bar{s})^2 - (\Delta u/2)^2] = \frac{1}{2}(U-u_L)(U-u_R)$ has the separable solution $U = \bar{u} - \frac{\Delta u}{2}\tanh(\Delta u\eta/4)$.

## Rarefaction Case: $u_L < u_R$

For the Riemann problem with $u_L < u_R$ (rarefaction), the Hopf-Cole formula still applies. Now $G(x,\cdot,t)$ has a unique minimizer for all $x$ and $t > 0$, and the solution is smooth for $t > 0$. As $\varepsilon\to 0^+$, $u^\varepsilon$ converges to the rarefaction wave:

$$u(x,t) = \begin{cases}u_L & x \leq u_L t \\ x/t & u_L t \leq x \leq u_R t \\ u_R & x \geq u_R t\end{cases}.$$

This is consistent with the entropy condition: the rarefaction is the entropy solution for the Riemann problem with $u_L < u_R$.

## Global Existence and Regularity

**Theorem.** For any $u_0 \in L^\infty(\mathbb{R})$ (or more generally $u_0 \in L^1(\mathbb{R})\cap L^\infty(\mathbb{R})$), the Hopf-Cole formula gives a unique smooth solution $u\in C^\infty(\mathbb{R}\times(0,\infty))$ of Burgers' equation. Moreover:
- $\|u(\cdot,t)\|_{L^\infty} \leq \|u_0\|_{L^\infty}$ (maximum principle).
- For $p\geq 1$: $\|u(\cdot,t)\|_{L^p} \leq \|u_0\|_{L^p}$ ($L^p$ contraction).
- $\int_{\mathbb{R}} u(x,t)\,dx = \int_{\mathbb{R}} u_0(x)\,dx$ (conservation of mass, if $u_0\in L^1$).

**Proof.** The positivity $\phi > 0$ of the heat equation solution (since $\phi_0 = e^{-\psi_0/(2\varepsilon)} > 0$) ensures the Hopf-Cole formula is well-defined. Smoothness follows from the smoothness of the heat kernel for $t > 0$. The $L^\infty$ bound follows from the maximum principle for the heat equation applied to $\phi$.

## Summary of the Hopf-Cole Method

The Hopf-Cole transformation provides the complete solution theory for Burgers' equation:

1. **Transform:** $u_0 \mapsto \phi_0 = e^{-\psi_0/(2\varepsilon)}$ (where $\psi_0' = u_0$).
2. **Solve:** Heat equation $\phi_t = \varepsilon\phi_{xx}$, $\phi(x,0) = \phi_0(x)$.
3. **Invert:** $u = -2\varepsilon(\log\phi)_x$.

This is the analog, for Burgers' equation, of the Fourier transform method for the heat equation: both provide explicit integral formulas for the solution and enable rigorous analysis of long-time behavior, regularity, and the inviscid limit.
