"""
Flux Balance Analysis: Toy Metabolic Network
==============================================
Tier 2.2 — Systems Biology: Metabolic Modeling

Concepts demonstrated:
  - Stoichiometric matrix S and steady-state constraint S·v = 0
  - Flux Balance Analysis (FBA) as a linear program
  - Objective: maximize biomass or product flux
  - Flux variability analysis (FVA)
  - Knockouts: zeroing reactions and their effect on growth
  - Shadow prices / reduced costs
  - E. coli core network (simplified)

Usage:
    python 11_fba_metabolic.py [--obj OBJECTIVE]

This script implements FBA from scratch (no COBRApy) so that
the linear algebra is fully transparent to the student.

Key insight:
    FBA exploits the fact that at metabolic steady state, the net
    production of every metabolite is zero: S·v = 0.  Given bounds
    on fluxes (from enzyme capacity / nutrient availability) and an
    objective, FBA finds the optimal flux distribution using an LP.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.optimize import linprog


# ── Toy Network Definition ────────────────────────────────────────────────────
# A simplified 'core' E. coli-like network.
# Metabolites: GLC, PYR, ACoA, OAA, ATP, CO2, BIOMASS, EtOH

# Reactions:
#  0. GLC uptake:         ∅ → GLC         (exchange)
#  1. Glycolysis:         GLC → 2 PYR + 2 ATP
#  2. PDH:                PYR → ACoA + CO2
#  3. TCA (simplified):   ACoA + OAA → 2 CO2 + 3 ATP
#  4. Biomass:            3 ATP → BIOMASS
#  5. ATP maintenance:    ATP → ∅
#  6. CO2 exchange:       CO2 → ∅           (exchange)
#  7. Acetate overflow:   ACoA → EtOH + ATP
#  8. EtOH exchange:      EtOH → ∅
#  9. OAA anaplerosis:    PYR + ATP → OAA    (replenish TCA)

METABOLITES = ['GLC', 'PYR', 'ACoA', 'OAA', 'ATP', 'CO2', 'BIOMASS', 'EtOH']
REACTIONS   = ['GLC_ex', 'Glycolysis', 'PDH', 'TCA', 'Biomass',
               'ATPmaint', 'CO2_ex', 'OvflwAc', 'EtOH_ex', 'Anapl']

# Stoichiometric matrix S[metabolite, reaction]
# Rows = metabolites, columns = reactions
S = np.array([
#   GLC_ex  Glycol  PDH  TCA  Biom  ATPm  CO2ex  OvflAc  EtOHex  Anapl
    [ 1,    -1,     0,   0,    0,    0,    0,     0,      0,      0  ],  # GLC
    [ 0,     2,    -1,   0,    0,    0,    0,     0,      0,     -1  ],  # PYR
    [ 0,     0,     1,  -1,    0,    0,    0,    -1,      0,      0  ],  # ACoA
    [ 0,     0,     0,  -1,    0,    0,    0,     0,      0,      1  ],  # OAA
    [ 0,     2,     0,   3,   -3,   -1,    0,     1,      0,     -1  ],  # ATP
    [ 0,     0,     1,   2,    0,    0,   -1,     0,      0,      0  ],  # CO2
    [ 0,     0,     0,   0,    1,    0,    0,     0,      0,      0  ],  # BIOMASS
    [ 0,     0,     0,   0,    0,    0,    0,     1,     -1,      0  ],  # EtOH
], dtype=float)

# Flux bounds [lb, ub] for each reaction
BOUNDS = [
    (-10, 0),    # GLC uptake (negative = import)
    (0,  1000),  # Glycolysis
    (0,  1000),  # PDH
    (0,  1000),  # TCA
    (0,  1000),  # Biomass
    (0,  1000),  # ATPmaint
    (0,  1000),  # CO2 export
    (0,  1000),  # Overflow acetate
    (0,  1000),  # EtOH export
    (0,  1000),  # Anaplerosis
]

N_MET = len(METABOLITES)
N_RXN = len(REACTIONS)

# Objective indices
OBJ_INDICES = {
    'biomass':  4,   # maximize BIOMASS flux
    'ethanol':  7,   # maximize acetate overflow (proxy for EtOH)
    'atp':      4,   # maximize ATP production via biomass
}


# ── FBA via LP ────────────────────────────────────────────────────────────────

def run_fba(S, bounds, obj_idx, maximize=True):
    """
    Minimize c·v subject to S·v = 0 and lb ≤ v ≤ ub.
    Uses scipy.optimize.linprog with revised simplex.
    """
    n_rxn = S.shape[1]
    c = np.zeros(n_rxn)
    c[obj_idx] = -1.0 if maximize else 1.0

    lb = np.array([b[0] for b in bounds])
    ub = np.array([b[1] for b in bounds])

    result = linprog(
        c,
        A_eq=S,
        b_eq=np.zeros(S.shape[0]),
        bounds=list(zip(lb, ub)),
        method='highs',
    )
    if result.success:
        return result.x, -result.fun if maximize else result.fun, result
    return None, None, result


def fva(S, bounds, target_idx, fraction=0.99):
    """
    Flux Variability Analysis: for each reaction, find min and max flux
    while maintaining obj ≥ fraction × optimal.
    """
    v_opt, obj_opt, _ = run_fba(S, bounds, target_idx)
    if v_opt is None:
        return None, None

    n_rxn = S.shape[1]
    v_min = np.zeros(n_rxn)
    v_max = np.zeros(n_rxn)

    # Add constraint: obj_reaction ≥ fraction * optimal
    S_aug = np.vstack([S, np.eye(n_rxn)[target_idx]])
    b_aug = np.zeros(S.shape[0] + 1)
    b_aug[-1] = fraction * obj_opt

    # Use inequality constraints
    for i in range(n_rxn):
        c_min = np.zeros(n_rxn);  c_min[i] = 1.0
        c_max = np.zeros(n_rxn);  c_max[i] = -1.0
        lb = [b[0] for b in bounds]
        ub = [b[1] for b in bounds]
        # Add biomass lower bound
        lb_copy = lb.copy();  lb_copy[target_idx] = fraction * obj_opt

        res_min = linprog(c_min, A_eq=S, b_eq=np.zeros(S.shape[0]),
                          bounds=list(zip(lb_copy, ub)), method='highs')
        res_max = linprog(c_max, A_eq=S, b_eq=np.zeros(S.shape[0]),
                          bounds=list(zip(lb_copy, ub)), method='highs')
        v_min[i] = res_min.fun if res_min.success else 0
        v_max[i] = -res_max.fun if res_max.success else 0

    return v_min, v_max


def knockout_scan(S, bounds, obj_idx):
    """Single reaction knockout: set flux to 0 and reoptimize."""
    v_wt, obj_wt, _ = run_fba(S, bounds, obj_idx)
    results = {}
    for i, rxn in enumerate(REACTIONS):
        ko_bounds = list(bounds)
        ko_bounds[i] = (0, 0)
        v_ko, obj_ko, _ = run_fba(S, ko_bounds, obj_idx)
        results[rxn] = obj_ko if obj_ko is not None else 0.0
    return obj_wt, results


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(objective='biomass'):
    obj_idx = OBJ_INDICES.get(objective, 4)
    v_opt, obj_opt, result = run_fba(S, BOUNDS, obj_idx)
    v_min_fva, v_max_fva = fva(S, BOUNDS, obj_idx)
    obj_wt, ko_results = knockout_scan(S, BOUNDS, obj_idx)

    if v_opt is None:
        print("FBA failed to find a feasible solution.")
        return

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Flux Balance Analysis — Toy E. coli Core  (objective: {objective})',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_flux  = fig.add_subplot(gs[0, :2])
    ax_fva   = fig.add_subplot(gs[1, :2])
    ax_ko    = fig.add_subplot(gs[0, 2])
    ax_info  = fig.add_subplot(gs[1, 2])

    for ax in [ax_flux, ax_fva, ax_ko, ax_info]:
        ax.set_facecolor('#0f3460')

    x = np.arange(N_RXN)
    bar_colors = ['#50fa7b' if v > 0 else '#ff5555' for v in v_opt]
    bar_colors[obj_idx] = '#f5a623'

    # ── Optimal flux distribution ─────────────────────────────────────────────
    bars = ax_flux.bar(x, v_opt, color=bar_colors, edgecolor='white', width=0.7)
    ax_flux.axhline(0, color='#888', lw=0.8)
    ax_flux.set_xticks(x)
    ax_flux.set_xticklabels(REACTIONS, rotation=35, ha='right', color='white', fontsize=8)
    ax_flux.set_ylabel('Flux (mmol/gDW/h)', color='white')
    ax_flux.set_title(f'Optimal Flux Distribution  (obj={objective}, value={obj_opt:.3f})',
                      color='white', fontsize=9)
    ax_flux.tick_params(colors='white')
    for sp in ax_flux.spines.values():
        sp.set_edgecolor('#888')

    # Annotate bars
    for bar, v in zip(bars, v_opt):
        if abs(v) > 0.01:
            ax_flux.text(bar.get_x() + bar.get_width()/2,
                         bar.get_height() + 0.02, f'{v:.2f}',
                         ha='center', va='bottom', fontsize=6.5, color='white')

    # ── FVA ranges ────────────────────────────────────────────────────────────
    if v_min_fva is not None:
        range_widths = v_max_fva - v_min_fva
        fva_colors = ['#8be9fd' if w < 0.01 else '#ff79c6' for w in range_widths]
        ax_fva.bar(x, range_widths, bottom=v_min_fva,
                   color=fva_colors, edgecolor='white', width=0.7, alpha=0.85)
        ax_fva.plot(x, v_opt, 'wo', ms=5, zorder=5, label='Optimal flux')
        ax_fva.axhline(0, color='#888', lw=0.8)
        ax_fva.set_xticks(x)
        ax_fva.set_xticklabels(REACTIONS, rotation=35, ha='right', color='white', fontsize=8)
        ax_fva.set_ylabel('Flux range', color='white')
        ax_fva.set_title('Flux Variability Analysis  (blue=fixed, pink=variable)',
                         color='white', fontsize=9)
        ax_fva.tick_params(colors='white')
        ax_fva.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax_fva.spines.values():
            sp.set_edgecolor('#888')

    # ── Knockout scan ─────────────────────────────────────────────────────────
    ko_rxns = list(ko_results.keys())
    ko_vals = list(ko_results.values())
    relative = [v / obj_wt if obj_wt else 0 for v in ko_vals]
    ko_colors = ['#50fa7b' if r > 0.99 else '#f5a623' if r > 0.1 else '#ff5555'
                 for r in relative]
    ax_ko.barh(range(len(ko_rxns)), relative, color=ko_colors, edgecolor='white')
    ax_ko.axvline(1.0, color='#888', lw=1.2, ls='--', label='WT growth')
    ax_ko.axvline(0.5, color='#f5a623', lw=0.8, ls=':', alpha=0.7)
    ax_ko.set_yticks(range(len(ko_rxns)))
    ax_ko.set_yticklabels(ko_rxns, color='white', fontsize=8)
    ax_ko.set_xlabel('Growth fraction (KO/WT)', color='white')
    ax_ko.set_title('Knockout Scan', color='white', fontsize=9)
    ax_ko.tick_params(colors='white')
    ax_ko.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_ko.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    essential = [r for r, rel in zip(ko_rxns, relative) if rel < 0.01]

    info = (
        f"Network: Toy E. coli core\n"
        f"  {N_MET} metabolites\n"
        f"  {N_RXN} reactions\n\n"
        f"FBA result:\n"
        f"  Objective: {objective}\n"
        f"  Optimal value: {obj_opt:.4f}\n\n"
        f"Flux distribution:\n"
        + '\n'.join(f"  {r}: {v:.3f}" for r, v in zip(REACTIONS, v_opt)) +
        f"\n\nEssential reactions (KO lethal):\n"
        + '\n'.join(f"  {r}" for r in essential) +
        f"\n\nS·v=0 satisfied: {np.allclose(S @ v_opt, 0, atol=1e-6)}"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nFBA: objective={objective}, optimal={obj_opt:.4f}")
    print("Flux distribution:")
    for rxn, v in zip(REACTIONS, v_opt):
        print(f"  {rxn:15s}: {v:.4f}")
    print(f"\nS·v = 0: {np.allclose(S @ v_opt, 0, atol=1e-6)}")
    print(f"\nEssential reactions: {essential}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='FBA toy metabolic network')
    parser.add_argument('--obj', default='biomass',
                        choices=['biomass', 'ethanol'])
    args = parser.parse_args()

    print("Flux Balance Analysis: Toy Metabolic Network")
    print("=" * 55)
    run(objective=args.obj)
