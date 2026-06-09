# Symmetric Matrices and the Spectral Theorem

The Spectral Theorem is the most beautiful result in finite-dimensional linear algebra: every real symmetric matrix is orthogonally diagonalizable — its eigenvalues are all real, and its eigenvectors can be chosen to form an orthonormal basis for $\mathbb{R}^n$. This theorem is the finite-dimensional analog of Sturm-Liouville theory for differential equations, and it explains why symmetric matrices appear so naturally in physics and engineering.

## Symmetric Matrices

**Definition.** A matrix $A \in M_{n\times n}(\mathbb{R})$ is **symmetric** if $A^T = A$, i.e., $a_{ij} = a_{ji}$ for all $i, j$.

Examples: covariance matrices in statistics, stiffness matrices in structural mechanics, the matrix of second partial derivatives (Hessian) of a smooth function, and the coefficient matrices of many ODE boundary value problems.

## Key Properties

**Theorem 1.** All eigenvalues of a real symmetric matrix are real.

*Proof.* Suppose $Av = \lambda v$ with $v \neq 0$ (over $\mathbb{C}$). Take the complex inner product $\bar{v}^T(Av) = \lambda\bar{v}^Tv = \lambda\|v\|^2$. Also, $\bar{v}^T(Av) = \bar{v}^T(A^Tv) = (Av)^T\bar{v} = \lambda\overline{\bar{v}^Tv} = \bar\lambda\|v\|^2$. Since $\|v\|^2 > 0$: $\lambda = \bar\lambda$, so $\lambda$ is real. $\square$

**Theorem 2.** Eigenvectors of a real symmetric matrix corresponding to distinct eigenvalues are orthogonal.

*Proof.* If $Av = \lambda v$ and $Aw = \mu w$ with $\lambda \neq \mu$:
$$\lambda\langle v,w\rangle = \langle\lambda v,w\rangle = \langle Av,w\rangle = \langle v,A^Tw\rangle = \langle v,Aw\rangle = \langle v,\mu w\rangle = \mu\langle v,w\rangle.$$
So $(\lambda-\mu)\langle v,w\rangle = 0$; since $\lambda \neq \mu$, $\langle v,w\rangle = 0$. $\square$

## The Spectral Theorem

**Theorem (Spectral Theorem for Real Symmetric Matrices).** Let $A$ be an $n\times n$ real symmetric matrix. Then:
1. All eigenvalues of $A$ are real.
2. $\mathbb{R}^n$ has an orthonormal basis consisting of eigenvectors of $A$.
3. $A$ is orthogonally diagonalizable: $A = Q\Lambda Q^T$ where $Q$ is orthogonal ($Q^TQ = I$) and $\Lambda = \text{diag}(\lambda_1,\ldots,\lambda_n)$ with $\lambda_i$ the (real) eigenvalues.

*Proof sketch.* By the fundamental theorem of algebra and Theorem 1, all eigenvalues are real. Pick any eigenvalue $\lambda_1$ and a corresponding unit eigenvector $q_1$. The subspace $W = \{q_1\}^\perp$ is invariant under $A$ (if $\langle v,q_1\rangle = 0$, then $\langle Av,q_1\rangle = \langle v,Aq_1\rangle = \lambda_1\langle v,q_1\rangle = 0$, so $Av \in W$). The restriction $A|_W: W\to W$ is also symmetric, so by induction (on $n$), $W$ has an orthonormal eigenbasis. Combining with $q_1$ gives the orthonormal eigenbasis for $\mathbb{R}^n$. $\square$

## The Spectral Decomposition

If $q_1, \ldots, q_n$ are orthonormal eigenvectors with eigenvalues $\lambda_1, \ldots, \lambda_n$, then:
$$A = Q\Lambda Q^T = \sum_{i=1}^n \lambda_i q_i q_i^T.$$

Each term $\lambda_i q_i q_i^T$ is a **rank-1 projection** onto $\text{span}\{q_i\}$, scaled by $\lambda_i$. This decomposition expresses $A$ as a sum of simple "scaling in one direction" operations.

**Example.** $A = \begin{pmatrix}3&1\\1&3\end{pmatrix}$.

Eigenvalues: $\lambda_1 = 2$, $\lambda_2 = 4$. Eigenvectors (orthonormal): $q_1 = (-1,1)^T/\sqrt{2}$, $q_2 = (1,1)^T/\sqrt{2}$.

$$A = 2 \cdot \frac{1}{2}\begin{pmatrix}1&-1\\-1&1\end{pmatrix} + 4 \cdot \frac{1}{2}\begin{pmatrix}1&1\\1&1\end{pmatrix} = \begin{pmatrix}1&-1\\-1&1\end{pmatrix} + \begin{pmatrix}2&2\\2&2\end{pmatrix} = \begin{pmatrix}3&1\\1&3\end{pmatrix}. \checkmark$$

## Quadratic Forms

A **quadratic form** is a function $Q(x) = x^TAx$ for symmetric $A$. Using the spectral decomposition $A = Q\Lambda Q^T$ and the substitution $y = Q^Tx$:
$$Q(x) = x^TQ\Lambda Q^Tx = y^T\Lambda y = \sum_{i=1}^n \lambda_i y_i^2.$$

This **principal axis theorem** says every quadratic form can be diagonalized in an orthonormal basis. The quadratic form is:
- **Positive definite** ($Q(x) > 0$ for $x \neq 0$) iff all $\lambda_i > 0$.
- **Positive semidefinite** iff all $\lambda_i \geq 0$.
- **Indefinite** iff some $\lambda_i > 0$ and some $\lambda_i < 0$.

## Connection to Self-Adjoint Operators and ODEs

A linear operator $L$ on a function space is **self-adjoint** (with respect to an inner product $\langle \cdot,\cdot\rangle$) if $\langle Lu,v\rangle = \langle u,Lv\rangle$ for all $u,v$ in the domain. The Spectral Theorem generalizes to self-adjoint operators: their eigenvalues are real and their eigenfunctions are orthogonal.

For Sturm-Liouville problems:
$$(p(x)y')' + q(x)y = \lambda w(x)y, \quad y(a) = y(b) = 0,$$
the operator $Ly = -(py')'- qy$ is self-adjoint with respect to the weighted inner product $\langle f,g\rangle_w = \int_a^b f(x)g(x)w(x)\,dx$. Its eigenvalues are real and form an increasing sequence $\lambda_1 < \lambda_2 < \cdots \to \infty$, and the eigenfunctions $\{\phi_n\}$ form a complete orthonormal system: every $L^2_w([a,b])$ function expands as $f = \sum_n \langle f,\phi_n\rangle_w \phi_n$.

This is the Sturm-Liouville analog of the spectral decomposition $A = \sum_i \lambda_i q_iq_i^T$, with the sum replaced by a series and finite matrices replaced by differential operators.
