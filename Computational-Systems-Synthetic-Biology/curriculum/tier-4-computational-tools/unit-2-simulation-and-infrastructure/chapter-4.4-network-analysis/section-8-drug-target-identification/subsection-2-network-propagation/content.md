# Network Propagation for Gene Prioritization

Imagine placing a drop of dye into a network of connected tubes. The dye spreads through the tubes, flowing fastest through the widest and most connected passages, diffusing more slowly into side branches. After enough time, every tube has some dye, but the concentration drops with distance from the source. Network propagation does essentially this, with probability flowing instead of dye: you start at a set of known disease genes, let probability spread across the interactome following its edge structure, and then read off which nodes accumulated the most probability. Nodes close to many disease genes, through many paths, receive high scores — they are network-proximal to the disease, even if they were never individually identified in a genetic study. This simple physical intuition, formalized as a random walk, is one of the most practically impactful algorithms in network biology.

**Network propagation** (also called network diffusion or random walk with restart) is a powerful technique that spreads a signal from a set of seed nodes across the network, ranking all other nodes by their proximity to the seeds. In biological contexts, the seeds are known disease genes or drug targets, and propagation identifies additional genes that are network-close — making them candidates for novel disease associations or drug targets.

## Random Walk with Restart (RWR)

The standard propagation model is **Random Walk with Restart (RWR)**. A random walker starts at a seed node, and at each step either:
- Follows a random edge to a neighbor with probability $\alpha$
- Teleports back to the seed set with probability $1 - \alpha$

The stationary distribution of this process gives a score $p_i$ for each node $i$, reflecting how accessible that node is from the seed set through network connectivity.

The iterative update equation:

$$\mathbf{p}^{(t+1)} = \alpha \cdot W \mathbf{p}^{(t)} + (1-\alpha) \cdot \mathbf{e}$$

where:
- $\mathbf{p}^{(t)} \in \mathbb{R}^n$: current score vector
- $W = D^{-1} A$: column-normalized adjacency matrix (random walk transition matrix)
- $\mathbf{e} \in \mathbb{R}^n$: initial seed vector (uniform over seed nodes, zero elsewhere)
- $\alpha \in (0,1)$: restart probability (typically 0.7–0.85)

The stationary solution:

$$\mathbf{p}^* = (1-\alpha)(I - \alpha W)^{-1} \mathbf{e}$$

```python
import numpy as np
import networkx as nx
from scipy.sparse import csr_matrix
from scipy.sparse.linalg import spsolve

def random_walk_with_restart(G, seed_genes, alpha=0.85, max_iter=1000,
                              tol=1e-6):
    """
    Run RWR from seed genes and return stationary scores for all nodes.
    
    G: NetworkX graph (interactome)
    seed_genes: list of seed gene names
    alpha: propagation constant (1-alpha = restart probability)
    Returns: dict {node: score}
    """
    nodes = list(G.nodes())
    n = len(nodes)
    node_to_idx = {node: i for i, node in enumerate(nodes)}

    # Build column-normalized adjacency matrix W = D^(-1) A
    A = nx.to_numpy_array(G, nodelist=nodes, weight="weight")
    degree = A.sum(axis=1, keepdims=True)
    degree[degree == 0] = 1  # avoid division by zero
    W = A / degree  # row-normalized: W[i,j] = prob of walking from i to j

    # Seed vector e: uniform over seed nodes in network
    seeds_in_network = [g for g in seed_genes if g in node_to_idx]
    if not seeds_in_network:
        print("WARNING: No seed genes found in network")
        return {}

    e = np.zeros(n)
    for gene in seeds_in_network:
        e[node_to_idx[gene]] = 1.0 / len(seeds_in_network)

    # Iterative propagation
    p = e.copy()
    for iteration in range(max_iter):
        p_new = alpha * (W.T @ p) + (1 - alpha) * e
        delta = np.abs(p_new - p).max()
        p = p_new
        if delta < tol:
            print(f"Converged after {iteration+1} iterations (max|Δp| = {delta:.2e})")
            break
    else:
        print(f"Did not converge in {max_iter} iterations (max|Δp| = {delta:.2e})")

    scores = {nodes[i]: p[i] for i in range(n)}
    return scores

# Example: propagate from TP53-related disease genes in a PPI network
G_ppi = nx.barabasi_albert_graph(500, 3, seed=42)
node_labels = {i: f"GENE{i}" for i in G_ppi.nodes()}
G_ppi = nx.relabel_nodes(G_ppi, node_labels)

seed_genes = ["GENE0", "GENE1", "GENE2"]  # known disease genes
scores = random_walk_with_restart(G_ppi, seed_genes, alpha=0.85)

# Rank all genes by score (excluding seeds)
ranked = sorted(
    [(gene, score) for gene, score in scores.items() if gene not in seed_genes],
    key=lambda x: -x[1]
)

print("\nTop 10 candidate genes by RWR score:")
for gene, score in ranked[:10]:
    print(f"  {gene}: score = {score:.6f}, "
          f"degree = {G_ppi.degree(gene)}")
```

