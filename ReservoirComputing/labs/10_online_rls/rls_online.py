"""
================================================================================
Lab 10 — Online Readout Training via Recursive Least Squares (RLS)
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 10: "Online Learning with Recursive Least Squares."
Follows Jaeger (2003, "Adaptive Nonlinear System Identification with Echo
State Networks") and the derivation in Haykin (2002, "Adaptive Filter Theory",
Chapter 9).

Learning objectives
-------------------
  * Understand the RLS algorithm as an online alternative to batch ridge
    regression.  Both converge to the same solution.
  * See exponential error decay during online training.
  * Understand the role of δ (initialisation of the inverse covariance P = δI):
    small δ → fast initial learning, large δ → conservative start.
  * Apply online RLS to a real task: channel equalization.
  * Verify that offline and online weights converge to the same values.

The RLS update (one step):
    Gain:        k_t = P_{t-1} x_t / (1 + x_t^T P_{t-1} x_t)
    Error:       e_t = y*_t - w_{t-1}^T x_t
    Weight:      w_t = w_{t-1} + k_t e_t
    Covariance:  P_t = P_{t-1} - k_t x_t^T P_{t-1}

This is O(N^2) per step and exploits the Sherman–Morrison identity.

What to observe
---------------
  * The learning curve (error vs t) drops roughly exponentially, with the
    rate controlled by δ.
  * Small δ (aggressive): fast convergence but potentially noisy early phases.
  * Large δ (conservative): slow but smooth convergence.
  * The final weight vector matches the offline ridge regression solution to
    high precision.
  * Channel equalisation BER decreases as more training data is absorbed.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from scipy import linalg

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, nmse
from utils.esn import RLSReadout
from utils.benchmarks import channel_equalization


# ── Configuration ─────────────────────────────────────────────────────────────

SEED      = 42
N         = 100          # reservoir size
RHO       = 0.95
T_TRAIN   = 4000
T_TEST    = 2000
WASHOUT   = 100
DELTA_DEFAULT = 1.0      # default RLS initialisation
DELTA_VALUES  = [0.01, 1.0, 100.0]

# Sinusoid target parameters
FREQ_STEPS  = 50         # period in steps
NOISE_STD   = 0.1
TARGET_THRESHOLD = 0.01  # error threshold for "convergence"


# ── Build ESN and collect states ──────────────────────────────────────────────

def build_esn_states(u_seq, seed=SEED):
    """
    Drive ESN with white-noise input u_seq.  Returns extended state matrix.
    u_seq : (T,) 1-D array
    """
    esn = EchoStateNetwork(
        n_reservoir=N,
        spectral_radius=RHO,
        input_scaling=0.5,
        leak_rate=1.0,
        ridge_alpha=1e-6,
        washout=0,
        extend_with_input=True,
        seed=seed,
    )
    esn._build_reservoir(1)
    _, X = esn.run(u_seq.reshape(-1, 1))
    return esn, X   # X : (T, N+1)


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate sinusoid + noise target with white-noise reservoir input ──────
    rng = np.random.default_rng(SEED)
    T_total = T_TRAIN + T_TEST

    t_axis = np.arange(T_total)
    y_target_full = np.sin(2 * np.pi * t_axis / FREQ_STEPS) + NOISE_STD * rng.standard_normal(T_total)

    u_input = rng.standard_normal(T_total) * 0.5   # white-noise driving input

    esn, X_full = build_esn_states(u_input, seed=SEED)

    X_train = X_full[WASHOUT:T_TRAIN]
    y_train = y_target_full[WASHOUT:T_TRAIN]
    X_test  = X_full[T_TRAIN:]
    y_test  = y_target_full[T_TRAIN:]

    # ── Part 1: Online RLS training ───────────────────────────────────────────
    print("=" * 60)
    print("Part 1: Online RLS Training")
    print("=" * 60)

    n_features = X_train.shape[1]
    rls = RLSReadout(n_features=n_features, n_outputs=1, delta=DELTA_DEFAULT)

    online_preds  = np.zeros(len(X_train))
    online_errors = np.zeros(len(X_train))

    for t in range(len(X_train)):
        y_pred = rls.step(X_train[t], np.array([y_train[t]]))
        online_preds[t]  = y_pred[0]
        online_errors[t] = abs(y_train[t] - y_pred[0])

    w_online = rls.w[:, 0].copy()

    # ── Offline ridge regression for comparison ───────────────────────────────
    A = X_train.T @ X_train + 1e-6 * np.eye(n_features)
    B = X_train.T @ y_train
    w_offline = linalg.solve(A, B, assume_a="pos")

    # Weight distance between online and offline
    w_dist = np.linalg.norm(w_online - w_offline)
    print(f"  Online  NMSE (train): {nmse(y_train, online_preds):.4f}")
    offline_train_preds = X_train @ w_offline
    print(f"  Offline NMSE (train): {nmse(y_train, offline_train_preds):.4f}")
    print(f"  ||w_online - w_offline||: {w_dist:.6f}")

    # Test predictions
    y_test_pred_online  = X_test @ w_online
    y_test_pred_offline = X_test @ w_offline
    print(f"  Online  NMSE (test):  {nmse(y_test, y_test_pred_online):.4f}")
    print(f"  Offline NMSE (test):  {nmse(y_test, y_test_pred_offline):.4f}")

    # Convergence: steps until error < threshold
    smoothed_err = np.convolve(online_errors, np.ones(50) / 50, mode="same")
    conv_steps = np.where(smoothed_err < TARGET_THRESHOLD)[0]
    conv_at = conv_steps[0] if len(conv_steps) > 0 else len(online_errors)
    print(f"\n  Convergence (error < {TARGET_THRESHOLD}): step {conv_at}")

    # ── Figure 1: Online convergence ─────────────────────────────────────────
    fig, axes = plt.subplots(3, 1, figsize=(12, 9), sharex=False)

    # Panel a: prediction vs target (last 500 training steps)
    show = slice(len(X_train) - 500, None)
    t_show = np.arange(WASHOUT, T_TRAIN)[show]
    axes[0].plot(t_show, y_train[show], color="black", lw=1.5, label="Target y*(t)")
    axes[0].plot(t_show, online_preds[show], color="tomato", lw=1.5, ls="--",
                 label="Online RLS prediction")
    axes[0].set_ylabel("Signal", fontsize=10)
    axes[0].set_title("(a) Online RLS Prediction (final 500 training steps)", fontsize=10)
    axes[0].legend(fontsize=9)

    # Panel b: learning curve
    t_lc = np.arange(len(online_errors))
    axes[1].semilogy(t_lc, online_errors + 1e-10, color="steelblue", lw=1, alpha=0.5)
    axes[1].semilogy(t_lc, smoothed_err + 1e-10, color="steelblue", lw=2, label="Smoothed error")
    axes[1].axhline(TARGET_THRESHOLD, color="tomato", ls="--", lw=1.5,
                    label=f"Threshold = {TARGET_THRESHOLD}")
    if conv_at < len(online_errors):
        axes[1].axvline(conv_at, color="green", ls=":", lw=1.5,
                        label=f"Converged at t={conv_at}")
    axes[1].set_ylabel("|error|", fontsize=10)
    axes[1].set_title("(b) Learning Curve: Absolute Error vs Time", fontsize=10)
    axes[1].legend(fontsize=9)

    # Panel c: weight comparison
    axes[2].plot(w_online,  color="steelblue", lw=1.5, label="Online RLS weights")
    axes[2].plot(w_offline, color="tomato",    lw=1.5, ls="--", label="Offline ridge weights")
    axes[2].set_xlabel("Feature index", fontsize=10)
    axes[2].set_ylabel("Weight", fontsize=10)
    axes[2].set_title(f"(c) Weight Comparison  ||w_online - w_offline|| = {w_dist:.2e}", fontsize=10)
    axes[2].legend(fontsize=9)

    plt.tight_layout()
    plt.savefig("output/10_rls_convergence.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/10_rls_convergence.png")

    # ── Part 2: Effect of delta ───────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Part 2: Effect of δ (Initial Covariance P = δI)")
    print("=" * 60)

    fig, ax = plt.subplots(figsize=(10, 5))
    colors_delta = ["green", "steelblue", "tomato"]

    for delta, color in zip(DELTA_VALUES, colors_delta):
        rls_d = RLSReadout(n_features=n_features, n_outputs=1, delta=delta)
        errors_d = np.zeros(len(X_train))
        for t in range(len(X_train)):
            y_p = rls_d.step(X_train[t], np.array([y_train[t]]))
            errors_d[t] = abs(y_train[t] - y_p[0])

        smoothed_d = np.convolve(errors_d, np.ones(30) / 30, mode="same")
        ax.semilogy(smoothed_d + 1e-10, color=color, lw=2, label=f"δ = {delta}")

        # Convergence step
        cs = np.where(smoothed_d < TARGET_THRESHOLD)[0]
        ca = cs[0] if len(cs) > 0 else len(errors_d)
        print(f"  δ = {delta:6.2f}  →  convergence step: {ca}")

    ax.axhline(TARGET_THRESHOLD, color="black", ls=":", lw=1.5,
               label=f"Threshold = {TARGET_THRESHOLD}")
    ax.set_xlabel("Training step", fontsize=11)
    ax.set_ylabel("Smoothed |error| (log scale)", fontsize=11)
    ax.set_title("Effect of δ (Initial P = δI) on RLS Convergence Speed", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/10_rls_delta_effect.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/10_rls_delta_effect.png")

    print(
        "\n  Interpretation:\n"
        "  Small δ: P starts small → large initial gain k → rapid but noisy adaptation.\n"
        "  Large δ: P starts large → small initial gain → cautious, smooth convergence.\n"
        "  All three converge to the same offline solution."
    )

    # ── Part 3: Channel equalization with RLS ─────────────────────────────────
    print("\n" + "=" * 60)
    print("Part 3: Channel Equalization via Online RLS")
    print("=" * 60)

    T_CEQ = 6000
    u_ch, d_ch = channel_equalization(T=T_CEQ, snr_db=20.0, seed=0)

    # Build ESN for channel eq
    esn_ch = EchoStateNetwork(
        n_reservoir=N,
        spectral_radius=RHO,
        input_scaling=1.0,
        leak_rate=1.0,
        washout=0,
        extend_with_input=True,
        seed=SEED,
    )
    esn_ch._build_reservoir(1)
    _, X_ch = esn_ch.run(u_ch.reshape(-1, 1))

    # Online RLS on channel task
    rls_ch = RLSReadout(n_features=X_ch.shape[1], n_outputs=1, delta=1.0)
    window  = 200   # compute BER over rolling window
    ber_trace = []
    decisions = []

    symbols = np.array([-3.0, -1.0, 1.0, 3.0])

    for t in range(len(X_ch)):
        y_pred = rls_ch.step(X_ch[t], np.array([d_ch[t]]))
        # Hard decision: find nearest symbol
        decision = symbols[np.argmin(np.abs(symbols - y_pred[0]))]
        decisions.append(decision)

    decisions = np.array(decisions)
    # Compute BER in sliding window
    for start in range(0, len(decisions) - window, window // 4):
        end = start + window
        ber = np.mean(decisions[start:end] != d_ch[start:end])
        ber_trace.append((start + window // 2, ber))

    t_ber, ber_vals = zip(*ber_trace)
    final_ber = float(np.mean(decisions[T_CEQ // 2:] != d_ch[T_CEQ // 2:]))
    print(f"  Final BER (second half): {final_ber:.4f}")
    print(f"  Final BER (last 500):    {np.mean(decisions[-500:] != d_ch[-500:]):.4f}")

    fig, axes = plt.subplots(2, 1, figsize=(11, 7))

    # Received signal and decisions
    t_show_ch = slice(WASHOUT, WASHOUT + 400)
    axes[0].plot(u_ch[t_show_ch], color="gray", lw=1, alpha=0.7, label="Received signal")
    axes[0].step(np.arange(WASHOUT, WASHOUT + 400), d_ch[t_show_ch], where="post",
                 color="black", lw=1.5, label="True symbols")
    axes[0].step(np.arange(WASHOUT, WASHOUT + 400), decisions[t_show_ch], where="post",
                 color="tomato", lw=1, ls="--", alpha=0.8, label="RLS decisions")
    axes[0].set_ylabel("Amplitude", fontsize=10)
    axes[0].set_title("(a) Channel Equalization: Received Signal and Symbol Decisions", fontsize=10)
    axes[0].legend(fontsize=8)

    # BER over time
    axes[1].plot(t_ber, ber_vals, "o-", color="steelblue", ms=4, lw=1.5)
    axes[1].set_xlabel("Training step", fontsize=11)
    axes[1].set_ylabel("Bit Error Rate", fontsize=11)
    axes[1].set_title("(b) BER vs Training Time (Online RLS)", fontsize=10)
    axes[1].set_ylim(0, 1)
    axes[1].axhline(final_ber, color="tomato", ls="--", lw=1.5,
                    label=f"Final BER = {final_ber:.3f}")
    axes[1].legend(fontsize=9)

    plt.tight_layout()
    plt.savefig("output/10_channel_equalization.png", dpi=150)
    plt.close()
    print("Figure saved: output/10_channel_equalization.png")


if __name__ == "__main__":
    main()
