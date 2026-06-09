# Overview of Finite Elements in Higher Dimensions

The extension of the finite element method from 1D to 2D and 3D requires: (1) a mesh of the domain $\Omega$ using triangles (2D) or tetrahedra (3D) as elements; (2) local basis functions on each element; (3) a global assembly procedure that combines element stiffness matrices into the global system; and (4) efficient solvers for the resulting sparse linear systems. The theoretical foundations — Céa's lemma, the approximation theory for piecewise polynomials — extend directly, giving the same optimal error rates as in 1D.

## Triangular Meshes in 2D

**Triangulation.** A triangulation $\mathcal{T}_h$ of a polygonal domain $\Omega\subset\mathbb{R}^2$ is a partition of $\Omega$ into non-overlapping triangles $\{T_e\}$ such that: (i) $\bigcup_e T_e = \bar\Omega$; (ii) any two triangles share at most a full edge or a vertex (conforming mesh — no hanging nodes). The mesh size is $h = \max_e\text{diam}(T_e)$.

**Shape regularity.** A family of meshes $\{\mathcal{T}_h\}$ is **shape regular** if there exists a constant $\sigma$ such that $h_e/\rho_e \leq \sigma$ for all elements $T_e$ and all $h$, where $\rho_e$ is the radius of the largest inscribed circle. This prevents degenerate (thin/sliver) triangles, which would cause deterioration of the approximation constants.

## Linear Triangular Elements (P1)

The simplest 2D finite element is the **piecewise linear** (P1) element. On each triangle $T_e$ with vertices $\mathbf{x}_{e,1}, \mathbf{x}_{e,2}, \mathbf{x}_{e,3}$, the local basis functions are the three **barycentric coordinates** $\lambda_1, \lambda_2, \lambda_3$:

$$\lambda_k(\mathbf{x}) = \frac{\text{area of triangle opposite vertex }k \text{ containing }\mathbf{x}}{\text{area of }T_e}.$$

Properties: $\lambda_k(\mathbf{x}_{e,j}) = \delta_{kj}$, $\lambda_1+\lambda_2+\lambda_3 = 1$, $\lambda_k\geq 0$ on $T_e$.

**Global basis functions.** The global basis function $\phi_j$ associated with node $\mathbf{x}_j$ is:
- $\phi_j(\mathbf{x}) = \lambda_k$ (the barycentric coordinate corresponding to $\mathbf{x}_j$) on each triangle $T_e$ containing $\mathbf{x}_j$.
- $\phi_j(\mathbf{x}) = 0$ on triangles not containing $\mathbf{x}_j$.

These functions are piecewise linear, globally continuous ($\phi_j\in C^0(\Omega)$), and satisfy $\phi_j(\mathbf{x}_i) = \delta_{ij}$. Their support is the "star" of node $j$ (union of all triangles containing $\mathbf{x}_j$).

**Gradients.** On each triangle $T_e$, $\lambda_k$ is affine, so $\nabla\lambda_k$ is constant on $T_e$:

$$\nabla\lambda_k = \frac{1}{2|T_e|}\mathbf{n}_{e,k}^\perp,$$

where $\mathbf{n}_{e,k}$ is the outward normal to the edge opposite vertex $k$, scaled by the edge length.

## Element Stiffness Matrix

The **element stiffness matrix** for triangle $T_e$ (with basis $\lambda_1,\lambda_2,\lambda_3$) is the $3\times 3$ matrix:

$$k^e_{ij} = a_e(\lambda_j,\lambda_i) = \int_{T_e}\nabla\lambda_j\cdot\nabla\lambda_i\,d\mathbf{x} = |T_e|\,\nabla\lambda_j\cdot\nabla\lambda_i$$

(since $\nabla\lambda_j$ is constant on $T_e$, the integral is just $|T_e|$ times the dot product). The $3\times 3$ matrix $k^e$ is the **element stiffness matrix**.

## Assembly

The **global stiffness matrix** $\mathbf{K}$ (of size $N_{\text{dof}}\times N_{\text{dof}}$, where $N_{\text{dof}}$ is the number of interior nodes) is assembled by summing element contributions:

$$K_{IJ} = \sum_{e: T_e\text{ contains nodes }I,J} k^e_{\text{local}(I,e),\text{local}(J,e)},$$

where $\text{local}(I,e)$ is the local index (1, 2, or 3) of global node $I$ in element $e$.

**Assembly algorithm:**
```
Initialize K = 0, F = 0
For each element T_e:
    Compute k^e (3x3 element stiffness matrix)
    Compute f^e (3-vector element load vector)
    For i = 1,2,3:
        I = global node index of local node i in T_e
        For j = 1,2,3:
            J = global node index of local node j in T_e
            K[I,J] += k^e[i,j]
        F[I] += f^e[i]
Apply boundary conditions (eliminate rows/cols for Dirichlet nodes)
Solve K U = F
```

