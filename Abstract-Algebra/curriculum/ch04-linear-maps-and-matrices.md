# Chapter 4 — Linear Maps and Matrices

**Part II: Linear Algebra**
*Prerequisites: [Chapter 3](ch03-fields-and-vector-spaces.md)*
*Next: [Chapter 5 — Determinants and Multilinear Forms](ch05-determinants-and-multilinear-forms.md)*

---

## Learning Objectives

- Define linear maps (linear transformations) and verify linearity
- Understand kernel, image, and the rank-nullity theorem
- Represent linear maps by matrices relative to chosen bases
- Perform matrix algebra; understand why matrix multiplication encodes composition
- Change bases and understand similarity transformation
- Classify linear maps by their rank

---

## 4.1 Linear Maps

### 4.1.1 Definition

Let $V, W$ be vector spaces over $F$. A function $T: V \to W$ is **linear** (a **linear map** or **linear transformation**) if:
1. $T(\mathbf{u} + \mathbf{v}) = T(\mathbf{u}) + T(\mathbf{v})$ for all $\mathbf{u}, \mathbf{v} \in V$
2. $T(c\mathbf{v}) = cT(\mathbf{v})$ for all $c \in F$, $\mathbf{v} \in V$

Equivalently: $T(a\mathbf{u} + b\mathbf{v}) = aT(\mathbf{u}) + bT(\mathbf{v})$ for all $a,b \in F$.

**Immediate consequence:** $T(\mathbf{0}) = \mathbf{0}$ and $T(-\mathbf{v}) = -T(\mathbf{v})$.

### 4.1.2 Examples

| Map | Formula | Domain → Codomain |
|-----|---------|-------------------|
| Zero map | $T(\mathbf{v}) = \mathbf{0}$ | $V \to W$ |
| Identity | $I(\mathbf{v}) = \mathbf{v}$ | $V \to V$ |
| Projection | $\pi(x,y,z) = (x,y,0)$ | $\mathbb{R}^3 \to \mathbb{R}^3$ |
| Differentiation | $D(p) = p'$ | $F[x] \to F[x]$ |
| Integration | $I(f)(x) = \int_0^x f(t)\,dt$ | $\mathcal{C}([0,1]) \to \mathcal{C}([0,1])$ |
| Transpose | $T(A) = A^T$ | $M_{m\times n} \to M_{n\times m}$ |

### 4.1.3 Kernel and Image

$$\ker(T) = \{\mathbf{v} \in V \mid T(\mathbf{v}) = \mathbf{0}\}$$
$$\mathrm{im}(T) = \{T(\mathbf{v}) \mid \mathbf{v} \in V\} = T(V)$$

**Theorem:** $\ker(T)$ is a subspace of $V$; $\mathrm{im}(T)$ is a subspace of $W$.

**Injectivity criterion:** $T$ is injective $\Leftrightarrow$ $\ker(T) = \{\mathbf{0}\}$.

### 4.1.4 Rank-Nullity Theorem

**Theorem:** Let $T: V \to W$ with $\dim V = n < \infty$. Then:
$$\dim(\ker T) + \dim(\mathrm{im}\, T) = \dim V$$

Define: $\mathrm{nullity}(T) = \dim(\ker T)$, $\mathrm{rank}(T) = \dim(\mathrm{im}\, T)$.

So $\mathrm{nullity}(T) + \mathrm{rank}(T) = n$.

**Proof strategy:** Take a basis $\{\mathbf{k}_1, \ldots, \mathbf{k}_r\}$ of $\ker T$; extend to a basis $\{\mathbf{k}_1, \ldots, \mathbf{k}_r, \mathbf{v}_1, \ldots, \mathbf{v}_s\}$ of $V$. Show $\{T(\mathbf{v}_1), \ldots, T(\mathbf{v}_s)\}$ is a basis of $\mathrm{im}\, T$.

**Corollaries:**
- $T$ injective $\Rightarrow$ $\mathrm{rank}(T) = \dim V$
- $T$ surjective $\Rightarrow$ $\dim W = \mathrm{rank}(T) \leq \dim V$
- If $\dim V = \dim W$: $T$ injective $\Leftrightarrow$ $T$ surjective $\Leftrightarrow$ $T$ bijective

### 4.1.5 The Space of Linear Maps

