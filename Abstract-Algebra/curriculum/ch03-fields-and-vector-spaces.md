# Chapter 3 — Fields and Vector Spaces

**Part II: Linear Algebra**
*Prerequisites: [Chapter 1](ch01-logic-sets-proof.md), [Chapter 2](ch02-relations-functions-cardinality.md)*
*Next: [Chapter 4 — Linear Maps and Matrices](ch04-linear-maps-and-matrices.md)*

---

## Learning Objectives

- Define fields axiomatically; recognize $\mathbb{Q}, \mathbb{R}, \mathbb{C}, \mathbb{F}_p$ as fields
- Define vector spaces over arbitrary fields and verify the axioms
- Understand span, linear independence, and bases rigorously
- Prove that any two bases of a vector space have the same cardinality (dimension theorem)
- Compute with subspaces, sums, and direct sums
- Internalize why the abstract definition generalizes and unifies geometry, function spaces, and polynomial spaces

---

## 3.1 Fields

### 3.1.1 Definition

A **field** is a set $F$ with two operations $+$ (addition) and $\cdot$ (multiplication) satisfying:

**Additive structure $(F, +)$:**
- (A1) Closure: $a + b \in F$
- (A2) Associativity: $(a+b)+c = a+(b+c)$
- (A3) Identity: $\exists 0 \in F,\, a + 0 = a$
- (A4) Inverses: $\forall a,\, \exists {-a},\, a + (-a) = 0$
- (A5) Commutativity: $a + b = b + a$

**Multiplicative structure $(F \setminus \{0\}, \cdot)$:**
- (M1–M5) Same axioms with $1$ as identity

**Distributivity:** $a \cdot (b + c) = a \cdot b + a \cdot c$

So $(F, +)$ is an abelian group, $(F \setminus \{0\}, \cdot)$ is an abelian group, and the two are linked by distributivity.

### 3.1.2 Examples

- $\mathbb{Q}$, $\mathbb{R}$, $\mathbb{C}$ — the classical fields of rationals, reals, complexes
- $\mathbb{F}_p = \mathbb{Z}/p\mathbb{Z}$ for prime $p$ — the field of $p$ elements
- $\mathbb{F}_{p^n}$ — the finite field with $p^n$ elements (exists and is unique up to isomorphism for each prime power $p^n$)
- $\mathbb{Q}(\sqrt{2}) = \{a + b\sqrt{2} \mid a, b \in \mathbb{Q}\}$ — a field extension

**Non-examples:** $\mathbb{Z}$ (no multiplicative inverses for non-units), $\mathbb{Z}/6\mathbb{Z}$ (has zero divisors: $2 \cdot 3 = 0$)

### 3.1.3 Basic Consequences of the Axioms

From the axioms alone, prove:
- $0 \cdot a = 0$ for all $a$
- $(-1) \cdot a = -a$
- If $ab = 0$ then $a = 0$ or $b = 0$ (fields have no zero divisors)
- The additive and multiplicative identities are unique
- Additive and multiplicative inverses are unique

These are not assumed — they are derived.

---

## 3.2 Vector Spaces

### 3.2.1 Definition

A **vector space** over a field $F$ is a set $V$ with:
- A binary operation $+: V \times V \to V$ (**vector addition**)
- A scalar multiplication $\cdot: F \times V \to V$

Satisfying:
- (V1) $(V, +)$ is an abelian group
- (V2) $1 \cdot \mathbf{v} = \mathbf{v}$
- (V3) $(ab) \cdot \mathbf{v} = a \cdot (b \cdot \mathbf{v})$
- (V4) $a \cdot (\mathbf{u} + \mathbf{v}) = a\mathbf{u} + a\mathbf{v}$
- (V5) $(a + b) \cdot \mathbf{v} = a\mathbf{v} + b\mathbf{v}$

Elements of $V$ are **vectors**; elements of $F$ are **scalars**.

### 3.2.2 Examples

| Space | Vectors | Field | Notes |
|-------|---------|-------|-------|
| $F^n$ | $n$-tuples $(a_1, \ldots, a_n)$ | $F$ | The standard model |
| $M_{m \times n}(F)$ | $m \times n$ matrices | $F$ | |
| $F[x]$ | Polynomials with coefficients in $F$ | $F$ | Infinite-dimensional |
| $F[x]_{\leq n}$ | Polynomials of degree $\leq n$ | $F$ | Dimension $n+1$ |
| $\mathcal{C}([a,b], \mathbb{R})$ | Continuous functions $[a,b] \to \mathbb{R}$ | $\mathbb{R}$ | Infinite-dimensional |
| $\{0\}$ | Just the zero vector | $F$ | The zero space, dimension 0 |

