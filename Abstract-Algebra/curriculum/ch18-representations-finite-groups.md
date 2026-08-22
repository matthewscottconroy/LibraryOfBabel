# Chapter 18 — Representations of Finite Groups

**Part V: Representation Theory**
*Prerequisites: [Chapter 10](ch10-group-theory-structure.md), [Chapter 12](ch12-module-theory.md)*
*Next: [Chapter 19 — Character Theory](ch19-character-theory.md)*

---

## Learning Objectives

- Define group representations and equivalence; understand matrix representations
- Prove Maschke's theorem: complete reducibility over $\mathbb{C}$
- Decompose representations into irreducibles; understand isotypic decomposition
- Define and compute subrepresentations and quotient representations
- Understand the regular representation and its decomposition
- Recognize representations as modules over the group algebra

---

## 18.1 Representations

### 18.1.1 Definition

A **representation** of a group $G$ over a field $k$ is a group homomorphism:
$$\rho: G \to GL(V)$$

where $V$ is a $k$-vector space (the **representation space**) and $GL(V) =$ group of invertible linear maps.

The **dimension** (or **degree**) of $\rho$ is $\dim_k V$.

**Matrix representation:** Choosing a basis of $V$, identify $GL(V) \cong GL_n(k)$; each $g$ acts by an invertible matrix $\rho(g) \in GL_n(k)$.

### 18.1.2 Equivalent Formulations

A representation $\rho: G \to GL(V)$ is equivalent to:
- Making $V$ into a **$k[G]$-module** (left module over the group algebra) via $g \cdot v = \rho(g)(v)$
- A ring homomorphism $k[G] \to \mathrm{End}_k(V)$

This is the key insight: representation theory is module theory over $k[G]$.

### 18.1.3 Examples

| Group $G$ | Representation | Space $V$ |
|-----------|---------------|-----------|
| $\mathbb{Z}/n\mathbb{Z}$ | $\rho(1) = e^{2\pi i/n}$ | $\mathbb{C}^1$ |
| $S_n$ | Permutation matrices | $\mathbb{C}^n$ |
| $SO(2)$ | Rotation matrices | $\mathbb{R}^2$ |
| Any $G$ | Trivial: $\rho(g) = 1$ | $k^1$ |
| Any $G$ | Regular representation | $k[G]$ |

### 18.1.4 Morphisms of Representations

A **$G$-map** (or **intertwiner**) between representations $(V, \rho)$ and $(W, \sigma)$ is a linear map $T: V \to W$ with:
$$T \circ \rho(g) = \sigma(g) \circ T \quad \text{for all } g \in G$$

Two representations are **isomorphic** (equivalent) if a bijective $G$-map exists.

The set of $G$-maps: $\mathrm{Hom}_G(V, W) = \mathrm{Hom}_{k[G]}(V, W)$.

---

## 18.2 Constructions on Representations

### 18.2.1 Subrepresentations and Quotients

A **subrepresentation** is a subspace $W \subseteq V$ stable under all $\rho(g)$: $g \cdot w \in W$ for all $g, w$.

The **quotient representation** $V/W$ with $g \cdot (v + W) = gv + W$.

A representation is **irreducible** (or **simple**) if it has no proper non-zero subrepresentation. Irreducibles are the atoms — the building blocks.

### 18.2.2 Direct Sum and Tensor Product

$(V \oplus W, \rho \oplus \sigma)$ with $g \cdot (v, w) = (gv, gw)$.

$(V \otimes W, \rho \otimes \sigma)$ with $g \cdot (v \otimes w) = (gv) \otimes (gw)$.

**Dual representation:** $V^*$ with $(g \cdot f)(v) = f(g^{-1}v)$ for $f \in V^*$.

**Hom as representation:** $\mathrm{Hom}(V, W) \cong V^* \otimes W$ with $(g \cdot T)(v) = g \cdot T(g^{-1} v)$.

---

## 18.3 Maschke's Theorem and Complete Reducibility

### 18.3.1 Maschke's Theorem

**Theorem (Maschke):** Let $G$ be a finite group and $k$ a field with $\mathrm{char}(k) \nmid |G|$ (in particular, any field of characteristic 0). Then every representation of $G$ over $k$ is **completely reducible** (semisimple): it decomposes as a direct sum of irreducible representations.

**Proof:** Let $W \subseteq V$ be a subrepresentation. We want a $G$-stable complement. Choose any projection $p: V \to W$ (linear, not necessarily $G$-equivariant). **Average over $G$**:
$$\bar{p} = \frac{1}{|G|} \sum_{g \in G} g \circ p \circ g^{-1}$$

Then $\bar{p}$ is $G$-equivariant, $\bar{p}|_W = \mathrm{id}_W$, and $\ker \bar{p}$ is a $G$-stable complement to $W$. $\square$

**Division by $|G|$ requires $\mathrm{char}(k) \nmid |G|$** — this is why the theorem fails in modular representation theory ($\mathrm{char}(k) \mid |G|$).

### 18.3.2 Consequences

- Every representation decomposes: $V \cong V_1^{\oplus m_1} \oplus V_2^{\oplus m_2} \oplus \cdots \oplus V_r^{\oplus m_r}$ where $V_1, \ldots, V_r$ are the distinct irreducibles (up to iso) with multiplicities $m_i$.

- The group algebra is semisimple: $k[G] \cong \bigoplus_i M_{n_i}(k)$ (Artin-Wedderburn theorem), where the sum is over irreducibles.

### 18.3.3 Artin–Wedderburn Theorem

