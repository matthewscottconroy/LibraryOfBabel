# Building Co-expression Networks from RNA-seq Data

You have 500 RNA-seq samples from a cohort of cancer patients. You have 20,000 genes measured in each. What do you do with it? One approach is differential expression: compare two groups, find genes that go up or down. That is useful, but it throws away most of the information. What you have, if you look at it as a matrix, is a description of how genes move together across patients — which genes rise and fall in concert, which are independent. Two genes that are always co-expressed across hundreds of biologically distinct samples are almost certainly co-regulated, participating in the same process, or responding to the same upstream signals. Co-expression networks make this structure explicit: an edge between two genes means their expression is correlated; a dense cluster of co-expressed genes is a functional module. The method is entirely data-driven — no prior knowledge of pathways or regulators required. And the results, when analyzed carefully, can reveal which cellular programs vary most across patients, which are linked to survival, and which are candidate therapeutic targets.

A **co-expression network** connects genes that exhibit similar expression patterns across a set of samples. The underlying assumption is that genes with correlated expression are likely co-regulated, functionally related, or participating in the same biological process. Co-expression networks are one of the most widely used network types in systems biology, reconstructable from public RNA-seq datasets without any experimental perturbation.

## From Expression Matrix to Network

The starting point is a gene expression matrix $X \in \mathbb{R}^{n \times p}$ where $n$ is the number of samples (tissues, conditions, time points) and $p$ is the number of genes. The network is derived by computing a measure of co-variation between each pair of genes.

## Data Preprocessing

```python
import numpy as np
import pandas as pd
from scipy.stats import pearsonr, spearmanr
import matplotlib.pyplot as plt
import networkx as nx

def load_and_preprocess_rna_seq(count_matrix_file, metadata_file,
                                 min_cpm=1, min_samples=0.5):
    """
    Load raw count matrix and preprocess for co-expression analysis.
    
    min_cpm: minimum counts-per-million threshold for gene inclusion
    min_samples: fraction of samples that must meet CPM threshold
    """
    counts = pd.read_csv(count_matrix_file, index_col=0)
    metadata = pd.read_csv(metadata_file, index_col=0)

    print(f"Raw matrix: {counts.shape[0]} genes × {counts.shape[1]} samples")

    # 1. CPM normalization + low-expression filtering
    cpm = counts / counts.sum(axis=0) * 1e6

    # Keep genes expressed in >= min_samples fraction of samples
    expressed_mask = (cpm >= min_cpm).mean(axis=1) >= min_samples
    cpm_filtered = cpm[expressed_mask]
    print(f"After filtering (CPM ≥ {min_cpm} in ≥ {min_samples:.0%} samples): "
          f"{cpm_filtered.shape[0]} genes")

    # 2. Log2(CPM+1) transformation
    log_cpm = np.log2(cpm_filtered + 1)

    # 3. Quantile normalization (optional but recommended)
    from sklearn.preprocessing import quantile_transform
    log_cpm_norm = pd.DataFrame(
        quantile_transform(log_cpm.T, output_distribution="normal").T,
        index=log_cpm.index,
        columns=log_cpm.columns
    )

    return log_cpm_norm, metadata

# For demonstration: generate synthetic expression data
np.random.seed(42)
n_genes, n_samples = 1000, 50
expression = pd.DataFrame(
    np.random.randn(n_genes, n_samples) + np.random.randn(n_genes, 1),
    index=[f"GENE{i}" for i in range(n_genes)],
    columns=[f"Sample{i}" for i in range(n_samples)]
)
# Add some co-expression structure (gene modules)
for module_start in range(0, 1000, 100):
    module_size = 10
    module_signal = np.random.randn(n_samples)
    for gene_idx in range(module_start, module_start + module_size):
        expression.iloc[gene_idx] += 2 * module_signal

print(f"Expression matrix: {expression.shape}")
```

## Pearson Correlation Co-expression Network

```python
def pearson_coexpression_network(expression, threshold=0.7, min_genes=None):
    """
    Build co-expression network using Pearson correlation.
    threshold: minimum |r| for edge inclusion
    Returns: NetworkX graph with correlation edge weights
    """
    # Compute all pairwise Pearson correlations
    corr_matrix = expression.T.corr(method="pearson")

    print(f"Correlation matrix computed: {corr_matrix.shape}")
    print(f"Mean |r|: {np.abs(corr_matrix.values[np.triu_indices_from(corr_matrix.values, k=1)]).mean():.3f}")

    # Build network from thresholded correlations
    G = nx.Graph()
    genes = expression.index.tolist()

    for i, gene_i in enumerate(genes):
        for j, gene_j in enumerate(genes[i+1:], i+1):
            r = corr_matrix.iloc[i, j]
            if abs(r) >= threshold:
                G.add_edge(gene_i, gene_j, weight=r, abs_weight=abs(r))

    print(f"\nCo-expression network (|r| ≥ {threshold}):")
    print(f"  Genes: {G.number_of_nodes()}")
    print(f"  Edges: {G.number_of_edges()}")
    print(f"  Density: {nx.density(G):.4f}")

    return G, corr_matrix

G, corr_mat = pearson_coexpression_network(expression, threshold=0.7)
```