$\mathcal{L}(V, W) = \{T: V \to W \mid T \text{ linear}\}$ is itself a vector space over $F$:
- $(S + T)(\mathbf{v}) = S(\mathbf{v}) + T(\mathbf{v})$
- $(cT)(\mathbf{v}) = c \cdot T(\mathbf{v})$

$\dim \mathcal{L}(V, W) = (\dim V)(\dim W)$.

---

## 4.2 Matrix Representations

### 4.2.1 Matrices as Linear Maps

A matrix $A \in M_{m \times n}(F)$ defines a linear map $L_A: F^n \to F^m$ by $L_A(\mathbf{x}) = A\mathbf{x}$ (matrix-vector multiplication).

Conversely, any linear $T: F^n \to F^m$ equals $L_A$ for a unique matrix $A$.

### 4.2.2 The Matrix of a Linear Map

Given bases $\mathcal{B} = (\mathbf{b}_1, \ldots, \mathbf{b}_n)$ of $V$ and $\mathcal{C} = (\mathbf{c}_1, \ldots, \mathbf{c}_m)$ of $W$, define the **matrix of $T$ relative to $\mathcal{B}$ and $\mathcal{C}$**:

$$[T]_{\mathcal{B}}^{\mathcal{C}} = \begin{pmatrix} | & & | \\ [T(\mathbf{b}_1)]_{\mathcal{C}} & \cdots & [T(\mathbf{b}_n)]_{\mathcal{C}} \\ | & & | \end{pmatrix}$$

The $j$-th column of $[T]_{\mathcal{B}}^{\mathcal{C}}$ is the coordinate vector of $T(\mathbf{b}_j)$ in the basis $\mathcal{C}$.

**Fundamental equation:**
$$[T(\mathbf{v})]_{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}} \cdot [\mathbf{v}]_{\mathcal{B}}$$

### 4.2.3 Composition and Matrix Multiplication

If $S: U \to V$ and $T: V \to W$ with bases $\mathcal{A}, \mathcal{B}, \mathcal{C}$:
$$[T \circ S]_{\mathcal{A}}^{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}} \cdot [S]_{\mathcal{A}}^{\mathcal{B}}$$

This is why matrix multiplication is defined as it is: it encodes composition of linear maps.

**Definition of matrix product:** $(AB)_{ij} = \sum_k A_{ik} B_{kj}$ (the dot product of row $i$ of $A$ with column $j$ of $B$).

### 4.2.4 Matrix Algebra

For compatible matrices:
- Multiplication is associative: $(AB)C = A(BC)$
- Distributive: $A(B+C) = AB + AC$
- **Not commutative in general:** $AB \neq BA$
- Identity: $I_n A = A I_m = A$ for $A \in M_{m \times n}$

**Transpose:** $(AB)^T = B^T A^T$ — note the reversal.

---

## 4.3 Change of Basis

### 4.3.1 Transition Matrices

