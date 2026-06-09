# Graph Algorithms

Alon, Surette, Barkai, and Leibler published a landmark paper in *Nature* in 1999 showing that the gene regulatory network of *E. coli* is not random. Certain small subgraph patterns — particularly the feed-forward loop, in which a regulator X activates a second regulator Y, and both X and Y together activate a target gene Z — appeared far more often than would be expected by chance. These overrepresented patterns, which Alon's group called "network motifs," turned out to have functional significance: the coherent feed-forward loop filters out transient signals and only activates its target when the input is sustained. Biology had discovered a circuit design principle, and graph algorithms were the tool that revealed it.

Biological systems are fundamentally relational — genes regulate other genes, proteins interact with proteins, metabolites are connected through reactions, species are connected through evolutionary history. Graphs are the natural data structure for relational data, and graph algorithms — BFS, DFS, Dijkstra, topological sort — are the tools for extracting information from these networks. From finding the shortest metabolic pathway between two compounds to detecting regulatory network motifs to assembling genomes from k-mer overlap graphs, graph algorithms are central to computational biology.

## Graph Fundamentals

A **graph** $G = (V, E)$ consists of vertices $V$ and edges $E$. Variations:
- **Directed (digraph)**: edges have orientation ($u \to v$); regulatory networks, metabolic pathways
- **Undirected**: edges are symmetric; protein-protein interaction networks
- **Weighted**: edges have a numerical weight; metabolic fluxes, expression correlation
- **Bipartite**: vertices split into two sets; drug-target interaction networks (drugs on one side, proteins on other)

In Python, `networkx` is the standard library:

```python
import networkx as nx

# Directed weighted graph (metabolic network)
G = nx.DiGraph()
G.add_edge("glucose", "glucose-6-P", weight=1.0)
G.add_edge("glucose-6-P", "fructose-6-P", weight=1.0)
G.add_edge("glucose-6-P", "glucose-1-P", weight=0.5)
G.add_edge("fructose-6-P", "fructose-1,6-BP", weight=1.0)
```

## Breadth-First Search (BFS)

BFS explores vertices layer by layer from a source. It finds **shortest paths** in unweighted graphs (fewest edges).

**Algorithm**: Use a queue (FIFO). Mark vertices as visited when they are enqueued (not when processed) to avoid revisiting.

```python
from collections import deque

def bfs(G: nx.Graph, source) -> dict:
    """Return shortest path lengths from source to all reachable vertices."""
    dist = {source: 0}
    queue = deque([source])
    while queue:
        u = queue.popleft()
        for v in G.neighbors(u):
            if v not in dist:
                dist[v] = dist[u] + 1
                queue.append(v)
    return dist
```

**Time complexity**: $O(V + E)$

**Biological application**:
- Metabolic network reachability: from glucose, which metabolites can be produced? (BFS from glucose node)
- Protein interaction network diameter: maximum shortest path between any two proteins (breadth-first from all sources)
- Metagenomics assembly: BFS on de Bruijn graph to detect connected components (separate genomes in the graph)

## Depth-First Search (DFS) and Topological Sort

DFS explores as far as possible along each branch before backtracking. Key applications: connected components, cycle detection, topological sort.

```python
def dfs(G: nx.DiGraph, start) -> list:
    """DFS visit order using explicit stack."""
    visited, order = set(), []
    stack = [start]
    while stack:
        node = stack.pop()
        if node not in visited:
            visited.add(node)
            order.append(node)
            stack.extend(G.successors(node))
    return order
```

**Topological sort**: A linear ordering of vertices in a DAG (directed acyclic graph) such that for every directed edge $u \to v$, $u$ comes before $v$ in the ordering. Only possible for DAGs.

```python
# Khan's algorithm (BFS-based topological sort)
def topological_sort(G: nx.DiGraph) -> list:
    in_degree = dict(G.in_degree())
    queue = deque([v for v in G if in_degree[v] == 0])
    order = []
    while queue:
        u = queue.popleft()
        order.append(u)
        for v in G.successors(u):
            in_degree[v] -= 1
            if in_degree[v] == 0:
                queue.append(v)
    if len(order) != len(G):
        raise ValueError("Graph has a cycle — topological sort not possible")
    return order
```

**Biological application**:
- **Gene regulatory network analysis**: If gene A activates gene B which activates gene C, topological sort gives the temporal order of activation after a signal
- **Workflow scheduling**: Snakemake determines the order of rule execution from the DAG of input/output dependencies using topological sort
- **Pathway ordering**: Metabolic reactions can be topologically sorted to determine processing order in steady-state models

## Dijkstra's Algorithm: Shortest Paths in Weighted Graphs

**Dijkstra's algorithm** finds the shortest path from a source to all other vertices in a graph with non-negative edge weights. Uses a **priority queue** (min-heap):

