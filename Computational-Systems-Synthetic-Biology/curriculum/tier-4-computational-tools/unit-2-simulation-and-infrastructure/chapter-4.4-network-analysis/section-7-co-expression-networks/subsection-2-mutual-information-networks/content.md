# Mutual Information Networks and GRN Inference

Pearson correlation is a measure of linear dependence: if gene A's expression is a straight-line function of gene B's, they will have a high correlation coefficient. But many gene regulatory interactions are not linear. A transcription factor that activates a gene only above a threshold concentration — a switch-like induction — will produce a nonlinear, sigmoidal relationship between the TF's expression and its target's expression. A repressor that saturates at low concentrations will produce a hyperbolic relationship. In both cases, Pearson correlation will underestimate the true statistical dependence between the two genes, potentially missing the regulatory relationship entirely. Mutual information has no such limitation: it detects any statistical dependency, linear or not, as long as there is information shared between two variables. This makes it a more principled choice for inferring gene regulatory interactions, where the biology of binding and saturation guarantees nonlinearity.

Pearson correlation captures only linear relationships between gene expression levels. Many gene regulatory interactions are inherently nonlinear — a transcription factor that activates a gene only above a threshold concentration, or a repressor that saturates at low concentrations. **Mutual information (MI)**-based networks can detect both linear and nonlinear dependencies, making them better suited for regulatory network inference from expression data.

## Mutual Information

For two random variables $X$ (expression of gene $i$) and $Y$ (expression of gene $j$), mutual information quantifies how much knowing $X$ reduces uncertainty about $Y$:

$$I(X; Y) = \sum_{x,y} p(x,y) \log \frac{p(x,y)}{p(x)p(y)}$$

For continuous variables:

$$I(X; Y) = \int\!\!\int p(x,y) \log \frac{p(x,y)}{p(x)p(y)} \, dx \, dy$$

**Key properties**:
- $I(X;Y) \geq 0$; equality iff $X$ and $Y$ are independent
- $I(X;Y)$ is symmetric
- $I(X;Y)$ captures any statistical dependence, not just linear
- For Gaussian variables: $I(X;Y) = -\frac{1}{2}\log(1 - r^2)$ where $r$ is the Pearson correlation

## Estimating Mutual Information from Data

Estimating MI from finite samples is challenging — naive bin-based estimators are biased for small sample sizes. Two robust approaches:

```python
import numpy as np
from sklearn.neighbors import KernelDensity
from scipy.stats import entropy

def knn_mutual_information(x, y, k=5):
    """
    k-nearest-neighbor estimator for mutual information (Kraskov et al., 2004).
    Robust for small to medium sample sizes.
    """
    from sklearn.neighbors import NearestNeighbors

    n = len(x)
    X = np.column_stack([x, y])

    # k-NN in joint space
    knn_joint = NearestNeighbors(n_neighbors=k+1, metric="chebyshev")
    knn_joint.fit(X)
    dist_joint, _ = knn_joint.kneighbors(X)
    eps = dist_joint[:, -1]  # distance to k-th neighbor

    # Count points in marginal spaces within eps
    knn_x = NearestNeighbors(metric="chebyshev")
    knn_x.fit(x.reshape(-1, 1))
    nx = np.array([len(knn_x.radius_neighbors([[xi]], radius=e,
                                               return_distance=False)[0])
                   for xi, e in zip(x, eps)]) - 1

    knn_y = NearestNeighbors(metric="chebyshev")
    knn_y.fit(y.reshape(-1, 1))
    ny = np.array([len(knn_y.radius_neighbors([[yi]], radius=e,
                                               return_distance=False)[0])
                   for yi, e in zip(y, eps)]) - 1

    from scipy.special import digamma
    mi = (digamma(k) + digamma(n)
          - np.mean(digamma(nx + 1))
          - np.mean(digamma(ny + 1)))
    return max(0, mi)

def bin_mi_matrix(expression, n_bins=10):
    """
    Fast MI estimation using equal-frequency binning.
    Less accurate than KNN but practical for large gene sets.
    """
    n_genes, n_samples = expression.shape
    mi_matrix = np.zeros((n_genes, n_genes))

    # Discretize each gene into equal-frequency bins
    from pandas import qcut
    discretized = np.zeros_like(expression, dtype=int)
    for i in range(n_genes):
        # Equal-frequency binning (quantile-based)
        ranks = np.argsort(np.argsort(expression[i]))
        discretized[i] = (ranks * n_bins / n_samples).astype(int)

    # Compute MI from joint histogram
    for i in range(n_genes):
        for j in range(i+1, n_genes):
            # Joint histogram
            joint = np.zeros((n_bins, n_bins))
            for s in range(n_samples):
                joint[discretized[i,s], discretized[j,s]] += 1
            joint /= n_samples

            px = joint.sum(axis=1)
            py = joint.sum(axis=0)
            mi = 0
            for bi in range(n_bins):
                for bj in range(n_bins):
                    if joint[bi,bj] > 0:
                        mi += joint[bi,bj] * np.log(joint[bi,bj] / (px[bi] * py[bj] + 1e-30))
            mi_matrix[i,j] = mi_matrix[j,i] = max(0, mi)

    return mi_matrix
```

