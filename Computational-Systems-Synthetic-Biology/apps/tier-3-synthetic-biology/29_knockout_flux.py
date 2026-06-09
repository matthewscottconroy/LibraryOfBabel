"""
Gene Knockout Flux Balance Analysis
=====================================
Tier 3.4 — Synthetic Biology: Metabolic Engineering

Concepts demonstrated:
  - Flux balance analysis (FBA): linear programming on stoichiometric matrix
  - Steady-state assumption: Sv = 0
  - Objective function: maximise biomass (growth) or target metabolite
  - Gene knockouts: constrain reaction flux to zero
  - Optimal vs feasible solutions
  - Yield trade-offs: growth vs product

Usage:
    python 29_knockout_flux.py

Controls:
    Checkboxes: knock out individual reactions (set upper bound = 0)
    Radio: Objective — "Maximise Growth" or "Maximise Ethanol"
    Button: "Reset" — restore all reactions
    Slider: "Glucose uptake rate" — controls v_GLC (carbon input)
"""

import numpy as np
import matplotlib
try:
    matplotlib.use('TkAgg')
except Exception:
    pass  # fall back to whatever backend is available
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors
from matplotlib.widgets import CheckButtons, RadioButtons, Slider, Button
from scipy.optimize import linprog

# ---------------------------------------------------------------------------
# Network definition
# ---------------------------------------------------------------------------
REACTIONS = {
    'v_GLC':  {
        'name': 'Glucose uptake',
        'reversible': False,
        'gene': 'ptsG',
        'stoich': {'G6P': 1, 'ATP': -1},
        'bounds': (0, 10),
        'knockable': False,
    },
    'v_PGI':  {
        'name': 'Phosphoglucose isomerase',
        'reversible': True,
        'gene': 'pgi',
        'stoich': {'G6P': -1, 'F6P': 1},
        'bounds': (-10, 10),
        'knockable': True,
    },
    'v_PFK':  {
        'name': 'Phosphofructokinase',
        'reversible': False,
        'gene': 'pfkA',
        'stoich': {'F6P': -1, 'GAP': 2, 'ATP': -1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_PYK':  {
        'name': 'Pyruvate kinase',
        'reversible': False,
        'gene': 'pykF',
        'stoich': {'GAP': -1, 'PYR': 1, 'ATP': 1, 'NADH': 1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_PDH':  {
        'name': 'Pyruvate dehydrogenase',
        'reversible': False,
        'gene': 'aceE',
        'stoich': {'PYR': -1, 'AcCoA': 1, 'NADH': 1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_CS':   {
        'name': 'Citrate synthase',
        'reversible': False,
        'gene': 'gltA',
        'stoich': {'AcCoA': -1, 'OAA': -1, 'CIT': 1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_MDH':  {
        'name': 'Malate dehydrogenase',
        'reversible': True,
        'gene': 'mdh',
        'stoich': {'CIT': -1, 'OAA': 1, 'NADH': 1},
        'bounds': (-10, 10),
        'knockable': True,
    },
    'v_ADH':  {
        'name': 'Alcohol dehydrogenase',
        'reversible': False,
        'gene': 'adhE',
        'stoich': {'AcCoA': -1, 'EtOH_int': 1, 'NADH': -1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_ETH':  {
        'name': 'Ethanol export',
        'reversible': False,
        'gene': 'transport',
        'stoich': {'EtOH_int': -1},
        'bounds': (0, 10),
        'knockable': False,
    },
    'v_ATPM': {
        'name': 'ATP maintenance',
        'reversible': False,
        'gene': 'atpA',
        'stoich': {'ATP': -1},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_RESP': {
        'name': 'Respiration (NADH→ATP)',
        'reversible': False,
        'gene': 'nuoA',
        'stoich': {'NADH': -1, 'ATP': 2},
        'bounds': (0, 10),
        'knockable': True,
    },
    'v_BIOM': {
        'name': 'Biomass (growth)',
        'reversible': False,
        'gene': 'growth',
        'stoich': {'ATP': -2, 'NADH': -1},
        'bounds': (0, 10),
        'knockable': False,
    },
}

METABOLITES = ['G6P', 'F6P', 'GAP', 'PYR', 'AcCoA', 'OAA', 'CIT', 'ATP', 'NADH', 'EtOH_int']
RXN_IDS     = list(REACTIONS.keys())

# Knockable reactions (shown in checkboxes)
KNOCKABLE_RXNS = [r for r in RXN_IDS if REACTIONS[r]['knockable']]

# ---------------------------------------------------------------------------
# FBA
# ---------------------------------------------------------------------------
def run_fba(knockouts, objective_rxn, glc_uptake):
    """Run flux balance analysis with scipy.optimize.linprog (HiGHS)."""
    n_rxn = len(RXN_IDS)
    n_met = len(METABOLITES)

    # Build stoichiometric matrix
    S_mat = np.zeros((n_met, n_rxn))
    for j, rxn_id in enumerate(RXN_IDS):
        for met, coeff in REACTIONS[rxn_id]['stoich'].items():
            i = METABOLITES.index(met)
            S_mat[i, j] = coeff

    # Objective: minimise negative objective reaction flux
    c = np.zeros(n_rxn)
    c[RXN_IDS.index(objective_rxn)] = -1.0

    # Bounds
    bounds = []
    for rxn_id in RXN_IDS:
        lb, ub = REACTIONS[rxn_id]['bounds']
        if rxn_id == 'v_GLC':
            ub = float(glc_uptake)
        if rxn_id in knockouts:
            lb, ub = 0.0, 0.0
        bounds.append((lb, ub))

    result = linprog(
        c,
        A_eq=S_mat,
        b_eq=np.zeros(n_met),
        bounds=bounds,
        method='highs',
    )

    if result.success:
        fluxes = dict(zip(RXN_IDS, result.x))
        obj_val = float(-result.fun)
        return fluxes, obj_val
    else:
        return {r: 0.0 for r in RXN_IDS}, 0.0


# ---------------------------------------------------------------------------
# Network layout positions (for metabolite nodes)
# ---------------------------------------------------------------------------
NET_POS = {
    'G6P':      (0.12, 0.82),
    'F6P':      (0.12, 0.62),
    'GAP':      (0.12, 0.42),
    'PYR':      (0.12, 0.22),
    'AcCoA':    (0.42, 0.22),
    'EtOH_int': (0.72, 0.22),
    'OAA':      (0.42, 0.52),
    'CIT':      (0.72, 0.52),
    'ATP':      (0.80, 0.82),
    'NADH':     (0.80, 0.70),
}

# Map each reaction to (source_met, dest_met) for drawing arrows
# We pick the "main" substrate → product pair for display
RXN_ARROW = {
    'v_GLC':  (None,       'G6P'),       # external → G6P
    'v_PGI':  ('G6P',      'F6P'),
    'v_PFK':  ('F6P',      'GAP'),
    'v_PYK':  ('GAP',      'PYR'),
    'v_PDH':  ('PYR',      'AcCoA'),
    'v_CS':   ('AcCoA',    'CIT'),
    'v_MDH':  ('CIT',      'OAA'),
    'v_ADH':  ('AcCoA',    'EtOH_int'),
    'v_ETH':  ('EtOH_int', None),        # EtOH_int → external
    'v_ATPM': ('ATP',      None),
    'v_RESP': ('NADH',     'ATP'),
    'v_BIOM': (None,       None),        # biomass handled separately
}

MET_LABELS = {
    'G6P': 'G6P', 'F6P': 'F6P', 'GAP': 'GAP', 'PYR': 'PYR',
    'AcCoA': 'AcCoA', 'EtOH_int': 'EtOH', 'OAA': 'OAA',
    'CIT': 'CIT', 'ATP': 'ATP', 'NADH': 'NADH',
}

# ---------------------------------------------------------------------------
# Network drawing
# ---------------------------------------------------------------------------
def draw_network(ax, fluxes, knockouts, max_flux=10.0):
    ax.cla()
    ax.set_facecolor('#0f3460')
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis('off')
    ax.set_title('Metabolic Network', color='white', fontsize=11, fontweight='bold', pad=4)

    # Colour map for flux: dark → bright cyan
    flux_cmap = mcolors.LinearSegmentedColormap.from_list(
        'flux', ['#16213e', '#8be9fd', '#50fa7b'], N=256
    )
    norm = mcolors.Normalize(vmin=0, vmax=max(max_flux, 0.1))

    # Draw metabolite nodes
    for met, (x, y) in NET_POS.items():
        ax.plot(x, y, 'o', markersize=18, color='#1a1a2e',
                markeredgecolor='#8be9fd', markeredgewidth=1.5, zorder=5)
        ax.text(x, y, MET_LABELS[met],
                ha='center', va='center', color='white', fontsize=7,
                fontweight='bold', zorder=6)

    # Draw reaction arrows
    for rxn_id, (src, dst) in RXN_ARROW.items():
        if rxn_id == 'v_BIOM':
            continue
        flux_val = abs(fluxes.get(rxn_id, 0.0))
        knocked  = rxn_id in knockouts

        if knocked:
            arrow_color = '#e94560'
            lw = 1.0
            alpha = 0.5
        else:
            arrow_color = flux_cmap(norm(flux_val))
            lw = max(1.0, 3.5 * flux_val / max(max_flux, 0.1))
            alpha = 0.9

        # Determine arrow endpoints
        if src is None:
            x1, y1 = NET_POS[dst][0] - 0.12, NET_POS[dst][1]
        else:
            x1, y1 = NET_POS[src]

        if dst is None:
            x2, y2 = NET_POS[src][0] + 0.10, NET_POS[src][1] - 0.10
        else:
            x2, y2 = NET_POS[dst]

        # Offset arrow slightly to avoid overlap
        dx, dy = x2 - x1, y2 - y1
        dist = np.sqrt(dx**2 + dy**2) + 1e-9
        nx, ny = -dy / dist, dx / dist  # normal
        offset = 0.018
        x1o = x1 + nx * offset
        y1o = y1 + ny * offset
        x2o = x2 + nx * offset
        y2o = y2 + ny * offset

        ax.annotate(
            '', xy=(x2o, y2o), xytext=(x1o, y1o),
            arrowprops=dict(
                arrowstyle='->', color=arrow_color,
                lw=lw, alpha=alpha,
            ),
            zorder=4,
        )

        # Flux label at midpoint
        mx, my = (x1o + x2o) / 2 + nx * 0.03, (y1o + y2o) / 2 + ny * 0.03
        label = f'{REACTIONS[rxn_id]["gene"]}\n{flux_val:.2f}'
        ax.text(mx, my, label,
                ha='center', va='center', color='white' if not knocked else '#e94560',
                fontsize=5.5, alpha=0.9, zorder=7)

    # Biomass arrow (separate: comes off ATP/NADH)
    bx, by = 0.42, 0.82
    ax.annotate(
        '', xy=(bx, by), xytext=(NET_POS['ATP'][0] - 0.05, NET_POS['ATP'][1] - 0.05),
        arrowprops=dict(
            arrowstyle='->',
            color='#ff79c6' if 'v_BIOM' not in knockouts else '#e94560',
            lw=max(1.0, 3.0 * abs(fluxes.get('v_BIOM', 0.0)) / max(max_flux, 0.1)),
            alpha=0.85,
        ),
        zorder=4,
    )
    ax.text(bx, by + 0.04, f'BIOMASS\n{abs(fluxes.get("v_BIOM", 0.0)):.2f}',
            ha='center', va='bottom', color='#ff79c6', fontsize=6.5, fontweight='bold')

    # External glucose source
    ax.annotate(
        '', xy=(NET_POS['G6P'][0], NET_POS['G6P'][1] + 0.08),
        xytext=(NET_POS['G6P'][0], NET_POS['G6P'][1] + 0.15),
        arrowprops=dict(arrowstyle='->', color='#f5a623', lw=2.0),
        zorder=4,
    )
    ax.text(NET_POS['G6P'][0], NET_POS['G6P'][1] + 0.16,
            f'Glc\n{abs(fluxes.get("v_GLC", 0.0)):.2f}',
            ha='center', va='bottom', color='#f5a623', fontsize=6.5)

    # Ethanol export label
    ex, ey = NET_POS['EtOH_int']
    ax.annotate(
        '', xy=(ex + 0.12, ey - 0.08), xytext=(ex + 0.03, ey),
        arrowprops=dict(arrowstyle='->', color='#50fa7b', lw=1.5, alpha=0.8),
        zorder=4,
    )
    ax.text(ex + 0.14, ey - 0.10,
            f'EtOH\nexport\n{abs(fluxes.get("v_ETH", 0.0)):.2f}',
            ha='left', va='top', color='#50fa7b', fontsize=6.0)


# ---------------------------------------------------------------------------
# Bar chart drawing
# ---------------------------------------------------------------------------
def draw_bar_flux(ax, fluxes, knockouts):
    ax.cla()
    ax.set_facecolor('#16213e')
    for spine in ax.spines.values():
        spine.set_color('#8be9fd')
    ax.tick_params(colors='white', labelsize=7)
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')
    ax.title.set_color('white')

    vals   = [abs(fluxes.get(r, 0.0)) for r in RXN_IDS]
    colors = ['#e94560' if r in knockouts else '#8be9fd' for r in RXN_IDS]
    x_pos  = np.arange(len(RXN_IDS))

    bars = ax.bar(x_pos, vals, color=colors, edgecolor='#1a1a2e', linewidth=0.5, zorder=3)
    ax.set_xticks(x_pos)
    ax.set_xticklabels(
        [REACTIONS[r]['gene'] for r in RXN_IDS],
        rotation=45, ha='right', fontsize=7,
    )
    ax.set_ylabel('Flux', fontsize=9)
    ax.set_title('Reaction Fluxes', fontsize=10, fontweight='bold')
    ax.set_ylim(0, max(max(vals) * 1.2, 0.5))
    ax.grid(axis='y', color='#8be9fd', alpha=0.15, zorder=0)

    # Value labels on bars
    for bar, val in zip(bars, vals):
        if val > 0.01:
            ax.text(
                bar.get_x() + bar.get_width() / 2,
                bar.get_height() + 0.05,
                f'{val:.2f}',
                ha='center', va='bottom', color='white', fontsize=6,
            )


# ---------------------------------------------------------------------------
# Objective panel drawing
# ---------------------------------------------------------------------------
def draw_objectives(ax, fluxes, objective_rxn, max_growth=3.0, max_etoh=10.0):
    ax.cla()
    ax.set_facecolor('#16213e')
    for spine in ax.spines.values():
        spine.set_color('#8be9fd')
    ax.tick_params(colors='white', labelsize=9)
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')
    ax.title.set_color('white')

    growth = abs(fluxes.get('v_BIOM', 0.0))
    etoh   = abs(fluxes.get('v_ETH',  0.0))

    labels  = ['Growth\n(v_BIOM)', 'Ethanol\n(v_ETH)']
    values  = [growth, etoh]
    maxvals = [max_growth, max_etoh]
    colors  = [
        '#50fa7b' if objective_rxn == 'v_BIOM' else '#555577',
        '#50fa7b' if objective_rxn == 'v_ETH'  else '#555577',
    ]

    x_pos = np.array([0, 1])
    bars_bg = ax.bar(x_pos, maxvals, color='#0f3460', edgecolor='#8be9fd',
                     linewidth=0.8, zorder=2, width=0.55)
    bars_fg = ax.bar(x_pos, values,  color=colors, edgecolor='none',
                     linewidth=0, zorder=3, width=0.55)

    ax.set_xticks(x_pos)
    ax.set_xticklabels(labels, fontsize=9, color='white')
    ax.set_ylabel('Flux units', fontsize=9)
    ax.set_title('Objective Fluxes', fontsize=10, fontweight='bold')
    ax.set_ylim(0, max(max(maxvals) * 1.15, 0.5))
    ax.grid(axis='y', color='#8be9fd', alpha=0.15, zorder=0)

    for i, (val, mv) in enumerate(zip(values, maxvals)):
        ax.text(x_pos[i], val + 0.05, f'{val:.2f}',
                ha='center', va='bottom', color='white', fontsize=10, fontweight='bold')
        pct = 100 * val / mv if mv > 0 else 0
        ax.text(x_pos[i], -0.25, f'{pct:.0f}% max',
                ha='center', va='top', color='#8be9fd', fontsize=8)


# ---------------------------------------------------------------------------
# Main application
# ---------------------------------------------------------------------------
def build_app():
    # Figure
    fig = plt.figure(figsize=(16, 9))
    fig.patch.set_facecolor('#1a1a2e')

    from matplotlib.gridspec import GridSpec
    gs = GridSpec(
        2, 3,
        figure=fig,
        left=0.03, right=0.73,
        top=0.93, bottom=0.08,
        wspace=0.10, hspace=0.35,
        height_ratios=[2, 1],
        width_ratios=[1.4, 1.2, 0.01],
    )
    ax_network   = fig.add_subplot(gs[:, 0])
    ax_bar_flux  = fig.add_subplot(gs[0, 1])
    ax_objectives= fig.add_subplot(gs[1, 1])

    for ax in [ax_network, ax_bar_flux, ax_objectives]:
        ax.set_facecolor('#0f3460')

    # ---- Control widgets (right column, manual placement) ----
    ctrl_left = 0.76
    ctrl_w    = 0.22

    # CheckButtons for knockouts
    ax_check = fig.add_axes([ctrl_left, 0.38, ctrl_w, 0.52])
    ax_check.set_facecolor('#0f3460')
    check_labels = [f'{r}  [{REACTIONS[r]["gene"]}]' for r in KNOCKABLE_RXNS]
    check_init   = [False] * len(KNOCKABLE_RXNS)
    chk_buttons  = CheckButtons(ax_check, check_labels, check_init)
    # Style checkboxes
    for rect in chk_buttons.rectangles:
        rect.set_facecolor('#16213e')
        rect.set_edgecolor('#8be9fd')
    for text in chk_buttons.labels:
        text.set_color('white')
        text.set_fontsize(8)
    ax_check.set_title('Knock out reactions:', color='white', fontsize=9, pad=2)

    # RadioButtons for objective
    ax_radio = fig.add_axes([ctrl_left, 0.25, ctrl_w, 0.10])
    ax_radio.set_facecolor('#0f3460')
    radio_obj = RadioButtons(ax_radio, ['Maximise Growth', 'Maximise Ethanol'])
    for circle in radio_obj.circles:
        circle.set_facecolor('#16213e')
        circle.set_edgecolor('#8be9fd')
    for text in radio_obj.labels:
        text.set_color('white')
        text.set_fontsize(9)
    ax_radio.set_title('Objective:', color='white', fontsize=9, pad=2)

    # Glucose slider
    ax_sl_glc = fig.add_axes([ctrl_left, 0.17, ctrl_w, 0.025])
    ax_sl_glc.set_facecolor('#16213e')
    sl_glc = Slider(ax_sl_glc, 'Glucose uptake', 0.0, 10.0, valinit=10.0, color='#f5a623')
    sl_glc.label.set_color('white')
    sl_glc.valtext.set_color('white')

    # Reset button
    ax_btn_reset = fig.add_axes([ctrl_left + 0.03, 0.10, ctrl_w * 0.5, 0.05])
    btn_reset = Button(ax_btn_reset, 'Reset All', color='#16213e', hovercolor='#e94560')
    btn_reset.label.set_color('white')

    # Info text
    ax_info = fig.add_axes([ctrl_left, 0.02, ctrl_w, 0.07])
    ax_info.set_facecolor('#0f3460')
    ax_info.axis('off')
    info_text = ax_info.text(
        0.05, 0.95, '',
        transform=ax_info.transAxes,
        color='white', fontsize=9, va='top', family='monospace',
    )

    # ---- State ----
    state = {
        'knockouts':  set(),
        'objective':  'v_BIOM',
        'glc_uptake': 10.0,
    }

    # ------------------------------------------------------------------ #
    def update():
        ko  = state['knockouts']
        obj = state['objective']
        glc = state['glc_uptake']

        fluxes, obj_val = run_fba(ko, obj, glc)

        draw_network(ax_network, fluxes, ko)
        draw_bar_flux(ax_bar_flux, fluxes, ko)
        draw_objectives(ax_objectives, fluxes, obj)

        growth = abs(fluxes.get('v_BIOM', 0.0))
        etoh   = abs(fluxes.get('v_ETH',  0.0))
        glc_in = abs(fluxes.get('v_GLC',  0.0))
        etoh_yield = etoh / glc_in if glc_in > 1e-6 else 0.0

        ko_str = ', '.join(REACTIONS[r]['gene'] for r in sorted(ko)) if ko else 'none'
        info_text.set_text(
            f'Growth: {growth:.3f}\n'
            f'Ethanol: {etoh:.3f}\n'
            f'EtOH yield: {etoh_yield:.3f}\n'
            f'KOs: {ko_str}'
        )
        fig.canvas.draw_idle()

    # ------------------------------------------------------------------ #
    def on_checkbox(label):
        # Extract reaction id from label (format "v_PGI  [pgi]")
        rxn_id = label.split('  ')[0].strip()
        if rxn_id in state['knockouts']:
            state['knockouts'].discard(rxn_id)
        else:
            state['knockouts'].add(rxn_id)
        update()

    def on_radio(label):
        if 'Growth' in label:
            state['objective'] = 'v_BIOM'
        else:
            state['objective'] = 'v_ETH'
        update()

    def on_slider(val):
        state['glc_uptake'] = sl_glc.val
        update()

    def on_reset(event):
        state['knockouts'] = set()
        # Un-check all checkboxes
        for i, status in enumerate(chk_buttons.get_status()):
            if status:
                chk_buttons.set_active(i)
        update()

    chk_buttons.on_clicked(on_checkbox)
    radio_obj.on_clicked(on_radio)
    sl_glc.on_changed(on_slider)
    btn_reset.on_clicked(on_reset)

    # ---- Title ----
    fig.text(
        0.5, 0.975,
        'Gene Knockout Flux Balance Analysis',
        ha='center', va='top', color='white', fontsize=15, fontweight='bold',
    )
    fig.text(
        0.5, 0.950,
        'Tier 3.4  |  FBA via scipy.optimize.linprog  |  E. coli core metabolic network',
        ha='center', va='top', color='#8be9fd', fontsize=9,
    )

    # Key hints
    fig.text(
        0.76, 0.96,
        'Key: Knock out aceE (PDH) to redirect\nflux to ethanol. Knock out nuoA\n(respiration) for fermentative growth.',
        ha='left', va='top', color='#f5a623', fontsize=8,
        bbox=dict(boxstyle='round,pad=0.4', facecolor='#0f3460', alpha=0.8),
    )

    # Initial draw
    update()
    return fig


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    print("=" * 60)
    print("Gene Knockout Flux Balance Analysis")
    print("=" * 60)
    print()
    print("Controls:")
    print("  Checkboxes       — toggle gene knockouts")
    print("  Radio buttons    — switch objective (growth / ethanol)")
    print("  Glucose slider   — set carbon input rate")
    print("  Reset button     — restore all reactions")
    print()
    print("Try these knockouts:")
    print("  v_PDH (aceE)  → blocks TCA, forces ethanol fermentation")
    print("  v_RESP (nuoA) → forces fermentative metabolism")
    print("  v_CS (gltA)   → blocks TCA entry")
    print("  v_PDH + v_RESP → maximum ethanol yield")
    print()
    print("Concept: Growth/ethanol trade-off emerges from limited")
    print("carbon and cofactor (ATP, NADH) balancing.")
    print()
    fig = build_app()
    plt.show()