**Theorem:** A semisimple ring $R$ is isomorphic to a product of matrix rings:
$$R \cong M_{n_1}(D_1) \times M_{n_2}(D_2) \times \cdots \times M_{n_r}(D_r)$$
where $D_i$ are division rings.

For $k[G]$ with $k = \mathbb{C}$: $D_i = \mathbb{C}$ (since $\mathbb{C}$ is algebraically closed), and:
$$\mathbb{C}[G] \cong M_{n_1}(\mathbb{C}) \times \cdots \times M_{n_r}(\mathbb{C}), \quad \sum n_i^2 = |G|$$

where $r$ = number of irreducible representations = number of conjugacy classes.

---

## 18.4 Schur's Lemma

### 18.4.1 Statement

**Lemma (Schur):** Let $V, W$ be irreducible representations of $G$ over $k$.
1. Any $G$-map $T: V \to W$ is either zero or an isomorphism.
2. If $k = \mathbb{C}$ (or $k$ algebraically closed): any $G$-map $T: V \to V$ is $\lambda \cdot \mathrm{id}_V$ for some $\lambda \in k$.

**Proof:** For (1): $\ker T$ and $\mathrm{im}\, T$ are subrepresentations; by irreducibility, each is 0 or the whole space.
For (2): $T$ has an eigenvalue $\lambda \in \mathbb{C}$; then $T - \lambda I$ is a $G$-map with non-trivial kernel, hence $T - \lambda I = 0$.

### 18.4.2 Consequences

- $\mathrm{Hom}_G(V, W) = 0$ if $V \not\cong W$ (irreducible)
- $\mathrm{Hom}_G(V, V) \cong k$ if $V$ is irreducible over algebraically closed $k$
- The multiplicity of irreducible $V_i$ in $V$ is $m_i = \dim_k \mathrm{Hom}_G(V_i, V)$

---

## 18.5 The Regular Representation

### 18.5.1 Definition

The **regular representation** is $V = k[G]$ (the group algebra as a $k[G]$-module), with $G$ acting by left multiplication: $g \cdot \sum_h a_h h = \sum_h a_h (gh)$.

$\dim k[G] = |G|$.

### 18.5.2 Decomposition

Over $k = \mathbb{C}$:
$$k[G] \cong \bigoplus_i V_i^{\oplus n_i}$$

where the sum is over all irreducible representations $V_i$ with $n_i = \dim V_i$.

This is the Peter–Weyl decomposition for finite groups.

**Dimension formula:**
$$|G| = \sum_i n_i^2$$

Each irreducible $V_i$ appears in the regular representation with multiplicity equal to its dimension.

---

## 18.6 Examples

### 18.6.1 Representations of $S_3$

$|S_3| = 6 = 1^2 + 1^2 + 2^2$. Three irreducibles:
- **Trivial:** $V_\mathrm{triv}$, $\dim = 1$, $\sigma \mapsto 1$
- **Sign:** $V_\mathrm{sgn}$, $\dim = 1$, $\sigma \mapsto \mathrm{sgn}(\sigma)$
- **Standard:** $V_\mathrm{std}$, $\dim = 2$, the permutation rep minus trivial

### 18.6.2 Representations of $\mathbb{Z}/n\mathbb{Z}$

$G = \mathbb{Z}/n\mathbb{Z}$ is abelian. Over $\mathbb{C}$: all irreducibles are 1-dimensional. They are $\rho_k: m \mapsto e^{2\pi i km/n}$ for $k = 0, 1, \ldots, n-1$.

$$\mathbb{C}[\mathbb{Z}/n\mathbb{Z}] \cong \mathbb{C} \times \mathbb{C} \times \cdots \times \mathbb{C} \quad (n \text{ factors})$$

This is a matrix form of the discrete Fourier transform.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Maschke's theorem | Complete reducibility when $\mathrm{char}(k) \nmid \|G\|$ |
| Artin–Wedderburn | Semisimple ring $\cong$ product of matrix rings |
| Schur's lemma | Irreducible Hom spaces are 0 or 1-dimensional (over $\mathbb{C}$) |
| Regular rep decomposition | $k[G] \cong \bigoplus V_i^{\oplus \dim V_i}$; $\|G\| = \sum (\dim V_i)^2$ |

---

## Milestone Exercises

1. Prove Maschke's theorem in full.

2. Show $\mathbb{Z}[G]$ for $G = \mathbb{Z}/2\mathbb{Z}$ and field $k = \mathbb{F}_2$ (char 2) has a representation that is not completely reducible.

3. Find all irreducible representations of $D_4$ (dihedral group of order 8) over $\mathbb{C}$.

4. Prove: the number of irreducible complex representations of $G$ equals the number of conjugacy classes.

5. Show that if $V$ is an irreducible $G$-representation, $V \otimes V^*$ contains the trivial representation exactly once.

6. Let $V$ be the standard 2-dimensional representation of $S_3$. Decompose $V \otimes V$ into irreducibles.

7. Verify the dimension formula $\sum_i n_i^2 = |G|$ for $G = A_4$ (order 12).

---

## Connections Forward

- **Chapter 19:** Character theory gives a complete invariant for representations; orthogonality relations organize the data.
- **Chapter 20:** Representations of Lie groups reduce to representations of Lie algebras — continuous analogue of the finite group theory here.
- **Chapter 22:** Classification of irreducible representations via highest weights is the Lie algebra version of finding all irreducibles.

---

*Next: [Chapter 19 — Character Theory](ch19-character-theory.md)*
