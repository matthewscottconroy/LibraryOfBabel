# Chapter 20 — Lie Groups and Lie Algebras

**Part V: Representation Theory**
*Prerequisites: [Chapter 19](ch19-character-theory.md), [Chapter 7](ch07-inner-product-spaces.md)*
*Next: [Chapter 21 — Semisimple Lie Algebras and Root Systems](ch21-semisimple-lie-algebras.md)*

---

## Learning Objectives

- Define Lie groups as smooth manifolds with group structure
- Define Lie algebras as the tangent space at the identity
- Understand the exponential map and its relationship between Lie groups and algebras
- Work with the classical Lie groups and their Lie algebras
- Understand the adjoint representation and the Killing form
- Prove the fundamental correspondence: Lie group homomorphisms correspond to Lie algebra homomorphisms (for simply connected groups)

---

## 20.1 Lie Groups

### 20.1.1 Definition

A **Lie group** $G$ is a group that is also a smooth manifold, with the group operations:
$$\mu: G \times G \to G, \quad (g,h) \mapsto gh$$
$$\iota: G \to G, \quad g \mapsto g^{-1}$$
both smooth.

*No significant differential geometry required here — take "smooth manifold" as "a space locally like $\mathbb{R}^n$ with smoothly compatible coordinate charts."*

### 20.1.2 Classical Lie Groups

| Group | Definition | Dimension | Compact? |
|-------|-----------|-----------|---------|
| $GL_n(\mathbb{R})$ | Invertible $n \times n$ real matrices | $n^2$ | No |
| $SL_n(\mathbb{R})$ | $\det = 1$ real matrices | $n^2 - 1$ | No |
| $O(n)$ | $A^T A = I$ | $\frac{n(n-1)}{2}$ | Yes |
| $SO(n)$ | $O(n) \cap SL_n(\mathbb{R})$ | $\frac{n(n-1)}{2}$ | Yes |
| $U(n)$ | $A^* A = I$ (unitary) | $n^2$ | Yes |
| $SU(n)$ | $U(n) \cap SL_n(\mathbb{C})$ | $n^2 - 1$ | Yes |
| $Sp(2n)$ | Symplectic group | $n(2n+1)$ | $Sp(n)$: Yes |

These are all **matrix Lie groups** — closed subgroups of $GL_n(\mathbb{R})$ or $GL_n(\mathbb{C})$.

### 20.1.3 One-Parameter Subgroups

A **one-parameter subgroup** is a smooth homomorphism $\gamma: (\mathbb{R}, +) \to G$. Such maps are of the form:
$$\gamma(t) = e^{tX}$$

for some $X \in M_n(\mathbb{R})$ (in the matrix case). This motivates the exponential map.

---

## 20.2 Lie Algebras

### 20.2.1 Definition (Algebraic)

A **Lie algebra** $\mathfrak{g}$ over $k$ is a $k$-vector space with a bilinear operation $[\cdot, \cdot]: \mathfrak{g} \times \mathfrak{g} \to \mathfrak{g}$ (the **Lie bracket**) satisfying:
- **Antisymmetry:** $[X, Y] = -[Y, X]$
- **Jacobi identity:** $[X,[Y,Z]] + [Y,[Z,X]] + [Z,[X,Y]] = 0$

### 20.2.2 Examples

| Lie group $G$ | Lie algebra $\mathfrak{g}$ | Bracket |
|--------------|---------------------------|---------|
| $GL_n(\mathbb{R})$ | $\mathfrak{gl}_n = M_n(\mathbb{R})$ | $[A,B] = AB - BA$ |
| $SL_n$ | $\mathfrak{sl}_n = \{A : \mathrm{tr}\, A = 0\}$ | $[A,B] = AB - BA$ |
| $O(n), SO(n)$ | $\mathfrak{so}_n = \{A : A^T = -A\}$ (skew-symmetric) | $[A,B] = AB - BA$ |
| $U(n)$ | $\mathfrak{u}_n = \{A : A^* = -A\}$ (skew-Hermitian) | $[A,B] = AB - BA$ |
| $SU(n)$ | $\mathfrak{su}_n = \{A \in \mathfrak{u}_n : \mathrm{tr}\, A = 0\}$ | $[A,B] = AB - BA$ |

