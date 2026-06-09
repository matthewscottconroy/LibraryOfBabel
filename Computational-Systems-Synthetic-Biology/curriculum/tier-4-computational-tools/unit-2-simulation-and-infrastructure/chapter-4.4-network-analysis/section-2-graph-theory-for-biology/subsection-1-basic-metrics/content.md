# Basic Graph Metrics for Biological Networks

Here is a question worth sitting with for a moment: what do you actually learn when you look at a list of 300,000 protein-protein interactions? In raw form, very little — the list is a fog of gene names. But compute a handful of numbers from that list and something comes into focus. Count the interactions per protein and you find a handful of proteins with hundreds of partners and thousands with just one or two. Compute the average path between any two proteins and you discover that the human interactome has a diameter of roughly 14 steps — but the average path is only about 5. Measure how tightly each protein's neighbors interconnect and you identify protein complexes without any biochemical fractionation. Graph metrics are not abstractions imposed on biology from outside; they are instruments for making the biology legible.

Graph theory provides a vocabulary of quantitative descriptors that characterize network topology. For biological networks, these metrics are not just mathematical abstractions — they have direct biological interpretations and predictive power for function, essentiality, and disease relevance. This section introduces the foundational metrics; subsequent sections treat degree distribution, clustering, path length, and centrality in depth.

## Nodes, Edges, and Degree

For a graph $G = (V, E)$ with $|V| = n$ nodes and $|E| = m$ edges:

**Degree** $k_i$ of node $i$: the number of edges incident to $i$.
- For undirected graphs: $k_i = \sum_{j} A_{ij}$, where $A$ is the adjacency matrix
- For directed graphs: in-degree $k_i^\text{in}$ (edges entering $i$) and out-degree $k_i^\text{out}$ (edges leaving $i$)

The **adjacency matrix** $A \in \{0,1\}^{n \times n}$ encodes graph topology:

$$A_{ij} = \begin{cases} 1 & \text{if } (i,j) \in E \\ 0 & \text{otherwise} \end{cases}$$

```python
import networkx as nx
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# Load a PPI network from BioGRID (simplified example)
def build_example_ppi():
    """Construct a small PPI network for demonstration."""
    G = nx.Graph()
    edges = [
        ("TP53", "MDM2", {"weight": 0.95}),
        ("TP53", "BRCA1", {"weight": 0.78}),
        ("TP53", "CDKN1A", {"weight": 0.88}),
        ("TP53", "ATM", {"weight": 0.82}),
        ("MDM2", "HAUSP", {"weight": 0.71}),
        ("BRCA1", "BARD1", {"weight": 0.91}),
        ("BRCA1", "RAD51", {"weight": 0.85}),
        ("ATM", "CHEK2", {"weight": 0.79}),
        ("CHEK2", "CDC25A", {"weight": 0.68}),
        ("EGFR", "KRAS", {"weight": 0.77}),
        ("KRAS", "RAF1", {"weight": 0.84}),
        ("RAF1", "MAP2K1", {"weight": 0.90}),
        ("TP53", "KRAS", {"weight": 0.55}),
    ]
    G.add_edges_from(edges)
    return G

G = build_example_ppi()

# Basic degree analysis
degrees = dict(G.degree())
in_degree  = dict(G.in_degree())  if G.is_directed() else degrees
out_degree = dict(G.out_degree()) if G.is_directed() else degrees

# Summary statistics
k_values = list(degrees.values())
print(f"Nodes: {G.number_of_nodes()}")
print(f"Edges: {G.number_of_edges()}")
print(f"Mean degree <k>: {np.mean(k_values):.2f}")
print(f"Max degree:      {max(k_values)} ({max(degrees, key=degrees.get)})")
print(f"Min degree:      {min(k_values)}")

# Top hub proteins
sorted_by_degree = sorted(degrees.items(), key=lambda x: -x[1])
print("\nTop 5 hubs (by degree):")
for protein, degree in sorted_by_degree[:5]:
    print(f"  {protein}: degree = {degree}")
```

## Network Density

**Density** measures the fraction of possible edges that actually exist:

$$d = \frac{m}{\binom{n}{2}} = \frac{2m}{n(n-1)}$$

for an undirected graph. PPI networks are typically very sparse ($d \approx 10^{-4}$ for the full human interactome), while curated pathway-level networks are denser ($d \approx 0.1$).

```python
density = nx.density(G)
print(f"Network density: {density:.4f}")
print(f"  Sparse: < 0.01; Dense: > 0.1")
print(f"  Fully connected: 1.0; No edges: 0.0")
```

## Connected Components

A **connected component** is a maximal set of nodes where every pair is connected by a path. Most large biological networks have:
- A **giant connected component (GCC)**: containing the vast majority of nodes
- Small isolated components or singletons

```python
if not nx.is_connected(G):
    components = sorted(nx.connected_components(G), key=len, reverse=True)
    print(f"Number of connected components: {len(components)}")
    print(f"Largest component: {len(components[0])} nodes "
          f"({len(components[0])/G.number_of_nodes()*100:.1f}%)")
    # Work with the giant connected component (GCC)
    G_gcc = G.subgraph(components[0]).copy()
else:
    G_gcc = G
    print("Network is connected (single component)")
```

## Shortest Paths and Average Path Length

