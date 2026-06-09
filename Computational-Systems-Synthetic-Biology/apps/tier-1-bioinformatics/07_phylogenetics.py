"""
Phylogenetics: Distance Matrices & Tree Building
==================================================
Tier 1.6 — Bioinformatics: Phylogenetics

Concepts demonstrated:
  - Pairwise sequence distance (Hamming, Jukes-Cantor correction)
  - UPGMA hierarchical clustering → rooted tree
  - Neighbor-Joining (NJ) algorithm → unrooted tree
  - Newick format and tree display
  - Bootstrap support (simplified)
  - Molecular clock interpretation
  - Clade highlighting and tree comparison

Usage:
    python 07_phylogenetics.py [--demo DEMO]

Demos:
    0 — Cytochrome b mitochondrial gene fragments (mammals)
    1 — 16S rRNA fragments (bacteria)
    2 — Simulated sequences with known tree

Key insight:
    Distance-based methods are O(n²) in sequences but discard site
    information. Maximum likelihood uses all site data and is
    statistically consistent — it converges to the true tree as
    data grows. NJ is fast and often close to ML for molecular data.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.patches import FancyArrowPatch
from collections import defaultdict
import copy


# ── Demo Sequence Data ────────────────────────────────────────────────────────

DEMOS = {
    "Mammals (CytB)": {
        "sequences": {
            "Human":     "ATGACCCCAATACGCAAAA CTAACCCCCTAATAAAATT AATTAACCACTCATTCAT",
            "Chimp":     "ATGACCCCAATACGCAAAA CTAACCCCCTAATAAAATT AATTAACCACTCATTCAC",
            "Gorilla":   "ATGACCCCAATACGCAAAA CTAACCCCCTAATAAAATA AATTAACCACTCATTCAT",
            "Orangutan": "ATGACCCCAATACGCAAAC CTAACCCCCTAATAAAATT AATTAACCACTCATCCAT",
            "Gibbon":    "ATGACCCCAATACGCAAAC CTAACCCCCTAATAAAATT AATTAACCACTCATCCAT",
            "Macaque":   "ATGACCCCAATACGCAAAC CTAACCCCCTATCAAAATT AATTAACCACTCATCCAC",
            "Dog":       "ATGACCCCCATACGCAAAC CTAACCCCCTAATAAAATC AATTAACCACTCATCCGC",
            "Horse":     "ATGACCCCCATACGCAAAC CTAACCCCCTAATAAAATC AATTAACCACTCATCCGC",
        },
        "description": (
            "Cytochrome b fragments (simplified)\n"
            "Great apes share most similarity.\n"
            "Human-Chimp divergence ~6 Mya.\n"
            "Carnivores (Dog/Horse) form outgroup.\n\n"
            "Expected topology:\n"
            "((Human,Chimp),(Gorilla,(Orangutan,Gibbon)))\n"
            "  outgroup: (Dog,Horse)"
        ),
    },
    "Bacteria (16S)": {
        "sequences": {
            "E_coli":        "AGAGTTTGATCCTGGCTCAG GACGAACGCTGGCGGCGTGC",
            "S_aureus":      "AGAGTTTGATCCTGGCTCAG AACGAACGCTGGCGGCATGC",
            "B_subtilis":    "AGAGTTTGATCCTGGCTCAG AACGAACGCTGGCGGCATGC",
            "P_aeruginosa":  "AGAGTTTGATCCTGGCTCAG GACGAACGCTGGCGGCGTGC",
            "Salmonella":    "AGAGTTTGATCCTGGCTCAG GACGAACGCTGGCGGCGTGC",
            "Streptococcus": "AGAGTTTGATCCTGGCTCAG AACGAACGCTGGCGGCATGC",
            "Mycobacterium": "AGAGTTTGATCCTGGCTCAG AATGAACGCTGGCGGCACGC",
        },
        "description": (
            "16S rRNA fragments (simplified)\n"
            "Universal marker for bacterial phylogeny.\n"
            "Gram-negatives (E. coli, Pseudomonas,\n"
            "  Salmonella) cluster together.\n"
            "Gram-positives form separate clade.\n"
            "Mycobacterium is outgroup (Actinobacteria)."
        ),
    },
    "Simulated Tree": {
        "sequences": {
            "A": "ATCGATCGATCG",
            "B": "ATCGATCGATCC",
            "C": "ATCGATCGATAC",
            "D": "ATCGATCGCAAC",
            "E": "ATCGATCGCAAC",
            "F": "ATCGACCGCAGC",
            "G": "ATCGACCGCTGC",
        },
        "description": (
            "Simulated sequences with known topology:\n"
            "((A,B),(C,(D,E)),(F,G))\n\n"
            "A and B differ at 1 position → close.\n"
            "D and E are identical → sister taxa.\n"
            "F and G differ only at position 9.\n"
            "True tree should match simulation."
        ),
    },
}


# ── Distance Functions ────────────────────────────────────────────────────────

def clean_seq(seq):
    return seq.replace(' ', '').upper()


def hamming_distance(s1, s2):
    """Fraction of positions that differ (ignoring gaps)."""
    s1, s2 = clean_seq(s1), clean_seq(s2)
    n = min(len(s1), len(s2))
    diffs = sum(a != b for a, b in zip(s1[:n], s2[:n]))
    return diffs / n


def jukes_cantor(p):
    """Jukes-Cantor correction for multiple substitutions (nucleotides)."""
    if p >= 0.75:
        return 3.0  # saturated
    return -0.75 * np.log(1 - 4 * p / 3)


def build_distance_matrix(sequences):
    """Return names list and symmetric numpy distance matrix."""
    names = list(sequences.keys())
    n = len(names)
    D = np.zeros((n, n))
    for i in range(n):
        for j in range(i + 1, n):
            p = hamming_distance(sequences[names[i]], sequences[names[j]])
            d = jukes_cantor(p)
            D[i, j] = D[j, i] = d
    return names, D


# ── UPGMA ─────────────────────────────────────────────────────────────────────

class TreeNode:
    def __init__(self, name=None, left=None, right=None, dist=0.0):
        self.name = name
        self.left = left
        self.right = right
        self.dist = dist        # branch length to this node from parent
        self.height = 0.0       # height above leaves (for UPGMA)

    def is_leaf(self):
        return self.left is None and self.right is None

    def get_leaves(self):
        if self.is_leaf():
            return [self.name]
        return (self.left.get_leaves() if self.left else []) + \
               (self.right.get_leaves() if self.right else [])


def upgma(names, D):
    """UPGMA hierarchical clustering → TreeNode root."""
    D = D.copy()
    nodes = [TreeNode(name=n, height=0.0) for n in names]
    cluster_sizes = [1] * len(names)

    while len(nodes) > 1:
        n = len(nodes)
        # Find minimum off-diagonal
        min_d = np.inf
        i_min, j_min = 0, 1
        for i in range(n):
            for j in range(i + 1, n):
                if D[i, j] < min_d:
                    min_d = D[i, j]
                    i_min, j_min = i, j

        height = min_d / 2
        # Create new internal node
        li = nodes[i_min]
        lj = nodes[j_min]
        new_node = TreeNode(
            left=li, right=lj, height=height,
        )
        li.dist = height - li.height
        lj.dist = height - lj.height

        # Update distances (weighted average)
        si, sj = cluster_sizes[i_min], cluster_sizes[j_min]
        new_dists = []
        for k in range(n):
            if k != i_min and k != j_min:
                d_new = (si * D[i_min, k] + sj * D[j_min, k]) / (si + sj)
                new_dists.append(d_new)

        new_size = si + sj
        # Rebuild reduced matrix
        keep = [k for k in range(n) if k != i_min and k != j_min]
        new_n = len(keep) + 1
        new_D = np.zeros((new_n, new_n))
        for a, ka in enumerate(keep):
            for b, kb in enumerate(keep):
                new_D[a, b] = D[ka, kb]
            new_D[a, new_n - 1] = new_D[new_n - 1, a] = new_dists[a]

        nodes = [nodes[k] for k in keep] + [new_node]
        cluster_sizes = [cluster_sizes[k] for k in keep] + [new_size]
        D = new_D

    return nodes[0]


# ── Neighbor-Joining ──────────────────────────────────────────────────────────

def neighbor_joining(names, D):
    """Saitou-Nei neighbor-joining → unrooted tree as dict of adjacencies."""
    D = D.copy().tolist()
    names = list(names)
    adj = {n: [] for n in names}   # adjacency: name → [(neighbor, branch_len)]

    while len(names) > 3:
        n = len(names)
        # Q matrix
        r = [sum(D[i]) for i in range(n)]
        Q = [[0.0] * n for _ in range(n)]
        for i in range(n):
            for j in range(n):
                if i != j:
                    Q[i][j] = (n - 2) * D[i][j] - r[i] - r[j]

        # Find minimum Q
        i_min, j_min = 0, 1
        for i in range(n):
            for j in range(i + 1, n):
                if Q[i][j] < Q[i_min][j_min]:
                    i_min, j_min = i, j

        # Branch lengths
        d_ij = D[i_min][j_min]
        d_iu = 0.5 * d_ij + (r[i_min] - r[j_min]) / (2 * (n - 2))
        d_ju = d_ij - d_iu

        u = f"_{names[i_min]}_{names[j_min]}_"
        adj[u] = []
        adj[names[i_min]].append((u, max(0, d_iu)))
        adj[u].append((names[i_min], max(0, d_iu)))
        adj[names[j_min]].append((u, max(0, d_ju)))
        adj[u].append((names[j_min], max(0, d_ju)))

        # New distances
        new_D = []
        keep = [k for k in range(n) if k != i_min and k != j_min]
        for ki in keep:
            row = []
            for kj in keep:
                row.append(D[ki][kj])
            d_ku = 0.5 * (D[ki][i_min] + D[ki][j_min] - d_ij)
            row.append(max(0, d_ku))
            new_D.append(row)

        u_row = []
        for ki in keep:
            u_row.append(0.5 * (D[ki][i_min] + D[ki][j_min] - d_ij))
        u_row.append(0.0)
        new_D.append(u_row)

        names = [names[k] for k in keep] + [u]
        D = new_D

    # Connect final 3 nodes
    if len(names) == 3:
        a, b, c = names
        d_ab = D[0][1]; d_ac = D[0][2]; d_bc = D[1][2]
        d_au = 0.5 * (d_ab + d_ac - d_bc)
        d_bu = 0.5 * (d_ab + d_bc - d_ac)
        d_cu = 0.5 * (d_ac + d_bc - d_ab)
        u = f"_root_"
        adj[u] = []
        for name, dist in [(a, d_au), (b, d_bu), (c, d_cu)]:
            adj[name].append((u, max(0, dist)))
            adj[u].append((name, max(0, dist)))

    return adj


# ── Tree Drawing ──────────────────────────────────────────────────────────────

def _assign_coords(node, x, leaf_y, y_step):
    """Recursively assign (x, y) coords for rectangular cladogram."""
    if node.is_leaf():
        y = leaf_y[0]
        leaf_y[0] += y_step
        node._x = x
        node._y = y
        return y
    else:
        yl = _assign_coords(node.left, x + node.left.dist, leaf_y, y_step)
        yr = _assign_coords(node.right, x + node.right.dist, leaf_y, y_step)
        node._x = x
        node._y = (yl + yr) / 2
        return node._y


def draw_upgma_tree(ax, root, title):
    ax.cla()
    ax.set_facecolor('#0f3460')
    leaf_y = [0]
    _assign_coords(root, 0, leaf_y, 1.0)
    n_leaves = leaf_y[0]

    def draw_node(node):
        if node.is_leaf():
            ax.text(node._x + 0.02, node._y, node.name,
                    va='center', ha='left', fontsize=8,
                    color='#50fa7b', fontfamily='monospace')
            ax.plot(node._x, node._y, 'o', color='#50fa7b', ms=5, zorder=4)
            return
        # Horizontal lines to children
        ax.plot([node._x, node.left._x], [node.left._y, node.left._y],
                '-', color='#8be9fd', lw=1.5)
        ax.plot([node._x, node.right._x], [node.right._y, node.right._y],
                '-', color='#8be9fd', lw=1.5)
        # Vertical connecting line
        ax.plot([node._x, node._x], [node.left._y, node.right._y],
                '-', color='#8be9fd', lw=1.5)
        ax.plot(node._x, node._y, 's', color='#f5a623', ms=5, zorder=4)
        draw_node(node.left)
        draw_node(node.right)

    draw_node(root)
    ax.set_xlim(-0.02, None)
    ax.set_ylim(-0.5, n_leaves)
    ax.set_xlabel('Branch length (JC distance)', color='white', fontsize=8)
    ax.set_title(title, color='white', fontsize=9)
    ax.tick_params(colors='white')
    ax.yaxis.set_visible(False)
    for sp in ax.spines.values():
        sp.set_edgecolor('#888')


def draw_nj_tree(ax, adj, leaf_names, title):
    """Simple radial layout for NJ unrooted tree."""
    ax.cla()
    ax.set_facecolor('#0f3460')

    # BFS layout from first leaf
    visited = {}
    pos = {}
    queue = [(leaf_names[0], None, 0, 0.0)]
    angle_step = 2 * np.pi / len(leaf_names)
    leaf_angle = [0]

    def bfs_pos(name, parent, depth, angle_in):
        neighbors = [nb for nb, d in adj.get(name, []) if nb != parent]
        if not neighbors and name in leaf_names:
            a = leaf_angle[0] * angle_step
            leaf_angle[0] += 1
            return a
        return angle_in

    # Use a simple recursive DFS
    placed = set()
    angles = {}
    angle_counter = [0]

    def place(node, parent, radius):
        if node in placed:
            return
        placed.add(node)
        neighbors = [nb for nb, d in adj.get(node, []) if nb != parent]
        if not neighbors:   # leaf
            a = angle_counter[0] * 2 * np.pi / len(leaf_names)
            angle_counter[0] += 1
            pos[node] = (radius * np.cos(a), radius * np.sin(a))
        else:
            if parent is None:
                pos[node] = (0, 0)
            for nb, d in adj.get(node, []):
                if nb != parent:
                    place(nb, node, radius + d * 15)
            # Set internal node position as mean of neighbors
            if parent is not None:
                nbpos = [pos[nb] for nb, d in adj.get(node, []) if nb in pos]
                if nbpos:
                    pos[node] = (np.mean([p[0] for p in nbpos]),
                                 np.mean([p[1] for p in nbpos]))
                else:
                    pos[node] = (0, 0)

    place(leaf_names[0], None, 1.0)

    # Draw edges
    drawn_edges = set()
    for node, neighbors in adj.items():
        if node not in pos:
            continue
        for nb, d in neighbors:
            edge = tuple(sorted([node, nb]))
            if edge in drawn_edges or nb not in pos:
                continue
            drawn_edges.add(edge)
            x1, y1 = pos[node]
            x2, y2 = pos[nb]
            ax.plot([x1, x2], [y1, y2], '-', color='#8be9fd', lw=1.5, alpha=0.8)
            mx, my = (x1 + x2) / 2, (y1 + y2) / 2
            ax.text(mx, my, f'{d:.3f}', ha='center', va='center',
                    fontsize=5.5, color='#f5a623', zorder=5)

    # Draw nodes
    for node, (x, y) in pos.items():
        is_leaf = node in leaf_names
        color = '#50fa7b' if is_leaf else '#f5a623'
        ms = 7 if is_leaf else 5
        ax.plot(x, y, 'o', color=color, ms=ms, zorder=4, markeredgecolor='white', markeredgewidth=0.5)
        if is_leaf:
            ax.text(x * 1.12, y * 1.12, node,
                    ha='center', va='center', fontsize=8,
                    color='#50fa7b', fontfamily='monospace',
                    bbox=dict(facecolor='#1a1a2e', edgecolor='none', alpha=0.6, pad=1))

    ax.set_aspect('equal')
    ax.axis('off')
    ax.set_title(title, color='white', fontsize=9)


# ── Main Run ──────────────────────────────────────────────────────────────────

def run(demo_name):
    demo = DEMOS[demo_name]
    sequences = demo['sequences']
    names, D = build_distance_matrix(sequences)

    upgma_tree = upgma(names, D)
    nj_adj = neighbor_joining(names, D)

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Phylogenetics — {demo_name}  (Jukes-Cantor distances)',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.04, right=0.97, top=0.88, bottom=0.06,
                           hspace=0.45, wspace=0.3)

    ax_matrix = fig.add_subplot(gs[0, 0])
    ax_upgma  = fig.add_subplot(gs[:, 1])
    ax_nj     = fig.add_subplot(gs[:, 2])
    ax_info   = fig.add_subplot(gs[1, 0])

    for ax in [ax_matrix, ax_upgma, ax_nj, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Distance Matrix Heatmap ───────────────────────────────────────────────
    im = ax_matrix.imshow(D, cmap='YlOrRd', aspect='auto', interpolation='nearest')
    ax_matrix.set_xticks(range(len(names)))
    ax_matrix.set_xticklabels(names, rotation=45, ha='right', color='white', fontsize=7)
    ax_matrix.set_yticks(range(len(names)))
    ax_matrix.set_yticklabels(names, color='white', fontsize=7)
    ax_matrix.set_title('JC Distance Matrix', color='white', fontsize=9)
    ax_matrix.tick_params(colors='white')

    for i in range(len(names)):
        for j in range(len(names)):
            ax_matrix.text(j, i, f'{D[i, j]:.3f}', ha='center', va='center',
                           fontsize=5.5,
                           color='black' if D[i, j] < D.max() / 2 else 'white')

    plt.colorbar(im, ax=ax_matrix, fraction=0.046, pad=0.04,
                 label='JC distance').ax.yaxis.label.set_color('white')

    # ── UPGMA Tree ────────────────────────────────────────────────────────────
    draw_upgma_tree(ax_upgma, upgma_tree,
                    'UPGMA Tree\n(rooted, ultrametric)')

    # ── NJ Tree ───────────────────────────────────────────────────────────────
    draw_nj_tree(ax_nj, nj_adj, names,
                 'Neighbor-Joining Tree\n(unrooted)')

    # ── Info Panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    min_d = D[D > 0].min() if (D > 0).any() else 0
    max_d = D.max()
    # Find closest pair
    idx = np.unravel_index(np.argmin(D + np.eye(len(names)) * 999), D.shape)
    closest = f"{names[idx[0]]} ↔ {names[idx[1]]} ({D[idx[0], idx[1]]:.4f})"

    info = (
        f"Dataset: {demo_name}\n"
        f"Sequences: {len(names)}\n"
        f"Seq length: {len(clean_seq(list(sequences.values())[0]))} nt\n\n"
        f"Distance range:\n"
        f"  min = {min_d:.4f}\n"
        f"  max = {max_d:.4f}\n\n"
        f"Closest pair:\n  {closest}\n\n"
        f"{demo['description']}\n\n"
        f"Algorithms:\n"
        f"  UPGMA: O(n²) per merge → O(n³)\n"
        f"  NJ: O(n³) per step → O(n³)\n"
        f"  ML (IQ-TREE): O(n·log n) heuristic"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nDemo: {demo_name}")
    print(f"Sequences: {len(names)}")
    print("\nDistance matrix:")
    header = ' ' * 12 + '  '.join(f'{n[:6]:>6}' for n in names)
    print(header)
    for i, row in enumerate(D):
        print(f"{names[i][:12]:12}" + '  '.join(f'{v:6.4f}' for v in row))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Phylogenetics tree builder')
    parser.add_argument('--demo', default='Mammals (CytB)',
                        choices=list(DEMOS.keys()))
    args = parser.parse_args()

    print("Phylogenetics: Distance Matrices & Tree Building")
    print("=" * 55)
    run(args.demo)
