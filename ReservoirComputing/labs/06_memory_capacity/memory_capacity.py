"""
================================================================================
Lab 06 — Linear Memory Capacity: Measuring and Bounding
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 8: "Memory Capacity."
Memory Capacity (MC) was defined and bounded by Jaeger (2002, "Short term
memory in echo state networks"). The bound MC ≤ N is a fundamental result:
no reservoir of size N can linearly recall more than N independent past inputs.
This lab makes that bound tangible and shows how ρ and reservoir architecture
affect the distribution of memory across delays.

Key result to understand: MC_k measures how well the reservoir can reconstruct
the input from k steps ago using a linear readout. The total MC = Σ_k MC_k ≤ N.
For a *linear* reservoir driven by white noise, MC = N exactly.

Learning objectives
-------------------
  * Measure the per-delay memory profile MC_k for different ρ values and see
    the geometric vs. slow-decay tradeoffs.
  * Verify the Jaeger bound MC ≤ N empirically.
  * Understand that the linear ESN (tanh → identity) saturates MC = N, while
    the nonlinear ESN trades some linear memory for nonlinear computation.
  * Explore the Simple Cycle Reservoir (SCR) architecture as a structured
    alternative that provably concentrates memory at specific delays.
  * Connect total MC to the spectral radius sweep from Lab 05.

What to observe
---------------
  * For small ρ (0.5): MC_k decays rapidly — memory is concentrated at short
    delays, and total MC is well below N.
  * For large ρ (0.99): MC_k decays slowly — memory is spread over many delays,
    and total MC approaches N.
  * The linear reservoir (tanh ≈ identity regime) achieves MC ≈ N exactly.
  * The SCR has a flat(ish) MC_k profile up to delay N, then sharp cutoff —
    uniform memory rather than decaying memory.
================================================================================
"""

import os
import sys
import copy
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, memory_capacity


# ── Configuration ────────────────────────────────────────────────────────────

N_RESERVOIR  = 50        # keep small so MC ≤ 50 is verifiable
RHO_VALUES   = [0.5, 0.7, 0.9, 0.99]
MC_T_TRAIN   = 3000
MC_T_TEST    = 1000
MAX_DELAY    = 100       # probe up to 100 lags
SEED         = 42


# ── Simple Cycle Reservoir (SCR) ─────────────────────────────────────────────

