# Evaluating Community Detection Results

A student once showed me a network analysis where Louvain community detection had partitioned a yeast PPI network into 47 communities, each with a modularity Q of 0.62. "Strong community structure," they concluded. When I asked what the communities corresponded to biologically, the answer was: nothing. The communities did not match known pathways, did not enrich for any GO terms, and changed completely between algorithm runs. High Q is not the same as biological meaning, and this distinction — between topological quality and biological validity — is the central theme of this section.

Detecting communities is only half the challenge; the other half is evaluating whether the detected communities are meaningful. Evaluation operates at two levels: internal validation (do the communities have good topological properties?) and external validation (do the communities correspond to known biology?). Both are necessary — a topologically excellent partition that has no biological coherence is scientifically worthless, and vice versa.

## Modularity Q as an Internal Metric

Modularity $Q$ is the most commonly reported internal quality metric, but it has limitations. It measures excess within-community connectivity relative to a null model, with maximum possible value $Q = 1$ (complete separation of communities) and $Q = 0$ (no community structure). In practice, $Q > 0.3$ indicates meaningful structure, $Q > 0.6$ is very strong.

**Limitations of Q:**
- Resolution limit: small communities are penalized
- Not comparable across networks of different sizes
- Maximum Q does not guarantee biologically meaningful communities
- Non-deterministic algorithms (Louvain) produce variable Q across runs

```python
import networkx as nx
import numpy as np
from networkx.algorithms.community.quality import modularity

def evaluate_partition_internal(G, communities):
    """Compute internal quality metrics for a community partition."""
    # 1. Modularity
    Q = modularity(G, communities)

    # 2. Coverage: fraction of edges within communities
    total_edges = G.number_of_edges()
    within_edges = sum(
        G.subgraph(c).number_of_edges()
        for c in communities
    )
    coverage = within_edges / total_edges if total_edges > 0 else 0

    # 3. Performance: fraction of node pairs correctly classified
    # (within community & connected, or across communities & not connected)
    n = G.number_of_nodes()
    total_pairs = n * (n - 1) // 2
    correct_pairs = within_edges  # within-community connected pairs
    # Within-community but NOT connected (should not be there)
    for c in communities:
        n_c = len(c)
        possible_within = n_c * (n_c - 1) // 2
        actual_within = G.subgraph(c).number_of_edges()
        correct_pairs += (total_pairs - (sum(len(c)*(len(c)-1)//2
                           for c in communities) - possible_within + actual_within)
                          - within_edges)

    # 4. Conductance: fraction of edges leaving each community
    conductances = []
    for c in communities:
        c_nodes = set(c)
        subg = G.subgraph(c)
        within = subg.number_of_edges()
        degree_sum = sum(G.degree(n) for n in c_nodes)
        boundary = degree_sum - 2 * within  # edges leaving community
        if degree_sum > 0:
            conductances.append(boundary / degree_sum)

    metrics = {
        "Q (modularity)": Q,
        "Coverage": coverage,
        "Mean conductance": np.mean(conductances) if conductances else 0,
        "N communities": len(communities),
        "Size range": f"{min(len(c) for c in communities)}–{max(len(c) for c in communities)}",
    }
    for k, v in metrics.items():
        if isinstance(v, float):
            print(f"  {k}: {v:.4f}")
        else:
            print(f"  {k}: {v}")
    return metrics
```

## Normalized Mutual Information (NMI): Comparing to Ground Truth

When ground truth community labels are available (e.g., known protein complexes, GO biological process annotations), **Normalized Mutual Information (NMI)** quantifies agreement between detected and true communities:

$$\text{NMI}(A, B) = \frac{2 I(A; B)}{H(A) + H(B)}$$

where $I(A;B)$ is the mutual information between partition $A$ (detected) and $B$ (ground truth), and $H(\cdot)$ is Shannon entropy. NMI = 1 means perfect agreement; NMI = 0 means no agreement beyond chance.

```python
from sklearn.metrics import normalized_mutual_info_score
import numpy as np

def compare_to_ground_truth(detected_communities, ground_truth_communities, node_list):
    """
    Compare detected communities to ground truth using NMI and ARI.
    Both partitions must cover the same set of nodes.
    """
    from sklearn.metrics import adjusted_rand_score

    n = len(node_list)
    node_to_idx = {n: i for i, n in enumerate(node_list)}

    # Convert community sets to label arrays
    detected_labels = np.zeros(n, dtype=int)
    for comm_idx, community in enumerate(detected_communities):
        for node in community:
            if node in node_to_idx:
                detected_labels[node_to_idx[node]] = comm_idx

    gt_labels = np.zeros(n, dtype=int)
    for comm_idx, community in enumerate(ground_truth_communities):
        for node in community:
            if node in node_to_idx:
                gt_labels[node_to_idx[node]] = comm_idx

    nmi = normalized_mutual_info_score(gt_labels, detected_labels)
    ari = adjusted_rand_score(gt_labels, detected_labels)

    print(f"Comparison to ground truth:")
    print(f"  NMI: {nmi:.4f}  (1 = perfect, 0 = random)")
    print(f"  ARI: {ari:.4f}  (1 = perfect, 0 = random, negative = worse than random)")
    return nmi, ari

# Example: synthetic network with known communities
sizes = [30, 30, 30, 30]
p_matrix = [[0.4 if i == j else 0.02 for j in range(4)] for i in range(4)]
G_test = nx.stochastic_block_model(sizes, p_matrix, seed=42)
node_list = list(G_test.nodes())

# Ground truth communities
gt_communities = [
    {n for n, data in G_test.nodes(data=True) if data["block"] == i}
    for i in range(4)
]

# Detected communities (Louvain)
from networkx.algorithms.community import louvain_communities
detected = louvain_communities(G_test, seed=42)

nmi, ari = compare_to_ground_truth(detected, gt_communities, node_list)
```