## ARACNE: Network Inference via Data Processing Inequality

**ARACNE** (Algorithm for the Reconstruction of Accurate Cellular Networks, Margolin et al., 2006) builds a gene regulatory network by computing pairwise MI between all genes and then applying the **Data Processing Inequality (DPI)** to remove indirect interactions.

**DPI theorem**: for any three variables $X$, $Y$, $Z$ forming a regulatory cascade ($X \to Z \to Y$), the indirect connection satisfies:

$$I(X; Y) \leq \min(I(X; Z), I(Z; Y))$$

In a regulatory cascade, the weakest link limits information transmission. ARACNE removes the edge with the smallest MI in every 3-gene triplet, leaving only direct regulatory connections.

```python
def aracne(mi_matrix, dpi_tolerance=0.1):
    """
    ARACNE algorithm: apply DPI to remove indirect regulatory edges.
    dpi_tolerance: ε in DPI pruning — edges with MI < min_MI * (1 + ε) are removed.
    Returns: pruned adjacency matrix
    """
    n = len(mi_matrix)
    adj = mi_matrix.copy()

    removed = 0
    for a in range(n):
        for b in range(a+1, n):
            if adj[a, b] == 0:
                continue
            for c in range(n):
                if c == a or c == b:
                    continue
                # Triangle: a-b, b-c, a-c
                mi_ab = adj[a, b]
                mi_bc = adj[b, c]
                mi_ac = adj[a, c]

                # DPI: remove weakest edge in triangle
                min_mi = min(mi_ab, mi_bc, mi_ac)
                max_mi = max(mi_ab, mi_bc, mi_ac)

                if min_mi < max_mi * (1 - dpi_tolerance):  # clear weakest
                    if mi_ab == min_mi:
                        adj[a, b] = adj[b, a] = 0
                        removed += 1
                        break  # edge removed; move to next pair
    print(f"ARACNE DPI pruning: removed {removed} indirect edges")
    return adj

# Practical usage: use the Python ARACNE implementation
def build_aracne_network(expression_df, mi_threshold=0.2, dpi_tolerance=0.1):
    """
    Full ARACNE pipeline for GRN inference from expression data.
    expression_df: genes × samples DataFrame
    """
    print(f"Computing MI matrix for {len(expression_df)} genes × "
          f"{expression_df.shape[1]} samples...")

    # Use fast bin-based MI for exploration
    mi_mat = bin_mi_matrix(expression_df.values, n_bins=10)

    # Threshold low MI values (noise reduction)
    mi_mat[mi_mat < mi_threshold] = 0
    n_before = (mi_mat > 0).sum() // 2
    print(f"Edges before DPI (MI > {mi_threshold}): {n_before}")

    # Apply DPI
    mi_pruned = aracne(mi_mat, dpi_tolerance=dpi_tolerance)
    n_after = (mi_pruned > 0).sum() // 2
    print(f"Edges after DPI: {n_after} ({n_after/n_before*100:.0f}% retained)")

    # Build NetworkX graph
    genes = expression_df.index.tolist()
    G = nx.Graph()
    for i in range(len(genes)):
        for j in range(i+1, len(genes)):
            if mi_pruned[i, j] > 0:
                G.add_edge(genes[i], genes[j], weight=mi_pruned[i, j])

    return G, mi_pruned
```

