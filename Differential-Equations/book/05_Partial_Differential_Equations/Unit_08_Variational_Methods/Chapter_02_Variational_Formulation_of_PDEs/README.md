# Chapter 2: Variational Formulation of PDEs

The variational formulation of a PDE replaces the requirement that $u$ satisfy an equation pointwise with the requirement that $u$ satisfies an **integral identity** (the weak form) for all test functions. This reformulation has three advantages: (1) it extends the class of admissible solutions to functions that may not have classical (pointwise) second derivatives; (2) it provides the natural framework for existence proofs via the Lax-Milgram theorem; and (3) it is the direct starting point for Galerkin and finite element numerical methods. This chapter develops the weak formulation systematically, introduces Sobolev spaces as the appropriate function spaces, and proves the Lax-Milgram theorem.

## From Strong to Weak Formulation

**Classical (strong) formulation.** Find $u\in C^2(\Omega)\cap C(\bar\Omega)$ such that $-\Delta u = f$ in $\Omega$, $u = g$ on $\partial\Omega$.

**Weak formulation.** Multiply $-\Delta u = f$ by a test function $v\in C_c^\infty(\Omega)$ and integrate:

$$-\int_\Omega(\Delta u)v\,dx = \int_\Omega fv\,dx.$$

Integrate by parts (Green's first identity): $-\int_\Omega(\Delta u)v = \int_\Omega\nabla u\cdot\nabla v - \int_{\partial\Omega}(\partial_\nu u)v = \int_\Omega\nabla u\cdot\nabla v$ (since $v = 0$ on $\partial\Omega$). So:

$$\int_\Omega\nabla u\cdot\nabla v\,dx = \int_\Omega fv\,dx \quad \text{for all }v\in C_c^\infty(\Omega). \tag{Weak form}$$

A classical solution satisfies the weak form. But the weak form makes sense for any $u\in H^1(\Omega)$ with $u|_{\partial\Omega} = g$ and any $v\in H^1_0(\Omega)$ — much larger function spaces. A **weak solution** is any $u\in H^1(\Omega)$ satisfying $u = g$ on $\partial\Omega$ in the trace sense and the integral identity (Weak form) for all $v\in H^1_0(\Omega)$.

## The Three Sections

**Section 1: Weak Solutions** formalizes the notion of weak (distributional) solution, shows that any classical solution is also a weak solution (but not conversely), discusses when weak solutions are actually smooth (regularity), and presents examples of problems where only weak solutions exist (discontinuous coefficients, irregular boundary data).

**Section 2: Sobolev Spaces** introduces $H^k(\Omega) = W^{k,2}(\Omega)$ — the space of $L^2$ functions whose weak derivatives up to order $k$ are also in $L^2$. The inner product on $H^k(\Omega)$ makes it a Hilbert space. Key results: the Poincaré inequality $\|u\|_{L^2} \leq C\|\nabla u\|_{L^2}$ (for $u\in H^1_0(\Omega)$) and the Sobolev embedding theorem $H^1(\Omega)\hookrightarrow L^{2n/(n-2)}(\Omega)$ (for $n \geq 3$).

**Section 3: Lax-Milgram Theorem** proves the existence and uniqueness of weak solutions. The abstract theorem: if $a:H\times H\to\mathbb{R}$ is a bounded coercive bilinear form on a Hilbert space $H$ and $F\in H^*$ is a bounded linear functional, then there exists a unique $u\in H$ with $a(u,v) = F(v)$ for all $v\in H$. For the Poisson problem: $a(u,v) = \int_\Omega\nabla u\cdot\nabla v$, $F(v) = \int_\Omega fv$, $H = H^1_0(\Omega)$, and the Poincaré inequality gives coercivity.

## Key Theorem: Lax-Milgram

**Theorem (Lax-Milgram, 1954).** Let $H$ be a real Hilbert space with inner product $\langle\cdot,\cdot\rangle$ and norm $\|\cdot\|$. Let $a:H\times H\to\mathbb{R}$ be a bilinear form satisfying:

1. **Boundedness:** $|a(u,v)| \leq M\|u\|\,\|v\|$ for all $u,v\in H$.
2. **Coercivity:** $a(u,u) \geq \alpha\|u\|^2$ for all $u\in H$, some $\alpha > 0$.

Let $F:H\to\mathbb{R}$ be a bounded linear functional ($|F(v)| \leq \|F\|_{H^*}\|v\|$). Then there exists a unique $u\in H$ such that $a(u,v) = F(v)$ for all $v\in H$, and $\|u\| \leq \|F\|_{H^*}/\alpha$.

**Applications.** The Lax-Milgram theorem applies to:
- Poisson's equation with coercive $a(u,v) = \int|\nabla u||\nabla v|$ (via Poincaré).
- Elliptic equations $-\text{div}(A\nabla u) + cu = f$ with $A$ uniformly positive definite and $c \geq 0$.
- Parabolic equations in time-integrated form.
- Biharmonic equation $\Delta^2 u = f$ with $a(u,v) = \int\Delta u\Delta v$.

## Why Sobolev Spaces?

Sobolev spaces are not just a technical convenience; they are the natural setting for the variational theory:

- The Dirichlet energy $\int|\nabla u|^2\,dx$ is finite exactly for $u\in H^1(\Omega)$.
- The trace $u|_{\partial\Omega}$ is well-defined for $u\in H^1(\Omega)$ (trace theorem), even though a general $L^2$ function has no boundary values.
- The Poincaré inequality makes $\|\nabla u\|_{L^2}$ a norm on $H^1_0(\Omega)$ (equivalent to the full $H^1$ norm), simplifying the coercivity estimate.
- Sobolev embeddings control the nonlinear terms in semilinear PDEs.

The development of Sobolev spaces in this chapter provides the analytical foundation for all subsequent work: the Galerkin method (Chapter 3) is simply the Lax-Milgram theorem applied in a finite-dimensional subspace, and the finite element method is a systematic way to choose optimal finite-dimensional subspaces.
