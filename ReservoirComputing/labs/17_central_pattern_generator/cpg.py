"""
================================================================================
Lab 17 — Reservoir as a Central Pattern Generator for Robot Locomotion
================================================================================

Textbook connection
-------------------
Unit 5, Chapter 17: "Reservoir Computing for Motor Control."
Inspired by Ijspeert (2008, "Central pattern generators for locomotion control
in animals and robots: a review", Neural Networks 21:642–653) and the
reservoir-based CPG work of Lechner et al. (2020).

Learning objectives
-------------------
  * Understand how a trained reservoir can function as a CPG — producing
    stable rhythmic output without sensory feedback.
  * Appreciate the role of a supra-critical reservoir (ρ > 1): it supplies
    rich, self-sustaining dynamics that FORCE can sculpt into any target rhythm.
  * Observe the limit-cycle property: perturbed trajectories return to the
    gait pattern.
  * Read phase portraits (hip vs knee) and foot contact diagrams.

Gait definition (trot):
  Diagonal pairs FL+RR move together; FR+RL are in antiphase.
  Each leg: hip (primary) + knee (secondary) sinusoids.
  8 outputs total: FL_hip, FL_knee, FR_hip, FR_knee, RL_hip, RL_knee, RR_hip, RR_knee.

Training: FORCE with teacher forcing — same as Lab 11, but for 8 outputs.

What to observe
---------------
  * Stable 8-channel limit cycle in autonomous mode.
  * Hip–knee phase portraits show clean ellipses, characteristic of a limit cycle.
  * Foot contact diagram reproduces the trot gait pattern (alternating diagonal pairs).
  * After a perturbation, the reservoir returns to the same rhythm within ~1 cycle.
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
N          = 300           # reservoir size
RHO        = 1.25          # above stability: chaotic reservoir
ALPHA_LEAK = 1.0
INPUT_SC   = 1.0
CONN       = 0.1
DELTA      = 1.0           # RLS initial covariance

DT         = 0.01          # time step (seconds)
FREQ_GAIT  = 1.5           # gait frequency (Hz)
T_TRAIN    = 6000          # training steps
T_TEST     = 2000          # autonomous test steps
T_PERTURB  = 500           # perturbation test length (post-perturbation)

CONTACT_THRESHOLD = 0.0    # hip flexion > 0 → leg in stance phase


# ── Gait target ───────────────────────────────────────────────────────────────

def make_gait(T, dt=DT, freq=FREQ_GAIT):
    """
    Generate 8-channel trot gait target.
    Channels: FL_hip, FL_knee, FR_hip, FR_knee, RL_hip, RL_knee, RR_hip, RR_knee

    Trot: FL+RR in phase; FR+RL in antiphase.
    Each joint: primary (hip) + secondary (knee) with a phase lead.
    """
    t = np.arange(T) * dt
    omega = 2 * np.pi * freq
    # Hip: fundamental
    # Knee: second harmonic with π/4 phase offset (characteristic knee bend)
    fl_hip  = np.sin(omega * t)
    fl_knee = 0.5 * np.sin(2 * omega * t + np.pi / 4)
    fr_hip  = np.sin(omega * t + np.pi)
    fr_knee = 0.5 * np.sin(2 * omega * t + np.pi + np.pi / 4)
    rl_hip  = np.sin(omega * t + np.pi)   # RL same as FR (trot diagonal)
    rl_knee = 0.5 * np.sin(2 * omega * t + np.pi + np.pi / 4)
    rr_hip  = np.sin(omega * t)           # RR same as FL
    rr_knee = 0.5 * np.sin(2 * omega * t + np.pi / 4)

    return np.column_stack([fl_hip, fl_knee, fr_hip, fr_knee,
                             rl_hip, rl_knee, rr_hip, rr_knee])   # (T, 8)


# ── Reservoir construction ────────────────────────────────────────────────────

def build_reservoir(N=N, rho=RHO, seed=SEED):
    """Build W_rec (N×N) and W_in (N×8). Returns (W_rec, W_in)."""
    rng = default_rng(seed)
    mask  = rng.random((N, N)) < CONN
    W     = rng.standard_normal((N, N)) * mask
    rho_W = np.abs(np.linalg.eigvals(W)).max()
    W_rec = (rho / rho_W) * W
    W_in  = rng.standard_normal((N, 8)) * INPUT_SC
    return W_rec, W_in


# ── FORCE training (8-output version) ─────────────────────────────────────────

def force_train_gait(y_star, N=N, rho=RHO, seed=SEED):
    """
    FORCE training for an 8-dimensional gait target.

    Teacher forcing: feed y*(t-1) as the 8-dim input at each step.
    RLS update: update W_out (N×8) at each step.

    Returns:
        W_out : (N, 8) trained readout
        x_final : (N,) final reservoir state
        y_pred_train : (T_TRAIN, 8) online training predictions
    """
    T, n_out = y_star.shape
    W_rec, W_in = build_reservoir(N=N, rho=rho, seed=seed)

    # RLS: one P matrix, one W per output
    P = DELTA * np.eye(N)
    W = np.zeros((N, n_out))

    x = np.zeros(N)
    y_pred_train = np.zeros_like(y_star)

    for t in range(T):
        # Teacher-forced input: y*(t-1)
        u_t = y_star[t - 1] if t > 0 else np.zeros(n_out)

        # Reservoir step
        pre = W_rec @ x + W_in @ u_t
        x   = (1.0 - ALPHA_LEAK) * x + ALPHA_LEAK * np.tanh(pre)

        # Prediction before update
        y_pred = W.T @ x   # (n_out,)
        y_pred_train[t] = y_pred

        # RLS update (shared P for all outputs — simplified FORCE)
        Px = P @ x
        k  = Px / (1.0 + x @ Px)
        e  = y_star[t] - y_pred       # (n_out,)
        W  += np.outer(k, e)
        P  -= np.outer(k, Px)

    return W, x, y_pred_train, W_rec, W_in


# ── Autonomous run ─────────────────────────────────────────────────────────────

def autonomous_run(W_out, W_rec, W_in, x_init, T, perturbation_at=None,
                   perturbation_std=0.5):
    """
    Run autonomously for T steps.  Optionally inject a perturbation.

    perturbation_at : int or None  — if set, add noise burst at this step
    perturbation_std : float       — std of the noise burst
    """
    n_out = W_out.shape[1]
    x     = x_init.copy()
    y_auto = np.zeros((T, n_out))
    u     = np.zeros(n_out)

    for t in range(T):
        pre = W_rec @ x + W_in @ u
        x   = (1.0 - ALPHA_LEAK) * x + ALPHA_LEAK * np.tanh(pre)

        if perturbation_at is not None and t == perturbation_at:
            rng = default_rng(SEED + 99)
            x  += rng.standard_normal(len(x)) * perturbation_std

        y_pred = W_out.T @ x
        y_auto[t] = y_pred
        u = y_pred

    return y_auto


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate gait target ──────────────────────────────────────────────────
    print("=" * 65)
    print(f"FORCE CPG Training  (N={N}, ρ={RHO}, freq={FREQ_GAIT} Hz)")
    print("=" * 65)

    y_train_target = make_gait(T_TRAIN, dt=DT, freq=FREQ_GAIT)
    y_test_target  = make_gait(T_TEST,  dt=DT, freq=FREQ_GAIT)

    # ── FORCE training ────────────────────────────────────────────────────────
    W_out, x_final, y_pred_train, W_rec, W_in = force_train_gait(
        y_train_target, N=N, rho=RHO, seed=SEED
    )

    train_nmse = nmse(y_train_target, y_pred_train)
    print(f"  Training NMSE: {train_nmse:.4f}")

    # ── Autonomous run ────────────────────────────────────────────────────────
    y_auto = autonomous_run(W_out, W_rec, W_in, x_final, T_TEST)
    auto_nmse = nmse(y_test_target, y_auto)

    # VPT per channel
    err_auto = np.abs(y_auto - y_test_target)
    thresh = 0.1 * np.std(y_test_target)
    exceed = np.where(err_auto.mean(axis=1) > thresh)[0]
    vpt = exceed[0] if len(exceed) > 0 else T_TEST
    print(f"  Autonomous NMSE: {auto_nmse:.4f}")
    print(f"  Stable for ≈ {vpt} steps  (= {vpt * DT:.2f} s  = {vpt * DT * FREQ_GAIT:.1f} gait cycles)")

    # ── Figure 1: 8-channel output ────────────────────────────────────────────
    joint_labels = ["FL_hip", "FL_knee", "FR_hip", "FR_knee",
                    "RL_hip", "RL_knee", "RR_hip", "RR_knee"]
    colors_ch = plt.cm.tab10(np.arange(8))

    t_plot = np.arange(T_TEST) * DT

    fig, axes = plt.subplots(8, 1, figsize=(14, 14), sharex=True)
    for ci, (label, ax, color) in enumerate(zip(joint_labels, axes, colors_ch)):
        ax.plot(t_plot, y_test_target[:, ci], color="black", lw=1.5, alpha=0.6, label="Target")
        ax.plot(t_plot, y_auto[:, ci], color=color, lw=1.5, ls="--", label="Autonomous")
        ax.set_ylabel(label, fontsize=8, rotation=0, ha="right", va="center")
        ax.set_ylim(-1.3, 1.3)
        if ci == 0:
            ax.legend(fontsize=8, loc="upper right")

    axes[-1].set_xlabel("Time (s)", fontsize=11)
    fig.suptitle(f"CPG: 8-Channel Trot Gait (Autonomous)  |  NMSE = {auto_nmse:.4f}", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/17_cpg_gait_channels.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/17_cpg_gait_channels.png")

    # ── Figure 2: Foot contact diagram ────────────────────────────────────────
    # Stance = hip flexion > threshold
    # Columns: FL (0), FR (2), RL (4), RR (6)
    hip_channels = [0, 2, 4, 6]
    leg_labels   = ["FL", "FR", "RL", "RR"]

    show_n = min(800, T_TEST)
    t_show = np.arange(show_n) * DT

    fig, ax = plt.subplots(figsize=(13, 3.5))
    for row_i, (ch_i, leg) in enumerate(zip(hip_channels, leg_labels)):
        stance = (y_auto[:show_n, ch_i] > CONTACT_THRESHOLD).astype(float)
        # Offset for visibility
        ax.fill_between(t_show, row_i + 0.1, row_i + 0.9,
                        where=stance.astype(bool), color=f"C{row_i}", alpha=0.75)
        ax.text(-0.03, row_i + 0.5, leg, ha="right", va="center",
                fontsize=11, transform=ax.get_yaxis_transform())

    ax.set_xlim(t_show[0], t_show[-1])
    ax.set_ylim(-0.1, 4.1)
    ax.set_yticks([])
    ax.set_xlabel("Time (s)", fontsize=11)
    ax.set_title(f"Foot Contact Diagram — Trot Gait  (freq={FREQ_GAIT} Hz)", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/17_foot_contact_diagram.png", dpi=150)
    plt.close()
    print("Figure saved: output/17_foot_contact_diagram.png")

    # ── Figure 3: Phase portraits (hip vs knee per leg) ───────────────────────
    fig, axes = plt.subplots(1, 4, figsize=(13, 3.5))
    hip_pairs = [(0, 1), (2, 3), (4, 5), (6, 7)]   # (hip_ch, knee_ch) per leg

    show_cycles = int(5 / (DT * FREQ_GAIT))   # 5 gait cycles

    for ax, (h_ch, k_ch), leg, color in zip(axes, hip_pairs, leg_labels, colors_ch):
        hip_auto  = y_auto[:show_cycles, h_ch]
        knee_auto = y_auto[:show_cycles, k_ch]
        c_idx = np.linspace(0, 1, show_cycles)
        sc = ax.scatter(hip_auto, knee_auto, c=c_idx, cmap="viridis", s=4, alpha=0.8)
        ax.set_xlabel("Hip angle", fontsize=9)
        ax.set_ylabel("Knee angle", fontsize=9)
        ax.set_title(f"{leg} leg", fontsize=10)
        ax.set_aspect("equal", "datalim")

    fig.suptitle("Phase Portraits: Hip vs Knee (Limit Cycle, 5 Cycles)", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/17_phase_portraits.png", dpi=150)
    plt.close()
    print("Figure saved: output/17_phase_portraits.png")

    # ── Figure 4: Perturbation recovery ──────────────────────────────────────
    print("\n" + "=" * 65)
    print("Perturbation Recovery Test")
    print("=" * 65)

    T_perturb_total = 800
    PERTURB_AT      = 200
    y_perturbed = autonomous_run(W_out, W_rec, W_in, x_final,
                                 T_perturb_total,
                                 perturbation_at=PERTURB_AT,
                                 perturbation_std=1.0)

    y_clean = autonomous_run(W_out, W_rec, W_in, x_final, T_perturb_total)
    t_perturb = np.arange(T_perturb_total) * DT

    # Distance from limit cycle (approximated by unperturbed trajectory)
    lc_dist = np.linalg.norm(y_perturbed - y_clean, axis=1)
    recover_thresh = 0.1 * np.std(y_clean)
    recover_steps  = np.where(lc_dist[PERTURB_AT:] < recover_thresh)[0]
    recover_at     = (PERTURB_AT + recover_steps[0]) if len(recover_steps) > 0 else T_perturb_total
    print(f"  Perturbation at t={PERTURB_AT * DT:.2f}s")
    print(f"  Recovery to limit cycle at t={recover_at * DT:.2f}s  "
          f"(after {(recover_at - PERTURB_AT) * DT:.2f}s = "
          f"{(recover_at - PERTURB_AT) * DT * FREQ_GAIT:.1f} cycles)")

    fig, axes = plt.subplots(2, 1, figsize=(12, 7), sharex=True)

    # FL hip: clean vs perturbed
    axes[0].plot(t_perturb, y_clean[:, 0], color="black", lw=1.5, label="Unperturbed")
    axes[0].plot(t_perturb, y_perturbed[:, 0], color="tomato", lw=1.5, ls="--",
                 label="Perturbed")
    axes[0].axvline(PERTURB_AT * DT, color="purple", ls=":", lw=1.5, label="Perturbation")
    axes[0].axvline(recover_at * DT, color="green",  ls=":", lw=1.5,
                    label=f"Recovery at t={recover_at * DT:.2f}s")
    axes[0].set_ylabel("FL hip angle", fontsize=10)
    axes[0].set_title("(a) FL Hip: Perturbed vs Unperturbed", fontsize=10)
    axes[0].legend(fontsize=9)

    axes[1].semilogy(t_perturb, lc_dist + 1e-6, color="steelblue", lw=2)
    axes[1].axhline(recover_thresh, color="tomato", ls="--", lw=1.5,
                    label=f"Recovery threshold = {recover_thresh:.3f}")
    axes[1].axvline(PERTURB_AT * DT, color="purple", ls=":", lw=1.5)
    axes[1].axvline(recover_at * DT, color="green",  ls=":", lw=1.5)
    axes[1].set_ylabel("||perturbation - clean||", fontsize=10)
    axes[1].set_xlabel("Time (s)", fontsize=11)
    axes[1].set_title("(b) Distance from Limit Cycle", fontsize=10)
    axes[1].legend(fontsize=9)

    fig.suptitle("Perturbation Recovery — Basin of Attraction of the Gait Limit Cycle", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/17_perturbation_recovery.png", dpi=150)
    plt.close()
    print("Figure saved: output/17_perturbation_recovery.png")

    # ── Summary ───────────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Summary")
    print("=" * 65)
    print(f"  N={N}, ρ={RHO}, freq={FREQ_GAIT} Hz, T_train={T_TRAIN}")
    print(f"  Training NMSE:    {train_nmse:.4f}")
    print(f"  Autonomous NMSE:  {auto_nmse:.4f}")
    print(f"  Stable for:       {vpt} steps = {vpt * DT:.2f}s = {vpt * DT * FREQ_GAIT:.1f} gait cycles")
    print(f"  Recovery time:    {(recover_at - PERTURB_AT) * DT:.2f}s after perturbation")
    print(
        "\nKey insight: The supra-critical reservoir (ρ > 1) has intrinsic rhythmic\n"
        "dynamics. FORCE sculpts these into the target gait. The result is a\n"
        "genuine limit cycle with a basin of attraction — the hallmark of a CPG.\n"
        "Small perturbations decay and the gait resumes, exactly as in biological CPGs."
    )


if __name__ == "__main__":
    main()
