# Section 9.2: Linear Maps and Linear Transformations

---

## Section Introduction

A **linear map** (linear transformation) $T: V\to W$ between vector spaces satisfies two conditions: $T(\mathbf{u}+\mathbf{v}) = T(\mathbf{u})+T(\mathbf{v})$ and $T(\alpha\mathbf{u}) = \alpha T(\mathbf{u})$ for all vectors and scalars. These two conditions — additivity and homogeneity — are the definition of linearity, and they constrain $T$ completely: once you know $T$ on a basis, you know $T$ everywhere. Linear maps are the morphisms of the category of vector spaces — they are the structure-preserving maps.

The **matrix representation** of a linear map (covered in the companion section on matrices) is one description; the intrinsic, basis-free description is another. The intrinsic description is more powerful: it applies to infinite-dimensional vector spaces (function spaces), to abstract vector spaces over arbitrary fields, and to situations where no natural basis exists. In physics, the physical content of a law should not depend on the choice of coordinate system — this is the coordinate-independence principle that underlies both SR and GR. Linear maps defined intrinsically satisfy this principle automatically.

The **kernel** of $T$ is $\ker T = \{\mathbf{v}: T(\mathbf{v})=\mathbf{0}\}$ — the set of vectors that $T$ sends to zero. The **image** (or range) is $\text{im}\,T = \{T(\mathbf{v}): \mathbf{v}\in V\}$. The **rank-nullity theorem** (or dimension theorem): $\dim V = \dim\ker T + \dim\text{im}\,T$. This fundamental result constrains the relationship between injectivity (trivial kernel) and surjectivity (full image), and it generalizes to the index theorems of analysis.

For GR, linear maps appear in multiple guises: the tangent map $f_*: T_pM\to T_{f(p)}N$ of a smooth map between manifolds; the contraction and raising/lowering maps defined by the metric tensor; the exterior derivative $d: \Omega^k(M)\to\Omega^{k+1}(M)$; the covariant derivative $\nabla_X$. Understanding linear maps at the abstract level is essential for understanding these geometric objects.

---

## Subsections

- [9.2.1: Definition and Basic Properties](9.2.1-definition.md)
- [9.2.2: Kernel and Image](9.2.2-kernel-image.md)
- [9.2.3: The Rank-Nullity Theorem](9.2.3-rank-nullity.md)
- [9.2.4: Isomorphisms and Invertible Maps](9.2.4-isomorphisms.md)
- [9.2.5: Linear Maps Between Function Spaces](9.2.5-function-spaces.md)
