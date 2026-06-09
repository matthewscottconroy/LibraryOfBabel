# Clustering Coefficient

Proteins do not interact randomly. They form complexes — stable, structured assemblies in which every member touches every other member. The proteasome, the ribosome, the nuclear pore complex: these are cliques in the network, dense little clusters of mutual interaction. Between complexes, individual "adaptor" proteins reach out in many directions without their neighbors forming tight groups. You can see these structural features in the network by asking, for each node, what fraction of its neighbors are connected to one another. This is the clustering coefficient, and it partitions proteins into two fundamentally different roles: embedded complex members with high clustering, and bridging connectors with low clustering. That distinction turns out to matter for everything from drug targeting to understanding how cellular signals propagate.

The **clustering coefficient** measures how interconnected the neighbors of a node are — or equivalently, the density of triangles in the local neighborhood. In biological networks, high clustering reflects the tendency of proteins to form complexes, metabolic reactions to cluster into pathways, and genes to participate in co-regulated modules. The clustering coefficient complements degree by capturing local cohesion rather than just connectivity breadth.

## Local Clustering Coefficient

For node $i$ with degree $k_i \geq 2$, the **local clustering coefficient** $C_i$ is the fraction of possible edges among $i$'s neighbors that actually exist:

$$C_i = \frac{|\{e_{jk} : j,k \in N(i), e_{jk} \in E\}|}{\binom{k_i}{2}} = \frac{2 t_i}{k_i(k_i - 1)}$$

where $N(i)$ is the set of neighbors of $i$ and $t_i$ is the number of triangles containing $i$.

**Geometric interpretation**: if node $i$ has 4 neighbors and exactly 2 of the 6 possible edges between those neighbors exist, then $C_i = 2/6 = 0.33$.

**Biological interpretation**:
- $C_i = 1$: all neighbors of $i$ interact with each other — $i$ is at the center of a clique (protein complex)
- $C_i \approx 0$: neighbors of $i$ do not interact — $i$ connects otherwise separate groups (bridge or bottleneck)

```python
import networkx as nx
import numpy as np
import matplotlib.pyplot as plt

G = nx.karate_club_graph()  # Classic social network (proxy for PPI structure)

# Local clustering coefficients
local_cc = nx.clustering(G)  # dict: {node: C_i}

# Display top clustered nodes
sorted_cc = sorted(local_cc.items(), key=lambda x: -x[1])
print("Nodes with highest local clustering:")
for node, cc in sorted_cc[:5]:
    k = G.degree(node)
    print(f"  Node {node}: C = {cc:.3f}, k = {k}")

# Manual calculation to verify
def manual_clustering(G, node):
    """Manually compute clustering coefficient for verification."""
    neighbors = list(G.neighbors(node))
    k = len(neighbors)
    if k < 2:
        return 0.0

    # Count edges among neighbors
    n_triangles = 0
    for i in range(len(neighbors)):
        for j in range(i+1, len(neighbors)):
            if G.has_edge(neighbors[i], neighbors[j]):
                n_triangles += 1

    possible = k * (k - 1) / 2
    return n_triangles / possible

# Verify for one node
node = 0
manual = manual_clustering(G, node)
nx_result = local_cc[node]
print(f"\nVerification for node {node}:")
print(f"  NetworkX: {nx_result:.4f}")
print(f"  Manual:   {manual:.4f}")
assert abs(manual - nx_result) < 1e-10
```

## Global Clustering Coefficient

The **global clustering coefficient** (or **transitivity**) is the fraction of all possible triangles that are closed:

$$C_\text{global} = \frac{3 \times \text{(number of triangles)}}{\text{(number of connected triples)}}$$

A **connected triple** is any path of length 2 (center node and two neighbors). The factor of 3 accounts for the three ways to root a triangle at one of its nodes.

```python
# Global clustering coefficient
C_global = nx.transitivity(G)
print(f"Global clustering coefficient (transitivity): {C_global:.4f}")

# Average clustering coefficient (arithmetic mean of local C_i)
C_avg = nx.average_clustering(G)
print(f"Average clustering coefficient: {C_avg:.4f}")
print("Note: average_clustering ≠ transitivity (weighted differently by degree)")
```

## Clustering vs. Degree: The Scale-Free Pattern

In scale-free networks, high-degree nodes tend to have lower clustering coefficients than low-degree nodes. This reflects the architecture of the network: hubs connect to many nodes in different parts of the network (low clustering), while low-degree nodes are often embedded within tight clusters (high clustering).

This scaling follows:

$$C(k) \sim k^{-\beta}, \quad \beta \approx 1$$

