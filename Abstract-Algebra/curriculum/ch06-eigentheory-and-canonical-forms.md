# Chapter 6 — Eigentheory and Canonical Forms

**Part II: Linear Algebra**
*Prerequisites: [Chapter 5](ch05-determinants-and-multilinear-forms.md)*
*Next: [Chapter 7 — Inner Product Spaces and Spectral Theory](ch07-inner-product-spaces.md)*

---

## Learning Objectives

- Find eigenvalues and eigenvectors; understand their geometric meaning
- Diagonalize operators when possible; understand when diagonalization fails
- Compute and understand the Jordan canonical form
- Understand the minimal polynomial and Cayley–Hamilton theorem
- Work with the rational canonical form (companion matrices)
- Recognize canonical forms as the resolution to the similarity problem

---

## 6.1 Eigenvalues and Eigenvectors

### 6.1.1 Definitions

Let $T: V \to V$ be a linear operator on a finite-dimensional $F$-vector space. A **scalar** $\lambda \in F$ is an **eigenvalue** of $T$ if there exists a nonzero $\mathbf{v} \in V$ with:
$$T(\mathbf{v}) = \lambda \mathbf{v}$$

Such $\mathbf{v}$ is an **eigenvector** of $T$ with eigenvalue $\lambda$.

**Geometric meaning:** Eigenvectors are directions preserved by $T$; they are only scaled (by $\lambda$), not rotated.

### 6.1.2 Finding Eigenvalues

$\lambda$ is an eigenvalue $\Leftrightarrow$ $T - \lambda I$ is not injective $\Leftrightarrow$ $\det(\lambda I - A) = 0$.

The **characteristic polynomial**:
$$p_T(\lambda) = \det(\lambda I - A) = \lambda^n - \mathrm{tr}(A)\lambda^{n-1} + \cdots + (-1)^n \det(A)$$

Eigenvalues are roots of $p_T(\lambda)$.

**Note:** Eigenvalues depend on the field $F$. Over $\mathbb{R}$, some matrices have no real eigenvalues (e.g., rotation by 90°). Over $\mathbb{C}$, every $n \times n$ matrix has exactly $n$ eigenvalues (counted with multiplicity), since $\mathbb{C}$ is algebraically closed.

### 6.1.3 Eigenspaces

The **eigenspace** of $\lambda$:
$$E_\lambda = \ker(\lambda I - T) = \{\mathbf{v} \in V \mid T\mathbf{v} = \lambda\mathbf{v}\}$$

This is a non-trivial subspace when $\lambda$ is an eigenvalue.

**Algebraic multiplicity** $m_a(\lambda)$: the multiplicity of $\lambda$ as a root of $p_T$.
**Geometric multiplicity** $m_g(\lambda)$: $\dim(E_\lambda)$.

**Theorem:** $1 \leq m_g(\lambda) \leq m_a(\lambda)$.

---

## 6.2 Diagonalization

### 6.2.1 When Is $T$ Diagonalizable?

$T$ is **diagonalizable** if $V$ has a basis of eigenvectors of $T$. Equivalently, $[T]_\mathcal{B}$ is diagonal for some basis $\mathcal{B}$.

**Theorem:** $T$ is diagonalizable $\Leftrightarrow$ $m_g(\lambda) = m_a(\lambda)$ for every eigenvalue $\lambda$.

Equivalently: $V = \bigoplus_\lambda E_\lambda$ (direct sum of eigenspaces).

**Sufficient condition:** If $p_T$ has $n$ distinct roots (in $F$), then $T$ is diagonalizable.

### 6.2.2 Algorithm for Diagonalization

1. Compute $p_T(\lambda) = \det(\lambda I - A)$
2. Factor $p_T$ over $F$
3. For each eigenvalue $\lambda_i$, compute $E_{\lambda_i} = \ker(\lambda_i I - A)$
4. Check: do the eigenspaces span $V$? If yes, $P$ = matrix of eigenvectors, $D$ = diagonal eigenvalue matrix, $A = PDP^{-1}$

