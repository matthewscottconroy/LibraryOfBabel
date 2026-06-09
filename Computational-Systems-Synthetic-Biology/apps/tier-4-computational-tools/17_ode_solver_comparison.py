"""
ODE Solver Comparison: Explicit vs. Stiff Methods
===================================================
Tier 4.1 — Computational Tools: Scientific Computing

Concepts demonstrated:
  - Euler method (1st order explicit)
  - RK4 (4th order explicit)
  - Adaptive RK45 (Dormand-Prince, variable step size)
  - LSODA / BDF for stiff ODEs
  - Stiffness: λ_max / λ_min ratio of Jacobian eigenvalues
  - Step-size stability regions for each method
  - Error accumulation and global error vs. step size
  - Repressilator as stiff system example

Usage:
    python 17_ode_solver_comparison.py [--system SYSTEM]

Systems:
    lotka      — Non-stiff: Lotka-Volterra predator-prey
    stiff      — Stiff: Robertson chemical kinetics
    repressilator — Genetic oscillator (mildly stiff)

Key insight:
    Explicit methods fail for stiff systems: the step size needed for
    stability is far smaller than the step size needed for accuracy.
    Implicit methods (BDF, Radau) solve a nonlinear system at each step
    but remain stable with large step sizes. LSODA automatically switches
    between stiff and non-stiff modes.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.integrate import solve_ivp
import time


# ── ODE Systems ───────────────────────────────────────────────────────────────

SYSTEMS = {
    "lotka": {
        "name": "Lotka-Volterra (non-stiff)",
        "y0": [2.0, 1.0],
        "t_span": (0, 50),
        "rhs": lambda t, y: [
            1.0 * y[0] - 0.2 * y[0] * y[1],
            -1.0 * y[1] + 0.15 * y[0] * y[1],
        ],
        "labels": ["Prey", "Predator"],
        "stiff_ratio": None,
        "description": (
            "Lotka-Volterra (non-stiff)\n"
            "dx/dt = ax – bxy\n"
            "dy/dt = –cy + dxy\n\n"
            "Stiffness index ≈ 1\n"
            "All explicit methods work.\n\n"
            "Explicit RK4 is fast and accurate."
        ),
    },
    "stiff": {
        "name": "Robertson Kinetics (stiff)",
        "y0": [1.0, 0.0, 0.0],
        "t_span": (0, 1e11),
        "rhs": lambda t, y: [
            -0.04 * y[0] + 1e4 * y[1] * y[2],
             0.04 * y[0] - 1e4 * y[1] * y[2] - 3e7 * y[1]**2,
             3e7 * y[1]**2,
        ],
        "labels": ["A", "B", "C"],
        "stiff_ratio": "~1e14",
        "description": (
            "Robertson Chemical Kinetics (stiff)\n"
            "Classic stiff benchmark.\n\n"
            "Rate constants: 0.04, 1e4, 3e7\n"
            "Stiffness ratio ~ 10¹⁴\n\n"
            "Euler/RK4 need dt < 10⁻¹⁴\n"
            "→ billions of steps for t_max=10¹¹\n\n"
            "BDF/LSODA handle this with\n"
            "~100 adaptive steps total."
        ),
    },
    "repressilator": {
        "name": "Repressilator (moderately stiff)",
        "y0": [0.0, 0.0, 0.0, 2.0, 1.0, 3.0],
        "t_span": (0, 300),
        "rhs": lambda t, y: [
            216 / (1 + y[5]**2) + 0.216 - y[0],  # m1
            216 / (1 + y[3]**2) + 0.216 - y[1],  # m2
            216 / (1 + y[4]**2) + 0.216 - y[2],  # m3
            0.2 * (y[0] - y[3]),                   # p1
            0.2 * (y[1] - y[4]),                   # p2
            0.2 * (y[2] - y[5]),                   # p3
        ],
        "labels": ["m1", "m2", "m3", "p1", "p2", "p3"],
        "stiff_ratio": "~100-1000",
        "description": (
            "Repressilator (moderately stiff)\n"
            "Three-gene genetic oscillator.\n\n"
            "Stiffness arises from mismatch\n"
            "between mRNA (fast) and protein\n"
            "(slow) timescales.\n\n"
            "Ratio β = k_protein/k_mRNA = 0.2\n"
            "creates moderate stiffness.\n\n"
            "RK45 works but LSODA is faster."
        ),
    },
}


# ── Manual ODE Solvers ────────────────────────────────────────────────────────

def euler_method(rhs, y0, t_span, dt):
    t0, tf = t_span
    t_vals = [t0]
    y = np.array(y0, dtype=float)
    y_vals = [y.copy()]
    t = t0
    while t < tf:
        dt_eff = min(dt, tf - t)
        y = y + dt_eff * np.array(rhs(t, y))
        t += dt_eff
        t_vals.append(t)
        y_vals.append(y.copy())
    return np.array(t_vals), np.array(y_vals).T


def rk4_method(rhs, y0, t_span, dt):
    t0, tf = t_span
    t_vals = [t0]
    y = np.array(y0, dtype=float)
    y_vals = [y.copy()]
    t = t0
    while t < tf:
        dt_eff = min(dt, tf - t)
        k1 = np.array(rhs(t, y))
        k2 = np.array(rhs(t + dt_eff/2, y + dt_eff * k1/2))
        k3 = np.array(rhs(t + dt_eff/2, y + dt_eff * k2/2))
        k4 = np.array(rhs(t + dt_eff, y + dt_eff * k3))
        y = y + dt_eff * (k1 + 2*k2 + 2*k3 + k4) / 6
        t += dt_eff
        t_vals.append(t)
        y_vals.append(y.copy())
    return np.array(t_vals), np.array(y_vals).T


def run_scipy_solver(method, rhs, y0, t_span, t_eval):
    t0 = time.perf_counter()
    sol = solve_ivp(rhs, t_span, y0, method=method, t_eval=t_eval,
                    rtol=1e-8, atol=1e-10, dense_output=False)
    elapsed = time.perf_counter() - t0
    return sol.t, sol.y, elapsed, len(sol.t)


# ── Error Analysis ────────────────────────────────────────────────────────────

def error_vs_stepsize(rhs, y0, t_span, ref_method='LSODA'):
    """Compute global error at t_span[1] vs step size for Euler and RK4."""
    # Get reference solution
    t_eval_ref = [t_span[1]]
    sol_ref = solve_ivp(rhs, t_span, y0, method=ref_method,
                        t_eval=t_eval_ref, rtol=1e-12, atol=1e-14)
    y_ref = sol_ref.y[:, -1]

    dt_vals = np.logspace(-2, 0, 15)
    euler_errors = []
    rk4_errors = []

    for dt in dt_vals:
        n_steps = int((t_span[1] - t_span[0]) / dt)
        if n_steps > 100000:
            euler_errors.append(np.nan)
            rk4_errors.append(np.nan)
            continue
        try:
            t_e, y_e = euler_method(rhs, y0, t_span, dt)
            euler_errors.append(np.linalg.norm(y_e[:, -1] - y_ref))
        except Exception:
            euler_errors.append(np.nan)

        try:
            t_r, y_r = rk4_method(rhs, y0, t_span, dt)
            rk4_errors.append(np.linalg.norm(y_r[:, -1] - y_ref))
        except Exception:
            rk4_errors.append(np.nan)

    return dt_vals, np.array(euler_errors), np.array(rk4_errors)


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(system_name='repressilator'):
    sys = SYSTEMS[system_name]
    rhs = sys['rhs']
    y0 = sys['y0']
    t_span = sys['t_span']
    labels = sys['labels']

    # Reference solution
    t0, tf = t_span
    t_eval_ref = np.linspace(t0, tf, 2000)

    print(f"Running {system_name}...")

    # Reference (most accurate)
    t_ref, y_ref, time_ref, nsteps_ref = run_scipy_solver('LSODA', rhs, y0, t_span, t_eval_ref)

    # Compare methods (RK45 and BDF on subset of time range if stiff)
    t_eval_short = np.linspace(t0, min(tf, t0 + (tf - t0) * 0.001), 500) if system_name == 'stiff' \
        else t_eval_ref

    results = {}
    for method in ['RK45', 'BDF', 'LSODA']:
        try:
            t_m, y_m, time_m, nsteps_m = run_scipy_solver(method, rhs, y0, t_span, t_eval_short)
            results[method] = {'t': t_m, 'y': y_m, 'time': time_m, 'nsteps': nsteps_m}
        except Exception as e:
            results[method] = None
            print(f"  {method} failed: {e}")

    # Error vs. step size (only for non-stiff or short window)
    if system_name != 'stiff':
        t_span_short = (t0, min(tf, t0 + 10))
        dt_vals, euler_err, rk4_err = error_vs_stepsize(rhs, y0, t_span_short)
    else:
        dt_vals = None

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'ODE Solver Comparison — {sys["name"]}',
                 color='white', fontsize=12, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_traj   = fig.add_subplot(gs[0, :2])
    ax_error  = fig.add_subplot(gs[1, 0])
    ax_perf   = fig.add_subplot(gs[1, 1])
    ax_info   = fig.add_subplot(gs[:, 2])

    for ax in [ax_traj, ax_error, ax_perf, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Trajectories ──────────────────────────────────────────────────────────
    colors = ['#50fa7b', '#ff79c6', '#8be9fd', '#f5a623', '#f1fa8c', '#ff5555']
    for i, label in enumerate(labels[:min(3, len(labels))]):
        ax_traj.plot(t_ref, y_ref[i], color=colors[i], lw=2.5, label=label)

    ax_traj.set_xlabel('Time', color='white')
    ax_traj.set_ylabel('Concentration', color='white')
    ax_traj.set_title(f'Reference Solution (LSODA, {nsteps_ref} steps)',
                      color='white', fontsize=9)
    ax_traj.tick_params(colors='white')
    ax_traj.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
    for sp in ax_traj.spines.values():
        sp.set_edgecolor('#888')

    # ── Error vs step size ────────────────────────────────────────────────────
    if dt_vals is not None:
        valid_e = ~np.isnan(euler_err)
        valid_r = ~np.isnan(rk4_err)
        ax_error.loglog(dt_vals[valid_e], euler_err[valid_e], '-o',
                        color='#ff5555', lw=2, ms=5, label='Euler (O(h))')
        ax_error.loglog(dt_vals[valid_r], rk4_err[valid_r], '-s',
                        color='#50fa7b', lw=2, ms=5, label='RK4 (O(h⁴))')
        # Reference slopes
        h = dt_vals[valid_e]
        ax_error.loglog(h, 0.1 * h, '--', color='#888', lw=1, alpha=0.6, label='O(h)')
        ax_error.loglog(h, 0.01 * h**4, ':', color='#888', lw=1, alpha=0.6, label='O(h⁴)')
        ax_error.set_xlabel('Step size h', color='white')
        ax_error.set_ylabel('Global error |y(T) - y_ref|', color='white')
        ax_error.set_title('Error vs. Step Size', color='white', fontsize=9)
        ax_error.tick_params(colors='white')
        ax_error.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax_error.spines.values():
            sp.set_edgecolor('#888')
    else:
        ax_error.text(0.5, 0.5,
                      'Error analysis skipped\n(stiff system: Euler/RK4\nwould need dt<10⁻¹⁴)',
                      transform=ax_error.transAxes, ha='center', va='center',
                      color='white', fontsize=9)
        ax_error.axis('off')

    # ── Performance comparison ────────────────────────────────────────────────
    method_names = [m for m in results if results[m] is not None]
    times_cpu = [results[m]['time'] * 1000 for m in method_names]
    nsteps_all = [results[m]['nsteps'] for m in method_names]

    x = np.arange(len(method_names))
    bar_colors = ['#50fa7b', '#8be9fd', '#f5a623'][:len(method_names)]
    ax_perf.bar(x, times_cpu, color=bar_colors, edgecolor='white', width=0.4, label='CPU time (ms)')
    ax_perf2 = ax_perf.twinx()
    ax_perf2.plot(x, nsteps_all, 'D--', color='#ff79c6', lw=2, ms=8, label='Steps taken')
    ax_perf2.set_ylabel('Number of steps', color='#ff79c6')
    ax_perf2.tick_params(colors='#ff79c6')

    ax_perf.set_xticks(x)
    ax_perf.set_xticklabels(method_names, color='white', fontsize=9)
    ax_perf.set_ylabel('CPU time (ms)', color='white')
    ax_perf.set_title('Solver Performance', color='white', fontsize=9)
    ax_perf.tick_params(colors='white')
    for sp in ax_perf.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    perf_text = "Solver performance:\n"
    for m in method_names:
        r = results[m]
        perf_text += f"  {m:8s}: {r['time']*1000:.1f}ms, {r['nsteps']} steps\n"

    info = (
        f"System: {sys['name']}\n"
        f"t_span: [{t_span[0]:.0e}, {t_span[1]:.0e}]\n"
        f"Stiffness ratio: {sys.get('stiff_ratio', 'Low')}\n\n"
        f"{sys['description']}\n\n"
        f"{perf_text}\n"
        f"Method guide:\n"
        f"  RK45 — non-stiff, adaptive\n"
        f"  BDF  — stiff, implicit\n"
        f"  LSODA — auto-switches\n\n"
        f"Stability condition (Euler):\n"
        f"  |1 + h·λ| ≤ 1\n"
        f"  For stiff λ << 0: h very small"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='ODE solver comparison')
    parser.add_argument('--system', default='repressilator',
                        choices=list(SYSTEMS.keys()))
    args = parser.parse_args()

    print("ODE Solver Comparison: Explicit vs. Stiff Methods")
    print("=" * 55)
    run(system_name=args.system)