**Key insight:** The abstract definition captures all of these simultaneously. A theorem proved for abstract vector spaces applies to all of them.

### 3.2.3 Elementary Consequences

From the axioms:
- $0 \cdot \mathbf{v} = \mathbf{0}$ (scalar zero times any vector is zero vector)
- $a \cdot \mathbf{0} = \mathbf{0}$ (any scalar times zero vector is zero vector)
- $(-1)\mathbf{v} = -\mathbf{v}$
- If $a\mathbf{v} = \mathbf{0}$ then $a = 0$ or $\mathbf{v} = \mathbf{0}$

---

## 3.3 Subspaces

### 3.3.1 Definition and Subspace Test

A non-empty subset $W \subseteq V$ is a **subspace** if $W$ is itself a vector space under the inherited operations.

**Subspace Test (three conditions):**
1. $\mathbf{0} \in W$
2. $\mathbf{u}, \mathbf{v} \in W \Rightarrow \mathbf{u} + \mathbf{v} \in W$ (closed under addition)
3. $a \in F,\, \mathbf{v} \in W \Rightarrow a\mathbf{v} \in W$ (closed under scalar multiplication)

Conditions 2 and 3 together: $a\mathbf{u} + b\mathbf{v} \in W$ for all $a, b \in F$, $\mathbf{u}, \mathbf{v} \in W$.

### 3.3.2 Examples of Subspaces

- In $\mathbb{R}^3$: lines and planes through the origin; $\{\mathbf{0}\}$; all of $\mathbb{R}^3$
- In $F[x]$: the subspace of polynomials of degree $\leq n$; the subspace of even polynomials
- The kernel of a linear map (anticipated from Chapter 4)
- The set of symmetric matrices inside $M_n(F)$

### 3.3.3 Sums and Direct Sums

For subspaces $U, W \subseteq V$:
$$U + W = \{u + w \mid u \in U, w \in W\}$$

This is a subspace. If additionally $U \cap W = \{0\}$, we say $V = U \oplus W$ is a **direct sum**, and every $v \in V$ decomposes uniquely as $v = u + w$.

**External direct sum:** $U \oplus W$ can also be defined as $U \times W$ with componentwise operations, without reference to a containing space $V$.

**Modular law (Dedekind):** For subspaces $A, B, C$ with $A \subseteq B$:
$$B \cap (A + C) = A + (B \cap C)$$

---

## 3.4 Linear Independence, Span, and Bases

### 3.4.1 Linear Combinations and Span

A **linear combination** of vectors $\mathbf{v}_1, \ldots, \mathbf{v}_k \in V$ is any vector $a_1\mathbf{v}_1 + \cdots + a_k\mathbf{v}_k$ with $a_i \in F$.

The **span** of a set $S \subseteq V$:
$$\mathrm{span}(S) = \left\{\sum_{i=1}^k a_i \mathbf{v}_i \;\middle|\; k \geq 0,\, \mathbf{v}_i \in S,\, a_i \in F\right\}$$

$\mathrm{span}(S)$ is the smallest subspace containing $S$. It is the intersection of all subspaces containing $S$.

$V = \mathrm{span}(S)$ means $S$ **spans** $V$ (or $S$ is a **spanning set**).

### 3.4.2 Linear Independence

A finite list $\mathbf{v}_1, \ldots, \mathbf{v}_k$ is **linearly independent** if:
$$a_1\mathbf{v}_1 + \cdots + a_k\mathbf{v}_k = \mathbf{0} \Rightarrow a_1 = \cdots = a_k = 0$$

Equivalently: no $\mathbf{v}_i$ is a linear combination of the others.

An infinite set $S$ is linearly independent if every **finite** subset is.

**Key observations:**
- Any set containing $\mathbf{0}$ is linearly dependent
- A single non-zero vector is linearly independent
- Adding a vector in the span of existing vectors breaks independence

### 3.4.3 Basis

A **basis** of $V$ is a linearly independent spanning set.

**Equivalent characterizations:** $\mathcal{B} = \{\mathbf{b}_1, \ldots, \mathbf{b}_n\}$ is a basis iff:
- (a) $\mathcal{B}$ is linearly independent and spans $V$
- (b) $\mathcal{B}$ is a maximal linearly independent set
- (c) $\mathcal{B}$ is a minimal spanning set
- (d) Every $\mathbf{v} \in V$ is a unique linear combination of elements of $\mathcal{B}$

