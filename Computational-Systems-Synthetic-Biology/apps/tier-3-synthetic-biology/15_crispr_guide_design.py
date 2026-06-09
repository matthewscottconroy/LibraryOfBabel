"""
CRISPR Guide RNA Design & Off-Target Scoring
=============================================
Tier 3.3 — Synthetic Biology: Genome Editing

Concepts demonstrated:
  - Cas9 PAM site recognition (NGG for SpCas9)
  - Guide RNA (gRNA) identification from target sequence
  - On-target efficiency scoring (Rule Set 2 / Doench 2016 features)
  - Off-target scoring: mismatch counting and position weighting
  - Seed region importance (positions 1-12 from PAM)
  - GC content and secondary structure proxies
  - Visualization of guide candidates and their predicted activity

Usage:
    python 15_crispr_guide_design.py [--seq SEQUENCE] [--demo DEMO]

Demos:
    eGFP    — Guide RNAs targeting eGFP coding sequence
    lacZ    — Guide RNAs targeting E. coli lacZ gene
    PCSK9   — Guides targeting human PCSK9 (therapeutic target)

Key insight:
    Not all 20-nt guides are equally effective. On-target efficiency
    depends on: local chromatin (not modelable here), nucleotide
    composition at specific positions (Doench rules), and secondary
    structure of the gRNA. Off-target activity depends strongly on
    mismatches in the seed region (PAM-proximal positions 1-12).
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import re
from collections import Counter


# ── Target Sequences ──────────────────────────────────────────────────────────

DEMO_SEQUENCES = {
    "eGFP": (
        "ATGGTGAGCAAGGGCGAGGAGCTGTTCACCGGGGTGGTGCCCATCCTGGTCGAGCTGGACGGCGACGTAAAC"
        "GGCCACAAGTTCAGCGTGTCCGGCGAGGGCGAGGGCGATGCCACCTACGGCAAGCTGACCCTGAAGTTCATC"
        "TGCACCACCGGCAAGCTGCCCGTGCCCTGGCCCACCCTCGTGACCACCCTGACCTACGGCGTGCAGTGCTTC"
        "AGCCGCTACCCCGACCACATGAAGCAGCACGACTTCTTCAAGTCCGCCATGCCCGAAGGCTACGTCCAGGAG"
    ),
    "lacZ": (
        "ATGACCATGATTACGCCAAGCTATCGCAAGACTTTATGCTTCGGCTCGTATAATGTGTGGAATTGTGAGCGG"
        "ATAACAATTTCACACAGGAAACAGCTATGACCATGATTACGGAATTCGAGCTCGGTACCCGGGGATCCTCTA"
        "GAGTCGACCTGCAGGCATGCAAGCTTGGCGTAATCATGGTCATAGCTGTTTCCTGTGTGAAATTGTTATCC"
        "GCTCACAATTCCACACAACATACGAGCCGGAAGCATAAAGTGTAAAGCCTGGGGTGCCTAATGAGTGAGCTAA"
    ),
    "PCSK9": (
        "ATGGGGCCCAGCGGGCGGCGGCTGCGGGCGCTGGAGCTGCTGCTTCTGCTGCTGGCGCTGGCGCAGGCCAGC"
        "CGGCAGGATGCAGAGGCCCAGTTCCAGCGTCTGGAGCGCAAGCTCAAGGTGGAGGTGGACATGCGGCCCCTG"
        "CGCCTGCGCATCAGCACCTTCACGAAGACGCAGCTGCCCTTCGTGGAGGAGGTGCTGCACAGCGAGCAGAAG"
        "CAGGACTTCATCCTGGAGGTCAGCATCTTCACCAAGGCCACGTCGGGGCCCTTCAAGGAGGTGGAGAAGGTG"
    ),
}


# ── Guide RNA Finding ─────────────────────────────────────────────────────────

def find_guides(sequence, pam='NGG', guide_len=20):
    """Find all potential guide RNAs with the given PAM."""
    guides = []
    seq = sequence.upper()
    rc = {'A': 'T', 'T': 'A', 'C': 'G', 'G': 'C', 'N': 'N'}

    def rc_seq(s):
        return ''.join(rc.get(c, 'N') for c in reversed(s))

    pam_pattern = pam.replace('N', '[ACGT]')

    # Forward strand: guide is immediately 5' of PAM
    for m in re.finditer(f'(?=(.{{{guide_len}}})({pam_pattern}))', seq):
        guide = m.group(1)
        pos = m.start()
        if 'N' not in guide:
            guides.append({
                'seq': guide, 'pos': pos, 'strand': '+',
                'pam': m.group(2), 'context': seq[max(0, pos-3):pos+guide_len+3]
            })

    # Reverse strand: find PAM on reverse complement
    rc_seq_full = rc_seq(seq)
    for m in re.finditer(f'(?=(.{{{guide_len}}})({pam_pattern}))', rc_seq_full):
        guide = m.group(1)
        pos = len(seq) - m.start() - guide_len
        if 'N' not in guide:
            guides.append({
                'seq': guide, 'pos': pos, 'strand': '-',
                'pam': m.group(2), 'context': rc_seq_full[m.start()-3:m.start()+guide_len+3]
            })

    return guides


# ── Scoring Functions ─────────────────────────────────────────────────────────

def gc_content(seq):
    return (seq.count('G') + seq.count('C')) / len(seq) if seq else 0


def doench_score(guide, context=None):
    """
    Simplified Rule Set 2 proxy.
    Full Doench 2016 uses 30-nt context; we approximate with guide features.
    Score is a proxy only — not the actual ML model.
    """
    score = 0.5  # baseline

    gc = gc_content(guide)
    # Optimal GC: 40-70%
    if 0.40 <= gc <= 0.70:
        score += 0.15
    elif gc < 0.25 or gc > 0.80:
        score -= 0.20

    # Position-specific nucleotide preferences (simplified from Doench)
    # Position 20 (closest to PAM) should be G
    if guide[-1] == 'G':
        score += 0.08
    # Position 1 (5'-most) T is bad
    if guide[0] == 'T':
        score -= 0.05
    # Avoid TTTT (Pol III terminator)
    if 'TTTT' in guide:
        score -= 0.30
    # G at position 10 (mid-guide)
    if guide[9] == 'G':
        score += 0.05
    # Avoid runs of same nucleotide
    for nuc in 'ACGT':
        if nuc * 4 in guide:
            score -= 0.15

    return max(0.0, min(1.0, score))


def mismatch_score(guide1, guide2):
    """
    Off-target penalty: count mismatches with position weighting.
    Seed region (positions 12-20 from 5', i.e., 0-7 from PAM) is
    weighted more heavily.
    """
    n = len(guide1)
    seed_weight = 3.0    # seed region positions weighted 3x
    total_penalty = 0.0
    mismatches = 0

    for i, (a, b) in enumerate(zip(guide1, guide2)):
        if a != b:
            mismatches += 1
            # Position from PAM: last positions are PAM-proximal (seed)
            from_pam = n - i
            w = seed_weight if from_pam <= 8 else 1.0
            total_penalty += w

    # Normalize: return fraction (0=identical, 1=completely different)
    max_penalty = seed_weight * 8 + (n - 8) * 1.0
    return total_penalty / max_penalty, mismatches


def off_target_risks(guide, all_guides):
    """Return a risk score (0-1) vs. all other guides from the set."""
    risks = []
    for other in all_guides:
        if other['seq'] == guide['seq']:
            continue
        penalty, mm = mismatch_score(guide['seq'], other['seq'])
        if mm <= 4:   # potential off-target
            risks.append({'guide': other['seq'], 'penalty': penalty, 'mismatches': mm})
    return sorted(risks, key=lambda x: x['mismatches'])[:5]


def score_all(guides):
    """Score all guides and add metrics."""
    for g in guides:
        g['gc'] = gc_content(g['seq'])
        g['doench'] = doench_score(g['seq'], g.get('context'))
        g['has_tttt'] = 'TTTT' in g['seq']
    return guides


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(demo_name='eGFP'):
    sequence = DEMO_SEQUENCES[demo_name]
    guides = find_guides(sequence)
    guides = score_all(guides)

    # Sort by Doench score
    guides_sorted = sorted(guides, key=lambda g: g['doench'], reverse=True)
    top_guides = guides_sorted[:20]

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'CRISPR Guide RNA Design — {demo_name}  '
                 f'({len(guides)} total NGG guides found)',
                 color='white', fontsize=12, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_score  = fig.add_subplot(gs[0, :2])
    ax_gc     = fig.add_subplot(gs[1, 0])
    ax_mm     = fig.add_subplot(gs[1, 1])
    ax_info   = fig.add_subplot(gs[:, 2])

    for ax in [ax_score, ax_gc, ax_mm, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Doench score landscape ────────────────────────────────────────────────
    positions = [g['pos'] for g in guides]
    scores = [g['doench'] for g in guides]
    strands = [g['strand'] for g in guides]
    gcs = [g['gc'] for g in guides]

    sc = ax_score.scatter(
        [p for p, s in zip(positions, strands) if s == '+'],
        [s for s, st in zip(scores, strands) if st == '+'],
        c=[g for g, st in zip(gcs, strands) if st == '+'],
        cmap='RdYlGn', vmin=0.2, vmax=0.8, s=40, alpha=0.8, label='+ strand'
    )
    ax_score.scatter(
        [p for p, s in zip(positions, strands) if s == '-'],
        [s for s, st in zip(scores, strands) if st == '-'],
        c=[g for g, st in zip(gcs, strands) if st == '-'],
        cmap='RdYlGn', vmin=0.2, vmax=0.8, s=40, alpha=0.8, marker='^', label='− strand'
    )
    plt.colorbar(sc, ax=ax_score, label='GC content', fraction=0.02)

    # Mark top 5 guides
    for g in guides_sorted[:5]:
        ax_score.annotate('★', xy=(g['pos'], g['doench']),
                          fontsize=10, color='#f1fa8c', ha='center',
                          xytext=(g['pos'], g['doench'] + 0.03))

    ax_score.axhline(0.6, color='#50fa7b', lw=1.2, ls='--', alpha=0.7,
                     label='Score threshold (0.6)')
    ax_score.set_xlabel('Position in target sequence', color='white')
    ax_score.set_ylabel('Predicted on-target efficiency', color='white')
    ax_score.set_title('Guide RNA Score Landscape  (★ = top 5 candidates)',
                       color='white', fontsize=9)
    ax_score.tick_params(colors='white')
    ax_score.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_score.spines.values():
        sp.set_edgecolor('#888')

    # ── GC content distribution ───────────────────────────────────────────────
    gc_bins = np.arange(0, 1.05, 0.05)
    all_gc = [g['gc'] for g in guides]
    ax_gc.hist(all_gc, bins=gc_bins, color='#8be9fd', edgecolor='white', alpha=0.85)
    ax_gc.axvspan(0.40, 0.70, color='#50fa7b', alpha=0.15, label='Optimal GC')
    ax_gc.axvline(np.mean(all_gc), color='#f5a623', lw=1.5, ls='--',
                  label=f'Mean GC={np.mean(all_gc):.2f}')
    ax_gc.set_xlabel('GC content', color='white')
    ax_gc.set_ylabel('Number of guides', color='white')
    ax_gc.set_title('GC Content Distribution', color='white', fontsize=9)
    ax_gc.tick_params(colors='white')
    ax_gc.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_gc.spines.values():
        sp.set_edgecolor('#888')

    # ── Mismatch analysis between top 10 guides ───────────────────────────────
    n_top = min(10, len(top_guides))
    mm_matrix = np.zeros((n_top, n_top))
    for i in range(n_top):
        for j in range(n_top):
            if i != j:
                _, mm = mismatch_score(top_guides[i]['seq'], top_guides[j]['seq'])
                mm_matrix[i, j] = mm

    im = ax_mm.imshow(mm_matrix, cmap='RdYlGn', vmin=0, vmax=20,
                      aspect='auto', interpolation='nearest')
    short_labels = [f"g{i+1}" for i in range(n_top)]
    ax_mm.set_xticks(range(n_top))
    ax_mm.set_yticks(range(n_top))
    ax_mm.set_xticklabels(short_labels, color='white', fontsize=7)
    ax_mm.set_yticklabels(short_labels, color='white', fontsize=7)
    ax_mm.set_title('Mismatch Matrix (top 10 guides)', color='white', fontsize=9)
    ax_mm.tick_params(colors='white')
    plt.colorbar(im, ax=ax_mm, fraction=0.046, pad=0.04,
                 label='# mismatches').ax.yaxis.label.set_color('white')

    # Annotate cells with low mismatch count (potential cross-reactivity)
    for i in range(n_top):
        for j in range(n_top):
            if i != j and mm_matrix[i, j] <= 4:
                ax_mm.text(j, i, f'{int(mm_matrix[i,j])}',
                           ha='center', va='center', fontsize=7,
                           color='black', fontweight='bold')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    top5_text = "Top 5 Guide Candidates:\n"
    for i, g in enumerate(guides_sorted[:5]):
        top5_text += (f"  #{i+1}  {g['seq']}\n"
                      f"       pos={g['pos']}, strand={g['strand']}\n"
                      f"       score={g['doench']:.3f}, GC={g['gc']:.2f}\n\n")

    info = (
        f"Target: {demo_name}\n"
        f"Length: {len(sequence)} bp\n"
        f"Guides found: {len(guides)}\n"
        f"  (+strand: {sum(1 for g in guides if g['strand']=='+')})\n"
        f"  (−strand: {sum(1 for g in guides if g['strand']=='-')})\n\n"
        f"Scoring criteria:\n"
        f"  GC content: 40-70% optimal\n"
        f"  No TTTT run (Pol III stop)\n"
        f"  G at position 20 (PAM-prox.)\n"
        f"  No T at position 1\n\n"
        f"{top5_text}"
        f"Off-target note:\n"
        f"  Seed region = positions 12-20\n"
        f"  Mismatches there are fatal\n"
        f"  Use Cas-OFFinder for genome-wide"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.35)

    plt.show()

    print(f"\nTarget: {demo_name} ({len(sequence)} bp)")
    print(f"Guides found: {len(guides)}")
    print("\nTop 5 guides by predicted efficiency:")
    for i, g in enumerate(guides_sorted[:5]):
        print(f"  #{i+1} {g['seq']}  score={g['doench']:.3f}  GC={g['gc']:.2f}  "
              f"pos={g['pos']} ({g['strand']})")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='CRISPR guide RNA design tool')
    parser.add_argument('--demo', default='eGFP', choices=list(DEMO_SEQUENCES.keys()))
    parser.add_argument('--seq', type=str, default=None, help='Custom target sequence')
    args = parser.parse_args()

    print("CRISPR Guide RNA Design & Off-Target Scoring")
    print("=" * 55)
    if args.seq:
        DEMO_SEQUENCES['custom'] = args.seq
        run('custom')
    else:
        run(demo_name=args.demo)
