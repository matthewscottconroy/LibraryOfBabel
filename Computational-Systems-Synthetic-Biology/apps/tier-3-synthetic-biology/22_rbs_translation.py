"""
RBS Design & Translation Rate Explorer
========================================
Tier 3.1 — Synthetic Biology: Genetic Parts & Devices

Concepts demonstrated:
  - Shine-Dalgarno sequence and ribosome binding
  - Salis RBS Calculator thermodynamic model (simplified)
  - Spacing between SD and AUG start codon
  - RBS strength vs. protein expression level
  - Codon usage and codon optimization concept
  - Secondary structure impact on translation initiation
  - Tuning gene expression for metabolic burden trade-off

Usage:
    python 22_rbs_translation.py

Key insight:
    The Salis RBS Calculator uses thermodynamic models of ribosome-mRNA
    binding (ΔG_total = ΔG_mRNA-rRNA + ΔG_standby + ΔG_start - ΔG_mRNA_folding).
    The translation initiation rate ≈ exp(-ΔG_total / RT), so small ΔG
    changes create exponential differences in expression.
    A single nucleotide change can shift expression by 10-100×.
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec


# ── Simplified Thermodynamic Model ───────────────────────────────────────────

# Nearest-neighbor free energy parameters (simplified subset, RNA, 37°C)
NN_PARAMS = {
    'AA': -0.93, 'AU': -1.10, 'AC': -1.33, 'AG': -1.36,
    'UA': -1.33, 'UU': -0.93, 'UC': -2.08, 'UG': -2.11,
    'CA': -2.11, 'CU': -1.36, 'CC': -3.26, 'CG': -3.42,
    'GA': -2.35, 'GU': -2.08, 'GC': -3.42, 'GG': -3.26,
}
RT = 0.616   # kT at 37°C, kcal/mol

def rna_fold_energy(seq):
    """Very simplified: NN sum for single-stranded stem (proxy for folding ΔG)."""
    seq = seq.upper().replace('T', 'U')
    dG = 0.0
    for i in range(len(seq) - 1):
        pair = seq[i:i+2]
        dG += NN_PARAMS.get(pair, -1.5)   # default average
    return dG * 0.1   # scale down — not a real folding model


def sd_ribosome_energy(rbs_seq):
    """
    ΔG of SD:anti-SD interaction.
    Anti-SD is ACCUCCU (3' end of 16S rRNA in E. coli).
    Higher complementarity → more negative ΔG → stronger binding.
    """
    anti_sd = 'ACCUCCU'
    rbs = rbs_seq.upper().replace('T', 'U')

    def complement_score(a, b):
        pairs = {'A': 'U', 'U': 'A', 'G': 'C', 'C': 'G'}
        return 2.0 if pairs.get(a) == b else (-0.5 if a != b else 0.0)

    # Slide anti-SD over RBS, find best alignment
    best_score = 0.0
    for offset in range(-len(anti_sd), len(rbs)):
        score = 0.0
        for i, b in enumerate(anti_sd):
            j = offset + i
            if 0 <= j < len(rbs):
                score += complement_score(rbs[j], b)
        if score < best_score:
            best_score = score
    return best_score * 0.8   # scale to kcal/mol units


def spacing_energy(spacer_len, optimal=7):
    """
    Penalty for non-optimal spacing between SD and AUG.
    Optimal spacing ~ 7 nt in E. coli.
    Gaussian penalty.
    """
    return ((spacer_len - optimal)**2) * 0.3


def predict_translation_rate(rbs_seq, spacer_len, cds_start='ATG', fold_window=20):
    """
    Proxy translation initiation rate using simplified thermodynamics.
    Rate ∝ exp(-ΔG_total / RT)
    """
    # Get energy components
    dG_sd = sd_ribosome_energy(rbs_seq)         # SD binding (negative = good)
    dG_spacing = spacing_energy(spacer_len)     # spacing penalty (positive = bad)
    dG_fold = rna_fold_energy(rbs_seq[-fold_window:] if len(rbs_seq) > fold_window
                               else rbs_seq)    # mRNA folding (negative favors structure = bad for TX)

    # Total: more negative = better translation
    dG_total = dG_sd + dG_spacing * 0.5 - abs(dG_fold) * 0.1

    # Rate proportional to exp(-dG/RT)
    rate = np.exp(-dG_total / RT)
    return rate, dG_sd, dG_spacing, dG_fold, dG_total


# ── RBS Library ──────────────────────────────────────────────────────────────

RBS_LIBRARY = {
    "BBa_B0034 (Strong)":   ("AAAGAGGAGAAA", 7),
    "BBa_B0031 (Medium)":   ("AAGTTAAGAGG", 7),
    "BBa_B0032 (Weak)":     ("AATTTTAAGAGG", 7),
    "BBa_B0030 (Medium-2)": ("TTAACTTGTTTA", 7),
    "Salis RBS1":            ("GGAGAT", 5),
    "Salis RBS2":            ("AGGAGG", 7),
    "Salis RBS3":            ("AAGGAG", 6),
    "Salis RBS4 (Weak)":    ("GCCTCA", 8),
    "Salis RBS5 (Weak)":    ("AATGGG", 9),
    "T7 gene 10 leader":    ("AAATAATTTTGTTTAACTTTAAGAAGGAGATATACA", 8),
}


def run():
    # Score all library RBS
    results = {}
    for name, (rbs_seq, spacer) in RBS_LIBRARY.items():
        rate, dG_sd, dG_sp, dG_fold, dG_tot = predict_translation_rate(rbs_seq, spacer)
        results[name] = {
            'rbs': rbs_seq, 'spacer': spacer, 'rate': rate,
            'dG_sd': dG_sd, 'dG_spacing': dG_sp, 'dG_fold': dG_fold, 'dG_total': dG_tot,
        }

    # Normalize to BBa_B0034 (strong reference)
    ref_rate = results.get("BBa_B0034 (Strong)", {'rate': 1.0})['rate']
    for r in results.values():
        r['rel_rate'] = r['rate'] / ref_rate

    # Sweep spacer length
    spacer_lengths = range(3, 16)
    spacer_rates = []
    for sp in spacer_lengths:
        r, *_ = predict_translation_rate('AGGAGG', sp)
        spacer_rates.append(r)

    # Sweep SD complementarity
    sd_seqs = {
        'Perfect AGGAGG':  'AGGAGG',
        'Strong AAGGAG':   'AAGGAG',
        'Moderate AGGAAT': 'AGGAAT',
        'Weak GAAGAT':     'GAAGAT',
        'Very weak ATTCAT':'ATTCAT',
        'None AAAAAA':     'AAAAAA',
    }
    sd_rates = {name: predict_translation_rate(seq, 7)[0] for name, seq in sd_seqs.items()}

    # ── Figure ────────────────────────────────────────────────────────────────
    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle('RBS Design & Translation Rate Explorer  (Salis Model, Simplified)',
                 color='white', fontsize=12, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_lib    = fig.add_subplot(gs[0, :2])
    ax_spacer = fig.add_subplot(gs[1, 0])
    ax_sd     = fig.add_subplot(gs[1, 1])
    ax_info   = fig.add_subplot(gs[:, 2])

    for ax in [ax_lib, ax_spacer, ax_sd, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── RBS library comparison ────────────────────────────────────────────────
    sorted_rbs = sorted(results.items(), key=lambda x: x[1]['rel_rate'], reverse=True)
    names_sorted = [n for n, _ in sorted_rbs]
    rates_sorted = [r['rel_rate'] for _, r in sorted_rbs]

    bar_colors = ['#50fa7b' if r >= 0.5 else '#f5a623' if r >= 0.1 else '#ff5555'
                  for r in rates_sorted]
    bars = ax_lib.bar(range(len(names_sorted)), rates_sorted,
                      color=bar_colors, edgecolor='white', width=0.7)
    ax_lib.set_xticks(range(len(names_sorted)))
    ax_lib.set_xticklabels([n[:20] for n in names_sorted], rotation=35,
                            ha='right', color='white', fontsize=7.5)
    ax_lib.set_ylabel('Relative translation rate', color='white')
    ax_lib.set_title('RBS Library: Predicted Relative Translation Rates',
                     color='white', fontsize=9)
    ax_lib.axhline(1.0, color='#888', lw=0.8, ls='--', alpha=0.6)
    ax_lib.tick_params(colors='white')
    ax_lib.set_yscale('log')
    for sp in ax_lib.spines.values():
        sp.set_edgecolor('#888')
    for bar, rate in zip(bars, rates_sorted):
        ax_lib.text(bar.get_x() + bar.get_width()/2,
                    bar.get_height() * 1.1, f'{rate:.2f}×',
                    ha='center', va='bottom', fontsize=6, color='white')

    # ── Spacer length effect ──────────────────────────────────────────────────
    ax_spacer.plot(list(spacer_lengths), spacer_rates, '-o',
                   color='#8be9fd', lw=2.5, ms=7, markeredgecolor='white')
    opt_idx = np.argmax(spacer_rates)
    ax_spacer.axvline(list(spacer_lengths)[opt_idx], color='#50fa7b', lw=1.5, ls='--',
                      label=f'Optimal spacing = {list(spacer_lengths)[opt_idx]} nt')
    ax_spacer.set_xlabel('SD-AUG spacing (nt)', color='white')
    ax_spacer.set_ylabel('Predicted translation rate', color='white')
    ax_spacer.set_title('Effect of SD-AUG Spacing\n(RBS = AGGAGG)', color='white', fontsize=9)
    ax_spacer.tick_params(colors='white')
    ax_spacer.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_spacer.spines.values():
        sp.set_edgecolor('#888')

    # ── SD complementarity ────────────────────────────────────────────────────
    sd_names = list(sd_rates.keys())
    sd_rate_vals = [sd_rates[n] for n in sd_names]
    ref_sd = max(sd_rate_vals)
    sd_rel = [v / ref_sd for v in sd_rate_vals]

    sd_colors = ['#50fa7b' if r >= 0.5 else '#f5a623' if r >= 0.1 else '#ff5555'
                 for r in sd_rel]
    ax_sd.barh(range(len(sd_names)), sd_rel, color=sd_colors, edgecolor='white')
    ax_sd.set_yticks(range(len(sd_names)))
    ax_sd.set_yticklabels(sd_names, color='white', fontsize=8)
    ax_sd.set_xlabel('Relative translation rate', color='white')
    ax_sd.set_title('Effect of SD Complementarity\n(anti-SD = ACCUCCU)',
                    color='white', fontsize=9)
    ax_sd.tick_params(colors='white')
    for sp in ax_sd.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    top3 = sorted_rbs[:3]
    info = (
        f"Salis RBS Calculator (simplified)\n\n"
        f"Model:\n"
        f"  Rate ∝ exp(-ΔG_total / RT)\n\n"
        f"  ΔG_total = ΔG_SD-antiSD\n"
        f"           + ΔG_spacing_penalty\n"
        f"           - ΔG_mRNA_unfolding\n\n"
        f"Anti-SD sequence (E. coli 16S):\n"
        f"  3'-ACCUCCU-5'\n\n"
        f"Optimal E. coli spacing: ~7 nt\n\n"
        f"Top 3 from library:\n"
    )
    for name, res in top3:
        info += (f"  {name[:22]}\n"
                 f"    rate = {res['rel_rate']:.2f}×\n"
                 f"    ΔG_total = {res['dG_total']:.2f} kcal/mol\n\n")

    info += (
        f"Key rules:\n"
        f"  1. Maximize SD complementarity\n"
        f"  2. Optimal spacing: 5-11 nt\n"
        f"  3. Avoid stable mRNA structures\n"
        f"     near start codon\n"
        f"  4. A/U rich around AUG\n"
        f"  5. Avoid internal AUG codons"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print("\nRBS Library Results (relative to BBa_B0034):")
    for name, res in sorted_rbs:
        print(f"  {name[:30]:30s}: {res['rel_rate']:.3f}× "
              f"(ΔG_total={res['dG_total']:.2f}, spacer={res['spacer']})")


if __name__ == '__main__':
    print("RBS Design & Translation Rate Explorer")
    print("=" * 50)
    run()
