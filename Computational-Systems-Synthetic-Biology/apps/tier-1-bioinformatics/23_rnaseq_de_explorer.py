"""
RNA-seq Differential Expression Explorer
=========================================
Tier 1.3 — Bioinformatics: Transcriptomics

Concepts demonstrated:
  - Negative binomial model for count data (overdispersion vs Poisson)
  - Size-factor normalization (median-of-ratios approximation)
  - Log2 fold change and statistical significance are independent axes
  - Multiple testing correction: Benjamini-Hochberg FDR
  - Volcano plot interpretation: significance vs effect size
  - MA plot: mean expression vs log2FC

Usage:
    python 23_rnaseq_de_explorer.py

Controls:
    Sliders: adjust log2FC and -log10(padj) thresholds
    Click on any point in the volcano plot to see count distributions
    Toggle button: switch between Volcano and MA plot views
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import Button, Slider
from scipy.stats import ttest_ind


# ── Synthetic Data Generation ─────────────────────────────────────────────────

def generate_rnaseq_data():
    """Generate a synthetic RNA-seq dataset with known DE genes."""
    rng = np.random.default_rng(42)
    n_genes = 60
    n_reps = 3  # replicates per condition

    gene_names = [f"GENE_A{i+1:02d}" for i in range(n_genes)]

    # True log2 fold changes
    true_lfc = np.zeros(n_genes)
    true_lfc[:8] = rng.uniform(1.5, 3.5, 8)    # up-regulated
    true_lfc[8:16] = rng.uniform(-3.5, -1.5, 8)  # down-regulated
    # remaining 44 are null (0)

    # Base expression drawn from lognormal (mean ~200, sigma=1.5)
    # lognormal with given mean: mu_log = log(mean) - sigma^2/2
    sigma_base = 1.5
    mu_log = np.log(200) - sigma_base**2 / 2
    base_means = rng.lognormal(mean=mu_log, sigma=sigma_base, size=n_genes)

    # NB dispersion parameter
    dispersion = 0.2

    # Generate raw counts: negative_binomial(n, p) with mean=mu, var=mu+mu^2/n
    # NB parameterisation: n=1/dispersion, p=n/(n+mu)
    def draw_nb(mean_vals, disp, n_samples):
        """Draw NB counts for an array of per-gene means."""
        counts = np.zeros((n_genes, n_samples), dtype=np.float64)
        for g in range(n_genes):
            mu = mean_vals[g]
            nb_n = 1.0 / disp
            nb_p = nb_n / (nb_n + mu)
            counts[g, :] = rng.negative_binomial(nb_n, nb_p, size=n_samples).astype(float)
        return counts

    counts_c1 = draw_nb(base_means, dispersion, n_reps)  # condition 1
    means_c2 = base_means * (2.0 ** true_lfc)
    counts_c2 = draw_nb(means_c2, dispersion, n_reps)     # condition 2

    # Raw counts matrix: shape (n_genes, 6)
    raw_counts = np.hstack([counts_c1, counts_c2])

    # ── Size-factor normalization (median-of-ratios approximation) ───────────
    # library sizes
    lib_sizes = raw_counts.sum(axis=0)
    geom_mean_lib = np.exp(np.mean(np.log(lib_sizes + 1)))
    size_factors = lib_sizes / geom_mean_lib
    # avoid division by zero
    size_factors = np.where(size_factors == 0, 1.0, size_factors)
    norm_counts = raw_counts / size_factors[np.newaxis, :]

    # ── Estimated log2FC ─────────────────────────────────────────────────────
    nc1 = norm_counts[:, :n_reps]
    nc2 = norm_counts[:, n_reps:]
    mean_nc1 = nc1.mean(axis=1)
    mean_nc2 = nc2.mean(axis=1)
    log2fc = np.log2((mean_nc2 + 0.5) / (mean_nc1 + 0.5))

    # Mean expression (for MA plot)
    mean_expr = (mean_nc1 + mean_nc2) / 2.0
    log2_mean_expr = np.log2(mean_expr + 1)

    # ── P-values via t-test on log-transformed counts ────────────────────────
    log_nc1 = np.log2(nc1 + 1)
    log_nc2 = np.log2(nc2 + 1)
    pvals = np.array([
        ttest_ind(log_nc1[g], log_nc2[g]).pvalue for g in range(n_genes)
    ])
    # Guard against NaN
    pvals = np.where(np.isnan(pvals), 1.0, pvals)

    # ── Benjamini-Hochberg FDR correction ────────────────────────────────────
    padj = bh_correction(pvals)

    return {
        'gene_names': gene_names,
        'true_lfc': true_lfc,
        'log2fc': log2fc,
        'pvals': pvals,
        'padj': padj,
        'log2_mean_expr': log2_mean_expr,
        'norm_counts': norm_counts,
        'n_reps': n_reps,
        'n_genes': n_genes,
    }


def bh_correction(pvals):
    """Benjamini-Hochberg FDR correction. Returns adjusted p-values."""
    n = len(pvals)
    order = np.argsort(pvals)
    ranked_pvals = pvals[order]
    ranks = np.arange(1, n + 1)
    # BH adjusted: padj[i] = min over j>=i of (pval[j] * n / j)
    adjusted = ranked_pvals * n / ranks
    # Enforce monotonicity from right: cumulative minimum from the right
    for i in range(n - 2, -1, -1):
        adjusted[i] = min(adjusted[i], adjusted[i + 1])
    adjusted = np.minimum(adjusted, 1.0)
    # Put back in original order
    padj = np.empty(n)
    padj[order] = adjusted
    return padj


# ── Style helpers ─────────────────────────────────────────────────────────────

def style_ax(ax):
    """Apply dark theme styling to an axes."""
    ax.set_facecolor('#0f3460')
    ax.tick_params(colors='white', labelsize=8)
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')
    ax.title.set_color('white')
    for sp in ax.spines.values():
        sp.set_edgecolor('#555')


def style_ax_dark(ax):
    """Darker variant axes background."""
    ax.set_facecolor('#16213e')
    ax.tick_params(colors='white', labelsize=8)
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')
    ax.title.set_color('white')
    for sp in ax.spines.values():
        sp.set_edgecolor('#555')


def compute_colors(log2fc, neg_log10_padj, fc_thresh, padj_thresh):
    """Assign colors per gene based on thresholds."""
    colors = []
    for lfc, nlp in zip(log2fc, neg_log10_padj):
        fc_sig = abs(lfc) >= fc_thresh
        p_sig = nlp >= padj_thresh
        if fc_sig and p_sig:
            colors.append('#e94560')   # red: both
        elif p_sig and not fc_sig:
            colors.append('#50fa7b')   # green: padj only
        elif fc_sig and not p_sig:
            colors.append('#f5a623')   # orange: FC only
        else:
            colors.append('#888888')   # grey: neither
    return colors


# ── Main Application ──────────────────────────────────────────────────────────

class RNAseqDEExplorer:
    def __init__(self):
        self.data = generate_rnaseq_data()
        self.neg_log10_padj = -np.log10(self.data['padj'] + 1e-300)

        self.fc_thresh = 1.0
        self.padj_thresh = 1.3   # -log10(0.05)
        self.selected_gene = None
        self._annotation = None

        self._build_layout()
        self._draw_volcano()
        self._draw_ma()
        self._update_info()
        self._hide_counts_panel()

        plt.show()

    # ── Layout ───────────────────────────────────────────────────────────────

    def _build_layout(self):
        self.fig = plt.figure(figsize=(16, 9))
        self.fig.patch.set_facecolor('#1a1a2e')
        self.fig.suptitle('RNA-seq Differential Expression Explorer',
                          color='white', fontsize=14, fontweight='bold')

        # Main grid: left (volcano) | right (controls + counts + MA)
        gs_main = gridspec.GridSpec(
            1, 2,
            left=0.05, right=0.97,
            top=0.90, bottom=0.18,
            width_ratios=[1.05, 1],
            wspace=0.32,
        )

        # Left: volcano
        self.ax_volcano = self.fig.add_subplot(gs_main[0, 0])
        style_ax(self.ax_volcano)

        # Right column: top = counts bar, bottom = MA
        gs_right = gridspec.GridSpecFromSubplotSpec(
            2, 1,
            subplot_spec=gs_main[0, 1],
            hspace=0.45,
            height_ratios=[1, 1],
        )
        self.ax_counts = self.fig.add_subplot(gs_right[0])
        style_ax(self.ax_counts)

        self.ax_ma = self.fig.add_subplot(gs_right[1])
        style_ax(self.ax_ma)

        # Sliders — placed below the volcano plot
        ax_slider_fc = self.fig.add_axes([0.07, 0.10, 0.38, 0.025])
        ax_slider_fc.set_facecolor('#16213e')
        self.slider_fc = Slider(
            ax_slider_fc, '|log₂FC| thresh',
            0.0, 4.0, valinit=self.fc_thresh, color='#e94560',
        )
        self.slider_fc.label.set_color('white')
        self.slider_fc.valtext.set_color('white')
        ax_slider_fc.tick_params(colors='white')

        ax_slider_p = self.fig.add_axes([0.07, 0.055, 0.38, 0.025])
        ax_slider_p.set_facecolor('#16213e')
        self.slider_p = Slider(
            ax_slider_p, '−log₁₀(padj) thresh',
            0.0, 5.0, valinit=self.padj_thresh, color='#8be9fd',
        )
        self.slider_p.label.set_color('white')
        self.slider_p.valtext.set_color('white')
        ax_slider_p.tick_params(colors='white')

        # Info text box (below right panels)
        self.ax_info = self.fig.add_axes([0.55, 0.03, 0.42, 0.12])
        self.ax_info.set_facecolor('#16213e')
        self.ax_info.axis('off')
        for sp in self.ax_info.spines.values():
            sp.set_edgecolor('#555')

        # Wire up callbacks
        self.slider_fc.on_changed(self._on_slider_change)
        self.slider_p.on_changed(self._on_slider_change)
        self.fig.canvas.mpl_connect('button_press_event', self._on_click)

    # ── Volcano Plot ─────────────────────────────────────────────────────────

    def _draw_volcano(self):
        ax = self.ax_volcano
        ax.cla()
        style_ax(ax)

        d = self.data
        colors = compute_colors(d['log2fc'], self.neg_log10_padj,
                                self.fc_thresh, self.padj_thresh)

        ax.scatter(d['log2fc'], self.neg_log10_padj,
                   c=colors, s=38, alpha=0.85, zorder=3,
                   edgecolors='none', linewidths=0)

        # Threshold lines
        ax.axvline(self.fc_thresh,  color='#ffffff', lw=0.8, ls='--', alpha=0.5)
        ax.axvline(-self.fc_thresh, color='#ffffff', lw=0.8, ls='--', alpha=0.5)
        ax.axhline(self.padj_thresh, color='#ffffff', lw=0.8, ls='--', alpha=0.5)

        # Highlight selected gene
        if self.selected_gene is not None:
            g = self.selected_gene
            ax.scatter(d['log2fc'][g], self.neg_log10_padj[g],
                       c='white', s=90, zorder=5,
                       edgecolors='#e94560', linewidths=1.5)
            if self._annotation:
                try:
                    self._annotation.remove()
                except Exception:
                    pass
            self._annotation = ax.annotate(
                d['gene_names'][g],
                xy=(d['log2fc'][g], self.neg_log10_padj[g]),
                xytext=(8, 8), textcoords='offset points',
                color='white', fontsize=7.5,
                bbox=dict(boxstyle='round,pad=0.3', facecolor='#1a1a2e',
                          edgecolor='#e94560', alpha=0.85),
                arrowprops=dict(arrowstyle='->', color='#e94560', lw=0.8),
            )
        else:
            self._annotation = None

        ax.set_xlabel('log₂ Fold Change', color='white')
        ax.set_ylabel('−log₁₀(adjusted p-value)', color='white')
        ax.set_title('Volcano Plot — click a point', color='white', fontsize=10)

        # Legend patches
        from matplotlib.patches import Patch
        legend_elements = [
            Patch(facecolor='#e94560', label='Both significant'),
            Patch(facecolor='#50fa7b', label='padj only'),
            Patch(facecolor='#f5a623', label='|FC| only'),
            Patch(facecolor='#888888', label='Not significant'),
        ]
        ax.legend(handles=legend_elements, loc='upper left',
                  facecolor='#16213e', edgecolor='#555',
                  labelcolor='white', fontsize=7.5)

        self.fig.canvas.draw_idle()

    # ── MA Plot ──────────────────────────────────────────────────────────────

    def _draw_ma(self):
        ax = self.ax_ma
        ax.cla()
        style_ax(ax)

        d = self.data
        colors = compute_colors(d['log2fc'], self.neg_log10_padj,
                                self.fc_thresh, self.padj_thresh)

        ax.scatter(d['log2_mean_expr'], d['log2fc'],
                   c=colors, s=28, alpha=0.85, zorder=3,
                   edgecolors='none', linewidths=0)
        ax.axhline(0, color='white', lw=0.9, ls='--', alpha=0.5)
        ax.axhline(self.fc_thresh,   color='#ffffff', lw=0.6, ls=':', alpha=0.4)
        ax.axhline(-self.fc_thresh,  color='#ffffff', lw=0.6, ls=':', alpha=0.4)

        if self.selected_gene is not None:
            g = self.selected_gene
            ax.scatter(d['log2_mean_expr'][g], d['log2fc'][g],
                       c='white', s=70, zorder=5,
                       edgecolors='#e94560', linewidths=1.5)

        ax.set_xlabel('log₂(mean expression)', color='white')
        ax.set_ylabel('log₂ Fold Change', color='white')
        ax.set_title('MA Plot', color='white', fontsize=10)

        self.fig.canvas.draw_idle()

    # ── Count Bar Chart ───────────────────────────────────────────────────────

    def _draw_counts(self, gene_idx):
        ax = self.ax_counts
        ax.cla()
        style_ax(ax)
        ax.set_visible(True)

        d = self.data
        g = gene_idx
        nr = d['n_reps']
        nc = d['norm_counts']

        c1 = nc[g, :nr]
        c2 = nc[g, nr:]

        x_c1 = np.arange(nr)
        x_c2 = np.arange(nr) + nr + 0.8

        bars1 = ax.bar(x_c1, c1, color='#8be9fd', alpha=0.85, width=0.65,
                       zorder=3, label='Condition 1')
        bars2 = ax.bar(x_c2, c2, color='#f5a623', alpha=0.85, width=0.65,
                       zorder=3, label='Condition 2')

        # Mean ± std error bars
        m1, s1 = c1.mean(), c1.std()
        m2, s2 = c2.mean(), c2.std()
        xm1 = x_c1.mean()
        xm2 = x_c2.mean()
        ax.errorbar(xm1, m1, yerr=s1, fmt='none', ecolor='white',
                    capsize=5, elinewidth=1.5, zorder=5)
        ax.errorbar(xm2, m2, yerr=s2, fmt='none', ecolor='white',
                    capsize=5, elinewidth=1.5, zorder=5)
        # Mean marker
        ax.plot(xm1, m1, 'D', color='white', ms=6, zorder=6)
        ax.plot(xm2, m2, 'D', color='white', ms=6, zorder=6)

        tick_positions = list(x_c1) + list(x_c2)
        tick_labels = [f'C1-{i+1}' for i in range(nr)] + [f'C2-{i+1}' for i in range(nr)]
        ax.set_xticks(tick_positions)
        ax.set_xticklabels(tick_labels, color='white', fontsize=7.5)

        gene = d['gene_names'][g]
        lfc = d['log2fc'][g]
        padj = d['padj'][g]
        ax.set_title(f'{gene}   log₂FC={lfc:.2f}   padj={padj:.3g}',
                     color='white', fontsize=8.5)
        ax.set_ylabel('Normalized counts', color='white')
        ax.legend(facecolor='#16213e', edgecolor='#555',
                  labelcolor='white', fontsize=7.5)

        self.fig.canvas.draw_idle()

    def _hide_counts_panel(self):
        ax = self.ax_counts
        ax.cla()
        style_ax(ax)
        ax.text(0.5, 0.5,
                'Click a gene in the\nVolcano or MA plot\nto view its counts',
                transform=ax.transAxes, ha='center', va='center',
                color='#aaaaaa', fontsize=10, style='italic')
        ax.set_title('Normalized Count Distribution', color='white', fontsize=10)
        ax.axis('off')
        self.fig.canvas.draw_idle()

    # ── Info panel ────────────────────────────────────────────────────────────

    def _update_info(self):
        d = self.data
        n_sig_both = np.sum(
            (np.abs(d['log2fc']) >= self.fc_thresh) &
            (self.neg_log10_padj >= self.padj_thresh)
        )
        n_sig_p_only = np.sum(
            (np.abs(d['log2fc']) < self.fc_thresh) &
            (self.neg_log10_padj >= self.padj_thresh)
        )
        n_sig_fc_only = np.sum(
            (np.abs(d['log2fc']) >= self.fc_thresh) &
            (self.neg_log10_padj < self.padj_thresh)
        )
        n_up = np.sum((d['log2fc'] >= self.fc_thresh) &
                      (self.neg_log10_padj >= self.padj_thresh))
        n_down = np.sum((d['log2fc'] <= -self.fc_thresh) &
                        (self.neg_log10_padj >= self.padj_thresh))

        ax = self.ax_info
        ax.cla()
        ax.set_facecolor('#16213e')
        ax.axis('off')

        lines = (
            f"Thresholds:  |log₂FC| ≥ {self.fc_thresh:.2f}  |  "
            f"−log₁₀(padj) ≥ {self.padj_thresh:.2f}  "
            f"(padj ≤ {10**(-self.padj_thresh):.3g})\n"
            f"Significant (both):  {n_sig_both}   "
            f"Up: {n_up}   Down: {n_down}\n"
            f"padj-only:  {n_sig_p_only}   |FC|-only:  {n_sig_fc_only}"
        )
        if self.selected_gene is not None:
            g = self.selected_gene
            lines += (
                f"\nSelected:  {d['gene_names'][g]}"
                f"   log₂FC={d['log2fc'][g]:.3f}"
                f"   padj={d['padj'][g]:.3g}"
                f"   true_lfc={d['true_lfc'][g]:.2f}"
            )
        ax.text(0.03, 0.55, lines, transform=ax.transAxes,
                va='center', fontsize=8, color='white',
                fontfamily='monospace')

    # ── Callbacks ─────────────────────────────────────────────────────────────

    def _on_slider_change(self, val):
        self.fc_thresh = float(self.slider_fc.val)
        self.padj_thresh = float(self.slider_p.val)
        self._draw_volcano()
        self._draw_ma()
        self._update_info()

    def _on_click(self, event):
        if event.inaxes not in (self.ax_volcano, self.ax_ma):
            return
        if event.xdata is None or event.ydata is None:
            return

        d = self.data
        if event.inaxes == self.ax_volcano:
            xdata = d['log2fc']
            ydata = self.neg_log10_padj
        else:
            xdata = d['log2_mean_expr']
            ydata = d['log2fc']

        # Transform to display coordinates for pixel-distance hit testing
        ax = event.inaxes
        xy_disp = ax.transData.transform(
            np.column_stack([xdata, ydata])
        )
        click_disp = ax.transData.transform([[event.xdata, event.ydata]])[0]
        dists = np.hypot(xy_disp[:, 0] - click_disp[0],
                         xy_disp[:, 1] - click_disp[1])
        nearest = int(np.argmin(dists))
        if dists[nearest] < 15:
            self.selected_gene = nearest
        else:
            self.selected_gene = None

        if self.selected_gene is not None:
            self._draw_counts(self.selected_gene)
        else:
            self._hide_counts_panel()

        self._draw_volcano()
        self._draw_ma()
        self._update_info()


# ── Entry Point ───────────────────────────────────────────────────────────────

if __name__ == '__main__':
    print("RNA-seq Differential Expression Explorer")
    print("=" * 45)
    print("Synthetic dataset: 60 genes (8 up, 8 down, 44 null)")
    print("3 biological replicates per condition.")
    print()
    print("Controls:")
    print("  |log2FC| slider   — adjust fold-change significance threshold")
    print("  -log10(padj)      — adjust adjusted p-value threshold")
    print("  Click volcano/MA  — inspect individual gene counts")
    print()
    print("Color coding:")
    print("  Red    — significant by BOTH FC and padj (true hits)")
    print("  Green  — significant by padj only")
    print("  Orange — significant by |FC| only")
    print("  Grey   — not significant")
    print()
    RNAseqDEExplorer()
