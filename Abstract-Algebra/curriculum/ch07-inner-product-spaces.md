# Chapter 7 — Inner Product Spaces and Spectral Theory

**Part II: Linear Algebra**
*Prerequisites: [Chapter 6](ch06-eigentheory-and-canonical-forms.md)*
*Next: [Chapter 8 — Multilinear Algebra and Tensor Products](ch08-multilinear-algebra-tensors.md)*

---

## Learning Objectives

- Define inner products and norms; work with real and complex inner products
- Apply Gram–Schmidt orthogonalization to produce orthonormal bases
- Understand adjoint maps and classify operators by their relation to their adjoint
- Prove the spectral theorem for normal operators (real and complex versions)
- Compute the singular value decomposition (SVD)
- Recognize inner products as the bridge between linear algebra and analysis

---

## 7.1 Inner Products

### 7.1.1 Real Inner Products

An **inner product** on a real vector space $V$ is a map $\langle \cdot, \cdot \rangle: V \times V \to \mathbb{R}$ satisfying:
- (IP1) **Bilinearity:** Linear in each argument
- (IP2) **Symmetry:** $\langle \mathbf{u}, \mathbf{v} \rangle = \langle \mathbf{v}, \mathbf{u} \rangle$
- (IP3) **Positive definiteness:** $\langle \mathbf{v}, \mathbf{v} \rangle \geq 0$, with equality iff $\mathbf{v} = \mathbf{0}$

### 7.1.2 Complex Inner Products (Hermitian)