The **change-of-basis matrix** from $\mathcal{B}$ to $\mathcal{B}'$ is:
$$P = [\mathrm{id}_V]_{\mathcal{B}'}^{\mathcal{B}} \in M_{n \times n}(F)$$

Its columns are the coordinates of the new basis vectors $\mathbf{b}'_j$ expressed in the old basis $\mathcal{B}$.

**Coordinate transformation:** $[\mathbf{v}]_{\mathcal{B}} = P \cdot [\mathbf{v}]_{\mathcal{B}'}$.

### 4.3.2 Similarity

If $T: V \to V$ (an **endomorphism**) has matrix $A = [T]_{\mathcal{B}}$ in basis $\mathcal{B}$ and $A' = [T]_{\mathcal{B}'}$ in basis $\mathcal{B}'$, then:

$$A' = P^{-1} A P$$

Two matrices $A, A'$ are **similar** if $A' = P^{-1}AP$ for some invertible $P$.

**Key insight:** Similar matrices represent the same linear map in different bases. The question of finding the "best" basis — one that makes $A$ as simple as possible — is the central problem of Chapter 6 (canonical forms).

### 4.3.3 Invertible Maps and Matrices

$T: V \to W$ is an **isomorphism** if it is a bijective linear map. Then $T^{-1}: W \to V$ is also linear.

$A \in M_{n \times n}(F)$ is **invertible** (nonsingular) if $\exists A^{-1}$ with $AA^{-1} = A^{-1}A = I_n$.

**Equivalent conditions** (for square $n \times n$ matrix $A$):
- $A$ is invertible
- $\ker(L_A) = \{0\}$ (i.e., $L_A$ is injective)
- $L_A$ is surjective
- $\mathrm{rank}(A) = n$
- $\det(A) \neq 0$ (from Chapter 5)
- The columns of $A$ are linearly independent
- The rows of $A$ are linearly independent

---

## 4.4 Row Reduction and Computation

### 4.4.1 Row Echelon Form

**Elementary row operations:**
1. Swap two rows
2. Multiply a row by a nonzero scalar
3. Add a multiple of one row to another

**Row echelon form (REF):** Zeros below each leading entry (pivot); each pivot is to the right of the one above.

**Reduced row echelon form (RREF):** Additionally, each pivot is 1, and it is the only nonzero entry in its column.

### 4.4.2 Gauss-Jordan Elimination

Any matrix reduces to a unique RREF via elementary row operations. Use this to:
- Solve systems $A\mathbf{x} = \mathbf{b}$
- Compute $\ker(L_A)$ (set $\mathbf{b} = \mathbf{0}$)
- Compute $A^{-1}$ (augment $[A \mid I]$, row-reduce to $[I \mid A^{-1}]$)
- Determine rank

### 4.4.3 Rank and the Four Fundamental Subspaces

For $A \in M_{m \times n}(F)$ (thinking of $A$ as $T: F^n \to F^m$):

| Subspace | Of | Definition | Dimension |
|----------|----|------------|-----------|
| Column space | $F^m$ | $\mathrm{im}(A) = \mathrm{span}(\text{columns})$ | $r = \mathrm{rank}(A)$ |
| Null space | $F^n$ | $\ker(A) = \{\mathbf{x} \mid A\mathbf{x} = \mathbf{0}\}$ | $n - r$ |
| Row space | $F^n$ | $\mathrm{span}(\text{rows})$ | $r$ |
| Left null space | $F^m$ | $\ker(A^T)$ | $m - r$ |

**Fact:** $\mathrm{rank}(A) = \mathrm{rank}(A^T)$ (row rank equals column rank).

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Rank-Nullity | $\mathrm{rank}(T) + \mathrm{nullity}(T) = \dim V$ |
| Linear maps determined by basis images | $T$ is determined by $T(\mathbf{b}_i)$; any assignment extends uniquely to a linear map |
| Matrix of composition | $[T \circ S]_{\mathcal{A}}^{\mathcal{C}} = [T]_{\mathcal{B}}^{\mathcal{C}}[S]_{\mathcal{A}}^{\mathcal{B}}$ |
| Similarity | Change of basis gives similar matrices |
| RREF uniqueness | Every matrix has a unique RREF |

---

## Milestone Exercises

1. Let $T: \mathbb{R}^3 \to \mathbb{R}^2$ by $T(x,y,z) = (x+y, y+z)$. Find $\ker T$, $\mathrm{im}\, T$, and verify rank-nullity.

2. Let $D: \mathbb{R}[x]_{\leq 3} \to \mathbb{R}[x]_{\leq 3}$ be differentiation. Find the matrix of $D$ relative to the standard basis $\{1, x, x^2, x^3\}$. Compute $D^2$ from the matrix.

3. Find all matrices that commute with $A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \end{pmatrix}$.

4. Prove: if $T: V \to V$ is linear and $V$ is finite-dimensional, then $T$ is injective iff $T$ is surjective.

5. Let $\mathcal{B} = \{(1,1), (1,-1)\}$ be a basis of $\mathbb{R}^2$. Find the change-of-basis matrix from $\mathcal{B}$ to the standard basis, and express $T(x,y) = (2x-y, x+3y)$ as a matrix in the $\mathcal{B}$-basis.

6. Prove that the row rank and column rank of any matrix are equal.

7. If $A, B \in M_{n \times n}(F)$ with $AB = I$, prove $BA = I$. (Hint: rank-nullity.)

---

## Connections Forward

- **Chapter 5:** Determinants measure whether a square map is invertible; they are defined via the multilinear structure of matrices.
- **Chapter 6:** Canonical forms answer: "What is the simplest similar matrix?"
- **Chapter 7:** Adjoint maps require an inner product on $V$ and $W$.
- **Chapter 12:** Replace $F$ by a ring $R$; "vector spaces over $R$" are modules. Linear maps become module homomorphisms.

---

*Next: [Chapter 5 — Determinants and Multilinear Forms](ch05-determinants-and-multilinear-forms.md)*
