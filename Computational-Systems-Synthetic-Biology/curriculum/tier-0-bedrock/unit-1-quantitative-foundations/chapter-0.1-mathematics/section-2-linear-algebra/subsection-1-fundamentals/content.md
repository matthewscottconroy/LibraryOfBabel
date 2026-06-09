# Linear Algebra Fundamentals

In 2002, the first large-scale gene expression microarray studies began producing data matrices with thousands of rows (genes) and hundreds of columns (samples). Biologists suddenly faced a question that mathematicians had been thinking about for over a century: how do you find structure in a high-dimensional dataset? The answer came from linear algebra — specifically from the singular value decomposition, a tool for extracting the dominant patterns buried in a matrix. Today, every single-cell RNA-seq analysis begins with exactly this operation.

Linear algebra is the mathematics of linear transformations and their representations as matrices. It is the backbone of genomic data analysis, network modeling, and numerical computation in biology. Understanding vectors, matrices, and their operations fluently is as important as knowing calculus — perhaps more so for day-to-day computational work. Where calculus tells you how things change, linear algebra tells you how things are *related* — and in biology, relationships are everything.

## Vectors and Vector Spaces

A **vector** is an ordered list of numbers: $\mathbf{v} = (v_1, v_2, \ldots, v_n)^T \in \mathbb{R}^n$. In biology, vectors represent gene expression profiles (a vector in $\mathbb{R}^{20000}$, one entry per gene), concentration states of a metabolic network, or flux distributions through a pathway. The abstraction is powerful: once you recognize that a cell's transcriptional state is a point in a 20,000-dimensional space, the question "are these two cells similar?" becomes "how close are these two points?" — a geometric question with a clean mathematical answer.

A **vector space** is a set $V$ closed under vector addition and scalar multiplication, satisfying eight axioms. The key intuition: vector spaces are sets where you can meaningfully interpolate and scale. The space $\mathbb{R}^n$ of $n$-dimensional real vectors is the canonical example.

A **basis** for a vector space is a minimal set of vectors $\{\mathbf{b}_1, \ldots, \mathbf{b}_k\}$ such that every vector in the space can be written as a linear combination $\mathbf{v} = \sum_i c_i \mathbf{b}_i$. The **dimension** of a vector space equals the number of basis vectors. In genomics, we work in $\mathbb{R}^{n_{\text{genes}}}$ but the actual data (samples) lies in a lower-dimensional subspace — the insight that motivates PCA. The fact that 10,000 cells can be well-described by 30 principal components is a statement that gene expression data lives on a low-dimensional manifold within its nominal high-dimensional space.

**Inner product:** The **dot product** of two vectors is $\mathbf{u} \cdot \mathbf{v} = \sum_i u_i v_i = \|\mathbf{u}\| \|\mathbf{v}\| \cos\theta$, where $\theta$ is the angle between them. The dot product is zero when vectors are orthogonal — in data analysis, orthogonality between principal components means they capture independent sources of variation. This is why PCA components are interpretable: each one describes a pattern that is, by construction, distinct from all the others.

## Matrix Operations

A **matrix** $A \in \mathbb{R}^{m \times n}$ is a rectangular array with $m$ rows and $n$ columns. Matrices represent linear transformations, datasets (rows = samples, columns = features), network adjacency, or stoichiometric coefficients. In biology, the same mathematical object — a matrix — shows up in completely different guises: the data matrix for a genome-wide association study, the stoichiometric matrix of a metabolic model, and the adjacency matrix of a protein interaction network are all just arrays of numbers waiting to be analyzed with the same toolkit.

**Matrix multiplication:** $(AB)_{ij} = \sum_k A_{ik} B_{kj}$. This is the composition of linear transformations. For matrices to be multiplied, the number of columns of $A$ must equal the number of rows of $B$. Matrix multiplication is generally not commutative: $AB \neq BA$.

**Transpose:** $(A^T)_{ij} = A_{ji}$. The covariance matrix $C = \frac{1}{n-1} X^T X$ (where $X$ is a mean-centered data matrix) is computed via a transpose-multiply, and its eigenvectors are the principal components.

**Inverse:** For a square matrix $A$, the inverse $A^{-1}$ satisfies $A A^{-1} = I$ (identity matrix). Not all matrices are invertible — only those with nonzero determinant. In biological network analysis, singular matrices (with zero determinant) have a special structure: they represent systems with conservation laws or redundant constraints. When a metabolic network's stoichiometric matrix is singular, it signals a conserved moiety — a pool of material (like adenine nucleotides) whose total is constant regardless of which reactions are active.