def build_scr_esn(n_reservoir: int,
                   spectral_radius: float,
                   input_scaling: float = 0.1,
                   seed: int = 42) -> EchoStateNetwork:
    """
    Build an ESN whose recurrent weight matrix is the Simple Cycle Reservoir
    (Rodan & Tino 2011, "Minimum Complexity Echo State Network").

    W_rec = ρ * cyclic shift matrix  (i.e., a circulant permutation matrix)
    W_in  = each neuron i receives input scaled by ±input_scaling

    The SCR has the minimum possible number of non-zero recurrent weights
    (exactly N) while still providing a provably rich memory spectrum.

    In the SCR the state update is:
        x_i(t) = tanh(ρ * x_{i-1}(t-1) + w_i * u(t))
    with indices modulo N (x_0 "sees" x_{N-1}).
    """
    esn = EchoStateNetwork(
        n_reservoir=n_reservoir,
        spectral_radius=spectral_radius,
        input_scaling=input_scaling,
        connectivity=1.0 / n_reservoir,   # placeholder — overwritten below
        bias_scaling=0.0,
        ridge_alpha=1e-8,
        washout=min(100, MC_T_TRAIN // 10),
        extend_with_input=False,
        seed=seed,
    )

    # Override W_rec: cyclic shift matrix scaled to ρ
    N = n_reservoir
    W_rec = np.zeros((N, N))
    for i in range(N):
        W_rec[i, (i - 1) % N] = spectral_radius   # x_i ← x_{i-1}
    esn.W_rec = W_rec

    # Override W_in: each neuron gets ±input_scaling (alternating)
    rng = np.random.default_rng(seed)
    signs = rng.choice([-1.0, 1.0], size=(N, 1))
    esn.W_in = signs * input_scaling

    # Bias: zero (cleaner for analysis)
    esn.b = np.zeros(N)
    esn._n_inputs = 1

    return esn


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    print("=" * 60)
    print(f"Memory Capacity Analysis  (N={N_RESERVOIR})")
    print("=" * 60)

    # ─── Part 1: Standard ESN — MC profile for different ρ ───────────────────
    print("\nPart 1: Standard ESN — measuring MC_k for each ρ...")
    mc_profiles = {}
    mc_totals   = {}

    for rho in RHO_VALUES:
        print(f"  ρ = {rho:.2f} ...", end=" ", flush=True)
        esn = EchoStateNetwork(
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
        total_mc, mc_k = memory_capacity(
            esn,
            T_train=MC_T_TRAIN,
            T_test=MC_T_TEST,
            max_delay=MAX_DELAY,
            seed=SEED,
        )
        mc_profiles[rho] = mc_k
        mc_totals[rho]   = total_mc
        bound_ok = "OK" if total_mc <= N_RESERVOIR + 1.0 else "VIOLATED"
        print(f"total MC = {total_mc:.2f}  (≤ N={N_RESERVOIR}? {bound_ok})")

    # Print summary table
    print(f"\n{'ρ':>6}  {'Total MC':>10}  {'Bound MC≤N':>12}  {'Peak at k':>10}")
    print("-" * 44)
    for rho in RHO_VALUES:
        pk = int(np.argmax(mc_profiles[rho])) + 1
        ok = "Yes" if mc_totals[rho] <= N_RESERVOIR + 1 else "No"
        print(f"  {rho:4.2f}  {mc_totals[rho]:>10.3f}  {ok:>12}  {pk:>10}")

    # ─── Part 2: Linear ESN (tanh → identity approximation) ──────────────────
    # With very small input_scaling, the argument of tanh stays near 0 where
    # tanh(x) ≈ x.  The reservoir becomes approximately linear, and MC → N.
    print("\nPart 2: Linear ESN approximation (small input, ρ=0.99)...")
    esn_lin = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=0.99,
        input_scaling=0.1,   # small → tanh(x) ≈ x (linear regime)
        connectivity=0.1,
        bias_scaling=0.0,    # no bias — keep it linear
        ridge_alpha=1e-8,
        washout=min(100, MC_T_TRAIN // 10),
        extend_with_input=False,
        seed=SEED,
    )
    total_lin, mc_k_lin = memory_capacity(
        esn_lin,
        T_train=MC_T_TRAIN,
        T_test=MC_T_TEST,
        max_delay=MAX_DELAY,
        seed=SEED,
    )
    print(f"  Linear ESN total MC = {total_lin:.2f}  (theory: MC = N = {N_RESERVOIR})")
    print(f"  Fraction of N achieved: {total_lin / N_RESERVOIR * 100:.1f}%")

    # ─── Part 3: Simple Cycle Reservoir ──────────────────────────────────────
    print("\nPart 3: Simple Cycle Reservoir (Rodan & Tino 2011)...")
    esn_scr = build_scr_esn(N_RESERVOIR, spectral_radius=0.99,
                              input_scaling=0.1, seed=SEED)
    total_scr, mc_k_scr = memory_capacity(
        esn_scr,
        T_train=MC_T_TRAIN,
        T_test=MC_T_TEST,
        max_delay=MAX_DELAY,
        seed=SEED,
    )
    print(f"  SCR total MC = {total_scr:.2f}  (N={N_RESERVOIR})")
    print(f"  Fraction of N achieved: {total_scr / N_RESERVOIR * 100:.1f}%")
    print(
        "  Note: The SCR concentrates memory more uniformly across delays\n"
        "  compared to a random reservoir with the same ρ."
    )

    # ─── Figure 1: Stem plots of MC_k for each ρ ─────────────────────────────
    fig, axes = plt.subplots(2, 2, figsize=(12, 8))
    k_axis = np.arange(1, MAX_DELAY + 1)
    colors = plt.cm.viridis(np.linspace(0.15, 0.85, len(RHO_VALUES)))

    for ax, rho, color in zip(axes.ravel(), RHO_VALUES, colors):
        mc_k = mc_profiles[rho]
        markerline, stemlines, baseline = ax.stem(
            k_axis, mc_k, linefmt="-", markerfmt=".", basefmt="gray"
        )
        plt.setp(stemlines, color=color, lw=0.8, alpha=0.7)
        plt.setp(markerline, color=color, ms=3)
        ax.set_title(
            f"ρ = {rho}  |  Total MC = {mc_totals[rho]:.2f}  (N={N_RESERVOIR})",
            fontsize=10
        )
        ax.set_xlabel("Delay k", fontsize=9)
        ax.set_ylabel("MC_k", fontsize=9)
        ax.set_xlim(0, MAX_DELAY + 1)
        ax.set_ylim(0, 1.05)

    fig.suptitle(
        "Per-Delay Memory Capacity MC_k\n"
        r"$MC_k = \mathrm{Cov}(y_k,u)^2 / (\mathrm{Var}(u)\,\mathrm{Var}(y_k))$",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/06_mc_per_delay.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/06_mc_per_delay.png")

    # ─── Figure 2: Total MC vs. ρ ─────────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(7, 4.5))
    rho_arr = np.array(RHO_VALUES)
    mc_arr  = np.array([mc_totals[r] for r in RHO_VALUES])

    ax.plot(rho_arr, mc_arr, "o-", color="steelblue", lw=2, ms=8,
            label="Standard ESN")
    ax.scatter([0.99], [total_lin], marker="s", s=80, color="darkorange",
               zorder=5, label=f"Linear ESN (ρ=0.99): MC={total_lin:.1f}")
    ax.scatter([0.99], [total_scr], marker="^", s=80, color="seagreen",
               zorder=5, label=f"SCR (ρ=0.99): MC={total_scr:.1f}")
    ax.axhline(N_RESERVOIR, color="tomato", ls="--", lw=1.5,
               label=f"Jaeger bound: MC ≤ N = {N_RESERVOIR}")
    ax.set_xlabel("Spectral radius ρ", fontsize=11)
    ax.set_ylabel("Total Memory Capacity", fontsize=11)
    ax.set_title("Total Linear Memory Capacity vs. ρ", fontsize=12)
    ax.legend(fontsize=9)
    ax.set_ylim(0, N_RESERVOIR * 1.15)
    plt.tight_layout()
    plt.savefig("output/06_total_mc_vs_rho.png", dpi=150)
    plt.close()
    print("Figure saved: output/06_total_mc_vs_rho.png")

    # ─── Figure 3: Comparison — standard vs. linear vs. SCR ──────────────────
    fig, ax = plt.subplots(figsize=(10, 5))
    # Standard ESN at ρ=0.99
    esn_99 = EchoStateNetwork(
        n_reservoir=N_RESERVOIR, spectral_radius=0.99,
        input_scaling=1.0, connectivity=0.1, bias_scaling=0.1,
        ridge_alpha=1e-8, washout=min(100, MC_T_TRAIN//10),
        extend_with_input=False, seed=SEED,
    )
    _, mc_k_99 = memory_capacity(esn_99, T_train=MC_T_TRAIN, T_test=MC_T_TEST,
                                  max_delay=MAX_DELAY, seed=SEED)

    ax.plot(k_axis, mc_k_99,  color="steelblue",   lw=1.5,
            label=f"Standard ESN (ρ=0.99), MC={mc_totals[0.99]:.1f}")
    ax.plot(k_axis, mc_k_lin, color="darkorange",  lw=1.5,
            label=f"Linear ESN (ρ=0.99),   MC={total_lin:.1f}")
    ax.plot(k_axis, mc_k_scr, color="seagreen",    lw=1.5,
            label=f"SCR (ρ=0.99),           MC={total_scr:.1f}")
    ax.axhline(1.0 / N_RESERVOIR, color="gray", ls=":", lw=1,
               label=f"1/N = {1/N_RESERVOIR:.3f}")
    ax.set_xlabel("Delay k", fontsize=11)
    ax.set_ylabel("MC_k", fontsize=11)
    ax.set_title(
        "Memory Profile Comparison: Standard, Linear, and SCR Reservoirs\n"
        f"(All N={N_RESERVOIR}, ρ≈0.99)",
        fontsize=11
    )
    ax.legend(fontsize=9)
    ax.set_xlim(0, MAX_DELAY + 1)
    ax.set_ylim(0, 1.05)
    plt.tight_layout()
    plt.savefig("output/06_mc_comparison.png", dpi=150)
    plt.close()
    print("Figure saved: output/06_mc_comparison.png")

    # ─── Figure 4: Cumulative MC ──────────────────────────────────────────────
    fig, ax = plt.subplots(figsize=(9, 4.5))
    for rho, color in zip(RHO_VALUES, colors):
        mc_k = mc_profiles[rho]
        ax.plot(k_axis, np.cumsum(mc_k), color=color, lw=2,
                label=f"ρ={rho}  (total={mc_totals[rho]:.1f})")
    ax.plot(k_axis, np.cumsum(mc_k_lin), color="darkorange", lw=2, ls="--",
            label=f"Linear (total={total_lin:.1f})")
    ax.plot(k_axis, np.cumsum(mc_k_scr), color="seagreen",   lw=2, ls="-.",
            label=f"SCR (total={total_scr:.1f})")
    ax.axhline(N_RESERVOIR, color="tomato", ls=":", lw=1.5,
               label=f"Jaeger bound N={N_RESERVOIR}")
    ax.set_xlabel("Maximum delay k", fontsize=11)
    ax.set_ylabel("Cumulative MC", fontsize=11)
    ax.set_title("Cumulative Memory Capacity: MC(k) = Σ_{j=1}^{k} MC_j", fontsize=11)
    ax.legend(fontsize=9, ncol=2)
    ax.set_xlim(0, MAX_DELAY + 1)
    ax.set_ylim(0, N_RESERVOIR * 1.1)
    plt.tight_layout()
    plt.savefig("output/06_cumulative_mc.png", dpi=150)
    plt.close()
    print("Figure saved: output/06_cumulative_mc.png")

    # ─── Final summary ────────────────────────────────────────────────────────
    print("\n" + "="*60)
    print("Summary")
    print("="*60)
    print(f"  Jaeger bound:   MC ≤ N = {N_RESERVOIR}")
    for rho in RHO_VALUES:
        print(f"  Standard ESN ρ={rho}:  MC = {mc_totals[rho]:.2f}")
    print(f"  Linear ESN ρ=0.99:    MC = {total_lin:.2f}"
          f"  ({total_lin/N_RESERVOIR*100:.0f}% of N)")
    print(f"  SCR ρ=0.99:           MC = {total_scr:.2f}"
          f"  ({total_scr/N_RESERVOIR*100:.0f}% of N)")
    print(
        "\n  The linear reservoir achieves the theoretical maximum\n"
        "  MC = N, at the cost of zero nonlinear computation.\n"
        "  The nonlinear ESN and SCR trade some linear memory\n"
        "  for the ability to compute nonlinear functions of the input."
    )


if __name__ == "__main__":
    main()
