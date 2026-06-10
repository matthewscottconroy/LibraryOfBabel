"""
================================================================================
Lab 04 — Empirically Verifying the Echo State Property (ESP)
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 6: "The Echo State Property."
The Echo State Property (Jaeger 2001) is the fundamental sufficient condition
for an ESN to be well-defined: the reservoir state must be uniquely determined
by the input history, not by the initial condition. This lab demonstrates ESP
empirically and shows where it breaks down.

Formal statement: An ESN has the ESP iff for any bounded input sequence, any
two reservoir trajectories driven by the same input converge to the same state,
regardless of their starting points.

Learning objectives
-------------------
  * Empirically verify that for ρ < 1, all trajectories from different initial
    states converge to a single "echo state" — confirming the ESP holds.
  * Show that for ρ > 1, trajectories may diverge — the ESP can be violated.
  * Measure convergence speed and relate it to ρ as a proxy Lyapunov exponent.
  * Understand why ρ < 1 is *sufficient* but not *necessary* for the ESP:
    even ρ slightly above 1 may exhibit the ESP for specific inputs.
  * Relate the ESP to the reservoir's fading memory: convergence and forgetting
    are two sides of the same coin.

What to observe
---------------
  * For ρ = 0.1: trajectories converge within ~10 steps (rapid forgetting).
  * For ρ = 0.9: convergence takes ~50-100 steps (slower, more memory).
  * For ρ = 0.99: convergence is very slow (approaches the ESP boundary).
  * For ρ = 1.1: trajectories may or may not converge — ESP is borderline.
  * For ρ = 1.5: trajectories diverge — ESP is clearly violated.
  * The log-distance plots show approximately linear decay for ρ < 1,
    with slope ≈ log(ρ) per step (the theoretical convergence rate bound).
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork


# ── Configuration ────────────────────────────────────────────────────────────

T_INPUT      = 500       # length of driving input sequence
N_RESERVOIR  = 50        # keep small for speed
N_INIT       = 10        # number of different initial conditions
RHO_VALUES   = [0.1, 0.5, 0.9, 0.99, 1.1, 1.5]
SEED         = 42
CONV_THRESH  = 1e-4      # threshold below which we call it "converged"


# ── Helper: run multiple trajectories from different initial conditions ───────

def run_multiple_trajectories(rho: float,
                               inputs: np.ndarray,
                               n_init: int,
                               seed: int = 0) -> tuple[np.ndarray, EchoStateNetwork]:
    """
    Build an ESN with the given ρ, then drive it from n_init different
    random initial states using the same input sequence.

    Returns
    -------
    trajectories : (n_init, T, N) array of reservoir states
    esn          : the ESN object (for inspecting weights)
    """
    esn = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=rho,
        input_scaling=0.5,
        connectivity=0.1,
        bias_scaling=0.1,
        washout=0,
        extend_with_input=False,
        seed=seed,
    )
    esn._build_reservoir(1)   # initialise weights

    T = inputs.shape[0]
    rng = np.random.default_rng(seed + 100)
    trajectories = np.empty((n_init, T, N_RESERVOIR))

    for trial in range(n_init):
        # Random initial state with amplitude ~1
        x0 = rng.standard_normal(N_RESERVOIR) * 0.5
        states, _ = esn.run(inputs, initial_state=x0)
        trajectories[trial] = states

    return trajectories, esn


def pairwise_distances(trajectories: np.ndarray) -> np.ndarray:
    """
    For each time step, compute the mean pairwise distance between all
    trajectory pairs.

    Parameters
    ----------
    trajectories : (n_init, T, N)

    Returns
    -------
    distances : (T,) mean pairwise distance at each time step
    """
    n_init, T, N = trajectories.shape
    dist_sum = np.zeros(T)
    count = 0
    for i in range(n_init):
        for j in range(i + 1, n_init):
            diff = trajectories[i] - trajectories[j]   # (T, N)
            dist_sum += np.linalg.norm(diff, axis=1)
            count += 1
    return dist_sum / max(count, 1)


def convergence_time(distances: np.ndarray, threshold: float = CONV_THRESH) -> int:
    """Return the first time step where distance drops below threshold."""
    idx = np.where(distances < threshold)[0]
    return int(idx[0]) if len(idx) > 0 else len(distances)


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    rng = np.random.default_rng(SEED)
    # Random uniform input — mimics a generic bounded input stream
    inputs = rng.uniform(-0.5, 0.5, (T_INPUT, 1))

    print("=" * 60)
    print("Echo State Property Empirical Verification")
    print("=" * 60)
    print(f"  N={N_RESERVOIR}, T_input={T_INPUT}, n_initial_conditions={N_INIT}")
    print(f"  Convergence threshold: {CONV_THRESH:.0e}\n")

    results = {}
    all_trajectories = {}
    all_distances = {}

    for rho in RHO_VALUES:
        traj, esn = run_multiple_trajectories(rho, inputs, N_INIT, seed=SEED)
        dist = pairwise_distances(traj)
        conv_t = convergence_time(dist)
        esp_holds = dist[-1] < CONV_THRESH

        results[rho] = {
            "trajectories": traj,
            "distances": dist,
            "conv_time": conv_t,
            "esp_holds": esp_holds,
            "final_dist": dist[-1],
        }
        all_trajectories[rho] = traj
        all_distances[rho] = dist

    # ─── Print summary table ──────────────────────────────────────────────────
    print(f"{'ρ':>6}  {'ESP holds?':>12}  {'Conv. steps':>12}  "
          f"{'Final dist':>12}  {'Theoretical':>12}")
    print("-" * 62)
    for rho in RHO_VALUES:
        r = results[rho]
        esp_str  = "Yes" if r["esp_holds"] else "No"
        conv_str = f"{r['conv_time']}" if r["conv_time"] < T_INPUT else f">{T_INPUT}"
        if rho < 1.0:
            # Theoretical convergence time: approximately -log(CONV_THRESH)/log(1/rho)
            theory = -np.log(CONV_THRESH) / np.log(1.0 / rho) if rho > 0 else 0
            theo_str = f"{theory:.0f} steps"
        else:
            theo_str = "diverges"
        print(f"  {rho:4.2f}  {esp_str:>12}  {conv_str:>12}  "
              f"{r['final_dist']:>12.4e}  {theo_str:>12}")

    print(
        "\nNote: ρ < 1 is *sufficient* but not necessary for ESP.\n"
        "For ρ slightly above 1, the ESP may still hold for specific\n"
        "input distributions — this is why some practitioners use ρ > 1.\n"
        "For large ρ (here 1.5), trajectories clearly diverge."
    )

    # ─── Figure 1: Neuron-0 trajectories for each ρ ───────────────────────────
    n_cols = 3
    n_rows = (len(RHO_VALUES) + n_cols - 1) // n_cols
    fig, axes = plt.subplots(n_rows, n_cols, figsize=(14, 8))
    axes = axes.ravel()
    t_axis = np.arange(T_INPUT)
    palette = plt.cm.tab10(np.linspace(0, 1, N_INIT))

    for ax, rho in zip(axes[:len(RHO_VALUES)], RHO_VALUES):
        traj = results[rho]["trajectories"]   # (n_init, T, N)
        for trial in range(N_INIT):
            ax.plot(t_axis, traj[trial, :, 0],
                    color=palette[trial], lw=0.8, alpha=0.7)
        esp_str = "ESP holds" if results[rho]["esp_holds"] else "ESP violated"
        color_title = "seagreen" if results[rho]["esp_holds"] else "tomato"
        ax.set_title(f"ρ = {rho}  ({esp_str})",
                     fontsize=10, color=color_title)
        ax.set_xlabel("Time step", fontsize=9)
        ax.set_ylabel("x₀(t)", fontsize=9)
        ax.set_xlim(0, T_INPUT)

    # Hide unused subplots
    for ax in axes[len(RHO_VALUES):]:
        ax.set_visible(False)

    fig.suptitle(
        f"Neuron-0 Trajectories from {N_INIT} Different Initial Conditions\n"
        "ESP: all trajectories converge (green title) vs. diverge (red title)",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/04_esp_neuron_trajectories.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/04_esp_neuron_trajectories.png")

    # ─── Figure 2: Log-distance over time for all ρ ───────────────────────────
    fig, ax = plt.subplots(figsize=(9, 5))
    colors = plt.cm.coolwarm(np.linspace(0.05, 0.95, len(RHO_VALUES)))
    for rho, color in zip(RHO_VALUES, colors):
        dist = all_distances[rho]
        esp = results[rho]["esp_holds"]
        ls = "-" if esp else "--"
        label = f"ρ={rho} {'✓' if esp else '✗'}"
        ax.semilogy(t_axis, dist + 1e-15, color=color, lw=1.8, ls=ls, label=label)

    ax.axhline(CONV_THRESH, color="gray", ls=":", lw=1.5,
               label=f"Threshold {CONV_THRESH:.0e}")
    ax.set_xlabel("Time step", fontsize=11)
    ax.set_ylabel("Mean pairwise distance (log scale)", fontsize=11)
    ax.set_title(
        "Convergence of Reservoir Trajectories\n"
        "(Solid: ESP holds; Dashed: ESP violated)",
        fontsize=12
    )
    ax.legend(fontsize=9, ncol=2)
    ax.set_xlim(0, T_INPUT)
    plt.tight_layout()
    plt.savefig("output/04_esp_convergence.png", dpi=150)
    plt.close()
    print("Figure saved: output/04_esp_convergence.png")

    # ─── Figure 3: Convergence rate vs. ρ (log slope) ────────────────────────
    # Estimate the decay rate (slope of log dist vs t) from the first 100 steps
    # for ρ < 1 cases.
    fig, ax = plt.subplots(figsize=(7, 4.5))
    rho_sub = [r for r in RHO_VALUES if r < 1.0]
    measured_slopes = []
    for rho in rho_sub:
        dist = all_distances[rho]
        # Fit log-linear decay over first 50 steps where dist is still significant
        t_fit = np.arange(1, min(100, T_INPUT))
        log_d = np.log(dist[1:min(100, T_INPUT)] + 1e-15)
        valid = np.isfinite(log_d) & (log_d > np.log(CONV_THRESH))
        if valid.sum() > 5:
            slope, _ = np.polyfit(t_fit[valid], log_d[valid], 1)
        else:
            slope = 0.0
        measured_slopes.append(slope)

    ax.plot(rho_sub, measured_slopes, "o-", color="steelblue", lw=2, ms=8,
            label="Measured decay rate (slope of log dist)")
    ax.plot(rho_sub, [np.log(r) for r in rho_sub], "r--", lw=1.5,
            label="Theoretical log(ρ)")
    ax.set_xlabel("Spectral radius ρ", fontsize=11)
    ax.set_ylabel("Log-distance decay rate (per step)", fontsize=11)
    ax.set_title("Convergence Rate vs. ρ\n(More negative = faster convergence)", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/04_esp_convergence_rate.png", dpi=150)
    plt.close()
    print("Figure saved: output/04_esp_convergence_rate.png")

    # ─── Figure 4: Final-state distance across reservoir neurons ─────────────
    # Show which neurons have converged and which haven't, for each ρ
    fig, axes = plt.subplots(2, 3, figsize=(13, 7))
    axes = axes.ravel()
    for ax, rho in zip(axes, RHO_VALUES):
        traj = results[rho]["trajectories"]  # (n_init, T, N)
        # Distance between trajectory 0 and trajectory 1 at each neuron
        final_diff = traj[0, -1, :] - traj[1, -1, :]
        colors_neurons = ["seagreen" if abs(d) < CONV_THRESH else "tomato"
                          for d in final_diff]
        ax.bar(np.arange(N_RESERVOIR), np.abs(final_diff),
               color=colors_neurons, width=1.0)
        ax.axhline(CONV_THRESH, color="gray", ls="--", lw=1)
        esp_str = "ESP holds" if results[rho]["esp_holds"] else "ESP violated"
        ax.set_title(f"ρ = {rho}  ({esp_str})", fontsize=10)
        ax.set_xlabel("Neuron index", fontsize=9)
        ax.set_ylabel("|Δx_final|", fontsize=9)
        ax.set_ylim(0, max(np.abs(final_diff).max() * 1.2, CONV_THRESH * 2))

    fig.suptitle(
        "Final-State Distance Per Neuron\n"
        "(Between trajectories 0 and 1; green = converged, red = not)",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/04_esp_per_neuron.png", dpi=150)
    plt.close()
    print("Figure saved: output/04_esp_per_neuron.png")


if __name__ == "__main__":
    main()
