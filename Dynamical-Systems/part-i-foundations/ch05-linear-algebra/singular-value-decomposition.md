# 5.3 Singular Value Decomposition

The spectral theorem applies to normal matrices. For non-square or non-normal matrices, the right decomposition is the singular value decomposition (SVD). It's the most practically important decomposition in applied linear algebra, and it's the foundation for data-driven approaches to dynamical systems.

**Theorem 5.3.1 (SVD).** Every matrix $A \in M_{m \times n}(\mathbb{R})$ can be written as $A = U\Sigma V^T$ where $U \in M_{m \times m}$ and $V \in M_{n \times n}$ are orthogonal, and $\Sigma \in M_{m \times n}$ is diagonal with nonneg entries $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_{\min(m,n)} \geq 0$ (the *singular values*).

*(proof)* $A^TA$ is positive semi-definite, so $A^TA = V\Lambda V^T$ (spectral theorem). Set $\sigma_i = \sqrt{\lambda_i}$ and $u_i = Av_i/\sigma_i$ for $\sigma_i > 0$. Extend to an ONB to get $U$.

The singular values $\sigma_i$ are the "generalized eigenvalues" of $A$ — they measure how much $A$ stretches vectors in each of the principal directions. The columns of $U$ are the left singular vectors (the output directions), and the columns of $V$ are the right singular vectors (the input directions).

The geometric interpretation: $A$ takes the orthonormal basis $\{v_i\}$ to the scaled orthonormal set $\{\sigma_i u_i\}$. Every linear map, regardless of its symmetry properties, has this "stretch-rotate" decomposition.

**Low-Rank Approximation:**

**Theorem 5.3.2 (Eckart-Young-Mirsky).** The best rank-$k$ approximation to $A$ in the Frobenius norm (or operator norm) is $A_k = U\Sigma_k V^T$ where $\Sigma_k = \text{diag}(\sigma_1, \ldots, \sigma_k, 0, \ldots, 0)$.

The Eckart-Young theorem says: throw away all but the largest $k$ singular values, and you get the best rank-$k$ approximation to $A$. This is optimal compression: if you have to represent $A$ with a rank-$k$ matrix, this is the right choice.

The error in the approximation is $\|A - A_k\| = \sigma_{k+1}$ (operator norm) or $\|A - A_k\|_F = \sqrt{\sum_{i>k} \sigma_i^2}$ (Frobenius norm). If the singular values decay quickly, the rank-$k$ approximation is very accurate for small $k$ — the matrix has low *intrinsic rank*.

**Application in Data-Driven Dynamics.** Given time-series data from a dynamical system, *Dynamic Mode Decomposition (DMD)* uses the SVD to find the best linear approximation to the dynamics. Here's the setup:

You have data matrices $X = [x_1 | x_2 | \cdots | x_{n-1}]$ and $X' = [x_2 | x_3 | \cdots | x_n]$ (the same sequence shifted by one step). DMD finds the best-fit linear map $A$ with $X' \approx AX$, using the SVD of $X$ to avoid overfitting. The DMD modes are the eigenvectors of this best-fit linear map, and they reveal the dominant spatial patterns and their temporal evolution.

DMD is one of the primary tools for data-driven analysis of fluid flows, plasma dynamics, and complex systems where the governing equations may not be known. The SVD is what makes the computation numerically stable.
