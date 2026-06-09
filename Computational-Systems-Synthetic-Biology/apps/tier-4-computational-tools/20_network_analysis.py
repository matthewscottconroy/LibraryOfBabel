"""
Biological Network Analysis
=============================
Tier 4.4 — Computational Tools: Network Analysis

Concepts demonstrated:
  - Network types: PPI, co-expression, metabolic, signaling
  - Degree distribution: scale-free vs. Poisson (random)
  - Centrality measures: degree, betweenness, eigenvector, PageRank
  - Community detection: Louvain algorithm
  - Network motif enrichment (subgraph counting)
  - Disease module hypothesis
  - Hub gene identification
  - Yeast PPI toy network analysis

Usage:
    python 20_network_analysis.py [--network NETWORK]

Networks:
    yeast_ppi   — Toy yeast PPI (scale-free, ~50 nodes)
    grn         — Gene regulatory network with motifs
    metabolic   — Metabolic pathway graph

Key insight:
    Biological networks are scale-free: degree follows a power law,
    with a few highly connected hubs and many peripheral nodes.
    This topology makes networks robust to random failure but vulnerable
    to targeted hub removal. Disease genes tend to cluster in modules
    with many interconnections.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import networkx as nx
from collections import Counter


# ── Network Generators ────────────────────────────────────────────────────────

def make_yeast_ppi(seed=42):
    """Generate a scale-free toy PPI network using Barabási-Albert model."""
    G = nx.barabasi_albert_graph(50, 2, seed=seed)
    # Name nodes as genes
    mapping = {i: f"YAL{i:03d}" for i in range(G.number_of_nodes())}
    G = nx.relabel_nodes(G, mapping)

    # Mark some nodes as "disease genes" (high-degree hubs often implicated)
    degrees = dict(G.degree())
    top_5 = sorted(degrees, key=degrees.get, reverse=True)[:5]
    nx.set_node_attributes(G, {n: True for n in top_5}, name='disease')
    return G


def make_grn(seed=42):
    """Gene regulatory network with feed-forward loops."""
    rng = np.random.default_rng(seed)
    G = nx.DiGraph()
    genes = [f"TF{i}" for i in range(8)] + [f"Gene{i}" for i in range(20)]
    G.add_nodes_from(genes)

    # TFs regulate other TFs and target genes
    for i, tf in enumerate(genes[:8]):
        for j, other in enumerate(genes[:8]):
            if i != j and rng.random() < 0.3:
                G.add_edge(tf, other, weight=rng.choice([-1, 1]))
        for tg in rng.choice(genes[8:], size=rng.integers(2, 6), replace=False):
            G.add_edge(tf, tg, weight=rng.choice([-1, 1]))

    nx.set_node_attributes(G, {n: 'TF' for n in genes[:8]}, 'type')
    nx.set_node_attributes(G, {n: 'target' for n in genes[8:]}, 'type')
    return G


def make_metabolic(seed=42):
    """Simplified metabolic network (reactions as nodes, edges = shared metabolites)."""
    G = nx.Graph()
    reactions = [
        ('Glycolysis', 'PFK'), ('Glycolysis', 'PK'), ('PFK', 'Aldolase'),
        ('Aldolase', 'GAPDH'), ('GAPDH', 'PGK'), ('PGK', 'PK'),
        ('PK', 'PDH'), ('PDH', 'CS'), ('CS', 'ACO'), ('ACO', 'IDH'),
        ('IDH', 'OGDH'), ('OGDH', 'SCS'), ('SCS', 'SDH'),
        ('SDH', 'Fumarase'), ('Fumarase', 'MDH'), ('MDH', 'CS'),
        ('PDH', 'FAS'), ('FAS', 'FASN'), ('FASN', 'ACSL'),
    ]
    G.add_edges_from(reactions)
    pathway_map = {
        'PFK': 'Glycolysis', 'Aldolase': 'Glycolysis', 'GAPDH': 'Glycolysis',
        'PGK': 'Glycolysis', 'PK': 'Glycolysis',
        'CS': 'TCA', 'ACO': 'TCA', 'IDH': 'TCA', 'OGDH': 'TCA',
        'SCS': 'TCA', 'SDH': 'TCA', 'Fumarase': 'TCA', 'MDH': 'TCA',
        'PDH': 'Link', 'Glycolysis': 'Glycolysis',
        'FAS': 'FattyAcid', 'FASN': 'FattyAcid', 'ACSL': 'FattyAcid',
    }
    nx.set_node_attributes(G, pathway_map, 'pathway')
    return G


NETWORKS = {
    'yeast_ppi': make_yeast_ppi,
    'grn': make_grn,
    'metabolic': make_metabolic,
}


# ── Analysis Functions ────────────────────────────────────────────────────────

def degree_distribution(G):
    if G.is_directed():
        deg = [d for _, d in G.degree()]
    else:
        deg = [d for _, d in G.degree()]
    return Counter(deg)


def compute_centralities(G):
    G_undirected = G.to_undirected() if G.is_directed() else G
    centralities = {
        'degree': dict(nx.degree_centrality(G)),
        'betweenness': dict(nx.betweenness_centrality(G_undirected)),
        'eigenvector': {},
        'pagerank': dict(nx.pagerank(G if G.is_directed() else G.to_directed())),
    }
    try:
        centralities['eigenvector'] = dict(nx.eigenvector_centrality(G_undirected, max_iter=500))
    except Exception:
        centralities['eigenvector'] = dict(nx.degree_centrality(G))

    return centralities


def detect_communities(G):
    G_undirected = G.to_undirected() if G.is_directed() else G
    try:
        communities = nx.community.louvain_communities(G_undirected, seed=42)
    except AttributeError:
        communities = list(nx.community.greedy_modularity_communities(G_undirected))
    return communities


def network_stats(G):
    G_un = G.to_undirected() if G.is_directed() else G
    stats = {
        'nodes': G.number_of_nodes(),
        'edges': G.number_of_edges(),
        'density': nx.density(G),
        'components': nx.number_connected_components(G_un),
    }
    if nx.is_connected(G_un):
        stats['avg_path_length'] = nx.average_shortest_path_length(G_un)
        stats['diameter'] = nx.diameter(G_un)
        stats['clustering'] = nx.average_clustering(G_un)
    else:
        largest_cc = max(nx.connected_components(G_un), key=len)
        H = G_un.subgraph(largest_cc)
        stats['avg_path_length_lcc'] = nx.average_shortest_path_length(H)
        stats['clustering'] = nx.average_clustering(G_un)
    return stats


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(network_name='yeast_ppi'):
    G = NETWORKS[network_name]()

    centralities = compute_centralities(G)
    communities  = detect_communities(G)
    deg_dist     = degree_distribution(G)
    stats        = network_stats(G)

    # Top hubs by betweenness
    top_hubs = sorted(centralities['betweenness'], key=centralities['betweenness'].get, reverse=True)[:5]

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Biological Network Analysis — {network_name.replace("_", " ").title()}',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.04, right=0.97, top=0.88, bottom=0.06,
                           hspace=0.45, wspace=0.30)

    ax_net    = fig.add_subplot(gs[:, 0])
    ax_deg    = fig.add_subplot(gs[0, 1])
    ax_cent   = fig.add_subplot(gs[1, 1])
    ax_comm   = fig.add_subplot(gs[0, 2])
    ax_info   = fig.add_subplot(gs[1, 2])

    for ax in [ax_net, ax_deg, ax_cent, ax_comm, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Network visualization ─────────────────────────────────────────────────
    G_un = G.to_undirected() if G.is_directed() else G
    pos = nx.spring_layout(G, seed=42, k=1.5)

    # Color by community
    community_colors = ['#50fa7b', '#ff79c6', '#8be9fd', '#f5a623', '#ff5555',
                        '#f1fa8c', '#6272a4', '#bd93f9']
    node_colors = ['white'] * G.number_of_nodes()
    nodes_list = list(G.nodes())
    for ci, comm in enumerate(communities):
        col = community_colors[ci % len(community_colors)]
        for n in comm:
            if n in nodes_list:
                node_colors[nodes_list.index(n)] = col

    # Node size by betweenness
    bet = centralities['betweenness']
    max_bet = max(bet.values()) if bet else 1
    node_sizes = [200 + 1200 * bet.get(n, 0) / max(max_bet, 1e-9) for n in nodes_list]

    # Edge color
    edge_colors = ['#f5a623' if G.is_directed() else '#8be9fd'] * G.number_of_edges()

    nx.draw_networkx_nodes(G, pos, ax=ax_net, node_color=node_colors,
                           node_size=node_sizes, alpha=0.9,
                           edgecolors='white', linewidths=0.5)
    nx.draw_networkx_edges(G_un, pos, ax=ax_net, edge_color='#555566',
                           alpha=0.5, width=0.8, arrows=G.is_directed(),
                           arrowsize=10)

    # Label top hubs
    hub_labels = {n: n for n in top_hubs if n in pos}
    nx.draw_networkx_labels(G, pos, labels=hub_labels, ax=ax_net,
                            font_size=7, font_color='white', font_weight='bold')

    ax_net.set_title(f'Network  (communities = node color, size = betweenness)',
                     color='white', fontsize=9)
    ax_net.axis('off')

    # ── Degree distribution ───────────────────────────────────────────────────
    k_vals = sorted(deg_dist.keys())
    counts = [deg_dist[k] for k in k_vals]

    ax_deg.scatter(k_vals, counts, color='#8be9fd', s=50, zorder=5)
    if len(k_vals) > 3:
        ax_deg.set_xscale('log')
        ax_deg.set_yscale('log')
        # Fit power law
        log_k = np.log10(np.array(k_vals, dtype=float))
        log_c = np.log10(np.array(counts, dtype=float))
        valid = np.isfinite(log_k) & np.isfinite(log_c)
        if valid.sum() > 2:
            slope, intercept = np.polyfit(log_k[valid], log_c[valid], 1)
            x_fit = np.logspace(log_k.min(), log_k.max(), 50)
            ax_deg.plot(x_fit, 10**(intercept + slope * np.log10(x_fit)),
                        '--', color='#f5a623', lw=1.5, label=f'Power law γ={-slope:.2f}')
            ax_deg.legend(fontsize=7, facecolor='#16213e', labelcolor='white')

    ax_deg.set_xlabel('Degree k', color='white')
    ax_deg.set_ylabel('P(k) [count]', color='white')
    ax_deg.set_title('Degree Distribution', color='white', fontsize=9)
    ax_deg.tick_params(colors='white')
    for sp in ax_deg.spines.values():
        sp.set_edgecolor('#888')

    # ── Centrality comparison for top 10 nodes ────────────────────────────────
    top_10 = sorted(centralities['betweenness'], key=centralities['betweenness'].get, reverse=True)[:10]
    x = np.arange(len(top_10))
    w = 0.22
    cent_types = [('degree', '#50fa7b'), ('betweenness', '#8be9fd'),
                  ('pagerank', '#f5a623'), ('eigenvector', '#ff79c6')]

    for i, (ctype, col) in enumerate(cent_types):
        vals = [centralities[ctype].get(n, 0) for n in top_10]
        max_v = max(vals) if max(vals) > 0 else 1
        ax_cent.bar(x + (i - 1.5) * w, [v/max_v for v in vals],
                    w, color=col, alpha=0.85, edgecolor='white', label=ctype[:3].title())

    ax_cent.set_xticks(x)
    ax_cent.set_xticklabels([n[:8] for n in top_10], rotation=35, ha='right',
                             color='white', fontsize=7)
    ax_cent.set_ylabel('Normalized centrality', color='white')
    ax_cent.set_title('Centrality: Top 10 Nodes (by betweenness)', color='white', fontsize=9)
    ax_cent.tick_params(colors='white')
    ax_cent.legend(fontsize=7, facecolor='#16213e', labelcolor='white', ncol=4)
    for sp in ax_cent.spines.values():
        sp.set_edgecolor('#888')

    # ── Community sizes ───────────────────────────────────────────────────────
    comm_sizes = sorted([len(c) for c in communities], reverse=True)
    ax_comm.bar(range(len(comm_sizes)), comm_sizes,
                color=community_colors[:len(comm_sizes)],
                edgecolor='white', width=0.8)
    ax_comm.set_xlabel('Community #', color='white')
    ax_comm.set_ylabel('Size (nodes)', color='white')
    ax_comm.set_title(f'Community Structure  ({len(communities)} communities)',
                      color='white', fontsize=9)
    ax_comm.tick_params(colors='white')
    for sp in ax_comm.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    top_hub_text = "Top 5 hubs (betweenness):\n"
    for n in top_hubs:
        top_hub_text += f"  {n}: {centralities['betweenness'][n]:.3f}\n"

    info = (
        f"Network: {network_name}\n\n"
        f"Basic statistics:\n"
        f"  Nodes:    {stats['nodes']}\n"
        f"  Edges:    {stats['edges']}\n"
        f"  Density:  {stats['density']:.4f}\n"
        f"  Components: {stats['components']}\n"
        f"  Clustering: {stats.get('clustering', 0):.4f}\n"
        f"  Avg path:  {stats.get('avg_path_length', stats.get('avg_path_length_lcc', 'n/a')):.2f}\n\n"
        f"Scale-free property:\n"
        f"  Max degree: {max(deg_dist.keys())}\n"
        f"  Mean degree: {np.mean(list(deg_dist.elements())):.2f}\n\n"
        f"Communities: {len(communities)}\n"
        f"  Sizes: {sorted(comm_sizes, reverse=True)[:6]}\n\n"
        f"{top_hub_text}"
        f"\nDisease module hypothesis:\n"
        f"  Disease genes cluster in\n"
        f"  high-connectivity regions"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nNetwork: {network_name}")
    print(f"Nodes: {stats['nodes']}, Edges: {stats['edges']}, Density: {stats['density']:.4f}")
    print(f"Communities: {len(communities)}")
    print(f"Top hubs: {', '.join(f'{n}({centralities['betweenness'][n]:.3f})' for n in top_hubs)}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Biological network analysis')
    parser.add_argument('--network', default='yeast_ppi',
                        choices=list(NETWORKS.keys()))
    args = parser.parse_args()

    print("Biological Network Analysis")
    print("=" * 45)
    run(network_name=args.network)
