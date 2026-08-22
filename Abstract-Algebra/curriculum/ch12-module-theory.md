# Chapter 12 — Module Theory

**Part III: Abstract Algebra**
*Prerequisites: [Chapter 11](ch11-ring-theory.md), [Chapter 8](ch08-multilinear-algebra-tensors.md)*
*Next: [Chapter 13 — Field Theory and Galois Theory](ch13-field-theory-galois.md)*

---

## Learning Objectives

- Define modules and understand them as the common generalization of vector spaces and abelian groups
- Classify modules as free, projective, injective, or flat
- Prove the structure theorem for finitely generated modules over a PID
- Derive Jordan form and the classification of abelian groups as corollaries
- Define and compute tensor products of modules
- Understand Hom and tensor as the central functors of module theory

---

## 12.1 Modules

### 12.1.1 Definition

Let $R$ be a ring (with 1). A **left $R$-module** is an abelian group $(M, +)$ with a scalar multiplication $R \times M \to M$, $(r, m) \mapsto rm$, satisfying:
- $r(m + m') = rm + rm'$
- $(r + r')m = rm + r'm$
- $(rr')m = r(r'm)$
- $1_R \cdot m = m$

**Right modules** defined analogously. Over commutative $R$, left and right modules coincide.

### 12.1.2 Examples

| Ring $R$ | Module $M$ | What it is |
|----------|-----------|------------|
| Field $F$ | Any $F$-module | An $F$-vector space |
| $\mathbb{Z}$ | Any $\mathbb{Z}$-module | An abelian group |
| $F[x]$ | $F^n$ with a chosen operator $T$ | $F[x]$-module via $x \cdot \mathbf{v} = T\mathbf{v}$ |
| $R$ | $R$ itself | $R$ as a left module (regular representation) |
| $R$ | $I \trianglelefteq R$ | Ideal as module |

The $F[x]$-module structure on a vector space is exactly the data of a linear operator — this is the key connection to canonical forms.

### 12.1.3 Submodules and Quotients

A **submodule** $N \leq M$ is a subgroup closed under the $R$-action: $rn \in N$ for all $r \in R$, $n \in N$.

The **quotient module** $M/N$ has cosets $m + N$ with $r(m+N) = rm + N$. Well-defined.

**Module homomorphisms** (or $R$-linear maps): $\phi: M \to M'$ with $\phi(rm) = r\phi(m)$.

**Isomorphism theorems** hold for modules exactly as for groups and rings.

---

## 12.2 Free, Projective, and Injective Modules

### 12.2.1 Free Modules

A module $M$ is **free** if it has a basis — a subset $S \subseteq M$ such that every element of $M$ is a unique finite $R$-linear combination of elements of $S$.

$M \cong R^n$ (free module of rank $n$) if $M$ has a finite basis of $n$ elements.

**Universal property:** $\mathrm{Hom}_R(R^n, M) \cong M^n$ (a free module is the "most general" module on $n$ generators).

**Caution:** Over a non-commutative ring, the rank of a free module may not be well-defined. Over commutative rings (e.g., $\mathbb{Z}$, $F$): rank is well-defined.

### 12.2.2 Projective Modules

$P$ is **projective** if it satisfies the **lifting property**: for any surjection $f: M \twoheadrightarrow N$ and map $g: P \to N$, there exists $\tilde{g}: P \to M$ with $f \circ \tilde{g} = g$.

$$\xymatrix{ & P \ar[d]^g \ar@{-->}[dl]_{\tilde g} \\ M \ar@{->>}[r]^f & N }$$

**Equivalent:** $P$ is a direct summand of a free module.

**Free $\Rightarrow$ Projective $\Rightarrow$ Flat.** Over PIDs, all finitely generated projective modules are free.

**Example of non-free projective:** Over $\mathbb{Z}/6\mathbb{Z}$, the ideal $(2)$ is projective but not free.

### 12.2.3 Injective Modules

$I$ is **injective** if it satisfies the **extension property**: for any injection $f: N \hookrightarrow M$ and map $g: N \to I$, there exists $\tilde{g}: M \to I$ with $\tilde{g} \circ f = g$.

**Baer's criterion:** $I$ is injective iff for every ideal $\mathfrak{a} \trianglelefteq R$ and map $\phi: \mathfrak{a} \to I$, there exists $\tilde\phi: R \to I$ extending $\phi$.

**Injective hulls:** Every module embeds into an injective module; there is a minimal such embedding (the injective hull $E(M)$).

**Over $\mathbb{Z}$:** The injective $\mathbb{Z}$-modules are the divisible abelian groups: $\mathbb{Q}$, $\mathbb{Q}/\mathbb{Z}$, $\mathbb{Z}[p^{-1}]/\mathbb{Z}$, etc.

---

## 12.3 The Structure Theorem

### 12.3.1 Statement

**Theorem (Structure Theorem for Finitely Generated Modules over a PID):**

Let $R$ be a PID and $M$ a finitely generated $R$-module. Then:
$$M \cong R^r \oplus R/(d_1) \oplus R/(d_2) \oplus \cdots \oplus R/(d_k)$$
where $r \geq 0$ (the **rank** or **free part**), $d_1, d_2, \ldots, d_k \in R$ are non-zero non-units with $d_1 \mid d_2 \mid \cdots \mid d_k$ (**invariant factors**).

The invariant factors and rank are uniquely determined by $M$.

**Equivalently (primary decomposition):**
$$M \cong R^r \oplus \bigoplus_i R/(p_i^{a_i})$$

### 12.3.2 Two Corollaries

**Corollary 1 (Fundamental Theorem of Finitely Generated Abelian Groups):**

Take $R = \mathbb{Z}$. Every finitely generated abelian group is:
$$\mathbb{Z}^r \oplus \mathbb{Z}/d_1\mathbb{Z} \oplus \cdots \oplus \mathbb{Z}/d_k\mathbb{Z}$$

This is the result stated in Chapter 10, now proved via module theory.

**Corollary 2 (Jordan Canonical Form):**

Take $R = F[x]$ acting on an $F$-vector space $V$ via a linear operator $T$. The structure theorem gives:
$$V \cong F[x]/(f_1) \oplus F[x]/(f_2) \oplus \cdots$$

The invariant factors $f_i \in F[x]$ recover the rational canonical form of $T$. If $F$ is algebraically closed, the primary decomposition gives the Jordan canonical form.

This unifies the classification theorems of linear algebra with module theory over a PID.

---

## 12.4 Hom and Tensor

### 12.4.1 The Hom Functor

For $R$-modules $M, N$:
$$\mathrm{Hom}_R(M, N) = \{R\text{-linear maps } M \to N\}$$

This is an abelian group (and an $R$-module if $R$ is commutative).

**Left exact:** A short exact sequence $0 \to A \to B \to C \to 0$ induces:
$$0 \to \mathrm{Hom}_R(M, A) \to \mathrm{Hom}_R(M, B) \to \mathrm{Hom}_R(M, C)$$
(exact, but the last map need not be surjective — failure to be exact here is measured by $\mathrm{Ext}$, Chapter 16).

**Right exact (contravariant):** Similarly for $\mathrm{Hom}_R(-, N)$.

### 12.4.2 Tensor Product of Modules

The **tensor product** $M \otimes_R N$ is defined by the universal property:
bilinear maps $M \times N \to P$ correspond to $R$-linear maps $M \otimes_R N \to P$.

If $R$ is commutative, $M \otimes_R N$ is again an $R$-module.

**Right exact:** A short exact sequence $0 \to A \to B \to C \to 0$ induces:
$$M \otimes A \to M \otimes B \to M \otimes C \to 0$$
(right exact; failure at left is measured by $\mathrm{Tor}$, Chapter 16).

### 12.4.3 Adjunction: Hom-Tensor Adjunction

$$\mathrm{Hom}_R(M \otimes_R N, P) \cong \mathrm{Hom}_R(M, \mathrm{Hom}_R(N, P))$$

This is the **tensor-hom adjunction** (or currying isomorphism). It is a foundational example of an adjoint pair of functors (Chapter 14).

### 12.4.4 Flat Modules

$M$ is **flat** if $- \otimes_R M$ is exact (i.e., preserves all exact sequences).

Free $\Rightarrow$ Projective $\Rightarrow$ Flat. Flatness is the weakest of the three but still powerful.

**Key example:** $\mathbb{Q}$ is a flat (in fact, torsion-free) $\mathbb{Z}$-module; $\mathbb{Z}/n\mathbb{Z}$ is not flat.

---

## 12.5 Noetherian Modules and Rings

### 12.5.1 Noetherian Condition

A module $M$ is **Noetherian** if every ascending chain of submodules stabilizes:
$$M_1 \subseteq M_2 \subseteq \cdots \Rightarrow M_k = M_{k+1} = \cdots \text{ for some } k$$

Equivalently: every submodule of $M$ is finitely generated.

A ring $R$ is **Noetherian** if $R$ is Noetherian as a left module over itself.

**Hilbert's Basis Theorem:** If $R$ is Noetherian, so is $R[x]$. Thus $F[x_1, \ldots, x_n]$ is Noetherian.

### 12.5.2 Artinian Modules

A module is **Artinian** if every descending chain of submodules stabilizes.

**Hopkins–Levitzki theorem:** For a finitely generated module over a Noetherian ring: Artinian $\Leftrightarrow$ has a composition series.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Structure theorem (PID) | Finitely gen. module over PID $\cong R^r \oplus \bigoplus R/(d_i)$ |
| Baer's criterion | Injective $\Leftrightarrow$ extends maps from ideals |
| Hom-Tensor adjunction | $\mathrm{Hom}(M \otimes N, P) \cong \mathrm{Hom}(M, \mathrm{Hom}(N, P))$ |
| Hilbert basis theorem | Noetherian $\Rightarrow$ $R[x]$ Noetherian |

---

## Milestone Exercises

1. Show that $\mathbb{Q}$ is not a free $\mathbb{Z}$-module.

2. Let $T: \mathbb{R}^3 \to \mathbb{R}^3$ have minimal polynomial $(x-2)^2(x+1)$. Find the $\mathbb{R}[x]$-module structure and determine the rational canonical form.

3. Classify all finitely generated modules over $\mathbb{Z}$ of order 36.

4. Show that $M \otimes_R (N \oplus P) \cong (M \otimes_R N) \oplus (M \otimes_R P)$.

5. Compute $\mathbb{Z}/m\mathbb{Z} \otimes_{\mathbb{Z}} \mathbb{Z}/n\mathbb{Z} \cong \mathbb{Z}/\gcd(m,n)\mathbb{Z}$.

6. Prove: a finitely generated projective module over a PID is free.

7. Show: if $0 \to A \to B \to C \to 0$ is exact and $C$ is projective, then $B \cong A \oplus C$.

---

## Connections Forward

- **Chapter 13:** Galois theory uses modules over group rings and field extensions as modules.
- **Chapter 15:** Projective and injective resolutions are the foundation of homological algebra.
- **Chapter 16:** $\mathrm{Ext}^n(M,N)$ and $\mathrm{Tor}_n(M,N)$ measure failure of exactness of Hom and Tensor.
- **Chapter 18:** Group algebras $F[G]$ are rings; representations are $F[G]$-modules.

---

*Next: [Chapter 13 — Field Theory and Galois Theory](ch13-field-theory-galois.md)*
