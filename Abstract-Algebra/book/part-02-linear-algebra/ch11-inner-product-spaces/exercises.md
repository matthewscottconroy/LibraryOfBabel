# Chapter 11 — Exercises

## Important Figures

- **Jørgen Pedersen Gram (1850–1916)** — Gram–Schmidt orthogonalization process; Gram matrix
- **Erhard Schmidt (1876–1959)** — Gram–Schmidt in function spaces; Hilbert–Schmidt operators
- **David Hilbert (1862–1943)** — formalized infinite-dimensional inner product spaces (Hilbert spaces); spectral theory of symmetric operators
- **Carl Eckart (1902–1973) & Gale Young (1907–1998)** — singular value decomposition (1936); low-rank approximation

## References and Primary Sources

- **D. Hilbert, "Grundzüge einer allgemeinen Theorie der linearen Integralgleichungen" (1904–1910)** — spectral theory for integral operators; birth of Hilbert space theory
- **S. Axler, *Linear Algebra Done Right* (4th ed., Springer, 2024)**, Chs. 6–7 — spectral theorem without determinants
- **G. Golub & C. van Loan, *Matrix Computations* (4th ed., Johns Hopkins, 2013)** — SVD algorithms and applications

## Examples, Applications, and Thought Experiments

- **QR decomposition via Gram–Schmidt** — orthogonalize the columns of a matrix $A$; this produces$A = QR$ with$Q$ orthogonal and$R$ upper triangular; the QR algorithm iterates this to converge to eigenvalues; the inner product structure makes this work
- **Least squares** — given $Ax = b$ with no exact solution, minimize$\|Ax - b\|^2$; the solution is$\hat{x} = (A^T A)^{-1} A^T b$; geometrically,$A\hat{x}$ is the orthogonal projection of$b$ onto$\operatorname{col}(A)$; inner products provide the geometric language
- **Fourier series as orthogonal decomposition** — the functions $\{1, \cos(nx), \sin(nx)\}$ form an orthogonal family in$L^2([0, 2\pi])$; Fourier coefficients are inner products$\hat{f}(n) = \langle f, e^{inx}\rangle$; the spectral theorem for the self-adjoint operator$-d^2/dx^2$ gives the Fourier basis
- **The SVD in data science** — any real matrix $A$ decomposes as$A = U\Sigma V^T$ (SVD); the top$k$ singular values give the best rank-$k$ approximation; principal component analysis (PCA) is exactly the SVD of the centered data matrix; inner product geometry underlies modern data analysis

## Exercises

1. Apply the Gram–Schmidt process to the vectors $\mathbf{v}_1 = (1, 1, 0)$, $\mathbf{v}_2 = (1, 0, 1)$, $\mathbf{v}_3 = (0, 1, 1)$ in $\mathbb{R}^3$ with the standard inner product to produce an orthonormal basis $\{e_1, e_2, e_3\}$. Use this to write the $QR$ decomposition of the matrix $A$ whose columns are $\mathbf{v}_1, \mathbf{v}_2, \mathbf{v}_3$.

2. Let $W$ be the subspace of $\mathbb{R}^4$ spanned by $\mathbf{w}_1 = (1, 1, 0, 0)$ and $\mathbf{w}_2 = (0, 1, 1, 0)$. Find the orthogonal complement $W^\perp$, verify that $\mathbb{R}^4 = W \oplus W^\perp$, and compute the orthogonal projection of $\mathbf{b} = (1, 2, 3, 4)$ onto $W$.

3. Let $A = \begin{pmatrix} 2 & 1 \\ 1 & 2 \end{pmatrix}$ (a real symmetric matrix). Find an orthonormal basis of eigenvectors for $A$ and write $A = PDP^T$ where $P$ is orthogonal and $D$ is diagonal. Verify that the eigenvectors for distinct eigenvalues are orthogonal.

4. Prove the spectral theorem for $2 \times 2$ real symmetric matrices: if $A = A^T$, then $A$ has real eigenvalues and there exists an orthogonal matrix $P$ such that $P^T A P$ is diagonal. (Do not assume the result for larger matrices; give a direct proof using the characteristic polynomial and explicit computation of eigenvectors.)

5. Let $A$ be an $m \times n$ real matrix with singular value decomposition $A = U\Sigma V^T$. Prove the following from the SVD:
   (a) $\text{rank}(A)$ equals the number of nonzero singular values.
   (b) The columns of $U$ corresponding to zero singular values span the left null space of $A$.
   (c) $\|A\|_F^2 = \sigma_1^2 + \sigma_2^2 + \cdots + \sigma_r^2$ where $\|\cdot\|_F$ is the Frobenius norm.

6. Compute the singular value decomposition of $A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \\ 1 & 0 \end{pmatrix}$. Start by computing $A^T A$, find its eigenvalues and orthonormal eigenvectors to get $\Sigma$ and $V$, then compute $U$ from $U = AV\Sigma^{-1}$. What is the best rank-1 approximation to $A$?

7. A real quadratic form on $\mathbb{R}^3$ is given by $Q(\mathbf{x}) = 2x_1^2 + 4x_1 x_2 - 2x_2 x_3 + 3x_3^2$. Write $Q(\mathbf{x}) = \mathbf{x}^T A \mathbf{x}$ for a symmetric matrix $A$. Find the eigenvalues of $A$ and determine the signature of $Q$ (the number of positive and negative eigenvalues). Is $Q$ positive definite, positive semidefinite, or indefinite?

8. (Challenge) Let $T : V \to V$ be a normal operator on a finite-dimensional complex inner product space ($TT^* = T^*T$). Without assuming the spectral theorem, prove the following steps toward it: (a) if $T\mathbf{v} = \lambda \mathbf{v}$, then $T^*\mathbf{v} = \bar{\lambda}\mathbf{v}$; (b) eigenvectors of $T$ for distinct eigenvalues are orthogonal; (c) the eigenspace for each eigenvalue is invariant under $T^*$. Conclude that if $T$ has $n$ distinct eigenvalues then $T$ is unitarily diagonalizable.
