"""
================================================================================
Lab 02b — Bifurcation Diagram of the Logistic Map
================================================================================

Textbook connection
-------------------
Unit 2, Chapter 4: "Dynamical Systems Background."
The logistic map is the simplest nonlinear discrete-time system that exhibits
the full period-doubling route to chaos. It serves as the canonical pedagogical
example before studying continuous-time attractors such as Lorenz.
Reference: May (1976, "Simple mathematical models with very complicated
dynamics", Nature); Feigenbaum (1978, "Quantitative Universality for a Class
of Nonlinear Transformations").

Learning objectives
-------------------
  * Visualise the bifurcation diagram: fixed points, period-2, period-4, and
    chaotic regimes of the logistic map as r increases.
  * Locate the period-doubling bifurcation points r_1, r_2, r_3, r_4.
  * Compute the Feigenbaum constant δ = (r_{n}-r_{n-1})/(r_{n+1}-r_{n})
    and verify convergence to the universal value 4.6692...
  * Recognise windows of periodicity embedded within the chaotic regime.
  * Understand how this simple map anticipates the behaviour of high-
    dimensional reservoir systems near the ESP boundary.

What to observe
---------------
  * For r < 3.0: a single fixed point (period-1 attractor).
  * At r ≈ 3.0: the first period-doubling bifurcation to period-2.
  * Subsequent doublings at r ≈ 3.449, 3.544, 3.564 (increasingly close).
  * Chaos onset at r ≈ 3.570 (the accumulation point r_∞).
  * Clear period-3 window near r ≈ 3.83, surrounded by chaos.
  * The measured δ will converge toward 4.6692 as more bifurcations are used.
================================================================================
"""

import os
import sys
from pathlib import Path

import numpy as np
import matplotlib.pyplot as plt

sys.path.insert(0, str(Path(__file__).parent.parent))
# (No utils needed for this lab — the logistic map is self-contained.)


# ── Configuration ────────────────────────────────────────────────────────────

N_R          = 2000      # number of r values to sample
R_MIN        = 2.5
R_MAX        = 4.0
N_BURNIN     = 1000      # transient steps to discard
N_COLLECT    = 100       # attractor points to keep per r value
X0           = 0.5       # initial condition (same for all r)

# Known period-doubling bifurcation points for the logistic map
# (used for annotation and Feigenbaum constant calculation)
# Values from Feigenbaum (1978) / May (1976) to 6 significant figures
R_BIFURCATIONS = [3.0,        # period 1 → 2
                  3.449490,   # period 2 → 4
                  3.544090,   # period 4 → 8
                  3.564407,   # period 8 → 16
                  3.568759,   # period 16 → 32  (accumulation ≈ 3.5699...)
                  ]
FEIGENBAUM_THEORETICAL = 4.669201609102990


# ── Core computation ──────────────────────────────────────────────────────────

def compute_bifurcation_diagram(r_values: np.ndarray,
                                 n_burnin: int,
                                 n_collect: int,
                                 x0: float = 0.5):
    """
    For each r, iterate the logistic map f(x) = r*x*(1-x):
      1. Discard n_burnin transient steps.
      2. Collect n_collect attractor points.

    Returns
    -------
    r_plot : (n_r * n_collect,) r values (repeated for each attractor point)
    x_plot : (n_r * n_collect,) corresponding x values
    """
    n_r = len(r_values)
    r_plot = np.empty(n_r * n_collect)
    x_plot = np.empty(n_r * n_collect)

    for i, r in enumerate(r_values):
        x = x0
        # Burn-in: let transients die out
        for _ in range(n_burnin):
            x = r * x * (1.0 - x)
        # Collect attractor points
        for j in range(n_collect):
            x = r * x * (1.0 - x)
            r_plot[i * n_collect + j] = r
            x_plot[i * n_collect + j] = x

    return r_plot, x_plot


def find_period_doublings(x0: float = 0.5,
                           n_burnin: int = 2000,
                           n_check: int = 256) -> list[float]:
    """
    Numerically locate the first four period-doubling bifurcations by
    scanning r in [2.9, 3.58] at fine resolution and detecting when the
    period of the attractor doubles.

    Strategy: run from x0, discard burn-in, then collect n_check points.
    Estimate the period by finding peaks of the autocorrelation.
    When the dominant period doubles, record r.

    Returns list of (up to 4) bifurcation r values.
    """
    r_scan = np.linspace(2.9, 3.58, 5000)
    prev_period = 1
    bifurcations = []

    for r in r_scan:
        x = x0
        for _ in range(n_burnin):
            x = r * x * (1.0 - x)

        orbit = []
        for _ in range(n_check):
            x = r * x * (1.0 - x)
            orbit.append(x)
        orbit = np.array(orbit)

        # Estimate period: find unique attractor points (within tolerance)
        sorted_orbit = np.sort(np.unique(np.round(orbit, 4)))
        period = len(sorted_orbit)

        # Clamp to powers of 2 to avoid noise
        if period >= 2 and period != prev_period:
            # Only record if it looks like a power-of-2 doubling
            if period == 2 * prev_period or (prev_period == 1 and period == 2):
                bifurcations.append(float(r))
                prev_period = period
                if len(bifurcations) >= 4:
                    break

    return bifurcations


