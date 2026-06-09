# Chapter 3: The Finite Element Method

The finite element method (FEM) is the dominant numerical method for solving elliptic and parabolic PDEs on complex geometries. It is built directly on the variational formulation: the weak form of the PDE is projected onto a finite-dimensional subspace $V_h\subset H^1_0(\Omega)$, converting the abstract variational problem into a finite linear system. The subspace $V_h$ consists of piecewise polynomial functions on a mesh — the "finite elements." The method produces a sparse, symmetric, positive definite stiffness matrix that can be solved efficiently by direct or iterative methods, and it provides optimal convergence rates measured in Sobolev norms.

## From Variational Problem to Linear System

**Setting.** Find $u\in H^1_0(\Omega)$ such that $a(u,v) = F(v)$ for all $v\in H^1_0(\Omega)$, where $a(u,v) = \int_\Omega\nabla u\cdot\nabla v\,dx$ and $F(v) = \int_\Omega fv\,dx$.

**Galerkin projection.** Choose a finite-dimensional subspace $V_h\subset H^1_0(\Omega)$ (dim $V_h = N$) and find $u_h\in V_h$ such that:

$$a(u_h, v_h) = F(v_h) \quad \text{for all }v_h\in V_h. \tag{Galerkin}$$

This is the **Galerkin equation**. By Lax-Milgram applied to $V_h$ (a Hilbert space with the induced $H^1$ norm), a unique $u_h\in V_h$ exists. It is the $a$-orthogonal projection of the exact solution $u$ onto $V_h$.

**Linear system.** Let $\{\phi_1,\ldots,\phi_N\}$ be a basis for $V_h$. Write $u_h = \sum_{j=1}^N U_j\phi_j$. Substituting into (Galerkin) with $v_h = \phi_i$:

$$\sum_{j=1}^N U_j a(\phi_j,\phi_i) = F(\phi_i), \quad i=1,\ldots,N,$$

i.e., $\mathbf{K}\mathbf{U} = \mathbf{F}$ where $K_{ij} = a(\phi_j,\phi_i) = \int_\Omega\nabla\phi_j\cdot\nabla\phi_i\,dx$ (stiffness matrix) and $F_i = F(\phi_i) = \int_\Omega f\phi_i\,dx$ (load vector).

## Structure of This Chapter

**Section 1: Galerkin Method** develops the abstract Galerkin framework: the best approximation property (Céa's lemma), the Galerkin orthogonality ($a(u-u_h,v_h) = 0$ for all $v_h\in V_h$), and the error estimate in terms of the best approximation error. The Galerkin method applies to any finite-dimensional subspace — the specific choice of basis functions is what distinguishes different methods (FEM, spectral methods, boundary element methods).

**Section 2: Finite Elements in 1D** introduces the simplest finite element: piecewise linear functions on a uniform mesh of size $h$ on $[0,1]$. The hat functions $\phi_j(x) = $ piecewise linear with $\phi_j(x_j) = 1$ and $\phi_j(x_i) = 0$ for $i\neq j$ form the standard basis. The resulting stiffness matrix is the tridiagonal matrix $K_{ij} = (-1)^{|i-j|}/h$ — a familiar finite difference matrix, now derived variationally. The error estimate $\|u-u_h\|_{H^1} \leq Ch\|u''\|_{L^2}$ follows from Céa's lemma and the approximation theory for piecewise linear interpolants.

**Section 3: Overview of Higher Dimensions** surveys the extension to 2D and 3D. Meshes of triangles (2D) or tetrahedra (3D) replace the uniform 1D mesh. Local basis functions are defined element-by-element using the reference element and affine maps. The global stiffness matrix is assembled by summing element stiffness matrices. Adaptive mesh refinement (refining the mesh where the solution is rough) and $hp$-refinement (increasing polynomial degree in smooth regions) provide exponential convergence for smooth solutions.

## Key Error Estimate: Céa's Lemma

**Theorem (Céa's Lemma).** If $u$ solves the continuous problem and $u_h$ solves the Galerkin problem, then:

$$\|u-u_h\|_H \leq \frac{M}{\alpha}\inf_{v_h\in V_h}\|u-v_h\|_H.$$

**Proof.** By Galerkin orthogonality: $a(u-u_h,v_h) = 0$ for all $v_h\in V_h$. For any $w_h\in V_h$:

$$\alpha\|u-u_h\|^2 \leq a(u-u_h,u-u_h) = a(u-u_h,u-w_h) + a(u-u_h,w_h-u_h).$$

The second term vanishes (Galerkin orthogonality: $w_h - u_h\in V_h$). The first term: $|a(u-u_h,u-w_h)| \leq M\|u-u_h\|\|u-w_h\|$. So $\alpha\|u-u_h\| \leq M\|u-w_h\|$ for all $w_h\in V_h$. Taking infimum: $\|u-u_h\| \leq (M/\alpha)\inf_{w_h\in V_h}\|u-w_h\|$. $\square$

**Interpretation.** The FEM error is at most $(M/\alpha)$ times the best approximation error — the error of the best possible element in $V_h$ for approximating $u$. The quantity $M/\alpha$ is the condition number of the bilinear form; for the Laplacian with $M = \alpha = 1$, Céa's lemma gives equality with the best approximation.

## The FEM Error Rate

For piecewise polynomials of degree $k$ on a mesh of size $h$:

**Interpolation estimate:** $\inf_{v_h\in V_h}\|u-v_h\|_{H^1} \leq Ch^k\|u\|_{H^{k+1}}$.

**FEM error (Sobolev norm):** $\|u-u_h\|_{H^1} \leq Ch^k\|u\|_{H^{k+1}}$.

**$L^2$ error (Aubin-Nitsche duality):** $\|u-u_h\|_{L^2} \leq Ch^{k+1}\|u\|_{H^{k+1}}$ (one extra power of $h$, from the Aubin-Nitsche trick).

For $k=1$ (piecewise linear): $\|u-u_h\|_{H^1} = O(h)$ and $\|u-u_h\|_{L^2} = O(h^2)$ — quadratic convergence in $L^2$, linear in $H^1$.

## Physical Picture

The FEM has a clear physical interpretation. Each node $x_j$ has an associated hat function $\phi_j$, which is 1 at $x_j$ and 0 at all other nodes. The solution $u_h = \sum U_j\phi_j$ is a piecewise linear interpolation with nodal values $U_j$. The stiffness matrix entry $K_{ij} = \int\nabla\phi_j\cdot\nabla\phi_i\,dx$ is nonzero only when the supports of $\phi_i$ and $\phi_j$ overlap (neighboring nodes) — giving a sparse matrix with bandwidth proportional to the number of neighboring nodes.

The FEM thus converts the PDE into a spring network: each edge between neighboring nodes carries a stiffness $K_{ij}$, and the equilibrium displacement $U_j$ under applied loads $F_i$ satisfies $\mathbf{KU} = \mathbf{F}$. This analogy with structural mechanics is the historical origin of the finite element method (Turner, Clough, Martin, Topp, 1956).
