"""
Turing Patterns: Reaction-Diffusion Systems
=============================================
Tier 2.1 — Systems Biology: Reaction-Diffusion PDEs

Concepts demonstrated:
  - Activator-inhibitor and activator-substrate models
  - Turing instability condition: d = D_v/D_u >> 1
  - Pattern formation from near-uniform initial conditions + noise
  - Spots vs. stripes from parameter changes
  - Method of lines: spatial discretization → large ODE system
  - Effect of domain size on wavelength selection

Usage:
    python 12_turing_patterns.py [--model MODEL] [--d RATIO]

Models:
    gierer      — Gierer-Meinhardt (activator-inhibitor)
    gray_scott  — Gray-Scott (feed-kill reactions)
    schnakenberg — Schnakenberg (two-component)

Key insight:
    Turing's 1952 insight: a short-range activator + long-range
    inhibitor can break spatial symmetry spontaneously. The key
    condition is that the inhibitor diffuses much faster (d >> 1).
    This mechanism explains animal coat patterns, digit spacing,
    and many developmental decisions.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import matplotlib.animation as animation
from scipy.integrate import solve_ivp


# ── Reaction-Diffusion Models ─────────────────────────────────────────────────

MODELS = {
    "gierer": {
        "params": {"a": 0.1, "b": 0.9, "c": 1.0, "d": 30.0,
                   "Du": 0.01, "Dv": 0.30},
        "u0_eq": 1.0, "v0_eq": 1.0,
        "description": (
            "Gierer-Meinhardt Activator-Inhibitor\n"
            "∂u/∂t = Du·∇²u + c·(u²/v - a·u)\n"
            "∂v/∂t = Dv·∇²v + c·(u² - b·v)\n\n"
            "u = activator (e.g., WNT signaling)\n"
            "v = inhibitor (fast diffusion required)\n\n"
            "Turing condition: Dv/Du >> 1\n"
            "→ spots on animal coats, digit formation"
        ),
        "rhs": lambda u, v, p: (
            p['c'] * (u**2 / (v + 1e-9) - p['a'] * u),
            p['c'] * (u**2 - p['b'] * v)
        ),
    },
    "gray_scott": {
        "params": {"F": 0.035, "k": 0.065,
                   "Du": 0.16, "Dv": 0.08},
        "u0_eq": 1.0, "v0_eq": 0.0,
        "description": (
            "Gray-Scott Reaction-Diffusion\n"
            "∂u/∂t = Du·∇²u - u·v² + F·(1-u)\n"
            "∂v/∂t = Dv·∇²v + u·v² - (F+k)·v\n\n"
            "u = substrate (fed from boundary)\n"
            "v = autocatalytic product\n\n"
            "F = feed rate, k = kill rate\n"
            "Wide parameter space → spots, stripes, worms"
        ),
        "rhs": lambda u, v, p: (
            -u * v**2 + p['F'] * (1 - u),
            u * v**2 - (p['F'] + p['k']) * v
        ),
    },
    "schnakenberg": {
        "params": {"a": 0.1, "b": 0.9, "Du": 0.01, "Dv": 0.25},
        "u0_eq": 0.5, "v0_eq": 2.0,
        "description": (
            "Schnakenberg Two-Component\n"
            "∂u/∂t = Du·∇²u + a - u + u²·v\n"
            "∂v/∂t = Dv·∇²v + b - u²·v\n\n"
            "Simplest Turing model.\n"
            "Steady state: u*=a+b, v*=b/(a+b)²\n\n"
            "Instability when Dv/Du > dc (critical ratio)\n"
            "Pattern wavelength ∝ 1/√wave number"
        ),
        "rhs": lambda u, v, p: (
            p['a'] - u + u**2 * v,
            p['b'] - u**2 * v
        ),
    },
}


# ── PDE Solver: Method of Lines ───────────────────────────────────────────────

def laplacian_2d(u, dx):
    """5-point stencil Laplacian with periodic BC."""
    return (
        np.roll(u, 1, 0) + np.roll(u, -1, 0) +
        np.roll(u, 1, 1) + np.roll(u, -1, 1) - 4 * u
    ) / dx**2


def run_rd_simulation(model_name, N=64, L=10.0, t_end=500, n_frames=40, seed=42):
    """
    Simulate reaction-diffusion PDE on N×N grid using explicit Euler.
    Returns list of (u, v) snapshots at evenly spaced times.
    """
    model = MODELS[model_name]
    params = model['params']
    Du, Dv = params['Du'], params['Dv']
    rhs = model['rhs']
    u0_eq, v0_eq = model['u0_eq'], model['v0_eq']

    rng = np.random.default_rng(seed)
    dx = L / N
    # Stability: dt < dx²/(4·max(Du,Dv))
    dt = 0.4 * dx**2 / (4 * max(Du, Dv))

    # Initial condition: homogeneous + small noise
    noise_scale = 0.05
    u = u0_eq * np.ones((N, N)) + noise_scale * rng.standard_normal((N, N))
    v = v0_eq * np.ones((N, N)) + noise_scale * rng.standard_normal((N, N))
    u = np.maximum(u, 1e-6)
    v = np.maximum(v, 1e-6)

    n_steps = int(t_end / dt)
    save_every = max(1, n_steps // n_frames)

    snapshots = [(0.0, u.copy(), v.copy())]
    for step in range(n_steps):
        Lu = laplacian_2d(u, dx)
        Lv = laplacian_2d(v, dx)
        ru, rv = rhs(u, v, params)
        u = u + dt * (Du * Lu + ru)
        v = v + dt * (Dv * Lv + rv)
        u = np.maximum(u, 0)
        v = np.maximum(v, 0)
        if (step + 1) % save_every == 0:
            t = (step + 1) * dt
            snapshots.append((t, u.copy(), v.copy()))

    return snapshots


# ── Dispersion Relation ───────────────────────────────────────────────────────

def dispersion_gierer(k_vals, params):
    """Growth rate σ(k) for Gierer-Meinhardt linearized about steady state."""
    a, b, c = params['a'], params['b'], params['c']
    Du, Dv = params['Du'], params['Dv']
    # Steady state: u* = (c-a)/... (simplified)
    # Use eigenvalue of linearized system minus diffusion
    u_star = 1.0; v_star = 1.0
    f_u = c * (2 * u_star / v_star - a)
    f_v = -c * u_star**2 / v_star**2
    g_u = 2 * c * u_star
    g_v = -c * b
    sigmas = []
    for k2 in k_vals**2:
        J = np.array([[f_u - Du * k2, f_v],
                      [g_u, g_v - Dv * k2]])
        eigvals = np.linalg.eigvals(J)
        sigmas.append(eigvals.real.max())
    return np.array(sigmas)


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(model_name='gierer', d_ratio=None):
    model = MODELS[model_name]
    params = dict(model['params'])

    if d_ratio is not None and d_ratio > 0:
        Du_base = params['Du']
        params['Dv'] = Du_base * d_ratio

    print(f"Running {model_name} simulation (N=64, this may take ~30s)...")
    snapshots = run_rd_simulation(model_name, N=64, L=10.0, t_end=300, n_frames=6)

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(
        f'Turing Pattern Formation — {model_name.replace("_", "-").title()}  '
        f'(Du={params["Du"]}, Dv={params["Dv"]})',
        color='white', fontsize=13, fontweight='bold'
    )

    gs = gridspec.GridSpec(3, 4, figure=fig,
                           left=0.04, right=0.97, top=0.88, bottom=0.06,
                           hspace=0.35, wspace=0.25)

    # Show 4 time snapshots of u (activator)
    n_show = min(4, len(snapshots))
    indices = np.linspace(0, len(snapshots) - 1, n_show, dtype=int)

    axes_u = [fig.add_subplot(gs[0, i]) for i in range(n_show)]
    axes_v = [fig.add_subplot(gs[1, i]) for i in range(n_show)]
    ax_disp = fig.add_subplot(gs[2, :2])
    ax_info = fig.add_subplot(gs[2, 2:])

    for ax in axes_u + axes_v + [ax_disp, ax_info]:
        ax.set_facecolor('#0f3460')

    u_all = [snapshots[i][1] for i in indices]
    v_all = [snapshots[i][2] for i in indices]
    u_vmin = min(u.min() for u in u_all)
    u_vmax = max(u.max() for u in u_all)
    v_vmin = min(v.min() for v in v_all)
    v_vmax = max(v.max() for v in v_all)

    for col, idx in enumerate(indices):
        t, u, v = snapshots[idx]
        im_u = axes_u[col].imshow(u, cmap='RdYlGn', origin='lower',
                                   vmin=u_vmin, vmax=u_vmax, interpolation='bilinear')
        axes_u[col].set_title(f't={t:.0f}', color='white', fontsize=8)
        axes_u[col].axis('off')
        if col == 0:
            axes_u[col].set_ylabel('Activator u', color='white', fontsize=8)

        im_v = axes_v[col].imshow(v, cmap='PuBu', origin='lower',
                                   vmin=v_vmin, vmax=v_vmax, interpolation='bilinear')
        axes_v[col].axis('off')
        if col == 0:
            axes_v[col].set_ylabel('Inhibitor v', color='white', fontsize=8)

    # ── Dispersion relation (Gierer-Meinhardt only) ───────────────────────────
    if model_name == 'gierer':
        k_vals = np.linspace(0.01, 5, 200)
        sigma = dispersion_gierer(k_vals, params)
        ax_disp.plot(k_vals, sigma, color='#50fa7b', lw=2)
        ax_disp.axhline(0, color='#888', lw=1, ls='--')
        ax_disp.fill_between(k_vals, sigma, 0,
                              where=sigma > 0, color='#50fa7b', alpha=0.25,
                              label='Turing unstable modes')
        k_max = k_vals[np.argmax(sigma)]
        ax_disp.axvline(k_max, color='#ff79c6', lw=1.5, ls='--',
                        label=f'Fastest mode k={k_max:.2f}')
        ax_disp.set_xlabel('Wave number k', color='white')
        ax_disp.set_ylabel('Growth rate σ(k)', color='white')
        ax_disp.set_title('Dispersion Relation  (Turing instability)', color='white', fontsize=9)
        ax_disp.tick_params(colors='white')
        ax_disp.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
        for sp in ax_disp.spines.values():
            sp.set_edgecolor('#888')
    else:
        ax_disp.text(0.5, 0.5, f'Final pattern\n(activator)',
                     transform=ax_disp.transAxes, ha='center', va='center',
                     color='white', fontsize=10)
        ax_disp.imshow(snapshots[-1][1], cmap='RdYlGn', origin='lower',
                       interpolation='bilinear')
        ax_disp.axis('off')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    final_u = snapshots[-1][1]
    pattern_amplitude = final_u.max() - final_u.min()

    info = (
        f"Model: {model_name}\n\n"
        f"{model['description']}\n\n"
        f"Parameters:\n"
        + '\n'.join(f"  {k} = {v}" for k, v in params.items()) +
        f"\n\nSimulation result:\n"
        f"  Dv/Du ratio = {params['Dv']/params['Du']:.1f}\n"
        f"  Pattern amplitude = {pattern_amplitude:.3f}\n"
        f"  Grid: 64×64, periodic BC\n\n"
        f"Turing condition:\n"
        f"  Need Dv >> Du\n"
        f"  And: activator self-activates\n"
        f"       inhibitor faster diffuser"
    )
    ax_info.text(0.03, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nModel: {model_name}")
    print(f"Du={params['Du']}, Dv={params['Dv']}, ratio={params['Dv']/params['Du']:.1f}")
    print(f"Final pattern amplitude (u): {pattern_amplitude:.4f}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Turing pattern simulator')
    parser.add_argument('--model', default='gierer',
                        choices=['gierer', 'gray_scott', 'schnakenberg'])
    parser.add_argument('--d', type=float, default=None,
                        help='Override Dv/Du diffusion ratio')
    args = parser.parse_args()

    print("Turing Pattern Formation: Reaction-Diffusion PDEs")
    print("=" * 55)
    run(model_name=args.model, d_ratio=args.d)
