# Publication-Quality Figure Design

Consider what happens when a referee opens a paper and encounters Figure 1. In the first thirty seconds, before reading a single word of the caption, they have already formed an impression of the paper's quality. A figure with tiny unlabeled axes, five colors that look identical in grayscale, bars with no error representation, and a title that says "Results" communicates something about the care with which the underlying science was done. A figure that is immediately legible — clean axes, bold informative labels, carefully chosen colors, data distributed efficiently across the panel space — communicates the opposite. This is not superficial: a figure is a scientific argument encoded visually. How well it communicates that argument matters.

Publication-quality figures are not just aesthetically polished versions of exploratory plots — they are **arguments encoded visually**. Every axis label, color choice, panel arrangement, and data-to-ink ratio either clarifies or obscures the scientific claim the figure is intended to support. The difference between a figure that referees accept and one that generates revision comments is often purely the quality of visual communication, independent of the underlying data.

## matplotlib Configuration for Publications

The first step is configuring matplotlib's global settings for publication:

```python
import matplotlib as mpl
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import numpy as np
from pathlib import Path

# ── Publication-quality rcParams ──────────────────────────────────────────
PUBLICATION_STYLE = {
    # Font: match your journal's requirement (Nature/Cell: 6–8pt labels, 8pt text)
    "font.family":        "sans-serif",
    "font.sans-serif":    ["Helvetica", "Arial", "DejaVu Sans"],
    "font.size":          8,
    "axes.labelsize":     8,
    "axes.titlesize":     8,
    "xtick.labelsize":    7,
    "ytick.labelsize":    7,
    "legend.fontsize":    7,

    # Line widths: thin but visible
    "axes.linewidth":     0.8,
    "xtick.major.width":  0.8,
    "ytick.major.width":  0.8,
    "xtick.minor.width":  0.5,
    "ytick.minor.width":  0.5,
    "lines.linewidth":    1.2,

    # Tick marks: pointing inward (standard for publications)
    "xtick.direction":    "in",
    "ytick.direction":    "in",
    "xtick.top":          True,
    "ytick.right":        True,

    # Legend
    "legend.frameon":     False,
    "legend.borderaxespad": 0.5,

    # Figure
    "figure.dpi":         300,
    "savefig.dpi":        300,
    "savefig.bbox":       "tight",
    "savefig.pad_inches": 0.05,

    # Remove top and right spines (cleaner look)
    # Done per-axis; can't set globally here

    # PDF/SVG text as real text, not outlines
    "pdf.fonttype":       42,   # TrueType
    "ps.fonttype":        42,
    "svg.fonttype":       "none",
}

def set_publication_style():
    """Apply publication-quality matplotlib settings."""
    mpl.rcParams.update(PUBLICATION_STYLE)
    print("Publication style applied")

# Journal-specific figure widths (in inches)
JOURNAL_WIDTHS = {
    "nature":     {"single": 3.54, "double": 7.08, "full": 7.08},
    "cell":       {"single": 3.35, "double": 6.85, "full": 6.85},
    "pnas":       {"single": 3.42, "double": 7.08, "full": 7.08},
    "plos":       {"single": 3.27, "double": 6.83, "full": 6.83},
    "elife":      {"single": 4.09, "double": 8.27, "full": 8.27},
    "biorxiv":    {"single": 3.50, "double": 7.50, "full": 7.50},
}
```

The journal width table is not an aesthetic nicety — it is a practical requirement. A figure submitted to Nature at 7.08 inches will be printed at 7.08 inches. If your figure was designed at 10 inches and then scaled down, every label, every tick mark, every data point gets smaller. Text that was 8pt becomes 5.6pt. The figure becomes illegible. Designing figures at the final print width from the beginning means you see exactly what referees and readers will see.

## Colorblind-Safe Color Palettes

~8% of people have color vision deficiency. Using colorblind-safe palettes is both ethical and a journal requirement at Nature, Science, and Cell:

