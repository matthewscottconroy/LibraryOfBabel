# Definition and Examples of Linear Maps

A function between two sets can have any behavior — it might mix, fold, stretch, or permute its domain in complicated ways. A linear map is constrained to behave in the simplest possible way consistent with the algebraic structure of vector spaces: it must turn linear combinations into the corresponding linear combinations of outputs. This constraint is both restrictive (linear maps are completely determined by their values on a basis) and powerful (the resulting theory is entirely algebraic).

## Definition

**Definition.** Let $V$ and $W$ be vector spaces over the same field $F$. A function $T: V \to W$ is a **linear map** (or **linear transformation**) if for all $u, v \in V$ and $\alpha \in F$:
1. $T(u + v) = T(u) + T(v)$ (additivity)
2. $T(\alpha v) = \alpha T(v)$ (homogeneity)

These two conditions are equivalent to the single condition: $T(\alpha u + \beta v) = \alpha T(u) + \beta T(v)$ for all $u, v \in V$ and $\alpha, \beta \in F$.

**Immediate consequences:**
- $T(\mathbf{0}_V) = \mathbf{0}_W$: apply additivity with $u = v = \mathbf{0}$.
- $T(-v) = -T(v)$: apply homogeneity with $\alpha = -1$.
- $T\left(\sum_{i=1}^k \alpha_i v_i\right) = \sum_{i=1}^k \alpha_i T(v_i)$: finite induction on additivity and homogeneity.

## Basic Examples

**Matrix-vector multiplication.** For any $m \times n$ matrix $A$, the map $T: \mathbb{R}^n \to \mathbb{R}^m$ defined by $T(x) = Ax$ is linear:
$$A(\alpha x + \beta y) = \alpha Ax + \beta Ay.$$
Every linear map between finite-dimensional vector spaces is of this form (after choosing bases).

**Differentiation.** $D: C^1(I) \to C(I)$, $D(f) = f'$:
$$D(\alpha f + \beta g) = (\alpha f + \beta g)' = \alpha f' + \beta g' = \alpha D(f) + \beta D(g).$$

**Integration.** $I: C([a,b]) \to \mathbb{R}$, $I(f) = \int_a^b f(x)\,dx$:
$$I(\alpha f + \beta g) = \alpha \int_a^b f + \beta \int_a^b g = \alpha I(f) + \beta I(g).$$
(Or $I: C([a,b]) \to C([a,b])$, $I(f)(x) = \int_a^x f(t)\,dt$.)

**Rotation in $\mathbb{R}^2$.** $R_\theta: \mathbb{R}^2 \to \mathbb{R}^2$, rotation by angle $\theta$:
$$R_\theta\begin{pmatrix}x\\y\end{pmatrix} = \begin{pmatrix}\cos\theta & -\sin\theta\\\sin\theta & \cos\theta\end{pmatrix}\begin{pmatrix}x\\y\end{pmatrix}.$$
Linear because it is matrix multiplication.

**Projection.** $P: \mathbb{R}^3 \to \mathbb{R}^3$, projection onto the $xy$-plane: $P(x,y,z) = (x,y,0)$. Linear: $P(\alpha u + \beta v) = \alpha P(u) + \beta P(v)$.

**Evaluation.** $\text{ev}_a: C([a,b]) \to \mathbb{R}$, $\text{ev}_a(f) = f(a)$. Linear: $\text{ev}_a(\alpha f + \beta g) = \alpha f(a) + \beta g(a)$.

**Differential operators.** $L: C^2(I) \to C(I)$, $L(y) = y'' + py' + qy$ for fixed continuous $p, q$:
$$L(\alpha y_1 + \beta y_2) = (\alpha y_1 + \beta y_2)'' + p(\alpha y_1 + \beta y_2)' + q(\alpha y_1 + \beta y_2) = \alpha L(y_1) + \beta L(y_2).$$

## Linear Maps Are Determined by Their Values on a Basis

**Theorem.** Let $V$ be finite-dimensional with basis $\{v_1, \ldots, v_n\}$, and let $W$ be any vector space. For any choice of vectors $w_1, \ldots, w_n \in W$, there is a unique linear map $T: V \to W$ with $T(v_i) = w_i$ for $i = 1, \ldots, n$.

*Proof.* **Existence:** Define $T\left(\sum \alpha_i v_i\right) = \sum \alpha_i w_i$. This is well-defined because every vector in $V$ has a unique representation in the basis. Check: $T(\alpha u + \beta v) = \alpha T(u) + \beta T(v)$ follows from the definition.

**Uniqueness:** If $S$ is another linear map with $S(v_i) = w_i$, then for any $v = \sum \alpha_i v_i$: $S(v) = \sum \alpha_i S(v_i) = \sum \alpha_i w_i = T(v)$. $\square$

This theorem is the reason that matrices represent linear maps: once a basis is fixed for both the domain and codomain, the matrix is just the table of values $T(v_i)$ expressed in the codomain basis.

## Compositions and Inverses

If $T: V \to W$ and $S: W \to X$ are linear, their composition $S \circ T: V \to X$ is linear:
$$(S \circ T)(\alpha u + \beta v) = S(T(\alpha u + \beta v)) = S(\alpha T(u) + \beta T(v)) = \alpha S(T(u)) + \beta S(T(v)).$$

A linear map $T: V \to W$ is an **isomorphism** if it is bijective (one-to-one and onto). An isomorphism identifies $V$ and $W$ as "the same" vector space. Any $n$-dimensional vector space over $F$ is isomorphic to $F^n$: the isomorphism sends each vector to its coordinate vector with respect to a chosen basis.

## Non-Linear Examples (for Contrast)

$T(x,y) = (x+1, y)$ is not linear: $T(\mathbf{0}) = (1,0) \neq \mathbf{0}$.

$T(f) = f^2$ is not linear: $T(f+g) = (f+g)^2 \neq f^2 + g^2 = T(f) + T(g)$ in general.

$T(f) = |f|$ is not linear: $T(-f) = |-f| = |f| = T(f)$, but $T(-f)$ should equal $-T(f)$ for linearity.

## Common Pitfalls

**Checking only one condition.** Both additivity and homogeneity must be verified independently, or the single combined condition checked.

**Confusing linear maps with linear functions.** A linear function $y = mx + b$ with $b \neq 0$ is not a linear map (it does not send $0$ to $0$). It is an affine map.

**Assuming linearity of common operations.** The absolute value, maximum, and squaring operations on functions are not linear. Recognizing linearity requires checking the definition.
