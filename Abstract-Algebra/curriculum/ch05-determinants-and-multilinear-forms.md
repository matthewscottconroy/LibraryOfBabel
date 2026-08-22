# Chapter 5 — Determinants and Multilinear Forms

**Part II: Linear Algebra**
*Prerequisites: [Chapter 4](ch04-linear-maps-and-matrices.md)*
*Next: [Chapter 6 — Eigentheory and Canonical Forms](ch06-eigentheory-and-canonical-forms.md)*

---

## Learning Objectives

- Define the determinant axiomatically via multilinearity and alternation
- Derive cofactor expansion and multiplicativity from first principles
- Understand the determinant geometrically: signed volume
- Compute determinants efficiently via row reduction
- Apply Cramer's rule and the adjugate matrix
- Lay the groundwork for exterior algebra (Chapter 8)

---

## 5.1 Multilinear Forms

### 5.1.1 Multilinear Maps

A map $f: V^k \to W$ is **multilinear** (or $k$-linear) if it is linear in each argument separately, holding the others fixed.

**Examples:**
- The dot product $\mathbb{R}^n \times \mathbb{R}^n \to \mathbb{R}$ is bilinear
- Matrix multiplication $M_{m \times n} \times M_{n \times p} \to M_{m \times p}$ is bilinear
- The determinant $F^n \times \cdots \times F^n \to F$ (what we are defining)

### 5.1.2 Alternating Forms

A multilinear map $f: V^n \to F$ is **alternating** if $f(\ldots, \mathbf{v}, \ldots, \mathbf{v}, \ldots) = 0$ whenever two arguments are equal.

**Equivalently:** Swapping any two arguments negates the value:
$$f(\ldots, \mathbf{u}, \ldots, \mathbf{v}, \ldots) = -f(\ldots, \mathbf{v}, \ldots, \mathbf{u}, \ldots)$$

This equivalence holds when $\mathrm{char}(F) \neq 2$.

---

## 5.2 The Determinant: Axiomatic Definition

### 5.2.1 Axioms

The **determinant** $\det: M_{n \times n}(F) \to F$ is the unique function satisfying:
1. **Multilinearity:** $\det$ is linear in each row (viewed as a function of $n$ row-vectors)
2. **Alternating:** Swapping two rows negates $\det$
3. **Normalization:** $\det(I_n) = 1$

These three properties uniquely determine $\det$.

**Consequence:** If two rows are equal, $\det = 0$. If a row is zero, $\det = 0$.

### 5.2.2 Permutations and the Leibniz Formula

The **symmetric group** $S_n$ is the group of all permutations of $\{1, \ldots, n\}$.

The **sign** (or **signature**) of $\sigma \in S_n$:
$$\mathrm{sgn}(\sigma) = (-1)^{N(\sigma)}$$
where $N(\sigma)$ is the number of **inversions** (pairs $(i,j)$ with $i < j$ but $\sigma(i) > \sigma(j)$).

**Leibniz formula:**
$$\det(A) = \sum_{\sigma \in S_n} \mathrm{sgn}(\sigma) \prod_{i=1}^n A_{i,\sigma(i)}$$

This is a sum of $n!$ terms, each a signed product of one entry from each row and column. For $n = 2$: $\det \begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$.

---

## 5.3 Computing Determinants

### 5.3.1 Cofactor Expansion

The **(signed) minor** $M_{ij}$ is the determinant of the $(n-1) \times (n-1)$ matrix obtained by deleting row $i$ and column $j$. The **cofactor** is $C_{ij} = (-1)^{i+j} M_{ij}$.

**Cofactor expansion along row $i$:**
$$\det(A) = \sum_{j=1}^n A_{ij} C_{ij}$$

**Cofactor expansion along column $j$:**
$$\det(A) = \sum_{i=1}^n A_{ij} C_{ij}$$

Choose the row or column with the most zeros for efficiency.

### 5.3.2 Row Reduction Method

Effect of row operations on $\det$:
1. Swap rows $i$ and $j$: multiply $\det$ by $-1$
2. Multiply row $i$ by $c \neq 0$: multiply $\det$ by $c$
3. Add $c$ times row $j$ to row $i$: $\det$ unchanged

**Algorithm:** Row-reduce to upper triangular form $U$ (tracking sign changes and scalings), then $\det(A) = \prod_i U_{ii}$ (product of diagonal entries).

For large matrices, this is $O(n^3)$ — far more efficient than Leibniz ($O(n!)$).

### 5.3.3 Block Triangular Matrices

$$\det \begin{pmatrix} A & B \\ 0 & C \end{pmatrix} = \det(A) \cdot \det(C)$$

This also holds for block diagonal and block lower triangular matrices.

---

## 5.4 Properties of the Determinant

### 5.4.1 Multiplicativity

**Theorem:** $\det(AB) = \det(A) \cdot \det(B)$ for all $A, B \in M_{n \times n}(F)$.

**Proof strategy:** Fix $B$; the map $A \mapsto \det(AB)/\det(B)$ satisfies the axioms for the determinant in $A$, hence equals $\det(A)$.

