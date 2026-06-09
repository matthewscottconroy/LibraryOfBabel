# LU and QR Decomposition

Matrix decompositions express a matrix as a product of simpler matrices, making computations more efficient and revealing structure. The LU decomposition factors a square matrix into lower and upper triangular factors, enabling fast linear system solving. The QR decomposition factors a matrix into an orthogonal matrix and an upper triangular matrix, enabling stable computation and the foundation for eigenvalue algorithms. Both are workhorses of numerical linear algebra.

## LU Decomposition

**Definition.** An $n\times n$ matrix $A$ has an **LU decomposition** if $A = LU$ where $L$ is lower triangular with $1$'s on the diagonal (unit lower triangular) and $U$ is upper triangular.

**Construction via Gaussian elimination.** Forward elimination transforms $A$ into an upper triangular matrix $U$ by a sequence of row operations (adding multiples of pivot rows to lower rows). Each such operation corresponds to multiplication by an elementary unit lower triangular matrix $E_k$. The product of these elimination matrices, inverted, gives $L$:
$$E_m \cdots E_2 E_1 A = U \implies A = E_1^{-1}E_2^{-1}\cdots E_m^{-1}U = LU.$$

The key observation: the inverses $E_k^{-1}$ of unit lower triangular matrices are also unit lower triangular, and their product $L = \prod E_k^{-1}$ has a particularly simple form: the entries of $L$ are exactly the multipliers used during elimination, placed in the corresponding positions.

**Example.** $A = \begin{pmatrix}2&3\\4&7\end{pmatrix}$.

$R_2 \leftarrow R_2 - 2R_1$ (multiplier $\ell_{21} = 2$) gives $U = \begin{pmatrix}2&3\\0&1\end{pmatrix}$. Then $L = \begin{pmatrix}1&0\\2&1\end{pmatrix}$.

Verify: $LU = \begin{pmatrix}1&0\\2&1\end{pmatrix}\begin{pmatrix}2&3\\0&1\end{pmatrix} = \begin{pmatrix}2&3\\4&7\end{pmatrix} = A$. $\checkmark$

**LU with partial pivoting (PLU).** If a pivot is zero or small (causing numerical instability), rows must be swapped (permuted). With permutations, the decomposition becomes $PA = LU$ where $P$ is a permutation matrix.

**Solving $Ax = b$ via LU.** Given $A = LU$ (or $PA = LU$):
1. Solve $Ly = b$ (or $Ly = Pb$) by forward substitution: $O(n^2)$.
2. Solve $Ux = y$ by backward substitution: $O(n^2)$.

The LU decomposition costs $O(n^3)$ once; subsequent solves with different $b$ cost only $O(n^2)$ each. This makes LU invaluable when many systems with the same $A$ but different right-hand sides must be solved — exactly the situation in implicit ODE methods.

## QR Decomposition

**Definition.** A matrix $A \in M_{m\times n}$ (with $m \geq n$) has a **QR decomposition** $A = QR$ where $Q \in M_{m\times n}$ has orthonormal columns ($Q^TQ = I_n$) and $R \in M_{n\times n}$ is upper triangular with positive diagonal entries.

If $m = n$, $Q$ is orthogonal: $Q^TQ = QQ^T = I$, so $Q^{-1} = Q^T$.

**Construction via Gram-Schmidt.** Apply the Gram-Schmidt process (Chapter 5) to the columns of $A$: $a_1, a_2, \ldots, a_n$. Produce orthonormal vectors $q_1, \ldots, q_n$ where $\text{span}\{q_1,\ldots,q_k\} = \text{span}\{a_1,\ldots,a_k\}$ for each $k$. The matrix $Q = [q_1|\cdots|q_n]$ and $r_{jk} = q_j^T a_k$ gives $R$:
$$a_k = \sum_{j=1}^k r_{jk} q_j \implies A = QR.$$

**Alternative construction via Householder reflections.** Numerically more stable than Gram-Schmidt; the standard method in practice.

**Example.** $A = \begin{pmatrix}1&1\\1&0\\0&1\end{pmatrix}$.

Column $a_1 = (1,1,0)^T$: $q_1 = a_1/\|a_1\| = (1,1,0)^T/\sqrt{2}$. $r_{11} = \|a_1\| = \sqrt{2}$.

Column $a_2 = (1,0,1)^T$: project out $q_1$ component: $\tilde{q}_2 = a_2 - (q_1^Ta_2)q_1 = (1,0,1)^T - (1/\sqrt{2})(1,1,0)^T/\sqrt{2} = (1/2, -1/2, 1)^T$. $\|\tilde{q}_2\| = \sqrt{3/2}$. $q_2 = \tilde{q}_2/\|\tilde{q}_2\|$.

## Applications of QR

**Least squares.** The least-squares solution to the overdetermined system $Ax \approx b$ minimizes $\|Ax-b\|^2$. With $A = QR$: $\|Ax-b\|^2 = \|QRx-b\|^2 = \|Rx - Q^Tb\|^2 + \|b - QQ^Tb\|^2$ (since $Q$ preserves norms). The minimum is achieved by solving $Rx = Q^Tb$, a triangular system.

**Eigenvalue computation (QR algorithm).** The QR algorithm iterates: given $A_0 = A$, decompose $A_k = Q_kR_k$ and set $A_{k+1} = R_kQ_k$. The sequence $A_k$ converges (under appropriate conditions) to a triangular or quasi-triangular matrix, revealing the eigenvalues. This is the standard algorithm for computing eigenvalues of dense matrices.

## Comparison

| Property | LU | QR |
|---|---|---|
| Applicable to | Square matrices | $m \geq n$ |
| Cost | $O(n^3)$ | $O(mn^2)$ |
| Numerically stable | With pivoting | Always (Householder) |
| Use case | Solving $Ax=b$ | Least squares, eigenvalues |
| Key property of factors | Triangular | $Q^TQ = I$ |

## Connection to ODEs

Stiff ODE problems (where solutions have components with very different time scales) require implicit methods that solve large linear systems at each time step. LU decomposition with partial pivoting is the standard solver for these systems. When the Jacobian of the right-hand side $f(t,y)$ is needed for implicit methods (e.g., Newton-Raphson within each step), both LU and QR decompositions appear. For eigenvalue-based stability analysis of linear systems $\mathbf{x}' = Ax$, the QR algorithm is the practical method for finding the eigenvalues of $A$.
