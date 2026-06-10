"""
================================================================================
Lab 03 — Your First Echo State Network: NARMA-10
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 5: "Echo State Networks."
This lab implements the complete ESN training pipeline from Jaeger (2001,
"The Echo State Approach to Analysing and Training Recurrent Neural Networks").
NARMA-10 (Atiya & Parlos 2000, Jaeger 2002) is the standard entry-level RC
benchmark: a 10th-order nonlinear autoregressive task that requires exactly
10 steps of memory and a mild nonlinear interaction.

Learning objectives
-------------------
  * Understand the full ESN pipeline: data preparation → reservoir drive →
    washout → ridge regression → prediction.
  * Visualise the reservoir state matrix and appreciate that the N neurons
    provide a rich, high-dimensional representation of the input history.
  * See the effect of regularisation: ridge_alpha=0 leads to overfitting
    (test NMSE degrades), while a well-tuned alpha controls generalisation.
  * Develop an intuition for what "good" ESN performance looks like:
    the predicted trace should closely overlay the true trace.
  * Understand the role of the washout period in removing initial-condition
    artefacts from the training data.

What to observe
---------------
  * With N=100 and ρ=0.9 the ESN achieves NMSE well below 0.01 on NARMA-10.
  * The reservoir state heatmap shows rich, diverse activation patterns —
    no two neurons respond identically to the same input.
  * Zero regularisation (ridge_alpha=0) typically gives lower *training* error
    but higher *test* error, a textbook example of overfitting.
  * The error time series tends to spike near rapid transitions in the target,
    where the input history required for accurate prediction is longest.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from matplotlib.gridspec import GridSpec

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, narma, nmse


# ── Configuration ────────────────────────────────────────────────────────────

T_TOTAL    = 6000
T_TRAIN    = 4000
T_TEST     = 2000
N_RESERVOIR = 100
RHO        = 0.9
RIDGE_ALPHA = 1e-6
SEED       = 42

# Heatmap display parameters
N_NEURONS_SHOW = 50    # first N neurons in heatmap
T_HEATMAP      = 200   # first T timesteps in heatmap


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ─── 1. Generate NARMA-10 data ────────────────────────────────────────────
    # NARMA-10: y_{t+1} = 0.3 y_t + 0.05 y_t sum_{i=0}^{9} y_{t-i}
    #                      + 1.5 u_{t-9} u_t + 0.1
    # The target depends on 10 past steps — memory depth exactly 10.
    print("Generating NARMA-10 data...")
    u_all, y_all = narma(T=T_TOTAL, order=10, seed=SEED)

    u_train = u_all[:T_TRAIN].reshape(-1, 1)
    y_train = y_all[:T_TRAIN]
    u_test  = u_all[T_TRAIN:T_TRAIN + T_TEST].reshape(-1, 1)
    y_test  = y_all[T_TRAIN:T_TRAIN + T_TEST]

    print(f"  Training set:  {T_TRAIN} steps")
    print(f"  Test set:      {T_TEST} steps")
    print(f"  Input range:   [{u_all.min():.3f}, {u_all.max():.3f}]")
    print(f"  Target range:  [{y_all.min():.3f}, {y_all.max():.3f}]")
    print(f"  Target mean:   {y_all.mean():.4f}  std: {y_all.std():.4f}")

    # ─── 2. Build and train the ESN ───────────────────────────────────────────
    print(f"\nBuilding ESN: N={N_RESERVOIR}, ρ={RHO}, λ={RIDGE_ALPHA}...")
    esn = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=RHO,
        input_scaling=1.0,
        connectivity=0.1,
        bias_scaling=0.1,
        ridge_alpha=RIDGE_ALPHA,
        washout=100,          # discard first 100 steps of training
        extend_with_input=True,
        seed=SEED,
    )
    esn.fit(u_train, y_train)
    y_pred = esn.predict(u_test)

    test_nmse = nmse(y_test, y_pred)
    print(f"  Test NMSE:  {test_nmse:.6f}")
    print(f"  Test NRMSE: {np.sqrt(test_nmse):.6f}")

    # ─── 3. Ablation: ridge_alpha = 0 ────────────────────────────────────────
    # With zero regularisation, the ridge regression becomes ordinary least
    # squares.  The system may overfit the training set or become ill-conditioned.
    print("\nAblation: ridge_alpha = 0 (ordinary least squares)...")
    esn_noreg = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=RHO,
        input_scaling=1.0,
        connectivity=0.1,
        bias_scaling=0.1,
        ridge_alpha=1e-15,    # effectively zero
        washout=100,
        extend_with_input=True,
        seed=SEED,
    )
    esn_noreg.fit(u_train, y_train)
    y_pred_noreg = esn_noreg.predict(u_test)
    nmse_noreg = nmse(y_test, y_pred_noreg)
    print(f"  Test NMSE (no reg.):  {nmse_noreg:.6f}")
    print(f"  Test NMSE (ridge):    {test_nmse:.6f}")
    print(f"  Ratio:                {nmse_noreg / test_nmse:.2f}x worse" if nmse_noreg > test_nmse
          else f"  (No regularisation performed similarly this seed)")

    # ─── 4. Collect reservoir states for visualisation ───────────────────────
    # Run the trained ESN on the test data and grab the raw state matrix
    states_test, _ = esn.run(u_test)   # (T_TEST, N_RESERVOIR)

    # ─── 5. Figures ───────────────────────────────────────────────────────────

    # Figure 1: True vs. predicted time series + scatter + error
    fig = plt.figure(figsize=(13, 9))
    gs = GridSpec(3, 2, figure=fig, hspace=0.45, wspace=0.3)

    t_axis = np.arange(T_TEST)
    SHOW = min(500, T_TEST)

    # (a) True vs. predicted — time series
    ax1 = fig.add_subplot(gs[0, :])
    ax1.plot(t_axis[:SHOW], y_test[:SHOW], color="steelblue",
             lw=1.5, label="True output", zorder=2)
    ax1.plot(t_axis[:SHOW], y_pred[:SHOW], color="tomato",
             lw=1.2, ls="--", label="ESN prediction", zorder=3)
    ax1.set_xlabel("Time step (test set)", fontsize=10)
    ax1.set_ylabel("y(t)", fontsize=10)
    ax1.set_title(
        f"NARMA-10 Prediction: N={N_RESERVOIR}, ρ={RHO}, λ={RIDGE_ALPHA:.0e}\n"
        f"Test NMSE = {test_nmse:.5f}",
        fontsize=11
    )
    ax1.legend(fontsize=10)

    # (b) Scatter plot
    ax2 = fig.add_subplot(gs[1, 0])
    lims = [min(y_test.min(), y_pred.min()) - 0.01,
            max(y_test.max(), y_pred.max()) + 0.01]
    ax2.scatter(y_test, y_pred, s=2, alpha=0.3, color="steelblue")
    ax2.plot(lims, lims, "r--", lw=1.5, label="y=x (perfect)")
    ax2.set_xlabel("True y", fontsize=10)
    ax2.set_ylabel("Predicted y", fontsize=10)
    ax2.set_title("Scatter: True vs. Predicted", fontsize=10)
    ax2.set_xlim(lims); ax2.set_ylim(lims)
    ax2.legend(fontsize=9)

    # (c) Error time series
    ax3 = fig.add_subplot(gs[1, 1])
    error = y_test[:SHOW] - y_pred[:SHOW]
    ax3.plot(t_axis[:SHOW], error, color="darkorange", lw=0.8)
    ax3.axhline(0, color="black", lw=0.7)
    ax3.set_xlabel("Time step (test set)", fontsize=10)
    ax3.set_ylabel("Error e(t) = y - ŷ", fontsize=10)
    ax3.set_title("Prediction Error Time Series", fontsize=10)

    # (d) NMSE comparison bar: with vs. without regularisation
    ax4 = fig.add_subplot(gs[2, 0])
    labels_bar = [f"Ridge\nλ={RIDGE_ALPHA:.0e}", "No\nRegularisation"]
    nmse_bars  = [test_nmse, nmse_noreg]
    colors_bar = ["steelblue", "tomato"]
    bars = ax4.bar(labels_bar, nmse_bars, color=colors_bar, width=0.5)
    for bar, val in zip(bars, nmse_bars):
        ax4.text(bar.get_x() + bar.get_width()/2, val + 0.002,
                 f"{val:.4f}", ha="center", fontsize=9)
    ax4.set_ylabel("Test NMSE", fontsize=10)
    ax4.set_title("Effect of Regularisation", fontsize=10)
    ax4.set_ylim(0, max(nmse_bars) * 1.4)

    # (e) No-regularisation predictions (overlay on short segment)
    ax5 = fig.add_subplot(gs[2, 1])
    seg = min(200, T_TEST)
    ax5.plot(t_axis[:seg], y_test[:seg], color="steelblue", lw=1.5,
             label="True", zorder=2)
    ax5.plot(t_axis[:seg], y_pred[:seg], color="tomato", lw=1.2, ls="--",
             label=f"Ridge (λ={RIDGE_ALPHA:.0e})", zorder=3)
    ax5.plot(t_axis[:seg], y_pred_noreg[:seg], color="purple", lw=1.0,
             ls=":", label="No reg.")
    ax5.set_xlabel("Time step", fontsize=10)
    ax5.set_ylabel("y(t)", fontsize=10)
    ax5.set_title("Regularisation Ablation (first 200 steps)", fontsize=10)
    ax5.legend(fontsize=8)

    plt.savefig("output/03_narma10_predictions.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/03_narma10_predictions.png")

    # Figure 2: Reservoir state heatmap
    fig, axes = plt.subplots(2, 1, figsize=(12, 7))

    ax = axes[0]
    hm_data = states_test[:T_HEATMAP, :N_NEURONS_SHOW].T  # (N_show, T_show)
    im = ax.imshow(hm_data, aspect="auto", cmap="RdBu_r",
                   vmin=-1, vmax=1,
                   extent=[0, T_HEATMAP, N_NEURONS_SHOW, 0])
    plt.colorbar(im, ax=ax, label="Activation")
    ax.set_xlabel("Time step (test set)", fontsize=10)
    ax.set_ylabel("Neuron index", fontsize=10)
    ax.set_title(
        f"Reservoir State Matrix: First {N_NEURONS_SHOW} Neurons, "
        f"First {T_HEATMAP} Steps\n"
        "Each row is a single neuron's activation over time.",
        fontsize=11
    )

    # Plot a few individual neuron traces
    ax2 = axes[1]
    t_trace = np.arange(T_HEATMAP)
    palette = plt.cm.tab10(np.linspace(0, 1, 8))
    for i, idx in enumerate([0, 5, 10, 20, 30, 40, 49]):
        if idx < N_RESERVOIR:
            ax2.plot(t_trace, states_test[:T_HEATMAP, idx],
                     color=palette[i % len(palette)], lw=1.2,
                     label=f"Neuron {idx}", alpha=0.8)
    ax2.set_xlabel("Time step", fontsize=10)
    ax2.set_ylabel("Activation x_i(t)", fontsize=10)
    ax2.set_title("Sample Neuron Activation Traces", fontsize=11)
    ax2.legend(fontsize=8, ncol=4, loc="upper right")
    ax2.set_xlim(0, T_HEATMAP)

    plt.tight_layout()
    plt.savefig("output/03_reservoir_state_heatmap.png", dpi=150)
    plt.close()
    print("Figure saved: output/03_reservoir_state_heatmap.png")

    # Figure 3: W_out weight visualisation
    W_out = esn.W_out.ravel()   # (N + n_inputs,)
    fig, ax = plt.subplots(figsize=(10, 3.5))
    ax.bar(np.arange(len(W_out)), W_out, width=1.0, color="steelblue", alpha=0.7)
    ax.axvline(N_RESERVOIR - 0.5, color="tomato", lw=1.5, ls="--",
               label=f"Input weight starts (idx {N_RESERVOIR})")
    ax.set_xlabel("Feature index (reservoir neurons then input)", fontsize=10)
    ax.set_ylabel("W_out weight", fontsize=10)
    ax.set_title("Readout Weight Vector W_out (trained by ridge regression)", fontsize=11)
    ax.legend(fontsize=9)
    plt.tight_layout()
    plt.savefig("output/03_readout_weights.png", dpi=150)
    plt.close()
    print("Figure saved: output/03_readout_weights.png")

    # ─── Summary ──────────────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Results Summary")
    print("=" * 60)
    print(f"  ESN configuration:  N={N_RESERVOIR}, ρ={RHO}, λ={RIDGE_ALPHA:.0e}")
    print(f"  Test NMSE (ridge):  {test_nmse:.6f}")
    print(f"  Test NMSE (no reg): {nmse_noreg:.6f}")
    print(f"  Jaeger (2002) reference NMSE ≈ 0.004 for N=200")
    print(f"  W_out: {len(W_out)} weights  "
          f"(norm={np.linalg.norm(W_out):.4f})")
    print(
        "\n  Key insight: The ESN readout is a *linear* function of the\n"
        "  reservoir state — all nonlinear temporal processing is done\n"
        "  by the random recurrent reservoir. Training is just a single\n"
        "  ridge regression, not backpropagation through time."
    )


if __name__ == "__main__":
    main()
