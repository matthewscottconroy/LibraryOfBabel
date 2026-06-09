# Chapter 19 — Character Theory

**Part V: Representation Theory**
*Prerequisites: [Chapter 18](ch18-representations-finite-groups.md)*
*Next: [Chapter 20 — Lie Groups and Lie Algebras](ch20-lie-groups-algebras.md)*

---

## Learning Objectives

- Define characters and understand their basic properties
- State and apply the orthogonality relations
- Use the character table to decompose representations and detect irreducibles
- Compute induced and restricted representations; prove Frobenius reciprocity
- Apply Burnside's theorem and other character-theoretic results
- Understand the character ring and its algebraic structure

---

## 19.1 Characters

### 19.1.1 Definition

The **character** of a representation $(\rho, V)$ of $G$ is the function:
$$\chi_V: G \to k, \quad \chi_V(g) = \mathrm{tr}(\rho(g))$$

**Basic properties:**
- $\chi_V(e) = \dim V$
- $\chi_V(hgh^{-1}) = \chi_V(g)$ (class function — constant on conjugacy classes)
- $\chi_V(g^{-1}) = \overline{\chi_V(g)}$ over $\mathbb{C}$ (since $\rho(g)$ is unitary in some basis)
- $\chi_{V \oplus W} = \chi_V + \chi_W$
- $\chi_{V \otimes W} = \chi_V \cdot \chi_W$
- $\chi_{V^*}(g) = \overline{\chi_V(g)}$

### 19.1.2 Characters as Class Functions

A **class function** is a function $f: G \to k$ constant on conjugacy classes. The space of class functions:
$$\mathrm{Cl}(G) = \{f: G \to \mathbb{C} \mid f(hgh^{-1}) = f(g)\}$$

This is a vector space of dimension equal to the number of conjugacy classes.

**Inner product on class functions:**
$$\langle \chi, \psi \rangle = \frac{1}{|G|} \sum_{g \in G} \chi(g)\overline{\psi(g)}$$

---

## 19.2 Orthogonality Relations

### 19.2.1 First Orthogonality Relation (Row Orthogonality)

**Theorem:** If $V, W$ are irreducible complex representations:
$$\langle \chi_V, \chi_W \rangle = \begin{cases} 1 & V \cong W \\ 0 & V \not\cong W \end{cases}$$

In other words, the characters of irreducible representations form an **orthonormal set** in $\mathrm{Cl}(G)$.

**Consequences:**
- The irreducible characters form a **basis** for $\mathrm{Cl}(G)$ — in particular, there are exactly as many irreducibles as conjugacy classes
- Decomposition formula: if $V = \bigoplus m_i V_i$, then $m_i = \langle \chi_V, \chi_{V_i} \rangle$
- $V \cong W \Leftrightarrow \chi_V = \chi_W$ (characters classify representations!)

### 19.2.2 Second Orthogonality Relation (Column Orthogonality)

**Theorem:** For conjugacy classes $C_i, C_j$ with representatives $g_i, g_j$:
$$\sum_{\text{irr. } V} \chi_V(g_i) \overline{\chi_V(g_j)} = \frac{|G|}{|C_i|} \delta_{ij}$$

This is orthogonality across columns of the character table.

---

## 19.3 The Character Table

### 19.3.1 Definition

The **character table** of $G$ is the matrix:
$$X = (\chi_i(g_j))$$

where $\chi_1, \ldots, \chi_r$ are all irreducible characters and $g_1, \ldots, g_r$ are representatives of the $r$ conjugacy classes.

The rows are orthogonal (first orthogonality); the columns are orthogonal (second orthogonality).

### 19.3.2 Constraints on the Character Table

