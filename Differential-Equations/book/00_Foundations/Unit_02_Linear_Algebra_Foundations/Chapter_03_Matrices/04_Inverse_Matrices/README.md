# Inverse Matrices

The inverse of a matrix $A$ is the matrix $A^{-1}$ such that $AA^{-1} = A^{-1}A = I$. It exists precisely when $A$ is invertible (equivalently, when $\det A \neq 0$, or when $A$ has full rank), and when it exists it allows linear systems to be "solved" by multiplication: $Ax = b$ implies $x = A^{-1}b$. However, matrix inversion is computationally expensive and rarely the best tool for solving a specific system — row reduction is more efficient for that purpose. The inverse is more useful theoretically and for deriving formulas.

## Conditions for Invertibility

For an $n \times n$ matrix $A$, the following are equivalent:
1. $A$ is invertible (there exists $A^{-1}$ with $AA^{-1} = A^{-1}A = I$).
2. $\det(A) \neq 0$.
3. $\ker A = \{\mathbf{0}\}$ (the system $Ax = 0$ has only the trivial solution).
4. $\text{rank}(A) = n$ (full rank).
5. The columns of $A$ are linearly independent.
6. The rows of $A$ are linearly independent.
7. For every $b \in \mathbb{R}^n$, $Ax = b$ has a unique solution.
8. $A$ is a product of elementary matrices.

These equivalences are the "Invertible Matrix Theorem" and provide many different angles from which to check or prove invertibility.

## Computing the Inverse

**Method 1: Augmented matrix.** Form $[A | I]$ and row-reduce. If $A$ is invertible, the left block reduces to $I$ and the right block becomes $A^{-1}$.

**Example.** Compute $A^{-1}$ for $A = \begin{pmatrix}2&1\\5&3\end{pmatrix}$.

$$\left[\begin{array}{cc|cc}2&1&1&0\\5&3&0&1\end{array}\right] \to \left[\begin{array}{cc|cc}1&0&3&-1\\0&1&-5&2\end{array}\right].$$

So $A^{-1} = \begin{pmatrix}3&-1\\-5&2\end{pmatrix}$. Check: $\det(A) = 6-5 = 1$ and $AA^{-1} = I$. $\checkmark$

**Method 2: Adjugate formula.** For $n\times n$ matrix $A$ with $\det A \neq 0$:
$$A^{-1} = \frac{1}{\det A}\,\text{adj}(A),$$
where $\text{adj}(A)_{ij} = C_{ji}$ (the $(j,i)$ cofactor — note the transposition). For $2\times 2$:
$$\begin{pmatrix}a&b\\c&d\end{pmatrix}^{-1} = \frac{1}{ad-bc}\begin{pmatrix}d&-b\\-c&a\end{pmatrix}.$$

This formula is useful for $2\times 2$ and $3\times 3$ matrices and for theoretical arguments; for larger matrices, row reduction is preferred.

## Properties of the Inverse

- $(A^{-1})^{-1} = A$.
- $(AB)^{-1} = B^{-1}A^{-1}$ (reversal of order).
- $(A^T)^{-1} = (A^{-1})^T$.
- $\det(A^{-1}) = 1/\det(A)$.
- $(\alpha A)^{-1} = (1/\alpha)A^{-1}$ for $\alpha \neq 0$.

## Left and Right Inverses

For non-square matrices, the notion splits: a **left inverse** of $A \in M_{m\times n}$ is $L$ with $LA = I_n$; a **right inverse** is $R$ with $AR = I_m$.

- A **left inverse** exists iff $A$ has full column rank ($\text{rank}(A) = n$, so $n \leq m$) iff $Ax = 0$ has only the trivial solution.
- A **right inverse** exists iff $A$ has full row rank ($\text{rank}(A) = m$, so $m \leq n$) iff $Ax = b$ has a solution for every $b$.

If both exist and $A$ is square, they agree with the (two-sided) inverse.

## The Pseudo-Inverse

For rectangular or rank-deficient matrices, the **Moore-Penrose pseudoinverse** $A^+$ generalizes the inverse. It satisfies $AA^+A = A$, $A^+AA^+ = A^+$, and $(AA^+)^T = AA^+$, $(A^+A)^T = A^+A$. For full-column-rank $A$: $A^+ = (A^TA)^{-1}A^T$. The pseudoinverse gives the least-squares solution to overdetermined systems.

## Application: Matrix Equations and ODE Parameter Identification

In parameter estimation for ODEs, one collects measurements $y_1, \ldots, y_m$ that (ideally) satisfy a linear relation $A\theta = y$ where $\theta \in \mathbb{R}^n$ is the parameter vector and $A$ is a "design matrix." When $m > n$, the system is overdetermined and generally inconsistent. The least-squares solution $\hat\theta = A^+y = (A^TA)^{-1}A^Ty$ (when $A^TA$ is invertible) minimizes $\|A\theta - y\|^2$, providing the best fit parameter vector. The invertibility of $A^TA$ is related to the "identifiability" of the ODE model.

## Inverting the Fundamental Matrix

For the linear ODE system $\mathbf{x}' = A(t)\mathbf{x}$, the fundamental matrix $\Phi(t)$ satisfies $\Phi'(t) = A(t)\Phi(t)$ and $\Phi(t_0) = I$. The solution is $\mathbf{x}(t) = \Phi(t)\mathbf{x}_0$. For the non-homogeneous system $\mathbf{x}' = A\mathbf{x} + g$, the variation of parameters formula gives $\mathbf{x}(t) = \Phi(t)\mathbf{x}_0 + \int_{t_0}^t \Phi(t)\Phi(s)^{-1}g(s)\,ds$. Computing $\Phi(t)^{-1}$ (which equals the fundamental matrix of the adjoint system) requires knowing when and how matrices are invertible.

## Common Pitfalls

**Expecting $A^{-1}$ to always exist.** It exists only for square, full-rank matrices. The $2\times 3$ matrix has no two-sided inverse.

**Computing $A^{-1}B$ vs. $BA^{-1}$.** These are different because matrix multiplication is non-commutative. Solve $AX = B$ as $X = A^{-1}B$; solve $XA = B$ as $X = BA^{-1}$.

**Using the inverse for computation.** To solve $Ax = b$, row reduction (Gaussian elimination) is $O(n^3)$ and more numerically stable than computing $A^{-1}$ (also $O(n^3)$ but with a larger constant and potentially larger roundoff).
