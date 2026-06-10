# Section 9.2: Matrices, Determinants, and Systems of Equations

---

## Section Introduction

A matrix is the concrete representation of a linear map in a choice of bases. Matrices are the calculational tools; the underlying linear maps are the conceptual objects. Understanding both levels — the abstract map and its matrix representation — is essential for GR, where the choice of coordinate basis determines the matrix but the underlying tensor is coordinate-independent.

---

## 9.2.1 Matrices as Linear Maps

Given bases {**e**₁, ..., **eₙ**} for V and {**f**₁, ..., **fₘ**} for W, a linear map T: V → W is completely determined by the images T**eⱼ** = Σᵢ Aᵢⱼ **fᵢ**. The matrix **A** = (Aᵢⱼ) (with i indexing rows and j columns) represents T.

**Action on column vectors**: If **v** = Σⱼ vʲ **eⱼ**, then T**v** = Σᵢ (Σⱼ Aᵢⱼ vʲ) **fᵢ**. In matrix form: **A v** with componentwise multiplication.

**Change of basis**: If we change the basis for V via **e**'ⱼ = Σₖ Pₖⱼ **eₖ**, the matrix changes as **A** → **A P** (on the right). If we also change the basis for W: **A** → **Q**⁻¹ **A P**. For endomorphisms T: V → V with the same change of basis in domain and range: **A** → **P**⁻¹ **A P** (similarity transformation). This is the fundamental reason that eigenvalues are basis-independent: they satisfy det(A − λI) = 0, and similar matrices have the same determinant.

**Tensor transformation law**: In GR, the metric transforms as g'_{μν} = (∂x^α/∂x'^μ)(∂x^β/∂x'^ν) g_{αβ}. This is the matrix identity g' = J^T g J, where J is the Jacobian matrix of the coordinate change. It is exactly the change-of-basis formula for a (0,2) tensor.

---

## 9.2.2 Determinants

The **determinant** det(**A**) of an n×n matrix is the unique function of the matrix satisfying:
1. **Multilinearity**: det is linear in each column separately.
2. **Antisymmetry**: swapping two columns changes the sign.
3. **Normalization**: det(**I**) = 1.

These three conditions uniquely determine the determinant, and give the explicit formula:

$$\det(\mathbf{A}) = \sum_{\sigma \in S_n} \text{sgn}(\sigma) \prod_{i=1}^n A_{i,\sigma(i)}$$

where the sum is over all permutations σ of {1, ..., n} and sgn(σ) = ±1 is the sign of the permutation.

**Key properties**:
- det(**AB**) = det(**A**) det(**B**)
- det(**A**^T) = det(**A**)
- **A** is invertible iff det(**A**) ≠ 0; in this case, det(**A**⁻¹) = 1/det(**A**)
- det(λ**A**) = λⁿ det(**A**)
- Row/column operations: adding a multiple of one row to another does not change the determinant; swapping rows negates the determinant; scaling a row multiplies the determinant by the scale factor.

**Geometric meaning**: |det(**A**)| is the volume scaling factor of the linear map T: the volume of T(S) equals |det(**A**)| times the volume of S.

**In GR**: The metric determinant g = det(gᵤᵥ) determines the invariant volume element √(−g) d⁴x. When the metric changes by a diffeomorphism, g transforms by the square of the Jacobian: g' = (det J)² g. The factor √(−g) absorbs this transformation, making ∫f √(−g) d⁴x coordinate-invariant.

---

## 9.2.3 Systems of Linear Equations

The system **Ax** = **b** (m equations, n unknowns) has:
- A unique solution iff det(**A**) ≠ 0 (square case, n = m).
- In general: solutions exist iff **b** ∈ im(**A**) (the column space of **A**).
- The solution set is an affine subspace: one particular solution plus the kernel of **A**.

**Gaussian elimination**: The standard algorithm for solving linear systems. Row-reduce the augmented matrix [**A** | **b**] to row-echelon form. Time complexity: O(n³) for dense matrices.

**Cramer's rule**: xⱼ = det(**A**ⱼ)/det(**A**), where **A**ⱼ is the matrix with the j-th column replaced by **b**. Useful theoretically but computationally inefficient.

**In linearized GR**: Perturbing the metric gᵤᵥ = ηᵤᵥ + hᵤᵥ (where ηᵤᵥ is Minkowski and hᵤᵥ is small) and linearizing the Einstein equations gives a system of PDEs for hᵤᵥ. In the transverse-traceless gauge, these reduce to the wave equation □h̄ᵤᵥ = −16πTᵤᵥ — a linear system of PDEs. The theory of such systems requires the techniques of both linear algebra and PDEs (Chapter 11).

---

## References

- Axler, S. (2015). *Linear Algebra Done Right*, 3rd ed. Springer. [Chapter 10 on trace and determinant.]
- Strang, G. (2016). *Introduction to Linear Algebra*, 5th ed. Wellesley-Cambridge Press. [The most computational and accessible treatment; Chapter 5 on determinants is excellent.]
- Horn, R.A. and Johnson, C.R. (2012). *Matrix Analysis*, 2nd ed. Cambridge University Press. [The definitive reference on matrix theory for researchers; advanced but authoritative.]