**Corollaries:**
- $\det(A^{-1}) = \det(A)^{-1}$ (when $A$ is invertible)
- $\det(A^T) = \det(A)$
- $A$ is invertible $\Leftrightarrow$ $\det(A) \neq 0$
- Similar matrices have equal determinants: $\det(P^{-1}AP) = \det(A)$

The last point means $\det$ is a property of the linear map $T$, not just of its matrix representation.

### 5.4.2 Geometric Interpretation

For $A \in M_{n \times n}(\mathbb{R})$, $|\det(A)|$ is the **$n$-dimensional volume** of the parallelepiped spanned by the columns of $A$.

The **sign** of $\det(A)$ records orientation: positive if the columns form a right-handed frame, negative if left-handed.

**2D example:** $|\det \begin{pmatrix} a & b \\ c & d \end{pmatrix}| = |ad - bc|$ is the area of the parallelogram spanned by $(a,c)$ and $(b,d)$.

---

## 5.5 The Adjugate and Cramer's Rule

### 5.5.1 The Adjugate Matrix

The **adjugate** (classical adjoint) of $A$ is:
$$\mathrm{adj}(A)_{ij} = C_{ji} \quad \text{(transpose of the cofactor matrix)}$$

**Theorem:** $A \cdot \mathrm{adj}(A) = \det(A) \cdot I_n$.

**Corollary:** If $\det(A) \neq 0$, then $A^{-1} = \frac{1}{\det(A)} \mathrm{adj}(A)$.

### 5.5.2 Cramer's Rule

For $A\mathbf{x} = \mathbf{b}$ with $A$ invertible, the unique solution has:
$$x_j = \frac{\det(A_j)}{\det(A)}$$
where $A_j$ is the matrix $A$ with column $j$ replaced by $\mathbf{b}$.

**Note:** Cramer's rule is theoretically elegant but computationally inefficient. Use row reduction in practice. Cramer's rule is important for proving existence/uniqueness in differential equations and algebraic geometry.

---

## 5.6 The Characteristic Polynomial

For $T: V \to V$ (endomorphism), the **characteristic polynomial** is:
$$p_T(\lambda) = \det(\lambda I - A)$$
where $A = [T]_{\mathcal{B}}$ for any basis $\mathcal{B}$.

This is a degree-$n$ polynomial in $\lambda$, with leading term $\lambda^n$.

**Independence of basis:** Since similar matrices have the same determinant, $p_T(\lambda)$ is independent of the choice of basis. It is an intrinsic invariant of $T$.

**Roots** of $p_T(\lambda)$ are the **eigenvalues** of $T$ — the subject of Chapter 6.

**Cayley–Hamilton theorem** (proved in Chapter 6): $p_T(T) = 0$ — every map satisfies its own characteristic polynomial.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Uniqueness of det | The three axioms uniquely determine $\det$ |
| Leibniz formula | $\det(A) = \sum_{\sigma \in S_n} \mathrm{sgn}(\sigma) \prod_i A_{i,\sigma(i)}$ |
| Multiplicativity | $\det(AB) = \det(A)\det(B)$ |
| Invertibility criterion | $A$ invertible $\Leftrightarrow \det(A) \neq 0$ |
| $\det(A^T) = \det(A)$ | Row and column expansions give same result |
| Cayley–Hamilton | $p_T(T) = 0$ |

---

## Milestone Exercises

1. Compute $\det \begin{pmatrix} 1 & 2 & 3 \\ 4 & 5 & 6 \\ 7 & 8 & 9 \end{pmatrix}$ by cofactor expansion and by row reduction. Explain the result.

2. Prove $\det(A^T) = \det(A)$ using the Leibniz formula and the fact that $\mathrm{sgn}(\sigma) = \mathrm{sgn}(\sigma^{-1})$.

3. Prove: if $A$ has two equal columns, $\det(A) = 0$.

4. Use Cramer's rule to solve $\begin{cases} 2x + y = 5 \\ x - y = 1 \end{cases}$.

5. If $A$ is $n \times n$ and $c \in F$, what is $\det(cA)$? Prove it.

6. Prove the Cayley–Hamilton theorem for $2 \times 2$ matrices directly.

7. Let $V = F[x]_{\leq 2}$. The map $T(f) = f'$ (differentiation) has characteristic polynomial $p_T(\lambda) = \lambda^3$. Verify that $T^3 = 0$ (nilpotent).

---

## Connections Forward

- **Chapter 6:** Eigenvalues are roots of the characteristic polynomial; Jordan form is the optimal canonical form.
- **Chapter 8:** The exterior algebra $\bigwedge V$ formalizes alternating multilinear forms; $\det$ is realized as the top exterior power.
- **Chapter 9:** The symmetric group $S_n$ and its sign homomorphism appear here; this is the first encounter with group theory.
- **Chapter 19:** Characters of representations are traces; trace and determinant are both similarity invariants.

---

*Next: [Chapter 6 — Eigentheory and Canonical Forms](ch06-eigentheory-and-canonical-forms.md)*
