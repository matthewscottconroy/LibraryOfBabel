# Section 7.1: Euclidean Space ℝⁿ

---

## Section Introduction

The real line $\mathbb{R}$ is the arena for single-variable calculus. Multivariable calculus takes place in **Euclidean space** $\mathbb{R}^n$: the set of all $n$-tuples of real numbers $(x^1, x^2, \ldots, x^n)$. For $n = 2$ this is the plane; for $n = 3$ it is the space we inhabit; for $n > 3$ it is a generalization that is mathematically natural but geometrically unvisualizable — though this need not hinder us.

$\mathbb{R}^n$ has three interrelated structures that make it useful for analysis. As a **set**, it has elements (points). As a **vector space**, points can be added and scaled, and vectors represent displacements. As a **metric space**, it carries a notion of distance: $d(\mathbf{x}, \mathbf{y}) = \|\mathbf{x} - \mathbf{y}\| = \sqrt{\sum_i (x^i - y^i)^2}$. The metric encodes the geometry and underlies the definition of limits.

The **inner product** (dot product) $\mathbf{x}\cdot\mathbf{y} = \sum_i x^i y^i$ encodes not just distances but angles. Two vectors are orthogonal when their inner product is zero. The Cauchy-Schwarz inequality $|\mathbf{x}\cdot\mathbf{y}|\leq \|\mathbf{x}\|\|\mathbf{y}\|$ bounds the inner product by the product of lengths and is one of the most useful inequalities in mathematics.

For physics, the distinction between points (locations) and vectors (displacements) is crucial and eventually leads to the concept of a manifold. In GR, spacetime is not $\mathbb{R}^4$ but a curved 4-manifold — Euclidean space is replaced by a space that is locally like $\mathbb{R}^4$ but globally curved. The linear structures of $\mathbb{R}^n$ survive locally as the **tangent space** at each point. Understanding $\mathbb{R}^n$ thoroughly is the prerequisite for understanding manifolds.

---

## Subsections

- [7.1.1: Points, Vectors, and the Vector Space Structure](7.1.1-vector-space.md)
- [7.1.2: The Euclidean Inner Product and Norm](7.1.2-inner-product.md)
- [7.1.3: Cauchy-Schwarz and Triangle Inequalities](7.1.3-inequalities.md)
- [7.1.4: Topology of ℝⁿ: Open and Closed Sets](7.1.4-topology.md)
- [7.1.5: Convergence of Sequences in ℝⁿ](7.1.5-convergence.md)
