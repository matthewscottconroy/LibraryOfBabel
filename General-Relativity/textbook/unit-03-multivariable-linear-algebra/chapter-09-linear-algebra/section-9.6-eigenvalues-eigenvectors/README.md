# Section 9.6: Eigenvalues and Eigenvectors

---

## Section Introduction

An **eigenvector** of a linear map $T: V\to V$ is a nonzero vector $\mathbf{v}$ that $T$ maps to a scalar multiple of itself: $T(\mathbf{v}) = \lambda\mathbf{v}$. The scalar $\lambda$ is the corresponding **eigenvalue**. Eigenvectors are the "natural directions" of $T$ — the directions that are preserved under $T$ up to stretching or compression. In the eigenvector basis, $T$ is represented by a diagonal matrix (if the eigenvectors span the space), making all computations trivial.

The eigenvalues are the roots of the **characteristic polynomial** $\det(A - \lambda I) = 0$. For an $n\times n$ matrix over $\mathbb{C}$, there are exactly $n$ eigenvalues (counted with multiplicity, by the Fundamental Theorem of Algebra). Over $\mathbb{R}$, some eigenvalues may be complex — rotation matrices, for instance, have complex eigenvalues.

The **spectral theorem** for symmetric (or Hermitian, in the complex case) matrices is one of the crowning results of linear algebra: every real symmetric matrix has all real eigenvalues and an orthonormal basis of eigenvectors. In matrix form: $A = QDQ^T$ where $D$ is diagonal and $Q$ is orthogonal. This is the **eigendecomposition** or **diagonalization** of $A$.

The spectral theorem has profound physical implications. The observable quantities in quantum mechanics are Hermitian operators (Hamiltonians, position, momentum), and the spectral theorem guarantees they have real eigenvalues (measured values) and orthogonal eigenstates (distinct measurement outcomes are mutually exclusive). In GR, the Ricci tensor and Weyl tensor are symmetric tensors; their eigenvalues and eigenvectors encode the local geometry. The Petrov classification of gravitational fields (Chapter 54) is essentially a spectral decomposition of the Weyl tensor.

---

## Subsections

- [9.6.1: Eigenvalues, Eigenvectors, and the Characteristic Polynomial](9.6.1-definition.md)
- [9.6.2: Diagonalization](9.6.2-diagonalization.md)
- [9.6.3: The Spectral Theorem for Symmetric Matrices](9.6.3-spectral.md)
- [9.6.4: Jordan Normal Form](9.6.4-jordan.md)
- [9.6.5: Eigenvalues in Physics](9.6.5-physics.md)
