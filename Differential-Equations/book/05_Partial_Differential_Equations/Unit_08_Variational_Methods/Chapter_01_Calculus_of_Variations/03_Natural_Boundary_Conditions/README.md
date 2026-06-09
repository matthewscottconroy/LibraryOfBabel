# Natural Boundary Conditions

In the derivation of the Euler-Lagrange equation, we assumed $v = 0$ on $\partial\Omega$ — a consequence of the fixed Dirichlet condition $u = g$ on $\partial\Omega$. But what if the boundary values of $u$ are not prescribed? In this case, the admissible variations $v$ need not vanish on $\partial\Omega$, and the boundary terms from integration by parts must also vanish. This produces **natural boundary conditions** — boundary conditions that are not imposed externally but instead emerge automatically from the stationarity of the functional. Natural BCs are the variational analog of Neumann or Robin boundary conditions.

## Derivation of Natural Boundary Conditions

For $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$ with no constraint on $u|_{\partial\Omega}$, the first variation is:

$$\delta\mathcal{E}[u;v] = \int_\Omega\left[L_u - \text{div}(\nabla_p L)\right]v\,dx + \int_{\partial\Omega}(\nabla_p L\cdot\nu)v\,dS = 0$$

for all smooth $v$ (including those nonzero on $\partial\Omega$). Setting the interior term to zero (by testing with $v$ compactly supported in $\Omega$) gives the Euler-Lagrange equation in $\Omega$. With the E-L equation satisfied, the remaining condition is:

$$\int_{\partial\Omega}(\nabla_p L\cdot\nu)v\,dS = 0 \quad \text{for all smooth }v,$$

which by the fundamental lemma on $\partial\Omega$ gives:

$$\nabla_p L(x,u,\nabla u)\cdot\nu = 0 \quad \text{on }\partial\Omega. \tag{Natural BC}$$

For the Dirichlet energy $L = |\nabla u|^2/2$: $\nabla_p L = \nabla u$, so the natural boundary condition is $\nabla u\cdot\nu = \frac{\partial u}{\partial\nu} = 0$ — the Neumann condition. The functional $\int|\nabla u|^2\,dx$ without prescribed boundary values is minimized by harmonic functions with zero normal derivative (constants, in the absence of a source term).

## Mixed Boundary Conditions

In many physical problems, the boundary $\partial\Omega$ is split into two parts: $\partial\Omega = \Gamma_D \cup \Gamma_N$ (Dirichlet and Neumann portions). The functional is:

$$\mathcal{E}[u] = \int_\Omega\left[\frac{1}{2}|\nabla u|^2 - fu\right]dx - \int_{\Gamma_N}hu\,dS,$$

where $h$ is a prescribed flux on $\Gamma_N$. Minimizing over $u$ with $u = g$ on $\Gamma_D$ (only), the admissible variations satisfy $v = 0$ on $\Gamma_D$ but are free on $\Gamma_N$:

$$\delta\mathcal{E}[u;v] = \int_\Omega(\nabla u\cdot\nabla v - fv)\,dx - \int_{\Gamma_N}hv\,dS.$$

Integrating by parts:

$$\delta\mathcal{E}[u;v] = \int_\Omega(-\Delta u - f)v\,dx + \int_{\partial\Omega}\frac{\partial u}{\partial\nu}v\,dS - \int_{\Gamma_N}hv\,dS.$$

Setting to zero: (i) $-\Delta u = f$ in $\Omega$; (ii) $\partial u/\partial\nu = h$ on $\Gamma_N$. The Neumann condition on $\Gamma_N$ is the natural boundary condition from the surface integral.

## Robin (Mixed) Boundary Condition

For a problem modeling a body losing heat to the environment, add a surface energy term:

$$\mathcal{E}[u] = \int_\Omega\frac{1}{2}|\nabla u|^2\,dx + \int_{\partial\Omega}\frac{\alpha}{2}(u-u_\infty)^2\,dS,$$

where $\alpha > 0$ is the heat transfer coefficient and $u_\infty$ is the ambient temperature. No Dirichlet condition; $u$ is free on $\partial\Omega$. First variation:

$$\delta\mathcal{E}[u;v] = \int_\Omega\nabla u\cdot\nabla v\,dx + \int_{\partial\Omega}\alpha(u-u_\infty)v\,dS.$$

Integrating by parts in the interior:

$$\delta\mathcal{E}[u;v] = \int_\Omega(-\Delta u)v\,dx + \int_{\partial\Omega}\left[\frac{\partial u}{\partial\nu} + \alpha(u-u_\infty)\right]v\,dS.$$