For matrix groups, $\mathfrak{g} = T_I G$ (tangent space at the identity), and $[X,Y] = XY - YX$ (matrix commutator).

### 20.2.3 The Lie Algebra as Tangent Space

For a matrix Lie group $G \subseteq GL_n$:
$$\mathfrak{g} = \{X \in M_n \mid e^{tX} \in G \text{ for all } t \in \mathbb{R}\}$$

The Lie bracket is $[X,Y] = XY - YX$, recovered as:
$$[X,Y] = \frac{d}{dt}\bigg|_{t=0} e^{tX} Y e^{-tX} = \lim_{t \to 0} \frac{e^{tX} e^{tY} e^{-tX} e^{-tY} - I}{t^2}$$

---

## 20.3 The Exponential Map

### 20.3.1 Definition

For a matrix Lie group $G$, the **exponential map**:
$$\exp: \mathfrak{g} \to G, \quad \exp(X) = e^X = \sum_{n=0}^\infty \frac{X^n}{n!}$$

**Properties:**
- $\exp(0) = I$ (identity)
- $\exp((s+t)X) = \exp(sX)\exp(tX)$
- $\det(\exp X) = e^{\mathrm{tr}(X)}$
- Near $0 \in \mathfrak{g}$, $\exp$ is a diffeomorphism onto a neighborhood of $e \in G$
- For compact connected Lie groups, $\exp$ is surjective

**Baker–Campbell–Hausdorff formula:** $\exp(X)\exp(Y) = \exp(X + Y + \frac{1}{2}[X,Y] + \frac{1}{12}[X,[X,Y]] - \frac{1}{12}[Y,[X,Y]] + \cdots)$

This says multiplication in $G$ near $e$ is entirely determined by the Lie bracket in $\mathfrak{g}$.

### 20.3.2 Lie Group $\leftrightarrow$ Lie Algebra Dictionary

| Lie Group $G$ | Lie Algebra $\mathfrak{g}$ |
|--------------|--------------------------|
| Group | Vector space with bracket |
| Closed subgroup $H \leq G$ | Lie subalgebra $\mathfrak{h} \subseteq \mathfrak{g}$ |
| Normal subgroup $N \trianglelefteq G$ | Ideal $\mathfrak{n} \trianglelefteq \mathfrak{g}$ |
| Quotient group $G/N$ | Quotient algebra $\mathfrak{g}/\mathfrak{n}$ |
| Homomorphism $\phi: G \to H$ | Lie algebra map $d\phi: \mathfrak{g} \to \mathfrak{h}$ |

**Theorem:** For simply connected Lie groups $G, H$: group homomorphisms $G \to H$ are in bijection with Lie algebra homomorphisms $\mathfrak{g} \to \mathfrak{h}$.

---

## 20.4 The Adjoint Representation

### 20.4.1 Adjoint of a Lie Group

For each $g \in G$, **conjugation** $\mathrm{Ad}(g): \mathfrak{g} \to \mathfrak{g}$ is defined by:
$$\mathrm{Ad}(g)(X) = \frac{d}{dt}\bigg|_{t=0} g e^{tX} g^{-1} = gXg^{-1} \quad \text{(for matrix groups)}$$

This gives the **adjoint representation** $\mathrm{Ad}: G \to GL(\mathfrak{g})$.

### 20.4.2 Adjoint of a Lie Algebra

The derivative of $\mathrm{Ad}$ at $e$ gives the **adjoint representation of the Lie algebra**:
$$\mathrm{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g}), \quad \mathrm{ad}(X)(Y) = [X, Y]$$

The Jacobi identity says exactly that $\mathrm{ad}: \mathfrak{g} \to \mathfrak{gl}(\mathfrak{g})$ is a Lie algebra homomorphism:
$$\mathrm{ad}([X,Y]) = [\mathrm{ad}(X), \mathrm{ad}(Y)]$$

### 20.4.3 The Killing Form

The **Killing form** is the symmetric bilinear form:
$$B(X, Y) = \mathrm{tr}(\mathrm{ad}(X) \circ \mathrm{ad}(Y))$$

It is $G$-invariant and $\mathfrak{g}$-invariant.

**Cartan's criterion (semisimplicity):** $\mathfrak{g}$ is semisimple $\Leftrightarrow$ $B$ is non-degenerate.