## Biological Validation: GO Term Enrichment

The most meaningful validation is whether detected communities correspond to known functional annotations. **GO term enrichment** tests whether the genes/proteins in a community are significantly enriched for specific Gene Ontology terms relative to the network background.

```python
from scipy.stats import hypergeom
from statsmodels.stats.multitest import multipletests
import pandas as pd

def go_enrichment_for_communities(communities, go_annotations, background=None):
    """
    Test GO term enrichment for each detected community.
    communities: list of sets of gene identifiers
    go_annotations: dict {gene_id: [GO_term_1, ...]}
    background: set of all genes (default: union of all communities)
    """
    if background is None:
        background = set().union(*communities)

    N = len(background)

    # Count GO terms in background
    go_background = {}
    for gene in background:
        for term in go_annotations.get(gene, []):
            go_background[term] = go_background.get(term, 0) + 1

    all_results = []

    for comm_idx, community in enumerate(communities):
        n = len(community)  # community size

        # Count GO terms in community
        go_comm = {}
        for gene in community:
            for term in go_annotations.get(gene, []):
                go_comm[term] = go_comm.get(term, 0) + 1

        # Test each term
        for term, k in go_comm.items():
            K = go_background.get(term, 0)
            if K >= 5:  # minimum prevalence in background
                pval = hypergeom.sf(k - 1, N, K, n)  # P(X >= k)
                all_results.append({
                    "community": comm_idx,
                    "comm_size": n,
                    "GO_term": term,
                    "k_in_comm": k,
                    "K_in_bg": K,
                    "enrichment": (k/n) / (K/N),
                    "p_value": pval
                })

    if not all_results:
        print("No enrichments found")
        return pd.DataFrame()

    df = pd.DataFrame(all_results)
    # Multiple testing correction (Benjamini-Hochberg FDR)
    _, q_values, _, _ = multipletests(df["p_value"], method="fdr_bh")
    df["q_value"] = q_values

    significant = df[df["q_value"] < 0.05].sort_values("q_value")
    print(f"Significant GO enrichments (FDR < 5%): {len(significant)}")
    print(significant[["community", "GO_term", "k_in_comm", "enrichment",
                         "q_value"]].head(15).to_string(index=False))
    return df

# Simulated GO annotations (replace with real gseapy/goatools output)
fake_go = {f"Gene{i}": [f"GO:{i % 10:07d}", f"GO:{i % 5:07d}"]
           for i in range(120)}
```

## Stability Analysis: Are Communities Robust?

Community detection is stochastic (Louvain, Leiden) and sensitive to small network perturbations. Stable communities should appear consistently across independent runs and under minor network rewiring.

```python
def community_stability(G, algorithm="louvain", n_runs=20, seed=0):
    """
    Assess stability of community detection via pairwise NMI across runs.
    High mean NMI (> 0.8) indicates stable communities.
    """
    rng = np.random.default_rng(seed)
    all_partitions = []
    node_list = list(G.nodes())

    for i in range(n_runs):
        if algorithm == "louvain":
            communities = louvain_communities(G, seed=int(rng.integers(1e9)))
        # Convert to label array
        labels = np.zeros(len(node_list), dtype=int)
        node_to_idx = {n: i for i, n in enumerate(node_list)}
        for comm_idx, community in enumerate(communities):
            for node in community:
                labels[node_to_idx[node]] = comm_idx
        all_partitions.append(labels)

    # Pairwise NMI
    nmi_values = []
    for i in range(n_runs):
        for j in range(i+1, n_runs):
            nmi = normalized_mutual_info_score(all_partitions[i], all_partitions[j])
            nmi_values.append(nmi)

    mean_nmi = np.mean(nmi_values)
    print(f"Community stability ({n_runs} runs, {algorithm}):")
    print(f"  Mean pairwise NMI: {mean_nmi:.4f}")
    print(f"  {'STABLE' if mean_nmi > 0.8 else 'UNSTABLE'} (threshold: 0.8)")
    print(f"  Std NMI: {np.std(nmi_values):.4f}")
    return mean_nmi, nmi_values

stability, nmi_dist = community_stability(G_test, n_runs=10)
```

## Why This Matters

The difference between rigorous and superficial community detection analysis is the difference between a defensible scientific result and an overfitted artifact. Reporting only modularity Q is insufficient: modularity can be high even for networks with no meaningful community structure (random graphs of certain sizes produce $Q \approx 0.4$). Biological validation via GO enrichment or known complex membership transforms a topological observation into a functional claim. Stability analysis reveals whether the algorithm is making consistent discoveries or finding different structures on each run. These validation steps are what distinguish a published computational biology paper from an exploratory analysis, and they are expected by reviewers in any network biology study.
