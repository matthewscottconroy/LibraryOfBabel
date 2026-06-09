# Trace Theorems

Functions in Sobolev spaces $W^{1,p}(\Omega)$ are defined almost everywhere in $\Omega$, but the boundary $\partial\Omega$ has measure zero in $\mathbb{R}^n$ and carries no information from the Lebesgue measure. This means the boundary values of a Sobolev function are not defined by the function itself—unlike continuous functions, which have well-defined values everywhere including on the boundary. The trace theorem resolves this by constructing a canonical "boundary restriction" operator that extends the pointwise boundary restriction from smooth functions to all of $W^{1,p}$.

## The Problem

For a smooth function $u \in C^1(\bar\Omega)$ on a bounded domain $\Omega$ with smooth boundary, the boundary values $u|_{\partial\Omega}$ are well-defined and form a function in $C^0(\partial\Omega) \subset L^p(\partial\Omega)$. One would like to extend this to all $u \in W^{1,p}(\Omega)$.

The difficulty: if $(u_n) \subset C^1(\bar\Omega)$ converges to $u \in W^{1,p}(\Omega)$, the convergence is in the $W^{1,p}$ norm, not pointwise or in $C^0$. The boundary values $u_n|_{\partial\Omega}$ may or may not converge pointwise. But they do converge in $L^p(\partial\Omega)$ (with respect to the $(n-1)$-dimensional surface measure), and the limit defines the **trace** of $u$.

## The Trace Theorem

**Theorem (Trace Theorem).** Let $\Omega \subset \mathbb{R}^n$ be a bounded open set with $C^1$ boundary, and $1 \leq p < \infty$. There exists a unique bounded linear operator

$$\gamma: W^{1,p}(\Omega) \to L^p(\partial\Omega)$$

(with respect to the $(n-1)$-dimensional Hausdorff measure on $\partial\Omega$) such that:

1. $\gamma(u) = u|_{\partial\Omega}$ for all $u \in C^1(\bar\Omega)$.
2. $\|\gamma(u)\|_{L^p(\partial\Omega)} \leq C\|u\|_{W^{1,p}(\Omega)}$ for some constant $C$ depending only on $\Omega$ and $p$.

The operator $\gamma$ is called the **trace operator**, and $\gamma(u)$ is the **trace** of $u$ on $\partial\Omega$.

## Proof Sketch

**Step 1: Reduction to a half-space.** Using a partition of unity and a flattening of the boundary, it suffices to prove the result for $\Omega = \mathbb{R}^n_+ = \{x_n > 0\}$ and $\partial\Omega = \{x_n = 0\} \cong \mathbb{R}^{n-1}$.

**Step 2: Estimate for smooth functions.** For $u \in C_c^\infty(\overline{\mathbb{R}^n_+})$:

$$|u(x',0)|^p = -\int_0^\infty \frac{\partial}{\partial x_n}|u(x', x_n)|^p \, dx_n = -\int_0^\infty p|u|^{p-1}\text{sgn}(u)\partial_{x_n}u \, dx_n.$$

By Young's inequality ($ab \leq a^p/p + b^q/q$ with $q = p/(p-1)$):

$$|u(x',0)|^p \leq \int_0^\infty (|u|^p + |\partial_{x_n}u|^p) \, dx_n.$$

Integrating over $x' \in \mathbb{R}^{n-1}$:

$$\int_{\mathbb{R}^{n-1}} |u(x',0)|^p \, dx' \leq \int_{\mathbb{R}^n_+} (|u|^p + |\nabla u|^p) \, dx \leq \|u\|_{W^{1,p}(\mathbb{R}^n_+)}^p.$$

So $\|\gamma(u)\|_{L^p(\mathbb{R}^{n-1})} \leq \|u\|_{W^{1,p}(\mathbb{R}^n_+)}$ for smooth $u$.

**Step 3: Extension by density.** Since $C_c^\infty(\overline{\mathbb{R}^n_+})$ is dense in $W^{1,p}(\mathbb{R}^n_+)$, the estimate shows that $\gamma$ extends uniquely to a bounded linear operator on all of $W^{1,p}$. $\square$

## The Kernel of the Trace Operator

**Theorem.** $\ker \gamma = W^{1,p}_0(\Omega)$.

That is, $\gamma(u) = 0$ if and only if $u \in W^{1,p}_0(\Omega)$ (the closure of $C_c^\infty(\Omega)$).

This theorem characterizes zero Dirichlet boundary conditions: $u \in W^{1,p}_0(\Omega)$ if and only if the trace of $u$ on $\partial\Omega$ vanishes. It justifies the definition of $W^{1,p}_0$ as the space of $W^{1,p}$ functions with zero boundary values.

## Higher-Order Traces

For $u \in W^{2,p}(\Omega)$ with $\Omega$ having $C^2$ boundary, one can define not only the trace of $u$ but also the trace of the normal derivative $\partial u/\partial\nu$ on $\partial\Omega$ (where $\nu$ is the outward unit normal):

$$\gamma_0(u) = u|_{\partial\Omega} \in W^{1-1/p,p}(\partial\Omega), \quad \gamma_1(u) = \frac{\partial u}{\partial\nu}\bigg|_{\partial\Omega} \in W^{-1/p,p}(\partial\Omega).$$

For $p = 2$ and $\Omega$ bounded with $C^2$ boundary:

$$\gamma_0: H^2(\Omega) \to H^{3/2}(\partial\Omega), \quad \gamma_1: H^2(\Omega) \to H^{1/2}(\partial\Omega)$$

are bounded and surjective.

## Fractional Sobolev Spaces and Trace Spaces

The trace $\gamma(u)$ of a function $u \in H^1(\Omega)$ lies not merely in $L^2(\partial\Omega)$ but in the **fractional Sobolev space** $H^{1/2}(\partial\Omega)$. Fractional Sobolev spaces $W^{s,p}$ for non-integer $s$ are defined by interpolation: either by the Fourier transform (for $\Omega = \mathbb{R}^n$) or by real interpolation theory.

**Theorem.** The trace operator $\gamma: H^1(\Omega) \to H^{1/2}(\partial\Omega)$ is bounded and surjective. Its kernel is $H^1_0(\Omega)$.

This implies a decomposition: $H^1(\Omega) = H^1_0(\Omega) \oplus_\perp \{\text{harmonic functions in } H^1(\Omega)\}$, and every function in $H^{1/2}(\partial\Omega)$ extends to a harmonic function in $H^1(\Omega)$.

## Application: The Neumann Problem

The trace theorem is also essential for Neumann boundary conditions. For the Neumann problem $-\Delta u = f$ in $\Omega$, $\partial u/\partial\nu = g$ on $\partial\Omega$, the variational formulation is:

$$\int_\Omega \nabla u \cdot \nabla v = \int_\Omega fv + \int_{\partial\Omega} gv \quad \text{for all } v \in H^1(\Omega).$$

The right-hand side is well-defined because $v|_{\partial\Omega} = \gamma(v) \in L^2(\partial\Omega)$ by the trace theorem, and $g \in L^2(\partial\Omega)$. The Neumann problem has a solution in $H^1(\Omega)$ (modulo constants, since the Laplacian with Neumann conditions is not invertible on the full space) by the Lax-Milgram theorem applied to the quotient $H^1/\mathbb{R}$.
