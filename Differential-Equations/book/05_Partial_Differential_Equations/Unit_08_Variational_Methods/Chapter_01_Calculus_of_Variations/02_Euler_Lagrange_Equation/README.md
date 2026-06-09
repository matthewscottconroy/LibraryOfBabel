# The Euler-Lagrange Equation

The Euler-Lagrange equation is the first-order necessary condition for a functional to be stationary. It transforms the variational problem (find $u$ minimizing $\mathcal{E}[u]$) into a PDE or ODE that any minimizer must satisfy. In this sense it is the "derivative equal to zero" condition in the infinite-dimensional setting. This section derives the equation in full generality — for integrands depending on $u$ and all its first derivatives — in both one and $n$ dimensions, provides the complete derivation including the integration by parts argument, and works through several important examples.

## Derivation in One Dimension

**Setting.** Minimize $\mathcal{E}[u] = \int_a^b L(x,u,u')\,dx$ over $u\in\mathcal{A} = \{u\in C^2([a,b]): u(a) = \alpha, u(b) = \beta\}$.

**Necessary condition.** If $u^*\in\mathcal{A}$ is a minimizer, then for any $v\in C_c^\infty((a,b))$ (smooth, compactly supported, so automatically zero at $a$ and $b$), the function $\varepsilon\mapsto\mathcal{E}[u^*+\varepsilon v]$ has a minimum at $\varepsilon = 0$, so its derivative at $\varepsilon = 0$ is zero:

$$0 = \left.\frac{d}{d\varepsilon}\right|_{\varepsilon=0}\mathcal{E}[u^*+\varepsilon v] = \int_a^b\left[L_u(x,u^*,u^{*\prime})v + L_{u'}(x,u^*,u^{*\prime})v'\right]dx.$$

**Integration by parts.** Integrate the second term by parts:

$$\int_a^b L_{u'}v'\,dx = \left[L_{u'}v\right]_a^b - \int_a^b\frac{d}{dx}[L_{u'}]v\,dx = -\int_a^b\frac{d}{dx}[L_{u'}]v\,dx,$$

since $v(a) = v(b) = 0$. Therefore:

$$0 = \int_a^b\left[L_u - \frac{d}{dx}L_{u'}\right]v\,dx \quad \text{for all }v\in C_c^\infty((a,b)).$$

**Fundamental lemma.** Since the integral of a continuous function times all smooth test functions is zero, the function itself is zero:

$$L_u(x,u^*,u^{*\prime}) - \frac{d}{dx}L_{u'}(x,u^*,u^{*\prime}) = 0 \quad \text{for all }x\in(a,b). \tag{E-L}$$

This is the **Euler-Lagrange equation**.

**Expanded form.** Using the chain rule to expand $\frac{d}{dx}L_{u'}$:

$$L_{u'u''}\equiv \frac{d}{dx}L_{u'} = L_{u'x} + L_{u'u}u' + L_{u'u'}u''.$$

Substituting into (E-L):

$$L_u - L_{u'x} - L_{u'u}u' - L_{u'u'}u'' = 0.$$

This is a second-order ODE for $u(x)$ (provided $L_{u'u'} \neq 0$, the Legendre condition). When $L_{u'u'} > 0$, the ODE is elliptic (in the 1D sense: can be solved for $u''$ uniquely).

## Examples

**Example 1: Laplace equation (1D).** $L = (u')^2/2$. Then $L_u = 0$, $L_{u'} = u'$, and E-L gives $-u'' = 0$, i.e., $u'' = 0$. The minimizer of $\int(u')^2$ with fixed endpoints is the linear interpolant $u = \alpha + (\beta-\alpha)(x-a)/(b-a)$.

**Example 2: Weighted Dirichlet energy.** $L = \frac{1}{2}p(x)(u')^2 - f(x)u$ (Sturm-Liouville problem). Then $L_u = -f$, $L_{u'} = p(x)u'$. E-L: $-f - (pu')' = 0$, i.e., $(pu')' = -f$ — the Sturm-Liouville ODE.

**Example 3: Geodesics on a surface.** On a surface with metric $g_{ij}(x)$, the arc length functional is $\mathcal{E}[\gamma] = \int_0^1\sqrt{g_{ij}\dot\gamma^i\dot\gamma^j}\,dt$. The Euler-Lagrange equations are the **geodesic equations**: $\ddot\gamma^k + \Gamma^k_{ij}\dot\gamma^i\dot\gamma^j = 0$, where $\Gamma^k_{ij}$ are the Christoffel symbols.

**Example 4: Bending energy of an Euler-Bernoulli beam.** $L = \frac{EI}{2}(u'')^2 - f(x)u$ (higher-order; requires $u\in H^2$). This is a 4th-order problem: $L_u = -f$, and the $\frac{d^2}{dx^2}L_{u''}$ term gives $EIu'''' = f$. This illustrates that higher-order Lagrangians give higher-order PDEs.

**Example 5: Minimal surface.** In 2D: $L(u,p) = \sqrt{1+|p|^2}$ (area density). E-L gives the minimal surface equation:

$$\text{div}\!\left(\frac{\nabla u}{\sqrt{1+|\nabla u|^2}}\right) = 0, \quad \text{i.e.,} \quad (1+u_y^2)u_{xx} - 2u_xu_yu_{xy} + (1+u_x^2)u_{yy} = 0.$$

## Derivation in $n$ Dimensions

For $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$ with $u:\Omega\subset\mathbb{R}^n\to\mathbb{R}$ and $u = g$ on $\partial\Omega$:

$$\delta\mathcal{E}[u;v] = \int_\Omega\left[L_u v + \sum_{i=1}^n L_{p_i}\frac{\partial v}{\partial x_i}\right]dx.$$

Integration by parts (Green's first identity) for the $i$-th term:

$$\int_\Omega L_{p_i}v_{x_i}\,dx = -\int_\Omega\frac{\partial L_{p_i}}{\partial x_i}v\,dx + \int_{\partial\Omega}L_{p_i}v\nu_i\,dS.$$

For $v|_{\partial\Omega} = 0$ (Dirichlet BC), the boundary term vanishes. Summing over $i$:

$$\delta\mathcal{E}[u;v] = \int_\Omega\left[L_u - \sum_{i=1}^n\frac{\partial L_{p_i}}{\partial x_i}\right]v\,dx = \int_\Omega\left[L_u - \text{div}(\nabla_pL)\right]v\,dx.$$

Setting this to zero for all $v$ compactly supported in $\Omega$ gives the **multi-dimensional Euler-Lagrange equation**:

$$L_u(x,u,\nabla u) - \text{div}(\nabla_p L(x,u,\nabla u)) = 0 \quad \text{in }\Omega. \tag{E-L multi}$$

Here $\nabla_p L = (\partial L/\partial p_1, \ldots, \partial L/\partial p_n)$ is the gradient of $L$ with respect to the $p = \nabla u$ argument.

## The Beltrami Identity (Conservation Law for Autonomous Problems)

When $L$ does not depend explicitly on $x$ (autonomous Lagrangian: $L = L(u,u')$), the Euler-Lagrange equation has a first integral:

$$L - u'L_{u'} = C \quad \text{(constant).} \tag{Beltrami}$$

**Proof.** Compute $\frac{d}{dx}[L - u'L_{u'}] = L_uu' + L_{u'}u'' - u''L_{u'} - u'\frac{d}{dx}L_{u'} = u'[L_u - \frac{d}{dx}L_{u'}] = 0$ (by E-L). $\square$

The Beltrami identity reduces the second-order ODE to a first-order one, greatly simplifying the computation. It is the 1D analog of Noether's theorem: the symmetry of $L$ under translations in $x$ (since $L$ doesn't depend on $x$) gives a conservation law.

**Application.** For the catenary (a hanging chain): $L = u\sqrt{1+(u')^2}$ (energy = $\int\rho\,u\,ds$ where $u$ is height). Beltrami: $u\sqrt{1+(u')^2} - u(u')^2/\sqrt{1+(u')^2} = u/\sqrt{1+(u')^2} = C$. This gives $\sqrt{1+(u')^2} = u/C$, then $u' = \sqrt{(u/C)^2-1}$, separating and integrating: $u = C\cosh((x-x_0)/C)$ — the catenary profile.

## Legendre-Hadamard Condition

For multi-dimensional problems with vector-valued $u:\Omega\to\mathbb{R}^m$ and $L(x,u,Du)$, the Legendre-Hadamard condition for the second variation to be non-negative is:

$$\sum_{i,j,\alpha,\beta}L_{p_i^\alpha p_j^\beta}(x,u,Du)\xi_i\xi_j\eta^\alpha\eta^\beta \geq 0 \quad \text{for all }\xi\in\mathbb{R}^n, \eta\in\mathbb{R}^m.$$

This is weaker than convexity of $L$ in $Du$ (it requires convexity only along rank-one matrices $\xi\otimes\eta$) and is the correct notion of ellipticity for systems. The failure of the Legendre-Hadamard condition leads to the Lavrentiev phenomenon (infimum of $\mathcal{E}$ over $W^{1,\infty}$ differs from infimum over $W^{1,1}$) and to microstructure formation in nonlinear elasticity.

## Regularity of Minimizers

A fundamental result (Hilbert, early 20th century; De Giorgi-Nash 1957 for elliptic systems; Evans 1986 for fully nonlinear):

**Theorem (Regularity of E-L solutions).** If $L(x,z,p)$ is smooth and uniformly convex in $p$ (i.e., $L_{pp}\geq \lambda I$ for some $\lambda > 0$), then any $H^1$ weak solution of the Euler-Lagrange equation is smooth: $u\in C^\infty(\Omega)$ (or $C^{k,\alpha}$ if $L$ is $C^k$).

The key step (De Giorgi-Nash theorem): a bounded measurable solution of a uniformly elliptic equation with measurable coefficients is Hölder continuous. This was a major 20th-century result, required to close the regularity gap in Hilbert's 19th problem (are minimizers of smooth functionals smooth?).
