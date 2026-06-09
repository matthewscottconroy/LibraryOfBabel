# Eigenvalues and Eigenvectors

Most linear maps distort the geometry of a vector space: they rotate, shear, and stretch in complicated ways. But every linear map has special directions — called eigenvectors — that are merely scaled (stretched or reversed), not rotated. These privileged directions, and the scaling factors associated with them, are the eigenvalues and eigenvectors. They encode the essential behavior of the map and are the foundation for solving linear differential equations.

## Definitions

**Definition.** Let $A$ be an $n\times n$ matrix over $\mathbb{R}$ (or $\mathbb{C}$). A nonzero vector $v \in \mathbb{R}^n$ is an **eigenvector** of $A$ with **eigenvalue** $\lambda$ if
$$Av = \lambda v.$$

The condition $v \neq \mathbf{0}$ is essential: the equation $A\mathbf{0} = \lambda\mathbf{0}$ holds trivially for any $\lambda$ and so gives no information.

Geometrically, $Av = \lambda v$ says the linear map $T(x) = Ax$ maps $v$ to a scalar multiple of itself — the vector $v$ is scaled by $\lambda$. If $\lambda > 0$, the direction is preserved; if $\lambda < 0$, it is reversed; if $\lambda = 0$, $v$ is mapped to the origin (so $A$ is singular); if $\lambda$ is complex, the vector is "rotated and scaled" in a sense that requires working over $\mathbb{C}$.

## Finding Eigenvectors

Rearrange $Av = \lambda v$ as $(\lambda I - A)v = \mathbf{0}$. This is a homogeneous linear system. For a nonzero solution $v$ to exist, the system must have a non-trivial solution, which requires
$$\det(\lambda I - A) = 0.$$
The **characteristic equation** determines which $\lambda$ can be eigenvalues. Once $\lambda$ is found, the eigenvectors are the nonzero elements of $\ker(\lambda I - A)$.

**Example.** $A = \begin{pmatrix}3&1\\1&3\end{pmatrix}$.

Characteristic equation: $\det(\lambda I - A) = (\lambda-3)^2 - 1 = \lambda^2 - 6\lambda + 8 = (\lambda-2)(\lambda-4) = 0$.

Eigenvalues: $\lambda_1 = 2$, $\lambda_2 = 4$.

For $\lambda_1 = 2$: $(\lambda_1 I - A) = \begin{pmatrix}-1&-1\\-1&-1\end{pmatrix}$. Row reduce: $\begin{pmatrix}1&1\\0&0\end{pmatrix}$. Solution: $v = t(-1,1)^T$. Eigenvector: $v_1 = (-1,1)^T$.

For $\lambda_2 = 4$: $(\lambda_2 I - A) = \begin{pmatrix}1&-1\\-1&1\end{pmatrix}$. Row reduce: $\begin{pmatrix}1&-1\\0&0\end{pmatrix}$. Solution: $v = t(1,1)^T$. Eigenvector: $v_2 = (1,1)^T$.

## Eigenspaces

**Definition.** For eigenvalue $\lambda$, the **eigenspace** is $E_\lambda = \ker(\lambda I - A) = \{v : Av = \lambda v\}$.

$E_\lambda$ is a subspace of $\mathbb{R}^n$ (it is the null space of $\lambda I - A$). Its dimension is the **geometric multiplicity** of $\lambda$. The **algebraic multiplicity** is the multiplicity of $\lambda$ as a root of the characteristic polynomial. We always have geometric multiplicity $\leq$ algebraic multiplicity.

**Example.** $A = \begin{pmatrix}2&1\\0&2\end{pmatrix}$. Characteristic polynomial: $(\lambda-2)^2$. Algebraic multiplicity of $\lambda=2$: $2$. $E_2 = \ker(2I - A) = \ker\begin{pmatrix}0&-1\\0&0\end{pmatrix} = \text{span}\{(1,0)^T\}$. Geometric multiplicity: $1 < 2$. This matrix is not diagonalizable.

## Linear Independence of Eigenvectors

**Theorem.** Eigenvectors corresponding to distinct eigenvalues are linearly independent.

*Proof.* Suppose $\lambda_1, \ldots, \lambda_k$ are distinct eigenvalues with eigenvectors $v_1, \ldots, v_k$, and assume $\sum_{i=1}^k c_i v_i = 0$ for some scalars $c_i$. Apply $A$: $\sum_i c_i \lambda_i v_i = 0$. Apply $A$ again and subtract multiples — or use the Vandermonde-style argument by applying $\prod_{j\neq i}(A - \lambda_j I)$ to kill all but the $i$-th term — to conclude $c_i = 0$ for all $i$. $\square$

**Corollary.** If an $n\times n$ matrix has $n$ distinct eigenvalues, it has $n$ linearly independent eigenvectors and is diagonalizable.

## Complex Eigenvalues of Real Matrices

If $A$ is a real matrix and $\lambda = \alpha + i\beta$ (with $\beta \neq 0$) is a complex eigenvalue, then $\bar\lambda = \alpha - i\beta$ is also an eigenvalue (complex eigenvalues of real matrices come in conjugate pairs). The corresponding eigenvectors $v$ and $\bar v$ give rise to real solutions.

**Example.** $A = \begin{pmatrix}0&-1\\1&0\end{pmatrix}$ (rotation by $90°$). Eigenvalues: $\lambda = \pm i$. Eigenvectors over $\mathbb{C}$: $v = (1, -i)^T$ for $\lambda = i$. The real solutions to $\mathbf{x}' = A\mathbf{x}$ are $\cos t\begin{pmatrix}1\\0\end{pmatrix} - \sin t\begin{pmatrix}0\\1\end{pmatrix} = \begin{pmatrix}\cos t\\-\sin t\end{pmatrix}$ and $\begin{pmatrix}\sin t\\\cos t\end{pmatrix}$ — the rotation of any initial vector.

## Connection to ODE Solutions

For the system $\mathbf{x}' = A\mathbf{x}$, if $Av = \lambda v$, then $\mathbf{x}(t) = e^{\lambda t}v$ is a solution:
$$(e^{\lambda t}v)' = \lambda e^{\lambda t}v = A(e^{\lambda t}v).$$
If $A$ has $n$ linearly independent eigenvectors $v_1, \ldots, v_n$ with eigenvalues $\lambda_1, \ldots, \lambda_n$, the general solution is:
$$\mathbf{x}(t) = c_1 e^{\lambda_1 t}v_1 + c_2 e^{\lambda_2 t}v_2 + \cdots + c_n e^{\lambda_n t}v_n.$$

The constants $c_1, \ldots, c_n$ are determined by initial conditions. The eigenvalues dictate the long-term behavior: $e^{\lambda_i t}$ grows if $\text{Re}(\lambda_i) > 0$, decays if $\text{Re}(\lambda_i) < 0$, and oscillates without decay if $\text{Re}(\lambda_i) = 0$.

## Common Pitfalls

**Setting $v = 0$ as an eigenvector.** Eigenvectors must be nonzero by definition.

**Assuming eigenvalues are real.** Real matrices can have complex eigenvalues. The characteristic polynomial of a real matrix has real coefficients, so complex roots come in conjugate pairs.

**Confusing algebraic and geometric multiplicity.** A matrix with a repeated eigenvalue may still be diagonalizable (if geometric multiplicity equals algebraic multiplicity) or may not be (if geometric multiplicity is strictly less).
