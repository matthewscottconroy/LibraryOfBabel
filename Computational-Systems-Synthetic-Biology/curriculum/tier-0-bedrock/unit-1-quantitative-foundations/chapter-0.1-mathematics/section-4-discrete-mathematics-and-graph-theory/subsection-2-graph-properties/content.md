# Graph Properties

When Jeong and colleagues analyzed the yeast protein interaction network, they did not just report a degree distribution. They ran an experiment: what happens to the network when you remove nodes? They found that random removal of yeast proteins rarely disconnected the network — there were many redundant paths. But targeted removal of the few high-degree hubs fragmented the network rapidly. This "robustness to random failures but vulnerability to targeted attack" was a direct consequence of the scale-free topology. The mathematical properties of the network predicted the biological robustness of yeast to random mutations — and, by implication, the targets that would be most lethal if disrupted.

Beyond the basic structure of a graph, a rich set of quantitative properties characterizes its topology. These properties reveal how biological networks are organized, how robust they are to perturbation, how information flows, and how they compare to random networks. Understanding these metrics is the foundation of network biology.

## Degree and Degree Distribution

The **degree** of a node $v$ in an undirected graph is the number of edges incident to it: $\deg(v) = |\{u : (u,v) \in E\}|$.

In directed graphs, distinguish **in-degree** (edges arriving at $v$) and **out-degree** (edges leaving $v$). In a gene regulatory network, a transcription factor with high out-degree is a master regulator; a gene with high in-degree is a highly regulated target.

The **degree distribution** $P(k)$ is the fraction of nodes with degree $k$. This is one of the most informative properties of a network:

- **Poisson distribution ($P(k) \sim \text{Poisson}(\langle k \rangle)$):** Characteristic of Erdos-Renyi random graphs where each pair of nodes is connected independently with probability $p$. Most nodes have degree close to the mean; very high-degree nodes are extremely rare.

- **Power-law distribution ($P(k) \sim k^{-\gamma}$):** Found empirically in many biological networks — protein interaction networks, metabolic networks, gene regulatory networks. Also called **scale-free** networks. Power-law degree distributions have "fat tails": there are a few extremely high-degree nodes (**hubs**) while most nodes have low degree. The exponent $\gamma$ typically falls between 2 and 3 for biological networks.

The scale-free topology of protein interaction networks has important implications: these networks are robust to random node removal (most nodes are low-degree, so removing a random node rarely disrupts connectivity) but vulnerable to targeted removal of hubs (which can rapidly fragment the network). This mathematical property has direct clinical implications: essential genes and drug targets are enriched among hubs.

## Clustering Coefficient

The **local clustering coefficient** of node $v$ measures how densely connected $v$'s neighbors are to each other:

$$C_v = \frac{\text{number of triangles through } v}{\binom{\deg(v)}{2}} = \frac{2 t_v}{k_v(k_v - 1)}$$

where $t_v$ is the number of edges between neighbors of $v$ and $k_v = \deg(v)$. If $k_v \leq 1$, define $C_v = 0$.

The **global clustering coefficient** (or transitivity) averages over all nodes: $C = \frac{1}{n} \sum_v C_v$.

High clustering indicates that biological networks are locally organized into **functional modules** — densely interconnected groups of proteins that cooperate in a specific cellular function. This modularity has been verified experimentally: proteins in the same module tend to have similar functions, similar expression patterns, and similar phenotypic effects when knocked out.

## Shortest Paths and Small-World Property

The **distance** $d(u, v)$ between two nodes is the length of the shortest path between them (infinite if no path exists). The **diameter** of a graph is $\max_{u,v} d(u, v)$.

The **average shortest path length** $\langle d \rangle = \frac{1}{n(n-1)} \sum_{u \neq v} d(u, v)$ measures the typical separation between nodes.

The **small-world property** describes networks where:
- $\langle d \rangle$ is small (comparable to $\log n$ — like random graphs)
- $C$ is large (much larger than random graphs)

