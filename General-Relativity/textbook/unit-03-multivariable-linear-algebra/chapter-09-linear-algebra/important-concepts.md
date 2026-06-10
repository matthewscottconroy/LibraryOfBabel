# Chapter 9 Important Concepts

---

**Vector Space** — A set V with addition and scalar multiplication satisfying 8 axioms. The abstract generalization of arrows: any collection of objects that can be linearly combined.

**Subspace** — A subset of V closed under addition and scalar multiplication. The intersection of subspaces is a subspace; the span of any set is a subspace.

**Basis** — A maximal linearly independent set, or equivalently, a minimal spanning set. Every basis of a finite-dimensional space has the same cardinality (the dimension).

**Dimension** — The cardinality of any basis. Invariant under change of basis and isomorphism. dim(ℝⁿ) = n; dim(C([a,b])) = ∞.

**Linear Map (Linear Transformation)** — A map T: V → W preserving addition and scalar multiplication. Completely determined by its action on any basis.

**Kernel (Null Space)** — ker(T) = {**v** : T**v** = **0**}. A subspace of V. T is injective iff ker(T) = {**0**}.

**Image (Column Space, Range)** — im(T) = {T**v** : **v** ∈ V}. A subspace of W.

**Rank-Nullity Theorem** — dim(ker T) + dim(im T) = dim(V). One of the central structural results of linear algebra.

**Matrix** — The coordinate representation of a linear map in given bases. The (i,j) entry of the matrix is the i-th component of the image of the j-th basis vector.

**Determinant** — A scalar assigned to a square matrix, measuring the volume-scaling factor of the corresponding linear map. det(**AB**) = det(**A**)det(**B**). **A** is invertible iff det(**A**) ≠ 0.

**Eigenvalue** — A scalar λ such that **Av** = λ**v** for some nonzero **v**. Satisfies det(**A** − λ**I**) = 0.

**Eigenvector** — A nonzero vector satisfying **Av** = λ**v**. Invariant direction of the linear map (scaled but not rotated).

**Characteristic Polynomial** — p(λ) = det(**A** − λ**I**). Eigenvalues are its roots. Degree n for an n×n matrix.

**Spectral Theorem** — Every real symmetric matrix is diagonalizable with real eigenvalues and orthogonal eigenvectors. The canonical form for symmetric linear maps.

**Signature** — For a symmetric bilinear form: the pair (p, q) where p is the number of positive eigenvalues and q the number of negative eigenvalues. An invariant by Sylvester's law of inertia. For the Minkowski metric: signature (1, 3) or (3, 1) depending on convention.

**Dual Space** — V* = L(V, ℝ): the space of linear functionals on V. Same dimension as V. Elements are covectors (1-forms).

**Dual Basis** — The basis {**eⁱ**} of V* defined by **eⁱ**(**eⱼ**) = δⁱⱼ. Dual to the basis {**eⱼ**} of V.

**Covector (1-Form)** — An element of V*. Acts on vectors to produce scalars. Components written with lower indices.

**Einstein Summation Convention** — Repeated upper-lower index pairs imply summation: aᵢ bⁱ = Σᵢ aᵢ bⁱ. Streamlines tensor calculations by eliminating explicit summation signs.

**Contravariant Vector** — A vector; its components transform with the inverse Jacobian under coordinate change. Components carry upper indices.

**Covariant Vector (Covector)** — A 1-form; its components transform with the Jacobian (not its inverse). Components carry lower indices.

**Tensor** — A multilinear map from r copies of V* and s copies of V to ℝ. A (r, s) tensor has r upper indices and s lower indices in its components.

**Metric Tensor** — A symmetric, non-degenerate (0,2) tensor. Provides an inner product on the tangent space. In GR, has Lorentzian signature (−, +, +, +).

**Raising/Lowering Indices** — Using the metric gᵤᵥ or its inverse g^{μν} to convert between upper and lower indices. The metric isomorphism between V and V*.

**Contraction** — Summing over one upper and one lower repeated index. Reduces a (r, s) tensor to a (r−1, s−1) tensor. The trace of a matrix is the contraction of a (1,1) tensor.

**Tensor Product** — For S of type (r,s) and T of type (p,q): S⊗T is of type (r+p, s+q). The product is not symmetric.

**Symmetric Tensor** — A tensor T_{ij} = T_{ji} (for a (0,2) tensor). The metric is symmetric. Symmetric tensors have ½n(n+1) independent components in n dimensions.

**Antisymmetric Tensor** — T_{ij} = −T_{ji}. Antisymmetric (0,2) tensors have ½n(n−1) independent components. The electromagnetic field tensor Fᵤᵥ is antisymmetric.

**Levi-Civita Symbol** — The completely antisymmetric symbol ε_{i₁...iₙ} with ε_{12...n} = 1. Represents the volume form. Used to define determinants and cross products.

**Kronecker Delta** — δⁱⱼ = 1 if i = j, 0 otherwise. The components of the identity map. The metric g^{μα}g_{αν} = δ^μ_ν.

**Petrov Classification** — The algebraic classification of the Weyl curvature tensor (or related operators) based on eigenvalue structure. Types I, II, III, N, D, O. Schwarzschild and Kerr are type D.

**Singular Value Decomposition (SVD)** — **A** = **U Σ V**^T: factorization into orthogonal matrices and a diagonal matrix of singular values. The fundamental tool for numerical linear algebra and least-squares problems.
