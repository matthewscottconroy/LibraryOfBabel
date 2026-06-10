# Section 9.4: Determinants

---

## Section Introduction

The **determinant** $\det A$ of a square matrix $A$ is a single number that encodes whether $A$ is invertible ($\det A\neq 0$) and, geometrically, by what factor $A$ multiplies volumes: if $P$ is a parallelepiped with edges given by $n$ vectors, the image of $P$ under $A$ has volume $|\det A|$ times the original. The determinant is negative when $A$ reverses orientation.

The determinant can be defined in several equivalent ways. Algebraically: $\det A = \sum_{\sigma\in S_n}\text{sgn}(\sigma)\prod_{i=1}^n a_{i,\sigma(i)}$, a sum over all permutations. Geometrically: $\det A$ is the unique multilinear, alternating function of the columns of $A$ normalized by $\det I = 1$. Recursively: the Laplace expansion expresses $\det A$ in terms of $(n-1)\times(n-1)$ determinants.

The key properties: $\det(AB) = (\det A)(\det B)$; $\det(A^T) = \det A$; $\det A\neq 0$ iff $A$ is invertible; adding a multiple of one row to another leaves $\det A$ unchanged. The multiplicativity $\det(AB) = \det(A)\det(B)$ means that the determinant is a **group homomorphism** from the group of invertible $n\times n$ matrices $GL_n(\mathbb{R})$ to the multiplicative group $\mathbb{R}^*$.

In differential geometry, the determinant of the metric tensor $g = \det(g_{\mu\nu})$ appears in the volume element $\sqrt{|g|}\,d^nx$ — the coordinate-invariant measure on a Riemannian or pseudo-Riemannian manifold. This is the factor that makes the Einstein-Hilbert action $\int R\sqrt{-g}\,d^4x$ a scalar. The determinant also appears in the transformation law of tensor densities: a quantity that transforms as $\sqrt{-g}$ times a tensor is called a tensor density.

---

## Subsections

- [9.4.1: Definition and Basic Properties](9.4.1-definition.md)
- [9.4.2: Cofactor Expansion (Laplace)](9.4.2-cofactor.md)
- [9.4.3: Geometric Interpretation: Signed Volume](9.4.3-geometric.md)
- [9.4.4: Cramer's Rule and the Inverse Formula](9.4.4-cramers.md)
- [9.4.5: Determinants and the Metric Tensor](9.4.5-metric.md)
