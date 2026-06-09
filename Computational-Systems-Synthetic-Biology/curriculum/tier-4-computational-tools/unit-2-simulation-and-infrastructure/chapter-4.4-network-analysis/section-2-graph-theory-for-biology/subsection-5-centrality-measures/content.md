# Centrality Measures

Consider two proteins, each with exactly ten interaction partners. Are they equally important in the network? Not necessarily. The first protein might interact with ten members of the same pathway, all of which also interact with each other — it is embedded in a tight cluster, redundant and peripheral. The second protein might be the only connection between two otherwise separate functional modules — a molecular bridge whose removal would sever communication between a receptor and its downstream effectors. Same degree; completely different functional role. Degree counts connections. Centrality asks a more subtle question: given the network's global structure, how strategically positioned is this node? The answer differs depending on what you mean by "strategic" — which is why there are multiple centrality measures, each illuminating a different aspect of importance.

Centrality quantifies the importance of individual nodes in a network. While degree captures local connectivity, centrality measures capture different aspects of a node's position in global network topology — how many paths it lies on, how closely connected it is to all other nodes, or how important its neighbors are. Different centrality metrics identify different types of biologically important nodes.

## Betweenness Centrality

**Betweenness centrality** $C_B(v)$ measures the fraction of all shortest paths in the network that pass through node $v$:

$$C_B(v) = \sum_{s \neq v \neq t \in V} \frac{\sigma(s, t | v)}{\sigma(s, t)}$$

where $\sigma(s,t)$ is the total number of shortest paths from $s$ to $t$ and $\sigma(s,t|v)$ is the number passing through $v$. Normalized by $(n-1)(n-2)/2$ for undirected graphs.

**Biological interpretation**: a node with high betweenness lies on many communication pathways between other nodes. Removing it would disconnect or severely lengthen many paths — it is a **bottleneck**. In biological networks:
- High betweenness proteins often lie at pathway crossroads (kinases, scaffold proteins)
- High betweenness metabolites are currency metabolites (ATP, NADH) in metabolic networks
- High betweenness genes in GRNs are master regulators

**Algorithm**: Brandes algorithm computes betweenness in $O(VE)$ for unweighted graphs (vs. naive $O(V^3)$).

```python
import networkx as nx
import numpy as np
import matplotlib.pyplot as plt

G = nx.karate_club_graph()  # proxy for PPI

# Betweenness centrality
betweenness = nx.betweenness_centrality(G, normalized=True)
# For large networks: approximate with k samples
# betweenness = nx.betweenness_centrality(G, k=100)  # k random source nodes

# Top betweenness nodes
top_bc = sorted(betweenness.items(), key=lambda x: -x[1])[:5]
print("Top 5 nodes by betweenness centrality:")
for node, bc in top_bc:
    print(f"  Node {node}: C_B = {bc:.4f}, degree = {G.degree(node)}")
```

## Closeness Centrality

**Closeness centrality** $C_C(v)$ measures how close a node is to all other nodes on average:

$$C_C(v) = \frac{n-1}{\sum_{u \neq v} d(v,u)}$$

A high closeness centrality means that signals or information from $v$ reach all other nodes quickly. In disconnected graphs, closeness is typically defined over the connected component.

**Biological interpretation**: nodes with high closeness can rapidly broadcast signals to all parts of the network. Master transcriptional regulators often have high closeness centrality in GRNs.

```python
closeness = nx.closeness_centrality(G)
top_cc = sorted(closeness.items(), key=lambda x: -x[1])[:5]
print("\nTop 5 nodes by closeness centrality:")
for node, cc in top_cc:
    print(f"  Node {node}: C_C = {cc:.4f}")
```

## Eigenvector Centrality

**Eigenvector centrality** $x_i$ assigns importance to a node based on the importance of its neighbors. A node is important if it is connected to other important nodes:

$$x_i = \frac{1}{\lambda} \sum_{j \in N(i)} x_j = \frac{1}{\lambda} \sum_j A_{ij} x_j$$

In matrix form: $A\mathbf{x} = \lambda\mathbf{x}$. The centrality vector $\mathbf{x}$ is the eigenvector corresponding to the largest eigenvalue $\lambda_\text{max}$ of the adjacency matrix $A$.

**Biological interpretation**: proteins connected to other hub proteins rank highly — they are embedded in the functional core of the network. Eigenvector centrality corresponds to the steady-state probability of a random walker on the network.

```python
eigenvector = nx.eigenvector_centrality(G, max_iter=1000)
top_ec = sorted(eigenvector.items(), key=lambda x: -x[1])[:5]
print("\nTop 5 nodes by eigenvector centrality:")
for node, ec in top_ec:
    print(f"  Node {node}: EC = {ec:.4f}")
```

## PageRank

**PageRank** (Brin & Page, 1998) extends eigenvector centrality by including a teleportation term — a random walker follows edges with probability $\alpha$ and teleports to a random node with probability $1-\alpha$:

$$\text{PR}(v) = \frac{1-\alpha}{n} + \alpha \sum_{u \in N(v)} \frac{\text{PR}(u)}{k_u^\text{out}}$$

For biological applications, $\alpha = 0.85$ is standard. PageRank is particularly useful for **directed networks** (regulatory networks, signal transduction) where in-links (being regulated by or activated by many important proteins) confer importance.

```python
pagerank = nx.pagerank(G, alpha=0.85)
top_pr = sorted(pagerank.items(), key=lambda x: -x[1])[:5]
print("\nTop 5 nodes by PageRank:")
for node, pr in top_pr:
    print(f"  Node {node}: PR = {pr:.4f}")
```

