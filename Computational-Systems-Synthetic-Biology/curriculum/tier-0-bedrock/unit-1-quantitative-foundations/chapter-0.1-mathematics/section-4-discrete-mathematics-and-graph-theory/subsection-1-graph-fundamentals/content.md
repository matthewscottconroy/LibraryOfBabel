# Graph Fundamentals

In 1999, two groups published back-to-back papers in *Science* that changed how biologists thought about the cell. Albert, Jeong, and Barabási had analyzed the network of metabolic reactions across 43 organisms; Jeong, Tombor, Albert, Oltvai, and Barabási had analyzed the yeast protein interaction network. In both cases, they found the same striking pattern: the distribution of node degrees — how many partners each protein or metabolite had — was not bell-shaped. It followed a power law. A few nodes had dozens or hundreds of connections while most had just one or two. The authors called these high-degree nodes "hubs," and they proposed that these networks were fundamentally different in their organization from random graphs. The field of network biology was born.

Biological systems are fundamentally relational — molecules interact with molecules, genes regulate genes, species eat species, neurons connect to neurons. The mathematics of graphs (also called networks) provides the formal language for representing, analyzing, and reasoning about these relationships. Mastering graph theory is essential for working with protein interaction networks, gene regulatory networks, metabolic networks, and signaling pathways.

## What Is a Graph?

A **graph** $G = (V, E)$ consists of:
- A set of **vertices** (nodes) $V$ — typically representing biological entities (genes, proteins, metabolites, cells)
- A set of **edges** $E \subseteq V \times V$ — representing relationships or interactions between entities

The **order** of a graph is $|V|$ (number of nodes); the **size** is $|E|$ (number of edges).

**Directed vs. undirected graphs:**

In an **undirected graph**, edges have no direction: $(u, v) \in E$ implies $(v, u) \in E$. Undirected graphs model symmetric relationships — protein-protein physical interactions, co-expression relationships, and genomic contact maps (Hi-C data) are typically represented as undirected.

In a **directed graph (digraph)**, edges are ordered pairs $(u, v)$ — there is a distinction between an edge from $u$ to $v$ and from $v$ to $u$. Gene regulatory networks are directed: TF A activates gene B, but B does not necessarily regulate A. Metabolic networks are directed: a reaction converts substrate to product, not vice versa (in general). The direction captures the causal or mechanistic flow of information.

**Weighted vs. unweighted:**

In a **weighted graph**, each edge $(u, v)$ carries a numerical weight $w_{uv}$. Protein interaction networks can be weighted by interaction confidence scores; co-expression networks by Pearson correlation coefficient; metabolic networks by reaction flux.

## Adjacency Matrices and Adjacency Lists

Two primary data structures represent graphs:

**Adjacency matrix:** An $n \times n$ matrix $A$ where $A_{ij} = 1$ (or $w_{ij}$) if there is an edge from $i$ to $j$, and 0 otherwise.

- For undirected graphs: $A$ is symmetric ($A_{ij} = A_{ji}$)
- Advantage: $O(1)$ edge lookup — ideal for dense graphs
- Disadvantage: $O(n^2)$ memory — prohibitive for large sparse networks (a PPI network with 10,000 proteins uses 100 million entries, but has only $\sim 150,000$ edges)

**Adjacency list:** Each node stores the list of its neighbors. Total memory is $O(n + m)$ where $m = |E|$. Biological networks are almost always sparse ($m \ll n^2$), so adjacency lists are standard.

```python
import networkx as nx

# Build a small gene regulatory network
G = nx.DiGraph()
G.add_edges_from([
    ('TF1', 'GeneA'), ('TF1', 'GeneB'),
    ('TF2', 'GeneA'), ('TF2', 'GeneC'),
    ('GeneA', 'GeneC'),   # feedforward
    ('GeneC', 'TF2')      # feedback
])

# Adjacency matrix
A = nx.adjacency_matrix(G).toarray()
print("Adjacency matrix:\n", A)
print("Nodes:", list(G.nodes()))
```

## Paths, Cycles, and Connectivity

A **path** from $u$ to $v$ is a sequence of vertices $u = v_0, v_1, \ldots, v_k = v$ where $(v_{i-1}, v_i) \in E$ for all $i$. The **length** of the path is $k$ (number of edges).

A **cycle** is a path that starts and ends at the same vertex with at least one edge — it represents a feedback loop in a regulatory network. The presence of cycles is not a curiosity; it is biologically meaningful. Positive cycles (where the sign product around the loop is positive) tend to create bistability and memory. Negative cycles (odd number of repressing interactions) tend to create oscillations. This connection between graph topology and dynamical behavior is one of the central insights of systems biology.

**Connectivity:**
- An undirected graph is **connected** if there is a path between every pair of vertices
- A directed graph is **strongly connected** if there is a directed path from $u$ to $v$ and from $v$ to $u$ for every pair
- A directed graph is **weakly connected** if the underlying undirected graph is connected

Most real biological networks are not fully connected — they consist of several **connected components**. The **giant connected component** (largest component) typically contains most nodes; the other components are smaller, isolated subnetworks.

**Simple vs. multigraph:** A **simple graph** has no self-loops (edges from a node to itself) and no multi-edges (at most one edge between any pair of nodes). In gene regulatory networks, an autoregulatory loop (a gene regulating its own expression) is represented as a self-loop.

## Types of Graphs in Biology

**Trees:** Acyclic connected graphs. Phylogenetic trees represent evolutionary relationships between species. A tree on $n$ nodes has exactly $n-1$ edges. The tree topology — branching pattern, branch lengths — encodes the history of divergence from a common ancestor.

**Bipartite graphs:** Vertex set split into two disjoint sets $U$ and $V$; all edges go between $U$ and $V$. Gene-disease association networks are bipartite (genes on one side, diseases on the other). Drug-target networks. Transcription factor binding: TFs and their target genes.

**Hypergraphs:** Edges (hyperedges) can connect more than two vertices. Metabolic reactions are hyperedges connecting multiple substrates and products — a standard edge representation is an approximation.

**Multilayer networks:** Networks where different types of relationships between the same entities are captured in separate layers — for example, protein-protein interactions, gene regulatory relationships, and metabolic connections in an integrated multi-omics network.

## Why This Matters for Computational Biology

Networks are the native representation of biological systems. The protein interaction network tells you which proteins are physically associated. The gene regulatory network encodes transcriptional logic. The metabolic network describes biochemical transformations. Understanding graphs — their fundamental properties, how to represent them efficiently, and how to ask structural questions about them — is the prerequisite for every analysis that follows. Network topology, centrality, motif analysis, and flow algorithms all build on this foundation.