## WGCNA: Weighted Gene Co-expression Network Analysis

**WGCNA** (Langfelder & Horvath, 2008) is the gold standard method for co-expression network construction. It addresses a key limitation of thresholded Pearson networks: the binary threshold discards quantitative information about correlation strength and is sensitive to threshold choice.

WGCNA uses **soft thresholding**: instead of a binary cutoff, it applies a power transformation to the correlation matrix:

$$a_{ij} = |r_{ij}|^\beta$$

where the **soft threshold power** $\beta$ is chosen to make the resulting network approximately scale-free. This power transformation continuously down-weights weak correlations while preserving strong ones.

```python
def wgcna_soft_threshold_selection(expression, powers=None):
    """
    Select the soft thresholding power β for WGCNA.
    Criterion: the resulting network should have scale-free topology (R² > 0.85).
    """
    if powers is None:
        powers = list(range(1, 11)) + list(range(12, 22, 2))

    corr_matrix = expression.T.corr(method="pearson").values
    n_genes = len(expression)
    results = []

    for beta in powers:
        # Adjacency matrix with soft thresholding
        adj = np.abs(corr_matrix) ** beta
        np.fill_diagonal(adj, 0)

        # Connectivity (weighted degree)
        k = adj.sum(axis=1)

        # Scale-free fit: regress log(P(k)) ~ log(k)
        # Bin k into equal-width bins
        bins = np.percentile(k, np.linspace(5, 95, 20))
        k_binned = pd.cut(k, bins=bins, labels=False)
        k_means = [k[k_binned == i].mean() for i in range(len(bins)-1)
                   if (k_binned == i).sum() > 2]
        p_k = [(k_binned == i).sum() / n_genes for i in range(len(bins)-1)
               if (k_binned == i).sum() > 2]

        # Log-log regression
        valid = [(km, pk) for km, pk in zip(k_means, p_k)
                 if km > 0 and pk > 0]
        if len(valid) > 3:
            log_k = np.log([v[0] for v in valid])
            log_p = np.log([v[1] for v in valid])
            coeffs = np.polyfit(log_k, log_p, 1)
            r2 = np.corrcoef(log_k, log_p)[0, 1] ** 2
        else:
            r2 = 0

        results.append({
            "beta": beta,
            "mean_connectivity": k.mean(),
            "R2": r2
        })

    df = pd.DataFrame(results)
    print("WGCNA soft threshold selection:")
    print(df.to_string(index=False, float_format=lambda x: f"{x:.3f}"))

    # Select: smallest β with R² > 0.85
    scale_free = df[df["R2"] > 0.85]
    optimal_beta = scale_free["beta"].min() if len(scale_free) > 0 else 6
    print(f"\nSelected β = {optimal_beta} (R² > 0.85, minimum connectivity)")
    return optimal_beta

optimal_beta = wgcna_soft_threshold_selection(expression)
```

## Module Detection with Hierarchical Clustering

After computing the WGCNA adjacency matrix, the **Topological Overlap Measure (TOM)** quantifies shared network neighborhood:

$$\text{TOM}_{ij} = \frac{\sum_k a_{ik} a_{kj} + a_{ij}}{k_i + k_j - a_{ij} + 1}$$

TOM-based dissimilarity ($1 - \text{TOM}_{ij}$) is used for hierarchical clustering to identify gene modules:

