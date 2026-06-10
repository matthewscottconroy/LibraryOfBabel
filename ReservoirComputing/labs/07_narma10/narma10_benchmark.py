"""
================================================================================
Lab 07 — The NARMA-10 Benchmark: Systematic Comparison
================================================================================

Textbook connection
-------------------
Unit 4, Chapter 9: "Benchmarking Reservoir Computers."
NARMA-10 is the standard entry-level reservoir computing benchmark
(Jaeger 2002; Atiya & Parlos 2000). It is a 10th-order nonlinear
autoregressive task with a difficulty level that has been extensively
characterised in the literature. Running this benchmark reproducibly
is a key skill for any practitioner.

Reference value: Jaeger (2002) reports NMSE ≈ 0.004 for N=200.

Learning objectives
-------------------
  * Run the complete ESN benchmarking pipeline with multiple seeds to obtain
    statistically reliable results (mean ± std NMSE).
  * Understand how increasing N improves performance — but with diminishing
    returns.
  * See that the optimal ρ from Lab 05 transfers to different N values.
  * Appreciate the practical importance of the washout period — without it,
    the training targets are contaminated by initial-condition artefacts.
  * Compare your results to the published Jaeger (2002) reference value.

What to observe
---------------
  * NMSE decreases monotonically with N, roughly as 1/N (more capacity
    → better nonlinear approximation).
  * ρ = 0.9 outperforms both 0.5 and 0.99 for most N values on NARMA-10.
  * Washout = 0 produces noticeably worse results than washout = 100:
    the first 100 steps carry the "memory" of the arbitrary initial state.
  * By N = 200 you should approach Jaeger's NMSE ≈ 0.004.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, narma, nmse


# ── Configuration ────────────────────────────────────────────────────────────

N_VALUES      = [50, 100, 200, 500]
RHO_VALUES    = [0.5, 0.9, 0.99]
N_SEEDS       = 5               # random seeds for statistical stability
T_TRAIN       = 4000
T_TEST        = 2000
RIDGE_ALPHA   = 1e-6
WASHOUT       = 100
DATA_SEED     = 0               # fixed data seed — same data for all configs
ESN_SEEDS     = list(range(N_SEEDS))   # ESN structure seeds


# ── Utility ───────────────────────────────────────────────────────────────────

def run_single(n_reservoir: int,
               rho: float,
               u_train: np.ndarray,
               y_train: np.ndarray,
               u_test: np.ndarray,
               y_test: np.ndarray,
               washout: int = WASHOUT,
               seed: int = 42) -> float:
    """Train one ESN and return the test NMSE."""
    esn = EchoStateNetwork(
        n_reservoir=n_reservoir,
        spectral_radius=rho,
        input_scaling=1.0,
        connectivity=0.1,
        bias_scaling=0.1,
        ridge_alpha=RIDGE_ALPHA,
        washout=washout,
        extend_with_input=True,
        seed=seed,
    )
    try:
        esn.fit(u_train, y_train)
        y_pred = esn.predict(u_test)
        return nmse(y_test, y_pred)
    except Exception:
        return float("nan")


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # Generate fixed NARMA-10 dataset (same for all configurations)
    print("Generating NARMA-10 data...")
    T_total = T_TRAIN + T_TEST + 50
    u_all, y_all = narma(T=T_total, order=10, seed=DATA_SEED)
    u_train = u_all[:T_TRAIN].reshape(-1, 1)
    y_train = y_all[:T_TRAIN]
    u_test  = u_all[T_TRAIN:T_TRAIN + T_TEST].reshape(-1, 1)
    y_test  = y_all[T_TRAIN:T_TRAIN + T_TEST]

    # ─── Main grid: N × ρ, averaged over N_SEEDS ──────────────────────────────
    print(f"\nRunning {len(N_VALUES)} × {len(RHO_VALUES)} × {N_SEEDS} configurations...\n")

    # results[n][rho] = list of NMSE values (one per seed)
    results = {n: {rho: [] for rho in RHO_VALUES} for n in N_VALUES}

    header = (f"{'Config':>22}  {'Seed':>5}  {'NMSE':>10}")
    print(header)
    print("-" * 44)

    for n in N_VALUES:
        for rho in RHO_VALUES:
            for seed in ESN_SEEDS:
                val = run_single(n, rho, u_train, y_train, u_test, y_test,
                                 washout=WASHOUT, seed=seed)
                results[n][rho].append(val)
                print(f"  N={n:>3}, ρ={rho:.2f}             "
                      f"  {seed:>5}  {val:>10.6f}")

    # ─── Print summary table ──────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("NARMA-10 Benchmark Results")
    print("=" * 65)
    print(f"  T_train={T_TRAIN}, T_test={T_TEST}, washout={WASHOUT}, seeds={N_SEEDS}")
    print(f"  Jaeger (2002) reference: NMSE ≈ 0.004  for N=200\n")
    print(f"  {'Config':>22}  {'Mean NMSE':>10}  {'Std NMSE':>10}  {'Min NMSE':>10}")
    print("  " + "-" * 56)

    # Find overall best config
    best_nmse = float("inf")
    best_config = None

    for n in N_VALUES:
        for rho in RHO_VALUES:
            vals = [v for v in results[n][rho] if not np.isnan(v)]
            if not vals:
                continue
            mean_v = np.mean(vals)
            std_v  = np.std(vals)
            min_v  = np.min(vals)
            tag = " ← best" if mean_v < best_nmse else ""
            if mean_v < best_nmse:
                best_nmse = mean_v
                best_config = (n, rho)
            label = f"N={n:>3}, ρ={rho:.2f}"
            print(f"  {label:>22}  {mean_v:>10.6f}  {std_v:>10.6f}  {min_v:>10.6f}{tag}")

    print(f"\n  Best config: N={best_config[0]}, ρ={best_config[1]:.2f}  "
          f"(mean NMSE={best_nmse:.6f})")

    # ─── Ablation: washout effect ─────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Ablation: Effect of Washout Length")
    print("=" * 65)
    washout_values = [0, 10, 50, 100, 200]
    washout_nmse   = {}
    n_abl, rho_abl = 100, 0.9
    print(f"  Config: N={n_abl}, ρ={rho_abl}, averaged over {N_SEEDS} seeds\n")
    print(f"  {'Washout':>10}  {'Mean NMSE':>12}  {'Std':>10}")
    print("  " + "-" * 36)

    for w in washout_values:
        vals = [run_single(n_abl, rho_abl, u_train, y_train, u_test, y_test,
                           washout=w, seed=s) for s in ESN_SEEDS]
        vals = [v for v in vals if not np.isnan(v)]
        washout_nmse[w] = (np.mean(vals), np.std(vals))
        print(f"  {w:>10}  {washout_nmse[w][0]:>12.6f}  {washout_nmse[w][1]:>10.6f}")

    # ─── Figure 1: NMSE vs. N for each ρ (with error bars) ───────────────────
    fig, ax = plt.subplots(figsize=(8, 5))
    colors_rho = {"0.5": "steelblue", "0.9": "seagreen", "0.99": "darkorange"}
    markers    = {"0.5": "o",         "0.9": "s",         "0.99": "^"}

    for rho in RHO_VALUES:
        means = [np.nanmean(results[n][rho]) for n in N_VALUES]
        stds  = [np.nanstd(results[n][rho])  for n in N_VALUES]
        key   = str(rho)
        ax.errorbar(N_VALUES, means, yerr=stds,
                    color=colors_rho.get(key, "gray"),
                    marker=markers.get(key, "o"),
                    lw=2, ms=7, capsize=4,
                    label=f"ρ = {rho}")

    # Reference value
    ax.axhline(0.004, color="tomato", ls="--", lw=1.5,
               label="Jaeger (2002): NMSE ≈ 0.004 (N=200)")
    ax.scatter([200], [0.004], color="tomato", s=80, zorder=5)

    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_xticks(N_VALUES); ax.get_xaxis().set_major_formatter(
        plt.FuncFormatter(lambda v, _: str(int(v))))
    ax.set_xlabel("Reservoir size N", fontsize=11)
    ax.set_ylabel("Test NMSE (log scale)", fontsize=11)
    ax.set_title(
        f"NARMA-10 NMSE vs. Reservoir Size\n"
        f"({N_SEEDS} seeds, error bars = ±1 std)",
        fontsize=12
    )
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/07_nmse_vs_N.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/07_nmse_vs_N.png")

    # ─── Figure 2: Results heatmap (N × ρ) ────────────────────────────────────
    mean_grid = np.array([[np.nanmean(results[n][rho]) for rho in RHO_VALUES]
                           for n in N_VALUES])
    fig, ax = plt.subplots(figsize=(7, 5))
    im = ax.imshow(np.log10(mean_grid + 1e-10), aspect="auto", cmap="RdYlGn_r",
                   vmin=-3, vmax=0)
    plt.colorbar(im, ax=ax, label="log₁₀(Mean NMSE)")
    ax.set_xticks(range(len(RHO_VALUES)))
    ax.set_xticklabels([str(r) for r in RHO_VALUES])
    ax.set_yticks(range(len(N_VALUES)))
    ax.set_yticklabels([str(n) for n in N_VALUES])
    ax.set_xlabel("Spectral radius ρ", fontsize=11)
    ax.set_ylabel("Reservoir size N", fontsize=11)
    ax.set_title("NARMA-10 Mean Test NMSE Heatmap (log₁₀ scale)", fontsize=11)
    # Annotate cells
    for i in range(len(N_VALUES)):
        for j in range(len(RHO_VALUES)):
            ax.text(j, i, f"{mean_grid[i,j]:.4f}", ha="center", va="center",
                    fontsize=8, color="black")
    plt.tight_layout()
    plt.savefig("output/07_nmse_heatmap.png", dpi=150)
    plt.close()
    print("Figure saved: output/07_nmse_heatmap.png")

    # ─── Figure 3: Washout ablation ───────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(7, 4))
    w_arr = np.array(washout_values)
    m_arr = np.array([washout_nmse[w][0] for w in washout_values])
    s_arr = np.array([washout_nmse[w][1] for w in washout_values])
    ax.errorbar(w_arr, m_arr, yerr=s_arr, fmt="o-", color="steelblue",
                lw=2, ms=7, capsize=5)
    ax.set_xlabel("Washout length (steps)", fontsize=11)
    ax.set_ylabel("Mean test NMSE", fontsize=11)
    ax.set_title(
        f"Effect of Washout Length on NMSE\n"
        f"(N={n_abl}, ρ={rho_abl}, {N_SEEDS} seeds)",
        fontsize=11
    )
    ax.set_yscale("log")
    plt.tight_layout()
    plt.savefig("output/07_washout_ablation.png", dpi=150)
    plt.close()
    print("Figure saved: output/07_washout_ablation.png")

    # ─── Figure 4: Best prediction trace ──────────────────────────────────────
    n_best, rho_best = best_config
    esn_best = EchoStateNetwork(
        n_reservoir=n_best,
        spectral_radius=rho_best,
        input_scaling=1.0,
        connectivity=0.1,
        bias_scaling=0.1,
        ridge_alpha=RIDGE_ALPHA,
        washout=WASHOUT,
        extend_with_input=True,
        seed=ESN_SEEDS[0],
    )
    esn_best.fit(u_train, y_train)
    y_best_pred = esn_best.predict(u_test)
    best_actual_nmse = nmse(y_test, y_best_pred)

    fig, ax = plt.subplots(figsize=(11, 4))
    t_seg = min(400, T_TEST)
    t_axis = np.arange(t_seg)
    ax.plot(t_axis, y_test[:t_seg], color="steelblue", lw=1.5, label="True NARMA-10")
    ax.plot(t_axis, y_best_pred[:t_seg], color="tomato", lw=1.2, ls="--",
            label=f"Best ESN (N={n_best}, ρ={rho_best:.2f}, NMSE={best_actual_nmse:.5f})")
    ax.set_xlabel("Time step (test set)", fontsize=11)
    ax.set_ylabel("y(t)", fontsize=11)
    ax.set_title(f"NARMA-10 Best Prediction  (Jaeger ref: NMSE ≈ 0.004 for N=200)",
                 fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/07_best_prediction.png", dpi=150)
    plt.close()
    print("Figure saved: output/07_best_prediction.png")


if __name__ == "__main__":
    main()
