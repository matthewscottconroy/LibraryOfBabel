# Clustering in Biological Data Analysis

One of the most dramatic moments in modern cell biology came not from a new experiment, but from a new way of looking at existing data. When researchers applied clustering algorithms to single-cell RNA sequencing profiles from a tumor, they discovered that what appeared under the microscope to be a uniform mass of cancer cells was in fact a community of functionally distinct populations: some proliferating rapidly, some evading the immune system, some transitioning toward a more invasive phenotype. The cells had not changed; the analysis had simply found the structure that was always there, hidden in tens of thousands of dimensions.

**Clustering** is unsupervised learning: grouping samples into clusters without predefined labels. In biology, clustering is used everywhere — identifying cell types in scRNA-seq data, grouping patients by expression subtypes, discovering co-regulated gene modules, and detecting metabolic states in time-series data. The choice of clustering algorithm, distance metric, and hyperparameters profoundly affects results, and biological validation is always required to judge whether clusters are meaningful.

## k-means Clustering

**k-means** partitions $n$ samples into $k$ clusters by minimizing the **within-cluster sum of squares (WCSS)**:

$$\text{WCSS} = \sum_{c=1}^{k} \sum_{\mathbf{x} \in C_c} \|\mathbf{x} - \boldsymbol{\mu}_c\|^2$$

where $\boldsymbol{\mu}_c$ is the centroid of cluster $c$. The standard **Lloyd's algorithm** alternates between assigning each point to its nearest centroid and updating centroids until convergence.

**k-means++ initialization** (default in sklearn) selects the first centroid randomly, then chooses subsequent centroids with probability proportional to squared distance from existing centroids. This dramatically reduces the chance of poor local minima.

```python
from sklearn.cluster import KMeans, DBSCAN, AgglomerativeClustering
from sklearn.metrics import silhouette_score, adjusted_rand_score
from sklearn.preprocessing import StandardScaler
import numpy as np
import matplotlib.pyplot as plt

# Simulated scRNA-seq data: 500 cells, 50 PCs (already dimensionality-reduced)
rng = np.random.default_rng(42)
n_cells = 500
n_components = 50

# Three cell type clusters
centers = rng.standard_normal((3, n_components))
labels_true = rng.integers(0, 3, n_cells)
X = centers[labels_true] + 0.5 * rng.standard_normal((n_cells, n_components))

# Step 1: Choose k using elbow plot and silhouette score
k_range = range(2, 10)
inertias = []
silhouettes = []

for k in k_range:
    km = KMeans(n_clusters=k, n_init=10, random_state=42)
    km.fit(X)
    inertias.append(km.inertia_)
    silhouettes.append(silhouette_score(X, km.labels_))

fig, axes = plt.subplots(1, 2, figsize=(10, 4))
axes[0].plot(list(k_range), inertias, 'bo-')
axes[0].set_xlabel('Number of clusters k')
axes[0].set_ylabel('WCSS (inertia)')
axes[0].set_title('Elbow plot')

axes[1].plot(list(k_range), silhouettes, 'ro-')
axes[1].set_xlabel('Number of clusters k')
axes[1].set_ylabel('Silhouette score')
axes[1].set_title('Silhouette score (higher=better)')

plt.tight_layout()
plt.savefig('kmeans_selection.pdf')

# Best k according to silhouette
best_k = list(k_range)[np.argmax(silhouettes)]
print(f"Best k by silhouette: {best_k}")

# Fit final model
km_final = KMeans(n_clusters=best_k, n_init=20, random_state=42)
km_final.fit(X)
print(f"Adjusted Rand Index (vs true labels): {adjusted_rand_score(labels_true, km_final.labels_):.3f}")
```

## Hierarchical Clustering

**Agglomerative hierarchical clustering** builds a dendrogram by repeatedly merging the two closest clusters, starting from $n$ singleton clusters. The **linkage criterion** defines "closest":

| Linkage | Distance measure | Properties |
|---------|-----------------|------------|
| Complete | Max pairwise distance between clusters | Compact, spherical clusters |
| Average | Mean pairwise distance | Good general-purpose |
| Ward | Increase in WCSS after merge | Minimizes variance; best for expression data |
| Single | Min pairwise distance | Prone to chaining |

**Ward's method** is almost always preferred for gene expression and proteomics clustering because it minimizes the total within-cluster variance — the same objective as k-means, but hierarchically.

