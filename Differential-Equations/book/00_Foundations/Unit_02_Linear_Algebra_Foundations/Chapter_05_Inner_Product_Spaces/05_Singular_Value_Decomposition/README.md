# Singular Value Decomposition

The Spectral Theorem gives an orthogonal diagonalization $A = Q\Lambda Q^T$ for symmetric matrices. The Singular Value Decomposition (SVD) generalizes this to all matrices — rectangular or square, symmetric or not. Every $m\times n$ matrix $A$ factors as $A = U\Sigma V^T$ where $U$ and $V$ are orthogonal matrices and $\Sigma$ is diagonal with nonnegative entries. The SVD is the most fundamental and informative matrix decomposition, providing the cleanest description of the geometry of any linear map.

## Statement of the SVD Theorem

**Theorem (SVD).** Let $A \in M_{m\times n}(\mathbb{R})$ with $m \geq n$. Then $A = U\Sigma V^T$ where:
- $U \in M_{m\times m}$ is orthogonal ($U^TU = UU^T = I_m$).
- $V \in M_{n\times n}$ is orthogonal ($V^TV = VV^T = I_n$).
- $\Sigma \in M_{m\times n}$ is "diagonal": $\Sigma_{ij} = \sigma_i \delta_{ij}$ with $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_r > 0 = \sigma_{r+1} = \cdots = \sigma_n$ where $r = \text{rank}(A)$.

The values $\sigma_1 \geq \cdots \geq \sigma_r > 0$ are called the **singular values** of $A$. The columns $u_i$ of $U$ are the **left singular vectors**; the columns $v_i$ of $V$ are the **right singular vectors**.

## Proof of Existence

*Proof.* Consider the symmetric positive semidefinite matrix $A^TA \in M_{n\times n}$. By the Spectral Theorem, $A^TA = V\Lambda V^T$ where $V$ is orthogonal and $\Lambda = \text{diag}(\lambda_1,\ldots,\lambda_n)$ with $\lambda_1 \geq \cdots \geq \lambda_n \geq 0$ (all eigenvalues of $A^TA$ are nonnegative).

Set $\sigma_i = \sqrt{\lambda_i}$. For $i \leq r$ (where $r$ is the rank), define $u_i = Av_i/\sigma_i$. Then:
$$u_i^Tu_j = \frac{v_i^TA^TAv_j}{\sigma_i\sigma_j} = \frac{v_i^T(\lambda_j v_j)}{\sigma_i\sigma_j} = \frac{\lambda_j}{\sigma_i\sigma_j}\delta_{ij} = \frac{\sigma_j^2}{\sigma_i\sigma_j}\delta_{ij} = \delta_{ij}.$$

So $u_1,\ldots,u_r$ are orthonormal. Extend to an orthonormal basis $u_1,\ldots,u_m$ of $\mathbb{R}^m$. Then $U = [u_1|\cdots|u_m]$ and $V = [v_1|\cdots|v_n]$ satisfy $AV = U\Sigma$ (check: $Av_i = \sigma_i u_i$ for $i\leq r$ and $Av_i = 0$ for $i > r$), hence $A = U\Sigma V^T$. $\square$

## Geometric Interpretation

The SVD says every linear map $\mathbb{R}^n \to \mathbb{R}^m$ can be decomposed as:
1. An orthogonal change of basis in $\mathbb{R}^n$ (multiply by $V^T$).
2. A scaling along coordinate axes (multiply by $\Sigma$).
3. An orthogonal change of basis in $\mathbb{R}^m$ (multiply by $U$).

Every linear map is, in the right bases, a coordinate-wise scaling followed by an embedding (or projection). The singular values are the "stretching factors" in each direction.

## Relation to Fundamental Subspaces

