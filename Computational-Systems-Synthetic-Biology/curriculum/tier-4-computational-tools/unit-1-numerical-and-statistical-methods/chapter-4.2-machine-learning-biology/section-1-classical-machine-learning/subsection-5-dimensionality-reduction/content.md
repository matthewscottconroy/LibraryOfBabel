# Dimensionality Reduction

Open any single-cell RNA sequencing paper published in the last decade and you will find, somewhere near the beginning, a cloud of colored dots arranged in a kidney-bean or arc shape on two axes labeled "UMAP 1" and "UMAP 2." Each dot is a cell. Each color is a cell type. The figure looks simple, almost casual — but it represents a collapse from 30,000-dimensional gene expression space down to two dimensions, and how you perform that collapse determines what you see and what you miss.

High-dimensional biological data — scRNA-seq with 30,000 genes, proteomics with 10,000 proteins, genome-wide SNP arrays with millions of variants — cannot be visualized or analyzed directly. **Dimensionality reduction** projects high-dimensional data into a lower-dimensional space that preserves the most biologically relevant structure. The three dominant methods in biology are **PCA** (linear, interpretable), **UMAP** (nonlinear, topology-preserving), and **t-SNE** (nonlinear, cluster-revealing). Understanding what each preserves — and what it distorts — is essential for correct interpretation.

## PCA: Linear Variance Decomposition

**Principal Component Analysis (PCA)** finds the orthogonal directions that account for the most variance in the data. Formally, PCA computes the **eigendecomposition** of the sample covariance matrix $\Sigma = X^TX/(n-1)$, or equivalently the **Singular Value Decomposition (SVD)** of the centered data matrix $X$:

$$X = U \Sigma V^T$$

The **principal components (PCs)** are the columns of $V$ (the right singular vectors). The **variance explained** by PC $k$ is $\sigma_k^2 / \sum_i \sigma_i^2$, where $\sigma_k$ are the singular values.

```python
import numpy as np
import matplotlib.pyplot as plt
from sklearn.decomposition import PCA
from sklearn.preprocessing import StandardScaler
import pandas as pd

# Simulated scRNA-seq: 1000 cells, 5000 highly variable genes
# Three cell types with different expression programs
rng = np.random.default_rng(42)
n_cells = 1000
n_genes = 5000

# Cell type labels
labels = rng.integers(0, 3, n_cells)
# Gene programs for each cell type
programs = rng.standard_normal((3, n_genes))
# Add cell-type-specific signal + noise
X = programs[labels] + 2.0 * rng.standard_normal((n_cells, n_genes))
X = np.maximum(X, 0)  # non-negative expression

# Standard preprocessing: log1p normalize
X_log = np.log1p(X)

# Scale: each gene has unit variance (important for PCA!)
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X_log)

# PCA
pca = PCA(n_components=50, random_state=42)
X_pca = pca.fit_transform(X_scaled)

print(f"PCA explained variance:")
for i, var in enumerate(pca.explained_variance_ratio_[:10]):
    print(f"  PC{i+1}: {var:.3f} ({pca.explained_variance_ratio_[:i+1].sum():.3f} cumulative)")

# Scree plot
fig, axes = plt.subplots(1, 2, figsize=(12, 4))

axes[0].bar(range(1, 21), pca.explained_variance_ratio_[:20], color='steelblue')
axes[0].set_xlabel('Principal component')
axes[0].set_ylabel('Fraction of variance explained')
axes[0].set_title('Scree plot')
axes[0].axhline(y=1/50, color='red', linestyle='--', label='Uniform (1/50)')
axes[0].legend()

# PC1 vs PC2 scatter colored by cell type
colors = ['C0', 'C1', 'C2']
for ct in range(3):
    mask = labels == ct
    axes[1].scatter(X_pca[mask, 0], X_pca[mask, 1],
                   c=colors[ct], label=f'Cell type {ct}', alpha=0.5, s=10)
axes[1].set_xlabel(f'PC1 ({pca.explained_variance_ratio_[0]:.1%} variance)')
axes[1].set_ylabel(f'PC2 ({pca.explained_variance_ratio_[1]:.1%} variance)')
axes[1].legend()
axes[1].set_title('PCA projection')

plt.tight_layout()
plt.savefig('pca_analysis.pdf')

# Gene loadings: which genes drive PC1?
loadings = pd.DataFrame(
    pca.components_.T,
    index=[f"Gene_{i:04d}" for i in range(n_genes)],
    columns=[f"PC{i+1}" for i in range(50)]
)
top_pc1_genes = loadings['PC1'].abs().sort_values(ascending=False)[:20]
print(f"\nTop 10 genes driving PC1:")
print(top_pc1_genes[:10])
```

## UMAP: Topology-Preserving Nonlinear Embedding

**UMAP** (Uniform Manifold Approximation and Projection, McInnes et al. 2018) builds a weighted k-nearest-neighbor graph in high-dimensional space, then finds a low-dimensional embedding that preserves the fuzzy topological structure of this graph. It is the dominant visualization tool for scRNA-seq and proteomics.

