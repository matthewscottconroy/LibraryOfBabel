# Determinants

The determinant of a square matrix is a single real number that encodes, in one scalar, a remarkable amount of information about the matrix: whether it is invertible, what volume-scaling factor the corresponding linear map applies, and — through the characteristic polynomial — what the eigenvalues are. Determinants appear in Cramer's rule, in the Wronskian of ODE solutions, and in the change-of-variables formula for multiple integrals.

## Definition

The determinant of an $n \times n$ matrix $A$ can be defined by cofactor expansion. The **cofactor** $C_{ij}$ is $(-1)^{i+j}$ times the determinant of the $(n-1) \times (n-1)$ submatrix obtained by deleting row $i$ and column $j$ of $A$.

**Cofactor expansion along row $i$:**
$$\det(A) = \sum_{j=1}^n a_{ij} C_{ij} = \sum_{j=1}^n (-1)^{i+j} a_{ij} M_{ij},$$
where $M_{ij}$ is the $(i,j)$ minor (determinant of the submatrix after deleting row $i$ and column $j$). The expansion can be done along any row or column, giving the same result.

**Base cases:**
$$\det(a) = a \text{ for } 1\times 1, \quad \det\begin{pmatrix}a&b\\c&d\end{pmatrix} = ad - bc.$$

**$3 \times 3$:**
$$\det\begin{pmatrix}a&b&c\\d&e&f\\g&h&i\end{pmatrix} = a(ei-fh) - b(di-fg) + c(dh-eg).$$

## Geometric Meaning

For $A \in M_{n\times n}$, $|\det(A)|$ is the volume scaling factor of the linear map $T(x) = Ax$: the image of a unit cube under $T$ has volume $|\det A|$. The sign of $\det A$ encodes orientation: if $\det A > 0$, $T$ preserves orientation; if $\det A < 0$, $T$ reverses it.

**Example.** A rotation by $\theta$ in $\mathbb{R}^2$ has $\det = \cos^2\theta + \sin^2\theta = 1$: rotations preserve volume and orientation. A reflection has $\det = -1$.

## Properties

**(P1) Multilinearity:** $\det$ is linear in each row (or column) separately.

**(P2) Alternating:** Swapping two rows changes the sign of the determinant.

**(P3) Normalization:** $\det(I) = 1$.

These three properties uniquely characterize the determinant.

**Key consequences:**
- A matrix with two equal rows has $\det = 0$.
- Adding a multiple of one row to another does not change the determinant.
- $\det(A^T) = \det(A)$.
- $\det(AB) = \det(A)\det(B)$ (multiplicativity).
- $\det(A^{-1}) = 1/\det(A)$ (if $A$ is invertible).
- $\det(\alpha A) = \alpha^n \det(A)$ for $A \in M_{n\times n}$.

## Invertibility

**Theorem.** $A$ is invertible if and only if $\det(A) \neq 0$.

*Proof sketch.* Row reduction transforms $A$ to an echelon form. Each elementary row operation changes the determinant by a known nonzero factor. The final determinant is zero iff a zero row (hence a free variable, hence non-unique solutions to $Ax = 0$) appears, iff $A$ is not invertible. $\square$

## Computing Determinants Efficiently

For large matrices, the cofactor expansion is computationally expensive ($O(n!)$ operations). In practice, determinants are computed via LU decomposition: $\det(A) = \det(L)\det(U) = (\pm 1)\prod_i u_{ii}$, where $u_{ii}$ are the diagonal entries of $U$. This is $O(n^3)$.

## Cramer's Rule

For the system $Ax = b$ with $A$ invertible:
$$x_i = \frac{\det(A_i)}{\det(A)},$$
where $A_i$ is $A$ with its $i$-th column replaced by $b$.

Cramer's rule is theoretically elegant but computationally impractical for large systems (row reduction is far more efficient). It is useful in small cases and in theoretical arguments.

**Example.** Solve $\begin{pmatrix}2&1\\1&3\end{pmatrix}\begin{pmatrix}x\\y\end{pmatrix} = \begin{pmatrix}5\\4\end{pmatrix}$.

$\det(A) = 6 - 1 = 5$. $x = \frac{\det\begin{pmatrix}5&1\\4&3\end{pmatrix}}{5} = \frac{15-4}{5} = \frac{11}{5}$. $y = \frac{\det\begin{pmatrix}2&5\\1&4\end{pmatrix}}{5} = \frac{8-5}{5} = \frac{3}{5}$.

## The Characteristic Polynomial

For an $n \times n$ matrix $A$, the **characteristic polynomial** is
$$p(\lambda) = \det(\lambda I - A),$$
a polynomial of degree $n$ in $\lambda$. The eigenvalues of $A$ are the roots of $p(\lambda)$. Determinants are thus the gateway to eigenvalue computation; see Chapter 4.

## The Wronskian Revisited

For solutions $y_1, \ldots, y_n$ of $L[y] = 0$, the Wronskian is precisely the determinant of the matrix of solutions and their derivatives. Abel's identity $W'(t) = -p_{n-1}(t)W(t)$ follows from properties of the determinant and the ODE, connecting the theory of determinants directly to ODE solution theory.

## Common Pitfalls

**Cofactor expansion on the wrong sign pattern.** The checkerboard sign $(-1)^{i+j}$ must be applied correctly. The $(1,1)$ entry always has positive sign, $(1,2)$ negative, $(2,1)$ negative, and so on.

**Confusing $\det(A+B) = \det(A) + \det(B)$.** This is false. Determinant is not linear in $A$; it is multilinear in the rows.

**Computing $\det(\alpha A)$ as $\alpha\det(A)$.** Correct for $1\times 1$, but for $n\times n$: $\det(\alpha A) = \alpha^n\det(A)$.
