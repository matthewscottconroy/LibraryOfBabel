# Weak Solutions

A **weak solution** of a PDE is a function that satisfies the equation not pointwise but in an integral (distributional) sense. The passage from the strong (pointwise) formulation to the weak (integral) formulation is achieved by multiplying by a test function and integrating by parts, transferring derivatives from the solution to the test function. This distributes the regularity requirements: instead of needing $u$ to have two classical derivatives, we only need $u$ and the test function to have one derivative each — a much weaker requirement that greatly expands the class of admissible solutions.

## Motivation: Why Weak Solutions?

**Example 1: Irregular source term.** Consider $-u'' = f$ on $(0,1)$ with $u(0) = u(1) = 0$ and $f\in L^2(0,1)$ (possibly discontinuous). A classical solution requires $u\in C^2(0,1)$, which by $u'' = -f$ requires $f$ to be continuous — too strong. But a weak solution $u\in H^1_0(0,1)$ satisfying $\int_0^1 u'v'\,dx = \int_0^1 fv\,dx$ for all $v\in H^1_0(0,1)$ exists and is unique for any $f\in L^2(0,1)$.

**Example 2: Discontinuous coefficients.** The equation $-(a(x)u')' = f$ with $a$ piecewise constant (a composite material) has no classical solution (because $a$ is discontinuous, the derivative $(au')'$ is not defined pointwise). The weak solution $u\in H^1_0(0,1)$ with $\int a(x)u'v'\,dx = \int fv\,dx$ is well-defined and captures the correct interface condition $[au']_{x=x_0} = 0$ (continuity of flux at the material interface) automatically.

**Example 3: Weak solutions of conservation laws.** For the inviscid Burgers equation $u_t + (u^2/2)_x = 0$, classical solutions break down when shocks form. Weak solutions (satisfying the integral identity $\int\int[u\phi_t + (u^2/2)\phi_x]\,dx\,dt = 0$ for all $\phi\in C_c^\infty$) exist globally and include shocks. The Rankine-Hugoniot condition is automatically captured by the weak formulation.

## Formal Definition

**Definition (Weak solution of Poisson's equation).** Let $\Omega\subset\mathbb{R}^n$ be a bounded open set with Lipschitz boundary. Given $f\in L^2(\Omega)$ and $g\in H^{1/2}(\partial\Omega)$, a function $u\in H^1(\Omega)$ is a **weak solution** of $-\Delta u = f$ in $\Omega$, $u = g$ on $\partial\Omega$ if:

1. $u = g$ on $\partial\Omega$ in the trace sense: $\text{tr}(u) = g$ in $H^{1/2}(\partial\Omega)$.
2. For all $v\in H^1_0(\Omega)$:

$$\int_\Omega\nabla u\cdot\nabla v\,dx = \int_\Omega fv\,dx. \tag{Weak form}$$

(Here the condition $u-\tilde{g} \in H^1_0(\Omega)$ for some extension $\tilde{g}\in H^1(\Omega)$ of the boundary data is the standard way to handle non-homogeneous Dirichlet conditions.)

**Equivalence with classical solutions.** If $u\in C^2(\Omega)\cap C(\bar\Omega)$ is a classical solution of $-\Delta u = f$, it is also a weak solution. Conversely, if a weak solution happens to be in $C^2(\Omega)$, it is a classical solution (shown by reversing the integration by parts).

## Weak Derivatives

The key technical concept enabling weak solutions is the **weak derivative**. A function $u\in L^1_{\text{loc}}(\Omega)$ has a weak partial derivative $\partial u/\partial x_i \in L^1_{\text{loc}}(\Omega)$ if there exists a function $w_i\in L^1_{\text{loc}}(\Omega)$ such that:

$$\int_\Omega u\frac{\partial\phi}{\partial x_i}\,dx = -\int_\Omega w_i\phi\,dx \quad \text{for all }\phi\in C_c^\infty(\Omega).$$

We then write $w_i = \partial u/\partial x_i$ (weakly). This is the distributional derivative: integration by parts holds by definition.

**Example.** The absolute value function $u(x) = |x|$ on $(-1,1)$ has weak derivative $u'(x) = \text{sgn}(x)$ (the step function $+1$ for $x>0$, $-1$ for $x<0$). Verification: $\int_{-1}^1|x|\phi'\,dx = -\int_{-1}^1\text{sgn}(x)\phi\,dx$ for all $\phi\in C_c^\infty(-1,1)$ (by integration by parts on $(-1,0)$ and $(0,1)$ separately, with no boundary terms since $\phi$ is compactly supported).

However, $\text{sgn}(x)$ is not differentiable at $x=0$; its weak derivative is $2\delta(x)$ (a distribution, not an $L^1$ function). So $u = |x|\in H^1(-1,1)$ but $u\notin H^2(-1,1)$.

## Sobolev Spaces (Preview)

The function space for weak solutions of $-\Delta u = f$ with Dirichlet BC is $H^1_0(\Omega)$, defined as:

$$H^1_0(\Omega) = \{u\in L^2(\Omega): \nabla u\in L^2(\Omega;\mathbb{R}^n), u|_{\partial\Omega} = 0\}.$$

The norm $\|u\|_{H^1}^2 = \|u\|_{L^2}^2 + \|\nabla u\|_{L^2}^2$ makes $H^1_0(\Omega)$ a Hilbert space (with inner product $\langle u,v\rangle_{H^1} = \int uv\,dx + \int\nabla u\cdot\nabla v\,dx$). The condition $u|_{\partial\Omega} = 0$ is interpreted via the trace operator $\text{tr}:H^1(\Omega)\to L^2(\partial\Omega)$: $H^1_0(\Omega) = \ker(\text{tr})$.

**Poincaré inequality.** For $u\in H^1_0(\Omega)$ with $\Omega$ bounded:

$$\|u\|_{L^2(\Omega)} \leq C_P\|\nabla u\|_{L^2(\Omega)},$$

where $C_P$ depends only on $\Omega$. This allows the equivalent norm $|\!|\!|u|\!|\!| = \|\nabla u\|_{L^2}$ on $H^1_0(\Omega)$ — the Dirichlet norm — which is what appears in the bilinear form $a(u,v) = \int\nabla u\cdot\nabla v$.

## Verification: Weak Solution Exists for $f\in L^2$

**Theorem.** For any $f\in L^2(\Omega)$ and $\Omega$ a bounded Lipschitz domain, the problem $-\Delta u = f$ with $u = 0$ on $\partial\Omega$ has a unique weak solution $u\in H^1_0(\Omega)$.

**Proof (via Lax-Milgram, sketched).** Take $H = H^1_0(\Omega)$ with norm $|\!|\!|\cdot|\!|\!| = \|\nabla\cdot\|_{L^2}$.

- Bilinear form: $a(u,v) = \int_\Omega\nabla u\cdot\nabla v\,dx$.
- Boundedness: $|a(u,v)| = |\int\nabla u\cdot\nabla v| \leq \|\nabla u\|_{L^2}\|\nabla v\|_{L^2} = |\!|\!|u|\!|\!|\,|\!|\!|v|\!|\!|$ (Cauchy-Schwarz). So $M = 1$.
- Coercivity: $a(u,u) = \|\nabla u\|_{L^2}^2 = |\!|\!|u|\!|\!|^2$. So $\alpha = 1$.
- Linear functional: $F(v) = \int_\Omega fv\,dx$. By Cauchy-Schwarz: $|F(v)| \leq \|f\|_{L^2}\|v\|_{L^2} \leq C_P\|f\|_{L^2}\|\nabla v\|_{L^2} = C_P\|f\|_{L^2}|\!|\!|v|\!|\!|$. So $F$ is bounded.

By Lax-Milgram: unique $u\in H^1_0(\Omega)$ with $a(u,v) = F(v)$ for all $v\in H^1_0(\Omega)$, and $|\!|\!|u|\!|\!| \leq C_P\|f\|_{L^2}$. $\square$

## Regularity: When Are Weak Solutions Classical?

The existence theorem gives $u\in H^1_0(\Omega)$ — one derivative in $L^2$. Is this actually smooth?

**$H^2$ regularity.** If $\partial\Omega$ is $C^2$ and $f\in L^2(\Omega)$, then the weak solution satisfies $u\in H^2(\Omega)$ and $\|u\|_{H^2} \leq C\|f\|_{L^2}$. This requires a more delicate argument (elliptic regularity theory) beyond the scope of this introduction.

**$H^k$ regularity.** More generally, $f\in H^k(\Omega)\Rightarrow u\in H^{k+2}(\Omega)$ (Schauder estimates). For $k > n/2$, the Sobolev embedding theorem gives $u\in C^{k+2-n/2}(\Omega)$ (Hölder continuous).

**Corner singularities.** For domains with corners (like a square), the $H^2$ regularity can fail: even for $f$ smooth, $u$ may have a corner singularity of the form $r^\pi/\omega\sin(\pi\theta/\omega)$ (where $\omega$ is the corner angle and $r$ is the distance to the corner). This singularity belongs to $H^s$ for $s < 1 + \pi/\omega$ but not for larger $s$, limiting the regularity.

## Distributional Formulation

The weakest notion of solution is the **distributional solution**: $u\in\mathcal{D}'(\Omega)$ (a distribution) satisfying $-\Delta u = f$ in the distributional sense: $\langle u, -\Delta\phi\rangle = \langle f,\phi\rangle$ for all $\phi\in C_c^\infty(\Omega)$. This includes:
- $L^1_{\text{loc}}$ functions (integrate $u\cdot(-\Delta\phi)$).
- The Dirac delta: $-\Delta G = \delta_y$ gives the Green's function $G$.
- Sums of derivatives of $L^1$ functions.

The hierarchy: Classical solutions $\subset$ Strong $L^p$ solutions $\subset$ Weak $H^1$ solutions $\subset$ Distributional solutions. Each step enlarges the class at the cost of less regularity.
