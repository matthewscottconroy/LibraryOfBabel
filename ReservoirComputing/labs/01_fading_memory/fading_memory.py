"""
================================================================================
Lab 01 — Fading Memory in Echo State Networks
================================================================================

Textbook connection
-------------------
Unit 2, Chapter 3: "The Fading Memory Property."
Follows the formal treatment in Jaeger (2001, "The Echo State Approach to
Analysing and Training Recurrent Neural Networks") and Boyd & Chua (1985,
"Fading Memory and the Problem of Approximating Nonlinear Operators with
Volterra Series").

Learning objectives
-------------------
  * Understand what fading memory means: the network's current state depends
    more strongly on recent inputs than on distant past inputs.
  * Visualise how the influence of past inputs decays as a function of lag k
    for different spectral radii ρ.
  * Quantify "effective memory time" as the lag at which cross-correlation
    drops below 1/e of its peak.
  * See the core ρ → memory tradeoff: larger ρ → longer memory, but closer
    to instability.
  * Demonstrate the formal fading memory property: two sequences that agree
    after time k but differ before produce states that are nearly identical,
    with the difference vanishing as k → ∞.

What to observe
---------------
  * The mean absolute cross-correlation between reservoir states x_i(t) and
    the input u(t-k) decays with lag k. For small ρ the decay is rapid
    (short memory); for ρ → 1 it is slow (long memory).
  * The 1/e effective memory time grows roughly as 1/(1-ρ).
  * When two input sequences are joined k steps in the past, the resulting
    reservoir states converge as k → ∞, and converge faster for smaller ρ.
    For ρ ≥ 1 this convergence may fail.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

# Add the labs/ directory to the path so we can import utils
sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork


# ── Configuration ────────────────────────────────────────────────────────────

RNG_SEED      = 42
T             = 1000          # input sequence length
N_RESERVOIR   = 100           # reservoir size
MAX_LAG       = 100           # maximum correlation lag to probe
RHO_VALUES    = [0.1, 0.5, 0.9, 0.99]   # spectral radii to compare
RHO_FM_TEST   = [0.5, 0.9, 0.99, 1.1]  # for formal fading-memory demonstration
N_NEURONS_CC  = 20            # how many neurons to average over
K_DIVERGE     = 50            # the merge point for the FM demonstration


# ── Helper functions ──────────────────────────────────────────────────────────

def run_reservoir(rho: float, inputs: np.ndarray, seed: int = 0) -> np.ndarray:
    """
    Build an ESN with the given spectral radius and drive it with `inputs`.
    Returns the (T, N) state matrix.
    """
    esn = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=rho,
        input_scaling=0.5,
        connectivity=0.1,
        bias_scaling=0.0,   # no bias — clean signal for correlation analysis
        washout=0,
        extend_with_input=False,
        seed=seed,
    )
    # Build the reservoir (no training needed — we just observe states)
    esn._build_reservoir(1)
    states, _ = esn.run(inputs.reshape(-1, 1))
    return states   # (T, N)


def mean_abs_cross_correlation(states: np.ndarray,
                                inputs: np.ndarray,
                                max_lag: int,
                                n_neurons: int) -> np.ndarray:
    """
    For each of `n_neurons` neurons, compute the cross-correlation between
    state x_i(t) and input u(t-k) for k = 0, 1, ..., max_lag.
    Return the mean absolute correlation across neurons.

    Cross-correlation at lag k:
        C_k = Cov(x_i(t), u(t-k)) / (std(x_i) * std(u))

    This measures how much the state at time t "remembers" the input from
    k steps ago.
    """
    T_len = states.shape[0]
    std_u = np.std(inputs)
    corr_matrix = np.zeros((n_neurons, max_lag + 1))

    for i in range(n_neurons):
        xi = states[:, i]
        std_xi = np.std(xi)
        if std_xi < 1e-12:
            continue
        for k in range(max_lag + 1):
            # Overlap region: t = k .. T-1
            if k == 0:
                x_seg = xi
                u_seg = inputs
            else:
                x_seg = xi[k:]
                u_seg = inputs[:-k]
            # Normalised cross-correlation (Pearson)
            cov = np.mean((x_seg - np.mean(x_seg)) * (u_seg - np.mean(u_seg)))
            corr_matrix[i, k] = abs(cov) / (std_xi * std_u + 1e-15)

    return corr_matrix.mean(axis=0)   # (max_lag + 1,)


def effective_memory_time(corr: np.ndarray) -> int:
    """
    Return the lag at which `corr` first drops to 1/e of its peak value.
    If it never drops that far, return max_lag.
    """
    threshold = corr.max() / np.e
    indices = np.where(corr < threshold)[0]
    if len(indices) == 0:
        return len(corr) - 1
    return int(indices[0])


def fading_memory_demonstration(rho: float,
                                 base_input: np.ndarray,
                                 merge_at_k: int,
                                 seed: int = 0) -> np.ndarray:
    """
    Formal fading-memory demonstration.

    Create two input sequences:
      - Sequence A: `base_input` unchanged.
      - Sequence B: identical to A from time (T - merge_at_k) onward,
        but randomised before that.

    Drive an ESN (starting from the zero state) with both sequences.
    Return the trajectory of ||x_A(t) - x_B(t)|| over time.

    If the ESN has fading memory, this distance should shrink as merge_at_k
    increases (or equivalently, as we run for more time after the merge point).
    """
    T_len = len(base_input)
    merge_point = T_len - merge_at_k  # index where B becomes identical to A

    rng = np.random.default_rng(seed + 1)
    input_b = rng.uniform(-0.5, 0.5, T_len)
    # From merge_point onward, B matches A exactly
    input_b[merge_point:] = base_input[merge_point:]

    states_a = run_reservoir(rho, base_input, seed=seed)
    states_b = run_reservoir(rho, input_b,    seed=seed)

    distances = np.linalg.norm(states_a - states_b, axis=1)   # (T,)
    return distances


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    rng = np.random.default_rng(RNG_SEED)
    # White noise input in [-0.5, 0.5]
    inputs = rng.uniform(-0.5, 0.5, T)

    # ─── Part 1: Cross-correlation analysis ──────────────────────────────────
    print("=" * 60)
    print("Part 1: Cross-Correlation vs. Lag for Different ρ Values")
    print("=" * 60)

    lags = np.arange(MAX_LAG + 1)
    corr_per_rho = {}
    eff_mem_times = {}

    for rho in RHO_VALUES:
        states = run_reservoir(rho, inputs)
        corr = mean_abs_cross_correlation(states, inputs, MAX_LAG, N_NEURONS_CC)
        corr_per_rho[rho] = corr
        eff_mem_times[rho] = effective_memory_time(corr)

    # Print effective memory times
    print(f"\n{'ρ':>6}  {'Eff. memory time (1/e lag)':>28}  "
          f"{'Theoretical ~1/(1-ρ)':>22}")
    print("-" * 62)
    for rho in RHO_VALUES:
        if rho < 1.0:
            theoretical = 1.0 / (1.0 - rho)
        else:
            theoretical = float("inf")
        print(f"  {rho:4.2f}  {eff_mem_times[rho]:>28d}  {theoretical:>22.1f}")

    print("\nObservation: effective memory time grows rapidly as ρ → 1,")
    print("consistent with the 1/(1-ρ) scaling argument for linear reservoirs.")

    # Figure 1: Mean absolute correlation vs. lag
    fig, axes = plt.subplots(2, 2, figsize=(11, 8), sharey=False)
    colors = plt.cm.viridis(np.linspace(0.15, 0.85, len(RHO_VALUES)))

    for ax, rho, color in zip(axes.ravel(), RHO_VALUES, colors):
        corr = corr_per_rho[rho]
        ax.plot(lags, corr, color=color, lw=1.8)
        # Mark the 1/e threshold
        threshold = corr.max() / np.e
        ax.axhline(threshold, color="gray", ls="--", lw=1, label="1/e threshold")
        emt = eff_mem_times[rho]
        ax.axvline(emt, color="tomato", ls=":", lw=1.5,
                   label=f"Eff. mem. = {emt} steps")
        ax.set_title(f"ρ = {rho}", fontsize=12)
        ax.set_xlabel("Lag k", fontsize=10)
        ax.set_ylabel(r"Mean $|C_k|$", fontsize=10)
        ax.legend(fontsize=8)
        ax.set_xlim(0, MAX_LAG)
        ax.set_ylim(0, None)

    fig.suptitle(
        "Fading Memory: Mean Absolute Cross-Correlation\n"
        r"between Reservoir State $x_i(t)$ and Input $u(t-k)$",
        fontsize=13
    )
    plt.tight_layout()
    plt.savefig("output/01_cross_correlation_vs_lag.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/01_cross_correlation_vs_lag.png")

    # Figure 2: All ρ on one axes (summary)
    fig, ax = plt.subplots(figsize=(8, 4.5))
    for rho, color in zip(RHO_VALUES, colors):
        ax.semilogy(lags, corr_per_rho[rho] + 1e-8, color=color,
                    lw=2, label=f"ρ = {rho}")
    ax.set_xlabel("Lag k", fontsize=11)
    ax.set_ylabel(r"Mean $|C_k|$ (log scale)", fontsize=11)
    ax.set_title("Fading Memory Profile (Log Scale): All ρ Values", fontsize=12)
    ax.legend(fontsize=10)
    ax.set_xlim(0, MAX_LAG)
    plt.tight_layout()
    plt.savefig("output/01_fading_memory_summary.png", dpi=150)
    plt.close()
    print("Figure saved: output/01_fading_memory_summary.png")

    # Figure 3: Effective memory time vs. ρ
    rho_dense = [r for r in RHO_VALUES]
    emt_values = [eff_mem_times[r] for r in RHO_VALUES]

    fig, ax = plt.subplots(figsize=(6, 4))
    ax.plot(rho_dense, emt_values, "o-", color="steelblue", lw=2, ms=8,
            label="Measured eff. memory time")
    # Overlay theoretical 1/(1-ρ) curve (for values < 1)
    rho_theory = np.linspace(0.05, 0.98, 200)
    ax.plot(rho_theory, 1.0 / (1.0 - rho_theory), "--", color="tomato",
            lw=1.5, label=r"Theoretical $1/(1-\rho)$")
    ax.set_xlabel("Spectral radius ρ", fontsize=11)
    ax.set_ylabel("Effective memory time (steps)", fontsize=11)
    ax.set_title("Memory Time vs. Spectral Radius", fontsize=12)
    ax.legend(fontsize=10)
    ax.set_xlim(0, 1.05)
    plt.tight_layout()
    plt.savefig("output/01_memory_time_vs_rho.png", dpi=150)
    plt.close()
    print("Figure saved: output/01_memory_time_vs_rho.png")

    # ─── Part 2: Formal fading-memory property ───────────────────────────────
    print("\n" + "=" * 60)
    print("Part 2: Formal Fading-Memory Property Demonstration")
    print("=" * 60)
    print(
        "\nSetup: two input sequences that agree from time (T-k) onward,\n"
        "but differ before. If ESP holds, the state difference → 0."
    )

    # For a fixed merge point k, measure distance over time for each ρ
    k_values = [10, 30, 50, 80]  # merge-at-k values to compare

    fig, axes = plt.subplots(2, 2, figsize=(11, 8))
    t_axis = np.arange(T)

    for ax, rho in zip(axes.ravel(), RHO_FM_TEST):
        for k in k_values:
            dist = fading_memory_demonstration(rho, inputs, merge_at_k=k, seed=RNG_SEED)
            merge_t = T - k
            label = f"k={k} (merge@t={merge_t})"
            ax.semilogy(t_axis, dist + 1e-12, lw=1.5, label=label)

        ax.axvline(T - k_values[-1], color="gray", ls=":", lw=1)
        ax.set_title(f"ρ = {rho}", fontsize=12)
        ax.set_xlabel("Time step t", fontsize=10)
        ax.set_ylabel(r"$\|\mathbf{x}_A(t) - \mathbf{x}_B(t)\|$", fontsize=10)
        ax.legend(fontsize=7, ncol=2)
        ax.set_xlim(0, T)
        ax.set_ylim(bottom=1e-14)

    fig.suptitle(
        "Formal Fading Memory: State Distance Between\n"
        "Two Sequences That Merge at Different Times",
        fontsize=13
    )
    plt.tight_layout()
    plt.savefig("output/01_formal_fading_memory.png", dpi=150)
    plt.close()
    print("Figure saved: output/01_formal_fading_memory.png")

    # Final state distances as a function of merge-at-k
    print(f"\n{'ρ':>6}  {'k=10':>10}  {'k=30':>10}  {'k=50':>10}  {'k=80':>10}")
    print("-" * 52)
    for rho in RHO_FM_TEST:
        row_vals = []
        for k in [10, 30, 50, 80]:
            dist = fading_memory_demonstration(rho, inputs, merge_at_k=k,
                                                seed=RNG_SEED)
            # Distance at the final time step
            row_vals.append(dist[-1])
        sign = "✓ ESP" if rho < 1.0 else "✗ ESP"
        print(
            f"  {rho:4.2f}  "
            + "  ".join(f"{v:10.2e}" for v in row_vals)
            + f"  [{sign}]"
        )

    print(
        "\nConclusion:\n"
        "  For ρ < 1: final-state distances are tiny — the ESN has fading\n"
        "  memory (the distant past is effectively forgotten).\n"
        "  For ρ > 1: the distances may remain large or grow — the ESP can\n"
        "  be violated and the network may not forget its initial conditions."
    )


if __name__ == "__main__":
    main()
