# 5.2 The Spectral Theorem

For normal operators — a large class that includes self-adjoint, unitary, and skew-Hermitian operators — the Jordan form collapses to a diagonal, and the eigenvectors form an orthonormal basis. This is the spectral theorem, and it's the foundation for all spectral analysis in dynamics.

## 5.2.1 Normal Operators

**Definition 5.2.1.** A matrix $A \in M_n(\mathbb{C})$ is *normal* if $A^*A = AA^*$ (where $A^* = \bar{A}^T$). Important special cases:
- *Self-adjoint (Hermitian)*: $A = A^*$
- *Unitary*: $A^*A = AA^* = I$
- *Anti-Hermitian*: $A^* = -A$ (infinitesimal generators of unitary groups)
- *Skew-symmetric* real matrices (anti-Hermitian over $\mathbb{R}$)

Normal operators are exactly those that can be "simultaneously block-diagonalized" with their adjoint. Hermitian matrices (symmetric in the real case) are the natural setting for quantum observables and for symmetric linear maps. Unitary matrices are the natural setting for symmetries, reversible maps, and Koopman operators of measure-preserving transformations.

**Theorem 5.2.2 (Spectral Theorem for Normal Matrices).** $A$ is normal if and only if $A$ is unitarily diagonalizable: there exists a unitary $U$ with $U^*AU = D = \text{diag}(\lambda_1, \ldots, \lambda_n)$.

- If $A$ is Hermitian: all $\lambda_i \in \mathbb{R}$.
- If $A$ is unitary: all $|\lambda_i| = 1$.
- If $A$ is positive semi-definite ($\langle Av, v \rangle \geq 0$ for all $v$): all $\lambda_i \geq 0$.

**Proof (for Hermitian matrices):** Induction on $n$. Any Hermitian matrix has at least one real eigenvalue $\lambda$ with unit eigenvector $v$. The orthogonal complement $v^\perp$ is $A$-invariant (since $A$ is Hermitian), so restrict to $v^\perp$ and apply induction.

The theorem says: for normal matrices, there's no Jordan form complexity — every normal matrix is diagonalizable, and the eigenvectors are orthonormal. This is what makes normal operators the "nice" class for spectral analysis.

## 5.2.2 Spectral Theorem for Bounded Self-Adjoint Operators

On infinite-dimensional Hilbert spaces, the spectral theorem requires a more sophisticated framework — eigenvalues might not exist (continuous spectrum), and the diagonalization becomes an integral rather than a sum.

**Definition 5.2.3.** Let $H$ be a Hilbert space. A *projection-valued measure (PVM)* on $(\mathbb{R}, \mathcal{B}(\mathbb{R}))$ is a map $E: \mathcal{B}(\mathbb{R}) \to \mathcal{B}(H)$ where each $E(B)$ is an orthogonal projection, $E(\mathbb{R}) = I$, and $E(B \cap C) = E(B)E(C)$ and $E(B \cup C) = E(B) + E(C)$ for disjoint $B, C$.

A PVM assigns a projection to each Borel set in a way that's consistent: the projection onto a set is the "operator-valued indicator function" of that set.

**Theorem 5.2.4 (Spectral Theorem — Bounded Self-Adjoint Operators).** Let $A: H \to H$ be a bounded self-adjoint operator. There exists a unique PVM $E$ on $\sigma(A) \subseteq \mathbb{R}$ such that
$$A = \int_{\sigma(A)} \lambda\,dE(\lambda).$$

More generally, for any continuous $g: \sigma(A) \to \mathbb{C}$:
$$g(A) = \int_{\sigma(A)} g(\lambda)\,dE(\lambda).$$

What this is really saying: even for infinite-dimensional operators, there's a way to "diagonalize" a self-adjoint operator, but the eigenvalues might be distributed continuously over $\sigma(A)$, and the "eigenvectors" might not be normalizable elements of $H$. The PVM $E$ is the infinite-dimensional analog of the orthogonal projections onto eigenspaces.

**Application in Dynamics.** The Koopman operator $U_f: L^2(\mu) \to L^2(\mu)$ defined by $U_f(\varphi) = \varphi \circ f$ for a measure-preserving map $f$ is *unitary* (if $f$ is invertible). Its spectral structure reveals the ergodic properties of $f$:
- $1 \in \sigma(U_f)$ always (since $U_f(1) = 1$)
- The eigenvalues of $U_f$ are the *dynamical eigenvalues* of $f$ — the frequencies at which the system oscillates
- $f$ is ergodic iff $1$ is a simple eigenvalue of $U_f$ — meaning the only eigenfunctions for eigenvalue 1 are constants
- $f$ is weakly mixing iff the only eigenvalue of $U_f$ is $1$ — no periodic oscillations at all

This connection between ergodicity and the spectral theory of the Koopman operator is one of the central results of ergodic theory, and it's developed in full in Chapters 6 and 7.