**Cartan's criterion (solvability):** $\mathfrak{g}$ is solvable $\Leftrightarrow$ $B(\mathfrak{g}, [\mathfrak{g}, \mathfrak{g}]) = 0$.

---

## 20.5 Solvable and Nilpotent Lie Algebras

### 20.5.1 Derived Series and Solvability

Define the **derived series**:
$$\mathfrak{g}^{(0)} = \mathfrak{g}, \quad \mathfrak{g}^{(k+1)} = [\mathfrak{g}^{(k)}, \mathfrak{g}^{(k)}]$$

$\mathfrak{g}$ is **solvable** if $\mathfrak{g}^{(k)} = 0$ for some $k$.

**Lie's theorem:** Over $\mathbb{C}$, every representation of a solvable Lie algebra $\mathfrak{g}$ has a common eigenvector.

### 20.5.2 Lower Central Series and Nilpotency

$$\mathfrak{g}_0 = \mathfrak{g}, \quad \mathfrak{g}_{k+1} = [\mathfrak{g}, \mathfrak{g}_k]$$

$\mathfrak{g}$ is **nilpotent** if $\mathfrak{g}_k = 0$ for some $k$.

Nilpotent $\Rightarrow$ Solvable. Equivalent: $\mathrm{ad}(X)$ is nilpotent for all $X \in \mathfrak{g}$.

**Engel's theorem:** If $\mathrm{ad}(X)$ is nilpotent for all $X$, then $\mathfrak{g}$ is nilpotent.

---

## 20.6 Simply Connected Lie Groups and Covering Theory

A Lie group is **simply connected** if its underlying space is simply connected (every loop is contractible).

**Examples:**
- $SU(n)$ is simply connected; $U(n)$ and $SO(n)$ are not (for $n \geq 2$)
- $\widetilde{SL_2(\mathbb{R})}$ = universal cover of $SL_2(\mathbb{R})$ — not a matrix group!
- Every Lie algebra is the Lie algebra of a unique simply connected Lie group

**Fundamental theorem of Lie theory:** There is an equivalence of categories:
$$\{\text{simply connected Lie groups}\} \leftrightarrow \{\text{finite-dimensional Lie algebras over } \mathbb{R}\}$$

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| BCH formula | $\exp(X)\exp(Y) = \exp(X + Y + \frac{1}{2}[X,Y] + \cdots)$ |
| Lie's theorem | Solvable algebra $\Rightarrow$ common eigenvector over $\mathbb{C}$ |
| Engel's theorem | All $\mathrm{ad}(X)$ nilpotent $\Leftrightarrow$ $\mathfrak{g}$ nilpotent |
| Cartan's criterion | $\mathfrak{g}$ semisimple $\Leftrightarrow$ Killing form non-degenerate |
| Lie group-algebra equivalence | Simply connected groups $\leftrightarrow$ Lie algebras |

---

## Milestone Exercises

1. Verify $\mathfrak{so}(3) \cong \mathfrak{su}(2)$ as Lie algebras. What does this imply about $SO(3)$ and $SU(2)$?

2. Compute the Killing form for $\mathfrak{sl}_2(\mathbb{C})$. Is it non-degenerate?

3. Show $\mathfrak{sl}_2(\mathbb{C})$ is semisimple using Cartan's criterion.

4. The Lie algebra $\mathfrak{b}_n$ (upper triangular matrices) is solvable. Show this by computing its derived series.

5. Prove: $\mathrm{ad}([X,Y]) = [\mathrm{ad}(X), \mathrm{ad}(Y)]$ directly from the Jacobi identity.

6. For $G = SU(2)$: show $\exp: \mathfrak{su}(2) \to SU(2)$ is surjective and compute its kernel.

7. Show the Heisenberg algebra (3-dimensional nilpotent Lie algebra) has a faithful 3-dimensional representation but no faithful 1-dimensional representation.

---

## Connections Forward

- **Chapter 21:** Semisimple Lie algebras are classified by root systems and Dynkin diagrams.
- **Chapter 22:** Representations of $\mathfrak{g}$ are classified by highest weights; the Weyl character formula computes characters.
- **Chapter 23:** Algebraic groups (varieties with group structure) generalize Lie groups to arbitrary fields.

---

*Next: [Chapter 21 — Semisimple Lie Algebras and Root Systems](ch21-semisimple-lie-algebras.md)*
