"""
================================================================================
Lab 15 — Delay-Line Physical Reservoir (Appeltant et al. 2011)
================================================================================

Textbook connection
-------------------
Unit 5, Chapter 15: "Physical Reservoir Computing."
Reproduces the optoelectronic delay reservoir from Appeltant et al. (2011,
"Information processing using a single dynamical node as complex system",
Nature Communications 2:468).

Learning objectives
-------------------
  * Understand how a single time-delayed nonlinear node can emulate an N-neuron
    reservoir by multiplexing in time.
  * The 'virtual nodes' correspond to the system state sampled at regular
    intervals within the delay loop.
  * Understand the role of the feedback gain η: insufficient feedback → poor
    memory; excessive feedback → instability.
  * Compare the delay reservoir to a standard ESN on NARMA-10.
  * Explore how the number of virtual nodes N affects performance.

Physical model (Mackey-Glass-like MZM):
    x(t) = sin²(η · x(t-τ) + ε · mask_i · u(t//N) + φ_0)

where:
    τ = N · θ           (total delay = N virtual-node intervals)
    θ                   (inter-node interval, set to 1 here)
    η                   feedback gain
    ε                   input scaling
    mask_i ∈ [-0.5, 0.5] random input mask
    φ_0 = π/4           operating point of the Mach-Zehnder modulator

What to observe
---------------
  * States form a "wave" propagating through the virtual nodes — visible in the
    heatmap as a diagonal pattern.
  * NMSE is minimised near η ≈ 0.8–0.9; beyond η ≈ 1.0 the system saturates.
  * Increasing N improves NMSE up to a saturation point (diminishing returns).
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, narma, nmse


# ── Configuration ─────────────────────────────────────────────────────────────

SEED      = 42
N_DEFAULT = 100       # virtual nodes
ETA_DEFAULT = 0.8     # feedback gain
EPSILON   = 0.4       # input coupling
PHI_0     = np.pi / 4  # MZM operating point

T_TRAIN   = 3000
T_TEST    = 1000
WASHOUT   = 200

# Sweeps
ETA_VALUES = np.linspace(0.0, 1.5, 20)
N_VALUES   = [25, 50, 100, 200, 400]


# ── Delay-line reservoir simulation ───────────────────────────────────────────

def simulate_delay_reservoir(u, N=N_DEFAULT, eta=ETA_DEFAULT, epsilon=EPSILON,
                              phi0=PHI_0, seed=SEED):
    """
    Simulate the single-node delay reservoir.

    Parameters
    ----------
    u   : (T,) input sequence (scalar)
    N   : number of virtual nodes
    eta : feedback gain
    epsilon : input coupling
    phi0 : MZM operating point

    Returns
    -------
    states : (T, N) state matrix (one row per input sample,
                                   one column per virtual node)

    Implementation notes
    --------------------
    We keep a circular buffer of the last N virtual node values.
    Node i at input step t depends on node i-1 at the same step
    (intra-step propagation) and node i itself from the previous input step
    (delayed feedback).

    The exact update is:
        x[t, i] = sin²(η · x[t-1, (i-1) % N] + ε · mask[i] · u[t] + φ_0)

    This is a slight simplification that captures the key physics while
    being efficient to simulate.
    """
    rng = np.random.default_rng(seed)
    mask = rng.uniform(-0.5, 0.5, N)   # static random input mask

    T = len(u)
    states = np.zeros((T, N))
    x_prev = np.zeros(N)               # states from previous input step

    for t in range(T):
        x_curr = np.zeros(N)
        for i in range(N):
            # Delayed feedback: previous state of node (i-1) % N
            feedback = eta * x_prev[(i - 1) % N]
            inp      = epsilon * mask[i] * u[t]
            x_curr[i] = np.sin(feedback + inp + phi0) ** 2
        states[t] = x_curr
        x_prev    = x_curr

    return states


# ── NARMA-10 with delay reservoir ────────────────────────────────────────────

def narma_with_delay(eta=ETA_DEFAULT, N=N_DEFAULT, seed=SEED):
    """Train a ridge readout on the delay reservoir states; return NMSE."""
    from scipy import linalg

    u_all, y_all = narma(T=T_TRAIN + T_TEST, order=10, seed=0)
    u_train, y_train = u_all[:T_TRAIN], y_all[:T_TRAIN]
    u_test,  y_test  = u_all[T_TRAIN:], y_all[T_TRAIN:]

    # Check for stability: skip if eta is likely to explode
    # (sin² is always bounded [0,1] so it never truly explodes, but saturates)
    X_train = simulate_delay_reservoir(u_train, N=N, eta=eta, seed=seed)
    X_test  = simulate_delay_reservoir(u_test,  N=N, eta=eta, seed=seed)

    # Discard washout
    X_tr = X_train[WASHOUT:]
    Y_tr = y_train[WASHOUT:]

    # Ridge regression
    ridge_alpha = 1e-6
    A = X_tr.T @ X_tr + ridge_alpha * np.eye(N)
    B = X_tr.T @ Y_tr
    try:
        w = linalg.solve(A, B, assume_a="pos")
    except Exception:
        return np.inf

    y_pred = X_test @ w
    return nmse(y_test, y_pred)


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Part 1: Visualise virtual-node states as a heatmap ────────────────────
    print("=" * 60)
    print("Part 1: Wave-Like Propagation in Virtual Nodes")
    print("=" * 60)

    u_vis, _ = narma(T=300, order=10, seed=0)
    states_vis = simulate_delay_reservoir(u_vis, N=N_DEFAULT, eta=ETA_DEFAULT, seed=SEED)

    fig, axes = plt.subplots(1, 2, figsize=(13, 5))

    # Heatmap: time × virtual node
    im = axes[0].imshow(states_vis[:150].T, aspect="auto", cmap="viridis", origin="lower")
    axes[0].set_xlabel("Input time step", fontsize=11)
    axes[0].set_ylabel("Virtual node index", fontsize=11)
    axes[0].set_title(f"Delay Reservoir States (η={ETA_DEFAULT}, N={N_DEFAULT})\n"
                      "Wave-like propagation along virtual nodes", fontsize=10)
    fig.colorbar(im, ax=axes[0], label="State value")

    # Line traces for a few nodes
    for node_i, color in zip([0, 10, 25, 50, 75], plt.cm.tab10(np.arange(5))):
        axes[1].plot(states_vis[:80, node_i], color=color, lw=1.5, label=f"Node {node_i}")
    axes[1].set_xlabel("Input time step", fontsize=11)
    axes[1].set_ylabel("State", fontsize=11)
    axes[1].set_title("Selected Virtual Node Traces", fontsize=10)
    axes[1].legend(fontsize=8)

    plt.tight_layout()
    plt.savefig("output/15_delay_reservoir_states.png", dpi=150)
    plt.close()
    print("Figure saved: output/15_delay_reservoir_states.png")

    # ── Part 2: NMSE vs feedback gain η ──────────────────────────────────────
    print("\n" + "=" * 60)
    print("Part 2: NMSE vs Feedback Gain η")
    print("=" * 60)

    nmse_eta = []
    for eta in ETA_VALUES:
        val = narma_with_delay(eta=eta, N=N_DEFAULT, seed=SEED)
        nmse_eta.append(val)
        stable = "stable" if np.isfinite(val) else "inf"
        print(f"  η = {eta:.2f}  →  NMSE = {val:.4f}  [{stable}]")

    # ESN baseline
    print("\nESN baseline (N=100) ...")
    u_all, y_all = narma(T=T_TRAIN + T_TEST, order=10, seed=0)
    esn = EchoStateNetwork(n_reservoir=100, spectral_radius=0.9, input_scaling=0.5,
                           ridge_alpha=1e-6, washout=WASHOUT, seed=SEED)
    esn.fit(u_all[:T_TRAIN].reshape(-1, 1), y_all[:T_TRAIN])
    y_esn = esn.predict(u_all[T_TRAIN:].reshape(-1, 1))
    esn_nmse_val = nmse(y_all[T_TRAIN:], y_esn)
    print(f"  ESN NMSE: {esn_nmse_val:.4f}")

    fig, ax = plt.subplots(figsize=(8, 4.5))
    finite_mask = [np.isfinite(v) for v in nmse_eta]
    eta_finite  = [e for e, m in zip(ETA_VALUES, finite_mask) if m]
    nmse_finite = [v for v, m in zip(nmse_eta, finite_mask) if m]
    ax.plot(eta_finite, nmse_finite, "o-", color="steelblue", lw=2, ms=6, label="Delay reservoir")
    ax.axhline(esn_nmse_val, color="tomato", ls="--", lw=2, label=f"ESN baseline (NMSE={esn_nmse_val:.3f})")
    ax.set_xlabel("Feedback gain η", fontsize=11)
    ax.set_ylabel("NMSE", fontsize=11)
    ax.set_title(f"Delay Reservoir NARMA-10 Performance vs η  (N={N_DEFAULT} virtual nodes)", fontsize=11)
    ax.legend(fontsize=10)
    ax.set_xlim(0, 1.5)
    ax.set_ylim(0, None)
    plt.tight_layout()
    plt.savefig("output/15_nmse_vs_eta.png", dpi=150)
    plt.close()
    print("Figure saved: output/15_nmse_vs_eta.png")

    # ── Part 3: N sweep ───────────────────────────────────────────────────────
    print("\n" + "=" * 60)
    print("Part 3: Virtual Node Count Sweep")
    print("=" * 60)

    nmse_N = []
    for N_v in N_VALUES:
        val = narma_with_delay(eta=ETA_DEFAULT, N=N_v, seed=SEED)
        nmse_N.append(val)
        print(f"  N = {N_v:4d}  →  NMSE = {val:.4f}")

    fig, ax = plt.subplots(figsize=(7, 4.5))
    ax.plot(N_VALUES, nmse_N, "o-", color="steelblue", lw=2, ms=8)
    ax.axhline(esn_nmse_val, color="tomato", ls="--", lw=1.5,
               label=f"ESN N=100 (NMSE={esn_nmse_val:.3f})")
    for N_v, val in zip(N_VALUES, nmse_N):
        ax.annotate(f"{val:.3f}", (N_v, val), textcoords="offset points",
                    xytext=(4, 6), fontsize=8.5)
    ax.set_xlabel("Number of virtual nodes N", fontsize=11)
    ax.set_ylabel("NMSE", fontsize=11)
    ax.set_title(f"Delay Reservoir NARMA-10 vs Virtual Node Count  (η={ETA_DEFAULT})", fontsize=11)
    ax.legend(fontsize=10)
    plt.tight_layout()
    plt.savefig("output/15_nmse_vs_N.png", dpi=150)
    plt.close()
    print("Figure saved: output/15_nmse_vs_N.png")

    # ── Summary ───────────────────────────────────────────────────────────────
    best_eta_idx = int(np.argmin(nmse_eta))
    best_eta_val = ETA_VALUES[best_eta_idx]
    best_nmse_eta = nmse_eta[best_eta_idx]
    best_N_idx  = int(np.argmin(nmse_N))
    best_N_val  = N_VALUES[best_N_idx]
    best_nmse_N = nmse_N[best_N_idx]

    print("\n" + "=" * 60)
    print("Summary")
    print("=" * 60)
    print(f"  Best η: {best_eta_val:.2f}  →  NMSE = {best_nmse_eta:.4f}")
    print(f"  Best N: {best_N_val}    →  NMSE = {best_nmse_N:.4f}")
    print(f"  ESN baseline:         NMSE = {esn_nmse_val:.4f}")
    print(
        "\nKey insight: A single physical node with delay feedback can match\n"
        "the performance of a standard ESN. The virtual nodes emerge from\n"
        "the temporal multiplexing of the delay line. The MZM sin² nonlinearity\n"
        "provides the required nonlinear mixing of input and memory."
    )


if __name__ == "__main__":
    main()
