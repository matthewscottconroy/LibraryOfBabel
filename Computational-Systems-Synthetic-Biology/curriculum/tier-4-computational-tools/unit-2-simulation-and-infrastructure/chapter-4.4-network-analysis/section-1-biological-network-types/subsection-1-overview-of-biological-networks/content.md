# Overview of Biological Networks

Knock out a single gene in *Saccharomyces cerevisiae* and most of the time the yeast survives. Knock out one of the ~20% of genes considered essential, and the cell dies. What makes a gene essential? Not just its own biochemical activity, but its position in a web of relationships — who it activates, who activates it, what it has in common with its molecular neighbors. The same biochemical function, embedded at different positions in the cellular interaction network, can be dispensable or indispensable. This is the central insight that launched network biology: you cannot understand a biological molecule in isolation any more than you can understand a word by knowing only its dictionary definition. Meaning emerges from context, and in molecular biology, context is a network.

**Network biology** provides a mathematical framework for representing, analyzing, and interpreting these relationships as graphs. This abstraction has proven remarkably powerful: topological properties of biological networks predict functional importance, disease relevance, and evolutionary constraints.

## The Graph Abstraction

A **graph** $G = (V, E)$ consists of a set of vertices (nodes) $V$ and edges $E \subseteq V \times V$. In biological networks:
- **Nodes** represent biological entities (genes, proteins, metabolites, taxa)
- **Edges** represent relationships (physical interaction, regulation, co-expression, metabolic transformation)

Graphs may be:
- **Directed** ($A \rightarrow B \neq B \rightarrow A$): regulatory relationships, signal transduction
- **Undirected** ($A - B = B - A$): physical binding, co-expression
- **Weighted** (edge weight = interaction confidence, correlation strength)
- **Signed** (positive/negative edges): activation/inhibition, positive/negative genetic interactions

The choice of representation is not cosmetic — it constrains what you can discover. A directed graph of transcription factor regulation can ask "what does this TF control?" An undirected co-expression network can ask "which genes are coordinately regulated?" Getting the abstraction right is the first step in any network analysis.

## Major Classes of Biological Networks

### Protein-Protein Interaction (PPI) Networks

Proteins interact physically to form complexes, modulate each other's activity, and transmit signals.

- **Nodes**: individual proteins (typically gene products)
- **Edges**: experimentally detected binary interactions or co-complex membership
- **Directionality**: usually undirected (mutual binding)
- **Scale**: human interactome has ~20,000 proteins and ~300,000–500,000 interactions (estimated)

Key databases: BioGRID (>2M interactions, multiple organisms), STRING (predicted + experimental, scored), IntAct (curated high-confidence), HuRI (comprehensive Y2H-based human binary interactome).

### Gene Regulatory Networks (GRNs)

Transcription factors bind promoters and enhancers to activate or repress gene expression.

- **Nodes**: transcription factors (TFs) and regulated genes
- **Edges**: directed regulatory relationship (TF → gene); signed (+/- for activation/repression)
- **Reconstruction**: from ChIP-seq data, motif scanning, reporter assays, or perturbation experiments

The *E. coli* regulatory network (~1.8k nodes, ~4k edges) is the best-characterized GRN; it contains specific recurring patterns (motifs) discussed in Section 4.

### Metabolic Networks

Enzymatic reactions transform metabolites within cells.

- **Nodes**: metabolites (substrates, products) and/or reactions
- **Edges**: metabolites connected if they participate in the same reaction
- **Representation**: commonly as a bipartite graph (metabolite and reaction nodes), or as the stoichiometric matrix $S \in \mathbb{R}^{m \times n}$ ($m$ metabolites, $n$ reactions)

*Recon3D* (the human metabolic reconstruction): ~4,140 metabolites, 10,600 reactions.

### Signaling Networks

Cascades of protein modifications (phosphorylation, ubiquitination) transmit signals from receptors to effectors.

- **Nodes**: proteins, protein complexes, second messengers
- **Edges**: directed; activation, inhibition, or complex formation
- **Crosstalk**: signaling pathways are highly interconnected — understanding network structure is essential for predicting drug effects

### Co-expression Networks

Genes with similar expression profiles across conditions are likely co-regulated or functionally related.

- **Nodes**: genes
- **Edges**: Pearson correlation or mutual information > threshold; weighted by correlation strength
- **Construction**: from RNA-seq or microarray data across many samples/conditions

### Genetic Interaction Networks

Two genes interact genetically if their combined perturbation (double mutant) has an effect different from the sum of individual perturbations.

$$\varepsilon = W_{AB} - W_A \cdot W_B$$

where $W$ is fitness. If $\varepsilon < 0$: negative (synthetic lethal); $\varepsilon > 0$: positive (buffering).