On a complex vector space $V$, a **Hermitian inner product** satisfies:
- (IP1') **Sesquilinearity:** Linear in the second argument, **conjugate-linear** in the first: $\langle c\mathbf{u}, \mathbf{v} \rangle = \bar{c}\langle \mathbf{u}, \mathbf{v} \rangle$
- (IP2') **Conjugate symmetry:** $\langle \mathbf{u}, \mathbf{v} \rangle = \overline{\langle \mathbf{v}, \mathbf{u} \rangle}$
- (IP3) Positive definiteness

**Note:** Convention varies — some authors put the conjugate on the second argument. Fix a convention and stick to it.

### 7.1.3 Standard Examples

| Space | Inner product |
|-------|---------------|
| $\mathbb{R}^n$ | $\langle \mathbf{x}, \mathbf{y} \rangle = \mathbf{x}^T \mathbf{y} = \sum_i x_i y_i$ |
| $\mathbb{C}^n$ | $\langle \mathbf{x}, \mathbf{y} \rangle = \mathbf{x}^* \mathbf{y} = \sum_i \bar{x}_i y_i$ |
| $M_n(\mathbb{R})$ | $\langle A, B \rangle = \mathrm{tr}(A^T B)$ |
| $\mathcal{L}^2([a,b])$ | $\langle f, g \rangle = \int_a^b f(t)\overline{g(t)}\, dt$ |

### 7.1.4 Norms and Distance

The **norm** induced by an inner product: $\|\mathbf{v}\| = \sqrt{\langle \mathbf{v}, \mathbf{v} \rangle}$.

**Cauchy–Schwarz inequality:** $|\langle \mathbf{u}, \mathbf{v} \rangle| \leq \|\mathbf{u}\| \cdot \|\mathbf{v}\|$, with equality iff $\mathbf{u}, \mathbf{v}$ are proportional.

**Triangle inequality:** $\|\mathbf{u} + \mathbf{v}\| \leq \|\mathbf{u}\| + \|\mathbf{v}\|$.

**Parallelogram law:** $\|\mathbf{u} + \mathbf{v}\|^2 + \|\mathbf{u} - \mathbf{v}\|^2 = 2(\|\mathbf{u}\|^2 + \|\mathbf{v}\|^2)$.

---

## 7.2 Orthogonality

### 7.2.1 Orthogonal Vectors and Subspaces

$\mathbf{u} \perp \mathbf{v}$ if $\langle \mathbf{u}, \mathbf{v} \rangle = 0$. A set $S$ is **orthogonal** if all pairs are perpendicular; **orthonormal** if additionally each vector has norm 1.

**Theorem:** An orthogonal set of nonzero vectors is linearly independent.

For a subspace $W \subseteq V$:
$$W^\perp = \{\mathbf{v} \in V \mid \langle \mathbf{v}, \mathbf{w} \rangle = 0 \text{ for all } \mathbf{w} \in W\}$$

**Theorem:** $V = W \oplus W^\perp$ (orthogonal direct sum) when $V$ is finite-dimensional.

**Orthogonal projection** onto $W$: $\mathrm{proj}_W \mathbf{v} =$ the unique element of $W$ closest to $\mathbf{v}$.

### 7.2.2 Gram–Schmidt Orthogonalization

**Algorithm:** Given a basis $\{\mathbf{v}_1, \ldots, \mathbf{v}_n\}$, produce an orthonormal basis $\{\mathbf{e}_1, \ldots, \mathbf{e}_n\}$:

$$\mathbf{u}_k = \mathbf{v}_k - \sum_{j=1}^{k-1} \frac{\langle \mathbf{v}_k, \mathbf{u}_j \rangle}{\langle \mathbf{u}_j, \mathbf{u}_j \rangle} \mathbf{u}_j, \qquad \mathbf{e}_k = \frac{\mathbf{u}_k}{\|\mathbf{u}_k\|}$$

**Consequence (QR decomposition):** Every matrix $A$ with linearly independent columns factors as $A = QR$ where $Q$ has orthonormal columns and $R$ is upper triangular with positive diagonal entries.

---

## 7.3 Adjoint Maps

### 7.3.1 The Adjoint

Given $T: V \to W$ between inner product spaces, the **adjoint** $T^*: W \to V$ is the unique map satisfying:
$$\langle T\mathbf{v}, \mathbf{w} \rangle_W = \langle \mathbf{v}, T^*\mathbf{w} \rangle_V \quad \text{for all } \mathbf{v} \in V, \mathbf{w} \in W$$

In matrix terms (with orthonormal bases): if $T$ has matrix $A$, then $T^*$ has matrix $A^*$ (conjugate transpose, $A^* = \bar{A}^T$). Over $\mathbb{R}$: $A^* = A^T$.

### 7.3.2 Self-Adjoint and Normal Operators

| Name | Condition | Matrix |
|------|-----------|--------|
| Self-adjoint (Hermitian) | $T^* = T$ | $A^* = A$ |
| Skew-Hermitian | $T^* = -T$ | $A^* = -A$ |
| Unitary (orthogonal over $\mathbb{R}$) | $T^* = T^{-1}$ | $A^* A = I$ |
| Normal | $T^*T = TT^*$ | $A^*A = AA^*$ |

Self-adjoint, skew-Hermitian, and unitary operators are all normal.

---

## 7.4 The Spectral Theorem

### 7.4.1 Complex Spectral Theorem

**Theorem:** A complex linear operator $T$ on a finite-dimensional inner product space is **normal** if and only if $V$ has an orthonormal basis of eigenvectors of $T$.

Equivalently: $T$ is unitarily diagonalizable ($T = U D U^*$ with $U$ unitary, $D$ diagonal).

**Eigenvalue properties of normal operators:**
- Self-adjoint: eigenvalues are real
- Unitary: eigenvalues have $|\lambda| = 1$
- Positive semidefinite ($T^* = T$, $\langle T\mathbf{v},\mathbf{v}\rangle \geq 0$): eigenvalues $\geq 0$

### 7.4.2 Real Spectral Theorem

**Theorem:** A real linear operator $T$ on a finite-dimensional real inner product space is **self-adjoint** ($T^* = T$) if and only if $V$ has an orthonormal basis of eigenvectors of $T$.

Equivalently: $T$ is orthogonally diagonalizable ($T = Q D Q^T$ with $Q$ orthogonal, $D$ real diagonal).

### 7.4.3 Why the Spectral Theorem Matters

It says: self-adjoint operators are "diagonal in the right basis." The basis is orthonormal, so it respects the geometry. This is the mathematical foundation of:
- **Principal component analysis (PCA):** The eigenvectors of the covariance matrix
- **Quantum mechanics:** Observable operators are self-adjoint; their eigenvectors are measurement states
- **Fourier analysis:** The Laplacian is self-adjoint; its eigenfunctions (sines, cosines) form an orthonormal basis

---

## 7.5 Singular Value Decomposition

### 7.5.1 Singular Values

For any $T: V \to W$ (not necessarily square), the **singular values** $\sigma_1 \geq \sigma_2 \geq \cdots \geq 0$ are the square roots of the eigenvalues of the self-adjoint operator $T^*T: V \to V$.

Since $T^*T$ is positive semidefinite, its eigenvalues are non-negative.

### 7.5.2 SVD Theorem

**Theorem:** Every matrix $A \in M_{m \times n}(\mathbb{C})$ can be written:
$$A = U \Sigma V^*$$
where:
- $U \in M_{m \times m}$ is unitary
- $V \in M_{n \times n}$ is unitary
- $\Sigma \in M_{m \times n}$ has $\sigma_1 \geq \sigma_2 \geq \cdots \geq 0$ on the main diagonal and zeros elsewhere

The singular values are unique; $U$ and $V$ are not (when singular values repeat).

### 7.5.3 Applications

- **Rank:** $\mathrm{rank}(A) =$ number of nonzero singular values
- **Pseudoinverse:** $A^+ = V \Sigma^+ U^*$ (replace nonzero diagonal entries of $\Sigma$ with their reciprocals)
- **Least squares:** $A\mathbf{x} \approx \mathbf{b}$ solution is $\mathbf{x} = A^+ \mathbf{b}$
- **Low-rank approximation:** Truncate to top $k$ singular values → best rank-$k$ approximation (Eckart–Young theorem)
- **Data compression, PCA, latent semantic analysis**

---

## 7.6 Bilinear Forms

### 7.6.1 General Bilinear Forms

A **bilinear form** on $V$ is a map $B: V \times V \to F$ linear in each argument. In matrix form (with basis $\mathcal{B}$):
$$B(\mathbf{u}, \mathbf{v}) = [\mathbf{u}]_{\mathcal{B}}^T M [\mathbf{v}]_{\mathcal{B}}$$

**Symmetric forms** ($M = M^T$), **skew-symmetric forms** ($M = -M^T$), **Hermitian forms**.

### 7.6.2 Sylvester's Law of Inertia

For a real symmetric bilinear form, there exists a basis in which the matrix is diagonal with entries in $\{+1, -1, 0\}$. The **signature** $(p, q, r)$ (counts of $+1$, $-1$, $0$) is a basis-independent invariant.

**Theorem (Sylvester):** The signature is an invariant — it does not depend on the choice of diagonalizing basis.

---

## Key Theorems

| Theorem | Statement |
|---------|-----------|
| Cauchy–Schwarz | $\|\langle \mathbf{u},\mathbf{v}\rangle\| \leq \|\mathbf{u}\|\|\mathbf{v}\|$ |
| Gram–Schmidt | Every finite-dim inner product space has an orthonormal basis |
| Spectral theorem (complex) | $T$ normal $\Leftrightarrow$ unitarily diagonalizable |
| Spectral theorem (real) | $T$ self-adjoint $\Leftrightarrow$ orthogonally diagonalizable |
| SVD | Every matrix $A = U\Sigma V^*$ |
| Sylvester's law | Signature of real symmetric forms is invariant |

---

## Milestone Exercises

1. Apply Gram–Schmidt to $\{(1,1,0),(1,0,1),(0,1,1)\} \subset \mathbb{R}^3$.

2. Prove: the eigenvalues of a self-adjoint operator are real.

3. Prove: eigenvectors of a normal operator for distinct eigenvalues are orthogonal.

4. Find the SVD of $A = \begin{pmatrix} 1 & 1 \\ 0 & 1 \\ 1 & 0 \end{pmatrix}$.

5. Show that the set of $n \times n$ orthogonal matrices forms a group under multiplication. (This is the orthogonal group $O(n)$ — a Lie group from Chapter 20.)

6. Classify real symmetric $2 \times 2$ matrices by their signature. Give a geometric description of each case as a quadratic form on $\mathbb{R}^2$.

7. Prove the Eckart–Young theorem: the best rank-$k$ approximation to $A$ in Frobenius norm is $A_k = U_k \Sigma_k V_k^*$ (truncated SVD).

---

## Connections Forward

- **Chapter 8:** The bilinear form perspective generalizes to multilinear forms; exterior products encode skew-symmetric forms.
- **Chapter 19:** Characters of unitary representations use the spectral theorem heavily.
- **Chapter 20:** Lie groups $O(n)$, $U(n)$, $Sp(n)$ are the symmetry groups of inner products.
- **Chapter 21:** The Killing form on a Lie algebra is a symmetric bilinear form; semisimplicity $\Leftrightarrow$ non-degeneracy.

---

*Next: [Chapter 8 — Multilinear Algebra and Tensor Products](ch08-multilinear-algebra-tensors.md)*