**Critical limitations that are commonly misunderstood:**
1. **Distances in UMAP space are NOT meaningful** — only neighborhood relationships (topology) are preserved
2. **Cluster sizes in UMAP space are NOT informative** — UMAP stretches and compresses clusters unpredictably
3. **Different runs with different random seeds give different layouts** — don't over-interpret specific arrangements
4. **Always run PCA first** — UMAP on 30,000 genes is unstable and slow; use top 30–50 PCs as input

```python
from umap import UMAP  # pip install umap-learn

# Always: PCA first, then UMAP on PCs
umap = UMAP(
    n_neighbors=30,    # local vs. global structure tradeoff; try 15-50
    min_dist=0.3,      # cluster compactness; try 0.1-0.5
    n_components=2,
    metric='euclidean',
    random_state=42
)
X_umap = umap.fit_transform(X_pca[:, :30])  # Use top 30 PCs!

fig, ax = plt.subplots(figsize=(7, 6))
for ct in range(3):
    mask = labels == ct
    ax.scatter(X_umap[mask, 0], X_umap[mask, 1],
               c=colors[ct], label=f'Cell type {ct}', alpha=0.6, s=8)
ax.set_xlabel('UMAP 1')
ax.set_ylabel('UMAP 2')
ax.legend()
ax.set_title('UMAP (n_neighbors=30, min_dist=0.3)')
plt.tight_layout()
plt.savefig('umap.pdf')

# Effect of n_neighbors on global vs. local structure
fig, axes = plt.subplots(1, 3, figsize=(15, 5))
for ax, nn in zip(axes, [5, 30, 100]):
    u = UMAP(n_neighbors=nn, min_dist=0.3, random_state=42)
    X_u = u.fit_transform(X_pca[:, :30])
    for ct in range(3):
        mask = labels == ct
        ax.scatter(X_u[mask, 0], X_u[mask, 1], c=colors[ct], alpha=0.5, s=5)
    ax.set_title(f'n_neighbors={nn}')
    ax.set_xlabel('UMAP 1')
plt.suptitle('UMAP sensitivity to n_neighbors')
plt.tight_layout()
plt.savefig('umap_neighbors.pdf')
```

## t-SNE: Cluster-Revealing Visualization

**t-SNE** (t-distributed Stochastic Neighbor Embedding) minimizes the KL divergence between pairwise similarity distributions in high- and low-dimensional space. High-dimensional similarities are Gaussian; low-dimensional similarities use a heavy-tailed t-distribution (1 degree of freedom), which prevents the "crowding problem" that plagues Gaussian embeddings in low dimensions.

$$\text{KL}(P\|Q) = \sum_{i \neq j} p_{ij} \log \frac{p_{ij}}{q_{ij}}$$

```python
from sklearn.manifold import TSNE

# t-SNE on top 30 PCs (slower than UMAP; faster implementation: openTSNE)
tsne = TSNE(
    n_components=2,
    perplexity=30,    # "effective number of neighbors"; try 5-50
    learning_rate='auto',
    n_iter=1000,
    random_state=42,
    n_jobs=-1
)
X_tsne = tsne.fit_transform(X_pca[:, :30])

# For large datasets (n > 10,000), use openTSNE for speed
# pip install openTSNE
from openTSNE import TSNE as openTSNE
tsne_fast = openTSNE(perplexity=30, n_iter=500, random_state=42, n_jobs=-1)
X_tsne_fast = tsne_fast.fit(X_pca[:, :30])
```

## Comparing PCA, UMAP, and t-SNE

| Property | PCA | UMAP | t-SNE |
|----------|-----|------|-------|
| Linear? | Yes | No | No |
| Distances meaningful? | Yes | No | No |
| Global structure | Yes | Partially | No |
| Local structure | Partially | Yes | Yes |
| New data projection | Yes | Yes (transform) | No (refit) |
| Speed (n=10,000) | Fast | Fast | Slow |
| Interpretable axes? | Yes (loadings) | No | No |

**Best practice workflow:**
```python
# 1. Preprocessing + PCA
X_log = np.log1p(X)
X_scaled = StandardScaler().fit_transform(X_log)
pca = PCA(n_components=50, random_state=42)
X_pca = pca.fit_transform(X_scaled)

# 2. Choose dimensionality by elbow / 90% variance
n_pcs = np.argmax(pca.explained_variance_ratio_.cumsum() >= 0.90) + 1
print(f"PCs explaining 90% variance: {n_pcs}")

# 3. UMAP on top n_pcs
X_umap = UMAP(n_neighbors=30, min_dist=0.3, random_state=42).fit_transform(X_pca[:, :n_pcs])

# 4. Cluster in PCA space (NOT UMAP space — UMAP distances are meaningless)
from sklearn.cluster import KMeans
clusters = KMeans(n_clusters=3, random_state=42).fit_predict(X_pca[:, :n_pcs])

# 5. Visualize clusters on UMAP
# (cluster labels from PCA space, coordinates from UMAP space)
```

## Why This Matters

Every scRNA-seq paper published in the past decade includes a UMAP or t-SNE plot — it is the standard way to communicate single-cell data. Understanding what these plots do and do not show is essential for critical reading of the literature. PCA loadings identify the genes that drive biological variation between samples. UMAP and t-SNE reveal cluster structure that is invisible in the high-dimensional space. But distances and cluster sizes in UMAP are artifacts — confusing these is one of the most common errors in single-cell biology.
