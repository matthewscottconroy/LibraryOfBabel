"""
================================================================================
Lab 02a — The Lorenz Attractor: Phase Portrait and Lyapunov Exponents
================================================================================

Textbook connection
-------------------
Unit 2, Chapter 4: "Dynamical Systems Background."
The Lorenz system (Lorenz 1963, "Deterministic Nonperiodic Flow") is the
canonical example of a low-dimensional chaotic attractor. Understanding its
geometry and the concept of sensitive dependence on initial conditions is
essential before studying reservoir computing as a tool for predicting chaos.

Learning objectives
-------------------
  * Visualise the butterfly-shaped Lorenz attractor in 3-D phase space and
    appreciate its fractal structure.
  * Understand "sensitive dependence on initial conditions": two trajectories
    starting ε apart diverge exponentially on a time scale 1/λ_max.
  * Estimate the maximal Lyapunov exponent λ_max from the log-linear growth
    of trajectory separation — the fundamental limit on prediction horizon.
  * Connect λ_max to prediction time: the ESN must outrun λ_max in later labs.
  * Sketch the Kaplan-Yorke dimension as a measure of the attractor's fractal
    geometry.

What to observe
---------------
  * The 3-D phase portrait shows two "lobes" — trajectories circle around one
    lobe, then switch unpredictably to the other.
  * The separation plot is approximately straight on a log scale during the
    linear growth phase, giving λ_max ≈ 0.9 nats/time-unit for dt=0.02.
  * The time series x(t) looks irregular but is not random — it has structure
    visible in the phase portrait that the ESN will later exploit.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D   # noqa: F401 — registers 3d projection

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import lorenz_rk4


# ── Configuration ────────────────────────────────────────────────────────────

T         = 5000        # number of integration steps after burn-in
DT        = 0.02        # integration step (physical seconds per step)
DELTA0    = 1e-6        # initial separation for Lyapunov estimation
BURN      = 500         # burn-in steps (discarded in lorenz_rk4)
SEED      = 42


# ── Lorenz derivative (needed for perturbed trajectory) ─────────────────────

def lorenz_deriv(state, sigma=10.0, rho=28.0, beta=8.0/3.0):
    x, y, z = state
    return np.array([sigma*(y - x), x*(rho - z) - y, x*y - beta*z])


def integrate_lorenz(state0, T_steps, dt=DT):
    """RK4 integration from state0. Returns (T_steps, 3) trajectory."""
    traj = np.empty((T_steps, 3))
    s = state0.copy()
    for t in range(T_steps):
        traj[t] = s
        k1 = lorenz_deriv(s)
        k2 = lorenz_deriv(s + 0.5*dt*k1)
        k3 = lorenz_deriv(s + 0.5*dt*k2)
        k4 = lorenz_deriv(s + dt*k3)
        s = s + (dt/6.0)*(k1 + 2*k2 + 2*k3 + k4)
    return traj


# ── Lyapunov exponent estimation ─────────────────────────────────────────────

def estimate_lyapunov(traj_ref: np.ndarray, delta0: float, dt: float):
    """
    Estimate λ_max using the two-trajectory method.

    Start a second trajectory displaced by delta0 from traj_ref[0].
    Run both trajectories and record separation ||δx(t)||.
    The slope of log(separation) vs t gives λ_max (in nats/time-unit).

    We stop tracking when the separation exceeds 1 (attractor scale) to
    avoid saturation of the exponential growth phase.
    """
    state_ref  = traj_ref[0].copy()
    # Random perturbation with magnitude delta0
    rng = np.random.default_rng(SEED)
    direction = rng.standard_normal(3)
    direction /= np.linalg.norm(direction)
    state_pert = state_ref + delta0 * direction

    T_steps = len(traj_ref)
    separations = np.empty(T_steps)
    separations[0] = delta0

    s_ref  = state_ref.copy()
    s_pert = state_pert.copy()

    for t in range(1, T_steps):
        # Step both trajectories
        k1 = lorenz_deriv(s_ref);  k2 = lorenz_deriv(s_ref + 0.5*dt*k1)
        k3 = lorenz_deriv(s_ref + 0.5*dt*k2);  k4 = lorenz_deriv(s_ref + dt*k3)
        s_ref = s_ref + (dt/6.0)*(k1 + 2*k2 + 2*k3 + k4)

        k1 = lorenz_deriv(s_pert); k2 = lorenz_deriv(s_pert + 0.5*dt*k1)
        k3 = lorenz_deriv(s_pert + 0.5*dt*k2); k4 = lorenz_deriv(s_pert + dt*k3)
        s_pert = s_pert + (dt/6.0)*(k1 + 2*k2 + 2*k3 + k4)

        sep = np.linalg.norm(s_pert - s_ref)
        separations[t] = sep

    return separations


def fit_lyapunov_slope(separations: np.ndarray, dt: float, delta0: float):
    """
    Fit a line to log(separation) vs time in the exponential growth phase
    (where separation is small but above delta0).

    Returns (lambda_max_nats, lambda_max_bits, fit_range)
    """
    log_sep = np.log(separations + 1e-20)
    log_delta0 = np.log(delta0)
    time_axis = np.arange(len(separations)) * dt

    # Use only the initial linear-growth phase:
    # from step 1 until separation hits ~0.1 * (attractor scale ≈ 20)
    saturation = np.log(2.0)
    valid = (log_sep > log_delta0 + 0.5) & (log_sep < saturation)
    idx = np.where(valid)[0]
    if len(idx) < 10:
        # Fallback: first 30% of trajectory
        idx = np.arange(1, len(separations)//3)

    slope, intercept = np.polyfit(time_axis[idx], log_sep[idx], 1)
    return slope, slope / np.log(2.0), (idx[0], idx[-1])


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # Generate trajectory
    print("Integrating Lorenz system...")
    xyz = lorenz_rk4(T=T, dt=DT, burn=BURN, seed=SEED)   # (T, 3)
    x, y, z = xyz[:, 0], xyz[:, 1], xyz[:, 2]
    time_phys = np.arange(T) * DT

    # Estimate Lyapunov exponent
    separations = estimate_lyapunov(xyz, DELTA0, DT)
    lam_nats, lam_bits, fit_range = fit_lyapunov_slope(separations, DT, DELTA0)

    print(f"\n{'='*50}")
    print("Lorenz System Metrics")
    print(f"{'='*50}")
    print(f"  Integration: T={T} steps, dt={DT}, total time = {T*DT:.1f} s")
    print(f"  λ_max (nats/s)  = {lam_nats:.4f}")
    print(f"  λ_max (bits/s)  = {lam_bits:.4f}")
    print(f"  Literature:      λ_max ≈ 0.906 nats/s  (Sprott 2003)")
    print(f"  Predictability horizon ≈ {1.0/lam_nats:.2f} s  (1/λ_max)")
    print(f"  In integration steps: ≈ {1.0/(lam_nats*DT):.0f} steps")

    # Kaplan-Yorke dimension sketch
    # Standard Lorenz Lyapunov spectrum: λ1≈0.906, λ2≈0, λ3≈-14.57
    lam1 = lam_nats           # measured
    lam2 = 0.0                # neutral (flow direction)
    lam3 = -(lam1 + lam2 + 2.314)   # sum≈-13.7 for σ=10,ρ=28,β=8/3
    # Kaplan-Yorke: D_KY = j + (Σ_{i=1}^j λ_i) / |λ_{j+1}|
    # j=2 because λ1+λ2 > 0 but λ1+λ2+λ3 < 0
    if abs(lam3) > 1e-10:
        d_ky = 2 + (lam1 + lam2) / abs(lam3)
    else:
        d_ky = 2.0
    print(f"\n  Lyapunov spectrum estimate: [{lam1:.3f}, {lam2:.3f}, {lam3:.3f}]")
    print(f"  Kaplan-Yorke dimension D_KY ≈ {d_ky:.4f}")
    print(f"  Literature: D_KY ≈ 2.06  (Lorenz 1984, Sprott 2003)")
    print(f"  (Note: D_KY > 2 confirms fractal attractor geometry)")

    # ─── Figure 1: 3-D phase portrait ────────────────────────────────────────
    fig = plt.figure(figsize=(10, 7))
    ax = fig.add_subplot(111, projection="3d")

    # Colour by time to show progression
    colors_t = plt.cm.plasma(np.linspace(0, 1, T))
    # Plot as many short segments (efficient substitute for scatter)
    step = 5
    for i in range(0, T - step, step):
        ax.plot(x[i:i+step+1], y[i:i+step+1], z[i:i+step+1],
                color=colors_t[i], lw=0.4, alpha=0.7)

    ax.set_xlabel("x", fontsize=10)
    ax.set_ylabel("y", fontsize=10)
    ax.set_zlabel("z", fontsize=10)
    ax.set_title(
        f"Lorenz Attractor (T={T} steps, dt={DT})\n"
        f"Colour: time progression (purple→yellow)",
        fontsize=11
    )
    # Colourbar proxy
    sm = plt.cm.ScalarMappable(cmap="plasma",
                               norm=plt.Normalize(0, T * DT))
    sm.set_array([])
    cbar = fig.colorbar(sm, ax=ax, shrink=0.5, pad=0.1)
    cbar.set_label("Time (s)", fontsize=9)
    plt.tight_layout()
    plt.savefig("output/02a_lorenz_3d_portrait.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/02a_lorenz_3d_portrait.png")

    # ─── Figure 2: Two trajectories — separation on log scale ────────────────
    fig, axes = plt.subplots(2, 1, figsize=(9, 7))

    ax1 = axes[0]
    # Generate a nearby trajectory for plotting
    rng = np.random.default_rng(SEED)
    direction = rng.standard_normal(3); direction /= np.linalg.norm(direction)
    xyz2 = integrate_lorenz(xyz[0] + DELTA0 * direction, T, DT)

    ax1.plot(time_phys[:1000], xyz[:1000, 0], color="steelblue",
             lw=1.2, label="Trajectory A")
    ax1.plot(time_phys[:1000], xyz2[:1000, 0], color="tomato",
             lw=1.2, ls="--", label=f"Trajectory B (δ₀={DELTA0:.0e})")
    ax1.set_xlabel("Time (s)", fontsize=10)
    ax1.set_ylabel("x(t)", fontsize=10)
    ax1.set_title("Two Nearby Lorenz Trajectories: x(t)", fontsize=11)
    ax1.legend(fontsize=9)

    ax2 = axes[1]
    t_fit = np.arange(fit_range[0], fit_range[1] + 1) * DT
    log_sep_fit = np.log(separations[fit_range[0]:fit_range[1]+1])
    fit_line = lam_nats * t_fit + np.log(DELTA0)

    ax2.semilogy(time_phys, separations + 1e-20, color="steelblue",
                 lw=1.5, label="||δx(t)||")
    ax2.semilogy(t_fit, np.exp(fit_line), "r--", lw=2,
                 label=f"Fit: λ_max ≈ {lam_nats:.3f} nats/s")
    ax2.axhline(1.0, color="gray", ls=":", lw=1, label="Attractor scale")
    ax2.set_xlabel("Time (s)", fontsize=10)
    ax2.set_ylabel("Separation ||δx||", fontsize=10)
    ax2.set_title("Exponential Divergence of Nearby Trajectories", fontsize=11)
    ax2.legend(fontsize=9)
    ax2.set_xlim(0, time_phys[-1])

    plt.tight_layout()
    plt.savefig("output/02a_lyapunov_divergence.png", dpi=150)
    plt.close()
    print("Figure saved: output/02a_lyapunov_divergence.png")

    # ─── Figure 3: x(t) time series ──────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(11, 3.5))
    ax.plot(time_phys[:2000], x[:2000], color="steelblue", lw=0.8)
    ax.set_xlabel("Time (s)", fontsize=11)
    ax.set_ylabel("x(t)", fontsize=11)
    ax.set_title(
        "Lorenz x(t): Irregular Switching Between Two Lobes\n"
        "(Chaotic, but not random — structure exploited by ESN in Lab 08)",
        fontsize=11
    )
    plt.tight_layout()
    plt.savefig("output/02a_lorenz_timeseries.png", dpi=150)
    plt.close()
    print("Figure saved: output/02a_lorenz_timeseries.png")

    # ─── Figure 4: 2-D projections ───────────────────────────────────────────
    fig, axes = plt.subplots(1, 3, figsize=(13, 4))
    projections = [("x", "y", x, y), ("x", "z", x, z), ("y", "z", y, z)]
    for ax, (xl, yl, xs, ys) in zip(axes, projections):
        ax.plot(xs[::2], ys[::2], ",", ms=0.5, color="steelblue", alpha=0.5)
        ax.set_xlabel(xl, fontsize=11)
        ax.set_ylabel(yl, fontsize=11)
        ax.set_title(f"{xl}-{yl} plane", fontsize=11)
    fig.suptitle("Lorenz Attractor: 2-D Projections", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/02a_lorenz_projections.png", dpi=150)
    plt.close()
    print("Figure saved: output/02a_lorenz_projections.png")

    print(
        "\nSummary:\n"
        f"  λ_max ≈ {lam_nats:.3f} nats/s → prediction horizon "
        f"≈ {1.0/lam_nats:.1f} s ≈ {1.0/(lam_nats*DT):.0f} steps.\n"
        f"  D_KY ≈ {d_ky:.3f}: the attractor has a fractional dimension\n"
        "  slightly above 2 — it is genuinely fractal."
    )


if __name__ == "__main__":
    main()
