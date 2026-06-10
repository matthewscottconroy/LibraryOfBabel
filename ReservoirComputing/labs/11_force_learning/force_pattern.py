"""
================================================================================
Lab 11 — FORCE Learning for Autonomous Pattern Generation
================================================================================

Textbook connection
-------------------
Unit 4, Chapter 11: "FORCE Learning and Autonomous Dynamics."
Implements the simplified FORCE algorithm from Sussillo & Abbott (2009,
"Generating Coherent Patterns of Activity from Chaotic Neural Networks",
Neuron 63:544–557).

Learning objectives
-------------------
  * Understand FORCE learning: train W_out via online RLS while feeding the
    output back into the reservoir (teacher forcing during training).
  * Appreciate why a large, chaotic reservoir (ρ > 1) is used: the freely
    running reservoir already produces rich, high-dimensional dynamics, and
    FORCE 'shapes' those dynamics to match the target.
  * Observe the transition from teacher-forced to autonomous operation.
  * Compare N=500 (large) and N=50 (small) reservoirs.

Algorithm (simplified FORCE — readout-only version):
  Training:
    For t = 1..T_train:
      u_t = y*(t-1)           # teacher-forced input
      x_t = (1-α)x_{t-1} + α tanh(W_rec x_{t-1} + W_in u_t)
      y_t = W_out^T x_t       # current prediction
      RLS update: P, W_out ← step(x_t, y*(t))

  Autonomous:
    For t = T_train+1...:
      u_t = y_{t-1}           # use own output as input
      x_t = step(...)
      y_t = W_out^T x_t

RLS update (derived from first principles):
    k   = P x / (1 + x^T P x)    # Kalman gain
    e   = y* - W_out^T x          # error before update
    W_out ← W_out + k * e
    P   ← P - k (x^T P)           # covariance update

What to observe
---------------
  * During training the output tracks the target immediately after a short
    transient, because teacher forcing keeps the reservoir on a stable manifold.
  * In autonomous mode (N=500) the output remains close to the target for
    hundreds of steps.
  * N=50 often fails to sustain autonomous oscillation — the readout cannot
    correctly shape the low-dimensional dynamics.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from numpy.random import default_rng

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import nmse


# ── Configuration ─────────────────────────────────────────────────────────────

SEED       = 42
DT         = 1.0          # time step (abstract units)
T_TRAIN    = 5000
T_TEST     = 2000

# Target: sum of 4 sinusoids with incommensurable frequencies
FREQS      = [1 / 60, 1 / 87, 1 / 130, 1 / 200]   # Hz (or cycles/step)
AMPS       = [1.0, 0.7, 0.5, 0.3]
PHASES     = [0.0, np.pi / 4, np.pi / 2, np.pi]

# Reservoir (large)
N_LARGE    = 500
RHO_LARGE  = 1.2
ALPHA_LEAK = 1.0
INPUT_SC   = 1.0
CONN       = 0.1

# Small reservoir for comparison
N_SMALL    = 50

# RLS
DELTA      = 1.0   # P = delta * I


# ── Target signal ─────────────────────────────────────────────────────────────

def make_target(T, seed=0):
    """
    Sum of 4 sinusoids:
        y*(t) = sum_k A_k sin(2π f_k t + φ_k)
    """
    t = np.arange(T, dtype=float)
    y = np.zeros(T)
    for f, A, phi in zip(FREQS, AMPS, PHASES):
        y += A * np.sin(2 * np.pi * f * t + phi)
    return y


# ── Reservoir construction ────────────────────────────────────────────────────

def build_reservoir(N, rho, seed=SEED):
    """
    Build W_rec (sparse Gaussian, rescaled to ρ) and W_in (Gaussian).
    Returns (W_rec, W_in).
    """
    rng = default_rng(seed)
    # Sparse Gaussian W_rec
    mask = rng.random((N, N)) < CONN
    W = rng.standard_normal((N, N)) * mask
    eigvals = np.abs(np.linalg.eigvals(W))
    rho_W = eigvals.max()
    if rho_W < 1e-10:
        raise RuntimeError("Near-zero spectral radius. Increase N or connectivity.")
    W_rec = (rho / rho_W) * W

    # Input weights: single-column (scalar feedback)
    W_in = rng.standard_normal((N, 1)) * INPUT_SC

    return W_rec, W_in


# ── FORCE training ────────────────────────────────────────────────────────────

def force_train(N, rho, y_star, seed=SEED):
    """
    Run FORCE training:
      - Teacher force y_star as input
      - Update W_out via RLS at each step

    Returns:
      x_states_train : (T_train, N)
      y_train_pred   : (T_train,)  predictions during training
      W_out          : (N,) final readout weights
      P              : (N, N) final RLS covariance (for possible warm-start)
    """
    T = len(y_star)
    W_rec, W_in = build_reservoir(N, rho, seed=seed)

    # RLS initialisation
    P    = DELTA * np.eye(N)
    w    = np.zeros(N)

    x    = np.zeros(N)
    x_states    = np.zeros((T, N))
    y_pred_all  = np.zeros(T)

    for t in range(T):
        # Teacher-forced input: use y*(t-1) (or 0 at t=0)
        u_t = np.array([y_star[t - 1]]) if t > 0 else np.zeros(1)

        # Reservoir update
        pre = W_rec @ x + W_in @ u_t
        x   = (1.0 - ALPHA_LEAK) * x + ALPHA_LEAK * np.tanh(pre)

        # Current prediction (before update)
        y_pred = w @ x
        y_pred_all[t] = y_pred

        # RLS update (FORCE equations — derived in-line as per the spec)
        # k   = P x / (1 + x^T P x)     [Kalman gain]
        k = P @ x / (1.0 + x @ (P @ x))
        # e   = y*(t) - W_out^T x        [error]
        e = y_star[t] - y_pred
        # W_out ← W_out + k * e
        w += k * e
        # P ← P - outer(k, x^T P)
        P -= np.outer(k, x @ P)

        x_states[t] = x

    return x_states, y_pred_all, w, P


# ── Autonomous run ─────────────────────────────────────────────────────────────

def autonomous_run(N, rho, W_out, T, seed=SEED):
    """
    Run the reservoir autonomously for T steps, feeding back its own output.
    Returns:
        y_auto   : (T,) predicted output
        x_states : (T, N)
    """
    W_rec, W_in = build_reservoir(N, rho, seed=seed)
    x      = np.zeros(N)
    y_auto = np.zeros(T)
    x_states = np.zeros((T, N))

    # Warm-start state: drive with last known target value
    # (we'll discard this period — use the stored state from training instead)
    u = np.zeros(1)
    for t in range(T):
        pre = W_rec @ x + W_in @ u
        x   = (1.0 - ALPHA_LEAK) * x + ALPHA_LEAK * np.tanh(pre)
        y_pred  = W_out @ x
        y_auto[t] = y_pred
        u = np.array([y_pred])
        x_states[t] = x

    return y_auto, x_states


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # Target signal for full run
    T_total  = T_TRAIN + T_TEST
    y_star   = make_target(T_total)

    # ── Large reservoir (N=500) ────────────────────────────────────────────────
    print("=" * 60)
    print(f"FORCE Training — Large Reservoir (N={N_LARGE}, ρ={RHO_LARGE})")
    print("=" * 60)

    y_train_target = y_star[:T_TRAIN]
    y_test_target  = y_star[T_TRAIN:]

    x_tr, y_tr_pred, W_out_large, P_large = force_train(
        N_LARGE, RHO_LARGE, y_train_target, seed=SEED
    )

    train_nmse = nmse(y_train_target, y_tr_pred)
    print(f"  Training NMSE: {train_nmse:.4f}")

    # Autonomous run: start from the final reservoir state of training
    # We rebuild the autonomous loop with the trained W_out
    W_rec_large, W_in_large = build_reservoir(N_LARGE, RHO_LARGE, seed=SEED)

    x_auto = x_tr[-1].copy()  # warm start from final training state
    y_auto = np.zeros(T_TEST)
    u      = np.array([y_train_target[-1]])

    for t in range(T_TEST):
        pre    = W_rec_large @ x_auto + W_in_large @ u
        x_auto = (1.0 - ALPHA_LEAK) * x_auto + ALPHA_LEAK * np.tanh(pre)
        y_pred = W_out_large @ x_auto
        y_auto[t] = y_pred
        u = np.array([y_pred])

    auto_nmse = nmse(y_test_target, y_auto)
    print(f"  Autonomous NMSE: {auto_nmse:.4f}")

    # Lyapunov time for the network: rough estimate. For ρ=1.2 networks,
    # Lyapunov exponent λ ≈ 0.09 bits/step (Sussillo & Abbott 2009)
    lambda_max = 0.09  # nats/step (rough estimate for this reservoir)
    lyapunov_time = 1.0 / lambda_max  # steps per Lyapunov time
    # VPT: find when |error| > 0.1 * std(target)
    err_auto = np.abs(y_auto - y_test_target)
    thresh = 0.1 * np.std(y_test_target)
    exceed = np.where(err_auto > thresh)[0]
    vpt_steps = exceed[0] if len(exceed) > 0 else T_TEST
    vpt_lyap  = vpt_steps * DT / lyapunov_time
    print(f"  Stable for ≈ {vpt_steps} steps  ≈ {vpt_lyap:.1f} Lyapunov times")

    # ── Figure 1: Training and autonomous phase ────────────────────────────────
    fig, axes = plt.subplots(3, 1, figsize=(13, 10))

    # Panel a: training (last 500 steps for clarity)
    show_tr = slice(T_TRAIN - 500, T_TRAIN)
    t_tr = np.arange(T_TRAIN - 500, T_TRAIN)
    axes[0].plot(t_tr, y_train_target[show_tr], color="black", lw=1.5, label="Target y*(t)")
    axes[0].plot(t_tr, y_tr_pred[show_tr], color="steelblue", lw=1.5, ls="--",
                 label="Teacher-forced output")
    axes[0].set_ylabel("Amplitude", fontsize=10)
    axes[0].set_title(f"(a) Training Phase — Teacher Forcing (last 500 of {T_TRAIN} steps)", fontsize=10)
    axes[0].legend(fontsize=9)

    # Panel b: autonomous
    t_auto = np.arange(T_TRAIN, T_TRAIN + T_TEST)
    axes[1].plot(t_auto, y_test_target, color="black", lw=1.5, label="Target y*(t)")
    axes[1].plot(t_auto, y_auto, color="tomato", lw=1.5, ls="--", label="Autonomous output")
    axes[1].axvline(T_TRAIN + vpt_steps, color="green", ls=":", lw=2,
                    label=f"VPT ≈ {vpt_steps} steps")
    axes[1].set_ylabel("Amplitude", fontsize=10)
    axes[1].set_title(f"(b) Autonomous Phase — Self-Generated Output  (NMSE = {auto_nmse:.4f})", fontsize=10)
    axes[1].legend(fontsize=9)

    # Panel c: error
    axes[2].plot(t_auto, err_auto, color="purple", lw=1)
    axes[2].axhline(thresh, color="tomato", ls="--", lw=1.5,
                    label=f"Threshold = {thresh:.3f}")
    axes[2].axvline(T_TRAIN + vpt_steps, color="green", ls=":", lw=2)
    axes[2].set_xlabel("Time step", fontsize=11)
    axes[2].set_ylabel("|error|", fontsize=10)
    axes[2].set_title("(c) Absolute Error (Autonomous Phase)", fontsize=10)
    axes[2].legend(fontsize=9)

    fig.suptitle(
        f"FORCE Learning — N={N_LARGE}, ρ={RHO_LARGE}  |  Target: 4 Incommensurable Sinusoids",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/11_force_large_reservoir.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/11_force_large_reservoir.png")

    # ── Small reservoir comparison (N=50) ─────────────────────────────────────
    print(f"\n{'=' * 60}")
    print(f"FORCE Training — Small Reservoir (N={N_SMALL}, ρ={RHO_LARGE})")
    print("=" * 60)

    x_tr_s, y_tr_pred_s, W_out_small, _ = force_train(
        N_SMALL, RHO_LARGE, y_train_target, seed=SEED
    )
    train_nmse_s = nmse(y_train_target, y_tr_pred_s)
    print(f"  Training NMSE: {train_nmse_s:.4f}")

    W_rec_small, W_in_small = build_reservoir(N_SMALL, RHO_LARGE, seed=SEED)
    x_auto_s = x_tr_s[-1].copy()
    y_auto_s = np.zeros(T_TEST)
    u_s = np.array([y_train_target[-1]])

    for t in range(T_TEST):
        pre      = W_rec_small @ x_auto_s + W_in_small @ u_s
        x_auto_s = (1.0 - ALPHA_LEAK) * x_auto_s + ALPHA_LEAK * np.tanh(pre)
        y_pred   = W_out_small @ x_auto_s
        y_auto_s[t] = y_pred
        u_s = np.array([y_pred])

    auto_nmse_s = nmse(y_test_target, y_auto_s)
    err_auto_s = np.abs(y_auto_s - y_test_target)
    exceed_s   = np.where(err_auto_s > thresh)[0]
    vpt_s      = exceed_s[0] if len(exceed_s) > 0 else T_TEST
    print(f"  Autonomous NMSE: {auto_nmse_s:.4f}")
    print(f"  Stable for ≈ {vpt_s} steps  (vs {vpt_steps} for N={N_LARGE})")

    # ── Figure 2: Large vs small comparison ───────────────────────────────────
    fig, axes = plt.subplots(2, 2, figsize=(14, 8))

    for row, (N_size, y_auto_c, auto_err, vpt_c, label) in enumerate([
        (N_LARGE, y_auto,   err_auto,   vpt_steps, f"N={N_LARGE}"),
        (N_SMALL, y_auto_s, err_auto_s, vpt_s,     f"N={N_SMALL}"),
    ]):
        ax_sig  = axes[row, 0]
        ax_err  = axes[row, 1]
        t_axis  = np.arange(T_TEST)

        ax_sig.plot(t_axis, y_test_target, color="black", lw=1.5, label="Target")
        ax_sig.plot(t_axis, y_auto_c, color="steelblue" if row == 0 else "tomato",
                    lw=1.5, ls="--", label="Autonomous")
        ax_sig.set_title(f"{label}  —  Autonomous Output  (NMSE={auto_nmse if row==0 else auto_nmse_s:.3f})", fontsize=10)
        ax_sig.set_ylabel("Amplitude", fontsize=9)
        ax_sig.legend(fontsize=8)

        ax_err.plot(t_axis, auto_err, lw=1,
                    color="steelblue" if row == 0 else "tomato")
        ax_err.axhline(thresh, color="black", ls="--", lw=1)
        ax_err.axvline(vpt_c, color="green", ls=":", lw=1.5,
                       label=f"VPT = {vpt_c} steps")
        ax_err.set_title(f"{label}  —  Absolute Error", fontsize=10)
        ax_err.set_ylabel("|error|", fontsize=9)
        ax_err.legend(fontsize=8)
        if row == 1:
            ax_sig.set_xlabel("Autonomous step", fontsize=10)
            ax_err.set_xlabel("Autonomous step", fontsize=10)

    fig.suptitle("FORCE Learning: Large vs Small Reservoir Comparison", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/11_force_comparison.png", dpi=150)
    plt.close()
    print("Figure saved: output/11_force_comparison.png")

    # ── Final summary ─────────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Summary")
    print("=" * 60)
    print(f"  Large (N={N_LARGE}):  Training NMSE = {train_nmse:.4f}  |  Auto NMSE = {auto_nmse:.4f}")
    print(f"  Small (N={N_SMALL}):  Training NMSE = {train_nmse_s:.4f}  |  Auto NMSE = {auto_nmse_s:.4f}")
    print(f"  VPT: N={N_LARGE} → {vpt_steps} steps;  N={N_SMALL} → {vpt_s} steps")
    print(
        "\nKey insight: FORCE learning requires a reservoir large enough that\n"
        "the target trajectory can be expressed as a linear combination of\n"
        "reservoir states. N=50 often lacks the necessary dimensionality.\n"
        "The chaotic reservoir (ρ > 1) is paradoxically EASIER to train:\n"
        "its rich dynamics provide a diverse basis that spans the target."
    )


if __name__ == "__main__":
    main()