The SVD reveals all four fundamental subspaces simultaneously:
- $\text{col}(A) = \text{span}\{u_1,\ldots,u_r\}$ (first $r$ columns of $U$).
- $\ker(A) = \text{span}\{v_{r+1},\ldots,v_n\}$ (last $n-r$ columns of $V$).
- $\text{row}(A) = \text{span}\{v_1,\ldots,v_r\}$ (first $r$ columns of $V$).
- $\ker(A^T) = \text{span}\{u_{r+1},\ldots,u_m\}$ (last $m-r$ columns of $U$).

Moreover: $Av_i = \sigma_i u_i$ for $i \leq r$ — the right singular vectors are mapped to the left singular vectors, scaled by $\sigma_i$.

## Low-Rank Approximation

The **Eckart-Young Theorem** states that the best rank-$k$ approximation to $A$ (in the Frobenius or operator norm) is:
$$A_k = \sum_{i=1}^k \sigma_i u_i v_i^T = U\Sigma_k V^T,$$
where $\Sigma_k$ has $\sigma_1,\ldots,\sigma_k$ on the diagonal and zeros elsewhere. The approximation error is $\|A - A_k\|_2 = \sigma_{k+1}$.

This is the mathematical foundation for principal component analysis (PCA), image compression, and data-driven reduced-order models for ODEs.

## The Pseudoinverse

The **Moore-Penrose pseudoinverse** of $A$ is
$$A^+ = V\Sigma^+U^T,$$
where $\Sigma^+$ has $1/\sigma_i$ in place of $\sigma_i$ (for $i \leq r$) and zeros elsewhere. The pseudoinverse gives the minimum-norm least-squares solution to $Ax = b$: $\hat{x} = A^+b$ minimizes $\|Ax-b\|$ and, among all such minimizers, has the smallest norm.

## Relation to the Spectral Theorem

The SVD is the natural generalization of the Spectral Theorem:
- For symmetric $A = Q\Lambda Q^T$ with $\Lambda = \text{diag}(\lambda_i)$, the SVD has $\sigma_i = |\lambda_i|$, $v_i = q_i$, and $u_i = \text{sign}(\lambda_i)q_i$.
- The Spectral Theorem is the special case of SVD when $m = n$ and $A$ is symmetric.

## Applications in Differential Equations

**Reduced-order models.** A high-dimensional ODE system $\dot{x} = Ax + Bu$ (with state $x \in \mathbb{R}^n$, $n$ large) can be projected onto the dominant singular vector subspace of the observability or controllability matrix, giving a low-dimensional system that captures the dominant dynamics.

**Data-driven identification.** Given time-series data from an ODE, the Dynamic Mode Decomposition (DMD) algorithm uses SVD to identify a best-fit linear ODE $\dot{X} \approx AX$ from data matrices. The SVD provides the numerical stability and rank-revealing properties needed for this.

**Condition number.** The condition number of $A$ is $\kappa(A) = \sigma_1/\sigma_r$ (ratio of largest to smallest singular value). It measures how sensitive the solution of $Ax = b$ is to perturbations in $b$: $\|\delta x\|/\|x\| \leq \kappa(A)\|\delta b\|/\|b\|$. For implicit ODE solvers, the condition number of the system matrix at each step determines the numerical error amplification.

## Common Pitfalls

**Confusing singular values with eigenvalues.** Singular values are always nonnegative; eigenvalues can be complex or negative. For symmetric positive definite $A$: $\sigma_i = \lambda_i$. For general $A$: $\sigma_i = \sqrt{\lambda_i(A^TA)}$.

**Confusing $U$ and $V$.** $U$ has the left singular vectors (in $\mathbb{R}^m$), $V$ has the right singular vectors (in $\mathbb{R}^n$). The "thin SVD" uses only the first $r$ columns of $U$ and $V$, but the "full SVD" requires all $m$ columns of $U$.

**Order of factors.** $A = U\Sigma V^T$, not $U\Sigma V$ or $V\Sigma U^T$. The transpose is on $V$, not $U$.
