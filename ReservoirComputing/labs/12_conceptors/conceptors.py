"""
================================================================================
Lab 12 — Matrix Conceptors for Multi-Pattern Reservoir Memory
================================================================================

Textbook connection
-------------------
Unit 4, Chapter 12: "Conceptors."
Implements the framework from Jaeger (2014, "Controlling Recurrent Neural
Networks by Conceptors", arXiv:1403.3369) and the tutorial treatment in
Jaeger (2017, "A Conceptor-Based Framework for Managing Multiple Tasks in
Recurrent Networks").

Learning objectives
-------------------
  * Understand conceptors as a soft projection onto the subspace spanned by
    reservoir states during a particular pattern.
  * Compute the conceptor C = R (R + α^{-2} I)^{-1} from the state correlation
    matrix R.
  * Apply conceptors during autonomous recall to lock the reservoir to a
    specific attractor.
  * Explore Boolean operations (NOT, OR) on conceptors.
  * See how the aperture α controls the "width" of the conceptor.

Conceptor definition:
    R_p = (1/T) Σ_t x_t x_t^T    [state correlation during pattern p]
    C_p = R_p (R_p + α^{-2} I)^{-1}

This is equivalent to: C_p = (I + α^{-2} R_p^{-1})^{-1}

Boolean operations:
    NOT(C)  = I − C
    C OR B  = (C^{-} + B^{-} − I)^{-}   where A^{-} = (A + ε I)^{-1}

What to observe
---------------
  * Applying C_p during recall 'softly constrains' the reservoir to produce
    the p-th pattern; other patterns are suppressed.
  * NOT(C_p) describes the complement: everything the reservoir can do
    EXCEPT pattern p.
  * The aperture α controls pattern isolation: small α → very tight
    conceptors (may miss the pattern); large α → loose (admits too much noise).
  * Phase portraits reveal distinct attractor geometry for each frequency.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt
from numpy.random import default_rng

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork


# ── Configuration ─────────────────────────────────────────────────────────────

SEED        = 42
N           = 200          # reservoir neurons
RHO         = 0.9
INPUT_SC    = 1.0
CONN        = 0.1
ALPHA_LEAK  = 1.0

ALPHA_APT   = 10.0         # default aperture
T_DRIVE     = 1000         # steps to drive for conceptor estimation
T_RECALL    = 500          # steps to recall
WASHOUT     = 200

# Four patterns: sinusoids at different frequencies (periods in steps)
PERIODS     = [60, 87, 130, 200]
FREQS       = [1.0 / p for p in PERIODS]
PATTERN_LABELS = [f"{p}-step sine" for p in PERIODS]

APERTURE_SWEEP = [0.1, 1.0, 10.0, 100.0]

COLORS = plt.cm.tab10(np.arange(4))


# ── Reservoir construction ─────────────────────────────────────────────────────

def build_reservoir(seed=SEED):
    rng = default_rng(seed)
    mask  = rng.random((N, N)) < CONN
    W     = rng.standard_normal((N, N)) * mask
    rho_W = np.abs(np.linalg.eigvals(W)).max()
    W_rec = (RHO / rho_W) * W
    W_in  = rng.standard_normal((N, 1)) * INPUT_SC
    return W_rec, W_in


# ── Drive reservoir with teacher forcing ─────────────────────────────────────

def drive_reservoir(pattern, W_rec, W_in, washout=WASHOUT):
    """
    Drive the reservoir with 'pattern' as teacher-forced input.
    Returns state matrix after washout: (T_DRIVE - washout, N).
    """
    T = len(pattern)
    x = np.zeros(N)
    states = []
    for t in range(T):
        u = np.array([pattern[t]])
        pre = W_rec @ x + W_in @ u
        x   = (1.0 - ALPHA_LEAK) * x + ALPHA_LEAK * np.tanh(pre)
        if t >= washout:
            states.append(x.copy())
    return np.array(states)   # (T - washout, N)


# ── Compute conceptor ─────────────────────────────────────────────────────────

def compute_conceptor(states, alpha):
    """
    C = R (R + α^{-2} I)^{-1}
    where R = (1/T) Σ_t x_t x_t^T
    """
    R = (states.T @ states) / len(states)
    I = np.eye(N)
    C = R @ np.linalg.solve(R + (alpha ** -2) * I, I)
    return C, R


# ── Conceptor-constrained reservoir step ─────────────────────────────────────

def recall_with_conceptor(C, W_rec, W_in, x_init, T):
    """
    Autonomous recall: x_t = C @ tanh(W_rec x_{t-1})  (no input)
    Returns (T, N) state sequence.
    """
    x = x_init.copy()
    states = np.zeros((T, N))
    for t in range(T):
        pre = W_rec @ x
        x   = C @ np.tanh(pre)
        states[t] = x
    return states


# ── Boolean conceptor operations ─────────────────────────────────────────────

def not_conceptor(C):
    """NOT(C) = I - C"""
    return np.eye(N) - C


def or_conceptor(C1, C2, eps=1e-4):
    """
    C OR B = (C^{-} + B^{-} - I)^{-}
    where A^{-} = (A + ε I)^{-1}
    Uses a small regularisation ε for near-singular matrices.
    """
    I = np.eye(N)
    C1_inv = np.linalg.solve(C1 + eps * I, I)
    C2_inv = np.linalg.solve(C2 + eps * I, I)
    M = C1_inv + C2_inv - I
    return np.linalg.solve(M + eps * I, I)


# ── Recall quality metric ─────────────────────────────────────────────────────

def pattern_similarity(states, pattern, freq):
    """
    Project the mean state onto the target sinusoid to get a similarity score.
    Uses the mean absolute correlation across neurons.
    """
    T = len(states)
    t_idx = np.arange(T)
    target = np.sin(2 * np.pi * freq * t_idx)
    # Use first PC of states as signal proxy
    u, s, vt = np.linalg.svd(states - states.mean(axis=0), full_matrices=False)
    pc1 = u[:, 0]
    corr = abs(np.corrcoef(pc1, target)[0, 1])
    return corr


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    W_rec, W_in = build_reservoir(seed=SEED)

    # ── Storage: drive reservoir with each pattern, compute conceptors ─────────
    print("=" * 65)
    print("Storage Phase: Computing Conceptors")
    print("=" * 65)

    t_axis = np.arange(T_DRIVE)
    patterns = [np.sin(2 * np.pi * f * t_axis) for f in FREQS]

    conceptors  = []
    correlations = []
    x_inits     = []   # final states for warm-starting recall

    for i, (pat, freq, label) in enumerate(zip(patterns, FREQS, PATTERN_LABELS)):
        states  = drive_reservoir(pat, W_rec, W_in, washout=WASHOUT)
        C, R    = compute_conceptor(states, ALPHA_APT)
        conceptors.append(C)
        x_inits.append(states[-1].copy())

        # Conceptor "strength": mean singular value
        sv = np.linalg.svd(C, compute_uv=False)
        print(f"  Pattern {i+1} ({label}):  mean(SV) = {sv.mean():.3f}  "
              f"  trace(R) = {np.trace(R):.2f}")

    # ── Recall phase ──────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Recall Phase: Conceptor-Constrained Autonomous Run")
    print("=" * 65)

    t_recall_idx = np.arange(T_RECALL)
    recall_states_all = []
    recall_similarities = []

    for i, (C, x0, freq, label) in enumerate(zip(conceptors, x_inits, FREQS, PATTERN_LABELS)):
        states_recall = recall_with_conceptor(C, W_rec, W_in, x0, T_RECALL)
        recall_states_all.append(states_recall)
        sim = pattern_similarity(states_recall, None, freq)
        recall_similarities.append(sim)
        print(f"  Pattern {i+1} ({label}):  PC1 correlation = {sim:.3f}")

    # ── Figure 1: Recalled patterns (PC1 time series) ─────────────────────────
    fig, axes = plt.subplots(2, 2, figsize=(12, 8))
    for ax, i, freq, label, color in zip(axes.ravel(), range(4), FREQS, PATTERN_LABELS, COLORS):
        states = recall_states_all[i]
        # Project to 1D via PC1
        u_, s_, vt_ = np.linalg.svd(states - states.mean(axis=0), full_matrices=False)
        pc1 = u_[:, 0]  # first left singular vector (PC1 direction)
        # Rescale to unit amplitude for comparison
        if pc1.std() > 1e-8:
            pc1 = pc1 / pc1.std()
        target_c = np.sin(2 * np.pi * freq * t_recall_idx)

        ax.plot(t_recall_idx, target_c, color="gray", lw=1.5, ls="--", label="Target")
        ax.plot(t_recall_idx, pc1, color=color, lw=2, label=f"Recalled (corr={recall_similarities[i]:.2f})")
        ax.set_title(f"Pattern {i+1}: {label}", fontsize=10)
        ax.set_xlabel("Recall step", fontsize=9)
        ax.set_ylabel("PC1 (normalised)", fontsize=9)
        ax.legend(fontsize=8)

    fig.suptitle(f"Conceptor Recall — 4 Patterns  (α_aperture={ALPHA_APT})", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/12_conceptor_recall.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/12_conceptor_recall.png")

    # ── Figure 2: Conceptor heatmaps ──────────────────────────────────────────
    fig, axes = plt.subplots(2, 2, figsize=(11, 9))
    for ax, C, label, color in zip(axes.ravel(), conceptors, PATTERN_LABELS, COLORS):
        im = ax.imshow(C, cmap="RdBu", vmin=-0.3, vmax=0.3, aspect="equal")
        ax.set_title(f"Conceptor: {label}", fontsize=10)
        ax.set_xlabel(f"trace = {np.trace(C):.1f}", fontsize=9)
        plt.colorbar(im, ax=ax, shrink=0.7)

    fig.suptitle(f"Conceptor Matrices (N={N}, α={ALPHA_APT})  —  Near-Diagonal Structure", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/12_conceptor_heatmaps.png", dpi=150)
    plt.close()
    print("Figure saved: output/12_conceptor_heatmaps.png")

    # ── Boolean operations ─────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Boolean Operations on Conceptors")
    print("=" * 65)

    C1, C2 = conceptors[0], conceptors[1]
    NOT_C1 = not_conceptor(C1)
    OR_C1_C2 = or_conceptor(C1, C2)

    # NOT: should block pattern 1, allow others
    # OR:  should admit both pattern 1 and pattern 2

    not_sim_p1 = pattern_similarity(recall_with_conceptor(NOT_C1, W_rec, W_in, x_inits[0], T_RECALL),
                                    None, FREQS[0])
    or_sim_p1  = pattern_similarity(recall_with_conceptor(OR_C1_C2, W_rec, W_in, x_inits[0], T_RECALL),
                                    None, FREQS[0])
    or_sim_p2  = pattern_similarity(recall_with_conceptor(OR_C1_C2, W_rec, W_in, x_inits[1], T_RECALL),
                                    None, FREQS[1])

    print(f"  C1 alone:       PC1 corr with pattern 1 = {recall_similarities[0]:.3f}")
    print(f"  NOT(C1):        PC1 corr with pattern 1 = {not_sim_p1:.3f}  (should be lower)")
    print(f"  C1 OR C2:       PC1 corr with pattern 1 = {or_sim_p1:.3f}")
    print(f"  C1 OR C2:       PC1 corr with pattern 2 = {or_sim_p2:.3f}")

    fig, axes = plt.subplots(1, 3, figsize=(13, 4))
    for ax, C, title in zip(axes, [C1, NOT_C1, OR_C1_C2],
                             ["C1 (60-step)", "NOT C1", "C1 OR C2"]):
        im = ax.imshow(C, cmap="RdBu", vmin=-0.5, vmax=0.5, aspect="equal")
        ax.set_title(f"{title}  (tr={np.trace(C):.1f})", fontsize=10)
        plt.colorbar(im, ax=ax, shrink=0.7)

    fig.suptitle("Boolean Conceptor Operations", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/12_conceptor_boolean.png", dpi=150)
    plt.close()
    print("Figure saved: output/12_conceptor_boolean.png")

    # ── Aperture sweep ────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Aperture Sweep")
    print("=" * 65)

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))

    traces = np.zeros((len(APERTURE_SWEEP), 4))   # trace(C_p) per aperture per pattern
    similarities_sweep = np.zeros((len(APERTURE_SWEEP), 4))

    for ai, alpha_s in enumerate(APERTURE_SWEEP):
        for pi, (pat, freq, x0) in enumerate(zip(patterns, FREQS, x_inits)):
            states_drive = drive_reservoir(pat, W_rec, W_in, washout=WASHOUT)
            C_s, _       = compute_conceptor(states_drive, alpha_s)
            traces[ai, pi] = np.trace(C_s)
            states_rec = recall_with_conceptor(C_s, W_rec, W_in, x0, T_RECALL)
            similarities_sweep[ai, pi] = pattern_similarity(states_rec, None, freq)

    ax_t = axes[0]
    ax_s = axes[1]

    for pi, (label, color) in enumerate(zip(PATTERN_LABELS, COLORS)):
        ax_t.semilogx(APERTURE_SWEEP, traces[:, pi], "o-", color=color, lw=2, label=label)
        ax_s.semilogx(APERTURE_SWEEP, similarities_sweep[:, pi], "s--", color=color, lw=2, label=label)

    ax_t.set_xlabel("Aperture α", fontsize=11)
    ax_t.set_ylabel("trace(C)", fontsize=11)
    ax_t.set_title("Conceptor Trace vs Aperture", fontsize=11)
    ax_t.legend(fontsize=8)

    ax_s.set_xlabel("Aperture α", fontsize=11)
    ax_s.set_ylabel("Recall similarity (PC1 corr)", fontsize=11)
    ax_s.set_title("Recall Quality vs Aperture", fontsize=11)
    ax_s.legend(fontsize=8)
    ax_s.set_ylim(0, 1.05)

    plt.tight_layout()
    plt.savefig("output/12_aperture_sweep.png", dpi=150)
    plt.close()
    print("Figure saved: output/12_aperture_sweep.png")

    for ai, alpha_s in enumerate(APERTURE_SWEEP):
        print(f"  α = {alpha_s:6.1f}:  "
              + "  ".join(f"P{pi+1}={similarities_sweep[ai,pi]:.2f}" for pi in range(4)))

    # ── Figure: Phase portraits of recalled states ─────────────────────────────
    fig, axes = plt.subplots(1, 4, figsize=(14, 3.5))
    for ax, i, label, color in zip(axes, range(4), PATTERN_LABELS, COLORS):
        states = recall_states_all[i]
        # Show first 2 reservoir neurons
        ax.scatter(states[:, 0], states[:, 1], s=2, c=color, alpha=0.5)
        ax.set_xlabel("Neuron 0", fontsize=9)
        ax.set_ylabel("Neuron 1", fontsize=9)
        ax.set_title(f"P{i+1}: {label}", fontsize=9)
        ax.set_aspect("equal", "datalim")

    fig.suptitle("Phase Portraits of Recalled Patterns (Neurons 0 vs 1)", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/12_phase_portraits.png", dpi=150)
    plt.close()
    print("Figure saved: output/12_phase_portraits.png")

    print("\n" + "=" * 65)
    print("Summary")
    print("=" * 65)
    print(f"  N={N}, ρ={RHO}, aperture α={ALPHA_APT}")
    for i in range(4):
        print(f"  Pattern {i+1} ({PATTERN_LABELS[i]}): recall similarity = {recall_similarities[i]:.3f}")


if __name__ == "__main__":
    main()