### 6.2.3 Failure of Diagonalization

$A = \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix}$ has $p_A(\lambda) = \lambda^2$ (double eigenvalue $0$), but $\ker A = \mathrm{span}\{e_1\}$ is 1-dimensional. Not diagonalizable.

This is the prototypical **nilpotent** operator: $A^2 = 0$ but $A \neq 0$.

---

## 6.3 Jordan Canonical Form

### 6.3.1 Generalized Eigenvectors

For eigenvalue $\lambda$ with $m_g(\lambda) < m_a(\lambda)$, introduce **generalized eigenspaces**:
$$J_\lambda = \ker(T - \lambda I)^n$$

For large enough power, this stabilizes. The **generalized eigenspace decomposition**:
$$V = \bigoplus_\lambda J_\lambda$$

holds over any algebraically closed field.

### 6.3.2 Jordan Blocks

A **Jordan block** of size $k$ with eigenvalue $\lambda$:
$$J_k(\lambda) = \begin{pmatrix} \lambda & 1 & & \\ & \lambda & 1 & \\ & & \ddots & 1 \\ & & & \lambda \end{pmatrix} \in M_{k \times k}(F)$$

A $k \times k$ Jordan block has $\lambda$ on the diagonal and $1$'s on the superdiagonal.

### 6.3.3 Jordan Normal Form

**Theorem (Jordan Canonical Form):** Over an algebraically closed field $F$, every $T: V \to V$ is similar to a block diagonal matrix:
$$J = \mathrm{diag}(J_{k_1}(\lambda_1), J_{k_2}(\lambda_2), \ldots, J_{k_r}(\lambda_r))$$

The Jordan form is unique up to permutation of blocks.

**Reading the Jordan form:**
- Number of blocks with eigenvalue $\lambda$ = $m_g(\lambda) = \dim E_\lambda$
- Sum of block sizes for $\lambda$ = $m_a(\lambda)$
- Size of largest block for $\lambda$ = **index** of $\lambda$ (nilpotency degree of $T - \lambda I$ on $J_\lambda$)

### 6.3.4 Computing Jordan Form

1. Find eigenvalues and their algebraic multiplicities
2. For each $\lambda$, compute $\dim \ker(T - \lambda I)^k$ for $k = 1, 2, \ldots$ until stabilization
3. The block sizes for $\lambda$ are determined by the sequence of these dimensions

---

## 6.4 The Minimal Polynomial

### 6.4.1 Definition

The **minimal polynomial** $m_T(\lambda)$ of $T$ is the monic polynomial of least degree such that $m_T(T) = 0$.

**Theorem (Cayley–Hamilton):** $p_T(T) = 0$, so $m_T \mid p_T$.

**Theorem:** $m_T$ and $p_T$ have the same roots (same eigenvalues, possibly with different multiplicities).

### 6.4.2 Properties

- $m_T(\lambda) = \mathrm{lcm}$ of minimal polynomials of each Jordan block
- $T$ is diagonalizable $\Leftrightarrow$ $m_T$ has no repeated roots
- $T$ is nilpotent $\Leftrightarrow$ $m_T(\lambda) = \lambda^k$ for some $k$

### 6.4.3 Cayley–Hamilton Theorem

**Theorem:** Every square matrix satisfies its own characteristic polynomial: $p_A(A) = 0$.

**Proof sketch:** Verified directly for Jordan blocks; conjugation extends to all matrices since $p_{PAP^{-1}} = p_A$.

**Applications:**
- Express $A^k$ for large $k$ as a polynomial in $A$ of degree $< n$
- Compute $A^{-1}$ as a polynomial in $A$ (when $\det(A) \neq 0$)

---

## 6.5 Rational Canonical Form

### 6.5.1 Companion Matrices

