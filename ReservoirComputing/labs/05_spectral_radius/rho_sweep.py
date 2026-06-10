"""
================================================================================
Lab 05 — The ρ Sweep: Memory-Performance Tradeoff
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 7: "Hyperparameter Tuning and the Spectral Radius."
The spectral radius ρ is the single most important ESN hyperparameter. This
lab systematically sweeps ρ to reveal the canonical U-shaped NMSE curve and
the monotonically increasing memory capacity curve, making the fundamental
memory-performance tradeoff concrete and measurable.

References: Jaeger (2002), Lukoševičius (2012), Verstraeten et al. (2010,
"An experimental unification of reservoir computing methods").

Learning objectives
-------------------
  * Observe the U-shaped NMSE vs. ρ curve: performance degrades for both
    very small ρ (insufficient memory) and large ρ (approaching ESP boundary).
  * See that linear memory capacity (MC) increases monotonically with ρ.
  * Understand the fundamental tension: tasks requiring long memory need high ρ,
    but stability and generalisation favour ρ < 1.
  * Identify the ESP boundary at ρ = 1 and see its practical effect.
  * Recognise that the optimal ρ depends on the task's intrinsic memory
    requirement — for NARMA-10, roughly 10 steps → ρ ≈ 0.9.

What to observe
---------------
  * NMSE is high for ρ < 0.5 (too little memory to capture the 10-step NARMA
    dependency) and again high for ρ > 1.0 (loss of ESP, diverging dynamics).
  * The optimal ρ is near 0.8–0.95 for NARMA-10 on this ESN size.
  * Memory capacity increases with ρ but saturates near N at ρ → 1.
  * The vertical line at ρ = 1 marks the theoretical ESP boundary — notice
    that empirically the degradation begins slightly before ρ = 1.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, narma, nmse, memory_capacity


# ── Configuration ────────────────────────────────────────────────────────────

N_RESERVOIR  = 100
RHO_VALUES   = np.linspace(0.1, 1.3, 25)
T_TRAIN      = 3000
T_TEST       = 1000
MC_T_TRAIN   = 2000      # shorter train for MC to keep runtime reasonable
MC_T_TEST    = 500
SEED         = 42


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ─── Generate NARMA-10 data ───────────────────────────────────────────────
    print("Generating NARMA-10 data...")
    T_total = T_TRAIN + T_TEST + 50
    u_all, y_all = narma(T=T_total, order=10, seed=SEED)
    u_train = u_all[:T_TRAIN].reshape(-1, 1)
    y_train = y_all[:T_TRAIN]
    u_test  = u_all[T_TRAIN:T_TRAIN + T_TEST].reshape(-1, 1)
    y_test  = y_all[T_TRAIN:T_TRAIN + T_TEST]

    # ─── Sweep ρ ──────────────────────────────────────────────────────────────
    print(f"\nSweeping ρ over {len(RHO_VALUES)} values...")
    print(f"  N={N_RESERVOIR}, T_train={T_TRAIN}, T_test={T_TEST}")
    print()

    nmse_values = np.zeros(len(RHO_VALUES))
    mc_values   = np.zeros(len(RHO_VALUES))

    for i, rho in enumerate(RHO_VALUES):
        # ── NMSE ──
        esn = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=rho,
            input_scaling=1.0,
            connectivity=0.1,
            bias_scaling=0.1,
            ridge_alpha=1e-6,
            washout=100,
            extend_with_input=True,
            seed=SEED,
        )
        try:
            esn.fit(u_train, y_train)
            y_pred = esn.predict(u_test)
            nmse_val = nmse(y_test, y_pred)
            # Cap NMSE at 10 to prevent runaway values from dominating the plot
            nmse_values[i] = min(nmse_val, 10.0)
        except Exception:
            nmse_values[i] = 10.0

        # ── Memory Capacity ──
        # Use a fresh ESN (same weights, unfitted) for the MC measurement
        esn_mc = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=rho,
            input_scaling=1.0,
            connectivity=0.1,
            bias_scaling=0.1,
            ridge_alpha=1e-8,
            washout=min(100, MC_T_TRAIN // 10),
            extend_with_input=False,
            seed=SEED,
        )
        try:
            total_mc, _ = memory_capacity(
                esn_mc,
                T_train=MC_T_TRAIN,
                T_test=MC_T_TEST,
                max_delay=N_RESERVOIR,   # bounded at N for speed
                seed=SEED,
            )
            mc_values[i] = total_mc
        except Exception:
            mc_values[i] = 0.0

        status = "ok" if nmse_values[i] < 10 else "diverged"
        print(f"  ρ={rho:.3f}  NMSE={nmse_values[i]:.5f}  MC={mc_values[i]:.2f}  [{status}]")

    # ─── Identify optimal ρ ───────────────────────────────────────────────────
    # Only consider ρ < 1.05 for the optimum (ESP region)
    valid_mask = RHO_VALUES < 1.05
    valid_nmse = nmse_values.copy()
    valid_nmse[~valid_mask] = np.inf
    opt_idx  = int(np.argmin(valid_nmse))
    opt_rho  = RHO_VALUES[opt_idx]
    opt_nmse = nmse_values[opt_idx]
    opt_mc   = mc_values[opt_idx]

    print(f"\n{'='*55}")
    print("Sweep Results")
    print(f"{'='*55}")
    print(f"  Optimal ρ:          {opt_rho:.3f}")
    print(f"  Optimal NMSE:       {opt_nmse:.6f}")
    print(f"  MC at optimal ρ:    {opt_mc:.2f}")
    print(f"  Theoretical MC ≤ N = {N_RESERVOIR}")
    print(f"  MC bound respected: {'Yes' if max(mc_values) <= N_RESERVOIR + 2 else 'Check'}")

    # ─── Figure: Two-panel NMSE and MC vs. ρ ──────────────────────────────────
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(9, 8), sharex=True)

    # Panel 1: NMSE vs. ρ
    ax1.semilogy(RHO_VALUES, nmse_values, "o-", color="steelblue",
                 lw=2, ms=5, label="Test NMSE")
    ax1.axvline(1.0, color="tomato", ls="--", lw=1.5, label="ESP boundary (ρ=1)")
    ax1.axvline(opt_rho, color="seagreen", ls=":", lw=2,
                label=f"Optimal ρ = {opt_rho:.2f}  (NMSE={opt_nmse:.4f})")
    ax1.scatter([opt_rho], [opt_nmse], color="seagreen", s=80, zorder=5)
    ax1.set_ylabel("Test NMSE (log scale)", fontsize=11)
    ax1.set_title(
        f"NARMA-10: NMSE vs. Spectral Radius\n"
        f"N={N_RESERVOIR}, T_train={T_TRAIN}, T_test={T_TEST}",
        fontsize=12
    )
    ax1.legend(fontsize=9)
    ax1.set_ylim(bottom=1e-5)
    # Add annotation for the two failure modes
    ax1.annotate(
        "Too little\nmemory",
        xy=(0.2, nmse_values[2]), xytext=(0.25, nmse_values[2] * 3),
        fontsize=8, color="gray",
        arrowprops=dict(arrowstyle="->", color="gray"),
    )
    ax1.annotate(
        "ESP\nviolated",
        xy=(1.2, nmse_values[-3]), xytext=(1.05, nmse_values[-3] * 2),
        fontsize=8, color="tomato",
        arrowprops=dict(arrowstyle="->", color="tomato"),
    )

    # Panel 2: Memory Capacity vs. ρ
    ax2.plot(RHO_VALUES, mc_values, "s-", color="darkorange",
             lw=2, ms=5, label="Linear MC")
    ax2.axvline(1.0, color="tomato", ls="--", lw=1.5, label="ESP boundary (ρ=1)")
    ax2.axvline(opt_rho, color="seagreen", ls=":", lw=2,
                label=f"Optimal ρ = {opt_rho:.2f}")
    ax2.axhline(N_RESERVOIR, color="gray", ls=":", lw=1,
                label=f"Theoretical bound MC = N = {N_RESERVOIR}")
    ax2.scatter([opt_rho], [opt_mc], color="seagreen", s=80, zorder=5)
    ax2.set_xlabel("Spectral radius ρ", fontsize=11)
    ax2.set_ylabel("Linear Memory Capacity", fontsize=11)
    ax2.set_title("Linear Memory Capacity vs. Spectral Radius", fontsize=12)
    ax2.legend(fontsize=9)
    ax2.set_ylim(0, N_RESERVOIR * 1.1)

    plt.tight_layout()
    plt.savefig("output/05_rho_sweep.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/05_rho_sweep.png")

    # ─── Figure 2: Raw predictions for three ρ values ────────────────────────
    rho_examples = [0.2, opt_rho, 1.2]
    fig, axes = plt.subplots(3, 1, figsize=(11, 9), sharex=True)
    t_seg = min(300, T_TEST)
    t_axis = np.arange(t_seg)

    for ax, rho_ex in zip(axes, rho_examples):
        esn_ex = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=rho_ex,
            input_scaling=1.0,
            connectivity=0.1,
            bias_scaling=0.1,
            ridge_alpha=1e-6,
            washout=100,
            extend_with_input=True,
            seed=SEED,
        )
        try:
            esn_ex.fit(u_train, y_train)
            y_p = esn_ex.predict(u_test)
            score = nmse(y_test, y_p)
            label_pred = f"Prediction (NMSE={score:.4f})"
        except Exception:
            y_p = np.zeros(T_TEST)
            label_pred = "Diverged"

        ax.plot(t_axis, y_test[:t_seg], color="steelblue", lw=1.5,
                label="True NARMA-10")
        ax.plot(t_axis, y_p[:t_seg], color="tomato", lw=1.2, ls="--",
                label=label_pred)
        tag = "optimal" if abs(rho_ex - opt_rho) < 0.05 else ("too low" if rho_ex < 0.5 else "too high")
        ax.set_title(f"ρ = {rho_ex}  ({tag})", fontsize=11)
        ax.set_ylabel("y(t)", fontsize=10)
        ax.legend(fontsize=9)

    axes[-1].set_xlabel("Time step (test set)", fontsize=11)
    fig.suptitle("NARMA-10 Predictions: Low, Optimal, and High ρ", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/05_rho_example_predictions.png", dpi=150)
    plt.close()
    print("Figure saved: output/05_rho_example_predictions.png")

    # ─── Figure 3: Combined scatter — NMSE vs. MC coloured by ρ ─────────────
    fig, ax = plt.subplots(figsize=(7, 5))
    sc = ax.scatter(mc_values, nmse_values, c=RHO_VALUES, cmap="viridis",
                    s=60, zorder=3)
    plt.colorbar(sc, ax=ax, label="Spectral radius ρ")
    ax.set_xlabel("Linear Memory Capacity", fontsize=11)
    ax.set_ylabel("Test NMSE", fontsize=11)
    ax.set_yscale("log")
    ax.scatter([opt_mc], [opt_nmse], color="red", s=120, marker="*",
               zorder=5, label=f"Optimal (ρ={opt_rho:.2f})")
    ax.set_title("Memory-Performance Tradeoff Scatter\n"
                 "Each point = one ρ value", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/05_mc_vs_nmse_scatter.png", dpi=150)
    plt.close()
    print("Figure saved: output/05_mc_vs_nmse_scatter.png")


if __name__ == "__main__":
    main()