```python
def clustering_vs_degree(G):
    """Analyze relationship between degree and clustering coefficient."""
    degrees = np.array([G.degree(n) for n in G.nodes()])
    clustering = np.array([nx.clustering(G, n) for n in G.nodes()])

    # Bin by degree and compute mean clustering
    unique_k = np.unique(degrees)
    mean_c_by_k = []
    for k in unique_k:
        mask = degrees == k
        if mask.sum() > 0:
            mean_c_by_k.append(clustering[mask].mean())
        else:
            mean_c_by_k.append(np.nan)

    fig, axes = plt.subplots(1, 2, figsize=(12, 4))

    # Scatter plot
    axes[0].scatter(degrees, clustering, alpha=0.3, s=20, color="steelblue")
    axes[0].set_xlabel("Degree k")
    axes[0].set_ylabel("Clustering coefficient C")
    axes[0].set_title("Clustering vs. degree")

    # Log-log: scale-free relationship C(k) ~ k^(-β)
    mask = (unique_k > 0) & (np.array(mean_c_by_k) > 0)
    if mask.sum() > 3:
        log_k = np.log(unique_k[mask])
        log_c = np.log(np.array(mean_c_by_k)[mask])
        axes[1].loglog(unique_k[mask], np.array(mean_c_by_k)[mask],
                        "o", color="steelblue", markersize=5, label="Mean C(k)")
        # Fit power law
        coeffs = np.polyfit(log_k, log_c, 1)
        beta = -coeffs[0]
        k_range = np.linspace(unique_k[mask].min(), unique_k[mask].max(), 50)
        axes[1].loglog(k_range, np.exp(np.polyval(coeffs, np.log(k_range))),
                        "r--", lw=2, label=f"C ~ k^(-{beta:.2f})")
        axes[1].legend()
        axes[1].set_xlabel("log(k)")
        axes[1].set_ylabel("log <C(k)>")
        axes[1].set_title("Hierarchical structure: C(k) vs. k")
        print(f"C(k) ~ k^(-{beta:.2f}) (hierarchical if β ≈ 1)")

    plt.tight_layout()
    return fig

clustering_vs_degree(G)
```

## Biological Interpretation: Protein Complexes and Modules

High clustering among a set of proteins often indicates a **protein complex**. This provides a computational approach to complex detection:

```python
def find_high_clustering_clusters(G, cc_threshold=0.5, size_threshold=4):
    """
    Identify groups of nodes with high mutual clustering — potential protein complexes.
    Simple approach: find cliques and filter by size.
    """
    # Find all maximal cliques (all-to-all connected subgraphs)
    cliques = list(nx.find_cliques(G))
    large_cliques = [c for c in cliques if len(c) >= size_threshold]
    large_cliques.sort(key=len, reverse=True)

    print(f"Potential protein complexes (cliques ≥ {size_threshold} nodes):")
    for i, clique in enumerate(large_cliques[:5]):
        # Compute local clustering within this clique
        subg = G.subgraph(clique)
        density = nx.density(subg)
        print(f"  Complex {i+1}: {clique}, density = {density:.2f}")

    # For more realistic complex detection: use community detection (Section 3)
    return large_cliques

cliques = find_high_clustering_clusters(G)
```

## Worked Example: Comparing PPI and Random Networks

```python
def compare_clustering(n=500, p=0.02, seed=42):
    """
    Compare clustering coefficient of a scale-free PPI proxy to a random graph
    with the same number of nodes and edges.
    """
    np.random.seed(seed)

    # Scale-free (Barabási-Albert)
    G_sf = nx.barabasi_albert_graph(n, m=3, seed=seed)

    # Random with same edge count
    m = G_sf.number_of_edges()
    G_rand = nx.erdos_renyi_graph(n, 2*m/(n*(n-1)), seed=seed)

    C_sf   = nx.average_clustering(G_sf)
    C_rand = nx.average_clustering(G_rand)
    L_sf   = nx.average_shortest_path_length(G_sf) if nx.is_connected(G_sf) else np.nan
    L_rand = nx.average_shortest_path_length(G_rand) if nx.is_connected(G_rand) else np.nan

    print(f"{'Metric':<35} {'Scale-free':>12} {'Random':>12}")
    print("-" * 60)
    print(f"{'Nodes':<35} {n:>12} {n:>12}")
    print(f"{'Edges':<35} {G_sf.number_of_edges():>12} {G_rand.number_of_edges():>12}")
    print(f"{'Average clustering C':<35} {C_sf:>12.4f} {C_rand:>12.4f}")
    print(f"{'Average path length L':<35} {L_sf:>12.4f} {L_rand:>12.4f}")
    print(f"{'C / C_random':<35} {C_sf/C_rand:>12.2f} {1.0:>12.2f}")
    print()
    print("Small-world if: C >> C_random AND L ≈ L_random")

compare_clustering()
```

## Why This Matters

The clustering coefficient is the gateway to understanding modularity in biological networks. A protein with $C_i \approx 1$ is embedded in a protein complex — disrupting it is likely lethal to that complex. A protein with $C_i \approx 0$ despite high degree is a **bridge hub** — its interactions connect different functional modules, and disrupting it could disconnect multiple pathways. This distinction (party hub vs. date hub) has pharmacological consequences: date hubs (low clustering) are poor drug targets because inhibiting them causes widespread disruption; party hubs (high clustering) within a disease-relevant complex are more tractable. The average clustering coefficient, compared to a null model (random graph with same degree sequence), is also a key component of the small-world test discussed in the next section.