Setting to zero: $\Delta u = 0$ in $\Omega$ and $\frac{\partial u}{\partial\nu} + \alpha u = \alpha u_\infty$ on $\partial\Omega$ — the Robin condition. The physical content: Newton's law of cooling, $-k\frac{\partial u}{\partial\nu} = \alpha(u-u_\infty)$, emerges naturally from the surface energy term.

## Natural BCs for Higher-Order Problems

For the biharmonic functional $\mathcal{E}[u] = \frac{1}{2}\int_\Omega(\Delta u)^2\,dx$ (bending energy of a thin plate):

$$\delta\mathcal{E}[u;v] = \int_\Omega\Delta u\,\Delta v\,dx.$$

Integration by parts twice: $\int\Delta u\Delta v = -\int\nabla(\Delta u)\cdot\nabla v = \int v\Delta^2 u$ (applying Green's first identity twice, with boundary terms at each step). The two integrations by parts produce two boundary conditions from the two boundary terms. For a free boundary (no imposed conditions on $u$ or $\partial u/\partial\nu$):

**Natural BCs for the biharmonic:** $\Delta u = 0$ and $\frac{\partial(\Delta u)}{\partial\nu} = 0$ on $\partial\Omega$.

These are the natural boundary conditions for a freely vibrating or freely bent plate — no prescribed deflection or slope.

## Physical Interpretation

Natural boundary conditions have a consistent physical interpretation:

| Functional | Natural BC | Physical meaning |
|---|---|---|
| $\frac{1}{2}\int|\nabla u|^2$ | $\partial u/\partial\nu = 0$ | No heat flux through boundary |
| $\frac{1}{2}\int|\nabla u|^2 - \int_{\partial\Omega}hu$ | $\partial u/\partial\nu = h$ | Prescribed heat flux |
| $\frac{1}{2}\int|\nabla u|^2 + \frac{\alpha}{2}\int_{\partial\Omega}u^2$ | $\partial u/\partial\nu + \alpha u = 0$ | Convective heat loss |
| $\frac{1}{2}\int(u'')^2$ (beam) | $u'' = 0$ at endpoints | Free bending moment |
| $\frac{1}{2}\int(\Delta u)^2$ (plate) | $\Delta u = 0$, $\partial\Delta u/\partial\nu = 0$ | Free moment and shear |

The pattern: when a physical quantity (flux, moment, shear) is not externally prescribed, the natural boundary condition imposes that this quantity is zero at the boundary — the "free" condition.

## Worked Example: Elastically Supported Beam

A beam of length $L$ with Young's modulus $EI$ rests on an elastic foundation with stiffness $k$ and is subjected to distributed load $f(x)$. The energy functional is:

$$\mathcal{E}[u] = \int_0^L\left[\frac{EI}{2}(u'')^2 + \frac{k}{2}u^2 - f u\right]dx + \left[\frac{k_0}{2}u^2\right]_{x=0}^{x=L},$$

where $k_0$ is the stiffness of end supports. No Dirichlet conditions. The Euler-Lagrange equation in $(0,L)$:

$$EIu'''' + ku = f.$$

The natural boundary conditions at $x = 0$ and $x = L$ (from integrating by parts twice and collecting boundary terms from both the bulk term $(EI/2)(u'')^2$ and the end terms $(k_0/2)u^2$):

$$EIu'' = 0 \quad \text{(zero moment)}, \qquad -EIu''' + k_0 u = 0 \quad \text{(shear balance with end spring)}.$$

These are the natural BCs: the beam has no imposed bending moment at the ends, and the shear force at each end equals the spring reaction $k_0 u$.

## Relation to Weak Formulations

Natural boundary conditions are automatically incorporated into weak formulations. The weak form of $-\Delta u = f$ with Neumann condition $\partial u/\partial\nu = h$ on $\partial\Omega$ is:

$$\int_\Omega\nabla u\cdot\nabla v\,dx = \int_\Omega fv\,dx + \int_{\partial\Omega}hv\,dS \quad \text{for all }v\in H^1(\Omega).$$

This is obtained directly from $\delta\mathcal{E} = 0$ — the Neumann condition $h$ appears as a natural loading term, not as an essential (Dirichlet) condition that must be imposed on the function space. This is the key advantage of the variational / weak formulation: Neumann and Robin conditions come for free, while only Dirichlet conditions need to be built into the function space.
