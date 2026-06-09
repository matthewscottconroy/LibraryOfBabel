# The Rank-Nullity Theorem

The Rank-Nullity Theorem is one of the fundamental structural results of linear algebra. It says that for any linear map between finite-dimensional vector spaces, the "amount lost" (captured by the nullity, the dimension of the kernel) plus the "amount produced" (captured by the rank, the dimension of the image) equals the dimension of the domain. This single equation encodes the conservation of dimension under linear maps.

## Statement and Proof

**Theorem (Rank-Nullity Theorem).** Let $T: V \to W$ be a linear map with $V$ finite-dimensional. Then
$$\dim \ker T + \dim \text{im}\, T = \dim V,$$
or equivalently, $\text{null}(T) + \text{rank}(T) = \dim V$.

*Proof.* Let $\dim V = n$ and $\dim \ker T = k$. Choose a basis $\{u_1, \ldots, u_k\}$ for $\ker T$. Extend it (by the basis extension theorem) to a basis $\{u_1, \ldots, u_k, v_1, \ldots, v_{n-k}\}$ of $V$.

**Claim:** $\{T(v_1), \ldots, T(v_{n-k})\}$ is a basis for $\text{im}\, T$.

**Spanning.** Any $w \in \text{im}\, T$ has the form $T(v)$ for some $v \in V$. Write $v = \sum_{i=1}^k \alpha_i u_i + \sum_{j=1}^{n-k} \beta_j v_j$. Then:
$$w = T(v) = \sum_i \alpha_i T(u_i) + \sum_j \beta_j T(v_j) = \sum_j \beta_j T(v_j),$$
since $T(u_i) = \mathbf{0}$. So $\{T(v_j)\}$ spans $\text{im}\, T$.

**Independence.** Suppose $\sum_{j=1}^{n-k} \beta_j T(v_j) = \mathbf{0}$. Then $T\left(\sum_j \beta_j v_j\right) = \mathbf{0}$, so $\sum_j \beta_j v_j \in \ker T$. Write $\sum_j \beta_j v_j = \sum_i \alpha_i u_i$. Then $\sum_j \beta_j v_j - \sum_i \alpha_i u_i = \mathbf{0}$. Since $\{u_1, \ldots, u_k, v_1, \ldots, v_{n-k}\}$ is a basis of $V$, all coefficients are zero: $\beta_j = 0$ for all $j$.

So $\{T(v_1), \ldots, T(v_{n-k})\}$ is a linearly independent spanning set for $\text{im}\, T$, hence a basis. Therefore $\dim \text{im}\, T = n - k$, and $k + (n-k) = n$. $\square$

## Consequences and Applications

**Criterion for injectivity.** $T$ is injective iff $\text{null}(T) = 0$ iff $\text{rank}(T) = \dim V$.

**Criterion for surjectivity.** $T: V \to W$ is surjective iff $\text{rank}(T) = \dim W$.

**Square matrices.** If $T: V \to V$ with $\dim V = n$, then $T$ is injective iff it is surjective iff it is an isomorphism. For an $n \times n$ matrix $A$, the following are equivalent: $A$ is invertible; $\ker A = \{\mathbf{0}\}$; $\text{rank}(A) = n$; $\det A \neq 0$; $Ax = b$ has a unique solution for every $b$.

**Linear systems.** The equation $Ax = b$ where $A$ is $m \times n$:
- Has a solution iff $b \in \text{im}\, A$ iff $\text{rank}(A) = \text{rank}(A|b)$ (augmented matrix).
- Has a unique solution iff also $\ker A = \{\mathbf{0}\}$, i.e., $\text{rank}(A) = n$.
- Has infinitely many solutions (affine subspace of dimension $n - \text{rank}(A)$) if consistent but $\text{rank}(A) < n$.

**ODE application.** For the $n$-th order linear ODE $L[y] = 0$ on an interval $I$ (where the coefficients are continuous), the operator $L: C^n(I) \to C(I)$ has $\ker L$ as the solution space. The existence and uniqueness theorem says $\dim \ker L = n$. This is consistent with Rank-Nullity in the following sense: the map from $\ker L$ to initial data $(y(t_0), y'(t_0), \ldots, y^{(n-1)}(t_0)) \in \mathbb{R}^n$ is a bijective linear map (by the existence and uniqueness theorem), so $\dim \ker L = n$.

## The Four Fundamental Subspaces

For an $m \times n$ matrix $A$ (and its transpose $A^T$), the Rank-Nullity Theorem applied twice gives four fundamental subspaces:
1. **Column space** $\text{im}\, A \subseteq \mathbb{R}^m$, dimension $r = \text{rank}(A)$.
2. **Null space** $\ker A \subseteq \mathbb{R}^n$, dimension $n - r$.
3. **Row space** $\text{im}\, A^T \subseteq \mathbb{R}^n$, dimension $r$.
4. **Left null space** $\ker A^T \subseteq \mathbb{R}^m$, dimension $m - r$.

Key fact: the null space of $A$ is the orthogonal complement of the row space of $A$ in $\mathbb{R}^n$. This "Fundamental Theorem of Linear Algebra" (Strang's terminology) organizes the solvability of $Ax = b$: $b \in \text{im}\, A$ iff $b$ is orthogonal to $\ker A^T$.

## Rank-Nullity in Infinite Dimensions

For operators on infinite-dimensional spaces, the Rank-Nullity Theorem need not hold as stated (dimensions may be infinite). However, an analogous statement holds for Fredholm operators (compact perturbations of the identity), where the kernel is finite-dimensional, the image is closed and has finite codimension, and the "index" $\dim \ker T - \text{codim}\, \text{im}\, T$ is invariant under compact perturbations. This Fredholm theory is the operator-theoretic framework for proving existence of solutions to boundary value problems.

## Common Pitfalls

**Applying to non-square matrices without care.** If $A$ is $m \times n$ with $m \neq n$, $T$ cannot be both injective and surjective. Rank-Nullity says $\text{rank}(A) \leq \min(m,n)$.

**Forgetting that $\dim \ker T + \dim \text{im}\, T = \dim V$, not $\dim W$.** The theorem involves $\dim V$ on the right, the dimension of the domain. This is easy to mis-state when domain and codomain have different dimensions.
