"""
Genetic Circuit Simulator: Composable Parts → ODE
===================================================
Tier 3.2 — Synthetic Biology: Genetic Circuit Design

Concepts demonstrated:
  - Transfer function framework for genetic parts
  - Boolean logic gates from Hill functions (NOT, AND, OR, NAND, NOR)
  - Composing gates: output of one gate drives input of next
  - Retroactivity: downstream loading affects upstream output
  - Response time and steady-state behavior
  - Multi-input logic (truth table → ODE response)
  - Promoter strength, RBS efficiency, degradation tag effects

Usage:
    python 14_genetic_circuit_simulator.py [--circuit CIRCUIT]

Circuits:
    not_gate    — Single NOT gate (inverter)
    and_gate    — AND gate (Hill logic)
    nand_gate   — NAND (AND + NOT)
    toggle      — Toggle switch (two NOT gates in feedback)
    pulse       — Pulse generator (I1-FFL)

Key insight:
    Each genetic gate can be characterized by its transfer function —
    the input-output relationship at steady state. Composing gates
    multiplies the nonlinearities. At each stage, you lose some of the
    nonlinearity, so cascades quickly degrade signal quality.
    CELLO automates part selection to maintain signal integrity.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.integrate import solve_ivp


# ── Part Library ──────────────────────────────────────────────────────────────

PARTS_LIBRARY = {
    # name: (promoter_strength, beta_mRNA, rbs_efficiency, beta_protein, n, K)
    "pTac":   {"alpha": 3.0,  "beta_m": 0.5,  "rbs": 5.0,  "beta_p": 0.1,  "n": 2.0, "K": 1.0},
    "pTet":   {"alpha": 2.5,  "beta_m": 0.5,  "rbs": 4.0,  "beta_p": 0.1,  "n": 2.5, "K": 1.0},
    "pBAD":   {"alpha": 2.0,  "beta_m": 0.4,  "rbs": 3.0,  "beta_p": 0.08, "n": 2.0, "K": 1.0},
    "pLuxR":  {"alpha": 4.0,  "beta_m": 0.6,  "rbs": 6.0,  "beta_p": 0.12, "n": 3.0, "K": 1.0},
    "pCI":    {"alpha": 2.0,  "beta_m": 0.4,  "rbs": 4.0,  "beta_p": 0.1,  "n": 2.0, "K": 1.0},
}

GATE_COLORS = {
    'NOT':    '#ff79c6',
    'AND':    '#50fa7b',
    'OR':     '#8be9fd',
    'NAND':   '#f5a623',
    'NOR':    '#ff5555',
    'BUFFER': '#f1fa8c',
}


# ── Gate Transfer Functions ───────────────────────────────────────────────────

def hill_activation(x, alpha, K, n):
    return alpha * x**n / (K**n + x**n)


def hill_repression(x, alpha, K, n):
    return alpha * K**n / (K**n + x**n)


def and_gate(x1, x2, alpha, K1, K2, n):
    return alpha * (x1**n / (K1**n + x1**n)) * (x2**n / (K2**n + x2**n))


def or_gate(x1, x2, alpha, K1, K2, n):
    f1 = x1**n / (K1**n + x1**n)
    f2 = x2**n / (K2**n + x2**n)
    return alpha * (f1 + f2 - f1 * f2)   # noisy OR


def nand_gate(x1, x2, alpha, K1, K2, n):
    return hill_repression(and_gate(x1, x2, 1.0, K1, K2, n), alpha, 0.5, 2.0)


def nor_gate(x1, x2, alpha, K1, K2, n):
    combined = (x1**n + x2**n) / (K1**n + K2**n + x1**n + x2**n)
    return alpha * (1 - combined)


# ── Circuit ODEs ──────────────────────────────────────────────────────────────

def not_gate_odes(t, y, input_fn, alpha, K, n, beta_m, rbs, beta_p):
    m, p = y
    u = input_fn(t)
    dm = hill_repression(u, alpha, K, n) - beta_m * m
    dp = rbs * m - beta_p * p
    return [dm, dp]


def and_gate_odes(t, y, input_fn1, input_fn2, alpha, K, n, beta_m, rbs, beta_p):
    m, p = y
    u1 = input_fn1(t)
    u2 = input_fn2(t)
    dm = and_gate(u1, u2, alpha, K, K, n) - beta_m * m
    dp = rbs * m - beta_p * p
    return [dm, dp]


def nand_gate_odes(t, y, input_fn1, input_fn2, alpha, K, n, beta_m, rbs, beta_p):
    m, p = y
    u1 = input_fn1(t)
    u2 = input_fn2(t)
    dm = nand_gate(u1, u2, alpha, K, K, n) - beta_m * m
    dp = rbs * m - beta_p * p
    return [dm, dp]


# ── Circuit Definitions ───────────────────────────────────────────────────────

def make_step(t_on, t_off, hi=3.0, lo=0.0):
    return lambda t: hi if t_on <= t <= t_off else lo


def run_circuit(circuit_name, parts='pTac'):
    part = PARTS_LIBRARY[parts]
    alpha = part['alpha']
    K     = part['K']
    n     = part['n']
    beta_m = part['beta_m']
    rbs    = part['rbs']
    beta_p = part['beta_p']

    T_SPAN = (0, 200)
    T_EVAL = np.linspace(0, 200, 2000)

    if circuit_name == 'not_gate':
        input_fn = make_step(50, 150)
        sol = solve_ivp(not_gate_odes, T_SPAN, [0, 0], t_eval=T_EVAL,
                        args=(input_fn, alpha, K, n, beta_m, rbs, beta_p),
                        method='LSODA')
        inputs = [(T_EVAL, np.array([input_fn(t) for t in T_EVAL]), 'Input (Inducer)', '#f1fa8c')]
        outputs = [(sol.t, sol.y[1], 'Output (protein)', '#ff79c6')]
        title = f'NOT Gate — Inverter  (parts: {parts})'

    elif circuit_name == 'and_gate':
        in1 = make_step(30, 200)
        in2 = make_step(80, 200)
        sol = solve_ivp(and_gate_odes, T_SPAN, [0, 0], t_eval=T_EVAL,
                        args=(in1, in2, alpha, K, n, beta_m, rbs, beta_p),
                        method='LSODA')
        inputs = [
            (T_EVAL, np.array([in1(t) for t in T_EVAL]), 'Input A', '#f1fa8c'),
            (T_EVAL, np.array([in2(t) for t in T_EVAL]), 'Input B', '#8be9fd'),
        ]
        outputs = [(sol.t, sol.y[1], 'Output (A AND B)', '#50fa7b')]
        title = f'AND Gate  (parts: {parts})'

    elif circuit_name == 'nand_gate':
        in1 = make_step(30, 200)
        in2 = make_step(80, 200)
        sol = solve_ivp(nand_gate_odes, T_SPAN, [0, 0], t_eval=T_EVAL,
                        args=(in1, in2, alpha, K, n, beta_m, rbs, beta_p),
                        method='LSODA')
        inputs = [
            (T_EVAL, np.array([in1(t) for t in T_EVAL]), 'Input A', '#f1fa8c'),
            (T_EVAL, np.array([in2(t) for t in T_EVAL]), 'Input B', '#8be9fd'),
        ]
        outputs = [(sol.t, sol.y[1], 'Output (A NAND B)', '#f5a623')]
        title = f'NAND Gate  (parts: {parts})'

    elif circuit_name == 'toggle':
        # Two NOT gates in feedback
        def toggle_odes(t, y):
            m1, p1, m2, p2 = y
            in_1 = p2   # p2 represses gene 1
            in_2 = p1   # p1 represses gene 2
            ext1 = make_step(40, 60)(t) * 2.0   # brief IPTG (knocks out p2 repression)
            dm1 = hill_repression(in_1, alpha, K, n) + ext1 - beta_m * m1
            dp1 = rbs * m1 - beta_p * p1
            dm2 = hill_repression(in_2, alpha, K, n) - beta_m * m2
            dp2 = rbs * m2 - beta_p * p2
            return [dm1, dp1, dm2, dp2]

        sol = solve_ivp(toggle_odes, T_SPAN, [0, 0, 5.0, 30.0], t_eval=T_EVAL,
                        method='LSODA', rtol=1e-8)
        ext_signal = np.array([make_step(40, 60)(t) * 2.0 for t in T_EVAL])
        inputs = [(T_EVAL, ext_signal, 'IPTG pulse (flip signal)', '#f1fa8c')]
        outputs = [
            (sol.t, sol.y[1], 'State 1 (TetR)', '#50fa7b'),
            (sol.t, sol.y[3], 'State 2 (LacI)', '#ff79c6'),
        ]
        title = f'Toggle Switch (two NOT gates)  (parts: {parts})'

    elif circuit_name == 'pulse':
        # I1-FFL: pulse generator
        def pulse_odes(t, y):
            mY, pY, mZ, pZ = y
            X = make_step(30, 170)(t)
            dmY = hill_activation(X, alpha, K, n) - beta_m * mY
            dpY = rbs * mY - beta_p * pY
            # Z activated by X, repressed by pY
            dmZ = (hill_activation(X, alpha, K, n) *
                   hill_repression(pY, 1.0, K, n) - beta_m * mZ)
            dpZ = rbs * mZ - beta_p * pZ
            return [dmY, dpY, dmZ, dpZ]

        sol = solve_ivp(pulse_odes, T_SPAN, [0, 0, 0, 0], t_eval=T_EVAL,
                        method='LSODA', rtol=1e-8)
        X_vals = np.array([make_step(30, 170)(t) for t in T_EVAL])
        inputs = [(T_EVAL, X_vals, 'X (input)', '#f1fa8c')]
        outputs = [
            (sol.t, sol.y[1], 'Y (repressor builds up)', '#8be9fd'),
            (sol.t, sol.y[3], 'Z (pulse output)', '#ff79c6'),
        ]
        title = f'Pulse Generator (I1-FFL)  (parts: {parts})'

    else:
        raise ValueError(f"Unknown circuit: {circuit_name}")

    return title, inputs, outputs, part


# ── Transfer Function Sweep ───────────────────────────────────────────────────

def compute_transfer(gate_fn, part, x_range=(0, 5)):
    x = np.linspace(x_range[0], x_range[1], 300)
    alpha, K, n = part['alpha'], part['K'], part['n']
    if gate_fn == 'not':
        y = [hill_repression(xi, alpha, K, n) for xi in x]
    elif gate_fn == 'and':
        y = [and_gate(xi, xi, alpha, K, K, n) for xi in x]  # sweep both together
    elif gate_fn == 'or':
        y = [or_gate(xi, xi, alpha, K, K, n) for xi in x]
    else:
        y = [hill_activation(xi, alpha, K, n) for xi in x]
    return x, np.array(y)


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(circuit_name='not_gate', parts='pTac'):
    title, inputs, outputs, part = run_circuit(circuit_name, parts)

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Genetic Circuit Simulator — {title}',
                 color='white', fontsize=12, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_ts   = fig.add_subplot(gs[0, :2])    # time series
    ax_tf   = fig.add_subplot(gs[1, 0])     # transfer function
    ax_tt   = fig.add_subplot(gs[1, 1])     # truth table visualization
    ax_info = fig.add_subplot(gs[:, 2])     # info

    for ax in [ax_ts, ax_tf, ax_tt, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Time series ───────────────────────────────────────────────────────────
    for (t, y, label, color) in inputs:
        ax_ts.plot(t, y, color=color, lw=1.8, ls='--', alpha=0.8, label=label)
    for (t, y, label, color) in outputs:
        ax_ts.plot(t, y, color=color, lw=2.5, label=label)

    ax_ts.set_xlabel('Time (a.u.)', color='white')
    ax_ts.set_ylabel('Concentration', color='white')
    ax_ts.set_title('Circuit Dynamics', color='white', fontsize=9)
    ax_ts.tick_params(colors='white')
    ax_ts.legend(fontsize=8, facecolor='#16213e', labelcolor='white', ncol=3)
    for sp in ax_ts.spines.values():
        sp.set_edgecolor('#888')

    # ── Transfer function ─────────────────────────────────────────────────────
    gate_map = {'not_gate': 'not', 'and_gate': 'and', 'nand_gate': 'and',
                'toggle': 'not', 'pulse': 'buffer'}
    gfn = gate_map.get(circuit_name, 'not')
    x_tf, y_tf = compute_transfer(gfn, part)
    ax_tf.plot(x_tf, y_tf, color='#8be9fd', lw=2.5)
    ax_tf.set_xlabel('Input concentration', color='white')
    ax_tf.set_ylabel('Steady-state output', color='white')
    ax_tf.set_title('Gate Transfer Function', color='white', fontsize=9)
    ax_tf.tick_params(colors='white')
    # Mark logic threshold
    mid = y_tf.max() / 2
    x_mid = x_tf[np.argmin(np.abs(y_tf - mid))]
    ax_tf.axhline(mid, color='#888', lw=0.8, ls='--', alpha=0.6, label='50% threshold')
    ax_tf.axvline(x_mid, color='#f5a623', lw=0.8, ls='--', alpha=0.6)
    ax_tf.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_tf.spines.values():
        sp.set_edgecolor('#888')

    # ── Truth table visualization ─────────────────────────────────────────────
    ax_tt.axis('off')
    ax_tt.set_xlim(0, 1)
    ax_tt.set_ylim(0, 1)
    ax_tt.set_title('Logic Truth Table', color='white', fontsize=9)

    truth_tables = {
        'not_gate':  [('A', 'OUT'), ('0', '1'), ('1', '0')],
        'and_gate':  [('A', 'B', 'OUT'), ('0','0','0'), ('1','0','0'), ('0','1','0'), ('1','1','1')],
        'nand_gate': [('A', 'B', 'OUT'), ('0','0','1'), ('1','0','1'), ('0','1','1'), ('1','1','0')],
        'toggle':    [('IPTG', 'State'), ('OFF', 'Bistable'), ('PULSE', 'Flip')],
        'pulse':     [('X', 'Z'), ('0→0', 'Low'), ('0→1', 'Pulse then 0'), ('1→1', 'Low')],
    }

    tt = truth_tables.get(circuit_name, [('?', '?')])
    n_rows = len(tt)
    row_h = 0.9 / n_rows

    for row_i, row in enumerate(tt):
        y_pos = 0.95 - row_i * row_h
        n_cols = len(row)
        for col_i, val in enumerate(row):
            x_pos = 0.1 + col_i * 0.8 / n_cols
            is_header = row_i == 0
            is_one = val in ('1', 'OUT')
            bg = '#16213e' if is_header else ('#1a6630' if val == '1' else '#661a1a' if val == '0' else '#1a1a2e')
            color = 'white' if not is_header else '#f5a623'
            rect = plt.Rectangle((x_pos - 0.05, y_pos - row_h * 0.8),
                                  0.75 / n_cols, row_h * 0.9,
                                  facecolor=bg, edgecolor='#888', transform=ax_tt.transAxes)
            ax_tt.add_patch(rect)
            ax_tt.text(x_pos + 0.3 / n_cols, y_pos - row_h * 0.35, val,
                       ha='center', va='center', fontsize=9, color=color,
                       fontweight='bold' if is_header else 'normal',
                       transform=ax_tt.transAxes)

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    info = (
        f"Circuit: {circuit_name}\n"
        f"Parts:   {parts}\n\n"
        f"Part Parameters:\n"
        f"  Promoter α = {part['alpha']:.2f}\n"
        f"  Hill coeff n = {part['n']:.2f}\n"
        f"  Half-sat K  = {part['K']:.2f}\n"
        f"  mRNA deg βm = {part['beta_m']:.2f}\n"
        f"  RBS eff.    = {part['rbs']:.2f}\n"
        f"  Protein deg = {part['beta_p']:.3f}\n\n"
        f"Key concepts:\n"
        f"  Transfer fn: maps steady-state\n"
        f"    input → output concentration\n\n"
        f"  Retroactivity: connecting gates\n"
        f"    loads upstream → lower output\n\n"
        f"  Signal integrity:\n"
        f"    need sharp threshold (high n)\n"
        f"    and wide HIGH/LOW separation\n\n"
        f"Available circuits:\n"
        f"  not_gate, and_gate, nand_gate\n"
        f"  toggle, pulse"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=8, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Genetic circuit simulator')
    parser.add_argument('--circuit', default='not_gate',
                        choices=['not_gate', 'and_gate', 'nand_gate', 'toggle', 'pulse'])
    parser.add_argument('--parts', default='pTac', choices=list(PARTS_LIBRARY.keys()))
    args = parser.parse_args()

    print("Genetic Circuit Simulator")
    print("=" * 45)
    run(circuit_name=args.circuit, parts=args.parts)
