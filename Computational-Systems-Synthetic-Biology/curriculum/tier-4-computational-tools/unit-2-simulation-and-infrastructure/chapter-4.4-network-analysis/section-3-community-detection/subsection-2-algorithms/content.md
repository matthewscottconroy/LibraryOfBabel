# Community Detection Algorithms

Finding communities in a network is, in the worst case, an NP-hard problem — there is no guaranteed polynomial-time algorithm that finds the optimal partition of nodes into groups. This might sound discouraging, but in practice it is not a serious obstacle. The biological world is not adversarially constructed; real networks with genuine community structure can be accurately partitioned by efficient heuristic algorithms that optimize good objective functions. The Louvain and Leiden algorithms have become the workhorses of network biology not because they are provably optimal, but because they are fast, scalable to millions of nodes, and produce biologically meaningful results. Understanding what they are optimizing — and where they can go wrong — is what separates rigorous analysis from cargo-cult network science.

Community detection is an NP-hard problem in general — no algorithm is guaranteed to find the optimal partition. In practice, heuristic algorithms that optimize modularity or use information-theoretic objectives produce biologically useful community assignments. The Louvain and Leiden algorithms are the current workhorses for large biological networks; Girvan-Newman and Infomap serve specific use cases.

## The Louvain Algorithm

The **Louvain algorithm** (Blondel et al., 2008) is a greedy, hierarchical method for optimizing modularity $Q$. It proceeds in two phases that alternate until convergence:

**Phase 1 (local optimization)**: For each node $v$, compute the modularity gain from moving $v$ to each of its neighbors' communities:

$$\Delta Q = \left[\frac{\Sigma_\text{in} + 2k_{v,C}}{2m} - \left(\frac{\Sigma_\text{tot} + k_v}{2m}\right)^2\right] - \left[\frac{\Sigma_\text{in}}{2m} - \left(\frac{\Sigma_\text{tot}}{2m}\right)^2 - \left(\frac{k_v}{2m}\right)^2\right]$$

where $\Sigma_\text{in}$ is the sum of weights of edges inside community $C$, $\Sigma_\text{tot}$ is the sum of all edge weights incident to nodes in $C$, and $k_{v,C}$ is the sum of weights of edges from $v$ to nodes in $C$.

Move $v$ to the community giving the largest positive $\Delta Q$. Repeat for all nodes until no improvement possible.

**Phase 2 (aggregation)**: Build a new network where each community from Phase 1 becomes a single node. Edges between communities become weighted edges in the new network.

Iterate both phases until the network cannot be further aggregated.

```python
import networkx as nx
import numpy as np
from networkx.algorithms.community import louvain_communities

# Louvain community detection
communities_louvain = louvain_communities(G, seed=42, resolution=1.0)
# resolution > 1: smaller, more communities; < 1: larger, fewer communities

from networkx.algorithms.community.quality import modularity
Q_louvain = modularity(G, communities_louvain)
print(f"Louvain: {len(communities_louvain)} communities, Q = {Q_louvain:.4f}")
print(f"Community sizes: {sorted([len(c) for c in communities_louvain], reverse=True)}")

# Non-deterministic: run multiple times and take best
def louvain_best(G, n_runs=10, resolution=1.0, seed=None):
    """Run Louvain multiple times and return partition with highest modularity."""
    best_Q = -1
    best_partition = None
    rng = np.random.default_rng(seed)

    for i in range(n_runs):
        run_seed = int(rng.integers(1e9))
        partition = louvain_communities(G, seed=run_seed, resolution=resolution)
        Q = modularity(G, partition)
        if Q > best_Q:
            best_Q = Q
            best_partition = partition
            print(f"  Run {i+1}: Q = {Q:.4f} {'← new best' if Q == best_Q else ''}")

    print(f"\nBest partition: {len(best_partition)} communities, Q = {best_Q:.4f}")
    return best_partition, best_Q

best_partition, best_Q = louvain_best(G, n_runs=5, seed=42)
```

## The Leiden Algorithm

The **Leiden algorithm** (Traag et al., 2019) corrects a fundamental flaw in Louvain: the Louvain algorithm can produce **disconnected communities** — nodes assigned to the same community that are not actually connected to each other. This is particularly problematic for biological networks where disconnected communities have no functional interpretation.

Leiden adds a **community refinement** step between Phase 1 and Phase 2 that ensures each community is well-connected. Additionally, Leiden uses a different local moving criterion (Constant Potts Model, CPM) that avoids the resolution limit.

```python
# Leiden algorithm via leidenalg library
# pip install leidenalg igraph

import igraph as ig
import leidenalg

def networkx_to_igraph(G_nx):
    """Convert NetworkX graph to igraph for Leiden algorithm."""
    G_ig = ig.Graph()
    nodes = list(G_nx.nodes())
    node_to_idx = {n: i for i, n in enumerate(nodes)}

    G_ig.add_vertices(len(nodes))
    G_ig.vs["name"] = nodes

    edges = [(node_to_idx[u], node_to_idx[v]) for u, v in G_nx.edges()]
    G_ig.add_edges(edges)

    # Add edge weights if present
    if nx.is_weighted(G_nx):
        weights = [G_nx[u][v].get("weight", 1.0) for u, v in G_nx.edges()]
        G_ig.es["weight"] = weights

    return G_ig, nodes

G_ig, node_list = networkx_to_igraph(G)

# Modularity-based Leiden
partition_leiden = leidenalg.find_partition(
    G_ig,
    leidenalg.ModularityVertexPartition,
    seed=42,
    n_iterations=-1  # run until convergence
)
Q_leiden = partition_leiden.modularity
print(f"Leiden (modularity): {len(partition_leiden)} communities, Q = {Q_leiden:.4f}")

# CPM-based Leiden (no resolution limit; better for small communities)
resolution_parameter = 0.05  # controls community size: smaller = more communities
partition_cpm = leidenalg.find_partition(
    G_ig,
    leidenalg.CPMVertexPartition,
    resolution_parameter=resolution_parameter,
    seed=42
)
print(f"Leiden (CPM, γ={resolution_parameter}): {len(partition_cpm)} communities")

# Convert back to NetworkX community format
communities_leiden = [
    {node_list[i] for i in partition_leiden.membership.index(c)}
    for c in range(len(partition_leiden))
]
```