```python
from scipy.cluster.hierarchy import dendrogram, linkage, fcluster
from scipy.spatial.distance import squareform

def compute_tom(adj):
    """Compute Topological Overlap Measure matrix from adjacency matrix."""
    n = len(adj)
    numerator = adj @ adj + adj         # sum_k a_ik * a_kj + a_ij
    k = adj.sum(axis=1)                 # connectivity
    denominator = np.add.outer(k, k) - adj + 1  # k_i + k_j - a_ij + 1
    tom = numerator / (denominator + 1e-10)
    np.fill_diagonal(tom, 1)
    return tom

def identify_wgcna_modules(expression, beta, min_module_size=30,
                            deep_split=2):
    """
    Full WGCNA module detection pipeline.
    Returns: module assignments for each gene
    """
    # Adjacency matrix
    corr = expression.T.corr("pearson").values
    adj = np.abs(corr) ** beta
    np.fill_diagonal(adj, 0)

    # TOM-based dissimilarity
    tom = compute_tom(adj)
    dissimilarity = 1 - tom

    # Hierarchical clustering (average linkage)
    condensed = squareform(dissimilarity)
    Z = linkage(condensed, method="average")

    # Cut tree: minimum module size = min_module_size
    n_clusters = max(2, len(expression) // min_module_size)
    labels = fcluster(Z, t=n_clusters, criterion="maxclust")

    # Map to module colors (WGCNA convention)
    from collections import Counter
    label_counts = Counter(labels)

    # Sort modules by size, assign colors
    colors = ["turquoise", "blue", "brown", "yellow", "green",
              "red", "black", "pink", "magenta", "purple"]
    module_sizes = sorted(label_counts.items(), key=lambda x: -x[1])
    label_to_color = {label: colors[min(i, len(colors)-1)]
                      for i, (label, _) in enumerate(module_sizes)}

    module_assignments = {gene: label_to_color[labels[i]]
                           for i, gene in enumerate(expression.index)}

    print(f"WGCNA module detection (β = {beta}):")
    for i, (label, count) in enumerate(module_sizes[:10]):
        color = label_to_color[label]
        print(f"  Module {color}: {count} genes")

    return module_assignments, Z

modules, linkage_Z = identify_wgcna_modules(expression, optimal_beta)
```

## Module Eigengene and Trait Correlation

The **module eigengene** is the first principal component of the expression matrix within a module — a single vector summarizing the overall expression pattern of the module:

```python
from sklearn.decomposition import PCA

def compute_module_eigengenes(expression, module_assignments):
    """
    Compute module eigengenes (first PC of each module).
    Returns: DataFrame (n_samples × n_modules) of eigengene values
    """
    # Group genes by module
    modules = {}
    for gene, module in module_assignments.items():
        modules.setdefault(module, []).append(gene)

    eigengenes = {}
    for module_color, genes in modules.items():
        if len(genes) < 3:
            continue
        module_expr = expression.loc[genes].T  # (n_samples, n_genes)
        pca = PCA(n_components=1)
        ME = pca.fit_transform(module_expr).flatten()
        # Align sign: positive correlation with average module expression
        avg_expr = module_expr.mean(axis=1)
        if np.corrcoef(ME, avg_expr)[0,1] < 0:
            ME = -ME
        eigengenes[f"ME_{module_color}"] = ME

    ME_df = pd.DataFrame(eigengenes, index=expression.columns)
    print(f"Module eigengenes computed: {ME_df.shape}")
    return ME_df

ME_df = compute_module_eigengenes(expression, modules)

def correlate_eigengenes_with_traits(ME_df, trait_df):
    """
    Correlate module eigengenes with continuous phenotypic traits.
    Returns: correlation and p-value matrices.
    """
    from scipy.stats import pearsonr

    module_names = ME_df.columns.tolist()
    trait_names = trait_df.columns.tolist()

    corr_matrix = pd.DataFrame(index=module_names, columns=trait_names, dtype=float)
    pval_matrix = pd.DataFrame(index=module_names, columns=trait_names, dtype=float)

    for module in module_names:
        for trait in trait_names:
            common_idx = ME_df.index.intersection(trait_df.index)
            r, p = pearsonr(ME_df.loc[common_idx, module],
                            trait_df.loc[common_idx, trait])
            corr_matrix.loc[module, trait] = r
            pval_matrix.loc[module, trait] = p

    print("Module-trait correlation (top associations):")
    for module in module_names:
        for trait in trait_names:
            r = corr_matrix.loc[module, trait]
            p = pval_matrix.loc[module, trait]
            if abs(r) > 0.5 and p < 0.05:
                print(f"  {module} ~ {trait}: r = {r:.2f}, p = {p:.3f}")
    return corr_matrix, pval_matrix
```

## Why This Matters

Co-expression networks, particularly WGCNA modules, have become one of the most powerful tools for extracting biological signal from large transcriptomic datasets. A WGCNA analysis of 100 tumor RNA-seq samples can identify 20–30 gene modules, each enriched for specific biological processes (immune infiltration, proliferation, DNA repair), and correlate each module's activity with patient survival, drug response, or mutation burden. The module eigengene is a data-driven biomarker — more robust and biologically meaningful than any single gene expression value. In synthetic biology, co-expression analysis of time series data during a synthetic circuit induction reveals which native cellular processes are co-opted or perturbed by the circuit, guiding iterative design improvements.