## Sparse Analytical Solution

For large networks, iterative RWR can be slow. The analytical solution uses sparse linear algebra:

```python
def rwr_analytical(G, seed_genes, alpha=0.85):
    """
    Analytical RWR solution: p* = (1-alpha) * (I - alpha*W)^{-1} * e
    More accurate than iteration; practical for networks < 50,000 nodes.
    """
    from scipy.sparse import eye, diags
    from scipy.sparse.linalg import spsolve

    nodes = list(G.nodes())
    n = len(nodes)
    node_to_idx = {n_: i for i, n_ in enumerate(nodes)}

    A = nx.to_scipy_sparse_array(G, nodelist=nodes, format="csr", weight="weight")
    # Row-normalize (transition matrix)
    degree = np.array(A.sum(axis=1)).flatten()
    degree[degree == 0] = 1
    D_inv = diags(1 / degree)
    W = D_inv @ A  # W[i,j] = P(walk from i to j)

    # Seed vector
    seeds_in_net = [g for g in seed_genes if g in node_to_idx]
    e = np.zeros(n)
    for g in seeds_in_net:
        e[node_to_idx[g]] = 1.0 / len(seeds_in_net)

    # Solve: (I - alpha * W^T) * p = (1-alpha) * e
    system_matrix = eye(n, format="csr") - alpha * W.T
    rhs = (1 - alpha) * e
    p = spsolve(system_matrix, rhs)

    return {nodes[i]: p[i] for i in range(n)}
```

## Network Propagation for Drug Repurposing

Drug repurposing uses network propagation to identify diseases that are network-close to the known targets of an existing drug:

```python
def drug_repurposing_by_propagation(interactome, drug_targets,
                                     disease_genes_dict, alpha=0.85):
    """
    Score potential drug repurposing opportunities by network proximity.
    
    drug_targets: list of known targets for the drug
    disease_genes_dict: {disease_name: [list of disease genes]}
    Returns: ranked diseases by proximity score
    """
    # Propagate from drug targets
    drug_scores = rwr_analytical(interactome, drug_targets, alpha=alpha)

    # For each disease: average score over disease genes
    disease_scores = {}
    for disease, genes in disease_genes_dict.items():
        genes_in_net = [g for g in genes if g in drug_scores]
        if genes_in_net:
            mean_score = np.mean([drug_scores[g] for g in genes_in_net])
            disease_scores[disease] = mean_score

    # Normalize by background (null model: random gene sets)
    ranked_diseases = sorted(disease_scores.items(), key=lambda x: -x[1])

    print("Drug repurposing candidates (network proximity to drug targets):")
    print(f"Drug targets: {drug_targets}")
    print(f"{'Disease':<30} {'Network score':>15}")
    print("-" * 47)
    for disease, score in ranked_diseases[:15]:
        n_disease_genes = len(disease_genes_dict[disease])
        print(f"{disease:<30} {score:>15.6f} ({n_disease_genes} disease genes)")

    return ranked_diseases

# Simulate disease gene sets
disease_dict = {
    f"Disease_{i}": [f"GENE{j}" for j in range(i*5, i*5 + 10)]
    for i in range(10)
}
drug_repurposing_by_propagation(G_ppi, ["GENE0", "GENE1"], disease_dict)
```

