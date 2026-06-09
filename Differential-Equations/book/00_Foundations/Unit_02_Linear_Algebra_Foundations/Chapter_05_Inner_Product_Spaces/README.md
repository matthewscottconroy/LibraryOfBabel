# Chapter 05: Inner Product Spaces

Linear algebra, as developed in the previous chapters, is purely algebraic — it concerns addition and scalar multiplication, with no notion of length, angle, or perpendicularity. Inner product spaces add geometry to the algebraic structure by defining an inner product: a bilinear (or sesquilinear) map that produces a scalar from two vectors, generalizing the dot product of Euclidean geometry. This geometric structure enables the definition of orthogonality, projection, and the Spectral Theorem for symmetric matrices.

## What This Chapter Covers

**Section 1: Dot Product and Norms** defines the standard inner product on $\mathbb{R}^n$ and the resulting norm, reviews the Cauchy-Schwarz and triangle inequalities, and defines the general concept of an inner product space. The inner product on $L^2([a,b])$ — $\langle f, g\rangle = \int_a^b f(x)g(x)\,dx$ — is the function-space analog and is the inner product underlying Fourier series.

**Section 2: Gram-Schmidt Process** describes the constructive procedure for converting any linearly independent set into an orthonormal set spanning the same subspace. This procedure is fundamental both theoretically (proving existence of orthonormal bases) and computationally (QR decomposition).

**Section 3: Orthogonal Complements and Projections** develops the projection of a vector onto a subspace: the component of $v$ in $W$ and the component orthogonal to $W$. The projection formula and the best-approximation theorem (projection minimizes distance to $W$) are proved. These tools are the basis for least-squares methods and for understanding the image and null space of a matrix geometrically.

**Section 4: Symmetric Matrices and the Spectral Theorem** proves the central result: every real symmetric matrix is orthogonally diagonalizable — it has real eigenvalues and an orthonormal basis of eigenvectors. The Spectral Theorem provides a decomposition $A = Q\Lambda Q^T$ where $Q$ is orthogonal and $\Lambda$ is diagonal with real entries.

**Section 5: Singular Value Decomposition** generalizes the Spectral Theorem to non-square matrices. Every $m\times n$ matrix $A$ has a decomposition $A = U\Sigma V^T$ where $U, V$ are orthogonal and $\Sigma$ is diagonal with nonnegative entries (the singular values). SVD is the most general and useful matrix factorization.

## Connection to Differential Equations

The Sturm-Liouville theory of self-adjoint boundary value problems is the ODE analog of the Spectral Theorem: the eigenvalues are real and the eigenfunctions are orthogonal with respect to a suitable inner product. Fourier series are the expansion of a function in the orthonormal basis of eigenfunctions — the function-space analog of $A = Q\Lambda Q^T$. The SVD appears in data-driven methods for identifying ODE models from observations.
