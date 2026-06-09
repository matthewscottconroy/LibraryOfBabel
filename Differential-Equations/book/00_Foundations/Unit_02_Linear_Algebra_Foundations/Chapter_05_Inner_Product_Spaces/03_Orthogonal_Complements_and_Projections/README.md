# Orthogonal Complements and Projections

When a vector $v$ in an inner product space is not in a subspace $W$, the closest point in $W$ to $v$ is its **orthogonal projection** onto $W$. This projection decomposes $v$ uniquely as the sum of a component in $W$ and a component perpendicular to $W$. The perpendicular component is called the "error" or "residual," and minimizing it is the basis for all least-squares methods. The theory connects geometry (perpendicularity), algebra (subspaces), and analysis (best approximation).

## Orthogonal Complements

**Definition.** The **orthogonal complement** of a subspace $W \subseteq V$ is
$$W^\perp = \{v \in V : \langle v, w\rangle = 0 \text{ for all } w \in W\}.$$

**Theorem.** $W^\perp$ is a subspace of $V$.

*Proof.* $\mathbf{0} \in W^\perp$. If $u, v \in W^\perp$ and $w \in W$: $\langle \alpha u + \beta v, w\rangle = \alpha\langle u,w\rangle + \beta\langle v,w\rangle = 0$. $\square$

**Properties:**
- $(W^\perp)^\perp = W$ (in finite dimensions).
- $W \cap W^\perp = \{\mathbf{0}\}$.
- $\dim W + \dim W^\perp = \dim V$ (in finite dimensions).
- For a matrix $A$: $(\text{col}\,A)^\perp = \ker A^T$ (the null space of $A^T$).

## Orthogonal Projection

**Theorem.** Let $W$ be a finite-dimensional subspace of $V$ with orthonormal basis $\{q_1, \ldots, q_k\}$. For any $v \in V$, the **orthogonal projection** of $v$ onto $W$ is
$$\text{proj}_W v = \sum_{i=1}^k \langle v, q_i\rangle q_i.$$

This vector $\hat{v} = \text{proj}_W v$ satisfies:
1. $\hat{v} \in W$.
2. $v - \hat{v} \in W^\perp$ (i.e., $v - \hat{v}$ is orthogonal to every vector in $W$).

*Proof.* $\hat{v} \in W$ is clear (it is a linear combination of $q_i \in W$). For $w = q_j \in W$: $\langle v - \hat{v}, q_j\rangle = \langle v, q_j\rangle - \langle v, q_j\rangle = 0$. Since $W = \text{span}\{q_j\}$, $v-\hat{v} \perp W$. $\square$

**Uniqueness.** The decomposition $v = \hat{v} + (v - \hat{v})$ with $\hat{v} \in W$ and $v-\hat{v} \in W^\perp$ is unique. (If $v = w_1 + w_1^\perp = w_2 + w_2^\perp$, then $w_1 - w_2 = w_2^\perp - w_1^\perp \in W \cap W^\perp = \{\mathbf{0}\}$.)

## The Best Approximation Theorem

**Theorem.** Among all vectors in $W$, the projection $\hat{v}$ is the closest to $v$:
$$\|v - \hat{v}\| \leq \|v - w\| \quad \text{for all } w \in W,$$
with equality iff $w = \hat{v}$.

*Proof.* For any $w \in W$: $v - w = (v - \hat{v}) + (\hat{v} - w)$. Since $v - \hat{v} \in W^\perp$ and $\hat{v} - w \in W$, they are orthogonal. By the Pythagorean theorem:
$$\|v - w\|^2 = \|v - \hat{v}\|^2 + \|\hat{v} - w\|^2 \geq \|v - \hat{v}\|^2,$$
with equality iff $\hat{v} = w$. $\square$

This is the geometric foundation for least-squares: to minimize $\|Ax - b\|^2$ over $x$, the optimal $Ax$ is the projection of $b$ onto $\text{col}(A)$, so the residual $b - Ax$ is in $(\text{col}(A))^\perp = \ker(A^T)$.

## Projection Matrices

The orthogonal projection onto $W$ is a linear map $P_W: V \to V$. In terms of an orthonormal basis $Q = [q_1|\cdots|q_k]$ of $W$:
$$P_W v = QQ^Tv, \quad P_W = QQ^T.$$

**Properties of projection matrices:**
- $P^2 = P$ (idempotent: projecting twice gives the same result as projecting once).
- $P^T = P$ (symmetric).
- $\ker P = W^\perp$, $\text{im}\, P = W$.
- $\text{rank}(P) = \dim W$, $\text{null}(P) = \dim W^\perp$.

Conversely, any matrix satisfying $P^2 = P$ and $P^T = P$ is an orthogonal projection matrix.

**Formula for arbitrary (not orthonormal) basis.** If the columns of $A$ form a basis for $W$ (not necessarily orthonormal):
$$P_W = A(A^TA)^{-1}A^T \quad (\text{when } A \text{ has full column rank}).$$

*Derivation.* The projection $\hat{v} = Ax^*$ where $x^*$ minimizes $\|Ax - v\|^2$. The normal equations are $A^TAx^* = A^Tv$, giving $x^* = (A^TA)^{-1}A^Tv$ and $\hat{v} = A(A^TA)^{-1}A^Tv = P_Wv$.

## Application: Least Squares

For an overdetermined system $Ax \approx b$ (more equations than unknowns), the least-squares solution minimizes $\|Ax - b\|^2$:
$$\hat{x} = (A^TA)^{-1}A^Tb \quad (\text{when } A \text{ has full column rank}).$$

The residual is $r = b - A\hat{x} = b - P_{\text{col}(A)}b = P_{\ker A^T}b \perp \text{col}(A)$.

**Normal equations:** $A^TA\hat{x} = A^Tb$ — these are the equations $A^Tr = 0$ expressing the orthogonality of the residual to the column space.

## Connection to Fourier Series

The Fourier series of a function $f \in L^2([-\pi,\pi])$ is exactly the projection of $f$ onto the closed subspace spanned by $\{\cos(nx), \sin(nx)\}_{n\geq 0}$:
$$\hat{f} = \frac{a_0}{2} + \sum_{n=1}^\infty (a_n\cos(nx) + b_n\sin(nx)), \quad a_n = \frac{1}{\pi}\int_{-\pi}^\pi f(x)\cos(nx)\,dx.$$

Each coefficient is an inner product $a_n = \langle f, \cos(nx)\rangle / \|\cos(nx)\|^2$, exactly the projection formula. The fact that $\hat{f} = f$ (i.e., the Fourier series converges to $f$ in $L^2$) is the statement that the trigonometric system is a complete orthonormal basis for $L^2([-\pi,\pi])$.