The resulting $\mathbf{K}$ is sparse: each row has at most $\deg_{\max}+1$ nonzeros (where $\deg_{\max}$ is the maximum number of triangles sharing a node). This sparsity — essential for efficient solution — is the main practical advantage of FEM over spectral methods for complex geometries.

## Error Analysis in 2D

For the P1 finite element method on a shape-regular mesh of triangles:

**Interpolation estimate.** For $u\in H^2(\Omega)$ and $u_I$ the piecewise linear interpolant:

$$\|u-u_I\|_{H^1(\Omega)} \leq Ch|\nabla^2 u|_{L^2(\Omega)}.$$

Here $|\nabla^2 u|_{L^2}^2 = \int|\partial^2_{ij}u|^2\,d\mathbf{x}$ is the $H^2$ seminorm.

**FEM error.** By Céa's lemma: $\|u-u_h\|_{H^1} \leq \|u-u_I\|_{H^1} \leq Ch\|u\|_{H^2}$.

**$L^2$ error** (Aubin-Nitsche): $\|u-u_h\|_{L^2} \leq Ch^2\|u\|_{H^2}$.

These are optimal: it can be shown that no piecewise-linear method can achieve better than $O(h)$ in $H^1$ and $O(h^2)$ in $L^2$ for general $H^2$ solutions. The FEM achieves these optimal rates.

## Higher-Order Elements

**P2 elements.** Add midpoint nodes on each edge. Local basis: 6 Lagrange quadratic polynomials per triangle. Error: $O(h^2)$ in $H^1$, $O(h^3)$ in $L^2$.

**Quadrilateral elements.** On rectangles (or quadrilaterals via bilinear maps), use products of 1D polynomials: bilinear (Q1), biquadratic (Q2) elements.

**High-order ($hp$) elements.** On smooth regions, increase polynomial degree $p$ with element size $h$ fixed. For analytic solutions: exponential convergence in the number of degrees of freedom. The $hp$-FEM (Babuška and Guo, 1980s) is the optimal strategy for problems with smooth solutions but corner singularities.

## Adaptive Mesh Refinement

For solutions with local singularities (corners, layers), uniform meshes are inefficient: the $O(h)$ convergence in $H^1$ gives $\|u-u_h\|_{H^1} \leq Ch\|u\|_{H^2}$, but $\|u\|_{H^2}$ may be large (singular $u$). Adaptive mesh refinement (AMR) concentrates elements where the error is large:

1. **Solve:** Compute $u_h$ on the current mesh.
2. **Estimate:** Compute an **a posteriori error estimator** $\eta_e$ for each element (e.g., the residual $\|-\Delta u_h - f\|_{L^2(T_e)}$ or the jump in normal derivatives $\|[\nabla u_h\cdot\nu]\|_{L^2(\partial T_e)}$).
3. **Mark:** Flag elements with $\eta_e > \theta\max_{e'}\eta_{e'}$ for refinement (Dörfler marking).
4. **Refine:** Bisect marked elements (edge bisection, red-green refinement, or Newest Vertex Bisection to maintain conformity).
5. **Repeat** until $\sum_e\eta_e^2 < \text{TOL}^2$.

**Optimal complexity.** For solutions with corner singularities of the form $r^\alpha$ (with $\alpha < 1$, e.g., $\alpha = \pi/\omega$ for a reentrant corner of angle $\omega > \pi$), adaptive refinement recovers optimal $O(N^{-1/2})$ error in $H^1$ (where $N$ is the number of elements) — the same rate as for smooth solutions. Uniform refinement would give $O(N^{-\alpha/2})$ — much slower for small $\alpha$.

## Efficient Solvers

The global stiffness matrix $\mathbf{K}$ is sparse, symmetric, and positive definite. For $N$ degrees of freedom:
- **Direct solvers** (Cholesky factorization): $O(N^{3/2})$ operations in 2D, $O(N^2)$ in 3D — too slow for large problems.
- **Conjugate gradient (CG):** $O(N\cdot\kappa^{1/2})$ iterations, where $\kappa = \lambda_{\max}/\lambda_{\min} \sim h^{-2}$ is the condition number. Each iteration costs $O(N)$. Total: $O(N^2/h) = O(N^2)$ — still too slow.
- **Multigrid preconditioned CG:** $O(N\log N)$ or even $O(N)$ operations (optimal) — the state-of-the-art approach. Multigrid exploits the hierarchical structure of the FEM mesh.

The combination of adaptive mesh refinement and multigrid solvers makes the finite element method optimal in both approximation and computational cost, enabling the solution of PDE problems with millions of unknowns in seconds on modern hardware.
