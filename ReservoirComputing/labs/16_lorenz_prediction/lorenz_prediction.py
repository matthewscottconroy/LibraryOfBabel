"""
================================================================================
Lab 16 — Reservoir Computing for Chaotic Time Series Prediction
================================================================================

Textbook connection
-------------------
Unit 5, Chapter 16: "Predicting Chaos."
Reproduces the landmark results of Jaeger & Haas (2004, "Harnessing
nonlinearity: predicting chaotic systems and saving energy in wireless
communication", Science 304:78–80).

Learning objectives
-------------------
  * Apply an ESN to 3-component Lorenz prediction.
  * Understand Valid Prediction Time (VPT) in Lyapunov times.
  * Reconstruct the Lorenz attractor from predictions and compare with the
    true attractor using the Grassberger-Procaccia correlation dimension D_2.
  * Explore partial observation: predict from x(t) only using delay embedding
    (Takens' theorem).
  * Quantify noise robustness.

Lyapunov time: For the Lorenz system (σ=10, ρ=28, β=8/3, dt=0.02),
    λ_max ≈ 0.906 nats/unit-time → τ_L ≈ 1.105 time units ≈ 55 steps.

What to observe
---------------
  * Full-observation VPT: typically 5–12 Lyapunov times with N=500.
  * Delay-embedding VPT: close to full-observation despite seeing only x(t).
  * Correlation dimension D_2 ≈ 2.05 for Lorenz; the ESN prediction should
    reproduce this to within ≈ 0.1.
  * Noise (SNR=20 dB) degrades VPT but the ESN remains robust.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D   # noqa: F401

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, lorenz_rk4, nmse, valid_prediction_time


# ── Configuration ─────────────────────────────────────────────────────────────

SEED       = 42
DT         = 0.02
T_TOTAL    = 20000
T_TRAIN    = 15000
T_TEST     = 5000
WASHOUT    = 200

N_ESN      = 500
RHO        = 0.95
IS         = 0.5
RIDGE      = 1e-6

# Lyapunov time for standard Lorenz at dt=0.02
LAMBDA_MAX         = 0.906          # nats / time unit
LYAPUNOV_DT        = 1.0 / LAMBDA_MAX   # ≈ 1.10 time units
LYAPUNOV_STEPS     = int(LYAPUNOV_DT / DT)   # ≈ 55 steps

VPT_THRESHOLD      = 0.05

# Noise robustness
SNR_DB             = 20.0

# Grassberger-Procaccia parameters
GP_N_POINTS        = 2000    # subsample for GP to keep it fast
GP_EPSILON_RANGE   = np.logspace(-1.5, 1.0, 40)


# ── Correlation dimension (Grassberger-Procaccia) ─────────────────────────────

def grassberger_procaccia(traj, n_points=GP_N_POINTS, epsilon_range=GP_EPSILON_RANGE):
    """
    Estimate correlation dimension D_2 using the Grassberger-Procaccia algorithm.

    C(ε) = (2 / M^2) #{(i,j) : i < j, ||x_i - x_j|| < ε}
    D_2  = d log C / d log ε   (slope of log C vs log ε in the scaling region)

    Parameters
    ----------
    traj     : (T, d) trajectory
    n_points : subsample size (for speed)

    Returns
    -------
    d2    : float  estimated correlation dimension
    epsilons : array
    C_eps    : array
    """
    rng = np.random.default_rng(SEED)
    idx  = rng.choice(len(traj), size=min(n_points, len(traj)), replace=False)
    pts  = traj[idx]

    # Pairwise distances (vectorised for speed on n_points ≤ 2000)
    # ||x_i - x_j|| for all pairs
    diff = pts[:, None, :] - pts[None, :, :]
    dists = np.sqrt((diff ** 2).sum(axis=-1))   # (M, M)

    M = len(pts)
    C = np.array([
        (dists < eps).sum() - M   # subtract diagonal
        for eps in epsilon_range
    ]) / (M * (M - 1))

    C = np.clip(C, 1e-10, None)

    # Estimate slope in the linear region of log-log plot
    log_e = np.log(epsilon_range)
    log_C = np.log(C)

    # Find the linear region: exclude very small and very large ε
    valid = (C > 1e-6) & (C < 0.8)
    if valid.sum() < 5:
        d2 = float("nan")
    else:
        slope, _ = np.polyfit(log_e[valid], log_C[valid], 1)
        d2 = slope

    return d2, epsilon_range, C


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate Lorenz data ──────────────────────────────────────────────────
    print("Generating Lorenz attractor ...")
    xyz = lorenz_rk4(T=T_TOTAL, dt=DT, seed=SEED)
    print(f"  Shape: {xyz.shape}")

    # 1-step-ahead: input = x_t, target = x_{t+1}
    u_all = xyz[:-1]
    y_all = xyz[1:]

    u_train = u_all[:T_TRAIN]
    y_train = y_all[:T_TRAIN]
    u_test  = u_all[T_TRAIN : T_TRAIN + T_TEST]
    y_test  = y_all[T_TRAIN : T_TRAIN + T_TEST]

    # ── Part 1: Full observation ESN ──────────────────────────────────────────
    print("\n" + "=" * 60)
    print(f"Full Observation ESN (N={N_ESN}, ρ={RHO})")
    print("=" * 60)

    esn = EchoStateNetwork(
        n_reservoir=N_ESN,
        spectral_radius=RHO,
        input_scaling=IS,
        leak_rate=1.0,
        ridge_alpha=RIDGE,
        washout=WASHOUT,
        extend_with_input=True,
        seed=SEED,
    )
    esn.fit(u_train, y_train)
    y_pred = esn.predict(u_test)

    esn_nmse = nmse(y_test, y_pred)
    esn_vpt  = valid_prediction_time(y_test, y_pred, threshold=VPT_THRESHOLD, dt=DT)
    esn_vpt_lt = esn_vpt / LYAPUNOV_DT

    print(f"  NMSE: {esn_nmse:.4f}")
    print(f"  VPT:  {esn_vpt:.2f} time units  ≈  {esn_vpt_lt:.1f} Lyapunov times")

    # ── Figure 1: Prediction traces ───────────────────────────────────────────
    show_n = 600
    t_show = np.arange(show_n) * DT

    fig, axes = plt.subplots(3, 1, figsize=(13, 9), sharex=True)
    comp_labels = ["x", "y", "z"]
    for ci, (label, ax) in enumerate(zip(comp_labels, axes)):
        ax.plot(t_show, y_test[:show_n, ci], color="black", lw=1.5, label="True")
        ax.plot(t_show, y_pred[:show_n, ci], color="steelblue", lw=1.5, ls="--",
                label=f"ESN prediction")
        vpt_line = esn_vpt
        ax.axvline(vpt_line, color="tomato", ls=":", lw=1.5,
                   label=f"VPT ≈ {esn_vpt_lt:.1f} τ_L" if ci == 0 else None)
        ax.set_ylabel(label, fontsize=11)
        ax.legend(fontsize=8, loc="upper right")

    axes[-1].set_xlabel("Time (s)", fontsize=11)
    fig.suptitle(f"Lorenz Prediction — ESN N={N_ESN}, ρ={RHO}  |  VPT ≈ {esn_vpt_lt:.1f} Lyapunov times",
                 fontsize=12)
    plt.tight_layout()
    plt.savefig("output/16_lorenz_traces.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/16_lorenz_traces.png")

    # ── Figure 2: Attractor reconstruction ───────────────────────────────────
    print("\nReconstructing attractors ...")
    fig = plt.figure(figsize=(12, 5))

    ax_true = fig.add_subplot(121, projection="3d")
    ax_true.scatter(y_test[:, 0], y_test[:, 1], y_test[:, 2],
                    s=0.3, c=np.arange(len(y_test)), cmap="viridis", alpha=0.4)
    ax_true.set_xlabel("x"); ax_true.set_ylabel("y"); ax_true.set_zlabel("z")
    ax_true.set_title("True Lorenz Attractor", fontsize=11)

    ax_pred = fig.add_subplot(122, projection="3d")
    ax_pred.scatter(y_pred[:, 0], y_pred[:, 1], y_pred[:, 2],
                    s=0.3, c=np.arange(len(y_pred)), cmap="plasma", alpha=0.4)
    ax_pred.set_xlabel("x"); ax_pred.set_ylabel("y"); ax_pred.set_zlabel("z")
    ax_pred.set_title("Predicted Lorenz Attractor", fontsize=11)

    plt.suptitle("Attractor Reconstruction", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/16_attractor_reconstruction.png", dpi=150)
    plt.close()
    print("Figure saved: output/16_attractor_reconstruction.png")

    # ── Correlation dimension ─────────────────────────────────────────────────
    print("\nEstimating correlation dimension D_2 ...")
    d2_true, eps_t, C_t = grassberger_procaccia(y_test)
    d2_pred, eps_p, C_p = grassberger_procaccia(y_pred)
    print(f"  D_2 (true):      {d2_true:.3f}  (expected ≈ 2.05 for Lorenz)")
    print(f"  D_2 (predicted): {d2_pred:.3f}")

    fig, ax = plt.subplots(figsize=(7, 4.5))
    ax.loglog(eps_t, C_t, color="black", lw=2, label=f"True (D_2 ≈ {d2_true:.2f})")
    ax.loglog(eps_p, C_p, color="steelblue", lw=2, ls="--", label=f"Predicted (D_2 ≈ {d2_pred:.2f})")
    ax.set_xlabel("ε", fontsize=11)
    ax.set_ylabel("C(ε)", fontsize=11)
    ax.set_title("Grassberger-Procaccia Correlation Integral", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/16_correlation_dimension.png", dpi=150)
    plt.close()
    print("Figure saved: output/16_correlation_dimension.png")

    # ── Part 2: Partial observation with delay embedding ──────────────────────
    print("\n" + "=" * 60)
    print("Partial Observation: x(t) only + Takens Delay Embedding")
    print("=" * 60)

    # Use only x-component; delay-embed to [x(t), x(t-1), x(t-2)]
    DELAY = 2
    x_series = xyz[:, 0]   # x component only

    # Build delay-embedded input: (T, 3) where columns are [x(t), x(t-1), x(t-2)]
    T_all = len(x_series)
    u_delay = np.column_stack([
        x_series[DELAY:],
        x_series[DELAY-1:-1],
        x_series[DELAY-2:-2],
    ])   # (T_all - DELAY, 3)

    # Target: x(t+1)
    y_delay = x_series[DELAY + 1:]   # (T_all - DELAY - 1,)
    # Align lengths
    min_len = min(len(u_delay), len(y_delay))
    u_delay = u_delay[:min_len]
    y_delay = y_delay[:min_len]

    u_tr_d = u_delay[:T_TRAIN]
    y_tr_d = y_delay[:T_TRAIN]
    u_te_d = u_delay[T_TRAIN : T_TRAIN + T_TEST]
    y_te_d = y_delay[T_TRAIN : T_TRAIN + T_TEST]

    esn_delay = EchoStateNetwork(
        n_reservoir=N_ESN,
        spectral_radius=RHO,
        input_scaling=IS,
        leak_rate=1.0,
        ridge_alpha=RIDGE,
        washout=WASHOUT,
        extend_with_input=True,
        seed=SEED,
    )
    esn_delay.fit(u_tr_d, y_tr_d)
    y_pred_delay = esn_delay.predict(u_te_d)

    nmse_delay = nmse(y_te_d, y_pred_delay)
    vpt_delay  = valid_prediction_time(y_te_d, y_pred_delay, threshold=VPT_THRESHOLD, dt=DT)
    vpt_delay_lt = vpt_delay / LYAPUNOV_DT

    print(f"  Full observation:  VPT = {esn_vpt:.2f} s  ≈ {esn_vpt_lt:.1f} τ_L")
    print(f"  Delay embedding:   VPT = {vpt_delay:.2f} s  ≈ {vpt_delay_lt:.1f} τ_L  (x only + {DELAY} delays)")
    print(f"  Delay NMSE: {nmse_delay:.4f}")

    # ── Part 3: Noise robustness ──────────────────────────────────────────────
    print("\n" + "=" * 60)
    print(f"Noise Robustness (SNR = {SNR_DB} dB)")
    print("=" * 60)

    # Add Gaussian noise to the input
    signal_power = np.var(u_train)
    noise_power  = signal_power / (10 ** (SNR_DB / 10))
    rng = np.random.default_rng(SEED + 1)
    u_train_noisy = u_train + rng.standard_normal(u_train.shape) * np.sqrt(noise_power)
    u_test_noisy  = u_test  + rng.standard_normal(u_test.shape)  * np.sqrt(noise_power)

    esn_noisy = EchoStateNetwork(
        n_reservoir=N_ESN, spectral_radius=RHO, input_scaling=IS,
        leak_rate=1.0, ridge_alpha=RIDGE, washout=WASHOUT,
        extend_with_input=True, seed=SEED,
    )
    esn_noisy.fit(u_train_noisy, y_train)
    y_pred_noisy = esn_noisy.predict(u_test_noisy)

    nmse_noisy = nmse(y_test, y_pred_noisy)
    vpt_noisy  = valid_prediction_time(y_test, y_pred_noisy, threshold=VPT_THRESHOLD, dt=DT)
    vpt_noisy_lt = vpt_noisy / LYAPUNOV_DT

    print(f"  Clean input:  VPT = {esn_vpt:.2f} s  ≈ {esn_vpt_lt:.1f} τ_L  |  NMSE = {esn_nmse:.4f}")
    print(f"  Noisy input:  VPT = {vpt_noisy:.2f} s  ≈ {vpt_noisy_lt:.1f} τ_L  |  NMSE = {nmse_noisy:.4f}")
    vpt_drop = (esn_vpt - vpt_noisy) / esn_vpt * 100
    print(f"  VPT degradation: {vpt_drop:.1f}%")

    # ── Figure 3: VPT comparison ──────────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(8, 5))
    configs  = ["Full\nobservation", f"Delay embed\n(x only, d={DELAY})", f"Noisy input\n(SNR={SNR_DB}dB)"]
    vpt_vals = [esn_vpt_lt, vpt_delay_lt, vpt_noisy_lt]
    colors   = ["steelblue", "darkorange", "tomato"]
    bars = ax.bar(configs, vpt_vals, color=colors, alpha=0.8, edgecolor="black", lw=1.2)
    for bar, val in zip(bars, vpt_vals):
        ax.text(bar.get_x() + bar.get_width() / 2, bar.get_height() + 0.1,
                f"{val:.1f} τ_L", ha="center", va="bottom", fontsize=11, fontweight="bold")
    ax.set_ylabel("Valid Prediction Time (Lyapunov times)", fontsize=11)
    ax.set_title(f"VPT Comparison — Lorenz Prediction with N={N_ESN} ESN", fontsize=11)
    ax.set_ylim(0, max(vpt_vals) * 1.25)
    plt.tight_layout()
    plt.savefig("output/16_vpt_comparison.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/16_vpt_comparison.png")

    # ── Final summary ─────────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Summary")
    print("=" * 60)
    print(f"  ESN full observation:   VPT = {esn_vpt_lt:.2f} τ_L  |  NMSE = {esn_nmse:.4f}")
    print(f"  Delay embedding (x):    VPT = {vpt_delay_lt:.2f} τ_L  |  NMSE = {nmse_delay:.4f}")
    print(f"  Noisy (SNR={SNR_DB}dB): VPT = {vpt_noisy_lt:.2f} τ_L  |  NMSE = {nmse_noisy:.4f}")
    print(f"  D_2 (true): {d2_true:.3f}  |  D_2 (pred): {d2_pred:.3f}")


if __name__ == "__main__":
    main()
