# The Gram-Schmidt Process

Given any linearly independent set of vectors, the Gram-Schmidt process transforms them into an orthonormal set that spans the same subspace. The procedure is constructive: it peels off components one at a time, ensuring each new vector is orthogonal to all previous ones. The result is an orthonormal basis, which simplifies all subsequent computations (inner products, projections, expansion of arbitrary vectors) to the point where they become completely explicit.

## The Algorithm

Let $\{v_1, v_2, \ldots, v_k\}$ be linearly independent vectors in an inner product space. The Gram-Schmidt process produces orthonormal vectors $\{q_1, q_2, \ldots, q_k\}$ with $\text{span}\{q_1, \ldots, q_j\} = \text{span}\{v_1, \ldots, v_j\}$ for each $j$.

**Step 1.** Set $u_1 = v_1$ and $q_1 = u_1/\|u_1\|$.

**Step 2.** Remove from $v_2$ its component along $q_1$:
$$u_2 = v_2 - \langle v_2, q_1\rangle q_1, \qquad q_2 = u_2/\|u_2\|.$$

**Step $j$.** Remove from $v_j$ all components along $q_1, \ldots, q_{j-1}$:
$$u_j = v_j - \sum_{i=1}^{j-1} \langle v_j, q_i\rangle q_i, \qquad q_j = u_j/\|u_j\|.$$

**Validity.** $u_j \neq \mathbf{0}$ at each step because $v_j \notin \text{span}\{v_1, \ldots, v_{j-1}\} = \text{span}\{q_1, \ldots, q_{j-1}\}$ (by linear independence).

## Orthogonality Verification

For $i < j$:
$$\langle u_j, q_i\rangle = \langle v_j, q_i\rangle - \sum_{\ell=1}^{j-1}\langle v_j, q_\ell\rangle\langle q_\ell, q_i\rangle = \langle v_j, q_i\rangle - \langle v_j, q_i\rangle \cdot 1 = 0$$
(using $\langle q_\ell, q_i\rangle = \delta_{\ell i}$, which holds inductively). Since $q_j = u_j/\|u_j\|$, also $\langle q_j, q_i\rangle = 0$.

## Example in $\mathbb{R}^3$

Let $v_1 = (1,1,0)$, $v_2 = (1,0,1)$, $v_3 = (0,1,1)$.

**Step 1.** $u_1 = (1,1,0)$, $\|u_1\| = \sqrt{2}$, $q_1 = (1,1,0)/\sqrt{2}$.

**Step 2.** $\langle v_2, q_1\rangle = \frac{1\cdot1+0\cdot1}{\sqrt{2}} = \frac{1}{\sqrt{2}}$.
$$u_2 = (1,0,1) - \frac{1}{\sqrt{2}} \cdot \frac{1}{\sqrt{2}}(1,1,0) = (1,0,1) - \frac{1}{2}(1,1,0) = (1/2,-1/2,1).$$
$\|u_2\| = \sqrt{1/4+1/4+1} = \sqrt{3/2}$. $q_2 = (1/2,-1/2,1)/\sqrt{3/2} = (1,-1,2)/\sqrt{6}$.

**Step 3.** $\langle v_3, q_1\rangle = (0+1)/\sqrt{2} = 1/\sqrt{2}$. $\langle v_3, q_2\rangle = (0-1+2)/\sqrt{6} = 1/\sqrt{6}$.
$$u_3 = (0,1,1) - \frac{1}{\sqrt{2}} \cdot \frac{1}{\sqrt{2}}(1,1,0) - \frac{1}{\sqrt{6}} \cdot \frac{1}{\sqrt{6}}(1,-1,2) = (0,1,1) - \frac{1}{2}(1,1,0) - \frac{1}{6}(1,-1,2) = (-2/3, 2/3, 2/3).$$
$q_3 = (-1,1,1)/\sqrt{3}$.

## Example in $C([0,1])$: Orthogonal Polynomials

Let the inner product be $\langle f,g\rangle = \int_0^1 f(x)g(x)\,dx$ and apply Gram-Schmidt to $\{1, x, x^2\}$.

$q_1 = 1/\|1\| = 1$.

$u_2 = x - \langle x, 1\rangle \cdot 1 = x - 1/2$. $\|u_2\|^2 = \int_0^1 (x-1/2)^2\,dx = 1/12$. $q_2 = (x-1/2)/\sqrt{1/12} = 2\sqrt{3}(x-1/2)$.

$u_3 = x^2 - \langle x^2, q_1\rangle q_1 - \langle x^2, q_2\rangle q_2$: $\langle x^2, 1\rangle = 1/3$. $\langle x^2, 2\sqrt{3}(x-1/2)\rangle = 2\sqrt{3}(1/4 - 1/6) = 2\sqrt{3}/12 = \sqrt{3}/6$. So $u_3 = x^2 - 1/3 - \frac{\sqrt{3}}{6}\cdot 2\sqrt{3}(x-1/2) = x^2 - x + 1/6$.

These are the **Legendre polynomials** (shifted to $[0,1]$), which form the orthogonal polynomial basis for $L^2([0,1])$.

## Connection to QR Decomposition

The Gram-Schmidt process is exactly the construction of the QR decomposition $A = QR$:
- The columns of $Q$ are $q_1, \ldots, q_n$ (the orthonormal vectors).
- The matrix $R$ is upper triangular with $r_{ij} = \langle v_j, q_i\rangle$ for $i \leq j$ and $r_{ij} = 0$ for $i > j$.

Since $v_j = \sum_{i=1}^j r_{ij}q_i$ (from the Gram-Schmidt construction), the identity $A = QR$ follows.

## Modified Gram-Schmidt (Numerical Stability)

The classical Gram-Schmidt algorithm loses orthogonality due to floating-point errors, especially when vectors are nearly linearly dependent. The **modified Gram-Schmidt** algorithm reorders operations to reduce error propagation: instead of computing all projections $\langle v_j, q_i\rangle$ at once, it subtracts projections one at a time, updating $u_j$ incrementally. The result is mathematically equivalent but numerically much more stable.

## Common Pitfalls

**Applying to a linearly dependent set.** If some $v_j \in \text{span}\{v_1, \ldots, v_{j-1}\}$, then $u_j = \mathbf{0}$ and the algorithm fails at the normalization step. Linear independence must be checked first.

**Wrong projection formula.** The projection of $v$ onto $q$ (unit vector) is $\langle v,q\rangle q$, not $\langle v,q\rangle$ (the scalar alone) or $v/q$ (undefined).

**Forgetting to normalize.** Gram-Schmidt produces orthogonal vectors $u_j$; dividing by $\|u_j\|$ is needed to get orthonormal vectors $q_j$. Orthogonal is not the same as orthonormal.