**Matrix-vector multiplication:** $A\mathbf{x} = \mathbf{b}$ is a system of $m$ linear equations in $n$ unknowns. Solving this system is the core computation in flux balance analysis ($S\mathbf{v} = 0$), linear regression ($X^T X \boldsymbol{\beta} = X^T \mathbf{y}$), and the discretized diffusion equation.

## Determinants

The **determinant** $\det(A)$ of a square matrix is a scalar that encodes:
- Whether $A$ is invertible ($\det(A) \neq 0 \Leftrightarrow A$ is invertible)
- The signed volume scaling factor of the linear transformation represented by $A$
- For $2 \times 2$ matrices: $\det\begin{pmatrix} a & b \\ c & d \end{pmatrix} = ad - bc$

In stability analysis, $\det(J)$ and $\text{tr}(J)$ (trace) determine the character of a fixed point in a 2D ODE system without computing eigenvalues explicitly:
- $\det(J) < 0$: saddle point
- $\det(J) > 0$, $\text{tr}(J) < 0$: stable (spiral or node)
- $\det(J) > 0$, $\text{tr}(J) > 0$: unstable (spiral or node)

This is a remarkably useful shortcut: rather than computing eigenvalues of the Jacobian at every candidate steady state, you can classify the fixed point geometrically from just two numbers.

## Linear Transformations

A **linear transformation** $T: \mathbb{R}^n \to \mathbb{R}^m$ satisfies $T(\alpha\mathbf{u} + \beta\mathbf{v}) = \alpha T(\mathbf{u}) + \beta T(\mathbf{v})$. Every linear transformation can be represented as matrix multiplication. Geometrically, linear transformations preserve lines and the origin — they can rotate, scale, reflect, and shear, but cannot translate or curve.

Understanding that a matrix *is* a linear transformation helps build intuition:
- The stoichiometric matrix $S$ maps a flux vector $\mathbf{v}$ to rate-of-change vector $\dot{\mathbf{x}} = S\mathbf{v}$
- The covariance matrix maps a data vector to its projection onto the directions of maximum variance

**Worked example — stoichiometry:** In a simple pathway $A \to B \to C$, with fluxes $v_1$ (A consumed, B produced) and $v_2$ (B consumed, C produced):

$$S = \begin{pmatrix} -1 & 0 \\ 1 & -1 \\ 0 & 1 \end{pmatrix}, \quad \dot{\mathbf{x}} = S\mathbf{v} = \begin{pmatrix} -v_1 \\ v_1 - v_2 \\ v_2 \end{pmatrix}$$

At steady state, $S\mathbf{v} = 0$ implies $v_1 = v_2$: flux is conserved through the linear pathway. This result, so obvious when stated in words, falls out automatically from the linear algebra without any biological reasoning at all. That is what makes the stoichiometric matrix formalism powerful: biological constraints are encoded as linear constraints on a flux vector.

## Why This Matters for Computational Biology

Linear algebra is the computational substrate of modern bioinformatics. PCA on gene expression data, network centrality calculations, solving for steady-state fluxes in metabolic models, fitting linear regression models to phenotype data — all reduce to matrix operations. Understanding the geometry of vector spaces helps you reason about when data is well-conditioned or degenerate, when a system has unique solutions or infinitely many, and how transformations preserve or destroy information. Every NumPy array operation you write is linear algebra.

```python
import numpy as np

# Stoichiometric matrix for a simple pathway A -> B -> C
S = np.array([[-1,  0],
              [ 1, -1],
              [ 0,  1]], dtype=float)

# Flux vector at steady state
v = np.array([2.5, 2.5])   # Equal fluxes at steady state
x_dot = S @ v
print(f"dx/dt at steady state: {x_dot}")  # Should be [0, 0, 0]

# Solve a linear system: Ax = b
A = np.array([[3, 1], [1, 2]], dtype=float)
b = np.array([9, 8], dtype=float)
x = np.linalg.solve(A, b)
print(f"Solution: {x}")

# Verify
print(f"Ax = {A @ x}, b = {b}, match: {np.allclose(A @ x, b)}")

# Determinant and trace for stability analysis
J = np.array([[-0.5, -1.0],
              [ 0.8, -0.3]])
det_J = np.linalg.det(J)
tr_J = np.trace(J)
print(f"\nJacobian det = {det_J:.3f}, trace = {tr_J:.3f}")
print(f"Fixed point is {'stable' if det_J > 0 and tr_J < 0 else 'unstable or saddle'}")
```
