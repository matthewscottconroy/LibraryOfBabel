# Core Matrix Decompositions

There is a theorem in linear algebra that, when biologists first encounter it applied to gene expression data, often produces a kind of intellectual vertigo. A matrix of 10,000 genes measured across 200 cancer samples — two million numbers — can be almost perfectly reconstructed from just 20 numbers describing the dominant patterns, plus the 20 directions those patterns point in. The mechanism behind this miracle is the singular value decomposition, and it works because biological data is not random: cells of the same type share systematic patterns of gene expression that dominate the variation, and the mathematical structure of those patterns is exactly what SVD reveals.

Matrix decompositions — also called factorizations — are algorithms that express a matrix as a product of simpler matrices with known structure. They are not just numerical conveniences; each decomposition reveals deep geometric and algebraic structure. For computational biologists, eigendecomposition and singular value decomposition are the most important tools in the entire linear algebra toolbox.

## Eigenvalues and Eigenvectors

Given a square matrix $A \in \mathbb{R}^{n \times n}$, an **eigenvector** $\mathbf{v} \neq 0$ and **eigenvalue** $\lambda$ satisfy:

$$A\mathbf{v} = \lambda \mathbf{v}$$

The matrix acts on its eigenvectors by pure scaling — no rotation. The set of all eigenvalues is the **spectrum** of $A$.

**Computing eigenvalues:** Eigenvalues satisfy $\det(A - \lambda I) = 0$, the **characteristic polynomial**. For $n > 4$, this polynomial has no general closed-form solution (Abel-Ruffini theorem), so eigenvalues are computed numerically. In practice, this means using `numpy.linalg.eigvals` or similar routines, which implement efficient algorithms (variants of the QR algorithm) that converge in $O(n^3)$ time.

**Biological applications of eigenvalues:**

1. **ODE stability analysis.** As derived in the previous subsection, the eigenvalues of the Jacobian $J$ at a fixed point determine stability. If $\text{Re}(\lambda_i) < 0$ for all $i$, the steady state is stable. A pair of purely imaginary eigenvalues $\pm i\omega$ signals a Hopf bifurcation point — the birth of oscillations with angular frequency $\omega$.

2. **Markov chain stationary distribution.** A transition matrix $P$ (where $P_{ij}$ is the probability of going from state $i$ to state $j$) always has eigenvalue $\lambda = 1$ with eigenvector equal to the stationary distribution. Computing the dominant eigenvector gives the long-run probability of each state — used for CpG island models, gene prediction HMMs, and DNA methylation dynamics.

3. **Network centrality.** The eigenvector centrality of a network node is its component in the leading eigenvector of the adjacency matrix. Google's PageRank algorithm is a variant of eigenvector centrality applied to the web graph; the same idea applied to protein interaction networks identifies hubs. The eigenvector centrality of TP53 in a human protein interaction network is one quantitative measure of why it is the most frequently mutated gene in cancer.

## Singular Value Decomposition

The **Singular Value Decomposition (SVD)** of any $m \times n$ matrix $A$ is:

$$A = U \Sigma V^T$$

where $U \in \mathbb{R}^{m \times m}$ and $V \in \mathbb{R}^{n \times n}$ are **orthogonal matrices** (columns are orthonormal vectors) and $\Sigma \in \mathbb{R}^{m \times n}$ is **diagonal** with non-negative entries $\sigma_1 \geq \sigma_2 \geq \cdots \geq \sigma_r \geq 0$ called **singular values**.

The columns of $U$ are the left singular vectors; the columns of $V$ are the right singular vectors. The singular values measure the importance of each component. Here is the key geometric insight: SVD decomposes a matrix into a sum of rank-1 contributions, each weighted by its singular value. The first term $\sigma_1 \mathbf{u}_1 \mathbf{v}_1^T$ captures the dominant pattern; the second term captures the second most important orthogonal pattern; and so on.