Real biological networks — and social networks — exhibit this small-world structure. The yeast protein interaction network has $\langle d \rangle \approx 6$ despite having thousands of nodes, meaning any two proteins are connected through a short chain of interactions. This has functional consequences: signals can propagate quickly through the network. It also has implications for disease: a mutation that disrupts one hub can rapidly propagate its effects to functionally distant parts of the cell through these short paths.

## Centrality Measures

**Centrality** quantifies the "importance" of a node in a network. Different centrality measures capture different aspects of importance:

**Degree centrality:** $C_D(v) = \deg(v) / (n-1)$. Simple, but captures local connectivity only.

**Betweenness centrality:** The fraction of all-pairs shortest paths that pass through $v$:

$$C_B(v) = \sum_{s \neq v \neq t} \frac{\sigma_{st}(v)}{\sigma_{st}}$$

where $\sigma_{st}$ is the total number of shortest paths from $s$ to $t$ and $\sigma_{st}(v)$ is those passing through $v$. Nodes with high betweenness are **bottlenecks** — removing them fragments the network. In signaling networks, high-betweenness kinases are critical signal integration points. These are often the most valuable drug targets: they sit at crossroads where many signaling pathways converge.

**Closeness centrality:** $C_C(v) = (n-1) / \sum_{u \neq v} d(u, v)$. Nodes close to all others can broadcast information efficiently.

**Eigenvector centrality:** $\mathbf{c} = \lambda^{-1} A \mathbf{c}$, i.e., $\mathbf{c}$ is the principal eigenvector of the adjacency matrix $A$. A node's importance is proportional to the importance of its neighbors. PageRank is a damped version of eigenvector centrality.

```python
import networkx as nx
import numpy as np

# Build a small protein interaction network
ppi = nx.barabasi_albert_graph(100, 2, seed=42)  # scale-free network

# Degree distribution
degrees = [d for n, d in ppi.degree()]
print(f"Mean degree: {np.mean(degrees):.2f}")
print(f"Max degree (hub): {max(degrees)}")

# Clustering
print(f"Global clustering coefficient: {nx.average_clustering(ppi):.3f}")

# Shortest paths
print(f"Average shortest path length: {nx.average_shortest_path_length(ppi):.2f}")

# Centrality measures
bc = nx.betweenness_centrality(ppi, normalized=True)
dc = dict(ppi.degree())
ec = nx.eigenvector_centrality(ppi)

# Identify the top hub by betweenness
top_node = max(bc, key=bc.get)
print(f"\nTop betweenness node {top_node}:")
print(f"  Degree: {dc[top_node]}, Betweenness: {bc[top_node]:.3f}, Eigenvector: {ec[top_node]:.3f}")
```

## Spanning Trees and Minimum Spanning Trees

A **spanning tree** of a connected graph is a subgraph that is a tree and includes every vertex. It uses $n-1$ edges to keep the graph connected with no cycles.

The **minimum spanning tree (MST)** minimizes the total edge weight. MSTs arise in:
- **Phylogenetic reconstruction:** UPGMA and neighbor-joining produce minimum spanning trees under certain distance models
- **Single-cell trajectory analysis:** the MST of a cell-cell similarity graph defines a minimal trajectory skeleton connecting all cells
- **Network visualization:** MSTs provide a compact, hierarchical visualization of complex networks

**Kruskal's algorithm:** Sort edges by weight; add edges greedily if they do not create a cycle. $O(m \log m)$ time.

## Why This Matters for Computational Biology

The properties covered here are the quantitative descriptors used in network medicine, systems biology, and synthetic biology. Hub proteins in PPI networks are enriched for essential genes and drug targets. The clustering structure of metabolic networks reveals functional modules corresponding to metabolic pathways. Small-world topology enables efficient signal propagation in signaling networks. Betweenness centrality identifies the most critical regulatory bottlenecks — potential points of fragility or control. Understanding these properties lets you reason about biological networks far beyond what any individual node-level analysis can reveal.