The Costanzo et al. (2010, 2016) global genetic interaction network in yeast mapped >23 million gene pairs.

## Loading and Examining Biological Networks

```python
import networkx as nx
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

def load_ppi_from_string(species_id=9606, score_threshold=700,
                          limit_n=None):
    """
    Download a PPI network from STRING API.
    species_id: 9606 = Homo sapiens
    score_threshold: combined score (0-1000); 700 = medium confidence
    """
    import requests
    url = "https://string-db.org/api/tsv/network"
    params = {
        "identifiers": "TP53%0dEGFR%0dBRCA1%0dMDM2%0dAKT1",  # seed proteins
        "species": species_id,
        "required_score": score_threshold,
        "network_type": "functional",
        "caller_identity": "bioinformatics_course"
    }
    response = requests.get(url, params=params)

    # Parse TSV
    from io import StringIO
    df = pd.read_csv(StringIO(response.text), sep="\t")
    print(f"Interactions from STRING: {len(df)}")

    G = nx.Graph()
    for _, row in df.iterrows():
        G.add_edge(row["preferredName_A"], row["preferredName_B"],
                   weight=row["score"])
    return G

def network_summary(G):
    """Print key topological statistics of a network."""
    n = G.number_of_nodes()
    m = G.number_of_edges()

    print(f"Network summary:")
    print(f"  Nodes: {n}")
    print(f"  Edges: {m}")
    print(f"  Density: {nx.density(G):.4f}")

    if nx.is_connected(G):
        print(f"  Diameter: {nx.diameter(G)}")
        print(f"  Average path length: {nx.average_shortest_path_length(G):.2f}")
    else:
        cc = max(nx.connected_components(G), key=len)
        G_lcc = G.subgraph(cc)
        print(f"  Largest connected component: {G_lcc.number_of_nodes()} nodes "
              f"({G_lcc.number_of_nodes()/n*100:.0f}%)")

    degrees = [d for _, d in G.degree()]
    print(f"  Mean degree: {np.mean(degrees):.1f}")
    print(f"  Max degree: {max(degrees)} (most connected hub)")
    print(f"  Average clustering: {nx.average_clustering(G):.3f}")

# Example: build a toy biological network
G = nx.Graph()
edges = [("TP53", "MDM2"), ("TP53", "CDKN1A"), ("TP53", "BAX"),
         ("MDM2", "HAUSP"), ("EGFR", "KRAS"), ("EGFR", "PIK3CA"),
         ("KRAS", "RAF1"), ("RAF1", "MAP2K1"), ("MAP2K1", "MAPK1"),
         ("PIK3CA", "AKT1"), ("AKT1", "MTOR"), ("MTOR", "S6K1"),
         ("TP53", "AKT1")]  # cross-pathway link
G.add_edges_from(edges)
network_summary(G)
```

## Visualizing Biological Networks

```python
def visualize_network(G, layout="spring", node_size_attr=None,
                      title="Biological network"):
    """
    Visualize a small-to-medium biological network.
    node_size_attr: node attribute to use for size (e.g., 'degree')
    """
    fig, ax = plt.subplots(figsize=(10, 8))

    # Layout
    if layout == "spring":
        pos = nx.spring_layout(G, seed=42, k=2.0)
    elif layout == "kamada_kawai":
        pos = nx.kamada_kawai_layout(G)
    elif layout == "circular":
        pos = nx.circular_layout(G)

    # Node sizes based on degree (hubs larger)
    degrees = dict(G.degree())
    if node_size_attr == "degree":
        sizes = [degrees[n] * 200 for n in G.nodes()]
    else:
        sizes = 500

    nx.draw_networkx_nodes(G, pos, node_size=sizes, node_color="steelblue",
                            alpha=0.8, ax=ax)
    nx.draw_networkx_edges(G, pos, alpha=0.4, width=1.0, ax=ax)
    nx.draw_networkx_labels(G, pos, font_size=8, ax=ax)
    ax.set_title(title)
    ax.axis("off")
    plt.tight_layout()
    return fig

fig = visualize_network(G, layout="spring", node_size_attr="degree",
                         title="TP53/EGFR signaling subnetwork")
```

## Why This Matters

Network biology transformed how we understand disease and drug targets. The observation that disease genes are not randomly distributed in interactomes — they cluster in network modules — provided a new framework for understanding complex diseases like cancer and neurodegeneration. The scale-free topology of PPI networks (a few highly connected hubs, many poorly connected nodes) explains why essential genes tend to encode hub proteins and why most random mutations are tolerated. More practically, network centrality metrics predict which proteins are likely drug targets and which genetic interactions can be exploited therapeutically. Every topic in the following sections builds on this network abstraction as a foundation for computational biology.
