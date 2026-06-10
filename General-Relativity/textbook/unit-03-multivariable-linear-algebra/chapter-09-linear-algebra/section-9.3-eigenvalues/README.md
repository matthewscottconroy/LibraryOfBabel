# Section 9.3: Eigenvalues, Eigenvectors, and Diagonalization

---

## Section Introduction

Eigenvalues are the intrinsic "frequencies" of a linear map — the directions in which the map acts by pure scaling. Finding the eigenstructure of a linear operator is one of the central problems of applied mathematics: it determines the normal modes of a vibrating system, the stability of a dynamical system, the principal curvatures of a surface, and the eigenfrequencies of a black hole (quasi-normal modes).

---

## 9.3.1 Eigenvalues and Eigenvectors

**Definition**: A nonzero vector **v** is an **eigenvector** of the linear map T (or matrix **A**) with **eigenvalue** λ if:

$$T\mathbf{v} = \lambda \mathbf{v} \quad (\text{equivalently: } \mathbf{A}\mathbf{v} = \lambda \mathbf{v})$$

**Characteristic polynomial**: The eigenvalues satisfy det(**A** − λ**I**) = 0. The polynomial p(λ) = det(**A** − λ**I**) is the **characteristic polynomial** of **A**; it has degree n. By the fundamental theorem of algebra, it has n roots (over ℂ, counting multiplicity).

**Eigenspace**: For each eigenvalue λ, the **eigenspace** E_λ = ker(**A** − λ**I**) is the set of all eigenvectors with that eigenvalue (plus **0**). Its dimension is the **geometric multiplicity** of λ. The **algebraic multiplicity** is the multiplicity of λ as a root of the characteristic polynomial.

**Examples**:
- Rotation by θ ≠ 0, π in ℝ²: no real eigenvalues (the rotation has no fixed direction), but complex eigenvalues e^{±iθ}.
- Symmetric matrix in ℝⁿ: all eigenvalues real, eigenvectors mutually orthogonal (Spectral Theorem).
- Reflection across a line in ℝ²: eigenvalues +1 (along the line) and −1 (perpendicular).

---

## 9.3.2 The Spectral Theorem

**Theorem** (Real Symmetric): Every real symmetric matrix **A** = **A**^T has:
1. All eigenvalues real.
2. Eigenvectors for distinct eigenvalues are orthogonal.
3. There exists an orthonormal basis of ℝⁿ consisting of eigenvectors of **A**.

Equivalently: **A** = **Q** **Λ** **Q**^T where **Λ** = diag(λ₁, ..., λₙ) and **Q** is orthogonal (**Q**^T**Q** = **I**).

*Proof outline*: 
- Real eigenvalues: if λ = a+ib (b≠0) with eigenvector **v** = **u** + i**w**, expand **A**(**u**+i**w**) = (a+ib)(**u**+i**w**); take real/imaginary parts; use **A** = **A**^T and the inner product to show b||**u**||² + b||**w**||² = 0, so b = 0.
- Orthogonality: if **Av** = λ**v** and **Aw** = μ**w** with λ ≠ μ, then λ(**v**·**w**) = **Av**·**w** = **v**·**A**^T**w** = **v**·**Aw** = μ(**v**·**w**); since λ ≠ μ, we get **v**·**w** = 0. □

**Corollary**: The quadratic form **v**^T **A v** = Σ λᵢ (qᵢ)² in the eigenbasis, where qᵢ are the components in the eigenbasis. This diagonalizes the quadratic form.

**In GR**: The metric tensor gᵤᵥ at each point is a symmetric (0,2) tensor. The Spectral Theorem guarantees it can be diagonalized at any given point (but not necessarily simultaneously at all points). Locally, the metric can always be put in the form diag(−1, +1, +1, +1) (Minkowski metric) — this is the content of the equivalence principle in its mathematical form. The eigenvalues of the metric (−1, +1, +1, +1 in 4D) determine its **signature**, a topological invariant of the metric.

---

## 9.3.3 The Jordan Normal Form

For non-diagonalizable matrices (when geometric multiplicity < algebraic multiplicity), the best we can do is the **Jordan normal form**: **A** = **P J P**⁻¹ where **J** is block-diagonal with Jordan blocks:

$$J_k(\lambda) = \begin{pmatrix} \lambda & 1 & 0 & \cdots \\ 0 & \lambda & 1 & \cdots \\ \vdots & & \ddots & 1 \\ 0 & \cdots & 0 & \lambda \end{pmatrix}$$

**Jordan blocks appear** in the theory of resonances in dynamical systems, and in the classification of conjugacy classes of operators. For physics, the Spectral Theorem (symmetric/Hermitian case) is more commonly used — but Jordan form appears in the study of degenerate perturbations and in the theory of singular points of spacetime.

**In GR**: The algebraic type of the Weyl curvature tensor is classified by its Petrov type — essentially the Jordan normal form of a certain linear map on a 3D complex vector space (the "Newman-Penrose operator"). Schwarzschild and Kerr black holes are Petrov type D (two doubly-degenerate eigenvalues). This classification reveals deep structure in the geometry of these spacetimes. [Penrose, R. (1960). "A Spinor Approach to General Relativity." *Annals of Physics*, 10, 171–201.]

---

## 9.3.4 Singular Value Decomposition

Every matrix **A** (m×n, real) can be factored as:

$$\mathbf{A} = \mathbf{U} \mathbf{\Sigma} \mathbf{V}^T$$

where **U** (m×m) and **V** (n×n) are orthogonal, and **Σ** (m×n) is diagonal with non-negative entries σ₁ ≥ σ₂ ≥ ⋯ ≥ 0 (the **singular values**).

The singular values are the square roots of the eigenvalues of **A**^T**A**.

**Applications**: SVD is the fundamental tool for:
- Low-rank approximation (data compression, PCA in statistics)
- Solving least-squares problems (**Ax** = **b** when no exact solution exists)
- Analyzing the sensitivity of linear systems to perturbations (condition number = σ_max/σ_min)
- In numerical relativity: decomposing the constraint equations for the metric

---

## References

- Axler, S. (2015). *Linear Algebra Done Right*, 3rd ed. Springer. [Chapters 7–8 on inner product spaces and the spectral theorem.]
- Penrose, R. (1960). "A Spinor Approach to General Relativity." *Annals of Physics*, 10, 171–201. [The Petrov classification of the Weyl tensor.]
- Trefethen, L.N. and Bau, D. (1997). *Numerical Linear Algebra*. SIAM. [The SVD and its applications in numerical computation; particularly clear and modern.]
