# Subspaces

A subspace is a subset of a vector space that is itself a vector space under the same operations. Rather than verifying all eight axioms, one can use a much more efficient criterion: a nonempty subset is a subspace iff it is closed under addition and scalar multiplication. This streamlined test makes subspaces easy to recognize and work with in practice.

## The Subspace Criterion

**Theorem.** Let $V$ be a vector space over $F$ and $W \subseteq V$ a nonempty subset. Then $W$ is a subspace of $V$ if and only if:
1. For all $u, w \in W$: $u + w \in W$ (closed under addition).
2. For all $\alpha \in F$ and $w \in W$: $\alpha w \in W$ (closed under scalar multiplication).

*Proof.* If $W$ is a subspace, it satisfies the axioms by definition, so (1) and (2) hold. Conversely, if (1) and (2) hold, then all eight axioms for $W$ follow: commutativity and associativity are inherited from $V$; taking $\alpha = 0$ in (2) gives $\mathbf{0} = 0\cdot w \in W$ (so the zero vector is present); taking $\alpha = -1$ gives $-w \in W$ (additive inverses exist); the remaining axioms follow from $V$. $\square$

A combined form: $W$ is a subspace iff $W$ is nonempty and $\alpha u + \beta w \in W$ for all $u, w \in W$ and $\alpha, \beta \in F$.

## Examples of Subspaces

**Lines and planes through the origin in $\mathbb{R}^n$.** The set $\{x \in \mathbb{R}^n : Ax = \mathbf{0}\}$ (null space of $A$) is a subspace: if $Ax = 0$ and $Ay = 0$, then $A(\alpha x + \beta y) = \alpha Ax + \beta Ay = 0$.

**$C^k([a,b]) \subseteq C([a,b])$.** Functions with $k$ continuous derivatives form a subspace of continuous functions.

**Polynomials of even degree $\subseteq P_n$.** Wait — this is not a subspace: the sum of two even-degree polynomials need not have even degree. But $\{p \in P_n : p(0) = 0\}$ is a subspace: if $p(0) = 0$ and $q(0) = 0$, then $(\alpha p + \beta q)(0) = 0$.

**Solution spaces of linear ODEs.** The set of solutions to $L[y] = 0$ on $I$ is a subspace of $C^n(I)$, verified by the linearity of $L$.

## Non-Subspaces

**The set $W = \{(x,y) \in \mathbb{R}^2 : x \geq 0\}$.** Contains $\mathbf{0}$, closed under addition. But $2 \cdot (1, 0) = (2,0) \in W$ while $(-1)(1,0) = (-1, 0) \notin W$: not closed under scalar multiplication.

**The set $W = \{y \in C([-1,1]) : y(0) = 1\}$.** Contains $y \equiv 1$, but $2 \cdot 1 = 2 \notin W$ (since $2(0) = 2 \neq 1$). Also $\mathbf{0} \notin W$.

## Intersection and Sum of Subspaces

**Theorem.** If $W_1$ and $W_2$ are subspaces of $V$, then $W_1 \cap W_2$ is a subspace of $V$.

*Proof.* $\mathbf{0} \in W_1 \cap W_2$ (nonempty). If $u, v \in W_1 \cap W_2$, then $u+v \in W_1$ (since $W_1$ is a subspace) and $u+v \in W_2$, so $u+v \in W_1 \cap W_2$. Similarly for scalar multiplication. $\square$

The union $W_1 \cup W_2$ is generally not a subspace. Consider $W_1 = \{(x,0)\}$ and $W_2 = \{(0,y)\}$ in $\mathbb{R}^2$: $(1,0) \in W_1$ and $(0,1) \in W_2$ but $(1,0)+(0,1) = (1,1) \notin W_1 \cup W_2$.

The **sum** $W_1 + W_2 = \{w_1 + w_2 : w_1 \in W_1, w_2 \in W_2\}$ is always a subspace, and is the smallest subspace containing both $W_1$ and $W_2$. When $W_1 \cap W_2 = \{\mathbf{0}\}$, the sum is a **direct sum**, written $W_1 \oplus W_2$, and every element of $W_1 \oplus W_2$ has a unique decomposition $w_1 + w_2$.

## Span

The **span** of a set $S = \{v_1, \ldots, v_k\} \subseteq V$ is
$$\text{span}(S) = \left\{\sum_{i=1}^k \alpha_i v_i : \alpha_i \in F\right\}.$$

**Theorem.** $\text{span}(S)$ is the smallest subspace of $V$ containing $S$.

*Proof.* $\text{span}(S)$ is a subspace (closed under linear combinations). Any subspace containing $S$ must contain all linear combinations of elements of $S$, hence must contain $\text{span}(S)$. $\square$

**Example.** In $P_3$ (polynomials of degree $\leq 3$), $\text{span}\{1, x\} = \{a + bx : a, b \in \mathbb{R}\} = P_1$, the subspace of linear polynomials.

**Example.** The solution space of $y'' - y = 0$ on $\mathbb{R}$ is $\text{span}\{e^x, e^{-x}\} = \{c_1 e^x + c_2 e^{-x}\}$ (or equivalently $\text{span}\{\cosh x, \sinh x\}$).

## Affine Subspaces and Non-Homogeneous Systems

A set of the form $\{v_0 + w : w \in W\}$ where $W$ is a subspace and $v_0$ is a fixed vector is called an **affine subspace** or **coset** of $W$. It is not a subspace (it does not contain $\mathbf{0}$ unless $v_0 \in W$), but it has the same "shape" as $W$.

The solution set of a non-homogeneous linear equation $L[y] = f$ (with $f \neq 0$) is an affine subspace: it is a particular solution $y_p$ plus an arbitrary element of the null space $\ker L$. This is the structure of "general solution = particular solution + homogeneous solution" in ODE theory.

## Common Pitfalls

**Checking only one condition.** Both closure under addition and closure under scalar multiplication must be verified. A set might be closed under addition but not scalar multiplication (the positive orthant in $\mathbb{R}^2$), or closed under scalar multiplication but not addition (the axes in $\mathbb{R}^2$ together, but this fails closure under addition).

**Forgetting to check nonemptiness.** The empty set satisfies the closure conditions vacuously but is not a subspace. One quick way to check nonemptiness: verify $\mathbf{0} \in W$.
