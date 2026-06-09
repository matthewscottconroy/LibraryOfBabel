# 5.1 Jordan Canonical Form

Every matrix over $\mathbb{C}$ is similar to a matrix in Jordan canonical form — a block-diagonal matrix where each block looks almost diagonal, with the eigenvalue on the diagonal and 1's just above it. The Jordan form is the complete classification of matrices up to similarity, and it reveals the precise structure of the matrix exponential.

## 5.1.1 Generalized Eigenspaces

When a matrix is not diagonalizable — when eigenvalues have algebraic multiplicity greater than geometric multiplicity — the eigenvectors don't span the space. Generalized eigenvectors fill the gap:

**Definition 5.1.1.** Let $A \in M_n(\mathbb{C})$. For an eigenvalue $\lambda$ of $A$, the *generalized eigenspace* is
$$V_\lambda = \ker(A - \lambda I)^n = \{v : (A - \lambda I)^k v = 0 \text{ for some } k\}.$$

Every vector in $V_\lambda$ eventually gets killed by $(A - \lambda I)$, after enough applications. Actual eigenvectors are killed after one application; generalized eigenvectors require more.

**Theorem 5.1.2 (Primary Decomposition).** $\mathbb{C}^n = \bigoplus_{\lambda \in \sigma(A)} V_\lambda$ where $\sigma(A)$ is the spectrum of $A$. Each $V_\lambda$ is $A$-invariant and $A|_{V_\lambda} = \lambda I + N_\lambda$ where $N_\lambda$ is nilpotent.

The primary decomposition says: $\mathbb{C}^n$ breaks into a direct sum of generalized eigenspaces, one for each distinct eigenvalue. On each piece, the matrix $A$ looks like a scalar $\lambda$ plus a nilpotent perturbation. This nilpotent part is what the Jordan form captures.

## 5.1.2 Jordan Blocks and the Jordan Normal Form

**Definition 5.1.3.** A *Jordan block* of size $k$ for eigenvalue $\lambda$ is the $k \times k$ matrix
$$J_k(\lambda) = \begin{pmatrix} \lambda & 1 & 0 & \cdots & 0 \\ 0 & \lambda & 1 & \cdots & 0 \\ \vdots & & \ddots & \ddots & \vdots \\ 0 & \cdots & 0 & \lambda & 1 \\ 0 & \cdots & 0 & 0 & \lambda \end{pmatrix}.$$

**Theorem 5.1.4 (Jordan Canonical Form).** Every $A \in M_n(\mathbb{C})$ is similar to a block-diagonal matrix
$$J = \text{diag}(J_{k_1}(\lambda_1), J_{k_2}(\lambda_2), \ldots, J_{k_r}(\lambda_r))$$
where the $\lambda_i$ are eigenvalues (not necessarily distinct) and $\sum k_i = n$. This form is unique up to reordering of blocks.

The Jordan form is the complete invariant for similarity: two matrices are similar over $\mathbb{C}$ if and only if they have the same Jordan form. It captures both the eigenvalues (what $\lambda_i$ appear) and the failure of diagonalizability (the sizes $k_i$ of the blocks for each eigenvalue).

**Example 5.1.5.** If $A$ has distinct eigenvalues, all Jordan blocks are $1 \times 1$ and $J = \text{diag}(\lambda_1, \ldots, \lambda_n)$ — the familiar diagonalization. The Jordan form captures the failure of diagonalizability when eigenvalues repeat.

## 5.1.3 The Matrix Exponential Revisited

The Jordan form gives an explicit formula for the matrix exponential. For a Jordan block $J_k(\lambda)$:
$$e^{tJ_k(\lambda)} = e^{\lambda t} \begin{pmatrix} 1 & t & t^2/2! & \cdots & t^{k-1}/(k-1)! \\ 0 & 1 & t & \cdots & t^{k-2}/(k-2)! \\ \vdots & & \ddots & & \vdots \\ 0 & 0 & \cdots & 1 & t \\ 0 & 0 & \cdots & 0 & 1 \end{pmatrix}.$$

This shows something crucial: the growth rate of $e^{tA}$ is determined by the real parts of eigenvalues, while Jordan blocks of size $> 1$ contribute *polynomial* growth at the same exponential rate. An eigenvalue $\lambda$ with $\text{Re}(\lambda) = 0$ in a size-2 Jordan block gives solutions that grow linearly in $t$ — polynomially, not exponentially.

This is why the stability classification (Theorem 4.3.2) has a subtlety on the imaginary axis: it's not just whether eigenvalues have zero real part, but whether the Jordan blocks have size 1.

**Example 5.1.6.** If $A = \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix}$ (Jordan block with $\lambda = 0$), then $e^{tA} = \begin{pmatrix} 1 & t \\ 0 & 1 \end{pmatrix}$. Solutions grow linearly even though the eigenvalue is zero — this is exactly why zero-eigenvalue fixed points require center manifold analysis rather than linearization.

The Jordan form is, in some sense, the most complete "spectral fingerprint" of a matrix. For dynamical systems purposes, though, the truly important spectral information — the one that determines exponential growth rates and mixing — is the set of eigenvalues and their real parts. The Jordan block sizes are a correction that matters only on the boundary between stability and instability.