```python
# ── Colorblind-safe palettes ──────────────────────────────────────────────

# Okabe-Ito palette (designed for colorblindness)
OKABE_ITO = {
    "black":        "#000000",
    "orange":       "#E69F00",
    "sky_blue":     "#56B4E9",
    "green":        "#009E73",
    "yellow":       "#F0E442",
    "blue":         "#0072B2",
    "red":          "#D55E00",
    "pink":         "#CC79A7",
}

OKABE_ITO_LIST = list(OKABE_ITO.values())

# Paul Tol's colorblind-friendly schemes
TOLS_BRIGHT = ["#4477AA", "#EE6677", "#228833",
                "#CCBB44", "#66CCEE", "#AA3377", "#BBBBBB"]

# Diverging palette for fold-change / correlation (good for red-blue)
DIVERGING_RB = "RdBu_r"   # Red-white-Blue reversed; colorblind-friendly

# Colormap for continuous expression data
EXPRESSION_CMAP = "viridis"   # perceptually uniform, colorblind-safe


def test_colorblind_safety(colors: list, n_cols=4):
    """
    Visual test: plot color patches to check distinctness.
    Use with daltonize or Coblis online tools for formal checking.
    """
    fig, ax = plt.subplots(1, 1, figsize=(len(colors) * 0.6, 0.8))
    for i, color in enumerate(colors):
        ax.add_patch(mpl.patches.Rectangle((i, 0), 0.9, 1, color=color))
        ax.text(i + 0.45, 0.5, str(i), ha="center", va="center",
                fontsize=9, color="white" if i > len(colors)//2 else "black")
    ax.set_xlim(0, len(colors))
    ax.set_ylim(0, 1)
    ax.axis("off")
    ax.set_title("Colorblind test: are all patches visually distinct?")
    plt.tight_layout()
    return fig
```

## Multi-Panel Figure Construction

```python
def make_figure_with_panels(
    data_dict: dict,
    output_file: str,
    journal: str = "nature",
    panel_height_inches: float = 2.0
):
    """
    Create a publication-ready multi-panel figure.
    Demonstrates:
      - Correct panel labeling (A, B, C...)
      - Consistent axis styles
      - Proper spacing
    """
    set_publication_style()

    fig_width = JOURNAL_WIDTHS[journal]["double"]  # two-column figure
    fig_height = panel_height_inches * 2           # two rows

    fig = plt.figure(figsize=(fig_width, fig_height))

    # GridSpec: 2 rows, 3 columns, unequal widths
    gs = gridspec.GridSpec(
        2, 3,
        figure=fig,
        width_ratios=[1, 1, 0.8],
        hspace=0.45,
        wspace=0.45,
        left=0.10, right=0.97,
        top=0.94, bottom=0.10
    )

    axes = {
        "A": fig.add_subplot(gs[0, 0]),
        "B": fig.add_subplot(gs[0, 1]),
        "C": fig.add_subplot(gs[0, 2]),
        "D": fig.add_subplot(gs[1, 0]),
        "E": fig.add_subplot(gs[1, 1:]),   # spans columns 1–2
    }

    # Panel labels: bold, slightly outside axes
    for label, ax in axes.items():
        ax.text(
            -0.18, 1.08, label,
            transform=ax.transAxes,
            fontsize=10, fontweight="bold",
            va="top", ha="left"
        )

    # ── Panel A: time series ─────────────────────────────────────────────
    ax = axes["A"]
    t = np.linspace(0, 10, 200)
    ax.plot(t, np.sin(t), color=OKABE_ITO["blue"],    lw=1.2, label="LacI")
    ax.plot(t, np.sin(t + 2), color=OKABE_ITO["red"], lw=1.2, label="TetR")
    ax.set_xlabel("Time (h)")
    ax.set_ylabel("Expression (a.u.)")
    ax.set_title("Oscillation dynamics")
    ax.legend(loc="upper right", frameon=False)
    _style_axis(ax)

    # ── Panel B: scatter with regression ────────────────────────────────
    ax = axes["B"]
    rng = np.random.default_rng(42)
    x = rng.normal(0, 1, 50)
    y = 0.8 * x + rng.normal(0, 0.5, 50)
    ax.scatter(x, y, s=12, color=OKABE_ITO["sky_blue"],
               alpha=0.7, linewidths=0, zorder=3)
    m, b = np.polyfit(x, y, 1)
    ax.plot([-3, 3], [m * -3 + b, m * 3 + b],
            color="black", lw=0.8, ls="--", zorder=2)
    ax.set_xlabel("Gene A expression (log2 CPM)")
    ax.set_ylabel("Gene B expression (log2 CPM)")
    ax.set_title("Co-expression")
    _add_stats_text(ax, r"$r = 0.82$, $p < 10^{-12}$")
    _style_axis(ax)

    # ── Panel C: bar chart with error bars ───────────────────────────────
    ax = axes["C"]
    conditions = ["WT", "KO", "Rescue"]
    means = [1.0, 0.3, 0.85]
    errors = [0.05, 0.08, 0.12]
    colors = [OKABE_ITO["blue"], OKABE_ITO["red"], OKABE_ITO["green"]]
    bars = ax.bar(conditions, means, yerr=errors,
                  color=colors, width=0.5,
                  error_kw={"elinewidth": 0.8, "capsize": 2, "capthick": 0.8},
                  zorder=3)
    # Significance brackets
    _significance_bracket(ax, 0, 1, max(means) + max(errors) + 0.1, "***")
    ax.set_ylabel("Normalized activity")
    ax.set_ylim(0, 1.6)
    _style_axis(ax)

    # ── Panels D, E: placeholder for additional data ─────────────────────
    axes["D"].text(0.5, 0.5, "Panel D", transform=axes["D"].transAxes,
                  ha="center", va="center", color="gray")
    axes["E"].text(0.5, 0.5, "Panel E (wide)", transform=axes["E"].transAxes,
                  ha="center", va="center", color="gray")
    for label in ["D", "E"]:
        _style_axis(axes[label])

    # Save in publication formats
    for ext in ["pdf", "svg", "png"]:
        path = output_file.replace(".pdf", f".{ext}")
        fig.savefig(path, dpi=300, bbox_inches="tight")
        print(f"Saved: {path}")

    return fig


def _style_axis(ax):
    """Remove top and right spines; apply consistent styling."""
    ax.spines["top"].set_visible(False)
    ax.spines["right"].set_visible(False)
    ax.spines["left"].set_linewidth(0.8)
    ax.spines["bottom"].set_linewidth(0.8)


def _add_stats_text(ax, text):
    """Add statistics annotation in upper left of axis."""
    ax.text(0.05, 0.95, text, transform=ax.transAxes,
            fontsize=6, va="top", ha="left",
            bbox=dict(boxstyle="round,pad=0.2", facecolor="white",
                      edgecolor="none", alpha=0.8))


def _significance_bracket(ax, x1, x2, y, text):
    """Draw a significance bracket between two bars."""
    h = 0.03
    ax.plot([x1, x1, x2, x2], [y, y + h, y + h, y],
            lw=0.8, color="black")
    ax.text((x1 + x2) / 2, y + h + 0.01, text,
            ha="center", va="bottom", fontsize=8)
```

