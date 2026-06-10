"""
================================================================================
Lab 09 — Systematic Hyperparameter Search for ESN Performance
================================================================================

Textbook connection
-------------------
Unit 3, Chapter 9: "Choosing Hyperparameters."
Follows the practical guide in Lukoševičius (2012, "A Practical Guide to
Applying Echo State Networks") and Bergstra & Bengio (2012, "Random Search
for Hyper-Parameter Optimization").

Learning objectives
-------------------
  * Understand how spectral radius ρ, input scaling σ_in, and ridge alpha λ
    jointly determine NARMA-10 performance.
  * Perform a full grid search over a 3-dimensional hyperparameter space and
    identify the best configurations.
  * Compare grid search with random search and understand why random search
    is often just as effective with far fewer evaluations.
  * Read heatmaps: understand which regions of the (ρ, σ_in) plane work well
    and why.

What to observe
---------------
  * There is typically a "sweet spot" near ρ ≈ 0.9 and moderate input scaling.
  * Too small λ → overfitting; too large → underfitting. An intermediate value
    is usually optimal.
  * Random search with 50 evaluations reliably finds a solution within ≈ 10% of
    the grid-search optimum, while covering the space more efficiently.
  * The sorted-NMSE comparison plot shows rapid convergence of random search
    toward the grid-search best.
================================================================================
"""

import os
import sys
from pathlib import Path
from itertools import product

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.colors as mcolors

sys.path.insert(0, str(Path(__file__).parent.parent))
from utils import EchoStateNetwork, narma, nmse


# ── Configuration ─────────────────────────────────────────────────────────────

SEED          = 42
N_RESERVOIR   = 100
LEAK_RATE     = 1.0
T_TRAIN       = 3000
T_TEST        = 1000
WASHOUT       = 200

# Grid axes
SPECTRAL_RADII  = [0.5, 0.7, 0.9, 0.99, 1.1]
INPUT_SCALINGS  = [0.1, 0.5, 1.0, 2.0]
RIDGE_ALPHAS    = [1e-8, 1e-6, 1e-4, 1e-2]

# Random search budget
N_RANDOM = 50


# ── Helper ────────────────────────────────────────────────────────────────────

def evaluate(rho, sigma_in, alpha, u_train, y_train, u_test, y_test):
    """Train and test one ESN configuration. Returns NMSE (or np.inf on error)."""
    try:
        esn = EchoStateNetwork(
            n_reservoir=N_RESERVOIR,
            spectral_radius=rho,
            input_scaling=sigma_in,
            leak_rate=LEAK_RATE,
            ridge_alpha=alpha,
            washout=WASHOUT,
            seed=SEED,
        )
        esn.fit(u_train.reshape(-1, 1), y_train)
        y_pred = esn.predict(u_test.reshape(-1, 1))
        return nmse(y_test, y_pred)
    except Exception:
        return np.inf


# ── Main ──────────────────────────────────────────────────────────────────────