## Brandes Algorithm: Efficient Betweenness Computation

For large networks, naive betweenness computation is prohibitive. The Brandes algorithm uses accumulation to compute all-pairs betweenness in $O(VE)$:

```python
def brandes_betweenness(G, normalized=True):
    """
    Brandes algorithm for betweenness centrality.
    Time: O(VE) for unweighted; O(VE + V^2 log V) for weighted.
    """
    from collections import deque, defaultdict

    betweenness = {v: 0.0 for v in G}

    for s in G.nodes():
        # BFS
        stack = []
        pred = {w: [] for w in G}
        sigma = {w: 0.0 for w in G}
        sigma[s] = 1.0
        dist = {w: -1 for w in G}
        dist[s] = 0
        Q = deque([s])

        while Q:
            v = Q.popleft()
            stack.append(v)
            for w in G.neighbors(v):
                if dist[w] < 0:
                    Q.append(w)
                    dist[w] = dist[v] + 1
                if dist[w] == dist[v] + 1:
                    sigma[w] += sigma[v]
                    pred[w].append(v)

        # Accumulation (back-propagation)
        delta = {v: 0.0 for v in G}
        while stack:
            w = stack.pop()
            for v in pred[w]:
                delta[v] += sigma[v] / sigma[w] * (1 + delta[w])
            if w != s:
                betweenness[w] += delta[w]

    if normalized:
        n = G.number_of_nodes()
        scale = 2.0 / ((n-1) * (n-2)) if n > 2 else 1.0
        for v in betweenness:
            betweenness[v] *= scale

    return betweenness

# Verify against NetworkX
bc_manual = brandes_betweenness(G)
bc_nx = nx.betweenness_centrality(G, normalized=True)
max_error = max(abs(bc_manual[v] - bc_nx[v]) for v in G)
print(f"\nBrandes verification: max error vs NetworkX = {max_error:.2e}")
```

## Comparing Centrality Measures

Different centrality metrics identify different biologically relevant nodes:

```python
def centrality_comparison(G):
    """Compute all centrality measures and find which nodes they rank differently."""
    degree_c    = nx.degree_centrality(G)
    betweenness = nx.betweenness_centrality(G, normalized=True)
    closeness   = nx.closeness_centrality(G)
    eigenvector = nx.eigenvector_centrality(G, max_iter=1000)
    pagerank    = nx.pagerank(G, alpha=0.85)

    import pandas as pd
    df = pd.DataFrame({
        "Degree":      degree_c,
        "Betweenness": betweenness,
        "Closeness":   closeness,
        "Eigenvector": eigenvector,
        "PageRank":    pagerank
    })
    # Rank each metric (1 = most central)
    rank_df = df.rank(ascending=False, method="min")

    print("Centrality ranks for top nodes by degree:")
    top_degree_nodes = df["Degree"].nlargest(5).index
    print(rank_df.loc[top_degree_nodes].to_string(float_format=lambda x: f"{x:.0f}"))

    # Correlation between metrics
    print("\nCorrelation between centrality metrics:")
    print(df.corr().to_string(float_format=lambda x: f"{x:.2f}"))

    # Discrepant nodes: high betweenness, moderate degree (bridge hubs)
    df["BC_rank"] = rank_df["Betweenness"]
    df["Deg_rank"] = rank_df["Degree"]
    df["rank_diff"] = df["Deg_rank"] - df["BC_rank"]  # positive = higher BC than degree rank
    bridges = df.nlargest(3, "rank_diff")
    print(f"\nBridge nodes (high betweenness, moderate degree):")
    print(bridges[["Degree", "Betweenness", "rank_diff"]].to_string(float_format=lambda x: f"{x:.4f}"))

    return df

centrality_df = centrality_comparison(G)
```

## Visualization: Centrality as Node Size

```python
def visualize_centrality(G, centrality_dict, title="Centrality visualization"):
    """Visualize network with node size proportional to centrality."""
    pos = nx.spring_layout(G, seed=42, k=2.0)

    # Scale node sizes
    values = np.array(list(centrality_dict.values()))
    sizes = 100 + 2000 * (values - values.min()) / (values.max() - values.min() + 1e-10)

    fig, ax = plt.subplots(figsize=(10, 8))
    nx.draw_networkx_edges(G, pos, alpha=0.3, ax=ax)
    scatter = nx.draw_networkx_nodes(G, pos, node_size=sizes,
                                      node_color=values, cmap="Reds",
                                      ax=ax, alpha=0.8)
    nx.draw_networkx_labels(G, pos, font_size=7, ax=ax)
    plt.colorbar(scatter, ax=ax, label="Centrality value")
    ax.set_title(title)
    ax.axis("off")
    plt.tight_layout()

visualize_centrality(G, betweenness, "Betweenness centrality")
```

## Why This Matters

Centrality analysis translates network topology into biological priority lists. In drug target identification, proteins with high betweenness centrality in a disease subnetwork are candidates for disruption — they are bottlenecks through which disease signals flow. In synthetic biology, genes with high eigenvector centrality in a regulatory network are likely to have broad effects when perturbed, making them sensitive points for circuit engineering. Betweenness and eigenvector centrality often identify different proteins than raw degree: a kinase that connects two otherwise separate signaling pathways may have low degree but the highest betweenness in the network. Missing this distinction — treating degree as a proxy for all centrality — is a common analytical error that overlooks the most functionally important bridging nodes.
