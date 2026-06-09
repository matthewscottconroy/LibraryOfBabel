"""
Sequence Logo & Position Weight Matrix
=======================================
Tier 1.1 — Bioinformatics: Sequence Analysis

Concepts demonstrated:
  - Multiple sequence alignment and conservation
  - Position frequency matrix (PFM) and position weight matrix (PWM)
  - Information content: IC(i) = log2(4) - H(i) where H = Shannon entropy
  - Sequence logos: letter height ∝ IC × frequency
  - PSSM scoring of new sequences
  - Consensus sequence extraction
  - Motif scanning across a target sequence

Usage:
    python 06_sequence_logo_pwm.py [--demo DEMO]

Demos:
    0 — E. coli σ70 -10 element (TATAAT)
    1 — Kozak consensus (ribosome binding in eukaryotes)
    2 — TATA box (eukaryotic core promoter)
    3 — Custom: enter aligned sequences interactively

Key insight:
    Information content measures how much a position deviates from
    uniform. A perfectly conserved position contributes log2(4)=2 bits;
    a random position contributes 0 bits. The total logo height at each
    position is the IC, and individual letter heights are IC × frequency.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.patches as patches
from matplotlib.font_manager import FontProperties
from collections import Counter


# ── Built-in Demo Alignments ──────────────────────────────────────────────────

DEMOS = {
    "σ70 -10 Element": {
        "sequences": [
            "TATAAT", "TATGAT", "TATACT", "TACAAT",
            "TATAAT", "GATAAT", "TATAAT", "TATCAT",
            "TATAAA", "TATAAT", "CATAAT", "TATAAT",
            "TATAAT", "TAAAAT", "TATAAT", "TATAGT",
        ],
        "alphabet": "ACGT",
        "description": (
            "E. coli σ70 Promoter -10 Element\n"
            "Consensus: TATAAT\n\n"
            "Located ~10 bp upstream of transcription start.\n"
            "High conservation at T₁ and A₅ positions.\n"
            "σ factor contacts these bases directly."
        ),
        "color_scheme": "nucleotide",
    },
    "Kozak Sequence": {
        "sequences": [
            "GCCACCATGG", "GCCGCCATGG", "ACCACCATGG", "GCCACCATGG",
            "GCCGCCATGG", "CCCACCATGG", "GCCACCATGG", "GCAACCATGG",
            "GCCACCATGG", "GCTACCATGG", "GCCACCATGG", "ACCGCCATGG",
            "GCCACCATGG", "GCCACCATGG", "GCCACCATGG", "TCCACCATGG",
        ],
        "alphabet": "ACGT",
        "description": (
            "Kozak Consensus Sequence\n"
            "Consensus: GCCACCATGG\n\n"
            "Flanks the AUG start codon in eukaryotes.\n"
            "Positions -3 (A/G) and +4 (G) most critical.\n"
            "Affects ribosome recognition efficiency."
        ),
        "color_scheme": "nucleotide",
    },
    "TATA Box": {
        "sequences": [
            "TATAAAAG", "TATAATAG", "TATAATGG", "TATAAAAG",
            "TATAATAG", "TATTAAAG", "TATAATAG", "TATAAAGG",
            "TATAATAG", "TATATTTG", "TATAATAG", "TATAAAAG",
            "TATATAAG", "TATAATAG", "TATAACAG", "TATAAAAG",
        ],
        "alphabet": "ACGT",
        "description": (
            "TATA Box (eukaryotic core promoter)\n"
            "Consensus: TATAAA\n\n"
            "Recognized by TATA-binding protein (TBP).\n"
            "Located ~25-30 bp upstream of TSS.\n"
            "Only ~24% of human promoters have TATA box;\n"
            "most use CpG islands instead."
        ),
        "color_scheme": "nucleotide",
    },
    "RGD Motif (Protein)": {
        "sequences": [
            "GRGDSP", "GRGDTP", "ARGDSP", "GRGDNP",
            "SRGDSP", "GRGDSA", "GKGDSP", "GRGDSP",
            "ARGDAP", "GRGDSP", "GRGDTP", "GRGDSP",
            "GRGDSP", "NRGDSP", "GRGDSP", "GRGDVP",
        ],
        "alphabet": "ACDEFGHIKLMNPQRSTVWY",
        "description": (
            "RGD Integrin-Binding Motif (Fibronectin)\n"
            "Consensus: GRGDSP\n\n"
            "Arg-Gly-Asp (RGD) is the minimal recognition\n"
            "sequence for integrin cell adhesion receptors.\n"
            "G at -1 and P at +3 enhance binding.\n"
            "Used in bioengineering for scaffold adhesion."
        ),
        "color_scheme": "chemistry",
    },
}

# ── PWM / IC Functions ────────────────────────────────────────────────────────

def build_pfm(sequences, alphabet):
    """Position Frequency Matrix: count of each character at each position."""
    length = len(sequences[0])
    pfm = np.zeros((len(alphabet), length), dtype=float)
    alpha_idx = {c: i for i, c in enumerate(alphabet)}

    for seq in sequences:
        for pos, ch in enumerate(seq):
            if ch in alpha_idx:
                pfm[alpha_idx[ch], pos] += 1
    return pfm


def pfm_to_ppm(pfm, pseudocount=0.5):
    """Position Probability Matrix with Laplace pseudocounts."""
    ppm = pfm + pseudocount
    ppm /= ppm.sum(axis=0, keepdims=True)
    return ppm


def ppm_to_pwm(ppm, bg=None):
    """Position Weight Matrix: log-odds vs background."""
    if bg is None:
        bg = np.ones(ppm.shape[0]) / ppm.shape[0]
    bg = np.array(bg)[:, np.newaxis]
    return np.log2(ppm / bg)


def information_content(ppm):
    """IC at each position: log2(n_states) - H(column)."""
    n = ppm.shape[0]
    H = -np.sum(ppm * np.log2(ppm + 1e-12), axis=0)
    return np.log2(n) - H


def consensus_sequence(ppm, alphabet):
    """Return the most probable character at each position."""
    return ''.join(alphabet[i] for i in np.argmax(ppm, axis=0))


def score_sequence(seq, pwm, alpha_idx):
    """PSSM log-odds score for a sequence."""
    score = 0.0
    for pos, ch in enumerate(seq):
        if ch in alpha_idx and pos < pwm.shape[1]:
            score += pwm[alpha_idx[ch], pos]
    return score


def scan_sequence(target, pwm, alpha_idx, motif_len):
    """Slide PWM across target, return (position, score) pairs."""
    results = []
    for i in range(len(target) - motif_len + 1):
        subseq = target[i:i + motif_len]
        s = score_sequence(subseq, pwm, alpha_idx)
        results.append((i, s))
    return results


# ── Color Schemes ─────────────────────────────────────────────────────────────

NUCLEOTIDE_COLORS = {'A': '#50fa7b', 'C': '#8be9fd', 'G': '#f1fa8c', 'T': '#ff79c6', 'U': '#ff79c6'}
CHEMISTRY_COLORS  = {
    # Positive: blue, Negative: red, Polar: green, Nonpolar: yellow, Special: orange
    'R': '#6272a4', 'K': '#6272a4', 'H': '#6272a4',  # positive
    'D': '#ff5555', 'E': '#ff5555',                   # negative
    'S': '#50fa7b', 'T': '#50fa7b', 'N': '#50fa7b', 'Q': '#50fa7b', 'Y': '#50fa7b',  # polar
    'C': '#f1fa8c', 'M': '#f1fa8c',                   # sulfur
    'G': '#ff79c6', 'P': '#ff79c6',                   # special
    'A': '#ffb86c', 'V': '#ffb86c', 'I': '#ffb86c', 'L': '#ffb86c',
    'F': '#ffb86c', 'W': '#ffb86c',                   # nonpolar
}


def get_color(ch, scheme):
    if scheme == 'nucleotide':
        return NUCLEOTIDE_COLORS.get(ch, '#ffffff')
    return CHEMISTRY_COLORS.get(ch, '#aaaaaa')


# ── Logo Drawing ──────────────────────────────────────────────────────────────

def draw_logo(ax, ppm, ic, alphabet, color_scheme, title):
    """Draw a sequence logo on ax."""
    ax.cla()
    ax.set_facecolor('#0f3460')

    n_pos = ppm.shape[1]
    fp = FontProperties(family='monospace', weight='bold')

    for pos in range(n_pos):
        col_ic = ic[pos]
        # Sort letters by frequency (smallest at bottom)
        order = np.argsort(ppm[:, pos])
        y_bottom = 0.0

        for idx in order:
            ch = alphabet[idx]
            freq = ppm[idx, pos]
            height = freq * col_ic      # letter height in bits
            if height < 1e-4:
                continue
            color = get_color(ch, color_scheme)

            # Draw letter as scaled text in a bounding box
            # Use a rectangle + text approximation
            rect = patches.FancyBboxPatch(
                (pos + 0.05, y_bottom), 0.90, height,
                boxstyle="square,pad=0.0",
                facecolor=color, edgecolor='none', alpha=0.88, zorder=2
            )
            ax.add_patch(rect)
            # Center the letter in its box
            ax.text(pos + 0.5, y_bottom + height / 2, ch,
                    ha='center', va='center',
                    fontsize=max(4, min(22, height * 18)),
                    fontproperties=fp,
                    color='#1a1a2e', zorder=3, clip_on=True)
            y_bottom += height

    ax.set_xlim(-0.2, n_pos + 0.2)
    ax.set_ylim(0, np.log2(len(alphabet)) + 0.1)
    ax.set_xticks(np.arange(n_pos) + 0.5)
    ax.set_xticklabels([str(i + 1) for i in range(n_pos)], color='white', fontsize=8)
    ax.set_ylabel('Information\nContent (bits)', color='white', fontsize=8)
    ax.set_title(title, color='white', fontsize=9)
    ax.tick_params(colors='white')
    ax.axhline(np.log2(len(alphabet)), color='#888', lw=0.8, ls='--', alpha=0.5)
    for sp in ax.spines.values():
        sp.set_edgecolor('#888')


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(demo_name):
    demo = DEMOS[demo_name]
    seqs = demo['sequences']
    alphabet = demo['alphabet']
    alpha_idx = {c: i for i, c in enumerate(alphabet)}
    color_scheme = demo['color_scheme']
    motif_len = len(seqs[0])

    pfm = build_pfm(seqs, alphabet)
    ppm = pfm_to_ppm(pfm)
    pwm = ppm_to_pwm(ppm)
    ic  = information_content(ppm)
    consensus = consensus_sequence(ppm, alphabet)

    # Build a synthetic target for scanning demo
    target = seqs[0] * 3 + 'NNNNN' + seqs[1] + 'AAAA' + seqs[-1]
    target = target.replace('N', 'A')   # sanitize
    scan_results = scan_sequence(target, pwm, alpha_idx, motif_len)
    positions, scores = zip(*scan_results)

    # Score all training sequences
    train_scores = [score_sequence(s, pwm, alpha_idx) for s in seqs]

    # ── Figure ────────────────────────────────────────────────────────────────
    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Sequence Logo & PWM — {demo_name}',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_logo   = fig.add_subplot(gs[0, :2])   # sequence logo (spans 2 cols)
    ax_info   = fig.add_subplot(gs[0, 2])    # text info
    ax_ic     = fig.add_subplot(gs[1, 0])    # IC per position
    ax_pwm    = fig.add_subplot(gs[1, 1])    # PWM heatmap
    ax_scan   = fig.add_subplot(gs[1, 2])    # PSSM scan

    for ax in [ax_logo, ax_info, ax_ic, ax_pwm, ax_scan]:
        ax.set_facecolor('#0f3460')

    # ── Logo ──────────────────────────────────────────────────────────────────
    draw_logo(ax_logo, ppm, ic, alphabet, color_scheme,
              f'Sequence Logo  (n={len(seqs)} sequences, max IC={np.log2(len(alphabet)):.1f} bits/pos)')

    # ── Info Panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    total_ic = ic.sum()
    info = (
        f"{demo['description']}\n\n"
        f"Consensus: {consensus}\n"
        f"Total IC:  {total_ic:.2f} bits\n"
        f"Per-position IC (bits):\n"
        + '  '.join(f"{v:.2f}" for v in ic) + "\n\n"
        f"PWM (log2 odds vs uniform):\n"
        f"  Positive → enriched\n"
        f"  Negative → depleted"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.5)

    # ── IC per position bar chart ─────────────────────────────────────────────
    bar_colors = ['#50fa7b' if v > 1.5 else '#f1fa8c' if v > 0.7 else '#ff5555' for v in ic]
    ax_ic.bar(np.arange(motif_len) + 0.5, ic, color=bar_colors, edgecolor='white', width=0.7)
    ax_ic.axhline(1.0, color='#f5a623', lw=1, ls='--', label='1 bit (half-conserved)')
    ax_ic.axhline(np.log2(len(alphabet)), color='#888', lw=0.8, ls=':', label=f'Max ({np.log2(len(alphabet)):.1f} bits)')
    ax_ic.set_xticks(np.arange(motif_len) + 0.5)
    ax_ic.set_xticklabels([str(i + 1) for i in range(motif_len)], color='white', fontsize=8)
    ax_ic.set_ylabel('IC (bits)', color='white')
    ax_ic.set_title('Information Content per Position', color='white', fontsize=9)
    ax_ic.tick_params(colors='white')
    ax_ic.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_ic.spines.values():
        sp.set_edgecolor('#888')

    # ── PWM Heatmap ───────────────────────────────────────────────────────────
    # Show only top letters for readability when alphabet is large
    display_rows = min(len(alphabet), 6)
    # For nucleotides show all 4; for proteins show most variable positions
    if len(alphabet) <= 5:
        pwm_display = pwm
        alpha_display = list(alphabet)
    else:
        row_variance = np.var(pwm, axis=1)
        top_idx = np.argsort(row_variance)[-display_rows:][::-1]
        pwm_display = pwm[top_idx, :]
        alpha_display = [alphabet[i] for i in top_idx]

    im = ax_pwm.imshow(pwm_display, aspect='auto', cmap='RdYlGn',
                       vmin=-3, vmax=3, interpolation='nearest')
    ax_pwm.set_xticks(np.arange(motif_len))
    ax_pwm.set_xticklabels([str(i + 1) for i in range(motif_len)], color='white', fontsize=8)
    ax_pwm.set_yticks(np.arange(len(alpha_display)))
    ax_pwm.set_yticklabels(alpha_display, color='white', fontsize=8, fontfamily='monospace')
    ax_pwm.set_title('Position Weight Matrix (log₂ odds)', color='white', fontsize=9)
    ax_pwm.tick_params(colors='white')

    # Annotate cells
    for i in range(pwm_display.shape[0]):
        for j in range(pwm_display.shape[1]):
            v = pwm_display[i, j]
            ax_pwm.text(j, i, f'{v:.1f}', ha='center', va='center',
                        fontsize=6.5, color='black' if -1.5 < v < 1.5 else 'white')

    plt.colorbar(im, ax=ax_pwm, orientation='vertical', fraction=0.04, pad=0.02,
                 label='log₂ odds').ax.yaxis.label.set_color('white')

    # ── PSSM Scan ─────────────────────────────────────────────────────────────
    ax_scan.plot(positions, scores, color='#8be9fd', lw=1.5)
    ax_scan.fill_between(positions, scores, alpha=0.3, color='#8be9fd')
    # Threshold: median of training scores
    threshold = np.median(train_scores)
    ax_scan.axhline(threshold, color='#f5a623', lw=1.2, ls='--',
                    label=f'Threshold (median={threshold:.1f})')
    hits = [p for p, s in zip(positions, scores) if s >= threshold]
    if hits:
        hit_scores = [scores[p] for p in hits]
        ax_scan.scatter(hits, hit_scores, color='#ff79c6', s=50, zorder=5,
                        label=f'{len(hits)} hits')
    ax_scan.set_xlabel('Position in target', color='white')
    ax_scan.set_ylabel('PSSM score', color='white')
    ax_scan.set_title('Motif Scanning (PWM log-odds score)', color='white', fontsize=9)
    ax_scan.tick_params(colors='white')
    ax_scan.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_scan.spines.values():
        sp.set_edgecolor('#888')

    plt.show()

    # Terminal summary
    print(f"\nDemo: {demo_name}")
    print(f"Consensus: {consensus}")
    print(f"Total IC: {total_ic:.2f} bits")
    print(f"IC per position: {' '.join(f'{v:.2f}' for v in ic)}")
    print(f"Training sequence scores: min={min(train_scores):.2f} max={max(train_scores):.2f}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Sequence Logo & PWM explorer')
    parser.add_argument('--demo', default='σ70 -10 Element',
                        choices=list(DEMOS.keys()),
                        help='Which demo to display')
    args = parser.parse_args()

    print("Sequence Logo & Position Weight Matrix")
    print("=" * 50)
    print(f"Demo: {args.demo}")
    print()
    run(args.demo)