For a monic polynomial $f(\lambda) = \lambda^k + a_{k-1}\lambda^{k-1} + \cdots + a_0$, its **companion matrix** is:
$$C(f) = \begin{pmatrix} 0 & 0 & \cdots & 0 & -a_0 \\ 1 & 0 & \cdots & 0 & -a_1 \\ 0 & 1 & \cdots & 0 & -a_2 \\ \vdots & & \ddots & & \vdots \\ 0 & 0 & \cdots & 1 & -a_{k-1} \end{pmatrix}$$

The minimal and characteristic polynomial of $C(f)$ both equal $f(\lambda)$.

### 6.5.2 Rational Canonical Form

**Theorem:** Over any field $F$, every $T$ is similar to a block diagonal matrix with companion matrix blocks:
$$\mathrm{diag}(C(f_1), C(f_2), \ldots, C(f_r))$$
where $f_1 \mid f_2 \mid \cdots \mid f_r$ (divisibility chain), and $f_r = m_T$.

**Advantage over Jordan:** Works over any field, not just algebraically closed ones. No eigenvalues needed.

**Connection:** Over an algebraically closed field, RCF and JCF both classify $T$ up to similarity and are equivalent (different decompositions of the same data).

---

## 6.6 Applications

- **Solving linear ODEs:** $\mathbf{x}' = A\mathbf{x}$ solved via eigendecomposition or Jordan form
- **Matrix exponential:** $e^{tA} = P e^{tJ} P^{-1}$; computable from Jordan blocks
- **Markov chains:** Eigenvalue 1 (Perron-Frobenius theorem); long-run behavior from dominant eigenvalue
- **Google PageRank:** Dominant eigenvector of the web link matrix
- **Quantum mechanics:** Observables are self-adjoint operators; measurements are eigenvalues

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| $1 \leq m_g \leq m_a$ | Geometric $\leq$ algebraic multiplicity |
| Diagonalizability criterion | $T$ diagonalizable $\Leftrightarrow$ $m_g = m_a$ for all eigenvalues |
| Jordan canonical form | Over alg. closed $F$, every $T \sim$ block diagonal Jordan matrix (unique up to permutation) |
| Cayley–Hamilton | $p_T(T) = 0$ |
| Rational canonical form | Over any field, every $T \sim$ RCF (companion blocks) |

---

## Milestone Exercises

1. Find all eigenvalues and eigenvectors of $A = \begin{pmatrix} 3 & 1 \\ 0 & 3 \end{pmatrix}$. Is $A$ diagonalizable?

2. Find the Jordan form of $A = \begin{pmatrix} 0 & 1 & 0 \\ 0 & 0 & 1 \\ 0 & 0 & 0 \end{pmatrix}$.

3. Use Cayley–Hamilton to compute $A^5$ for $A = \begin{pmatrix} 1 & 2 \\ 0 & 3 \end{pmatrix}$.

4. Find the minimal polynomial and rational canonical form of $A = \begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}$ over $\mathbb{R}$ and over $\mathbb{C}$.

5. Classify (up to similarity over $\mathbb{C}$) all $3 \times 3$ matrices with characteristic polynomial $(\lambda - 2)^3$.

6. Prove: $T$ is nilpotent (i.e., $T^k = 0$ for some $k$) $\Leftrightarrow$ all eigenvalues of $T$ are $0$.

7. Compute $e^{tA}$ for $A = \begin{pmatrix} 0 & 1 \\ -1 & 0 \end{pmatrix}$.

---

## Connections Forward

- **Chapter 7:** Spectral theorem for self-adjoint operators is diagonalizability under orthonormal constraints.
- **Chapter 12:** Structure theorem for finitely generated modules over PIDs is the algebraic generalization of Jordan/RCF.
- **Chapter 19:** Characters of representations carry eigenvalue data; character theory generalizes eigenvalue arguments.
- **Chapter 21:** Eigenvalues of the adjoint action of a Cartan subalgebra are roots — the Lie algebra generalization.

---

*Next: [Chapter 7 — Inner Product Spaces and Spectral Theory](ch07-inner-product-spaces.md)*
