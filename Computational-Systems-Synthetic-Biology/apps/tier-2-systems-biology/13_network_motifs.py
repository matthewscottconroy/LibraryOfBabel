"""
Network Motifs: Feed-Forward Loops & Autoregulation
=====================================================
Tier 2.3 — Systems Biology: Gene Regulatory Networks

Concepts demonstrated:
  - The 8 types of feed-forward loops (FFL) and their logic
  - Coherent type-1 FFL: pulse generation (sign-sensitive delay)
  - Incoherent type-1 FFL: pulse with adaptation
  - Autoregulation: positive vs. negative feedback
  - Negative autoregulation: speeds up response, reduces noise
  - Positive autoregulation: bistability and slowed response
  - Motif enrichment vs. random networks (Z-score concept)

Usage:
    python 13_network_motifs.py [--motif MOTIF]

Motifs:
    c1ffl    — Coherent type-1 FFL
    i1ffl    — Incoherent type-1 FFL
    negfb    — Negative autoregulation
    posfb    — Positive autoregulation

Key insight:
    Network motifs are patterns of interconnection that appear far more
    frequently in biological networks than in random networks. Each motif
    has a specific dynamical function: C1-FFL filters noise and generates
    pulses; I1-FFL adapts and pulses; negative autoregulation speeds
    response and buffers noise.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.patches as mpatches
from matplotlib.patches import FancyArrowPatch
from scipy.integrate import solve_ivp


# ── ODE Models ────────────────────────────────────────────────────────────────

def step_input(t, t_on=20, t_off=80, high=1.0, low=0.0):
    return high if t_on <= t <= t_off else low


def coherent_c1ffl(t, y, alpha_Y, alpha_Z, beta_Y, beta_Z, K_XY, K_XZ, K_YZ, n=2):
    """
    X→Y→Z, X→Z  (all activating)
    Input: X(t) = step function
    """
    X = step_input(t)
    Y, Z = y

    # X activates Y
    dY = alpha_Y * X**n / (K_XY**n + X**n) - beta_Y * Y

    # Z requires BOTH X and Y (AND logic)
    act_XZ = X**n / (K_XZ**n + X**n)
    act_YZ = Y**n / (K_YZ**n + Y**n)
    dZ = alpha_Z * act_XZ * act_YZ - beta_Z * Z

    return [dY, dZ]


def incoherent_i1ffl(t, y, alpha_Y, alpha_Z, beta_Y, beta_Z, K_XY, K_XZ, K_YZ, n=2):
    """
    X→Y, X→Z, Y⊣Z  (X activates Y and Z; Y represses Z)
    """
    X = step_input(t)
    Y, Z = y

    dY = alpha_Y * X**n / (K_XY**n + X**n) - beta_Y * Y

    # Z activated by X, repressed by Y
    act_XZ = X**n / (K_XZ**n + X**n)
    rep_YZ = K_YZ**n / (K_YZ**n + Y**n)
    dZ = alpha_Z * act_XZ * rep_YZ - beta_Z * Z

    return [dY, dZ]


def neg_autoregulation(t, y, alpha, beta, K, n=2):
    """
    X → mRNA → Protein, Protein ⊣ own transcription
    Compare: unregulated (same alpha, no feedback)
    """
    m, p = y
    # Feedback: protein represses its own mRNA
    rep = K**n / (K**n + p**n)
    X = step_input(t)
    dm = alpha * X * rep - beta * m
    dp = 5.0 * m - 0.1 * p
    return [dm, dp]


def no_autoregulation(t, y, alpha, beta, K, n=2):
    """Same system without negative feedback."""
    m, p = y
    X = step_input(t)
    dm = alpha * X - beta * m
    dp = 5.0 * m - 0.1 * p
    return [dm, dp]


def pos_autoregulation(t, y, alpha0, alpha1, beta, K, n=2):
    """
    Positive feedback: protein activates own mRNA
    Shows bistability for strong enough feedback
    """
    m, p = y
    X = step_input(t)
    act = p**n / (K**n + p**n)
    dm = X * (alpha0 + alpha1 * act) - beta * m
    dp = 5.0 * m - 0.1 * p
    return [dm, dp]


# ── Run and Collect ───────────────────────────────────────────────────────────

def solve(rhs, y0, t_span, t_eval, args=()):
    sol = solve_ivp(rhs, t_span, y0, t_eval=t_eval, args=args,
                    method='LSODA', rtol=1e-8)
    return sol.t, sol.y


T_SPAN = (0, 120)
T_EVAL = np.linspace(0, 120, 1200)

MOTIF_DATA = {
    "c1ffl": {
        "title": "Coherent Type-1 FFL  (X→Y→Z, X→Z)",
        "description": (
            "Coherent Type-1 FFL:\n"
            "  X activates Y\n"
            "  X activates Z (direct)\n"
            "  Y activates Z (AND logic)\n\n"
            "Function:\n"
            "  Sign-sensitive delay: ON delay,\n"
            "  but fast OFF when X is removed.\n\n"
            "  Filters out brief X pulses.\n"
            "  Only sustained X turns on Z.\n\n"
            "Example: araBAD operon in E. coli\n"
            "  (arabinose + CRP required)"
        ),
    },
    "i1ffl": {
        "title": "Incoherent Type-1 FFL  (X→Y⊣Z, X→Z)",
        "description": (
            "Incoherent Type-1 FFL:\n"
            "  X activates Y\n"
            "  X activates Z (direct)\n"
            "  Y represses Z\n\n"
            "Function:\n"
            "  Pulse generator: Z peaks then\n"
            "  drops as inhibitor Y builds up.\n"
            "  Exact adaptation when params\n"
            "  are tuned correctly.\n\n"
            "Example: Flagellar regulation\n"
            "  (FlhDC→FliA, FlhDC→FlgM, FliA⊣FlgM)"
        ),
    },
    "negfb": {
        "title": "Negative Autoregulation  (X ⊣ X)",
        "description": (
            "Negative Autoregulation:\n"
            "  Protein represses own mRNA\n\n"
            "Benefits:\n"
            "  1. Faster response time\n"
            "     (doesn't wait for saturation)\n"
            "  2. Reduced noise (buffers\n"
            "     stochastic fluctuations)\n"
            "  3. Set-point robustness:\n"
            "     steady state less sensitive\n"
            "     to parameter variation\n\n"
            "~40% of E. coli TFs\n"
            "autoregulate negatively"
        ),
    },
    "posfb": {
        "title": "Positive Autoregulation  (X → X)",
        "description": (
            "Positive Autoregulation:\n"
            "  Protein activates own mRNA\n\n"
            "Effects:\n"
            "  1. Slows response time\n"
            "     (accelerates beyond steady state)\n"
            "  2. Can create bistability\n"
            "     for strong feedback\n"
            "  3. Amplifies noise\n\n"
            "Bimodal distribution when\n"
            "α₁ is large: two stable states\n"
            "(ON and OFF populations)\n\n"
            "Example: Cell fate commitment"
        ),
    },
}


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(motif='c1ffl'):
    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    data = MOTIF_DATA[motif]
    fig.suptitle(data['title'], color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax1 = fig.add_subplot(gs[0, :2])
    ax2 = fig.add_subplot(gs[1, :2])
    ax_scheme = fig.add_subplot(gs[0, 2])
    ax_info   = fig.add_subplot(gs[1, 2])

    for ax in [ax1, ax2, ax_scheme, ax_info]:
        ax.set_facecolor('#0f3460')

    # Draw X input
    t_input = np.linspace(0, 120, 1200)
    x_input = np.array([step_input(t) for t in t_input])

    if motif == 'c1ffl':
        t, y = solve(coherent_c1ffl, [0, 0], T_SPAN, T_EVAL,
                     args=(1.0, 1.0, 1.0, 1.0, 0.5, 0.5, 0.5, 2))
        Y, Z = y

        ax1.plot(t_input, x_input, color='#f1fa8c', lw=2, label='X (input signal)')
        ax1.plot(t, Y, color='#50fa7b', lw=2, label='Y (intermediate)')
        ax1.set_ylabel('Concentration', color='white')
        ax1.set_title('C1-FFL: Input X and Intermediate Y', color='white', fontsize=9)

        ax2.plot(t_input, x_input, color='#f1fa8c', lw=1.5, ls='--', alpha=0.5, label='X')
        ax2.plot(t, Z, color='#ff79c6', lw=2.5, label='Z (output)')
        ax2.axvline(20, color='#888', lw=1, ls=':', alpha=0.6)
        ax2.axvline(80, color='#888', lw=1, ls=':', alpha=0.6)
        ax2.set_ylabel('Z concentration', color='white')
        ax2.set_title('Output Z: delayed ON, fast OFF (sign-sensitive delay)', color='white', fontsize=9)

    elif motif == 'i1ffl':
        t, y = solve(incoherent_i1ffl, [0, 0], T_SPAN, T_EVAL,
                     args=(2.0, 2.0, 1.0, 0.5, 0.5, 0.5, 0.5, 2))
        Y, Z = y

        ax1.plot(t_input, x_input, color='#f1fa8c', lw=2, label='X (input)')
        ax1.plot(t, Y, color='#8be9fd', lw=2, label='Y (repressor builds up)')
        ax1.set_ylabel('Concentration', color='white')
        ax1.set_title('I1-FFL: Input X and Repressor Y', color='white', fontsize=9)

        ax2.plot(t_input, x_input * Z.max() * 0.9, color='#f1fa8c', lw=1.5, ls='--',
                 alpha=0.5, label='X (scaled)')
        ax2.plot(t, Z, color='#ff79c6', lw=2.5, label='Z (output pulse)')
        ax2.set_ylabel('Z concentration', color='white')
        ax2.set_title('Output Z: Pulse generator (I1-FFL adaptation)', color='white', fontsize=9)

    elif motif == 'negfb':
        # Compare negative autoregulation vs. no autoregulation
        t_neg, y_neg = solve(neg_autoregulation, [0, 0], T_SPAN, T_EVAL,
                             args=(1.0, 1.0, 5.0, 2))
        t_ref, y_ref = solve(no_autoregulation, [0, 0], T_SPAN, T_EVAL,
                             args=(1.0, 1.0, 5.0, 2))

        ax1.plot(t_input, x_input * y_neg[1].max(), color='#888', lw=1.5, ls='--',
                 alpha=0.5, label='X input')
        ax1.plot(t_neg, y_neg[1], color='#50fa7b', lw=2.5, label='Neg. autoregulation')
        ax1.plot(t_ref, y_ref[1], color='#ff5555', lw=2.5, ls='--',
                 label='No autoregulation')
        ax1.set_ylabel('Protein level', color='white')
        ax1.set_title('Negative Autoregulation: Faster response, lower steady state',
                      color='white', fontsize=9)
        ax1.legend(fontsize=8, facecolor='#16213e', labelcolor='white')

        # Response time comparison
        ss_neg = y_neg[1, -1]
        ss_ref = y_ref[1, -1]
        t50_neg = t_neg[np.argmin(np.abs(y_neg[1] - ss_neg / 2))] if ss_neg > 0 else 0
        t50_ref = t_ref[np.argmin(np.abs(y_ref[1] - ss_ref / 2))] if ss_ref > 0 else 0

        ax2.bar(['No feedback', 'Neg. autoregulation'], [t50_ref - 20, t50_neg - 20],
                color=['#ff5555', '#50fa7b'], edgecolor='white', width=0.5)
        ax2.set_ylabel('Response time T₅₀ (a.u.)', color='white')
        ax2.set_title('Response Time Comparison  (faster with negative feedback)',
                      color='white', fontsize=9)
        ax2.tick_params(colors='white')

    elif motif == 'posfb':
        # Multiple initial conditions
        ics = [(0, 0), (0, 5), (0, 15)]
        colors_ic = ['#8be9fd', '#50fa7b', '#ff79c6']

        for ic, col in zip(ics, colors_ic):
            t_p, y_p = solve(pos_autoregulation, list(ic), T_SPAN, T_EVAL,
                             args=(0.3, 3.0, 1.0, 5.0, 2))
            t_r, y_r = solve(no_autoregulation, list(ic), T_SPAN, T_EVAL,
                             args=(0.5, 1.0, 5.0, 2))
            ax1.plot(t_p, y_p[1], color=col, lw=2, label=f'pos_fb: p₀={ic[1]}')
            ax2.plot(t_r, y_r[1], color=col, lw=2, ls='--', label=f'no_fb: p₀={ic[1]}')

        ax1.set_ylabel('Protein (positive feedback)', color='white')
        ax1.set_title('Positive autoregulation: slow-then-fast dynamics',
                      color='white', fontsize=9)
        ax2.set_ylabel('Protein (no feedback)', color='white')
        ax2.set_title('Reference: no autoregulation', color='white', fontsize=9)

    for ax in [ax1, ax2]:
        ax.set_xlabel('Time', color='white')
        ax.tick_params(colors='white')
        if not ax.get_legend():
            pass
        ax.legend(fontsize=7.5, facecolor='#16213e', labelcolor='white',
                  loc='upper right') if not ax.get_legend() else None
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')
        ax.axvline(20, color='#f1fa8c', lw=0.8, alpha=0.5)
        ax.axvline(80, color='#f1fa8c', lw=0.8, alpha=0.5)

    # ── Circuit schematic ─────────────────────────────────────────────────────
    ax_scheme.axis('off')
    ax_scheme.set_xlim(0, 10)
    ax_scheme.set_ylim(0, 10)

    schematics = {
        'c1ffl': [
            ((2, 8), (5, 6), '+', '#50fa7b'),   # X→Y
            ((5, 6), (8, 4), '+', '#50fa7b'),   # Y→Z (AND)
            ((2, 8), (8, 4), '+', '#50fa7b'),   # X→Z
        ],
        'i1ffl': [
            ((2, 8), (5, 6), '+', '#50fa7b'),   # X→Y
            ((5, 6), (8, 4), '⊣', '#ff5555'),  # Y⊣Z
            ((2, 8), (8, 4), '+', '#50fa7b'),   # X→Z
        ],
        'negfb': [
            ((5, 7), (5, 3), '+', '#50fa7b'),   # X→Protein
            ((5, 3), (3, 7), '⊣', '#ff5555'),  # P⊣X
        ],
        'posfb': [
            ((5, 7), (5, 3), '+', '#50fa7b'),   # X→Protein
            ((5, 3), (7, 7), '+', '#50fa7b'),   # P→X
        ],
    }

    node_labels = {
        'c1ffl': [(2, 8, 'X'), (5, 6, 'Y'), (8, 4, 'Z')],
        'i1ffl': [(2, 8, 'X'), (5, 6, 'Y'), (8, 4, 'Z')],
        'negfb': [(5, 7, 'mRNA'), (5, 3, 'Protein')],
        'posfb': [(5, 7, 'mRNA'), (5, 3, 'Protein')],
    }

    ax_scheme.set_title('Circuit Schematic', color='white', fontsize=9)

    for (x1, y1), (x2, y2), sign, col in schematics[motif]:
        ax_scheme.annotate('', xy=(x2, y2), xytext=(x1, y1),
                           arrowprops=dict(arrowstyle='->', color=col, lw=2.5,
                                          connectionstyle='arc3,rad=0.1'))
        mx, my = (x1 + x2) / 2 + 0.3, (y1 + y2) / 2
        ax_scheme.text(mx, my, sign, color=col, fontsize=14, fontweight='bold',
                       ha='center', va='center')

    for (x, y, label) in node_labels[motif]:
        ax_scheme.plot(x, y, 'o', color='#8be9fd', ms=25, markeredgecolor='white',
                       markeredgewidth=1.5, zorder=4)
        ax_scheme.text(x, y, label, ha='center', va='center', fontsize=8,
                       color='#1a1a2e', fontweight='bold', zorder=5)

    # ── Info ──────────────────────────────────────────────────────────────────
    ax_info.axis('off')
    ax_info.text(0.04, 0.97, data['description'], transform=ax_info.transAxes,
                 va='top', fontsize=8, color='white', fontfamily='monospace',
                 linespacing=1.5)

    plt.show()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Network motif dynamics')
    parser.add_argument('--motif', default='c1ffl',
                        choices=['c1ffl', 'i1ffl', 'negfb', 'posfb'])
    args = parser.parse_args()

    print("Network Motifs: Feed-Forward Loops & Autoregulation")
    print("=" * 55)
    run(motif=args.motif)
