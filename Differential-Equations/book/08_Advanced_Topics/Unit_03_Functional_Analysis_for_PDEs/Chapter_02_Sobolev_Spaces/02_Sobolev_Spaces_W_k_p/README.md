# Sobolev Spaces $W^{k,p}$

Sobolev spaces are the precise function spaces needed for the variational theory of PDEs. They measure regularity through $L^p$ integrability of weak derivatives rather than pointwise differentiability, capturing exactly the regularity that the integration-by-parts formulation of PDEs requires. This section gives the precise definition, establishes completeness, and surveys the key properties.

## Definition

Let $\Omega \subset \mathbb{R}^n$ be an open set, $k \in \mathbb{N}_0$, and $1 \leq p \leq \infty$.

**Definition.** The **Sobolev space** $W^{k,p}(\Omega)$ is:

$$W^{k,p}(\Omega) = \{f \in L^p(\Omega) : D^\alpha f \in L^p(\Omega) \text{ for all } |\alpha| \leq k\},$$

where $D^\alpha f$ denotes the weak $\alpha$-partial derivative. The **Sobolev norm** is:

$$\|f\|_{W^{k,p}(\Omega)} = \left(\sum_{|\alpha| \leq k} \|D^\alpha f\|_{L^p(\Omega)}^p\right)^{1/p} \quad (1 \leq p < \infty)$$

$$\|f\|_{W^{k,\infty}(\Omega)} = \max_{|\alpha| \leq k} \|D^\alpha f\|_{L^\infty(\Omega)}.$$

**Notation.** For $p = 2$, the standard notation is $H^k(\Omega) = W^{k,2}(\Omega)$, with the Hilbert space inner product:

$$\langle f, g \rangle_{H^k} = \sum_{|\alpha| \leq k} \int_\Omega D^\alpha f \cdot D^\alpha g \, dx.$$

## Completeness

**Theorem.** $W^{k,p}(\Omega)$ is a Banach space for all $k \geq 0$ and $1 \leq p \leq \infty$. For $p = 2$, $H^k(\Omega)$ is a Hilbert space.

**Proof of completeness.** Let $(f_n)$ be Cauchy in $W^{k,p}$. Then $(D^\alpha f_n)$ is Cauchy in $L^p$ for each $|\alpha| \leq k$. By completeness of $L^p$, there exist $f, g_\alpha \in L^p$ with $f_n \to f$ and $D^\alpha f_n \to g_\alpha$ in $L^p$. It remains to show $g_\alpha = D^\alpha f$ (weakly). For any $\phi \in C_c^\infty(\Omega)$:

$$\int g_\alpha \phi \, dx = \lim_{n\to\infty}\int (D^\alpha f_n)\phi \, dx = (-1)^{|\alpha|}\lim_{n\to\infty}\int f_n D^\alpha\phi \, dx = (-1)^{|\alpha|}\int f D^\alpha\phi \, dx.$$

So $g_\alpha = D^\alpha f$ weakly, and $f \in W^{k,p}$ with $f_n \to f$. $\square$

## The Space $W^{k,p}_0$ and Zero Boundary Conditions

The space $W^{k,p}_0(\Omega)$ is defined as the closure of $C_c^\infty(\Omega)$ in $W^{k,p}(\Omega)$:

$$W^{k,p}_0(\Omega) = \overline{C_c^\infty(\Omega)}^{W^{k,p}}.$$

Elements of $W^{k,p}_0$ are characterized (when $\partial\Omega$ is smooth) as elements of $W^{k,p}$ whose traces (see Section 4) on $\partial\Omega$ of order $\leq k-1$ are zero. Thus $W^{1,p}_0(\Omega)$ consists of $W^{1,p}$ functions with zero boundary values in the generalized sense—the right space for the homogeneous Dirichlet problem.

## Equivalence of Norms and Density

**Meyers-Serrin theorem (H = W).** The smooth functions $C^\infty(\Omega) \cap W^{k,p}(\Omega)$ are dense in $W^{k,p}(\Omega)$ for $1 \leq p < \infty$. That is, every function in $W^{k,p}$ can be approximated in the $W^{k,p}$ norm by smooth functions.

This theorem is proved by mollification: $f_\varepsilon = f * \rho_\varepsilon$ is smooth and converges to $f$ in $W^{k,p}$ (on any compactly contained subdomain). A global approximation requires a partition of unity argument.

The density result justifies manipulations: one can prove identities for smooth functions and then pass to limits in $W^{k,p}$.

## Examples of Sobolev Functions

**$|x|^s$ near 0 in $\mathbb{R}^n$:** For $f(x) = |x|^s$ on the unit ball $B \subset \mathbb{R}^n$, $f \in W^{1,p}(B)$ if $s > 1 - n/p$ (so that both $f$ and $|\nabla f| = |s| |x|^{s-1}$ are in $L^p(B)$). In particular, $|x|^{1-n/p + \varepsilon} \in W^{1,p}(B)$ for any $\varepsilon > 0$. This shows that $H^1$ functions can have polynomial singularities near the boundary.

**Corner domains:** On a square domain $\Omega = (0,1)^2$, the solution $u$ of $-\Delta u = 1$ with $u = 0$ on $\partial\Omega$ is in $H^2(\Omega) = W^{2,2}(\Omega)$ but not in $H^3$ near corners—a manifestation of corner singularities in elliptic regularity theory.

## The Dual Space $H^{-1}$

The dual of $H^1_0(\Omega)$ is denoted $H^{-1}(\Omega)$. By the Riesz representation theorem, every $F \in H^{-1}(\Omega)$ has the form:

$$F(v) = \int_\Omega f_0 v + \sum_{i=1}^n \int_\Omega f_i \partial_{x_i} v, \quad f_0, f_1, \ldots, f_n \in L^2(\Omega).$$

Distributions are in $H^{-1}(\Omega)$ if and only if they can be written as finite sums of first derivatives of $L^2$ functions. In particular, $\delta_x \in H^{-1}(\Omega)$ if $n = 1$ (since $H^1$ embeds into $C^0$), but $\delta_x \notin H^{-1}(\Omega)$ for $n \geq 2$.

The Poisson equation $-\Delta u = f$ with $f \in H^{-1}(\Omega)$ has a unique solution $u \in H^1_0(\Omega)$ (by Lax-Milgram), even when $f$ is only a distribution. This extended right-hand side is one of the main advantages of the Sobolev framework.

## The Poincaré Inequality

On a bounded domain $\Omega$ with Dirichlet boundary conditions:

$$\|u\|_{L^2(\Omega)} \leq C_\Omega\|\nabla u\|_{L^2(\Omega)}, \quad u \in H^1_0(\Omega).$$

This is the Poincaré inequality, and it implies that on $H^1_0$, the seminorm $\|\nabla u\|_{L^2}$ is equivalent to the full $H^1$ norm $\|u\|_{H^1}$. The Poincaré inequality is the coercivity condition that makes the Dirichlet Laplacian invertible: $\int_\Omega |\nabla u|^2 \geq c\|u\|_{H^1}^2$.
