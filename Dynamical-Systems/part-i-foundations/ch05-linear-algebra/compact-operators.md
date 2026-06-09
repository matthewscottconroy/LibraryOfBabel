# 5.5 Compact Operators

Compact operators are the "almost finite-dimensional" operators on Hilbert spaces — they're the ones whose spectral theory looks most like the finite-dimensional case. Most of the operators that arise in analysis (integral operators, operators with smooth kernels) are compact, and their spectral theory is completely analogous to finite-dimensional diagonalization.

**Definition 5.5.1.** $T: H \to K$ is *compact* if it maps bounded sets to relatively compact sets (sets with compact closure). Equivalently: every bounded sequence $(x_n)$ has a subsequence $(x_{n_k})$ with $Tx_{n_k}$ convergent.

Compact operators are those that "compress" the infinite-dimensional space to something "essentially finite-dimensional." A bounded sequence, which might not converge, gets mapped to a sequence that has a convergent subsequence. This is exactly the condition that makes spectral theory work.

Here's the range of compact operators you'll encounter:

**Examples 5.5.2.**
- Every finite-rank operator (range is finite-dimensional) is compact. These are the true "finite-dimensional" operators.
- The Hilbert-Schmidt operators: $T$ with $\sum_{i,j} |\langle Te_i, e_j \rangle|^2 < \infty$ for any orthonormal basis.
- Integral operators $T_k f(x) = \int k(x,y) f(y)\,dy$ with square-integrable kernel $k \in L^2(\Omega \times \Omega)$.

The Hilbert-Schmidt class is the most natural infinite-dimensional generalization of matrices: just as matrices are characterized by their entry $(i,j)$ being $\langle Ae_j, e_i \rangle$, Hilbert-Schmidt operators have $\sum_{i,j} |\langle Te_i, e_j \rangle|^2 < \infty$.

The spectral theorem for compact self-adjoint operators is the clean infinite-dimensional version of the finite-dimensional spectral theorem:

**Theorem 5.5.3 (Spectral Theorem for Compact Self-Adjoint Operators).** Let $T: H \to H$ be compact and self-adjoint. Then:
1. $\sigma(T) \subseteq \mathbb{R}$ and $\sigma(T) \setminus \{0\}$ consists only of eigenvalues.
2. The eigenvalues form a (possibly finite or empty) sequence $\lambda_1, \lambda_2, \ldots \to 0$.
3. Each nonzero eigenspace is finite-dimensional.
4. The eigenvectors $\{e_i\}$ form an orthonormal basis for $H$ (if $T \neq 0$).

What this is really saying: for compact self-adjoint operators, the spectrum is discrete (except possibly at 0), eigenvalues accumulate only at 0, and the eigenvectors give an orthonormal basis. It's exactly the finite-dimensional situation, except that there are countably many eigenvalues rather than finitely many, and they must accumulate at 0.

This theorem has a beautiful consequence: you can write $T = \sum_i \lambda_i \langle \cdot, e_i \rangle e_i$ — the operator is the "sum of rank-1 projections," each weighted by its eigenvalue. This is the infinite-dimensional analog of diagonalization.

**Application — Perron-Frobenius for Operators.** The *transfer operator* (Ruelle-Perron-Frobenius operator) $\mathcal{L}: L^2(X, \mu) \to L^2(X, \mu)$ of an expanding map satisfies, under suitable conditions, the spectral theorem for compact operators. The dominant eigenvalue is $1$ (with corresponding eigenfunction the invariant density), and the spectral gap below $1$ controls mixing rates.

This is the bridge between the abstract spectral theory of this chapter and the concrete dynamical properties of expanding maps, hyperbolic maps, and Markov chains in Part II.
