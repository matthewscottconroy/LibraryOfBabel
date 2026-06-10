"""
================================================================================
Lab 14 — NVAR (Next-Generation RC) vs ESN on Lorenz Prediction
================================================================================

Textbook connection
-------------------
Unit 5, Chapter 14: "Next-Generation Reservoir Computing."
Reproduces the key results of Gauthier et al. (2021, "Next generation reservoir
computing", Nature Communications 12:5564).

Learning objectives
-------------------
  * Implement NVAR (Nonlinear Vector Autoregression): delay embedding + degree-2
    polynomial feature expansion, then ridge regression — no reservoir needed.
  * Understand why NVAR is remarkably efficient: the quadratic monomials provide
    the nonlinearity that a reservoir would otherwise generate.
  * Compare NVAR and ESN on Lorenz prediction using NMSE, Valid Prediction Time,
    and number of trainable parameters.
  * Inspect feature importance via ridge regression coefficients.
  * Study the sensitivity of NVAR to delay depth and polynomial degree.

NVAR feature construction (Gauthier et al. 2021):
    Input: u(t) ∈ R^d  (d = 3 for Lorenz)
    Delay embed: [u(t), u(t-k), u(t-2k), ...]    → linear features (linear_dim)
    Quadratic:   all products x_i * x_j for i ≤ j → quadratic features
    Full vector: [linear | quadratic]   D = linear_dim + C(linear_dim+1, 2)

For d=3, k=1, delay_steps=2:  D = 9 + 45 = 54  (but Gauthier uses k=1, 2 delays
giving linear_dim = 6, quadratic_dim = 21, total = 27 — matching the paper's
Table 1 exactly.)

What to observe
---------------
  * NVAR with only 27 features achieves VPT comparable to N=500 ESN.
  * NVAR has far fewer parameters (27 × 3 = 81 vs ESN's ~500 × 3).
  * Dominant NVAR coefficients correspond to self-products x·x, y·y, x·y —
    which encode the quadratic Lorenz nonlinearity.
  * Adding more delays or higher degree does not always help; optimal is 2 delays,
    degree 2 as in the original paper.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, lorenz_rk4, nmse, valid_prediction_time


# ── Configuration ─────────────────────────────────────────────────────────────

SEED      = 42
DT        = 0.02
T_TOTAL   = 20000
T_TRAIN   = 10000
T_TEST    = 10000
WASHOUT   = 200

# NVAR defaults (Gauthier et al. 2021)
K_DELAY   = 1          # stride between delay vectors
N_DELAYS  = 2          # number of extra delays (so we use t, t-k, t-2k)

# ESN baseline
N_ESN     = 500
RHO_ESN   = 0.95
IS_ESN    = 0.5
RIDGE_ESN = 1e-6

# Lorenz Lyapunov time at dt=0.02: λ_max ≈ 0.906 nats/unit-time → τ_L ≈ 1.1 s
LYAPUNOV_TIME_STEPS = int(1.1 / DT)  # ≈ 55 steps

VPT_THRESHOLD = 0.05


# ── NVAR feature builder ──────────────────────────────────────────────────────

def make_nvar_features(u, k=1, delay_steps=2):
    """
    Build NVAR feature matrix from input sequence u.

    Parameters
    ----------
    u           : (T, d) input time series
    k           : delay stride (1 = consecutive)
    delay_steps : number of additional delays (so delays are 0, k, 2k, ...)

    Returns
    -------
    features : (T, D) feature matrix
        - First T_invalid rows are set to NaN (boundary effect)
        - Caller should discard these rows

    Feature layout:
        Linear  : [u(t), u(t-k), u(t-2k), ...]  shape (T, d*(delay_steps+1))
        Quadratic: all xi*xj for i ≤ j           shape (T, linear_dim*(linear_dim+1)//2)

    For d=3, delay_steps=2, k=1:
        linear_dim  = 9 → quadratic_dim = 45 → D = 54
    For d=3, delay_steps=1, k=1  (Gauthier et al. exact):
        linear_dim  = 6 → quadratic_dim = 21 → D = 27
    """
    T, d = u.shape
    max_lag = delay_steps * k

    # Build delayed versions (roll introduces artifacts at boundary)
    delayed = [u]  # lag 0
    for i in range(1, delay_steps + 1):
        lag = i * k
        rolled = np.roll(u, lag, axis=0)
        delayed.append(rolled)

    linear = np.concatenate(delayed, axis=1)   # (T, d*(delay_steps+1))
    linear_dim = linear.shape[1]

    # Degree-2 products (upper triangle including diagonal)
    quadratic_cols = []
    for i in range(linear_dim):
        for j in range(i, linear_dim):
            quadratic_cols.append(linear[:, i] * linear[:, j])
    quadratic = np.stack(quadratic_cols, axis=1)   # (T, linear_dim*(linear_dim+1)//2)

    features = np.concatenate([linear, quadratic], axis=1)

    # Zero out the rows affected by boundary effects
    features[:max_lag] = 0.0

    return features, max_lag


# ── Ridge regression ─────────────────────────────────────────────────────────

def ridge_fit(X, Y, alpha=1e-6):
    """Solve (X^T X + αI) w = X^T Y. Returns W (n_features, n_outputs)."""
    from scipy import linalg
    A = X.T @ X + alpha * np.eye(X.shape[1])
    B = X.T @ Y
    return linalg.solve(A, B, assume_a="pos")


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate Lorenz data ──────────────────────────────────────────────────
    print("Generating Lorenz data ...")
    xyz = lorenz_rk4(T=T_TOTAL, dt=DT, seed=SEED)

    # Input is current state; target is next state (1-step-ahead prediction)
    u_all = xyz[:-1]    # (T_TOTAL-1, 3)
    y_all = xyz[1:]     # (T_TOTAL-1, 3)

    u_train = u_all[:T_TRAIN]
    y_train = y_all[:T_TRAIN]
    u_test  = u_all[T_TRAIN : T_TRAIN + T_TEST]
    y_test  = y_all[T_TRAIN : T_TRAIN + T_TEST]

    print(f"  Lorenz data shape: {xyz.shape}")
    print(f"  Train: {T_TRAIN} steps, Test: {T_TEST} steps")

    # ── NVAR (k=1, 2 delays → D=27 as in Gauthier et al. 2021) ───────────────
    print("\n" + "=" * 60)
    print("NVAR (k=1, delay_steps=1 → D=27, Gauthier et al. 2021)")
    print("=" * 60)

    # Gauthier uses delay_steps=1 giving [u(t), u(t-1)] → 6 linear features
    # and 21 quadratic → 27 total
    feat_train, max_lag = make_nvar_features(u_train, k=K_DELAY, delay_steps=1)
    feat_test,  _       = make_nvar_features(u_test,  k=K_DELAY, delay_steps=1)

    # Discard warm-up rows
    skip = max(WASHOUT, max_lag)
    X_tr = feat_train[skip:]
    Y_tr = y_train[skip:]
    X_te = feat_test[max_lag:]
    Y_te = y_test[max_lag:]

    W_nvar = ridge_fit(X_tr, Y_tr, alpha=1e-8)
    y_pred_nvar = (X_te @ W_nvar)

    nvar_nmse = nmse(Y_te, y_pred_nvar)
    nvar_vpt  = valid_prediction_time(Y_te, y_pred_nvar, threshold=VPT_THRESHOLD, dt=DT)
    nvar_params = W_nvar.size

    print(f"  Feature dimension D: {feat_train.shape[1]}")
    print(f"  Number of parameters: {nvar_params}")
    print(f"  NMSE: {nvar_nmse:.4f}")
    print(f"  VPT:  {nvar_vpt:.2f} s  ≈  {nvar_vpt / (LYAPUNOV_TIME_STEPS * DT):.2f} Lyapunov times")

    # ── ESN ────────────────────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print(f"ESN (N={N_ESN}, ρ={RHO_ESN})")
    print("=" * 60)

    esn = EchoStateNetwork(
        n_reservoir=N_ESN,
        spectral_radius=RHO_ESN,
        input_scaling=IS_ESN,
        leak_rate=1.0,
        ridge_alpha=RIDGE_ESN,
        washout=WASHOUT,
        extend_with_input=True,
        seed=SEED,
    )
    esn.fit(u_train, y_train)
    y_pred_esn = esn.predict(u_test)

    esn_nmse   = nmse(y_test, y_pred_esn)
    esn_vpt    = valid_prediction_time(y_test, y_pred_esn, threshold=VPT_THRESHOLD, dt=DT)
    esn_params = esn.W_out.size

    print(f"  Number of parameters: {esn_params}")
    print(f"  NMSE: {esn_nmse:.4f}")
    print(f"  VPT:  {esn_vpt:.2f} s  ≈  {esn_vpt / (LYAPUNOV_TIME_STEPS * DT):.2f} Lyapunov times")

    # ── Results table ─────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Results Table")
    print("=" * 65)
    print(f"{'Method':<15}  {'NMSE':>8}  {'VPT (s)':>10}  {'VPT (τ_L)':>12}  {'# Params':>10}")
    print("-" * 65)
    ly = LYAPUNOV_TIME_STEPS * DT
    print(f"{'NVAR (D=27)':<15}  {nvar_nmse:8.4f}  {nvar_vpt:10.2f}  {nvar_vpt/ly:12.2f}  {nvar_params:10d}")
    print(f"{'ESN (N=500)':<15}  {esn_nmse:8.4f}  {esn_vpt:10.2f}  {esn_vpt/ly:12.2f}  {esn_params:10d}")

    # ── Figure 1: Trajectories ─────────────────────────────────────────────────
    show = 400
    t_show = np.arange(show) * DT

    fig, axes = plt.subplots(3, 1, figsize=(13, 9), sharex=True)
    for comp, label, ax in zip(range(3), ["x", "y", "z"], axes):
        ax.plot(t_show, Y_te[:show, comp], color="black", lw=1.5, label="True")
        ax.plot(t_show, y_pred_nvar[:show, comp], color="steelblue", lw=1.5, ls="--",
                label=f"NVAR (VPT={nvar_vpt:.2f}s)")
        ax.plot(t_show, y_pred_esn[:show, comp], color="tomato", lw=1.5, ls=":",
                label=f"ESN  (VPT={esn_vpt:.2f}s)")
        ax.set_ylabel(label, fontsize=11)
        ax.legend(fontsize=8, loc="upper right")
    axes[-1].set_xlabel("Time (s)", fontsize=11)
    fig.suptitle("Lorenz 1-Step-Ahead Prediction: NVAR vs ESN", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/14_lorenz_predictions.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/14_lorenz_predictions.png")

    # ── Figure 2: Error separation ─────────────────────────────────────────────
    def separation(y_true, y_pred):
        var = np.var(y_true)
        return np.sqrt(np.mean((y_true - y_pred) ** 2, axis=1) / (var + 1e-12))

    sep_nvar = separation(Y_te[:show], y_pred_nvar[:show])
    sep_esn  = separation(y_test[:show], y_pred_esn[:show])

    fig, ax = plt.subplots(figsize=(10, 4.5))
    ax.semilogy(t_show, sep_nvar + 1e-6, color="steelblue", lw=2, label="NVAR")
    ax.semilogy(t_show, sep_esn  + 1e-6, color="tomato",    lw=2, label="ESN")
    ax.axhline(VPT_THRESHOLD, color="black", ls="--", lw=1.5, label=f"Threshold = {VPT_THRESHOLD}")
    ax.axvline(nvar_vpt, color="steelblue", ls=":", lw=1.5)
    ax.axvline(esn_vpt,  color="tomato",    ls=":", lw=1.5)
    ax.set_xlabel("Time (s)", fontsize=11)
    ax.set_ylabel("Normalised separation ||δx||", fontsize=11)
    ax.set_title("Prediction Error Growth: NVAR vs ESN", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/14_error_separation.png", dpi=150)
    plt.close()
    print("Figure saved: output/14_error_separation.png")

    # ── Feature importance ────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Feature Importance (NVAR Ridge Coefficients)")
    print("=" * 60)

    # Mean absolute coefficient across 3 output dimensions
    coeff_importance = np.abs(W_nvar).mean(axis=1)
    D = len(coeff_importance)

    # Label the features
    linear_dim_feat = 6   # d*(delay_steps+1) = 3*2 = 6
    feature_labels = []
    for delay_i in range(2):
        for comp in ["x", "y", "z"]:
            lag = delay_i * K_DELAY
            feature_labels.append(f"{comp}(t-{lag})")
    # Quadratic labels
    linear_names_short = [f"x{i}" for i in range(linear_dim_feat)]
    for i in range(linear_dim_feat):
        for j in range(i, linear_dim_feat):
            feature_labels.append(f"{linear_names_short[i]}·{linear_names_short[j]}")

    # Top-10
    top_idx = np.argsort(coeff_importance)[::-1][:10]
    print(f"\n  Top-10 most important features (mean |coefficient|):")
    for rank, idx in enumerate(top_idx, 1):
        label = feature_labels[idx] if idx < len(feature_labels) else f"feat_{idx}"
        print(f"    {rank:2d}.  [{idx:2d}] {label:<20}  {coeff_importance[idx]:.4f}")

    fig, ax = plt.subplots(figsize=(10, 4))
    ax.bar(range(D), coeff_importance, color="steelblue", alpha=0.8)
    ax.axvline(linear_dim_feat - 0.5, color="tomato", ls="--", lw=1.5,
               label="Linear | Quadratic boundary")
    ax.set_xlabel("Feature index", fontsize=11)
    ax.set_ylabel("Mean |coefficient|", fontsize=11)
    ax.set_title("NVAR Feature Importance (Mean |W| across 3 outputs)", fontsize=11)
    ax.legend(fontsize=9)
    plt.tight_layout()
    plt.savefig("output/14_feature_importance.png", dpi=150)
    plt.close()
    print("Figure saved: output/14_feature_importance.png")

    # ── Hyperparameter sensitivity: delay_steps × polynomial degree ───────────
    print("\n" + "=" * 60)
    print("NVAR Hyperparameter Sensitivity")
    print("=" * 60)

    delay_range  = [1, 2, 3, 4]
    # We approximate degree 1 by skipping quadratic; degree 3 by adding cubics
    # For simplicity, we test degree {1, 2} via make_nvar_features and no-quad mode

    vpt_grid = np.zeros((len(delay_range), 2))   # cols: degree 1, degree 2
    dim_grid  = np.zeros_like(vpt_grid, dtype=int)

    for di, n_delays in enumerate(delay_range):
        # Degree 2
        feat_tr, ml = make_nvar_features(u_train, k=1, delay_steps=n_delays)
        feat_te, _  = make_nvar_features(u_test,  k=1, delay_steps=n_delays)
        sk  = max(WASHOUT, ml)
        W_h = ridge_fit(feat_tr[sk:], y_train[sk:], alpha=1e-8)
        y_h = feat_te[ml:] @ W_h
        v   = valid_prediction_time(y_test[ml:], y_h, threshold=VPT_THRESHOLD, dt=DT)
        vpt_grid[di, 1] = v
        dim_grid[di, 1] = feat_tr.shape[1]

        # Degree 1 (linear only)
        linear_dim_l = 3 * (n_delays + 1)
        feat_tr_l = feat_tr[:, :linear_dim_l]
        feat_te_l = feat_te[:, :linear_dim_l]
        W_l = ridge_fit(feat_tr_l[sk:], y_train[sk:], alpha=1e-8)
        y_l = feat_te_l[ml:] @ W_l
        v_l = valid_prediction_time(y_test[ml:], y_l, threshold=VPT_THRESHOLD, dt=DT)
        vpt_grid[di, 0] = v_l
        dim_grid[di, 0] = linear_dim_l

        print(f"  delays={n_delays}:  deg-1 (D={dim_grid[di,0]}) VPT={vpt_grid[di,0]:.2f}s  |  "
              f"deg-2 (D={dim_grid[di,1]}) VPT={vpt_grid[di,1]:.2f}s")

    fig, ax = plt.subplots(figsize=(8, 4.5))
    ax.plot(delay_range, vpt_grid[:, 0], "o--", color="steelblue", lw=2, ms=7, label="Degree 1 (linear)")
    ax.plot(delay_range, vpt_grid[:, 1], "s-",  color="tomato",    lw=2, ms=7, label="Degree 2 (quadratic)")
    ax.axhline(esn_vpt, color="gray", ls=":", lw=1.5, label=f"ESN reference (VPT={esn_vpt:.2f}s)")
    ax.set_xlabel("Number of delays", fontsize=11)
    ax.set_ylabel("VPT (s)", fontsize=11)
    ax.set_title("NVAR Hyperparameter Sensitivity: Delay Depth vs Polynomial Degree", fontsize=11)
    ax.legend(fontsize=9)
    plt.tight_layout()
    plt.savefig("output/14_nvar_hyperparameters.png", dpi=150)
    plt.close()
    print("Figure saved: output/14_nvar_hyperparameters.png")


if __name__ == "__main__":
    main()