**Theorem (Existence):** Every vector space has a basis. (Proof uses Zorn's Lemma — see Chapter 2.)

**Standard bases:**
- $F^n$: standard basis $\mathbf{e}_1 = (1,0,\ldots,0), \ldots, \mathbf{e}_n = (0,\ldots,0,1)$
- $F[x]_{\leq n}$: $\{1, x, x^2, \ldots, x^n\}$

### 3.4.4 Dimension

**Theorem (Invariance of Dimension):** Any two bases of $V$ have the same cardinality.

*Proof sketch (finite case):* If $\mathcal{B}$ has $n$ elements and $\mathcal{C}$ has $m$ elements, both spanning, then the exchange lemma shows $m \leq n$ and $n \leq m$, so $m = n$.

The **dimension** $\dim_F V$ (or just $\dim V$) is this common cardinality.

| Space | Dimension |
|-------|-----------|
| $F^n$ | $n$ |
| $M_{m \times n}(F)$ | $mn$ |
| $F[x]_{\leq n}$ | $n+1$ |
| $F[x]$ | $\aleph_0$ |
| $\mathcal{C}([0,1], \mathbb{R})$ | $\mathfrak{c} = \|\mathbb{R}\|$ |

---

## 3.5 Coordinates and Change of Basis (Preview)

Once a basis $\mathcal{B} = (\mathbf{b}_1, \ldots, \mathbf{b}_n)$ is chosen (ordered), every $\mathbf{v} \in V$ has unique **coordinate vector**:
$$[\mathbf{v}]_\mathcal{B} = (c_1, \ldots, c_n)^T \quad \text{where } \mathbf{v} = \sum c_i \mathbf{b}_i$$

This gives a **coordinate isomorphism** $V \cong F^n$. The choice of basis is a choice of "coordinate system." Change of basis (how coordinates transform when $\mathcal{B}$ changes) is developed in Chapter 4.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Subspace Test | $W \neq \emptyset$ closed under $+$ and scalar mult. $\Rightarrow W \leq V$ |
| Spanning set contains basis | Every spanning set has a subset that is a basis |
| Extending independent sets | Every independent set extends to a basis |
| Dimension theorem | All bases have the same cardinality |
| $\dim(U+W)$ | $\dim(U+W) = \dim U + \dim W - \dim(U \cap W)$ |

---

## Milestone Exercises

1. Verify from axioms that $0 \cdot \mathbf{v} = \mathbf{0}$ for any $\mathbf{v} \in V$.

2. Show that $\mathbb{F}_2^3 = \{(a,b,c) \mid a,b,c \in \mathbb{F}_2\}$ is a vector space over $\mathbb{F}_2$ with 8 elements. Find all of its subspaces.

3. Determine which of the following are subspaces of $\mathbb{R}^3$: (a) $\{(x,y,z) \mid x+y+z=0\}$, (b) $\{(x,y,z) \mid x+y+z=1\}$, (c) $\{(x,y,z) \mid xy = 0\}$.

4. Find a basis for the solution space of $x_1 - 2x_2 + x_3 = 0$ in $\mathbb{R}^3$.

5. Prove: if $V = U \oplus W$ and $\{u_1, \ldots, u_r\}$, $\{w_1, \ldots, w_s\}$ are bases of $U$ and $W$ respectively, then $\{u_1, \ldots, u_r, w_1, \ldots, w_s\}$ is a basis of $V$.

6. In $\mathbb{R}^4$, let $U = \mathrm{span}((1,0,1,0),(0,1,0,1))$ and $W = \mathrm{span}((1,1,0,0),(0,0,1,1))$. Compute $\dim(U+W)$ and $\dim(U \cap W)$.

7. Prove the dimension formula: $\dim(U + W) = \dim U + \dim W - \dim(U \cap W)$.

---

## Connections Forward

- **Chapter 4:** Linear maps are structure-preserving maps between vector spaces; matrices arise from choosing bases.
- **Chapter 8:** Tensor products and exterior algebras are new vector spaces built from old ones.
- **Chapter 12:** Replacing $F$ with a ring $R$ gives modules — a strict generalization.
- **Chapter 18:** Representations are vector spaces with extra group action structure.

---

*Next: [Chapter 4 — Linear Maps and Matrices](ch04-linear-maps-and-matrices.md)*
