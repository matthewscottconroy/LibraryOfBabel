# Diagonalization

A diagonal matrix is the simplest possible matrix — its powers are immediate, its exponential is trivial, and its action on $\mathbb{R}^n$ is transparent (it scales each coordinate independently). Diagonalization asks: when is a matrix similar to a diagonal matrix, and how is that diagonal matrix found? The answer connects directly to eigenvectors: a matrix is diagonalizable precisely when it has enough linearly independent eigenvectors to form a basis.

## Definition and Criterion

**Definition.** An $n\times n$ matrix $A$ is **diagonalizable** if there exists an invertible matrix $P$ and a diagonal matrix $D$ such that $A = PDP^{-1}$.

Equivalently, $P^{-1}AP = D$, meaning $A$ and $D$ are similar.

**Theorem.** $A$ is diagonalizable iff $A$ has $n$ linearly independent eigenvectors.

If $A = PDP^{-1}$ where $D = \text{diag}(\lambda_1, \ldots, \lambda_n)$, then the columns of $P$ are the eigenvectors and the diagonal entries of $D$ are the corresponding eigenvalues: $Ap_i = \lambda_i p_i$ where $p_i$ is the $i$-th column of $P$.

*Proof.* $AP = PD$ iff $(Ap_1 | \cdots | Ap_n) = (\lambda_1 p_1 | \cdots | \lambda_n p_n)$ iff $Ap_i = \lambda_i p_i$ for each $i$. The columns of $P$ are linearly independent iff $P$ is invertible. $\square$

**Corollary.** If $A$ has $n$ distinct eigenvalues, then $A$ is diagonalizable.

(Since eigenvectors corresponding to distinct eigenvalues are linearly independent.)

## The Diagonalization Procedure

1. Find the characteristic polynomial $p(\lambda) = \det(\lambda I - A)$.
2. Find all roots $\lambda_1, \ldots, \lambda_k$ (distinct eigenvalues) and their algebraic multiplicities $m_1, \ldots, m_k$.
3. For each $\lambda_i$, find a basis for $E_{\lambda_i} = \ker(\lambda_i I - A)$ by row reduction.
4. Check: $A$ is diagonalizable iff $\sum_i \dim E_{\lambda_i} = n$ (i.e., geometric multiplicity equals algebraic multiplicity for every eigenvalue).
5. Form $P = [v_1 | \cdots | v_n]$ (columns are eigenvectors, ordered matching the eigenvalues).
6. $D = \text{diag}(\lambda_1, \ldots, \lambda_n)$ (eigenvalues in matching order).

**Example.** $A = \begin{pmatrix}4&1\\2&3\end{pmatrix}$.

$p(\lambda) = (\lambda-4)(\lambda-3) - 2 = \lambda^2 - 7\lambda + 10 = (\lambda-2)(\lambda-5)$.

For $\lambda_1 = 2$: $E_2 = \ker\begin{pmatrix}2&-1\\-2&1\end{pmatrix} = \text{span}\{(1,2)^T\}$.

For $\lambda_2 = 5$: $E_5 = \ker\begin{pmatrix}-1&-1\\-2&-2\end{pmatrix} = \text{span}\{(-1,1)^T\}$.

Wait: $\ker\begin{pmatrix}-1&-1\\-2&-2\end{pmatrix}$: row reduce to $\begin{pmatrix}1&1\\0&0\end{pmatrix}$, so $v_1 = -v_2$; eigenvector $(-1,1)^T$.

$P = \begin{pmatrix}1&-1\\2&1\end{pmatrix}$, $D = \begin{pmatrix}2&0\\0&5\end{pmatrix}$.

Verify: $AP = \begin{pmatrix}4&1\\2&3\end{pmatrix}\begin{pmatrix}1&-1\\2&1\end{pmatrix} = \begin{pmatrix}6&-3\\8&1\end{pmatrix} \cdot ...$

Let me recompute: $A(1,2)^T = (4+2, 2+6)^T = (6,8)^T = 2(3,4)^T$. Hmm, $(4\cdot1 + 1\cdot2, 2\cdot1 + 3\cdot2)^T = (6, 8)^T \neq 2(1,2)^T = (2,4)^T$. Let me recheck $\lambda_1 = 2$: $(2I-A) = \begin{pmatrix}-2&-1\\-2&-1\end{pmatrix}$, row reduce: $\begin{pmatrix}2&1\\0&0\end{pmatrix}$, so $2v_1 + v_2 = 0$, eigenvector $(1,-2)^T$. Verify: $A(1,-2)^T = (4-2, 2-6)^T = (2,-4)^T = 2(1,-2)^T$. Correct.

For $\lambda_2 = 5$: $(5I-A) = \begin{pmatrix}1&-1\\-2&2\end{pmatrix}$, row reduce: $\begin{pmatrix}1&-1\\0&0\end{pmatrix}$, so $v_1 = v_2$, eigenvector $(1,1)^T$. Verify: $A(1,1)^T = (5,5)^T = 5(1,1)^T$. Correct.

$P = \begin{pmatrix}1&1\\-2&1\end{pmatrix}$, $D = \begin{pmatrix}2&0\\0&5\end{pmatrix}$.

## Powers of Diagonalizable Matrices

If $A = PDP^{-1}$:
$$A^k = PD^kP^{-1} = P\begin{pmatrix}\lambda_1^k & & \\ & \ddots & \\ & & \lambda_n^k\end{pmatrix}P^{-1}.$$

This makes computing $A^{100}$ as easy as computing $\lambda_i^{100}$.

## Applications to ODE Systems

For $\mathbf{x}' = A\mathbf{x}$ with $A$ diagonalizable, $A = PDP^{-1}$. Let $\mathbf{y} = P^{-1}\mathbf{x}$. Then $\mathbf{y}' = P^{-1}A\mathbf{x} = P^{-1}APP^{-1}\mathbf{x} = D\mathbf{y}$. The system $\mathbf{y}' = D\mathbf{y}$ decouples:
$$y_i' = \lambda_i y_i \implies y_i(t) = y_i(0)e^{\lambda_i t}.$$

Transforming back: $\mathbf{x}(t) = P\mathbf{y}(t) = \sum_{i=1}^n y_i(0) e^{\lambda_i t} p_i$, where $p_i$ are columns of $P$.

With initial condition $\mathbf{x}(0) = \mathbf{x}_0$: $\mathbf{y}(0) = P^{-1}\mathbf{x}_0$, giving $y_i(0) = (P^{-1}\mathbf{x}_0)_i$.

## When Diagonalization Fails

A matrix fails to be diagonalizable when some eigenvalue has geometric multiplicity strictly less than its algebraic multiplicity. This happens exactly when, for that eigenvalue $\lambda_0$, $\dim\ker(\lambda_0 I - A) < m_{\lambda_0}$ (the algebraic multiplicity). In this case, the Jordan Normal Form (next section) provides the correct generalization.

## Common Pitfalls

**Confusing similar and equal.** $A = PDP^{-1}$ means $A$ and $D$ are similar — they represent the same linear map in different bases. They are not equal unless $P = I$.

**Column order matters.** The eigenvalues in $D$ must match the order of the eigenvectors in $P$. Swapping two eigenvectors in $P$ requires swapping the corresponding eigenvalues in $D$.

**Non-unique diagonalization.** A diagonalizable matrix has many diagonalizing matrices $P$ (any reordering or rescaling of eigenvectors). The diagonal matrix $D$ is unique up to reordering of the diagonal entries.