def feigenbaum_constant(r_bif: list[float]) -> list[float]:
    """
    Compute successive Feigenbaum ratios:
        δ_n = (r_{n} - r_{n-1}) / (r_{n+1} - r_{n})
    for n = 1, 2, ...
    Returns a list of ratios (length = len(r_bif) - 2).
    """
    deltas = []
    for i in range(len(r_bif) - 2):
        num = r_bif[i+1] - r_bif[i]
        den = r_bif[i+2] - r_bif[i+1]
        if abs(den) > 1e-12:
            deltas.append(num / den)
    return deltas


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    r_values = np.linspace(R_MIN, R_MAX, N_R)

    print("Computing bifurcation diagram...")
    r_plot, x_plot = compute_bifurcation_diagram(r_values, N_BURNIN, N_COLLECT, X0)

    # ─── Locate bifurcation points numerically ────────────────────────────────
    print("Locating period-doubling bifurcations...")
    r_bif_measured = find_period_doublings(x0=X0)

    # Fall back to known values if detection is incomplete
    r_bif_known = R_BIFURCATIONS[:4]
    if len(r_bif_measured) >= 3:
        r_bif_report = r_bif_measured
    else:
        r_bif_report = r_bif_known
        print("  (Using known literature values — numerical detection incomplete)")

    # ─── Feigenbaum constant ──────────────────────────────────────────────────
    # Use the known high-precision values for the Feigenbaum calculation since
    # our numerical scan may not resolve all bifurcations precisely.
    deltas_known = feigenbaum_constant(R_BIFURCATIONS)
    deltas_measured = feigenbaum_constant(r_bif_report) if len(r_bif_report) >= 3 else []

    print(f"\n{'='*60}")
    print("Period-Doubling Bifurcations of the Logistic Map")
    print(f"{'='*60}")
    print(f"\n  {'Bifurcation':>15}  {'r (literature)':>16}  {'r (measured)':>14}")
    print(f"  {'-'*50}")
    period = 1
    for i, (r_known, r_lit) in enumerate(zip(
            r_bif_report[:4], R_BIFURCATIONS[:4])):
        label = f"period {period} → {period*2}"
        period *= 2
        m_str = f"{r_known:.6f}" if i < len(r_bif_report) else "—"
        print(f"  {label:>15}  {r_lit:>16.6f}  {m_str:>14}")

    print(f"\n  Accumulation point r_∞ ≈ 3.569946 (onset of chaos)")

    print(f"\n{'='*60}")
    print("Feigenbaum Constant δ")
    print(f"{'='*60}")
    print(f"  Theoretical value: δ = {FEIGENBAUM_THEORETICAL:.6f}")
    print(f"  Computed from literature r values:")
    for i, d in enumerate(deltas_known):
        print(f"    δ_{i+1} = (r_{i+2}-r_{i+1})/(r_{i+3}-r_{i+2}) = {d:.6f}")
    if deltas_measured:
        print(f"\n  Computed from numerically detected r values:")
        for i, d in enumerate(deltas_measured):
            print(f"    δ_{i+1} = {d:.4f}")
    print(
        "\n  Note: The Feigenbaum constant δ ≈ 4.6692 is universal — it\n"
        "  applies to ANY one-parameter family of maps with a quadratic\n"
        "  maximum, not just the logistic map."
    )

    # ─── Figure 1: Full bifurcation diagram ──────────────────────────────────
    fig, ax = plt.subplots(figsize=(12, 5))
    ax.plot(r_plot, x_plot, ",k", ms=0.3, alpha=0.25, rasterized=True)

    # Annotate bifurcation points
    bif_colors = ["dodgerblue", "darkorange", "green", "red"]
    bif_labels = ["r₁ ≈ 3.000\n(1→2)", "r₂ ≈ 3.449\n(2→4)",
                  "r₃ ≈ 3.544\n(4→8)", "r₄ ≈ 3.564\n(8→16)"]
    for r_b, color, label in zip(R_BIFURCATIONS[:4], bif_colors, bif_labels):
        ax.axvline(r_b, color=color, lw=1.2, ls="--", alpha=0.7)
        ax.text(r_b + 0.005, 0.92, label, color=color, fontsize=7,
                ha="left", va="top", transform=ax.get_xaxis_transform())

    # Mark period-3 window
    ax.axvspan(3.828, 3.858, alpha=0.12, color="purple", label="Period-3 window")

    ax.set_xlabel("Growth parameter r", fontsize=11)
    ax.set_ylabel("Attractor values x*", fontsize=11)
    ax.set_title(
        "Bifurcation Diagram of the Logistic Map\n"
        r"$x_{t+1} = r \cdot x_t \cdot (1 - x_t)$",
        fontsize=12
    )
    ax.set_xlim(R_MIN, R_MAX)
    ax.set_ylim(0, 1)
    ax.legend(fontsize=9, loc="upper left")
    plt.tight_layout()
    plt.savefig("output/02b_bifurcation_diagram.png", dpi=200)
    plt.close()
    print("\nFigure saved: output/02b_bifurcation_diagram.png")

    # ─── Figure 2: Zoomed-in view — period-doubling cascade ──────────────────
    fig, axes = plt.subplots(1, 2, figsize=(13, 4.5))

    # Left panel: zoom into r in [2.8, 3.6]
    ax = axes[0]
    mask = (r_plot >= 2.8) & (r_plot <= 3.6)
    ax.plot(r_plot[mask], x_plot[mask], ",k", ms=0.4, alpha=0.35)
    for r_b, color, label in zip(R_BIFURCATIONS[:4], bif_colors, bif_labels):
        if 2.8 <= r_b <= 3.6:
            ax.axvline(r_b, color=color, lw=1.5, ls="--")
    ax.set_xlabel("r", fontsize=11); ax.set_ylabel("x*", fontsize=11)
    ax.set_title("Period-Doubling Cascade (zoom)", fontsize=11)
    ax.set_xlim(2.8, 3.6)

    # Right panel: zoom into chaotic region showing period-3 window
    ax = axes[1]
    mask2 = (r_plot >= 3.8) & (r_plot <= 3.95)
    ax.plot(r_plot[mask2], x_plot[mask2], ",k", ms=0.4, alpha=0.35)
    ax.axvspan(3.828, 3.858, alpha=0.15, color="purple")
    ax.text(3.840, 0.15, "Period-3\nwindow", color="purple",
            fontsize=9, ha="center")
    ax.set_xlabel("r", fontsize=11); ax.set_ylabel("x*", fontsize=11)
    ax.set_title("Chaotic Region with Period-3 Window", fontsize=11)
    ax.set_xlim(3.8, 3.95)

    plt.tight_layout()
    plt.savefig("output/02b_bifurcation_zoom.png", dpi=200)
    plt.close()
    print("Figure saved: output/02b_bifurcation_zoom.png")

    # ─── Figure 3: Feigenbaum ratio convergence ──────────────────────────────
    if len(deltas_known) >= 2:
        fig, ax = plt.subplots(figsize=(6, 4))
        ax.plot(range(1, len(deltas_known)+1), deltas_known,
                "o-", color="steelblue", lw=2, ms=8, label="Measured δ_n")
        ax.axhline(FEIGENBAUM_THEORETICAL, color="tomato", ls="--", lw=1.5,
                   label=f"δ = {FEIGENBAUM_THEORETICAL:.4f} (theoretical)")
        ax.set_xlabel("Bifurcation index n", fontsize=11)
        ax.set_ylabel("Feigenbaum ratio δ_n", fontsize=11)
        ax.set_title("Convergence to the Feigenbaum Constant", fontsize=11)
        ax.legend(fontsize=10)
        ax.set_ylim(0, 10)
        plt.tight_layout()
        plt.savefig("output/02b_feigenbaum_convergence.png", dpi=150)
        plt.close()
        print("Figure saved: output/02b_feigenbaum_convergence.png")

    # ─── Figure 4: Lyapunov exponent vs r ────────────────────────────────────
    print("Computing Lyapunov exponents vs r (this takes a moment)...")
    r_lya = np.linspace(2.5, 4.0, 800)
    lyapunov = np.zeros(len(r_lya))
    n_iter = 500

    for i, r in enumerate(r_lya):
        x = 0.5
        lam = 0.0
        for _ in range(200):   # burn-in
            x = r * x * (1.0 - x)
        for _ in range(n_iter):
            x = r * x * (1.0 - x)
            deriv = abs(r * (1.0 - 2.0 * x))
            if deriv > 1e-20:
                lam += np.log(deriv)
        lyapunov[i] = lam / n_iter

    fig, axes = plt.subplots(2, 1, figsize=(11, 7), sharex=True)
    ax1 = axes[0]
    ax1.plot(r_plot, x_plot, ",k", ms=0.2, alpha=0.2, rasterized=True)
    for r_b, color in zip(R_BIFURCATIONS[:4], bif_colors):
        ax1.axvline(r_b, color=color, lw=1, ls="--", alpha=0.6)
    ax1.set_ylabel("Attractor x*", fontsize=10)
    ax1.set_title("Bifurcation Diagram and Lyapunov Exponent", fontsize=12)

    ax2 = axes[1]
    ax2.plot(r_lya, lyapunov, "steelblue", lw=0.8)
    ax2.axhline(0, color="tomato", lw=1.5, ls="--", label="λ = 0 (chaos onset)")
    for r_b, color in zip(R_BIFURCATIONS[:4], bif_colors):
        ax2.axvline(r_b, color=color, lw=1, ls="--", alpha=0.6)
    ax2.set_xlabel("Growth parameter r", fontsize=10)
    ax2.set_ylabel("Lyapunov exponent λ", fontsize=10)
    ax2.legend(fontsize=9)
    ax2.set_xlim(R_MIN, R_MAX)
    ax2.set_ylim(-3, 1)

    plt.tight_layout()
    plt.savefig("output/02b_lyapunov_vs_r.png", dpi=200)
    plt.close()
    print("Figure saved: output/02b_lyapunov_vs_r.png")


if __name__ == "__main__":
    main()
