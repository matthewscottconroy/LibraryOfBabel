# Functionals and the First Variation

A **functional** is a map that assigns a real number to each function in some function space. In the calculus of variations, functionals are the central objects: we seek functions that minimize or maximize a given functional, subject to constraints. This is the infinite-dimensional analog of finding the minimum of a function $f:\mathbb{R}^n\to\mathbb{R}$ — but instead of differentiating with respect to a finite number of variables, we differentiate with respect to the entire function $u$, obtaining the Euler-Lagrange equation.

## The Prototype Functional

The archetypal variational problem in one dimension: given $a < b$ and boundary values $\alpha, \beta\in\mathbb{R}$, minimize:

$$\mathcal{E}[u] = \int_a^b L(x,u(x),u'(x))\,dx$$

over all $u\in\mathcal{A} = \{u\in C^1([a,b]): u(a) = \alpha, u(b) = \beta\}$. The function $L(x,z,p)$ is the **Lagrangian**, assumed smooth in all three arguments.

**Immediate examples:**

1. **Arc length.** $L = \sqrt{1+(u')^2}$, so $\mathcal{E}[u] = \int_a^b\sqrt{1+(u')^2}\,dx$ is the arc length of the graph of $u$ from $(a,\alpha)$ to $(b,\beta)$.

2. **Dirichlet energy (1D).** $L = (u')^2/2$, $\mathcal{E}[u] = \frac{1}{2}\int_a^b(u')^2\,dx$.

3. **Energy of a loaded beam.** For a beam on $[0,L]$ under transverse load $f(x)$: $L = \frac{1}{2}(u'')^2 - f(x)u$ (requiring $u\in H^2$). This is a fourth-order problem.

4. **Brachistochrone.** $L = \sqrt{(1+(u')^2)/(2g(y_0-u))}$ (time for a particle to slide down a frictionless curve from $(0,y_0)$ to $(b,\beta)$ under gravity). The minimizer is a cycloid.

## The Notion of Variation

To "differentiate" $\mathcal{E}$ with respect to $u$, perturb $u$ by a smooth function $v$ vanishing at the endpoints:

$$\mathcal{E}[u+\varepsilon v] = \int_a^b L(x,u+\varepsilon v, u'+\varepsilon v')\,dx.$$

Define the **first variation** (Gateaux derivative) of $\mathcal{E}$ at $u$ in the direction $v$:

$$\delta\mathcal{E}[u;v] = \left.\frac{d}{d\varepsilon}\right|_{\varepsilon=0}\mathcal{E}[u+\varepsilon v] = \int_a^b\left[L_u(x,u,u')v + L_{u'}(x,u,u')v'\right]dx, \tag{First Variation}$$

where $L_u = \partial L/\partial z$ and $L_{u'} = \partial L/\partial p$.

**Stationarity condition.** If $u$ minimizes $\mathcal{E}$ over $\mathcal{A}$, then for any admissible perturbation $v\in C_c^\infty((a,b))$ (vanishing at endpoints), $\varepsilon \mapsto \mathcal{E}[u+\varepsilon v]$ has a minimum at $\varepsilon = 0$, so $\delta\mathcal{E}[u;v] = 0$. This is the necessary condition for a minimizer.

**Note:** The first variation is linear in $v$ — it is a continuous linear functional on $C_c^\infty$. The stationarity condition $\delta\mathcal{E}[u;v] = 0$ for all $v$ is the abstract equation "$\delta\mathcal{E}/\delta u = 0$," the functional derivative of $\mathcal{E}$ at $u$ is zero.

## The Fundamental Lemma of the Calculus of Variations

The derivation of the Euler-Lagrange equation from the stationarity condition uses the following:

**Fundamental Lemma.** If $f\in C([a,b])$ and $\int_a^b f(x)\eta(x)\,dx = 0$ for all $\eta\in C_c^\infty((a,b))$ (smooth functions compactly supported in $(a,b)$), then $f\equiv 0$ on $(a,b)$.

**Proof.** Suppose $f(x_0) > 0$ for some $x_0\in(a,b)$. By continuity, $f > 0$ on some interval $[x_0-\delta, x_0+\delta]\subset(a,b)$. Choose $\eta \geq 0$ supported in $[x_0-\delta, x_0+\delta]$ with $\eta(x_0) > 0$. Then $\int_a^b f\eta\,dx \geq \int_{x_0-\delta}^{x_0+\delta}f\eta\,dx > 0$ — contradiction. $\square$

This lemma is the bridge from "$\delta\mathcal{E}[u;v] = 0$ for all $v$" to the pointwise Euler-Lagrange equation.

## Worked Examples

**Example 1: Dirichlet energy.** $L = (u')^2/2$. First variation:

$$\delta\mathcal{E}[u;v] = \int_a^b u'v'\,dx.$$

Integrating by parts (with $v(a) = v(b) = 0$): $\delta\mathcal{E} = -\int_a^b u''v\,dx$. Setting this to zero for all $v$: $u'' = 0$. The minimizer of the 1D Dirichlet energy with fixed endpoints is the linear function — makes sense: the straight line is the "most uniform" function.

**Example 2: Arc length.** $L = \sqrt{1+(u')^2}$. First variation:

$$\delta\mathcal{E}[u;v] = \int_a^b\frac{u'}{\sqrt{1+(u')^2}}v'\,dx = -\int_a^b\frac{d}{dx}\left[\frac{u'}{\sqrt{1+(u')^2}}\right]v\,dx.$$

Setting to zero: $\frac{d}{dx}\!\left[\frac{u'}{\sqrt{1+(u')^2}}\right] = 0$, so $u'/\sqrt{1+(u')^2} = \text{const}$, giving $u' = \text{const}$: a straight line. (The shortest path between two points is a straight line — confirming elementary geometry.)

**Example 3: Brachistochrone.** The Lagrangian $L = \sqrt{(1+(u')^2)/(c-u)}$ (with $c$ a constant depending on the initial height). Since $L$ does not depend explicitly on $x$, the Beltrami identity applies: $L - u'L_{u'} = \text{const}$. This gives $1/[\sqrt{(c-u)(1+(u')^2)}] = \text{const}$, and setting $(c-u)(1+(u')^2) = R$ (a constant), the solution is parametrized as $x = R(\theta-\sin\theta)/2$, $u = c - R(1-\cos\theta)/2$ — a cycloid.

## Second Variation and Minimality

The **second variation** is:

$$\delta^2\mathcal{E}[u;v] = \left.\frac{d^2}{d\varepsilon^2}\right|_{\varepsilon=0}\mathcal{E}[u+\varepsilon v] = \int_a^b\left[L_{uu}v^2 + 2L_{uu'}vv' + L_{u'u'}(v')^2\right]dx.$$

A critical point $u$ (satisfying E-L) is a **local minimizer** if $\delta^2\mathcal{E}[u;v] \geq 0$ for all admissible $v$ (**Legendre condition:** $L_{u'u'} \geq 0$, and the stronger **Jacobi condition** for strict positivity). The Legendre condition $L_{u'u'} > 0$ is the analog of positive second derivative for functions.

**Sufficiency.** If $L_{zz}(x,z,p) \geq 0$ and $L_{pp}(x,z,p) > 0$ (convexity in $z$ and strict convexity in $p$), then the Euler-Lagrange equation has a unique solution, and it is the unique global minimizer of $\mathcal{E}$.

## Multi-Dimensional Setting

For $u:\Omega\subset\mathbb{R}^n\to\mathbb{R}$ and $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$:

$$\delta\mathcal{E}[u;v] = \int_\Omega\left[L_u v + \sum_{i=1}^n L_{p_i}v_{x_i}\right]dx = \int_\Omega\left[L_u - \text{div}(L_p)\right]v\,dx + \int_{\partial\Omega}(L_p\cdot\nu)v\,dS.$$

(Integration by parts: $\int_\Omega L_p\cdot\nabla v = -\int_\Omega\text{div}(L_p)v + \int_{\partial\Omega}L_p\cdot\nu v$.)

For $u = g$ on $\partial\Omega$ (Dirichlet condition), $v = 0$ on $\partial\Omega$, and the boundary term vanishes. The interior stationarity condition $\delta\mathcal{E} = 0$ for all $v$ compactly supported in $\Omega$ gives the Euler-Lagrange PDE:

$$L_u(x,u,\nabla u) - \text{div}(L_p(x,u,\nabla u)) = 0 \quad \text{in }\Omega. \tag{E-L PDE}$$

**For $L = |\nabla u|^2/2$:** $L_u = 0$, $L_p = \nabla u$, so $\text{div}(\nabla u) = \Delta u = 0$. The Euler-Lagrange equation for the Dirichlet energy is the Laplace equation.

**For $L = |\nabla u|^2/2 - fu$:** $L_u = -f$, $L_p = \nabla u$, giving $-f - \Delta u = 0$, i.e., $-\Delta u = f$. Poisson's equation.

**For $L = |\nabla u|^p/p$ ($p$-Dirichlet energy):** $L_u = 0$, $L_p = |\nabla u|^{p-2}\nabla u$, giving $\text{div}(|\nabla u|^{p-2}\nabla u) = 0$ — the $p$-Laplacian.

## Functionals on Function Spaces

For the calculus of variations to make mathematical sense, the functional $\mathcal{E}$ must be defined on an appropriate Banach or Hilbert space:

- For $L = |\nabla u|^2/2$: the natural space is $H^1(\Omega)$ (functions with square-integrable first derivatives).
- For $L = |u''|^2/2$ (beam energy): the natural space is $H^2(\Omega)$ (functions with square-integrable second derivatives).
- For $L = |\nabla u|^p/p$: the natural space is $W^{1,p}(\Omega)$ (Sobolev space).

This motivates the introduction of Sobolev spaces (Chapter 2, Section 2): they are precisely the function spaces needed to make the variational problem well-posed. The first variation $\delta\mathcal{E}[u;v]$ is a bounded linear functional on the Sobolev space, and the Euler-Lagrange equation is its Riesz representation.