```python
import heapq

def dijkstra(G: nx.DiGraph, source) -> tuple[dict, dict]:
    """Return (dist, pred) dicts: shortest distances and predecessors."""
    dist = {source: 0}
    pred = {source: None}
    pq = [(0, source)]  # (distance, node)
    
    while pq:
        d, u = heapq.heappop(pq)
        if d > dist.get(u, float('inf')):
            continue  # stale entry
        for v, data in G[u].items():
            w = data.get('weight', 1)
            new_dist = dist[u] + w
            if new_dist < dist.get(v, float('inf')):
                dist[v] = new_dist
                pred[v] = u
                heapq.heappush(pq, (new_dist, v))
    
    return dist, pred

def shortest_path(G, source, target):
    dist, pred = dijkstra(G, source)
    path, node = [], target
    while node is not None:
        path.append(node)
        node = pred.get(node)
    return list(reversed(path)), dist.get(target, float('inf'))
```

**Time complexity**: $O((V + E) \log V)$ with a binary heap.

**Biological application**:
- **Metabolic pathway finding**: Edge weights can be negative log-probabilities of each reaction (thermodynamic feasibility, enzyme presence) → Dijkstra finds the most probable/optimal pathway from substrate to product in KEGG
- **Drug target identification**: Shortest path in protein interaction network from a disease gene to potential drug targets (network proximity)
- **Sequence assembly**: Finding the optimal path through a string graph

**Worked example**: Find the shortest metabolic path from glucose to pyruvate in glycolysis network:

```python
glycolysis = nx.DiGraph()
edges = [
    ("glucose", "G6P", 1), ("G6P", "F6P", 1), ("F6P", "F16BP", 1),
    ("F16BP", "DHAP", 1), ("F16BP", "G3P", 1), ("DHAP", "G3P", 1),
    ("G3P", "1,3BPG", 1), ("1,3BPG", "3PG", 1), ("3PG", "2PG", 1),
    ("2PG", "PEP", 1), ("PEP", "pyruvate", 1)
]
glycolysis.add_weighted_edges_from(edges)
path, length = shortest_path(glycolysis, "glucose", "pyruvate")
# path: ['glucose', 'G6P', 'F6P', 'F16BP', 'G3P', '1,3BPG', '3PG', '2PG', 'PEP', 'pyruvate']
```

## Minimum Spanning Trees

A **minimum spanning tree (MST)** connects all vertices of a weighted undirected graph with minimum total edge weight. Two algorithms:
- **Kruskal's**: Sort edges by weight; add edge if it does not create a cycle (use Union-Find for cycle detection). $O(E \log E)$.
- **Prim's**: Grow the MST from a starting vertex; always add the minimum-weight edge from the current tree to a new vertex. $O(E \log V)$ with a heap.

**Biological application**:
- **Phylogenetic tree construction**: UPGMA/NJ are conceptually related (average-linkage clustering vs. minimum-distance neighbor joining)
- **Network clustering**: MST as a skeleton of a co-expression network — communities in the MST correspond to functional gene modules
- **Traveling salesman approximation**: 2-approximation algorithm for minimum distance to visit all microbiome sampling locations

## Network Motif Detection

Biological networks show statistically overrepresented **motifs** — small subgraph patterns that recur more than expected in random networks. The three-node feed-forward loop (FFL) appears ~1000× more than expected in *E. coli* transcriptional network.

```python
# Count all 3-node subgraphs (triads) in a network
# networkx has built-in motif functionality
motif_counts = nx.triangles(G)  # for undirected, count triangles per node

# For directed motifs, use triad_type_count
triad_census = nx.triadic_census(G)
```

**Coherent type-1 FFL**: X activates Y; X and Y both activate Z; common signal: AND logic. Delays the response to transient signals (Y must accumulate before Z activates). The functional consequence of this topology is that the target gene Z is only activated by persistent, sustained inputs — the network has implemented a noise filter using circuit structure alone, without any molecular mechanism specifically devoted to filtering. Graph topology is function.

## Why This Matters for Computational Biology

Graph algorithms are the mathematical language of biological networks. Genome assembly (de Bruijn graphs, Eulerian paths), pathway analysis (shortest paths in KEGG), network biology (centrality, community detection, motif analysis), and evolutionary inference (parsimony on phylogenetic trees, HGT networks) all reduce to graph problems. When you use networkx to analyze a co-expression network, find hub genes (highest degree), identify communities (Louvain, Leiden algorithms), or trace information flow (betweenness centrality), you are applying graph algorithms. Understanding BFS/DFS at the algorithmic level means you can implement custom traversals for domain-specific problems — when no existing tool covers your specific graph query, you write it from scratch using these primitives.
