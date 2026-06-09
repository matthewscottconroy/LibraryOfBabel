"""
Pairwise Sequence Alignment — Needleman-Wunsch & Smith-Waterman
===============================================================
Tier 1.1 — Bioinformatics: Sequence Analysis

Concepts demonstrated:
  - Dynamic programming alignment (NW global; SW local)
  - Scoring matrices: identity, BLOSUM62
  - Gap penalties: linear and affine
  - Traceback path visualization
  - Alignment statistics: score, identity, gaps

Usage:
    python 04_pairwise_alignment.py [--seq1 SEQ1] [--seq2 SEQ2] [--mode nw|sw]

Examples:
    python 04_pairwise_alignment.py
    python 04_pairwise_alignment.py --seq1 ACGTACGT --seq2 ACGTTCGT --mode nw
    python 04_pairwise_alignment.py --seq1 HEAGAWGHEE --seq2 PAWHEAE --mode sw
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.patches import FancyArrowPatch
import textwrap


# ── BLOSUM62 Substitution Matrix (partial) ────────────────────────────────────

BLOSUM62 = {
    ('A','A'):4,  ('A','R'):-1, ('A','N'):-2, ('A','D'):-2, ('A','C'):0,
    ('A','Q'):-1, ('A','E'):-1, ('A','G'):0,  ('A','H'):-2, ('A','I'):-1,
    ('A','L'):-1, ('A','K'):-1, ('A','M'):-1, ('A','F'):-2, ('A','P'):-1,
    ('A','S'):1,  ('A','T'):0,  ('A','W'):-3, ('A','Y'):-2, ('A','V'):0,
    ('R','R'):5,  ('R','N'):0,  ('R','D'):-2, ('R','C'):-3, ('R','Q'):1,
    ('R','E'):0,  ('R','G'):-2, ('R','H'):0,  ('R','I'):-3, ('R','L'):-2,
    ('R','K'):2,  ('R','M'):-1, ('R','F'):-3, ('R','P'):-2, ('R','S'):-1,
    ('R','T'):-1, ('R','W'):-3, ('R','Y'):-2, ('R','V'):-3,
    ('N','N'):6,  ('N','D'):1,  ('N','C'):-3, ('N','Q'):0,  ('N','E'):0,
    ('N','G'):0,  ('N','H'):1,  ('N','I'):-3, ('N','L'):-3, ('N','K'):0,
    ('N','M'):-2, ('N','F'):-3, ('N','P'):-2, ('N','S'):1,  ('N','T'):0,
    ('N','W'):-4, ('N','Y'):-2, ('N','V'):-3,
    ('D','D'):6,  ('D','C'):-3, ('D','Q'):0,  ('D','E'):2,  ('D','G'):-1,
    ('D','H'):-1, ('D','I'):-3, ('D','L'):-4, ('D','K'):-1, ('D','M'):-3,
    ('D','F'):-3, ('D','P'):-1, ('D','S'):0,  ('D','T'):-1, ('D','W'):-4,
    ('D','Y'):-3, ('D','V'):-3,
    ('C','C'):9,  ('C','Q'):-3, ('C','E'):-4, ('C','G'):-3, ('C','H'):-3,
    ('C','I'):-1, ('C','L'):-1, ('C','K'):-3, ('C','M'):-1, ('C','F'):-2,
    ('C','P'):-3, ('C','S'):-1, ('C','T'):-1, ('C','W'):-2, ('C','Y'):-2,
    ('C','V'):-1,
    ('Q','Q'):5,  ('Q','E'):2,  ('Q','G'):-2, ('Q','H'):0,  ('Q','I'):-3,
    ('Q','L'):-2, ('Q','K'):1,  ('Q','M'):0,  ('Q','F'):-3, ('Q','P'):-1,
    ('Q','S'):0,  ('Q','T'):-1, ('Q','W'):-2, ('Q','Y'):-1, ('Q','V'):-2,
    ('E','E'):5,  ('E','G'):-2, ('E','H'):0,  ('E','I'):-3, ('E','L'):-3,
    ('E','K'):1,  ('E','M'):-2, ('E','F'):-3, ('E','P'):-1, ('E','S'):0,
    ('E','T'):-1, ('E','W'):-3, ('E','Y'):-2, ('E','V'):-2,
    ('G','G'):6,  ('G','H'):-2, ('G','I'):-4, ('G','L'):-4, ('G','K'):-2,
    ('G','M'):-3, ('G','F'):-3, ('G','P'):-2, ('G','S'):0,  ('G','T'):-2,
    ('G','W'):-2, ('G','Y'):-3, ('G','V'):-3,
    ('H','H'):8,  ('H','I'):-3, ('H','L'):-3, ('H','K'):-1, ('H','M'):-2,
    ('H','F'):-1, ('H','P'):-2, ('H','S'):-1, ('H','T'):-2, ('H','W'):-2,
    ('H','Y'):2,  ('H','V'):-3,
    ('I','I'):4,  ('I','L'):2,  ('I','K'):-1, ('I','M'):1,  ('I','F'):0,
    ('I','P'):-3, ('I','S'):-2, ('I','T'):-1, ('I','W'):-3, ('I','Y'):-1,
    ('I','V'):3,
    ('L','L'):4,  ('L','K'):-2, ('L','M'):2,  ('L','F'):0,  ('L','P'):-3,
    ('L','S'):-2, ('L','T'):-1, ('L','W'):-2, ('L','Y'):-1, ('L','V'):1,
    ('K','K'):5,  ('K','M'):-1, ('K','F'):-3, ('K','P'):-1, ('K','S'):0,
    ('K','T'):-1, ('K','W'):-3, ('K','Y'):-2, ('K','V'):-2,
    ('M','M'):5,  ('M','F'):0,  ('M','P'):-2, ('M','S'):-1, ('M','T'):-1,
    ('M','W'):-1, ('M','Y'):-1, ('M','V'):1,
    ('F','F'):6,  ('F','P'):-4, ('F','S'):-2, ('F','T'):-2, ('F','W'):1,
    ('F','Y'):3,  ('F','V'):-1,
    ('P','P'):7,  ('P','S'):-1, ('P','T'):-1, ('P','W'):-4, ('P','Y'):-3,
    ('P','V'):-2,
    ('S','S'):4,  ('S','T'):1,  ('S','W'):-3, ('S','Y'):-2, ('S','V'):-2,
    ('T','T'):5,  ('T','W'):-2, ('T','Y'):-2, ('T','V'):0,
    ('W','W'):11, ('W','Y'):2,  ('W','V'):-3,
    ('Y','Y'):7,  ('Y','V'):-1,
    ('V','V'):4,
}

def blosum62_score(a, b):
    a, b = a.upper(), b.upper()
    if a == b and a in 'ACGT':  # nucleotide identity
        return 2
    key = (a, b) if (a, b) in BLOSUM62 else (b, a)
    return BLOSUM62.get(key, 0)

def identity_score(a, b):
    return 1 if a.upper() == b.upper() else -1


# ── Alignment Algorithms ──────────────────────────────────────────────────────

DIAG, LEFT, UP = 0, 1, 2


def needleman_wunsch(seq1, seq2, score_fn=identity_score, gap=-2):
    """
    Global alignment (Needleman-Wunsch, 1970).
    Returns: score matrix, traceback matrix, score, alignment strings.
    """
    m, n = len(seq1), len(seq2)
    H = np.zeros((m + 1, n + 1))
    T = np.zeros((m + 1, n + 1), dtype=int)

    # Initialise edges
    for i in range(m + 1):
        H[i, 0] = gap * i
        T[i, 0] = UP
    for j in range(n + 1):
        H[0, j] = gap * j
        T[0, j] = LEFT
    T[0, 0] = -1

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            match  = H[i-1, j-1] + score_fn(seq1[i-1], seq2[j-1])
            delete = H[i-1, j]   + gap
            insert = H[i,   j-1] + gap
            best = max(match, delete, insert)
            H[i, j] = best
            if best == match:
                T[i, j] = DIAG
            elif best == delete:
                T[i, j] = UP
            else:
                T[i, j] = LEFT

    # Traceback
    a1, a2, path = [], [], []
    i, j = m, n
    while i > 0 or j > 0:
        path.append((i, j))
        if T[i, j] == DIAG:
            a1.append(seq1[i-1])
            a2.append(seq2[j-1])
            i -= 1; j -= 1
        elif T[i, j] == UP:
            a1.append(seq1[i-1])
            a2.append('-')
            i -= 1
        else:
            a1.append('-')
            a2.append(seq2[j-1])
            j -= 1
    path.append((0, 0))
    return H, T, H[m, n], ''.join(reversed(a1)), ''.join(reversed(a2)), path


def smith_waterman(seq1, seq2, score_fn=identity_score, gap=-2):
    """
    Local alignment (Smith-Waterman, 1981).
    Returns: score matrix, traceback, best score, alignment strings.
    """
    m, n = len(seq1), len(seq2)
    H = np.zeros((m + 1, n + 1))
    T = np.zeros((m + 1, n + 1), dtype=int) - 1

    best_score = 0
    best_cell = (0, 0)

    for i in range(1, m + 1):
        for j in range(1, n + 1):
            match  = H[i-1, j-1] + score_fn(seq1[i-1], seq2[j-1])
            delete = H[i-1, j]   + gap
            insert = H[i,   j-1] + gap
            best = max(0, match, delete, insert)
            H[i, j] = best
            if best == 0:
                T[i, j] = -1
            elif best == match:
                T[i, j] = DIAG
            elif best == delete:
                T[i, j] = UP
            else:
                T[i, j] = LEFT
            if best > best_score:
                best_score = best
                best_cell = (i, j)

    # Traceback from best cell
    a1, a2, path = [], [], []
    i, j = best_cell
    while H[i, j] > 0:
        path.append((i, j))
        if T[i, j] == DIAG:
            a1.append(seq1[i-1])
            a2.append(seq2[j-1])
            i -= 1; j -= 1
        elif T[i, j] == UP:
            a1.append(seq1[i-1])
            a2.append('-')
            i -= 1
        else:
            a1.append('-')
            a2.append(seq2[j-1])
            j -= 1
    path.append((i, j))
    return H, T, best_score, ''.join(reversed(a1)), ''.join(reversed(a2)), path


def alignment_stats(a1, a2):
    """Compute identity, gaps, and match string."""
    assert len(a1) == len(a2)
    match_str = []
    idents = gaps = 0
    for c1, c2 in zip(a1, a2):
        if c1 == c2 and c1 != '-':
            match_str.append('|')
            idents += 1
        elif c1 == '-' or c2 == '-':
            match_str.append(' ')
            gaps += 1
        else:
            match_str.append('.')
    pct = 100 * idents / len(a1)
    return ''.join(match_str), idents, gaps, pct


# ── Visualisation ─────────────────────────────────────────────────────────────

def visualise(seq1, seq2, mode='nw'):
    is_protein = any(c.upper() in 'DEFHIKLMNPQRSVWY' for c in seq1 + seq2)
    score_fn = blosum62_score if is_protein else identity_score
    score_label = 'BLOSUM62' if is_protein else 'Identity (+1/-1)'

    if mode == 'nw':
        H, T, score, a1, a2, path = needleman_wunsch(seq1, seq2, score_fn)
        title_mode = 'Needleman-Wunsch (Global)'
    else:
        H, T, score, a1, a2, path = smith_waterman(seq1, seq2, score_fn)
        title_mode = 'Smith-Waterman (Local)'

    match_str, idents, gaps, pct_id = alignment_stats(a1, a2)

    fig = plt.figure(figsize=(16, 9))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Pairwise Alignment: {title_mode}  |  Score matrix: {score_label}',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 2, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.05,
                           hspace=0.4, wspace=0.3)

    ax_matrix = fig.add_subplot(gs[:, 0])   # DP matrix heatmap
    ax_align  = fig.add_subplot(gs[0, 1])   # alignment text
    ax_stats  = fig.add_subplot(gs[1, 1])   # statistics bar chart

    for ax in [ax_matrix, ax_align, ax_stats]:
        ax.set_facecolor('#0f3460')

    # ── DP matrix ────────────────────────────────────────────────────────────
    m, n = len(seq1), len(seq2)
    im = ax_matrix.imshow(H, cmap='plasma', aspect='auto', origin='upper')
    plt.colorbar(im, ax=ax_matrix, fraction=0.04, label='Score', pad=0.02)

    # Traceback path
    if path:
        py, px = zip(*[(r, c) for r, c in path])
        ax_matrix.plot(px, py, 'w-', lw=2.2, alpha=0.85, label='Traceback')
        ax_matrix.plot(px[0], py[0], 'wo', ms=8, zorder=10)
        ax_matrix.plot(px[-1], py[-1], 'ws', ms=8, zorder=10)

    # Axis labels
    ax_matrix.set_xticks(range(n + 1))
    ax_matrix.set_xticklabels([''] + list(seq2), fontsize=8, color='white')
    ax_matrix.set_yticks(range(m + 1))
    ax_matrix.set_yticklabels([''] + list(seq1), fontsize=8, color='white')
    ax_matrix.set_title('DP Score Matrix + Traceback Path', color='white')
    ax_matrix.tick_params(colors='white')

    # Annotate cells with scores (if small enough)
    if m <= 12 and n <= 12:
        for i in range(m + 1):
            for j in range(n + 1):
                ax_matrix.text(j, i, f'{H[i,j]:.0f}',
                               ha='center', va='center', fontsize=6.5,
                               color='white', fontweight='bold')

    # ── Alignment text ────────────────────────────────────────────────────────
    ax_align.axis('off')
    wrap = 50
    lines = []
    for start in range(0, len(a1), wrap):
        chunk1 = a1[start:start+wrap]
        chunkm = match_str[start:start+wrap]
        chunk2 = a2[start:start+wrap]
        lines += [f"Seq1: {chunk1}", f"      {chunkm}", f"Seq2: {chunk2}", ""]

    full_text = '\n'.join(lines)
    ax_align.text(0.02, 0.97, full_text, transform=ax_align.transAxes,
                  va='top', fontsize=7.5, color='white', fontfamily='monospace')
    ax_align.set_title(f'Alignment  (score = {score:.0f})', color='white')

    # ── Statistics ────────────────────────────────────────────────────────────
    categories = ['Identity', 'Gaps', 'Mismatches']
    mismatches = len(a1) - idents - gaps
    values = [idents, gaps, mismatches]
    colors = ['#50fa7b', '#ff5555', '#f5a623']
    bars = ax_stats.bar(categories, values, color=colors, edgecolor='white', width=0.5)
    for bar, val in zip(bars, values):
        ax_stats.text(bar.get_x() + bar.get_width()/2, bar.get_height() + 0.05,
                      str(val), ha='center', color='white', fontsize=9)
    ax_stats.set_title(f'Alignment Length: {len(a1)}  |  Identity: {pct_id:.1f}%',
                       color='white')
    ax_stats.tick_params(colors='white')
    for sp in ax_stats.spines.values():
        sp.set_edgecolor('#888')

    # Print to terminal
    print(f"\n{'='*60}")
    print(f"  Mode:   {title_mode}")
    print(f"  Score:  {score:.0f}  |  Matrix: {score_label}")
    print(f"  Length: {len(a1)}  |  Identity: {pct_id:.1f}%  |  Gaps: {gaps}")
    print(f"{'='*60}")
    print(f"  Seq1: {a1}")
    print(f"        {match_str}")
    print(f"  Seq2: {a2}")
    print(f"{'='*60}\n")

    plt.tight_layout()
    plt.show()


# ── CLI Entry Point ────────────────────────────────────────────────────────────

DEMO_PAIRS = [
    ("ACGTACGTACGT", "ACGTTACGT", "nw", "DNA — global alignment with indel"),
    ("HEAGAWGHEE",   "PAWHEAE",   "sw", "Protein — classic local alignment example"),
    ("ATCGTAGCTA",   "ATCGTCGTA", "nw", "DNA — point mutation detection"),
]

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Pairwise Sequence Alignment Visualiser')
    parser.add_argument('--seq1', default=None)
    parser.add_argument('--seq2', default=None)
    parser.add_argument('--mode', default='nw', choices=['nw', 'sw'])
    parser.add_argument('--demo', type=int, default=None,
                        help='Run demo 0, 1, or 2 instead of custom sequences')
    args = parser.parse_args()

    print("Pairwise Sequence Alignment — Needleman-Wunsch & Smith-Waterman")
    print("=" * 60)

    if args.demo is not None:
        s1, s2, mode, desc = DEMO_PAIRS[args.demo]
        print(f"Demo {args.demo}: {desc}")
    elif args.seq1 and args.seq2:
        s1, s2, mode = args.seq1, args.seq2, args.mode
    else:
        print("No sequences provided — running default demo.")
        print("Usage: python 04_pairwise_alignment.py --seq1 SEQ1 --seq2 SEQ2 --mode nw|sw")
        print("Demo options: --demo 0, 1, or 2\n")
        s1, s2, mode, _ = DEMO_PAIRS[0]

    print(f"\nSeq1: {s1}")
    print(f"Seq2: {s2}")
    print(f"Mode: {'Global (NW)' if mode == 'nw' else 'Local (SW)'}\n")
    visualise(s1, s2, mode)
