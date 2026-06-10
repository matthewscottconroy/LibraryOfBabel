"""
================================================================================
Lab 09b — Effect of Leak Rate on Memory and Temporal Smoothing
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 9: "Choosing Hyperparameters — Leak Rate."
Follows Jaeger et al. (2007, "Optimization and Applications of Echo State
Networks with Leaky Integrator Neurons") and the treatment in Lukoševičius
(2012, "A Practical Guide to Applying Echo State Networks").

Learning objectives
-------------------
  * Understand the leaky integrator as a discrete-time exponential smoother:
        x_t = (1 - α) x_{t-1} + α · tanh(W_rec x_{t-1} + W_in u_t)
  * See the integrator effect: small α creates slow, smoothed dynamics;
    α = 1 is the standard (maximally responsive) ESN.
  * Relate α to the effective time constant τ_eff = 1 / (α(1 - ρ)).
  * Understand α as a "temporal zoom lens": choose α to match the timescale
    of the task.
  * Measure task performance (NARMA-10) as a function of α.

What to observe
---------------
  * Step-response plot: α=1.0 shows a near-instantaneous transition;
    α=0.1 shows a slow exponential approach to the new equilibrium.
  * Measured rise time (10%→90% of step) grows approximately as τ_eff.
  * For NARMA-10 (which has a 10-step memory requirement), intermediate
    α values (≈ 0.5–0.7) tend to be optimal.
  * Very small α causes the reservoir to "slow down" too much and miss the
    fast fluctuations in the NARMA input.
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

SEED        = 42
N_RESERVOIR = 100
RHO         = 0.9
LEAK_RATES  = [0.1, 0.3, 0.5, 0.7, 1.0]
T_STEP      = 200     # step-response simulation length
STEP_AT     = 50      # step occurs at t=50
T_TRAIN     = 3000
T_TEST      = 1000
WASHOUT     = 200

COLORS = plt.cm.plasma(np.linspace(0.1, 0.9, len(LEAK_RATES)))


# ── Step-response simulation ──────────────────────────────────────────────────

def step_response(alpha, seed=SEED):
    """
    Drive an ESN with a step-function input (0 → 1 at t=STEP_AT).
    Returns:
        t_axis  : (T_STEP,) time indices
        state0  : (T_STEP,) trajectory of neuron 0
        u       : (T_STEP,) input signal
    """
    u = np.zeros(T_STEP)
    u[STEP_AT:] = 1.0

    esn = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=RHO,
        input_scaling=0.5,
        leak_rate=alpha,
        bias_scaling=0.0,    # zero bias for clean step analysis
        washout=0,
        extend_with_input=False,
        seed=seed,
    )
    esn._build_reservoir(1)
    states, _ = esn.run(u.reshape(-1, 1))
    return np.arange(T_STEP), states[:, 0], u


def measure_rise_time(state, u):
    """
    Measure the 10%–90% rise time from the step transition at STEP_AT.
    We look at the response after the step and find crossing times.
    """
    # Look only at the post-step section
    post = state[STEP_AT:]
    # Normalise to [0, 1]
    lo = post.min()
    hi = post.max()
    if hi - lo < 1e-9:
        return 0
    normed = (post - lo) / (hi - lo)
    t10 = np.where(normed >= 0.1)[0]
    t90 = np.where(normed >= 0.9)[0]
    if len(t10) == 0 or len(t90) == 0:
        return len(post)
    return int(t90[0]) - int(t10[0])


# ── Effective time constant ────────────────────────────────────────────────────

def tau_eff(alpha, rho=RHO):
    """
    Theoretical effective time constant for a linear leaky ESN:
        τ_eff = 1 / (α (1 - ρ))
    This is the time constant of the dominant mode assuming small activations.
    """
    return 1.0 / (alpha * (1.0 - rho))


# ── NARMA-10 performance ──────────────────────────────────────────────────────

def narma_nmse(alpha):
    """Train and test an ESN on NARMA-10. Returns NMSE."""
    u_all, y_all = narma(T=T_TRAIN + T_TEST, order=10, seed=0)
    u_train, y_train = u_all[:T_TRAIN], y_all[:T_TRAIN]
    u_test,  y_test  = u_all[T_TRAIN:], y_all[T_TRAIN:]

    esn = EchoStateNetwork(
        n_reservoir=N_RESERVOIR,
        spectral_radius=RHO,
        input_scaling=0.5,
        leak_rate=alpha,
        ridge_alpha=1e-6,
        washout=WASHOUT,
        seed=SEED,
    )
    esn.fit(u_train.reshape(-1, 1), y_train)
    y_pred = esn.predict(u_test.reshape(-1, 1))
    return nmse(y_test, y_pred)


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ─── Part 1: Step-response trajectories ───────────────────────────────────
    print("=" * 60)
    print("Part 1: Step-Response Analysis")
    print("=" * 60)

    fig, axes = plt.subplots(1, 2, figsize=(12, 5))

    rise_times = []
    tau_measured = []
    tau_theory   = []

    ax_step = axes[0]
    for alpha, color in zip(LEAK_RATES, COLORS):
        t, state0, u = step_response(alpha)
        ax_step.plot(t, state0, color=color, lw=2, label=f"α = {alpha}")
        rt = measure_rise_time(state0, u)
        rise_times.append(rt)
        tau_measured.append(rt)
        tau_theory.append(tau_eff(alpha))

    # Plot the step input
    ax_step.step(np.arange(T_STEP), u * (ax_step.get_ylim()[1] if ax_step.get_ylim()[1] > 0 else 0.5),
                 where="post", color="black", lw=1, ls="--", alpha=0.4, label="Input u (scaled)")
    ax_step.axvline(STEP_AT, color="gray", ls=":", lw=1)
    ax_step.set_xlabel("Time step", fontsize=11)
    ax_step.set_ylabel("Reservoir state (neuron 0)", fontsize=11)
    ax_step.set_title(f"Step-Response for Different Leak Rates\n(ρ={RHO}, N={N_RESERVOIR})", fontsize=11)
    ax_step.legend(fontsize=9)
    ax_step.set_xlim(0, T_STEP)

    # Right panel: τ_eff comparison
    ax_tau = axes[1]
    alphas_arr = np.array(LEAK_RATES)
    tau_theory_arr = [tau_eff(a) for a in alphas_arr]

    ax_tau.plot(alphas_arr, tau_theory_arr, "o--", color="steelblue", lw=2, ms=8,
                label=r"Theory: $\tau_{eff} = 1/(\alpha(1-\rho))$")
    ax_tau.plot(alphas_arr, rise_times, "s-", color="tomato", lw=2, ms=8,
                label="Measured 10%–90% rise time")
    ax_tau.set_xlabel("Leak rate α", fontsize=11)
    ax_tau.set_ylabel("Time constant (steps)", fontsize=11)
    ax_tau.set_title("Effective Time Constant vs Leak Rate", fontsize=11)
    ax_tau.legend(fontsize=9)
    ax_tau.set_xlim(0, 1.05)

    plt.tight_layout()
    plt.savefig("output/09b_step_response.png", dpi=150)
    plt.close()
    print("Figure saved: output/09b_step_response.png")

    # Print time constant table
    print(f"\n{'α':>5}  {'τ_eff (theory)':>16}  {'Rise time (measured)':>22}")
    print("-" * 50)
    for alpha, tt, rt in zip(LEAK_RATES, tau_theory_arr, rise_times):
        print(f"  {alpha:4.1f}   {tt:15.1f}   {rt:>22d}")

    print(
        "\nInterpretation: τ_eff grows as α decreases. The measured rise time\n"
        "tracks this closely, confirming the integrator model.\n"
        "α is a 'temporal zoom': small α smooths over long timescales,\n"
        "large α resolves fine temporal structure."
    )

    # ─── Part 2: NARMA-10 performance vs α ────────────────────────────────────
    print("\n" + "=" * 60)
    print("Part 2: NARMA-10 Performance vs Leak Rate")
    print("=" * 60)

    nmse_values = []
    for alpha in LEAK_RATES:
        val = narma_nmse(alpha)
        nmse_values.append(val)
        print(f"  α = {alpha:.1f}  →  NMSE = {val:.4f}")

    best_alpha = LEAK_RATES[int(np.argmin(nmse_values))]
    print(f"\nBest leak rate for NARMA-10: α = {best_alpha}")

    fig, ax = plt.subplots(figsize=(7, 4.5))
    ax.plot(LEAK_RATES, nmse_values, "o-", color="steelblue", lw=2, ms=8)
    ax.axvline(best_alpha, color="tomato", ls="--", lw=1.5,
               label=f"Best α = {best_alpha}")
    for alpha, val, color in zip(LEAK_RATES, nmse_values, COLORS):
        ax.annotate(f"{val:.3f}", (alpha, val), textcoords="offset points",
                    xytext=(4, 6), fontsize=8.5)
    ax.set_xlabel("Leak rate α", fontsize=12)
    ax.set_ylabel("NMSE", fontsize=12)
    ax.set_title(f"NARMA-10 Performance vs Leak Rate\n(ρ={RHO}, N={N_RESERVOIR}, λ=1e-6)",
                 fontsize=11)
    ax.legend(fontsize=10)
    ax.set_xlim(0, 1.05)
    ax.set_ylim(0, None)
    plt.tight_layout()
    plt.savefig("output/09b_nmse_vs_leak_rate.png", dpi=150)
    plt.close()
    print("Figure saved: output/09b_nmse_vs_leak_rate.png")

    # ─── Part 3: State trajectories — the "temporal zoom" illustration ─────────
    # Compare fast (α=1) vs slow (α=0.1) response to white-noise input
    rng = np.random.default_rng(SEED)
    u_noise = rng.standard_normal(200) * 0.3

    fig, axes = plt.subplots(3, 1, figsize=(11, 8), sharex=True)
    axes[0].plot(u_noise, color="gray", lw=1)
    axes[0].set_ylabel("Input u", fontsize=10)
    axes[0].set_title("Input Signal and Reservoir Responses for Different α", fontsize=11)

    for idx, alpha in enumerate([0.1, 1.0]):
        esn = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=RHO,
            input_scaling=0.5,
            leak_rate=alpha,
            bias_scaling=0.0,
            washout=0,
            extend_with_input=False,
            seed=SEED,
        )
        esn._build_reservoir(1)
        states, _ = esn.run(u_noise.reshape(-1, 1))
        ax = axes[idx + 1]
        for n_idx in range(5):
            ax.plot(states[:, n_idx], lw=1.2, alpha=0.75)
        ax.set_ylabel(f"States (α={alpha})", fontsize=10)
        label = "fast (tracks input closely)" if alpha == 1.0 else "slow (smoothed, delayed)"
        ax.set_title(f"α = {alpha}: {label}", fontsize=10)

    axes[-1].set_xlabel("Time step", fontsize=11)
    plt.tight_layout()
    plt.savefig("output/09b_temporal_zoom.png", dpi=150)
    plt.close()
    print("Figure saved: output/09b_temporal_zoom.png")

    print(
        "\nConclusion: Leak rate α is the temporal resolution knob.\n"
        "Match α so that the reservoir's time constant τ_eff ≈ task memory length."
    )


if __name__ == "__main__":
    main()
