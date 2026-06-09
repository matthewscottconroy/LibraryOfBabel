# Unit 8: Variational Methods for PDEs

Every PDE studied in this course has a variational formulation: the Laplace equation $\Delta u = 0$ is the Euler-Lagrange equation for minimizing $\int|\nabla u|^2$; the wave equation is the Euler-Lagrange equation for the action functional; Schrödinger's equation is a gradient flow of an energy functional; even the heat equation is the gradient flow of the Dirichlet energy with respect to the $L^2$ metric. The variational perspective is not merely a reformulation — it is the foundation for proving existence of solutions (direct method of the calculus of variations), for understanding the correct solution spaces (Sobolev spaces), and for constructing numerical approximations (Galerkin and finite element methods).

## Why Variational Methods?

Classical PDE theory asks: given a boundary value problem, find a function $u\in C^2(\Omega)\cap C(\bar\Omega)$ satisfying the PDE pointwise. This is the **strong formulation**. For smooth data and smooth domains, the strong and weak formulations are equivalent. But real problems involve:

- **Irregular data:** Source terms $f\in L^2$, not continuous; boundary data in $H^{1/2}$, not $C^1$.
- **Corner singularities:** Solutions near corners of the domain have singular gradients even for smooth data.
- **Nonsmooth media:** Permeability, conductivity, or density may be discontinuous (piecewise constant in composite materials).

In all these cases, a classical solution does not exist, but a **weak (variational) solution** exists and is unique. The variational framework provides both the correct definition of the solution and the tools to prove its existence.

## The Direct Method

The oldest variational approach is the **direct method of the calculus of variations**: to find a minimizer of a functional $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$, take a minimizing sequence $\{u_n\}$ with $\mathcal{E}[u_n]\to\inf \mathcal{E}$, extract a convergent subsequence (by compactness), and verify that the limit minimizes $\mathcal{E}$. The key requirements are:
- **Coercivity:** $\mathcal{E}[u] \to\infty$ as $\|u\|_{W^{1,p}}\to\infty$ (prevents the minimizer from escaping to infinity).
- **Weak lower semicontinuity:** $\liminf_n \mathcal{E}[u_n] \geq \mathcal{E}[u]$ when $u_n\rightharpoonup u$ weakly (convexity of $L$ in $\nabla u$ suffices).
- **Compact embedding:** $W^{1,p}(\Omega)\hookrightarrow L^q(\Omega)$ compactly for appropriate $q$ (Rellich-Kondrachov).

## Structure of This Unit

**Chapter 1: Calculus of Variations** develops the classical theory: functionals, the first variation (Euler-Lagrange equation), natural boundary conditions, and constrained optimization with Lagrange multipliers. The Euler-Lagrange equation is derived in full generality and applied to the Dirichlet energy (giving Laplace's equation), the bending energy of beams (giving the Euler-Bernoulli beam equation), the arc length functional (giving geodesics), and the eigenvalue problem (giving the Rayleigh quotient characterization of eigenvalues).

**Chapter 2: Variational Formulation of PDEs** translates PDE boundary value problems into variational (weak) form, introduces Sobolev spaces $H^k(\Omega)$ and $W^{k,p}(\Omega)$ as the natural function spaces for weak solutions, and proves the Lax-Milgram theorem — the cornerstone of the existence theory for linear elliptic PDEs. The weak formulation of Poisson's equation $-\Delta u = f$ on a bounded domain with Dirichlet boundary conditions is: find $u \in H^1_0(\Omega)$ such that $\int_\Omega\nabla u\cdot\nabla v\,dx = \int_\Omega fv\,dx$ for all $v\in H^1_0(\Omega)$.

**Chapter 3: Finite Element Method** shows how the variational formulation is the starting point for numerical approximation. The Galerkin method projects the variational problem onto a finite-dimensional subspace $V_h\subset H^1_0(\Omega)$, converting it to a linear system $Au = f$ (stiffness matrix times coefficient vector equals load vector). The finite element method (FEM) chooses $V_h$ to be piecewise polynomial functions on a mesh, giving a sparse stiffness matrix that can be solved efficiently.

## Prerequisites

This unit requires:
- Functional analysis: inner product spaces, Hilbert spaces, bounded linear functionals (Riesz representation theorem).
- Measure theory and $L^2$ theory: Lebesgue integral, $L^2(\Omega)$ as a Hilbert space.
- Linear algebra: symmetric positive definite matrices, Cholesky factorization.
- Basic PDE theory from previous units: Green's first identity, boundary conditions, classical solutions.

## Key Theorems

**Euler-Lagrange theorem:** If $u$ minimizes $\mathcal{E}[u] = \int_\Omega L(x,u,\nabla u)\,dx$ over $H^1_0(\Omega)$ (or with appropriate boundary conditions), then $u$ satisfies the Euler-Lagrange PDE $-\text{div}(L_p) + L_u = 0$ in $\Omega$.

**Lax-Milgram theorem:** Let $H$ be a Hilbert space, $a:H\times H\to\mathbb{R}$ a continuous coercive bilinear form ($a(u,u) \geq \alpha\|u\|^2$), and $F\in H^*$ a continuous linear functional. Then there exists a unique $u\in H$ with $a(u,v) = F(v)$ for all $v\in H$. Moreover, $\|u\| \leq \|F\|_{H^*}/\alpha$.

**Céa's lemma (best approximation):** If $u_h\in V_h\subset H$ is the Galerkin approximation, then $\|u-u_h\|_H \leq \frac{M}{\alpha}\inf_{v_h\in V_h}\|u-v_h\|_H$ (the Galerkin error is bounded by the best approximation error in $V_h$, up to a constant $M/\alpha$ from the condition number of $a$).

These three theorems together provide a complete theory: existence and uniqueness (Lax-Milgram), convergence of the Galerkin approximation (Céa), and optimal rates of convergence when $V_h$ consists of piecewise polynomials of degree $k$ on a mesh of size $h$ (approximation theory gives $\inf_{v_h}\|u-v_h\|_{H^1} = O(h^k\|u\|_{H^{k+1}})$, leading to the finite element error estimate $\|u-u_h\|_{H^1} = O(h^k)$).
