"""
================================================================================
Lab 08 — Mackey-Glass Time Series Prediction
================================================================================

Textbook connection
-------------------
Unit 4, Chapter 10: "Temporal Prediction and Closed-Loop Operation."
The Mackey-Glass delay differential equation (Mackey & Glass 1977) is a
weakly chaotic system that became the second most common RC benchmark after
NARMA-10. It is used here to introduce:
  (1) 1-step ahead (open-loop / teacher-forced) prediction,
  (2) multistep autonomous (closed-loop) prediction, and
  (3) the Valid Prediction Time (VPT) as a metric for chaotic forecasting.

References: Jaeger & Haas (2004, Science), Lukoševičius (2012).

Learning objectives
-------------------
  * Understand the difference between open-loop (teacher-forced) and
    closed-loop (autonomous / generative) prediction.
  * Measure Valid Prediction Time (VPT) as a quantitative summary of
    prediction quality on a chaotic system.
  * Compare ESN prediction to the naive persistence baseline (y_pred = y_{t-1})
    to establish a lower bound on what counts as "useful" prediction.
  * See that higher τ (more chaotic Mackey-Glass) reduces VPT.
  * Appreciate that closed-loop error grows faster than open-loop error
    because errors feed back as inputs.

What to observe
---------------
  * Open-loop (1-step) NMSE should be very small — the ESN essentially
    learns a smooth interpolation of the signal.
  * The closed-loop prediction diverges from the truth after some time; this
    divergence time is the VPT.
  * Persistence (y_pred = y_true_{t-1}) performs reasonably well for 1-step
    but quickly degrades for closed-loop operation.
  * τ=17 (weakly chaotic) gives longer VPT than τ=30 (more chaotic).
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, mackey_glass, nmse, nrmse, valid_prediction_time


# ── Configuration ────────────────────────────────────────────────────────────

T_TOTAL     = 5000
T_TRAIN     = 3000
T_TEST      = 2000
N_RESERVOIR = 200
RHO         = 0.95
RIDGE_ALPHA = 1e-6
WASHOUT     = 100
VPT_THRESH  = 0.05       # 5% of signal variance → standard choice
SEED        = 42
TAU_VALUES  = [17, 30]   # τ=17: weakly chaotic; τ=30: more chaotic


# ── Helper: closed-loop prediction ───────────────────────────────────────────

def closed_loop_predict(esn: EchoStateNetwork,
                         u_seed: np.ndarray,
                         n_steps: int) -> np.ndarray:
    """
    Run the ESN in autonomous (generative) mode.

    1. Warm up the reservoir by teacher-forcing u_seed (open loop).
    2. Then feed the ESN's own output back as the next input for n_steps.

    Parameters
    ----------
    esn     : trained EchoStateNetwork
    u_seed  : (T_seed, 1) warmup input sequence (teacher-forced)
    n_steps : number of autonomous prediction steps

    Returns
    -------
    y_cl : (n_steps,) autonomous predictions
    """
    # Warmup: collect the state at the end of the seed sequence
    states_seed, _ = esn.run(u_seed)
    x = states_seed[-1].copy()   # final state after warmup

    y_cl = np.empty(n_steps)
    # Current input starts as last value of seed
    u_t = u_seed[-1].copy()      # (1,)

    for t in range(n_steps):
        # Step reservoir
        x = esn._step(x, u_t)
        # Readout
        if esn.extend_with_input:
            feat = np.concatenate([x, u_t])
        else:
            feat = x
        y_t = float(esn.W_out @ feat)
        y_cl[t] = y_t
        # Feed prediction back as next input
        u_t = np.array([y_t])

    return y_cl


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    print("=" * 65)
    print("Mackey-Glass Time Series Prediction")
    print("=" * 65)

    all_results = {}   # keyed by tau

    for tau in TAU_VALUES:
        print(f"\n{'─'*60}")
        print(f"  τ = {tau}  ({'weakly chaotic' if tau == 17 else 'more chaotic'})")
        print(f"{'─'*60}")

        # ─── 1. Generate and normalise data ──────────────────────────────────
        print(f"  Generating Mackey-Glass(τ={tau}), T={T_TOTAL}...")
        mg = mackey_glass(T=T_TOTAL, tau=tau, dt=1.0, burn=1000, seed=SEED)

        # Normalise to zero mean, unit variance — standard practice
        mg_mean = mg.mean()
        mg_std  = mg.std()
        mg_norm = (mg - mg_mean) / mg_std

        # 1-step ahead: input x_t → target x_{t+1}
        u_all    = mg_norm[:-1].reshape(-1, 1)   # inputs  (T-1, 1)
        y_all    = mg_norm[1:]                    # targets (T-1,)

        u_train  = u_all[:T_TRAIN]
        y_train  = y_all[:T_TRAIN]
        u_test   = u_all[T_TRAIN:T_TRAIN + T_TEST]
        y_test   = y_all[T_TRAIN:T_TRAIN + T_TEST]

        print(f"  Signal stats: mean={mg_norm.mean():.4f}, std={mg_norm.std():.4f}")
        print(f"  Train: {T_TRAIN} steps  |  Test: {T_TEST} steps")

        # ─── 2. Build and train ESN ───────────────────────────────────────────
        print(f"  Training ESN (N={N_RESERVOIR}, ρ={RHO})...")
        esn = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=RHO,
            input_scaling=1.0,
            connectivity=0.1,
            bias_scaling=0.1,
            ridge_alpha=RIDGE_ALPHA,
            washout=WASHOUT,
            extend_with_input=True,
            seed=SEED,
        )
        esn.fit(u_train, y_train)

        # ─── 3. Open-loop (1-step ahead) prediction ───────────────────────────
        y_pred_ol = esn.predict(u_test)
        ol_nmse  = nmse(y_test, y_pred_ol)
        ol_nrmse = nrmse(y_test, y_pred_ol)
        print(f"  1-step NMSE  = {ol_nmse:.6f}")
        print(f"  1-step NRMSE = {ol_nrmse:.6f}")

        # Persistence baseline: y_pred_t = y_true_{t-1}
        y_persist = u_test.ravel()   # u_test[t] = y_true[t-1] (shifted by 1)
        persist_nmse = nmse(y_test, y_persist)
        print(f"  Persistence NMSE  = {persist_nmse:.6f}")
        print(f"  ESN vs. persistence improvement: {persist_nmse/ol_nmse:.1f}×")

        # ─── 4. Closed-loop (autonomous) prediction ───────────────────────────
        # Use the training segment as warmup, then predict autonomously
        T_WARMUP = 200
        u_warmup = u_all[T_TRAIN - T_WARMUP:T_TRAIN]   # last T_WARMUP train inputs
        y_cl     = closed_loop_predict(esn, u_warmup, T_TEST)

        cl_nmse = nmse(y_test, y_cl)
        print(f"  Closed-loop NMSE = {cl_nmse:.6f}")

        # Valid Prediction Time
        vpt_ol = valid_prediction_time(y_test, y_pred_ol, threshold=VPT_THRESH)
        vpt_cl = valid_prediction_time(y_test, y_cl,      threshold=VPT_THRESH)
        vpt_ps = valid_prediction_time(y_test, y_persist, threshold=VPT_THRESH)
        print(f"  VPT (open-loop):     {vpt_ol:.1f} steps")
        print(f"  VPT (closed-loop):   {vpt_cl:.1f} steps")
        print(f"  VPT (persistence):   {vpt_ps:.1f} steps")

        all_results[tau] = {
            "mg_norm": mg_norm,
            "u_test": u_test, "y_test": y_test,
            "y_pred_ol": y_pred_ol,
            "y_cl": y_cl,
            "y_persist": y_persist,
            "ol_nmse": ol_nmse, "cl_nmse": cl_nmse, "persist_nmse": persist_nmse,
            "vpt_ol": vpt_ol, "vpt_cl": vpt_cl, "vpt_ps": vpt_ps,
        }

    # ─── Figure 1: Full time series for each τ ────────────────────────────────
    fig, axes = plt.subplots(len(TAU_VALUES), 1, figsize=(12, 7), sharex=False)
    for ax, tau in zip(axes, TAU_VALUES):
        mg_n = all_results[tau]["mg_norm"]
        t_axis = np.arange(len(mg_n))
        ax.plot(t_axis, mg_n, color="steelblue", lw=0.8)
        ax.axvline(T_TRAIN, color="tomato", ls="--", lw=1.5,
                   label="Train / Test split")
        ax.set_xlabel("Time step", fontsize=10)
        ax.set_ylabel("x(t) (normalised)", fontsize=10)
        ax.set_title(f"Mackey-Glass τ={tau}  "
                     f"({'weakly chaotic' if tau == 17 else 'more chaotic'})",
                     fontsize=11)
        ax.legend(fontsize=9)
    fig.suptitle("Mackey-Glass Time Series (normalised to zero mean, unit variance)",
                 fontsize=12)
    plt.tight_layout()
    plt.savefig("output/08_mg_timeseries.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/08_mg_timeseries.png")

    # ─── Figure 2: Open-loop 1-step predictions ───────────────────────────────
    fig, axes = plt.subplots(len(TAU_VALUES), 1, figsize=(12, 8))
    for ax, tau in zip(axes, TAU_VALUES):
        res = all_results[tau]
        t_axis = np.arange(T_TEST)
        SHOW = min(400, T_TEST)

        ax.plot(t_axis[:SHOW], res["y_test"][:SHOW], color="steelblue",
                lw=1.5, label="True x(t)", zorder=2)
        ax.plot(t_axis[:SHOW], res["y_pred_ol"][:SHOW], color="tomato",
                lw=1.2, ls="--",
                label=f"ESN 1-step  (NMSE={res['ol_nmse']:.5f})", zorder=3)
        ax.plot(t_axis[:SHOW], res["y_persist"][:SHOW], color="gray",
                lw=0.8, ls=":",
                label=f"Persistence (NMSE={res['persist_nmse']:.5f})")
        ax.set_xlabel("Test step", fontsize=10)
        ax.set_ylabel("x(t)", fontsize=10)
        ax.set_title(f"τ={tau}: 1-Step Ahead Prediction (Open-Loop)", fontsize=11)
        ax.legend(fontsize=9)

    fig.suptitle(
        f"Open-Loop (Teacher-Forced) 1-Step Prediction\n"
        f"ESN: N={N_RESERVOIR}, ρ={RHO}, λ={RIDGE_ALPHA:.0e}",
        fontsize=12
    )
    plt.tight_layout()
    plt.savefig("output/08_openloop_prediction.png", dpi=150)
    plt.close()
    print("Figure saved: output/08_openloop_prediction.png")

    # ─── Figure 3: Closed-loop vs. open-loop ──────────────────────────────────
    fig, axes = plt.subplots(len(TAU_VALUES), 1, figsize=(12, 8))
    for ax, tau in zip(axes, TAU_VALUES):
        res = all_results[tau]
        t_axis = np.arange(T_TEST)
        SHOW = min(600, T_TEST)

        ax.plot(t_axis[:SHOW], res["y_test"][:SHOW], color="steelblue",
                lw=1.5, label="True x(t)", zorder=2)
        ax.plot(t_axis[:SHOW], res["y_pred_ol"][:SHOW], color="seagreen",
                lw=1.0, ls="--",
                label=f"Open-loop  (VPT={res['vpt_ol']:.0f})", zorder=3)
        ax.plot(t_axis[:SHOW], res["y_cl"][:SHOW], color="tomato",
                lw=1.0, ls="-.",
                label=f"Closed-loop (VPT={res['vpt_cl']:.0f})")
        # Mark VPT for closed-loop
        vpt_step = int(res["vpt_cl"])
        if 0 < vpt_step < SHOW:
            ax.axvline(vpt_step, color="tomato", lw=1.2, ls=":",
                       label=f"CL VPT @ t={vpt_step}")
        ax.set_xlabel("Test step", fontsize=10)
        ax.set_ylabel("x(t)", fontsize=10)
        ax.set_title(f"τ={tau}: Open-Loop vs. Closed-Loop Prediction", fontsize=11)
        ax.legend(fontsize=9)

    fig.suptitle("Open-Loop vs. Closed-Loop (Autonomous) Prediction", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/08_closedloop_vs_openloop.png", dpi=150)
    plt.close()
    print("Figure saved: output/08_closedloop_vs_openloop.png")

    # ─── Figure 4: Error divergence on log scale ──────────────────────────────
    fig, axes = plt.subplots(1, len(TAU_VALUES), figsize=(12, 4.5))
    for ax, tau in zip(axes, TAU_VALUES):
        res = all_results[tau]
        sig_var = np.var(res["y_test"])
        err_ol  = (res["y_test"] - res["y_pred_ol"])**2 / sig_var
        err_cl  = (res["y_test"] - res["y_cl"])**2      / sig_var
        err_ps  = (res["y_test"] - res["y_persist"])**2 / sig_var
        t_axis  = np.arange(T_TEST)

        ax.semilogy(t_axis, err_ol + 1e-8, color="seagreen",  lw=1.2,
                    label=f"Open-loop  (VPT={res['vpt_ol']:.0f})")
        ax.semilogy(t_axis, err_cl + 1e-8, color="tomato",    lw=1.2,
                    label=f"Closed-loop (VPT={res['vpt_cl']:.0f})")
        ax.semilogy(t_axis, err_ps + 1e-8, color="gray",      lw=0.8, ls="--",
                    label=f"Persistence (VPT={res['vpt_ps']:.0f})")
        ax.axhline(VPT_THRESH, color="black", lw=1, ls=":",
                   label=f"VPT threshold = {VPT_THRESH}")
        ax.set_xlabel("Test step", fontsize=10)
        ax.set_ylabel("Normalised squared error", fontsize=10)
        ax.set_title(f"τ = {tau}", fontsize=11)
        ax.legend(fontsize=8)
        ax.set_xlim(0, min(1000, T_TEST))

    fig.suptitle("Prediction Error Divergence (Log Scale)\n"
                 "VPT = first crossing of the threshold line", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/08_error_divergence.png", dpi=150)
    plt.close()
    print("Figure saved: output/08_error_divergence.png")

    # ─── Figure 5: τ comparison summary ──────────────────────────────────────
    fig, axes = plt.subplots(1, 2, figsize=(10, 4.5))

    # VPT bar chart
    ax = axes[0]
    metrics = ["Open-loop", "Closed-loop", "Persistence"]
    x = np.arange(len(metrics))
    width = 0.35
    tau_colors = {17: "steelblue", 30: "tomato"}
    for i, tau in enumerate(TAU_VALUES):
        res = all_results[tau]
        vpts = [res["vpt_ol"], res["vpt_cl"], res["vpt_ps"]]
        ax.bar(x + (i - 0.5) * width, vpts, width,
               label=f"τ={tau}", color=tau_colors[tau], alpha=0.8)
    ax.set_xticks(x); ax.set_xticklabels(metrics, fontsize=10)
    ax.set_ylabel("Valid Prediction Time (steps)", fontsize=10)
    ax.set_title("VPT Comparison: τ=17 vs. τ=30", fontsize=11)
    ax.legend(fontsize=10)

    # NMSE comparison
    ax = axes[1]
    labels_nmse = ["1-step\n(open-loop)", "Closed-loop", "Persistence"]
    for i, tau in enumerate(TAU_VALUES):
        res = all_results[tau]
        nmse_vals = [res["ol_nmse"], res["cl_nmse"], res["persist_nmse"]]
        ax.bar(x + (i - 0.5) * width, nmse_vals, width,
               label=f"τ={tau}", color=tau_colors[tau], alpha=0.8)
    ax.set_xticks(x); ax.set_xticklabels(labels_nmse, fontsize=10)
    ax.set_ylabel("Test NMSE", fontsize=10)
    ax.set_yscale("log")
    ax.set_title("NMSE Comparison: τ=17 vs. τ=30", fontsize=11)
    ax.legend(fontsize=10)

    plt.tight_layout()
    plt.savefig("output/08_tau_comparison.png", dpi=150)
    plt.close()
    print("Figure saved: output/08_tau_comparison.png")

    # ─── Summary ──────────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Results Summary")
    print("=" * 65)
    print(f"  ESN: N={N_RESERVOIR}, ρ={RHO}, λ={RIDGE_ALPHA:.0e}")
    print(f"  VPT threshold: {VPT_THRESH} (5% of signal variance)")
    print()
    for tau in TAU_VALUES:
        res = all_results[tau]
        print(f"  τ = {tau}:")
        print(f"    1-step NMSE:        {res['ol_nmse']:.6f}")
        print(f"    Closed-loop NMSE:   {res['cl_nmse']:.6f}")
        print(f"    Persistence NMSE:   {res['persist_nmse']:.6f}")
        print(f"    VPT (open-loop):    {res['vpt_ol']:.1f} steps")
        print(f"    VPT (closed-loop):  {res['vpt_cl']:.1f} steps")
        print(f"    VPT (persistence):  {res['vpt_ps']:.1f} steps")
    print(
        "\n  Key takeaway:\n"
        "  Closed-loop VPT is shorter than open-loop VPT because prediction\n"
        "  errors compound when fed back as inputs. The larger τ (more chaotic\n"
        "  Mackey-Glass) gives shorter VPT, consistent with a shorter\n"
        "  predictability horizon (1/λ_max where λ_max grows with τ)."
    )


if __name__ == "__main__":
    main()