## Normalizing for Degree Bias

High-degree hub nodes receive high propagation scores regardless of biological relevance — simply because they have many connections. Degree normalization corrects for this:

```python
def degree_normalized_rwr(G, seed_genes, alpha=0.85):
    """
    Normalize RWR scores by expected score for random seeds of same size.
    Reduces hub bias; better for candidate gene prioritization.
    """
    # Raw RWR scores
    raw_scores = rwr_analytical(G, seed_genes, alpha=alpha)

    # Expected score: average over many random seed sets of same size
    n_seeds = len([g for g in seed_genes if g in G])
    n_null = 50
    nodes = list(G.nodes())
    rng = np.random.default_rng(42)

    null_scores = {node: [] for node in nodes}
    for _ in range(n_null):
        random_seeds = list(rng.choice(nodes, size=n_seeds, replace=False))
        null = rwr_analytical(G, random_seeds, alpha=alpha)
        for node, score in null.items():
            null_scores[node].append(score)

    # Z-score normalization
    normalized = {}
    for node in raw_scores:
        null_list = null_scores[node]
        mu = np.mean(null_list)
        sigma = np.std(null_list)
        if sigma > 0:
            normalized[node] = (raw_scores[node] - mu) / sigma
        else:
            normalized[node] = 0

    return normalized

print("Degree-normalized RWR eliminates hub bias:")
print("  Hub genes (TP53, AKT1, EGFR) may score lower after normalization")
print("  Functional candidates with fewer connections can rank higher")
```

## HotNet2: Statistical Framework for Subnetwork Identification

**HotNet2** (Leiserson et al., 2015) extends network propagation to identify significantly "hot" subnetworks — connected subgraphs where mutations or other genomic events are enriched after propagation. It has been applied to cancer genomics to identify oncogenic subnetworks beyond individual frequently mutated genes:

```python
def hotnet2_score(interactome, mutation_scores, beta=0.3):
    """
    HotNet2-inspired: propagate mutation scores through network,
    identify connected subgraphs with high propagated scores.
    
    mutation_scores: dict {gene: mutation_frequency_or_importance}
    beta: 1-alpha (restart probability); HotNet2 uses beta-normalized kernel
    """
    nodes = list(interactome.nodes())
    n = len(nodes)
    node_to_idx = {node: i for i, node in enumerate(nodes)}

    # Seed vector: mutation scores for each gene
    h = np.zeros(n)
    for gene, score in mutation_scores.items():
        if gene in node_to_idx:
            h[node_to_idx[gene]] = score

    # Propagate with beta restart
    scores = rwr_analytical(interactome, [], alpha=1-beta)
    # (Full implementation requires beta-normalized Laplacian diffusion kernel)

    # Threshold and find connected "hot" subnetwork
    threshold = np.percentile([s for s in scores.values() if s > 0], 90)
    hot_nodes = {gene for gene, score in scores.items() if score >= threshold}
    hot_subgraph = interactome.subgraph(hot_nodes)

    # Find largest connected hot subnetwork
    components = sorted(nx.connected_components(hot_subgraph), key=len, reverse=True)
    largest_hot = components[0] if components else set()

    print(f"HotNet2 results:")
    print(f"  Hot nodes (top 10%): {len(hot_nodes)}")
    print(f"  Largest hot subnetwork: {len(largest_hot)} nodes")
    print(f"  (Statistical significance requires permutation testing)")
    return largest_hot
```

## Why This Matters

Network propagation is one of the most practically impactful network analysis methods in biology. It operationalizes the intuition that "guilt by network association" — a gene with many interactions with disease genes is itself likely disease-relevant — into a mathematically principled algorithm. In cancer genomics, propagation from frequently mutated genes (seeds) prioritizes candidate driver genes in the long tail of rarely mutated, biologically important genes that statistics alone cannot identify. In pharmacology, propagating from drug target proteins identifies diseases whose causal networks overlap with the drug's mechanism, suggesting repurposing opportunities that have now been validated for multiple drugs. The method's simplicity (essentially sparse linear algebra) belies its power — it effectively summarizes the full network structure into a gene ranking in a single pass.