## CLR: Context Likelihood of Relatedness

**CLR** (Faith et al., 2007) normalizes MI values by the empirical distribution of MI scores for each gene against all other genes, reducing the effect of highly connected "hub" genes:

$$z_{ij} = \sqrt{z_i^2 + z_j^2}$$

where $z_i$ is the $z$-score of $I(X_i; X_j)$ relative to the null distribution of $\{I(X_i; X_k) : k \neq i, j\}$.

```python
def clr_normalize(mi_matrix):
    """
    Context Likelihood of Relatedness (CLR) normalization of MI matrix.
    Reduces hub bias by comparing each MI to the background distribution.
    """
    n = mi_matrix.shape[0]
    z_matrix = np.zeros_like(mi_matrix)

    for i in range(n):
        # MI values of gene i against all others (excluding self)
        mi_row = np.concatenate([mi_matrix[i, :i], mi_matrix[i, i+1:]])
        mu_i = mi_row.mean()
        sigma_i = mi_row.std()

        for j in range(n):
            if i != j:
                zi = (mi_matrix[i, j] - mu_i) / (sigma_i + 1e-10)
                z_matrix[i, j] = max(0, zi)  # clip negative z-scores

    # Symmetric CLR score
    clr_matrix = np.sqrt(z_matrix ** 2 + z_matrix.T ** 2)
    return clr_matrix

print("CLR normalization reduces hub bias:")
print("  Genes with many interactions have higher background MI")
print("  CLR corrects for this by z-scoring each gene's MI distribution")
```

## Benchmarking GRN Inference

```python
def evaluate_grn_inference(predicted_network, true_network):
    """
    Evaluate predicted GRN against known gold standard.
    Computes AUROC and AUPR from edge score rankings.
    """
    from sklearn.metrics import roc_auc_score, average_precision_score

    # Get all possible edges
    all_nodes = set(predicted_network.nodes()) | set(true_network.nodes())
    all_edges = [(i, j) for i in all_nodes for j in all_nodes if i < j]

    y_true = np.array([
        1 if true_network.has_edge(i, j) else 0
        for i, j in all_edges
    ])
    y_score = np.array([
        predicted_network[i][j]["weight"] if predicted_network.has_edge(i, j) else 0
        for i, j in all_edges
    ])

    auroc = roc_auc_score(y_true, y_score)
    aupr  = average_precision_score(y_true, y_score)
    prevalence = y_true.mean()

    print(f"GRN inference evaluation:")
    print(f"  AUROC: {auroc:.3f} (random = 0.5)")
    print(f"  AUPR:  {aupr:.3f} (random = {prevalence:.3f})")
    print(f"  Prevalence (% true edges): {prevalence*100:.2f}%")
    return auroc, aupr
```

## Why This Matters

MI-based network inference is one of the few approaches that can reconstruct gene regulatory network structure directly from expression data, without prior knowledge of regulators or binding sites. The DPI-pruning in ARACNE is a principled way to distinguish direct regulatory relationships from transitive correlations — the fundamental distinction between correlation and causation in network biology. Methods like ARACNE and GENIE3 (random forest-based GRN inference) have been extensively benchmarked in the DREAM challenges, demonstrating that they recover a significant fraction of experimentally verified regulatory interactions from expression data alone. In the context of drug discovery, MI networks identify master regulators of disease gene modules — transcription factors whose expression drives an entire co-expressed module — which are candidate therapeutic targets even when the individual module genes are not directly druggable.