def main():
    os.makedirs("output", exist_ok=True)

    # ── Generate NARMA-10 data ─────────────────────────────────────────────────
    print("Generating NARMA-10 data ...")
    u_all, y_all = narma(T=T_TRAIN + T_TEST, order=10, seed=0)
    u_train, y_train = u_all[:T_TRAIN], y_all[:T_TRAIN]
    u_test,  y_test  = u_all[T_TRAIN:], y_all[T_TRAIN:]

    # ── Grid search ───────────────────────────────────────────────────────────
    print("Running grid search ...")
    print(f"  Grid size: {len(SPECTRAL_RADII)} × {len(INPUT_SCALINGS)} × "
          f"{len(RIDGE_ALPHAS)} = "
          f"{len(SPECTRAL_RADII)*len(INPUT_SCALINGS)*len(RIDGE_ALPHAS)} configs")

    grid_results = []  # list of (rho, sigma_in, alpha, nmse_val)
    for rho, sigma_in, alpha in product(SPECTRAL_RADII, INPUT_SCALINGS, RIDGE_ALPHAS):
        val = evaluate(rho, sigma_in, alpha, u_train, y_train, u_test, y_test)
        grid_results.append((rho, sigma_in, alpha, val))

    # Sort by NMSE
    grid_results.sort(key=lambda r: r[3])

    # Print top-5
    print("\n" + "=" * 65)
    print("Grid Search — Top-5 Configurations by NMSE")
    print("=" * 65)
    print(f"{'Rank':>4}  {'ρ':>5}  {'σ_in':>6}  {'λ':>8}  {'NMSE':>10}")
    print("-" * 65)
    for rank, (rho, sigma_in, alpha, val) in enumerate(grid_results[:5], 1):
        print(f"  {rank:2d}    {rho:5.2f}   {sigma_in:5.2f}  {alpha:8.1e}  {val:10.4f}")

    best_rho, best_sigma, best_alpha, best_nmse_grid = grid_results[0]
    print(f"\nBest config: ρ={best_rho}, σ_in={best_sigma}, λ={best_alpha}")
    print(f"Best NMSE (grid): {best_nmse_grid:.4f}")

    # ── Heatmap: ρ vs σ_in at best ridge_alpha ───────────────────────────────
    # Build 2-D arrays for the heatmap
    # Also find which ridge alpha appears most often in top-20
    top20_alphas = [r[2] for r in grid_results[:20]]
    best_ridge_for_heatmap = best_alpha  # use the overall best

    heatmap_data = np.full((len(SPECTRAL_RADII), len(INPUT_SCALINGS)), np.nan)
    for i, rho in enumerate(SPECTRAL_RADII):
        for j, sigma_in in enumerate(INPUT_SCALINGS):
            val = evaluate(rho, sigma_in, best_ridge_for_heatmap,
                           u_train, y_train, u_test, y_test)
            heatmap_data[i, j] = val

    # Clip infinite values for display
    finite_mask = np.isfinite(heatmap_data)
    display_data = np.where(finite_mask, heatmap_data, np.nanmax(heatmap_data[finite_mask]) * 2)
    # Cap at 1.5 for colour scale readability
    display_data = np.clip(display_data, 0, 1.5)

    fig, ax = plt.subplots(figsize=(7, 5))
    im = ax.imshow(display_data, aspect="auto", origin="lower",
                   cmap="RdYlGn_r", vmin=0, vmax=1.5)
    cbar = fig.colorbar(im, ax=ax)
    cbar.set_label("NMSE (capped at 1.5)", fontsize=10)
    ax.set_xticks(range(len(INPUT_SCALINGS)))
    ax.set_xticklabels([str(s) for s in INPUT_SCALINGS])
    ax.set_yticks(range(len(SPECTRAL_RADII)))
    ax.set_yticklabels([str(r) for r in SPECTRAL_RADII])
    ax.set_xlabel("Input scaling σ_in", fontsize=11)
    ax.set_ylabel("Spectral radius ρ", fontsize=11)
    ax.set_title(f"Grid Search Heatmap (NMSE)\nλ = {best_ridge_for_heatmap:.0e}  |  N={N_RESERVOIR}  |  NARMA-10",
                 fontsize=11)
    # Annotate cells
    for i in range(len(SPECTRAL_RADII)):
        for j in range(len(INPUT_SCALINGS)):
            v = heatmap_data[i, j]
            text = f"{v:.3f}" if np.isfinite(v) and v < 10 else "inf"
            ax.text(j, i, text, ha="center", va="center", fontsize=7.5,
                    color="black" if display_data[i, j] < 0.75 else "white")
    plt.tight_layout()
    plt.savefig("output/09_heatmap_rho_vs_sigma.png", dpi=150)
    plt.close()
    print("\nFigure saved: output/09_heatmap_rho_vs_sigma.png")

    # ── Random search ─────────────────────────────────────────────────────────
    print(f"\nRunning random search ({N_RANDOM} samples) ...")
    rng = np.random.default_rng(SEED)

    # Sample uniformly in log-space for alpha, linearly for rho and sigma_in
    rho_lo, rho_hi       = min(SPECTRAL_RADII), max(SPECTRAL_RADII)
    sigma_lo, sigma_hi   = min(INPUT_SCALINGS), max(INPUT_SCALINGS)
    log_alpha_lo = np.log10(min(RIDGE_ALPHAS))
    log_alpha_hi = np.log10(max(RIDGE_ALPHAS))

    random_results = []
    for _ in range(N_RANDOM):
        rho      = rng.uniform(rho_lo, rho_hi)
        sigma_in = rng.uniform(sigma_lo, sigma_hi)
        alpha    = 10 ** rng.uniform(log_alpha_lo, log_alpha_hi)
        val      = evaluate(rho, sigma_in, alpha, u_train, y_train, u_test, y_test)
        random_results.append((rho, sigma_in, alpha, val))

    random_results.sort(key=lambda r: r[3])
    best_rho_r, best_sigma_r, best_alpha_r, best_nmse_random = random_results[0]

    print(f"Best config (random): ρ={best_rho_r:.3f}, σ_in={best_sigma_r:.3f}, "
          f"λ={best_alpha_r:.2e}")
    print(f"Best NMSE (random):   {best_nmse_random:.4f}")
    print(f"Grid best NMSE:       {best_nmse_grid:.4f}")
    gap = (best_nmse_random - best_nmse_grid) / (best_nmse_grid + 1e-15) * 100
    print(f"Gap vs grid best:     {gap:.1f}%")

    # ── Comparison plot ───────────────────────────────────────────────────────
    # Sort both lists; compare sorted NMSE trajectories (best-of-k)
    grid_nmse_sorted   = sorted([r[3] for r in grid_results if np.isfinite(r[3])])
    random_nmse_sorted = sorted([r[3] for r in random_results if np.isfinite(r[3])])

    # Running minimum (best-so-far as we evaluate more configs)
    grid_best_so_far   = np.minimum.accumulate(grid_nmse_sorted)
    random_best_so_far = np.minimum.accumulate(random_nmse_sorted)

    fig, axes = plt.subplots(1, 2, figsize=(12, 4.5))

    # Left panel: sorted NMSE of all evaluated configs
    ax = axes[0]
    ax.plot(range(1, len(grid_nmse_sorted) + 1), grid_nmse_sorted,
            "o-", ms=4, color="steelblue", label=f"Grid search ({len(grid_nmse_sorted)} configs)")
    ax.plot(range(1, len(random_nmse_sorted) + 1), random_nmse_sorted,
            "s--", ms=4, color="tomato", label=f"Random search ({len(random_nmse_sorted)} samples)")
    ax.axhline(best_nmse_grid, color="steelblue", lw=1, ls=":")
    ax.axhline(best_nmse_random, color="tomato", lw=1, ls=":")
    ax.set_xlabel("Rank", fontsize=11)
    ax.set_ylabel("NMSE", fontsize=11)
    ax.set_title("Sorted NMSE: Grid vs Random Search", fontsize=11)
    ax.legend(fontsize=9)
    ax.set_ylim(0, min(2.0, max(grid_nmse_sorted + random_nmse_sorted) * 1.1))

    # Right panel: best-so-far curves
    ax = axes[1]
    ax.plot(range(1, len(grid_best_so_far) + 1), grid_best_so_far,
            "-", lw=2, color="steelblue", label="Grid best-so-far")
    ax.plot(range(1, len(random_best_so_far) + 1), random_best_so_far,
            "--", lw=2, color="tomato", label="Random best-so-far")
    ax.set_xlabel("Number of evaluations", fontsize=11)
    ax.set_ylabel("Best NMSE found so far", fontsize=11)
    ax.set_title("Convergence: Best-So-Far NMSE", fontsize=11)
    ax.legend(fontsize=9)
    ax.set_ylim(0, None)

    fig.suptitle("Grid Search vs Random Search on NARMA-10  (N=100)", fontsize=12)
    plt.tight_layout()
    plt.savefig("output/09_grid_vs_random_search.png", dpi=150)
    plt.close()
    print("Figure saved: output/09_grid_vs_random_search.png")

    # ── Summary ───────────────────────────────────────────────────────────────
    print("\n" + "=" * 65)
    print("Summary")
    print("=" * 65)
    print(f"  Grid search:   {len(grid_results)} configs evaluated")
    print(f"  Random search: {N_RANDOM} configs evaluated")
    print(f"  Grid best NMSE:   {best_nmse_grid:.4f}")
    print(f"  Random best NMSE: {best_nmse_random:.4f}  (gap: {gap:.1f}%)")
    print(
        "\nKey insight: Random search with 50 samples explores the continuous\n"
        "hyperparameter space more efficiently than a 80-point grid, and\n"
        "reliably finds configurations within a few % of the grid optimum.\n"
        "This is because performance depends on only a few dimensions\n"
        "(Bergstra & Bengio 2012)."
    )


if __name__ == "__main__":
    main()