- Number of rows = number of columns = number of conjugacy classes
- $\chi_i(e) = \dim V_i = n_i$; first column gives dimensions
- $\sum_i n_i^2 = |G|$
- $n_i \mid |G|$ for each $i$ (a non-trivial result)
- $\chi_1$ is always the trivial character (all 1's)

### 19.3.3 Example: $S_3$

Conjugacy classes of $S_3$: $\{e\}$ (size 1), $\{(12),(13),(23)\}$ (size 3), $\{(123),(132)\}$ (size 2).

| | $e$ | $(12)$ | $(123)$ |
|---|---|---|---|
| $\chi_\mathrm{triv}$ | 1 | 1 | 1 |
| $\chi_\mathrm{sgn}$ | 1 | $-1$ | 1 |
| $\chi_\mathrm{std}$ | 2 | 0 | $-1$ |

Check: $1^2 + 1^2 + 2^2 = 6 = |S_3|$. ✓

### 19.3.4 Example: $A_4$

$|A_4| = 12$. Four conjugacy classes. Dimensions: $1, 1, 1, 3$ (since $1+1+1+9 = 12$). Three 1-dimensional representations from $A_4/V_4 \cong \mathbb{Z}/3\mathbb{Z}$; one 3-dimensional from permutation on 4 elements minus trivial.

---

## 19.4 Induced and Restricted Representations

### 19.4.1 Restriction

For $H \leq G$, the **restricted representation** $\mathrm{Res}^G_H V$: same space $V$, but only the action of $H$.

Character: $\chi_{\mathrm{Res}^G_H V}(h) = \chi_V(h)$ for $h \in H$.

### 19.4.2 Induced Representation

For $H \leq G$ and a representation $W$ of $H$, the **induced representation**:
$$\mathrm{Ind}^G_H W = k[G] \otimes_{k[H]} W$$

Dimension: $\dim \mathrm{Ind}^G_H W = [G:H] \cdot \dim W$.

**Character formula:**
$$\chi_{\mathrm{Ind}^G_H W}(g) = \frac{1}{|H|} \sum_{x \in G,\, x^{-1}gx \in H} \chi_W(x^{-1}gx)$$

### 19.4.3 Frobenius Reciprocity

**Theorem:**
$$\langle \mathrm{Ind}^G_H W, V \rangle_G = \langle W, \mathrm{Res}^G_H V \rangle_H$$

for any $H$-representation $W$ and $G$-representation $V$.

In categorical terms: $\mathrm{Ind}^G_H \dashv \mathrm{Res}^G_H$ (induction is left adjoint to restriction).

**Applications:** Compute characters of induced representations; determine which irreducibles appear in $\mathrm{Ind}^G_H \mathbf{1}$ (the induced trivial).

---

## 19.5 Further Results

### 19.5.1 Burnside's $p^a q^b$ Theorem

**Theorem (Burnside):** If $|G| = p^a q^b$ for primes $p, q$, then $G$ is solvable.

**Proof strategy:** Uses character theory. For any non-trivial irreducible $\chi$ of degree divisible by $p$, the element $|C|\chi(g)/\chi(e)$ (for any class $C$ with representative $g$) is an algebraic integer; use this to construct a normal subgroup.

### 19.5.2 The Character Ring

The irreducible characters $\{\chi_1, \ldots, \chi_r\}$ span an abelian group (the **character ring** $R(G)$) under addition; $\chi_V \cdot \chi_W = \chi_{V \otimes W}$ makes it a commutative ring. It is the Grothendieck ring of the category of representations.

**Remark:** For compact Lie groups, this is the representation ring $R(G)$; for $G = U(1)$, $R(G) = \mathbb{Z}[t, t^{-1}]$ (Laurent polynomials).

### 19.5.3 Frobenius Groups and Mackey Formula

A **Frobenius group** $G = K \rtimes H$ with a specific action has a complete character theory via Frobenius's theorem.

**Mackey's formula:** Decomposes $\mathrm{Res}^G_H \mathrm{Ind}^G_K W$ in terms of double cosets $H \backslash G / K$.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| First orthogonality | $\langle \chi_V, \chi_W \rangle = \delta_{VW}$ for irreducibles |
| Second orthogonality | Column orthogonality of character table |
| Characters classify reps | $\chi_V = \chi_W \Leftrightarrow V \cong W$ |
| Frobenius reciprocity | $\langle \mathrm{Ind} W, V \rangle_G = \langle W, \mathrm{Res} V \rangle_H$ |
| Burnside's theorem | $\|G\| = p^a q^b \Rightarrow G$ solvable |

---

## Milestone Exercises

1. Compute the full character table of $D_4$ (dihedral group of order 8).

2. Compute the full character table of $A_4$.

3. Decompose $\mathrm{Ind}^{S_3}_{\mathbb{Z}/3\mathbb{Z}} \rho$ (where $\rho$ is a non-trivial 1-dim rep of $\mathbb{Z}/3\mathbb{Z}$) into irreducibles of $S_3$.

4. Prove: two elements of $G$ are conjugate iff $\chi(g) = \chi(h)$ for every character $\chi$.

5. Show that if $\chi$ is a character, so are $\chi^2(g) = \chi(g)^2$ (symmetric square) and $\chi \cdot \overline{\chi}$. Decompose these for the standard rep of $S_3$.

6. Use Burnside's theorem to show every group of order $p^2 q$ (for primes $p \neq q$) is solvable.

7. Prove the number of irreducible representations equals the number of conjugacy classes using the orthogonality of characters as a basis for class functions.

---

## Connections Forward

- **Chapter 20:** For compact Lie groups, the Peter–Weyl theorem gives the analogue of the regular representation decomposition; characters are continuous class functions.
- **Chapter 21:** The representation ring $R(G)$ for Lie groups $G$ is computed via the Weyl character formula.
- **Chapter 22:** Highest weight theory classifies irreducible representations of semisimple Lie algebras — the Lie algebra analogue of listing irreducible characters.
- **Chapter 24:** Automorphic representations are the infinite-dimensional analogue for the absolute Galois group and adélic groups.

---

*Next: [Chapter 20 — Lie Groups and Lie Algebras](ch20-lie-groups-algebras.md)*
