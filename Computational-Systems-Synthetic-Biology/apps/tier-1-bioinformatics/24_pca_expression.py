"""
PCA / Dimensionality Reduction: Gene Expression Clustering
===========================================================
Tier 1.3 — Bioinformatics: Transcriptomics

Concepts demonstrated:
  - Principal component analysis (PCA) of a gene expression matrix
  - Explained variance and scree plot
  - Sample clustering: biological signal vs technical noise (batch effects)
  - Gene loadings: which genes drive each PC
  - PCA does not require labels — unsupervised

Usage:
    python 24_pca_expression.py

Controls:
    Batch Effect slider: add artificial batch effect and watch samples split
    Radio buttons: color points by Condition or Batch
    Click a sample point to see its top contributing genes
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import Slider, RadioButtons


# ── Synthetic Data Generation ─────────────────────────────────────────────────

def generate_expression_data(batch_strength=0.0):
    """
    Generate an 80-gene × 12-sample expression matrix.

    Conditions: 'Ctrl' (4 reps), 'Drug A' (4 reps), 'Drug B' (4 reps)
    Batch:      within each condition, reps 1-2 = batch 0, reps 3-4 = batch 1
    batch_strength: [0, 2] — adds batch_strength to log2 expression of
                    even-indexed genes for batch-1 samples.
    """
    rng = np.random.default_rng(7)
    n_genes = 80
    n_conditions = 3
    n_reps = 4
    n_samples = n_conditions * n_reps   # 12

    condition_names = ['Ctrl', 'Drug A', 'Drug B']
    sample_conditions = []
    sample_batches = []
    sample_labels = []
    for ci, cname in enumerate(condition_names):
        for r in range(n_reps):
            sample_conditions.append(ci)
            batch = 0 if r < 2 else 1
            sample_batches.append(batch)
            sample_labels.append(f"{cname} R{r+1}")

    sample_conditions = np.array(sample_conditions)
    sample_batches = np.array(sample_batches)

    # Base log2 expression per gene: lognormal(0, 1.5)
    base_log2 = rng.lognormal(mean=0, sigma=1.5, size=n_genes)

    # Condition effect genes (fixed, seeded)
    drug_a_genes = rng.choice(n_genes, size=20, replace=False)
    remaining = np.setdiff1d(np.arange(n_genes), drug_a_genes)
    drug_b_genes = rng.choice(remaining, size=20, replace=False)

    # Build expression matrix (genes × samples) in log2 space
    log2_expr = np.zeros((n_genes, n_samples))
    for s in range(n_samples):
        cond = sample_conditions[s]
        log2_expr[:, s] = base_log2.copy()
        if cond == 1:   # Drug A
            log2_expr[drug_a_genes, s] += np.log2(3.0)
        elif cond == 2: # Drug B
            log2_expr[drug_b_genes, s] += np.log2(3.0)
        # Add per-sample noise
        log2_expr[:, s] += rng.lognormal(mean=0, sigma=0.3, size=n_genes)

    # Batch effect: even-indexed genes get +batch_strength for batch-1 samples
    even_genes = np.arange(0, n_genes, 2)
    for s in range(n_samples):
        if sample_batches[s] == 1:
            log2_expr[even_genes, s] += batch_strength

    return {
        'log2_expr': log2_expr,
        'sample_conditions': sample_conditions,
        'sample_batches': sample_batches,
        'sample_labels': sample_labels,
        'condition_names': condition_names,
        'n_genes': n_genes,
        'n_samples': n_samples,
        'drug_a_genes': drug_a_genes,
        'drug_b_genes': drug_b_genes,
        'gene_names': [f'G{i+1:03d}' for i in range(n_genes)],
    }


def run_pca(log2_expr):
    """
    PCA via SVD on the gene × sample matrix.

    Returns:
        scores:      (n_samples, n_samples) — each row is a sample in PC space
        loadings:    (n_pcs, n_genes)       — each row is a PC's gene weights
        var_explained: fraction of variance per PC
    """
    # Center each gene (subtract row mean)
    centered = log2_expr - log2_expr.mean(axis=1, keepdims=True)

    # SVD: centered = U @ diag(S) @ Vt
    # rows of Vt are right singular vectors (sample directions)
    # We want sample scores: project samples onto PCs
    # X has shape (genes, samples); X.T @ X gives sample covariance (up to scale)
    # SVD of X: U(genes×k), S(k,), Vt(k×samples)
    U, S, Vt = np.linalg.svd(centered, full_matrices=False)

    # PC scores for samples: shape (n_samples, n_pcs)
    # scores = centered.T @ U  OR equivalently = Vt.T * S
    scores = Vt.T * S   # (n_samples, n_pcs)

    # Explained variance from singular values
    var_explained = (S ** 2) / np.sum(S ** 2)

    # Gene loadings = U (genes × pcs); transpose so rows = PCs
    loadings = U.T   # (n_pcs, n_genes)

    return scores, loadings, var_explained


# ── Style helpers ─────────────────────────────────────────────────────────────

COND_COLORS = ['#8be9fd', '#f5a623', '#ff79c6']   # Ctrl, Drug A, Drug B
BATCH_COLORS = ['#50fa7b', '#e94560']              # Batch 0, Batch 1


def style_ax(ax, bg='#0f3460'):
    ax.set_facecolor(bg)
    ax.tick_params(colors='white', labelsize=8)
    ax.xaxis.label.set_color('white')
    ax.yaxis.label.set_color('white')
    ax.title.set_color('white')
    for sp in ax.spines.values():
        sp.set_edgecolor('#555')


# ── Main Application ──────────────────────────────────────────────────────────

class PCAExplorer:
    def __init__(self):
        self.batch_strength = 0.0
        self.color_mode = 'Condition'  # or 'Batch'
        self.selected_sample = None

        self._build_layout()
        self._recompute_and_redraw()
        plt.show()

    # ── Layout ───────────────────────────────────────────────────────────────

    def _build_layout(self):
        self.fig = plt.figure(figsize=(16, 9))
        self.fig.patch.set_facecolor('#1a1a2e')
        self.fig.suptitle(
            'PCA: Gene Expression Clustering — Batch Effect Demo',
            color='white', fontsize=14, fontweight='bold'
        )

        # Top area: PCA scatter | scree | loadings
        gs_top = gridspec.GridSpec(
            1, 3,
            left=0.07, right=0.97,
            top=0.88, bottom=0.30,
            wspace=0.38,
            width_ratios=[1.4, 1, 1],
        )

        self.ax_pca = self.fig.add_subplot(gs_top[0])
        style_ax(self.ax_pca)

        self.ax_scree = self.fig.add_subplot(gs_top[1])
        style_ax(self.ax_scree, bg='#16213e')

        self.ax_loadings = self.fig.add_subplot(gs_top[2])
        style_ax(self.ax_loadings, bg='#16213e')

        # Bottom area: slider + radio + sample info
        # Slider
        ax_slider = self.fig.add_axes([0.10, 0.18, 0.40, 0.028])
        ax_slider.set_facecolor('#16213e')
        self.slider_batch = Slider(
            ax_slider, 'Batch Effect',
            0.0, 2.0, valinit=0.0, color='#e94560',
        )
        self.slider_batch.label.set_color('white')
        self.slider_batch.valtext.set_color('white')
        ax_slider.tick_params(colors='white')
        self.slider_batch.on_changed(self._on_slider)

        # Radio buttons
        ax_radio = self.fig.add_axes([0.57, 0.08, 0.14, 0.14])
        ax_radio.set_facecolor('#16213e')
        self.radio = RadioButtons(
            ax_radio,
            ['Color by: Condition', 'Color by: Batch'],
            activecolor='#e94560',
        )
        for lbl in self.radio.labels:
            lbl.set_color('white')
            lbl.set_fontsize(9)
        self.radio.on_clicked(self._on_radio)

        # Info panel (sample details on click)
        self.ax_info = self.fig.add_axes([0.74, 0.06, 0.23, 0.16])
        self.ax_info.set_facecolor('#16213e')
        self.ax_info.axis('off')
        for sp in self.ax_info.spines.values():
            sp.set_edgecolor('#555')

        # Explanation label below slider
        self.fig.text(
            0.10, 0.13,
            'As Batch Effect increases, PC1 separates batches instead of conditions — '
            'showing how technical variance can dominate biological signal.',
            color='#aaaaaa', fontsize=8, wrap=True,
            horizontalalignment='left',
        )

        # Wire click
        self.fig.canvas.mpl_connect('button_press_event', self._on_click)

    # ── Compute & Draw ────────────────────────────────────────────────────────

    def _recompute_and_redraw(self):
        self.data = generate_expression_data(self.batch_strength)
        self.scores, self.loadings, self.var_exp = run_pca(self.data['log2_expr'])
        self._draw_pca()
        self._draw_scree()
        self._draw_loadings()
        self._update_info()

    def _draw_pca(self):
        ax = self.ax_pca
        ax.cla()
        style_ax(ax)

        d = self.data
        s = self.scores
        ve = self.var_exp

        if self.color_mode == 'Condition':
            point_colors = [COND_COLORS[c] for c in d['sample_conditions']]
            # Legend
            from matplotlib.patches import Patch
            legend_elements = [
                Patch(facecolor=COND_COLORS[i], label=cn)
                for i, cn in enumerate(d['condition_names'])
            ]
        else:
            point_colors = [BATCH_COLORS[b] for b in d['sample_batches']]
            from matplotlib.patches import Patch
            legend_elements = [
                Patch(facecolor=BATCH_COLORS[0], label='Batch 0 (reps 1-2)'),
                Patch(facecolor=BATCH_COLORS[1], label='Batch 1 (reps 3-4)'),
            ]

        ax.scatter(s[:, 0], s[:, 1],
                   c=point_colors, s=90,
                   edgecolors='white', linewidths=0.5, zorder=4)

        # Highlight selected
        if self.selected_sample is not None:
            idx = self.selected_sample
            ax.scatter(s[idx, 0], s[idx, 1],
                       c='white', s=130, zorder=6,
                       edgecolors='#e94560', linewidths=2.0)

        # Sample labels
        for i, label in enumerate(d['sample_labels']):
            ax.annotate(
                label,
                xy=(s[i, 0], s[i, 1]),
                xytext=(4, 4), textcoords='offset points',
                color='white', fontsize=6.5, alpha=0.9,
                zorder=5,
            )

        pct0 = ve[0] * 100
        pct1 = ve[1] * 100
        ax.set_xlabel(f'PC1 ({pct0:.1f}%)', color='white')
        ax.set_ylabel(f'PC2 ({pct1:.1f}%)', color='white')
        ax.set_title('PCA Score Plot', color='white', fontsize=10)

        ax.axhline(0, color='#555', lw=0.6, ls='--')
        ax.axvline(0, color='#555', lw=0.6, ls='--')

        ax.legend(handles=legend_elements, loc='best',
                  facecolor='#16213e', edgecolor='#555',
                  labelcolor='white', fontsize=8)

        self.fig.canvas.draw_idle()

    def _draw_scree(self):
        ax = self.ax_scree
        ax.cla()
        style_ax(ax, bg='#16213e')

        n_show = 6
        ve = self.var_exp[:n_show] * 100
        x = np.arange(1, n_show + 1)

        bars = ax.bar(x, ve, color='#8be9fd', alpha=0.85, width=0.6, zorder=3)

        for bar, v in zip(bars, ve):
            ax.text(bar.get_x() + bar.get_width() / 2,
                    bar.get_height() + 0.3,
                    f'{v:.1f}%', ha='center', va='bottom',
                    color='white', fontsize=7)

        ax.set_xticks(x)
        ax.set_xticklabels([f'PC{i}' for i in x], color='white', fontsize=8)
        ax.set_xlabel('Principal Component', color='white')
        ax.set_ylabel('Variance Explained (%)', color='white')
        ax.set_title('Scree Plot', color='white', fontsize=10)
        ax.set_ylim(0, max(ve) * 1.20)

        # Cumulative line
        ax2 = ax.twinx()
        ax2.plot(x, np.cumsum(ve), 'o--', color='#f5a623',
                 lw=1.2, ms=4, zorder=4, label='Cumulative')
        ax2.set_ylabel('Cumulative (%)', color='#f5a623', fontsize=8)
        ax2.tick_params(colors='#f5a623', labelsize=7)
        ax2.set_facecolor('#16213e')
        ax2.set_ylim(0, 105)
        for sp in ax2.spines.values():
            sp.set_edgecolor('#555')

        self.fig.canvas.draw_idle()

    def _draw_loadings(self):
        ax = self.ax_loadings
        ax.cla()
        style_ax(ax, bg='#16213e')

        d = self.data
        # PC1 loadings
        loadings_pc1 = self.loadings[0]  # length = n_genes

        # Top 10 by absolute value
        top_idx = np.argsort(np.abs(loadings_pc1))[-10:]
        top_idx = top_idx[np.argsort(loadings_pc1[top_idx])]  # sort by value

        vals = loadings_pc1[top_idx]
        names = [d['gene_names'][i] for i in top_idx]
        colors = ['#50fa7b' if v >= 0 else '#e94560' for v in vals]

        y = np.arange(len(top_idx))
        ax.barh(y, vals, color=colors, alpha=0.85, height=0.6, zorder=3)
        ax.axvline(0, color='white', lw=0.8, alpha=0.6)

        ax.set_yticks(y)
        ax.set_yticklabels(names, color='white', fontsize=7.5)
        ax.set_xlabel('PC1 Loading', color='white')
        ax.set_title('Top Gene Loadings — PC1', color='white', fontsize=10)

        # Indicate Drug A / Drug B genes
        drug_a = set(d['drug_a_genes'])
        drug_b = set(d['drug_b_genes'])
        for yi, gi in zip(y, top_idx):
            if gi in drug_a:
                marker = ' ▲'
                mc = '#f5a623'
            elif gi in drug_b:
                marker = ' ●'
                mc = '#ff79c6'
            else:
                marker = ''
                mc = 'white'
            if marker:
                ax.text(0, yi, marker, va='center', ha='center',
                        color=mc, fontsize=7, zorder=5)

        # Small legend
        from matplotlib.patches import Patch
        ax.legend(
            handles=[
                Patch(facecolor='#50fa7b', label='Positive loading'),
                Patch(facecolor='#e94560', label='Negative loading'),
            ],
            loc='lower right',
            facecolor='#1a1a2e', edgecolor='#555',
            labelcolor='white', fontsize=7,
        )

        self.fig.canvas.draw_idle()

    def _update_info(self):
        ax = self.ax_info
        ax.cla()
        ax.set_facecolor('#16213e')
        ax.axis('off')

        if self.selected_sample is None:
            ax.text(
                0.5, 0.5,
                'Click a sample point\nto see top contributing\ngenes',
                transform=ax.transAxes, ha='center', va='center',
                color='#aaaaaa', fontsize=9, style='italic',
            )
            self.fig.canvas.draw_idle()
            return

        idx = self.selected_sample
        d = self.data
        label = d['sample_labels'][idx]
        cname = d['condition_names'][d['sample_conditions'][idx]]
        batch = d['sample_batches'][idx]

        # Top 5 contributing genes for this sample on PC1
        scores_pc1 = self.loadings[0] * d['log2_expr'][:, idx]
        top5_idx = np.argsort(np.abs(scores_pc1))[-5:][::-1]

        lines = [f'Sample: {label}',
                 f'Condition: {cname}   Batch: {batch}',
                 f'PC1={self.scores[idx,0]:.2f}  PC2={self.scores[idx,1]:.2f}',
                 '',
                 'Top PC1 drivers:']
        for gi in top5_idx:
            lines.append(f'  {d["gene_names"][gi]}  loading={self.loadings[0,gi]:.3f}')

        ax.text(0.05, 0.97, '\n'.join(lines),
                transform=ax.transAxes, va='top',
                color='white', fontsize=7.5, fontfamily='monospace')

        self.fig.canvas.draw_idle()

    # ── Callbacks ─────────────────────────────────────────────────────────────

    def _on_slider(self, val):
        self.batch_strength = float(self.slider_batch.val)
        self.selected_sample = None
        self._recompute_and_redraw()

    def _on_radio(self, label):
        if 'Condition' in label:
            self.color_mode = 'Condition'
        else:
            self.color_mode = 'Batch'
        self._draw_pca()

    def _on_click(self, event):
        if event.inaxes != self.ax_pca:
            return
        if event.xdata is None or event.ydata is None:
            return

        # Find nearest sample in display coordinates
        ax = self.ax_pca
        pts = self.scores[:, :2]
        disp = ax.transData.transform(pts)
        click = ax.transData.transform([[event.xdata, event.ydata]])[0]
        dists = np.hypot(disp[:, 0] - click[0], disp[:, 1] - click[1])
        nearest = int(np.argmin(dists))
        if dists[nearest] < 18:
            self.selected_sample = nearest
        else:
            self.selected_sample = None

        self._draw_pca()
        self._update_info()


# ── Entry Point ───────────────────────────────────────────────────────────────

if __name__ == '__main__':
    print("PCA / Dimensionality Reduction: Gene Expression Clustering")
    print("=" * 58)
    print("Synthetic dataset: 80 genes × 12 samples")
    print("  3 conditions × 4 replicates (Ctrl, Drug A, Drug B)")
    print("  2 batches per condition (reps 1-2 = batch 0; reps 3-4 = batch 1)")
    print()
    print("Controls:")
    print("  Batch Effect slider — increase to add artificial batch noise")
    print("  Radio buttons       — color points by Condition or Batch")
    print("  Click a point       — see sample details and top PC1 driver genes")
    print()
    print("Key insight:")
    print("  At Batch Effect = 0: PC1 separates conditions (biology).")
    print("  As Batch Effect increases: PC1 separates batches (noise).")
    print("  This illustrates why batch correction matters before PCA.")
    print()
    PCAExplorer()
