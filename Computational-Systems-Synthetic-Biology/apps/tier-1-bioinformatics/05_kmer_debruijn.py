"""
k-mer Spectrum & de Bruijn Graph
=================================
Tier 1.1 — Bioinformatics: Sequence Assembly

Concepts demonstrated:
  - k-mer decomposition of sequences
  - k-mer frequency histogram (genome size estimation)
  - de Bruijn graph: nodes=(k-1)-mers, edges=k-mers
  - Eulerian path as genome assembly
  - Why repeats cause assembly ambiguity
  - Coverage and sequencing depth

Usage:
    python 05_kmer_debruijn.py [--seq SEQUENCE] [--k K]

Key insight:
    Genome assemblers solve the problem of reconstructing a sequence from
    overlapping short reads by building a de Bruijn graph and finding an
    Eulerian path — which can be done in polynomial time (unlike the
    Hamiltonian path problem that earlier overlap graphs required).
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import networkx as nx
from collections import Counter


# ── k-mer Functions ───────────────────────────────────────────────────────────

def get_kmers(sequence, k):
    """Return all k-mers (substrings of length k) in order."""
    return [sequence[i:i+k] for i in range(len(sequence) - k + 1)]


def kmer_spectrum(sequence, k):
    """Return Counter of k-mer frequencies."""
    return Counter(get_kmers(sequence, k))


def build_debruijn_graph(sequence, k):
    """
    Build directed de Bruijn graph.
    Nodes: (k-1)-mers
    Edges: k-mers (each k-mer connects its left (k-1)-mer to its right (k-1)-mer)
    """
    G = nx.MultiDiGraph()
    kmers = get_kmers(sequence, k)
    edge_counts = Counter(kmers)

    for kmer, count in edge_counts.items():
        left  = kmer[:-1]
        right = kmer[1:]
        for _ in range(count):
            G.add_edge(left, right, label=kmer)

    return G, kmers


def simulate_reads(sequence, read_length, coverage, seed=42):
    """
    Simulate short read sequencing.
    Returns list of reads at the given coverage.
    """
    rng = np.random.default_rng(seed)
    n_reads = int(coverage * len(sequence) / read_length)
    reads = []
    for _ in range(n_reads):
        start = rng.integers(0, max(1, len(sequence) - read_length))
        reads.append(sequence[start:start + read_length])
    return reads


def reads_to_debruijn(reads, k):
    """Build de Bruijn graph from a collection of reads."""
    G = nx.MultiDiGraph()
    all_kmers = []
    for read in reads:
        kmers = get_kmers(read, k)
        all_kmers.extend(kmers)
    edge_counts = Counter(all_kmers)
    for kmer, count in edge_counts.items():
        left, right = kmer[:-1], kmer[1:]
        G.add_edge(left, right, weight=count, label=kmer)
    return G


def find_eulerian_path(G):
    """
    Attempt to find an Eulerian path (visits every edge exactly once).
    Conditions: at most 2 nodes with odd degree, graph connected.
    """
    UG = G.to_undirected()
    if not nx.is_connected(UG):
        return None, "Graph not connected — likely caused by low coverage or long repeats."

    in_deg  = dict(G.in_degree())
    out_deg = dict(G.out_degree())
    start_nodes = [n for n in G.nodes if out_deg.get(n, 0) - in_deg.get(n, 0) == 1]
    end_nodes   = [n for n in G.nodes if in_deg.get(n, 0) - out_deg.get(n, 0) == 1]

    if len(start_nodes) == 0 and len(end_nodes) == 0:
        # Eulerian circuit
        start = list(G.nodes)[0]
    elif len(start_nodes) == 1:
        start = start_nodes[0]
    else:
        return None, f"No Eulerian path: {len(start_nodes)} semi-balanced start nodes."

    try:
        path = list(nx.eulerian_path(G, source=start))
        assembled = path[0][0] + ''.join(edge[1][-1] for edge in path)
        return assembled, f"Eulerian path found. Assembled: {assembled}"
    except Exception as e:
        return None, f"Eulerian path failed: {e}"


# ── Visualisation ─────────────────────────────────────────────────────────────

def draw_debruijn(G, ax, title, max_nodes=30):
    ax.cla()
    ax.set_facecolor('#0f3460')

    if G.number_of_nodes() > max_nodes:
        ax.text(0.5, 0.5,
                f"Graph too large to display ({G.number_of_nodes()} nodes).\n"
                "Shown in summary only.",
                transform=ax.transAxes, ha='center', va='center',
                color='white', fontsize=10)
        ax.set_title(title, color='white')
        return

    pos = nx.spring_layout(G, seed=42, k=2.5)

    # Colour nodes by in/out degree balance
    in_deg  = dict(G.in_degree())
    out_deg = dict(G.out_degree())
    node_colors = []
    for n in G.nodes:
        diff = out_deg.get(n, 0) - in_deg.get(n, 0)
        if diff > 0:
            node_colors.append('#50fa7b')   # source-like
        elif diff < 0:
            node_colors.append('#ff79c6')   # sink-like
        else:
            node_colors.append('#8be9fd')   # balanced

    nx.draw_networkx_nodes(G, pos, ax=ax, node_color=node_colors,
                           node_size=900, edgecolors='white', linewidths=1.0)
    nx.draw_networkx_labels(G, pos, ax=ax, font_size=7,
                            font_color='#1a1a2e', font_weight='bold')

    # Draw edges (curved for multi-edges)
    edge_labels = {}
    for u, v, data in G.edges(data=True):
        nx.draw_networkx_edges(G, pos, edgelist=[(u, v)], ax=ax,
                               edge_color='#f5a623', arrows=True,
                               arrowsize=20, width=1.5, alpha=0.7,
                               connectionstyle='arc3,rad=0.15')
        edge_labels[(u, v)] = data.get('label', '')

    ax.set_title(title, color='white', fontsize=9)
    ax.axis('off')

    # Legend
    from matplotlib.lines import Line2D
    legend_elements = [
        Line2D([0], [0], marker='o', color='w', markerfacecolor='#50fa7b',
               markersize=9, label='Out > In (path start)'),
        Line2D([0], [0], marker='o', color='w', markerfacecolor='#ff79c6',
               markersize=9, label='In > Out (path end)'),
        Line2D([0], [0], marker='o', color='w', markerfacecolor='#8be9fd',
               markersize=9, label='Balanced'),
    ]
    ax.legend(handles=legend_elements, loc='lower right',
              facecolor='#16213e', labelcolor='white', fontsize=7)


def run(sequence, k):
    # Compute everything
    spectrum = kmer_spectrum(sequence, k)
    G_seq, kmers = build_debruijn_graph(sequence, k)
    assembled, msg = find_eulerian_path(G_seq)

    # Simulate reads
    reads = simulate_reads(sequence, read_length=max(k+2, 6), coverage=4)
    G_reads = reads_to_debruijn(reads, k)

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'k-mer Spectrum & de Bruijn Graph  (k={k}, seq="{sequence}")',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.04, right=0.97, top=0.88, bottom=0.06,
                           hspace=0.45, wspace=0.3)

    ax_spec   = fig.add_subplot(gs[0, 0])   # k-mer frequency histogram
    ax_dbg    = fig.add_subplot(gs[:, 1])   # de Bruijn graph (from sequence)
    ax_reads  = fig.add_subplot(gs[:, 2])   # de Bruijn graph (from reads)
    ax_info   = fig.add_subplot(gs[1, 0])   # info panel

    for ax in [ax_spec, ax_dbg, ax_reads, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── k-mer spectrum ────────────────────────────────────────────────────────
    freqs = list(spectrum.values())
    freq_counter = Counter(freqs)
    max_freq = max(freqs) + 1
    ax_spec.bar(freq_counter.keys(), freq_counter.values(),
                color='#50fa7b', edgecolor='white', width=0.7)
    ax_spec.set_xlabel('k-mer frequency', color='white')
    ax_spec.set_ylabel('Number of distinct k-mers', color='white')
    ax_spec.set_title(f'k-mer Frequency Spectrum\n(k={k}, {len(spectrum)} unique k-mers)',
                      color='white', fontsize=9)
    ax_spec.tick_params(colors='white')
    for sp in ax_spec.spines.values():
        sp.set_edgecolor('#888')

    # Annotate repeated k-mers
    repeated = [(km, c) for km, c in spectrum.items() if c > 1]
    if repeated:
        note = "Repeated k-mers:\n" + '\n'.join(f"  '{km}' ×{c}" for km, c in repeated[:5])
        ax_spec.text(0.97, 0.97, note, transform=ax_spec.transAxes,
                     ha='right', va='top', fontsize=7, color='#ff5555',
                     fontfamily='monospace',
                     bbox=dict(facecolor='#16213e', edgecolor='#ff5555', alpha=0.8))

    # ── de Bruijn graph from full sequence ────────────────────────────────────
    draw_debruijn(G_seq, ax_dbg,
                  f'de Bruijn Graph from sequence\n(nodes: {G_seq.number_of_nodes()}, '
                  f'edges: {G_seq.number_of_edges()})')

    # ── de Bruijn graph from simulated reads ─────────────────────────────────
    draw_debruijn(G_reads, ax_reads,
                  f'de Bruijn from simulated reads\n(coverage ≈4×, rl={max(k+2,6)})')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    all_kmers_str = ' '.join(get_kmers(sequence, k))
    info = (
        f"Sequence: {sequence}\n"
        f"Length: {len(sequence)} nt\n"
        f"k = {k}\n\n"
        f"Total k-mers: {len(kmers)}\n"
        f"Unique k-mers: {len(spectrum)}\n"
        f"Repeated: {len(repeated)}\n\n"
        f"Assembly attempt:\n"
        f"  {msg}\n\n"
        f"k-mers: {all_kmers_str[:80]}"
        + ("..." if len(all_kmers_str) > 80 else "")
        + "\n\nEulerian path condition:\n"
        f"  |out_deg - in_deg| ≤ 1 for all nodes\n"
        f"  → solvable in O(E) time (Hierholzer)"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.8, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    # Terminal summary
    print(f"\nk={k} | Sequence: {sequence}")
    print(f"Unique k-mers: {len(spectrum)}")
    print(f"Assembly: {msg}")
    top5 = spectrum.most_common(5)
    print("Top-5 k-mers by frequency:")
    for km, c in top5:
        print(f"  '{km}' × {c}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='k-mer & de Bruijn graph explorer')
    parser.add_argument('--seq', default='ATGCATGCATGCTAGCATGC',
                        help='Input sequence (default has repeats)')
    parser.add_argument('--k', type=int, default=4,
                        help='k-mer length (default: 4)')
    args = parser.parse_args()

    print("k-mer Spectrum & de Bruijn Graph")
    print("=" * 50)
    print(f"Sequence: {args.seq}")
    print(f"k = {args.k}")
    print()
    run(args.seq, args.k)
