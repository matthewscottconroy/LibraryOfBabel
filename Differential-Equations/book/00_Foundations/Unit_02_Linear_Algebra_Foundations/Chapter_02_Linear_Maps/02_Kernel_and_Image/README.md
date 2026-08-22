# Kernel and Image

Every linear map $T: V \to W$ naturally defines two important subspaces: one in the domain $V$ (capturing what gets "lost" by $T$) and one in the codomain $W$ (capturing what gets "produced" by $T$). These are the kernel and the image, and together they determine the structural properties of $T$.

## Definitions

**Definition.** For a linear map $T: V \to W$:
- The **kernel** of $T$ is $\ker T = \{v \in V : T(v) = \mathbf{0}_W\}$.
- The **image** (or range) of $T$ is $\text{im}\, T = \{T(v) : v \in V\} = T(V)$.

**Theorem.** $\ker T$ is a subspace of $V$ and $\text{im}\, T$ is a subspace of $W$.

*Proof.* **Kernel:** $T(\mathbf{0}) = \mathbf{0}$, so $\mathbf{0} \in \ker T$. If $T(u) = T(v) = \mathbf{0}$, then $T(\alpha u + \beta v) = \alpha T(u) + \beta T(v) = \mathbf{0}$, so $\alpha u + \beta v \in \ker T$.

**Image:** $T(\mathbf{0}) = \mathbf{0} \in \text{im}\, T$. If $w_1 = T(u)$ and $w_2 = T(v)$, then $\alpha w_1 + \beta w_2 = T(\alpha u + \beta v) \in \text{im}\, T$. $\square$

The **nullity** of $T$ is $\text{null}(T) = \dim \ker T$, and the **rank** of $T$ is $\text{rank}(T) = \dim \text{im}\, T$.

## Examples

**Matrix multiplication $T(x) = Ax$.** $\ker T$ is the **null space** of $A$, the set of solutions to $Ax = \mathbf{0}$. $\text{im}\, T$ is the **column space** of $A$, the span of the columns of $A$.

**Differentiation $D: P_n \to P_{n-1}$.** $D(p) = p'$. $\ker D = \{p \in P_n : p' = 0\} = \{c : c \in \mathbb{R}\} = P_0$ (the constant polynomials, 1-dimensional). $\text{im}\, D = P_{n-1}$ (every polynomial of degree $\leq n-1$ is the derivative of some element of $P_n$).

**Differentiation $D: C^1(I) \to C(I)$.** $\ker D =$ constants. $\text{im}\, D = C(I)$ (every continuous function is the derivative of its antiderivative, by FTC).

**The ODE operator $L[y] = y'' + py' + qy$ on $C^2(I)$.** $\ker L =$ solution space of $L[y] = 0$ (2-dimensional). $\text{im}\, L$ is a subspace of $C(I)$; the equation $L[y] = f$ has a solution iff $f \in \text{im}\, L$.

## Injectivity and Surjectivity

A linear map $T: V \to W$ is:
- **Injective** (one-to-one) iff $\ker T = \{\mathbf{0}\}$.
- **Surjective** (onto) iff $\text{im}\, T = W$.

*Proof of injectivity criterion.* ($\Rightarrow$) If $T$ is injective and $T(v) = \mathbf{0} = T(\mathbf{0})$, then $v = \mathbf{0}$. ($\Leftarrow$) If $\ker T = \{\mathbf{0}\}$ and $T(u) = T(v)$, then $T(u-v) = \mathbf{0}$, so $u - v \in \ker T = \{\mathbf{0}\}$, giving $u = v$. $\square$

**Example.** The differentiation operator $D: C^1 \to C$ is not injective ($\ker D$ contains all constants). The integration operator $I: C([a,b]) \to C([a,b])$, $I(f)(x) = \int_a^x f(t)\,dt$, satisfies $I(f)(a) = 0$ for all $f$, so $\ker I = \{\mathbf{0}\}$ — it is injective. But $\text{im}\, I = \{g \in C^1([a,b]) : g(a) = 0\}$, which is a proper subspace of $C([a,b])$, so $I$ is not surjective.

## The Inverse of an Injective Map

If $T: V \to W$ is linear and injective, then $T: V \to \text{im}\, T$ is an isomorphism (bijective linear map). When $V$ is finite-dimensional, the inverse $T^{-1}: \text{im}\,T \to V$ is also linear.

## The Pre-Image and Cosets

For $w \in \text{im}\, T$, the **pre-image** $T^{-1}(\{w\})$ is the set of all $v \in V$ with $T(v) = w$. This set is an affine subspace:

**Theorem.** If $T(v_0) = w$, then $T^{-1}(\{w\}) = v_0 + \ker T = \{v_0 + k : k \in \ker T\}$.

*Proof.* If $T(v) = w = T(v_0)$, then $T(v - v_0) = \mathbf{0}$, so $v - v_0 \in \ker T$. Conversely, if $k \in \ker T$, then $T(v_0 + k) = T(v_0) + T(k) = w + \mathbf{0} = w$. $\square$

This is the linear algebra formulation of "general solution = particular solution + homogeneous solution": the pre-image of $f$ under $L$ is a particular solution $y_p$ plus the entire kernel $\ker L$.

## Kernel and Image for Matrices

For an $m \times n$ matrix $A$:
- $\ker A =$ null space of $A$ = solutions to $Ax = 0 \subseteq \mathbb{R}^n$.
- $\text{im}\, A =$ column space of $A$ = $\text{span}(\text{columns of }A) \subseteq \mathbb{R}^m$.
- $\text{rank}(A) =$ number of pivot columns in row-reduced form = $\dim \text{im}\, A$.
- $\text{null}(A) = n - \text{rank}(A)$ (by Rank-Nullity Theorem, next section).

The equation $Ax = b$ has a solution iff $b \in \text{im}\, A$ (i.e., $b$ is in the column space). The solution is unique iff $\ker A = \{\mathbf{0}\}$, i.e., $\text{null}(A) = 0$.

## Common Pitfalls

**Confusing kernel and image.** $\ker T \subseteq V$ (domain); $\text{im}\, T \subseteq W$ (codomain). These live in different spaces.

**Assuming $T^{-1}(\{w\})$ is a subspace.** It is an affine subspace (coset), not a subspace, unless $w = \mathbf{0}$.

**Equating surjectivity with a "complete" image.** $\text{im}\, T = W$ only when $T$ is surjective. For many important linear maps (including many differential operators), $\text{im}\, T$ is a proper subspace, and $\text{im}\, T = W$ fails.