A **path** between nodes $i$ and $j$ is a sequence of distinct nodes connected by edges. The **shortest path** $d(i,j)$ is the minimum number of edges in any path between $i$ and $j$:

The **average shortest path length**:
$$L = \frac{1}{n(n-1)} \sum_{i \neq j} d(i,j)$$

For biological networks, $L$ is typically small (logarithmic in $n$) — the "small world" property discussed in Section 4.

```python
# Average path length (only for connected graph; expensive for large networks)
L = nx.average_shortest_path_length(G_gcc)
print(f"Average shortest path length: {L:.2f}")

# For large networks: sample-based estimate
def approximate_path_length(G, n_samples=1000, seed=42):
    """Approximate average path length by sampling node pairs."""
    rng = np.random.default_rng(seed)
    nodes = list(G.nodes())
    path_lengths = []

    for _ in range(n_samples):
        u, v = rng.choice(nodes, size=2, replace=False)
        try:
            length = nx.shortest_path_length(G, u, v)
            path_lengths.append(length)
        except nx.NetworkXNoPath:
            pass  # disconnected pair

    return np.mean(path_lengths), np.std(path_lengths)

L_approx, L_std = approximate_path_length(G_gcc)
print(f"Approximate L: {L_approx:.2f} ± {L_std:.2f}")
```

## Network Diameter

The **diameter** is the longest shortest path in the network:

$$\text{diam}(G) = \max_{i,j} d(i,j)$$

For many biological networks, the diameter is 4–6 (6 degrees of separation in PPI networks; 4–5 in co-expression networks). The diameter characterizes the most remote pair of nodes.

```python
diameter = nx.diameter(G_gcc)
print(f"Network diameter: {diameter}")
periphery = nx.periphery(G_gcc)  # nodes achieving diameter
center = nx.center(G_gcc)        # nodes with minimum eccentricity
print(f"Network center nodes: {center}")
```

## The Adjacency Matrix and Its Spectrum

The eigenvalues of the adjacency matrix (the **graph spectrum**) encode global network structure:

```python
# Adjacency matrix
A = nx.adjacency_matrix(G, weight=None).toarray().astype(float)
eigenvalues = np.linalg.eigvalsh(A)  # sorted ascending

# Spectral properties
lambda_max = eigenvalues[-1]  # largest eigenvalue (spectral radius)
spectral_gap = eigenvalues[-1] - eigenvalues[-2]  # larger gap → more modular

print(f"Spectral radius (λ_max): {lambda_max:.3f}")
print(f"  Related to maximum degree: λ_max ≤ k_max")
print(f"Spectral gap: {spectral_gap:.3f}")
print(f"  Larger gap → better connectivity / less modular structure")

# Plot eigenvalue distribution
fig, ax = plt.subplots(figsize=(7, 3))
ax.hist(eigenvalues, bins=30, color="steelblue", alpha=0.7)
ax.axvline(lambda_max, color="red", ls="--", label=f"λ_max = {lambda_max:.2f}")
ax.set_xlabel("Eigenvalue")
ax.set_ylabel("Count")
ax.set_title("Adjacency matrix spectrum")
ax.legend()
plt.tight_layout()
```

## Complete Basic Analysis Pipeline

```python
def complete_network_analysis(G, name="Network"):
    """Run complete basic analysis on a network and return summary dict."""
    metrics = {}

    # Basic topology
    metrics["n_nodes"] = G.number_of_nodes()
    metrics["n_edges"] = G.number_of_edges()
    metrics["density"] = nx.density(G)

    # Degree statistics
    degrees = [d for _, d in G.degree()]
    metrics["mean_degree"]   = np.mean(degrees)
    metrics["max_degree"]    = max(degrees)
    metrics["degree_std"]    = np.std(degrees)

    # Connectivity
    components = list(nx.connected_components(G) if not G.is_directed()
                      else nx.weakly_connected_components(G))
    metrics["n_components"] = len(components)
    metrics["gcc_fraction"]  = max(len(c) for c in components) / metrics["n_nodes"]

    # Path-based (on GCC)
    gcc = G.subgraph(max(components, key=len))
    if len(gcc) > 1:
        metrics["avg_path_length"]    = nx.average_shortest_path_length(gcc) \
            if len(gcc) < 5000 else np.nan
        metrics["diameter"]            = nx.diameter(gcc) if len(gcc) < 5000 else np.nan
    metrics["avg_clustering"]  = nx.average_clustering(G)

    # Print summary
    print(f"\n{'='*50}")
    print(f"Network: {name}")
    print(f"{'='*50}")
    for k, v in metrics.items():
        if isinstance(v, float):
            print(f"  {k}: {v:.4f}")
        else:
            print(f"  {k}: {v}")
    return metrics

complete_network_analysis(G, name="TP53/EGFR PPI subnetwork")
```

## Why This Matters

Basic graph metrics provide the starting vocabulary for all network biology analyses. Degree identifies hub proteins. Path length quantifies how quickly signals propagate through a regulatory network. Density distinguishes tightly interconnected pathway-level graphs from sparse interactome-wide data. These simple numbers tell a biological story: a high-degree node in a PPI network is likely an essential protein; a network with short average path length can rapidly propagate perturbations; a sparse network dominated by a giant component suggests that removing hub nodes could fragment communication. Before any sophisticated community detection or centrality analysis, computing basic metrics is the essential first step.