**Connection to PCA:** Given a mean-centered data matrix $X \in \mathbb{R}^{n \times p}$ (n samples, p genes), the SVD $X = U\Sigma V^T$ gives:
- **Principal components (PCs):** The right singular vectors $V$ (or columns of $V$) are the principal directions in gene space.
- **Scores:** The matrix $U\Sigma$ contains the projection of each sample onto each PC.
- **Variance explained:** The fraction of total variance explained by PC $k$ is $\sigma_k^2 / \sum_i \sigma_i^2$.

**Low-rank approximation:** The best rank-$k$ approximation of $A$ (in the Frobenius norm) is:

$$A_k = U_k \Sigma_k V_k^T$$

where $U_k$, $\Sigma_k$, $V_k$ use only the first $k$ components. This is the mathematical basis of dimensionality reduction: a 20,000-gene expression dataset can often be faithfully represented by 20–50 principal components. The Eckart-Young theorem guarantees that this truncated SVD is the *optimal* low-rank approximation — no other rank-$k$ matrix is closer to $A$ in the Frobenius norm. The biological interpretation: you are retaining the dominant sources of variation (cell type differences, cell cycle state, tissue origin) while discarding the noise.

**Worked example — PCA on RNA-seq data:**

```python
import numpy as np

# Simulate RNA-seq count matrix: 200 samples x 1000 genes
np.random.seed(42)
n_samples, n_genes = 200, 1000
# Two cell types differ in 100 genes
X = np.random.randn(n_samples, n_genes) * 2
X[:100, :100] += 5   # type A: high expression in first 100 genes
X[100:, 100:200] += 5  # type B: high expression in genes 100-200

# Center the data
X_centered = X - X.mean(axis=0)

# SVD
U, S, Vt = np.linalg.svd(X_centered, full_matrices=False)

# Variance explained
var_explained = S**2 / np.sum(S**2)
print(f"PC1 explains {var_explained[0]*100:.1f}% of variance")
print(f"PC2 explains {var_explained[1]*100:.1f}% of variance")

# Scores for plotting
scores = U * S  # equivalent to X_centered @ Vt.T
# scores[:100, 0] vs scores[100:, 0] should separate cell types
```

## LU Decomposition

**LU decomposition** factors a square matrix $A = LU$ where $L$ is lower triangular and $U$ is upper triangular. This is numerically equivalent to Gaussian elimination and is the standard method for solving $A\mathbf{x} = \mathbf{b}$ in $O(n^3)$ time.

Once $A = LU$, solving $LU\mathbf{x} = \mathbf{b}$ reduces to:
1. Forward substitution: solve $L\mathbf{y} = \mathbf{b}$ in $O(n^2)$
2. Backward substitution: solve $U\mathbf{x} = \mathbf{y}$ in $O(n^2)$

For systems where $A$ does not change but $\mathbf{b}$ varies — for example, solving the same linear system for multiple right-hand sides in a simulation — LU decomposition allows reusing the factorization at $O(n^2)$ cost per new $\mathbf{b}$. This is not a minor optimization: in metabolic modeling, you often need to solve $S\mathbf{v} = \mathbf{c}$ for many different right-hand sides during flux sampling, and reusing the LU factorization of $S$ turns an expensive operation into a cheap one.

## QR Decomposition

**QR decomposition** factors $A = QR$ where $Q$ is orthogonal and $R$ is upper triangular. QR is used for:
- Solving least squares problems ($\min \|A\mathbf{x} - \mathbf{b}\|^2$) in a numerically stable way
- Computing eigenvalues via the QR algorithm
- Gram-Schmidt orthogonalization

In bioinformatics, least squares fitting of linear models to expression data is solved via QR decomposition under the hood in R's `lm()` and `scipy.linalg.lstsq`. When you fit a differential expression model with limma or DESeq2, QR decomposition is happening inside.

## Why This Matters for Computational Biology

Eigendecomposition and SVD are the two most important computational tools in quantitative genomics. Every scRNA-seq analysis pipeline uses PCA (via SVD) as its first dimensionality reduction step. Stability analysis of gene regulatory networks requires computing the eigenvalues of the Jacobian. Flux balance analysis requires finding the null space of the stoichiometric matrix — which comes from the SVD. GWAS uses linear regression (solved via QR) to associate genetic variants with phenotypes. These decompositions are not abstract — they are running inside every bioinformatics tool you use.
