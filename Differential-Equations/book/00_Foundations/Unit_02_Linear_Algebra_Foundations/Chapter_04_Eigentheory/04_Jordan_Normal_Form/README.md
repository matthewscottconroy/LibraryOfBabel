# Jordan Normal Form

Diagonalization fails when a matrix has repeated eigenvalues whose geometric multiplicity does not reach their algebraic multiplicity. In such cases, there are not enough independent eigenvectors to form a basis, and the diagonal representation is impossible. The Jordan normal form is the canonical replacement: every square matrix over $\mathbb{C}$ is similar to a block-diagonal matrix where each block — a Jordan block — is as close to diagonal as possible. Understanding Jordan form is necessary for solving linear ODE systems when the coefficient matrix is not diagonalizable.

## Jordan Blocks

**Definition.** A **Jordan block** of size $k$ with eigenvalue $\lambda$ is the $k\times k$ matrix
$$J_k(\lambda) = \begin{pmatrix}\lambda&1&0&\cdots&0\\0&\lambda&1&\cdots&0\\\vdots&&\ddots&\ddots&\vdots\\0&0&\cdots&\lambda&1\\0&0&\cdots&0&\lambda\end{pmatrix} = \lambda I + N,$$
where $N$ is the nilpotent shift matrix with $1$'s on the superdiagonal and $0$'s elsewhere. $N^k = 0$ but $N^{k-1} \neq 0$.

A $1\times 1$ Jordan block is just $(\lambda)$ — an ordinary eigenvalue with eigenvector $e_1$.

## The Jordan Normal Form Theorem

**Theorem.** Every $n\times n$ matrix $A$ over $\mathbb{C}$ is similar to a **Jordan matrix** $J$:
$$A = PJP^{-1}, \quad J = \begin{pmatrix}J_{k_1}(\lambda_1) & & \\ & \ddots & \\ & & J_{k_s}(\lambda_s)\end{pmatrix},$$
where the $J_{k_i}(\lambda_i)$ are Jordan blocks (possibly with the same $\lambda$ for different blocks). The Jordan form $J$ is unique up to reordering the blocks.

For a given eigenvalue $\lambda$:
- The number of Jordan blocks with eigenvalue $\lambda$ equals the geometric multiplicity $\dim E_\lambda$.
- The sum of the block sizes for $\lambda$ equals the algebraic multiplicity $m_\lambda$.
- The largest block size for $\lambda$ is the **index** of $\lambda$.

## Generalized Eigenvectors

When an eigenvalue $\lambda$ has a Jordan block of size $k > 1$, the corresponding column of $P$ is a **generalized eigenvector chain**: $v, u_1, \ldots, u_{k-1}$ where
$$Av = \lambda v, \quad Au_1 = \lambda u_1 + v, \quad Au_2 = \lambda u_2 + u_1, \quad \ldots$$

Equivalently, $u_j \in \ker(\lambda I - A)^{j+1} \setminus \ker(\lambda I - A)^j$. The chain is:
$$(A - \lambda I)v = 0, \quad (A-\lambda I)u_1 = v, \quad (A-\lambda I)u_j = u_{j-1}.$$

One solves these successively by row reduction, starting from an eigenvector $v$.

## Example

$A = \begin{pmatrix}3&1&0\\0&3&1\\0&0&3\end{pmatrix}$ (a single $3\times 3$ Jordan block with $\lambda = 3$).

Characteristic polynomial: $(\lambda-3)^3$. Algebraic multiplicity: $3$. Geometric multiplicity: $\dim\ker(3I-A) = \dim\ker\begin{pmatrix}0&-1&0\\0&0&-1\\0&0&0\end{pmatrix} = 1$.

Since there is one Jordan block (dim $E_3 = 1$) of size $3$, $J = A$ already is in Jordan form. The eigenvector is $v = e_1 = (1,0,0)^T$. Generalized eigenvectors: $(3I-A)u_1 = v$, i.e., $-u_{1,2} = 1$, $-u_{1,3} = 0$: $u_1 = (0,-1,0)^T$ (up to adding multiples of $v$). Then $(3I-A)u_2 = u_1$: $-u_{2,2} = 0$, $-u_{2,3} = -1$: $u_2 = (0,0,-1)^T$ (up to adding multiples).

So $P = [v|u_1|u_2] = I$ (since the block is already in Jordan form in the standard basis).

## Solutions of ODEs via Jordan Form

For $\mathbf{x}' = A\mathbf{x}$ where $A = PJP^{-1}$, let $\mathbf{y} = P^{-1}\mathbf{x}$. Then $\mathbf{y}' = J\mathbf{y}$. Each Jordan block gives a subsystem $\mathbf{y}' = J_k(\lambda)\mathbf{y}$, solved as follows.

The system for a $k\times k$ block:
$$y_1' = \lambda y_1 + y_2, \quad y_2' = \lambda y_2 + y_3, \quad \ldots, \quad y_k' = \lambda y_k.$$

Solve from the bottom up: $y_k(t) = c_k e^{\lambda t}$, $y_{k-1}(t) = (c_{k-1} + c_k t)e^{\lambda t}$, ..., in general:
$$y_j(t) = e^{\lambda t}\sum_{m=0}^{k-j} c_{j+m}\frac{t^m}{m!}.$$

The general term is a **polynomial times an exponential**. The degree of the polynomial equals one less than the size of the Jordan block. This is why repeated eigenvalues in ODE systems give rise to solutions of the form $t^m e^{\lambda t}$.

**Example.** For the repeated eigenvalue $\lambda$ with a $2\times 2$ Jordan block: solutions are $e^{\lambda t}v$ and $te^{\lambda t}v + e^{\lambda t}u_1$ where $v$ is the eigenvector and $u_1$ the generalized eigenvector. More explicitly: $e^{\lambda t}\begin{pmatrix}1\\0\end{pmatrix}$ and $e^{\lambda t}\begin{pmatrix}t\\1\end{pmatrix}$ in the Jordan basis.

## Computing the Jordan Form

In theory: find the eigenvalues, compute the dimensions of $\ker(A-\lambda I)^j$ for $j = 1, 2, \ldots$ to determine the block structure, then find the generalized eigenvectors.

In practice (numerically): Jordan form is numerically unstable (a small perturbation can change the block structure completely). Numerical algorithms use the Schur decomposition ($A = QTQ^*$ where $Q$ is unitary and $T$ is upper triangular) instead, which is stable.

## Common Pitfalls

**Assuming Jordan blocks are unique.** The block sizes for a given eigenvalue are determined by the matrix, but which specific generalized eigenvectors form the chains is not unique — different choices lead to different $P$ but the same Jordan form $J$.

**Confusing Jordan form with diagonalization.** A diagonalizable matrix has a Jordan form in which every block is $1\times 1$. The Jordan form is a generalization, not a replacement.

**Forgetting complex entries.** Over $\mathbb{R}$, the Jordan form theorem applies only over $\mathbb{C}$. Real matrices may need complex $P$ and $J$. The real Jordan form uses $2\times 2$ real blocks for complex conjugate eigenvalue pairs.
