# Bases and Dimension

A basis for a vector space is a linearly independent set that spans the entire space. It provides a coordinate system: once a basis is chosen, every vector can be written uniquely as a linear combination of the basis vectors, and the coefficients of that combination serve as coordinates. The number of basis elements is the dimension of the space — a measure of its "size" that is independent of the particular basis chosen.

## Definition of a Basis

**Definition.** A set $\mathcal{B} = \{v_1, v_2, \ldots, v_n\} \subseteq V$ is a **basis** for $V$ if:
1. $\text{span}(\mathcal{B}) = V$ (spanning condition).
2. $\mathcal{B}$ is linearly independent.

Equivalently, $\mathcal{B}$ is a basis iff every vector $v \in V$ can be written in exactly one way as a linear combination $v = \alpha_1 v_1 + \cdots + \alpha_n v_n$.

*Proof of uniqueness.* If $v = \sum \alpha_i v_i = \sum \beta_i v_i$, then $\sum (\alpha_i - \beta_i)v_i = \mathbf{0}$, so by independence $\alpha_i = \beta_i$ for all $i$. $\square$

**Examples:**
- Standard basis of $\mathbb{R}^n$: $\{e_1, \ldots, e_n\}$ where $e_i$ has $1$ in position $i$ and $0$ elsewhere.
- $\{1, x, x^2, \ldots, x^n\}$ is a basis for $P_n$.
- $\{e^{r_1 x}, e^{r_2 x}, \ldots, e^{r_n x}\}$ for distinct $r_i$ is a basis for the solution space of $\sum a_k y^{(k)} = 0$ when its characteristic polynomial has roots $r_1, \ldots, r_n$.

## The Basis Theorem

**Theorem.** Let $V$ be a vector space. If $V$ has a basis with $n$ elements, then:
1. Every basis for $V$ has exactly $n$ elements.
2. Any spanning set with $n$ elements is a basis.
3. Any linearly independent set with $n$ elements is a basis.
4. Any linearly independent set can be extended to a basis.
5. Any spanning set contains a basis as a subset.

The number $n$ is called the **dimension** of $V$, written $\dim V = n$. If no finite basis exists, $V$ is infinite-dimensional.

*Proof of (1) sketch.* Suppose $\{u_1, \ldots, u_m\}$ and $\{v_1, \ldots, v_n\}$ are both bases. The first basis spans $V$, so each $v_j$ is a linear combination of the $u_i$. A linear algebra argument (the "exchange" or "replacement" lemma) shows $m \geq n$. By symmetry, $n \geq m$. $\square$

## Computing Coordinates

Given a basis $\mathcal{B} = \{v_1, \ldots, v_n\}$ of $V$, any $v \in V$ has a unique expression $v = \sum \alpha_i v_i$. The **coordinate vector** of $v$ with respect to $\mathcal{B}$ is $[v]_\mathcal{B} = (\alpha_1, \ldots, \alpha_n) \in \mathbb{R}^n$.

**Example.** In $P_2$ with basis $\mathcal{B} = \{1, x, x^2\}$: $[3 - 2x + x^2]_\mathcal{B} = (3, -2, 1)$.

**Example.** In $P_2$ with basis $\mathcal{C} = \{1, x-1, (x-1)^2\}$ (Taylor basis at $x=1$): $3 - 2x + x^2 = 3 - 2(1 + (x-1)) + (1 + (x-1))^2 = 3 - 2 - 2(x-1) + 1 + 2(x-1) + (x-1)^2 = 2 + 0\cdot(x-1) + (x-1)^2$. So $[3-2x+x^2]_\mathcal{C} = (2, 0, 1)$.

Different bases give different coordinates for the same vector, but the space is the same.

## Dimension and Subspaces

**Theorem.** If $W$ is a subspace of $V$ (finite-dimensional), then $\dim W \leq \dim V$, with equality iff $W = V$.

**Theorem.** $\dim(W_1 + W_2) = \dim W_1 + \dim W_2 - \dim(W_1 \cap W_2)$.

This dimension formula is the vector space analog of the inclusion-exclusion principle for sets.

## Infinite-Dimensional Spaces

The vector spaces most important for differential equations — $C([a,b])$, $L^2([a,b])$, the space of all solutions to $y^{(n)} = 0$ extended to all orders — are infinite-dimensional. Every finite set fails to span them. However, many results from the finite-dimensional theory generalize when suitably adapted.

For example, a Fourier series expresses an $L^2$ function as an infinite linear combination of basis functions $\{1, \sin(nx), \cos(nx)\}$ (or complex exponentials $\{e^{inx}\}$). This is an "orthonormal basis" in the sense of an inner product space, and the Fourier coefficients are the "coordinates" — but convergence must be handled carefully.

## Basis for ODE Solution Spaces

The existence and uniqueness theorem guarantees that the solution space of $L[y] = 0$ ($n$-th order, on an interval $I$ where the coefficients are continuous) has dimension exactly $n$. Any collection of $n$ linearly independent solutions forms a basis — a **fundamental system** — and the general solution is:
$$y(t) = c_1 y_1(t) + \cdots + c_n y_n(t).$$

The initial conditions $y(t_0) = y_0, y'(t_0) = y_0', \ldots, y^{(n-1)}(t_0) = y_0^{(n-1)}$ uniquely determine $c_1, \ldots, c_n$ (the coordinates of the solution in the basis $\{y_1, \ldots, y_n\}$).

## Change of Basis

If $\mathcal{B}$ and $\mathcal{C}$ are two bases for $V$, the **change-of-basis matrix** $P$ relates coordinates: $[v]_\mathcal{C} = P[v]_\mathcal{B}$. This matrix is invertible. Change of basis is the linear algebra behind the substitution methods for ODEs: choosing a basis of normal modes (eigenvectors) for a linear system simplifies the ODE into decoupled equations.

## Common Pitfalls

**Assuming a spanning set is a basis.** A spanning set might contain redundant elements (linearly dependent vectors). Remove them to get a basis.

**Assuming a linearly independent set spans.** A linearly independent set might not span the whole space — it is a basis for its span, not necessarily for $V$.

**Misidentifying dimension.** $P_n$ has dimension $n+1$ (it is spanned by $1, x, \ldots, x^n$, a set of $n+1$ elements). The space of $m \times n$ matrices has dimension $mn$.