## Girvan-Newman Algorithm

The **Girvan-Newman algorithm** is a hierarchical method that builds a dendrogram by iteratively removing the highest-betweenness edge:

1. Compute betweenness centrality for all edges
2. Remove the edge with highest betweenness
3. Recompute betweenness for all remaining edges
4. Repeat; track connected components (communities)

This produces a hierarchical decomposition of the network into communities at multiple scales.

```python
from networkx.algorithms.community import girvan_newman
import itertools

def girvan_newman_best_k(G, k_max=10):
    """
    Run Girvan-Newman and find the partition with highest modularity.
    WARNING: slow for large networks — O(m^2 n) per level.
    """
    gn = girvan_newman(G)
    best_Q = -1
    best_partition = None
    best_k = 1

    for k, communities in enumerate(itertools.islice(gn, k_max), start=2):
        Q = modularity(G, communities)
        print(f"  k = {k}: Q = {Q:.4f}")
        if Q > best_Q:
            best_Q = Q
            best_partition = communities
            best_k = k

    print(f"\nBest partition: k = {best_k} communities, Q = {best_Q:.4f}")
    return best_partition, best_k, best_Q

# Only practical for small networks (< 1000 nodes)
gn_partition, gn_k, gn_Q = girvan_newman_best_k(G, k_max=8)
```

## Infomap: Information-Theoretic Community Detection

**Infomap** (Rosvall & Bergstrom, 2008) uses the **map equation** — the minimum description length of a random walk on the network — rather than modularity. A partition into communities minimizes the description length if communities correspond to regions where a random walker spends most time before transitioning.

Infomap is particularly suited for **directed networks** (gene regulatory networks, signaling cascades) because it respects edge directionality in the random walk.

```python
# Infomap via igraph
partition_infomap = G_ig.community_infomap(
    edge_weights="weight" if "weight" in G_ig.edge_attributes() else None,
    trials=10  # random restarts
)
print(f"Infomap: {len(partition_infomap)} communities")
print(f"  Map equation value (codelength): {partition_infomap.codelength:.4f}")
print(f"  Lower codelength = better partition")

# For directed networks: Infomap >> Louvain
G_dir = nx.DiGraph()  # directed network
G_dir.add_edges_from([("TF1", "Gene1"), ("TF1", "Gene2"), ("TF2", "Gene2"),
                       ("TF2", "Gene3"), ("Gene1", "TF3"), ("Gene3", "TF3")])
G_dir_ig, node_list_dir = networkx_to_igraph(G_dir)
partition_dir = G_dir_ig.community_infomap()
print(f"Directed GRN Infomap: {len(partition_dir)} communities")
```

## Resolution Parameter and Multi-Scale Communities

Biological networks have communities at multiple scales: individual protein complexes (5–50 proteins), functional pathways (50–200 proteins), and broad functional categories (hundreds–thousands). The resolution parameter $\gamma$ in CPM-based Leiden controls which scale is detected:

```python
def multi_scale_communities(G, gamma_values=None):
    """
    Detect communities at multiple resolutions using CPM-Leiden.
    """
    if gamma_values is None:
        gamma_values = [0.01, 0.05, 0.1, 0.25, 0.5, 1.0]

    G_ig, node_list = networkx_to_igraph(G)

    print(f"Multi-scale community detection:")
    print(f"{'γ':<10} {'N communities':<18} {'Size range':<20} {'Biological scale'}")
    print("-" * 70)

    for gamma in gamma_values:
        part = leidenalg.find_partition(G_ig, leidenalg.CPMVertexPartition,
                                        resolution_parameter=gamma, seed=42)
        sizes = sorted([len(c) for c in part], reverse=True)
        if sizes:
            size_str = f"{min(sizes)}–{max(sizes)}"
            if gamma < 0.05:
                scale = "Large functional modules"
            elif gamma < 0.2:
                scale = "Pathways"
            else:
                scale = "Protein complexes"
            print(f"{gamma:<10.3f} {len(part):<18} {size_str:<20} {scale}")

multi_scale_communities(G)
```

## Why This Matters

The choice of community detection algorithm directly determines which biological modules are recovered from a network. Louvain is the practical default for large networks due to its speed and good performance; Leiden should be preferred when community quality matters (as it should, in any published analysis). Infomap is the correct choice for directed regulatory networks where signal flow direction matters. The resolution parameter allows targeted detection of biological structures at different scales: a $\gamma$ tuned to recover protein complexes will identify dozens of tight cliques, while a coarser $\gamma$ recovers broad pathway-level modules. Understanding the algorithm's assumptions — and whether they match your biological system — is as important as the choice of algorithm itself.