```python
from scipy.cluster.hierarchy import dendrogram, linkage, fcluster
from scipy.spatial.distance import pdist, squareform
import seaborn as sns

# Hierarchical clustering with visualization
# Compute pairwise distance matrix
dist_matrix = pdist(X, metric='euclidean')

# Build dendrogram with Ward linkage
Z = linkage(dist_matrix, method='ward')

# Cut dendrogram at specific number of clusters
hier_labels = fcluster(Z, t=best_k, criterion='maxclust')
print(f"Hierarchical ARI: {adjusted_rand_score(labels_true, hier_labels):.3f}")

# Beautiful heatmap + dendrogram for gene expression
# (requires genes × samples matrix)
n_genes, n_samples = 100, 50
expression = rng.lognormal(0, 1, (n_genes, n_samples))
gene_names = [f"Gene_{i:03d}" for i in range(n_genes)]
sample_names = [f"S{i:03d}" for i in range(n_samples)]

import pandas as pd
expr_df = pd.DataFrame(expression, index=gene_names, columns=sample_names)

# seaborn clustermap: hierarchically clusters both rows and columns
g = sns.clustermap(
    np.log2(expr_df + 1),
    method='ward',
    metric='euclidean',
    cmap='RdBu_r',
    center=0,
    figsize=(12, 10),
    dendrogram_ratio=0.15,
    cbar_kws={"shrink": 0.5}
)
g.ax_heatmap.set_xlabel('Samples')
g.ax_heatmap.set_ylabel('Genes')
plt.savefig('clustermap.pdf', dpi=150)
```

## DBSCAN: Density-Based Clustering

**DBSCAN** (Density-Based Spatial Clustering of Applications with Noise) identifies clusters as dense regions separated by sparse regions. It requires no specification of $k$, handles clusters of arbitrary shape, and explicitly labels outliers as noise.

Two parameters:
- **eps** ($\varepsilon$): neighborhood radius — a point is a "core point" if at least `min_samples` points lie within distance $\varepsilon$
- **min_samples**: minimum cluster size; also controls sensitivity to noise

```python
from sklearn.cluster import DBSCAN
from sklearn.neighbors import NearestNeighbors

# Choose eps using k-NN distance plot
# Rule: use 4th nearest neighbor distance; eps at the "knee"
nbrs = NearestNeighbors(n_neighbors=5)
nbrs.fit(X)
distances, _ = nbrs.kneighbors(X)
knn_distances = np.sort(distances[:, 4])  # 4th nearest neighbor

fig, ax = plt.subplots(figsize=(7, 4))
ax.plot(knn_distances)
ax.set_xlabel('Points (sorted)')
ax.set_ylabel('4th nearest neighbor distance')
ax.set_title('k-NN distance plot (choose eps at knee)')
plt.savefig('dbscan_eps.pdf')

# Apply DBSCAN
dbscan = DBSCAN(eps=1.5, min_samples=10)
dbscan_labels = dbscan.fit_predict(X)

n_clusters = len(set(dbscan_labels)) - (1 if -1 in dbscan_labels else 0)
n_noise = (dbscan_labels == -1).sum()
print(f"DBSCAN: {n_clusters} clusters, {n_noise} noise points ({n_noise/len(X):.1%})")
```

## Gaussian Mixture Models: Probabilistic Clustering

**Gaussian Mixture Models (GMMs)** provide a probabilistic alternative to k-means, modeling each cluster as a multivariate Gaussian. Unlike k-means, GMMs:
- Give soft cluster assignments (posterior probabilities)
- Model elliptical clusters via the covariance matrix
- Select $k$ using the Bayesian Information Criterion (BIC)

```python
from sklearn.mixture import GaussianMixture

# Select number of components using BIC
bics = []
for k in range(2, 10):
    gmm = GaussianMixture(n_components=k, covariance_type='full',
                          random_state=42, n_init=5)
    gmm.fit(X)
    bics.append(gmm.bic(X))

best_k_gmm = list(range(2, 10))[np.argmin(bics)]
print(f"GMM best k by BIC: {best_k_gmm}")

gmm_final = GaussianMixture(n_components=best_k_gmm, covariance_type='full',
                             random_state=42)
gmm_final.fit(X)
proba = gmm_final.predict_proba(X)  # soft assignments: (n_cells, k)
hard_labels = gmm_final.predict(X)
print(f"GMM ARI: {adjusted_rand_score(labels_true, hard_labels):.3f}")
```

## Biological Validation: Beyond Cluster Purity

No clustering result should ever be accepted on statistical grounds alone. Clustering results must always be validated biologically:
- **Marker genes**: do cells in cluster 1 consistently express known cell-type markers?
- **GO enrichment**: do genes in a co-expression module share GO terms?
- **Survival association**: do patient clusters differ in survival (log-rank test)?
- **Experimental validation**: FACS-sort cells from each cluster and confirm phenotype

```python
from scipy.stats import chi2_contingency, kruskal

# Test whether cluster membership is associated with a phenotype
phenotype = rng.integers(0, 2, n_cells)  # binary phenotype
contingency = np.zeros((best_k, 2), dtype=int)
for cell, (cluster, pheno) in enumerate(zip(km_final.labels_, phenotype)):
    contingency[cluster, pheno] += 1

chi2, p_val, dof, expected = chi2_contingency(contingency)
print(f"Cluster-phenotype association (chi²): χ²={chi2:.2f}, p={p_val:.4f}")
```

## Why This Matters

Cell type discovery from scRNA-seq depends entirely on clustering: the Leiden or Louvain algorithm (a graph-based variant) applied to a k-NN graph of cells in PCA space defines cell clusters in every standard scRNA-seq workflow. Understanding the mathematical foundations of k-means, hierarchical clustering, and DBSCAN — and their limitations — is essential for interpreting these results critically, choosing appropriate parameters, and validating that identified "cell types" are biologically meaningful rather than numerical artifacts.
