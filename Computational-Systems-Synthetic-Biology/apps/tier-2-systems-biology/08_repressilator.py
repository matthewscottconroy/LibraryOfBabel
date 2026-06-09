"""
Repressilator: ODE + Stochastic Simulation
============================================
Tier 2.1 — Systems Biology: Mathematical Modeling

Concepts demonstrated:
  - Repressilator circuit: three mutually repressing genes (Elowitz & Leibler 2000)
  - ODE model with Hill-function repression
  - Hopf bifurcation condition and oscillation onset
  - Gillespie SSA vs. ODE comparison
  - Effect of cooperativity n on oscillation amplitude/period
  - Noise in genetic oscillators

Usage:
    python 08_repressilator.py [--n HILL] [--alpha ALPHA] [--beta BETA]

Key parameters:
    n     — Hill coefficient (cooperativity). Oscillations require n > 1.
    alpha — maximum protein production rate
    beta  — ratio of protein to mRNA degradation (α₀ = leaky expression)

Key insight:
    The repressilator exhibits sustained oscillations when the negative
    feedback loop traverses a Hopf bifurcation. The Hopf condition
    for the 3-gene symmetric repressilator requires:
        n > 8α/(α + α₀) · 1 + cos(π/3) / (1 - cos(π/3))
    Increasing cooperativity n makes oscillations more robust.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.integrate import solve_ivp
from scipy.signal import find_peaks


# ── ODE Model ─────────────────────────────────────────────────────────────────

def repressilator_odes(t, y, alpha, alpha0, beta, n):
    """
    Elowitz & Leibler repressilator ODEs.

    State: [m1, m2, m3, p1, p2, p3]
      mᵢ = mRNA of gene i
      pᵢ = protein of gene i

    dm₁/dt = α/(1 + p₃ⁿ) + α₀ - m₁
    dp₁/dt = β·(m₁ - p₁)
    (cyclic: 3→1→2→3)
    """
    m1, m2, m3, p1, p2, p3 = y
    dm1 = alpha / (1 + p3**n) + alpha0 - m1
    dm2 = alpha / (1 + p1**n) + alpha0 - m2
    dm3 = alpha / (1 + p2**n) + alpha0 - m3
    dp1 = beta * (m1 - p1)
    dp2 = beta * (m2 - p2)
    dp3 = beta * (m3 - p3)
    return [dm1, dm2, dm3, dp1, dp2, dp3]


def run_ode(alpha=216, alpha0=0.216, beta=0.2, n=2, t_max=200, n_pts=5000):
    """Integrate repressilator ODEs."""
    y0 = [0.0, 0.0, 0.0, 2.0, 1.0, 3.0]  # slightly asymmetric IC
    sol = solve_ivp(repressilator_odes, (0, t_max), y0,
                    args=(alpha, alpha0, beta, n),
                    t_eval=np.linspace(0, t_max, n_pts),
                    method='LSODA', rtol=1e-8)
    return sol.t, sol.y


# ── Gillespie SSA ─────────────────────────────────────────────────────────────

def repressilator_gillespie(alpha, alpha0, beta_rate, n, t_max=500, seed=42):
    """
    Gillespie SSA for the repressilator.
    State: [m1, m2, m3, p1, p2, p3] (integer molecule counts)

    Reactions (per gene; cyclic 3→1→2→3):
      1. Transcription mᵢ:  rate = alpha_eff / (1 + pⱼⁿ) + alpha0_eff
      2. mRNA degradation:   rate = km · mᵢ
      3. Translation:        rate = kp · mᵢ
      4. Protein degradation: rate = kp · pᵢ
    """
    rng = np.random.default_rng(seed)

    # Scale rates to molecular units
    km = 1.0       # mRNA degradation rate
    kp = beta_rate  # protein degradation rate (= beta * km)
    kTx = alpha    # max transcription rate (molecules/min)
    kTx0 = alpha0  # basal transcription
    kTl = kp       # translation rate

    state = np.array([0, 0, 0, 2, 1, 3], dtype=int)
    m = state[:3]
    p = state[3:]

    times = [0.0]
    states = [state.copy()]

    t = 0.0
    while t < t_max:
        m = state[:3]
        p = state[3:]

        # Repressor indices: p3→m1, p1→m2, p2→m3
        repressors = [p[2], p[0], p[1]]

        rates = np.array([
            # Transcription
            kTx / (1 + repressors[0]**n / max(1, repressors[0]**n)) + kTx0,
            kTx / (1 + repressors[1]**n / max(1, repressors[1]**n)) + kTx0,
            kTx / (1 + repressors[2]**n / max(1, repressors[2]**n)) + kTx0,
            # mRNA degradation
            km * max(0, m[0]),
            km * max(0, m[1]),
            km * max(0, m[2]),
            # Translation
            kTl * max(0, m[0]),
            kTl * max(0, m[1]),
            kTl * max(0, m[2]),
            # Protein degradation
            kp * max(0, p[0]),
            kp * max(0, p[1]),
            kp * max(0, p[2]),
        ], dtype=float)

        R = rates.sum()
        if R < 1e-12:
            break

        dt = rng.exponential(1.0 / R)
        t += dt

        # Choose reaction
        cumrates = np.cumsum(rates)
        u = rng.uniform(0, R)
        rxn = np.searchsorted(cumrates, u)

        # Update state
        changes = [
            [1, 0, 0, 0, 0, 0],   # TX m1
            [0, 1, 0, 0, 0, 0],   # TX m2
            [0, 0, 1, 0, 0, 0],   # TX m3
            [-1, 0, 0, 0, 0, 0],  # deg m1
            [0, -1, 0, 0, 0, 0],  # deg m2
            [0, 0, -1, 0, 0, 0],  # deg m3
            [0, 0, 0, 1, 0, 0],   # TL p1
            [0, 0, 0, 0, 1, 0],   # TL p2
            [0, 0, 0, 0, 0, 1],   # TL p3
            [0, 0, 0, -1, 0, 0],  # deg p1
            [0, 0, 0, 0, -1, 0],  # deg p2
            [0, 0, 0, 0, 0, -1],  # deg p3
        ]
        state = np.maximum(0, state + changes[rxn])
        times.append(t)
        states.append(state.copy())

    return np.array(times), np.array(states).T


# ── Parameter Sweep ───────────────────────────────────────────────────────────

def sweep_hill_coefficient(alpha=216, alpha0=0.216, beta=0.2):
    """Run ODE for several values of n, measure oscillation amplitude."""
    n_vals = np.arange(1.0, 6.1, 0.5)
    amplitudes = []
    periods = []

    for n in n_vals:
        t, y = run_ode(alpha, alpha0, beta, n=n, t_max=300)
        p1 = y[3]
        # Use last 50% of trajectory
        half = len(t) // 2
        p_late = p1[half:]
        t_late = t[half:]
        amp = (p_late.max() - p_late.min()) / 2
        amplitudes.append(amp)
        peaks, _ = find_peaks(p_late, height=amp * 0.5)
        if len(peaks) >= 2:
            period = np.mean(np.diff(t_late[peaks]))
        else:
            period = np.nan
        periods.append(period)

    return n_vals, np.array(amplitudes), np.array(periods)


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(alpha=216, alpha0=0.216, beta=0.2, n=2):
    t_ode, y_ode = run_ode(alpha, alpha0, beta, n)
    t_ssa, y_ssa = repressilator_gillespie(alpha, alpha0, beta, n, t_max=300)
    n_vals, amplitudes, periods = sweep_hill_coefficient(alpha, alpha0, beta)

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(
        f'Repressilator  (Elowitz & Leibler 2000)  —  n={n}, α={alpha}, β={beta}',
        color='white', fontsize=13, fontweight='bold'
    )

    gs = gridspec.GridSpec(3, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.06,
                           hspace=0.55, wspace=0.35)

    ax_ode    = fig.add_subplot(gs[0, :2])   # ODE time series
    ax_phase  = fig.add_subplot(gs[1, :2])   # Phase portrait
    ax_ssa    = fig.add_subplot(gs[2, :2])   # Gillespie SSA
    ax_sweep  = fig.add_subplot(gs[0, 2])    # Hill sweep amplitude
    ax_period = fig.add_subplot(gs[1, 2])    # Hill sweep period
    ax_info   = fig.add_subplot(gs[2, 2])    # Info text

    for ax in [ax_ode, ax_phase, ax_ssa, ax_sweep, ax_period, ax_info]:
        ax.set_facecolor('#0f3460')

    colors = ['#50fa7b', '#ff79c6', '#8be9fd']
    labels = ['LacI', 'TetR', 'CI']

    # ── ODE time series ───────────────────────────────────────────────────────
    for i, (c, lbl) in enumerate(zip(colors, labels)):
        ax_ode.plot(t_ode, y_ode[i + 3], color=c, lw=1.8, label=f'{lbl} protein', alpha=0.9)
    ax_ode.set_xlabel('Time (a.u.)', color='white')
    ax_ode.set_ylabel('Protein concentration', color='white')
    ax_ode.set_title('ODE Repressilator  (deterministic)', color='white', fontsize=9)
    ax_ode.tick_params(colors='white')
    ax_ode.legend(fontsize=8, facecolor='#16213e', labelcolor='white',
                  loc='upper right', ncol=3)
    for sp in ax_ode.spines.values():
        sp.set_edgecolor('#888')

    # ── Phase portrait (p1 vs p2 vs p3 in 3D projected) ──────────────────────
    half = len(t_ode) // 2
    p1, p2, p3 = y_ode[3, half:], y_ode[4, half:], y_ode[5, half:]
    # 2D projection: p1 vs p2
    sc = ax_phase.scatter(p1, p2,
                          c=np.linspace(0, 1, len(p1)), cmap='plasma',
                          s=1.5, alpha=0.6)
    ax_phase.set_xlabel('LacI protein', color='white')
    ax_phase.set_ylabel('TetR protein', color='white')
    ax_phase.set_title('Phase Portrait  (LacI vs TetR, steady state)',
                       color='white', fontsize=9)
    ax_phase.tick_params(colors='white')
    for sp in ax_phase.spines.values():
        sp.set_edgecolor('#888')
    plt.colorbar(sc, ax=ax_phase, label='time →', fraction=0.03).ax.yaxis.label.set_color('white')

    # ── Gillespie SSA ─────────────────────────────────────────────────────────
    for i, (c, lbl) in enumerate(zip(colors, labels)):
        ax_ssa.step(t_ssa, y_ssa[i + 3], color=c, lw=0.9, label=f'{lbl}', alpha=0.85)
    ax_ssa.set_xlabel('Time (a.u.)', color='white')
    ax_ssa.set_ylabel('Molecule count', color='white')
    ax_ssa.set_title('Gillespie SSA  (stochastic)', color='white', fontsize=9)
    ax_ssa.tick_params(colors='white')
    ax_ssa.legend(fontsize=8, facecolor='#16213e', labelcolor='white', ncol=3)
    for sp in ax_ssa.spines.values():
        sp.set_edgecolor('#888')

    # ── Hill sweep: amplitude ─────────────────────────────────────────────────
    ax_sweep.plot(n_vals, amplitudes, '-o', color='#50fa7b', lw=2, ms=6, markeredgecolor='white')
    ax_sweep.axvline(1.0, color='#ff5555', lw=1.2, ls='--', label='n=1 (no cooperativity)')
    ax_sweep.set_xlabel('Hill coefficient n', color='white')
    ax_sweep.set_ylabel('Oscillation amplitude', color='white')
    ax_sweep.set_title('Amplitude vs. Cooperativity', color='white', fontsize=9)
    ax_sweep.tick_params(colors='white')
    ax_sweep.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_sweep.spines.values():
        sp.set_edgecolor('#888')

    # ── Hill sweep: period ────────────────────────────────────────────────────
    valid = ~np.isnan(periods)
    ax_period.plot(n_vals[valid], periods[valid], '-s', color='#f5a623', lw=2, ms=6,
                   markeredgecolor='white')
    ax_period.set_xlabel('Hill coefficient n', color='white')
    ax_period.set_ylabel('Oscillation period (a.u.)', color='white')
    ax_period.set_title('Period vs. Cooperativity', color='white', fontsize=9)
    ax_period.tick_params(colors='white')
    for sp in ax_period.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    p_late = y_ode[3, len(t_ode)//2:]
    amp = (p_late.max() - p_late.min()) / 2
    t_late = t_ode[len(t_ode)//2:]
    peaks, _ = find_peaks(p_late, height=amp * 0.5)
    period = np.mean(np.diff(t_late[peaks])) if len(peaks) >= 2 else float('nan')

    info = (
        f"Repressilator parameters:\n"
        f"  n (Hill)    = {n}\n"
        f"  α (max TX)  = {alpha}\n"
        f"  α₀ (basal)  = {alpha0}\n"
        f"  β           = {beta}\n\n"
        f"Measured (ODE):\n"
        f"  Amplitude   = {amp:.1f}\n"
        f"  Period      = {period:.1f} a.u.\n\n"
        f"Circuit topology:\n"
        f"  LacI ─┤ tetR\n"
        f"  TetR ─┤ cI\n"
        f"  CI   ─┤ lacI\n\n"
        f"Hopf condition:\n"
        f"  n > 1 required\n"
        f"  Larger n → stronger oscillations\n\n"
        f"Reference:\n"
        f"  Elowitz & Leibler, Nature 2000"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.5)

    plt.show()

    print(f"\nRepressilator: n={n}, α={alpha}, β={beta}")
    print(f"Measured amplitude: {amp:.2f}")
    print(f"Measured period: {period:.2f} a.u.")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Repressilator ODE & SSA explorer')
    parser.add_argument('--n', type=float, default=2.0, help='Hill coefficient')
    parser.add_argument('--alpha', type=float, default=216.0, help='Max transcription rate')
    parser.add_argument('--beta', type=float, default=0.2, help='β = kp/km')
    args = parser.parse_args()

    print("Repressilator Simulator (Elowitz & Leibler 2000)")
    print("=" * 55)
    run(alpha=args.alpha, alpha0=args.alpha / 1000, beta=args.beta, n=args.n)