## Saving Figures for Publications

```python
def save_publication_figure(fig, base_path: str, formats=("pdf", "svg", "png")):
    """
    Save figure in all required formats.
    
    PDF/SVG: for submission (vector, editable)
    PNG 300 DPI: for preprint servers and review
    TIFF 600 DPI: required by some journals (Cell, JBC)
    """
    base = Path(base_path).with_suffix("")
    for fmt in formats:
        path = base.with_suffix(f".{fmt}")
        kwargs = {"bbox_inches": "tight", "pad_inches": 0.05}
        if fmt in ("png", "tiff"):
            kwargs["dpi"] = 600 if fmt == "tiff" else 300
        fig.savefig(path, format=fmt, **kwargs)
        size_kb = path.stat().st_size / 1024
        print(f"  {path.name}: {size_kb:.0f} KB")
```

## The Key Design Principles

1. **Every data point needs a purpose**: do not include information the figure does not argue about
2. **Maximize data-to-ink ratio**: remove unnecessary gridlines, background fills, 3D effects
3. **Use color deliberately**: one categorical dimension per color axis; perceptually uniform colormaps for continuous data; colorblind-safe always
4. **Align text to data**: axis labels describe the variable with units; avoid "Figure shows..." as a title — state the conclusion instead
5. **Consistent scale**: panels comparing the same quantity should share axis limits
6. **Show distributions, not just means**: use box plots, violin plots, or strip plots instead of bar graphs for n < 30

## Why This Matters

Publication-quality figures are not a cosmetic concern — they are the primary mode through which scientific results are communicated and evaluated. A figure that takes 3 minutes to decode (due to overlapping labels, confusing colors, or ambiguous axes) is a figure that slows science. Referees and readers are busy; clarity is a professional obligation. The practical investment is modest: configuring matplotlib once with a project-wide style sheet, using vector formats (PDF/SVG) so figures are resolution-independent, and matching journal column widths so submitted figures do not require editorial reformatting. Python-generated figures have an additional advantage: they are exactly reproducible. A figure is not a photograph of data; it is a claim about data, generated by code that can be audited, modified, and updated when data changes.
