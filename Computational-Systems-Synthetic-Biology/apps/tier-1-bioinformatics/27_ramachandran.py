"""
Ramachandran Plot Explorer
===========================
Tier 1.5 — Bioinformatics: Structural Bioinformatics

Concepts demonstrated:
  - Backbone torsion angles φ (phi) and ψ (psi)
  - Sterically allowed regions vs forbidden conformations
  - Secondary structure: α-helices, β-sheets, polyproline II
  - Glycine's special properties (no beta-carbon)
  - Relationship between torsion angle clusters and 3D structure

Usage:
    python 27_ramachandran.py

Controls:
    Radio buttons: add residues with torsion angles typical of
                   α-helix, β-sheet (parallel), β-sheet (antiparallel),
                   polyproline II, or Glycine distributions
    Slider: number of residues to add per click of "Add Residues"
    Button: "Add Residues" — scatter residues for the selected type
    Button: "Clear" — clear all scattered residues
    Click anywhere on the Ramachandran plot to add a custom residue
      and see if it falls in an allowed region
"""

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.widgets import RadioButtons, Slider, Button
from matplotlib.colors import LinearSegmentedColormap



# ---------------------------------------------------------------------------
# Residue type definitions
# ---------------------------------------------------------------------------
RESIDUE_TYPES = {
    'α-Helix':        {'phi_mean': -63,  'psi_mean': -43,  'phi_std': 8,   'psi_std': 8,   'color': '#e94560'},
    'β-Sheet (anti)': {'phi_mean': -119, 'psi_mean': 128,  'phi_std': 12,  'psi_std': 12,  'color': '#8be9fd'},
    'β-Sheet (par)':  {'phi_mean': -113, 'psi_mean': 118,  'phi_std': 10,  'psi_std': 10,  'color': '#50fa7b'},
    'PPII Helix':     {'phi_mean': -60,  'psi_mean': 140,  'phi_std': 10,  'psi_std': 12,  'color': '#f5a623'},
    'Glycine (all)':  {'phi_mean': 0,    'psi_mean': 0,    'phi_std': 60,  'psi_std': 60,  'color': '#ff79c6'},
    'Loop / Random':  {'phi_mean': -90,  'psi_mean': 80,   'phi_std': 50,  'psi_std': 60,  'color': '#bd93f9'},
}

REGION_LABELS = [
    (-63,  -43,  'α-helix'),
    (-119,  128, 'β-sheet\n(anti)'),
    (-113,  113, 'β-sheet\n(par)'),
    (-60,   140, 'PPII'),
    (63,    43,  'αL'),
]

# ---------------------------------------------------------------------------
# Background density
# ---------------------------------------------------------------------------
def ramachandran_density(phi_grid, psi_grid):
    """
    Approximate Ramachandran density as Gaussian mixture over known clusters.
    Returns 2D density array over (phi, psi) in [-180, 180] x [-180, 180].
    """
    clusters = [
        # (phi_mean, psi_mean, phi_std, psi_std, weight, name)
        (-63,  -43,  15, 15, 0.35, 'alpha-helix (right)'),
        (-119,  128, 20, 25, 0.25, 'beta-sheet (anti)'),
        (-113,  118, 18, 22, 0.15, 'beta-sheet (par)'),
        (-60,   140, 15, 20, 0.10, 'PPII helix'),
        (63,    40,  25, 25, 0.05, 'alpha-helix (left) — rare, Gly only'),
        (-85,   0,   20, 20, 0.05, 'epsilon/delta region'),
        (60,   -40,  20, 20, 0.05, 'gamma region'),
    ]
    density = np.zeros_like(phi_grid, dtype=float)
    for phi_m, psi_m, phi_s, psi_s, w, _ in clusters:
        d = w * np.exp(
            -0.5 * ((phi_grid - phi_m) / phi_s) ** 2
            - 0.5 * ((psi_grid - psi_m) / psi_s) ** 2
        )
        density += d
    return density


def is_in_region(phi, psi, density_map, phi_vals, psi_vals, max_density):
    """Return (label, color) for whether (phi,psi) is in an allowed region."""
    phi_idx = int(np.clip(np.searchsorted(phi_vals, phi), 0, len(phi_vals) - 1))
    psi_idx = int(np.clip(np.searchsorted(psi_vals, psi), 0, len(psi_vals) - 1))
    d = density_map[psi_idx, phi_idx]
    frac = d / max_density
    if frac > 0.10:
        return 'Allowed', '#50fa7b'
    elif frac > 0.01:
        return 'Generously Allowed', '#f5a623'
    else:
        return 'Disallowed', '#e94560'


# ---------------------------------------------------------------------------
# Build custom dark colormap: #0f3460 → #8be9fd
# ---------------------------------------------------------------------------
RAMA_CMAP = LinearSegmentedColormap.from_list(
    'rama_dark',
    ['#0f3460', '#16213e', '#1a3a6e', '#1e5a9e', '#3a8abf', '#8be9fd'],
    N=256
)

# ---------------------------------------------------------------------------
# State
# ---------------------------------------------------------------------------
state = {
    'residues': [],   # list of (phi, psi, color, marker)
    'current_type': 'α-Helix',
    'click_artist': None,
    'info_text': None,
}

# ---------------------------------------------------------------------------
# Precompute density
# ---------------------------------------------------------------------------
RESOLUTION = 200
phi_vals = np.linspace(-180, 180, RESOLUTION)
psi_vals = np.linspace(-180, 180, RESOLUTION)
PHI_GRID, PSI_GRID = np.meshgrid(phi_vals, psi_vals)
DENSITY = ramachandran_density(PHI_GRID, PSI_GRID)
MAX_DENSITY = DENSITY.max()

# ---------------------------------------------------------------------------
# Figure layout
# ---------------------------------------------------------------------------
fig = plt.figure(figsize=(14, 8))
fig.patch.set_facecolor('#1a1a2e')

# Main Ramachandran plot: left ~60% of figure
ax = fig.add_axes([0.04, 0.10, 0.56, 0.82])
ax.set_facecolor('#0f3460')

# Right panel background rect
right_bg = fig.add_axes([0.63, 0.02, 0.36, 0.96])
right_bg.set_facecolor('#16213e')
right_bg.set_xticks([])
right_bg.set_yticks([])
for spine in right_bg.spines.values():
    spine.set_edgecolor('#8be9fd')
    spine.set_linewidth(0.5)

# ---------------------------------------------------------------------------
# Draw Ramachandran background
# ---------------------------------------------------------------------------
img = ax.imshow(
    DENSITY,
    extent=[-180, 180, -180, 180],
    origin='lower',
    cmap=RAMA_CMAP,
    aspect='auto',
    vmin=0,
    vmax=MAX_DENSITY * 0.9,
    zorder=0,
)

# Isocontour lines at 20%, 50%, 80% of max
contour_levels = [0.01, 0.05, 0.10, 0.20, 0.50]
ax.contour(
    phi_vals, psi_vals, DENSITY,
    levels=[lv * MAX_DENSITY for lv in contour_levels],
    colors='white',
    alpha=0.25,
    linewidths=0.8,
    zorder=1,
)

# Grid lines at 0°
ax.axhline(0, color='white', alpha=0.3, linewidth=0.8, zorder=2)
ax.axvline(0, color='white', alpha=0.3, linewidth=0.8, zorder=2)

# Region text labels
for phi_lbl, psi_lbl, name in REGION_LABELS:
    ax.text(
        phi_lbl, psi_lbl, name,
        color='white', alpha=0.55,
        fontsize=7.5, ha='center', va='center',
        fontweight='bold', zorder=3,
        bbox=dict(facecolor='#1a1a2e', alpha=0.35, boxstyle='round,pad=0.2', edgecolor='none'),
    )

ax.set_xlim(-180, 180)
ax.set_ylim(-180, 180)
ax.set_xlabel('φ (degrees)', color='white', fontsize=11)
ax.set_ylabel('ψ (degrees)', color='white', fontsize=11)
ax.set_title('Ramachandran Plot  |  Backbone Torsion Angles', color='white', fontsize=13, pad=8)
ax.tick_params(colors='white', labelsize=9)
for spine in ax.spines.values():
    spine.set_edgecolor('#8be9fd')
    spine.set_linewidth(0.8)
ax.set_xticks([-180, -90, 0, 90, 180])
ax.set_yticks([-180, -90, 0, 90, 180])

# Scatter container (will hold PathCollections)
scatter_artists = []
click_artist_list = []

# ---------------------------------------------------------------------------
# Right panel: radio buttons label
# ---------------------------------------------------------------------------
fig.text(0.645, 0.955, 'Residue Type', color='white', fontsize=10,
         fontweight='bold', ha='left')

# RadioButtons
radio_ax = fig.add_axes([0.645, 0.68, 0.32, 0.26])
radio_ax.set_facecolor('#16213e')
radio = RadioButtons(
    radio_ax,
    list(RESIDUE_TYPES.keys()),
    active=0,
    activecolor='#e94560',
)
for label in radio.labels:
    label.set_color('white')
    label.set_fontsize(9)

# Slider: n_residues
slider_ax = fig.add_axes([0.69, 0.62, 0.25, 0.03])
slider_ax.set_facecolor('#16213e')
n_slider = Slider(
    slider_ax, 'N Residues',
    valmin=5, valmax=50, valinit=20, valstep=1,
    color='#e94560',
)
n_slider.label.set_color('white')
n_slider.valtext.set_color('white')

# Buttons
btn_add_ax = fig.add_axes([0.645, 0.545, 0.155, 0.055])
btn_add = Button(btn_add_ax, 'Add Residues', color='#16213e', hovercolor='#e94560')
btn_add.label.set_color('white')
btn_add.label.set_fontsize(9)

btn_clear_ax = fig.add_axes([0.820, 0.545, 0.155, 0.055])
btn_clear = Button(btn_clear_ax, 'Clear All', color='#16213e', hovercolor='#e94560')
btn_clear.label.set_color('white')
btn_clear.label.set_fontsize(9)

# Info text area
info_ax = fig.add_axes([0.645, 0.08, 0.33, 0.44])
info_ax.set_facecolor('#0f3460')
info_ax.set_xticks([])
info_ax.set_yticks([])
for spine in info_ax.spines.values():
    spine.set_edgecolor('#8be9fd')
    spine.set_linewidth(0.5)

info_text_obj = info_ax.text(
    0.05, 0.97, 'No residues added yet.\nClick plot or use controls.',
    transform=info_ax.transAxes,
    color='white', fontsize=8.5, va='top', ha='left',
    fontfamily='monospace',
    wrap=True,
)

fig.text(0.645, 0.535, 'Statistics & Click Info', color='#8be9fd',
         fontsize=9, fontweight='bold', ha='left')

# Legend patches (small)
legend_y = 0.030
legend_x = 0.645
for i, (name, props) in enumerate(RESIDUE_TYPES.items()):
    col = props['color']
    fig.add_artist(mpatches.FancyArrowPatch(
        (legend_x + (i % 3) * 0.12, legend_y + (1 - i // 3) * 0.022),
        (legend_x + (i % 3) * 0.12, legend_y + (1 - i // 3) * 0.022),
        arrowstyle='-',
        color=col,
    ))

# Compact legend below info box
legend_patches = [
    mpatches.Patch(color=props['color'], label=name, linewidth=0)
    for name, props in RESIDUE_TYPES.items()
]
legend = fig.legend(
    handles=legend_patches,
    loc='lower right',
    bbox_to_anchor=(0.985, 0.01),
    fontsize=7.5,
    framealpha=0.2,
    facecolor='#16213e',
    edgecolor='#8be9fd',
    labelcolor='white',
    ncol=2,
)


# ---------------------------------------------------------------------------
# Helper: build info text string
# ---------------------------------------------------------------------------
def build_info_text(last_click_info=None):
    lines = []
    total = len(state['residues'])
    lines.append(f'Total residues: {total}')
    lines.append('')
    counts = {}
    for _, _, color, _ in state['residues']:
        counts[color] = counts.get(color, 0) + 1
    for name, props in RESIDUE_TYPES.items():
        c = counts.get(props['color'], 0)
        if c > 0:
            lines.append(f'  {name}: {c}')
    if last_click_info:
        lines.append('')
        lines.append('─' * 28)
        lines.append('Last click:')
        phi_c, psi_c, region, rcol = last_click_info
        lines.append(f'  φ = {phi_c:+.1f}°,  ψ = {psi_c:+.1f}°')
        lines.append(f'  Region: {region}')
    if total > 0:
        phis = [r[0] for r in state['residues']]
        psis = [r[1] for r in state['residues']]
        lines.append('')
        lines.append('─' * 28)
        lines.append('Stats (all residues):')
        lines.append(f'  φ mean = {np.mean(phis):+.1f}°')
        lines.append(f'  ψ mean = {np.mean(psis):+.1f}°')
        lines.append(f'  φ std  = {np.std(phis):.1f}°')
        lines.append(f'  ψ std  = {np.std(psis):.1f}°')
    return '\n'.join(lines)


def redraw_residues():
    """Clear and redraw all residues from state."""
    for artist in scatter_artists:
        try:
            artist.remove()
        except Exception:
            pass
    scatter_artists.clear()

    if not state['residues']:
        return

    # Group by color for efficiency
    groups = {}
    for phi, psi, color, marker in state['residues']:
        if marker == '*':
            key = (color, '*')
        else:
            key = (color, 'o')
        groups.setdefault(key, ([], []))
        groups[key][0].append(phi)
        groups[key][1].append(psi)

    for (color, marker), (phis, psis) in groups.items():
        size = 120 if marker == '*' else 40
        sc = ax.scatter(
            phis, psis,
            c=color, marker=marker,
            s=size,
            edgecolors='white',
            linewidths=0.5,
            alpha=0.85,
            zorder=5,
        )
        scatter_artists.append(sc)


last_click_info = [None]


def update_info(click_info=None):
    if click_info is not None:
        last_click_info[0] = click_info
    info_text_obj.set_text(build_info_text(last_click_info[0]))
    fig.canvas.draw_idle()


# ---------------------------------------------------------------------------
# Callbacks
# ---------------------------------------------------------------------------
def on_radio_change(label):
    state['current_type'] = label


def on_add_residues(event):
    n = int(n_slider.val)
    rtype = state['current_type']
    props = RESIDUE_TYPES[rtype]
    color = props['color']

    if rtype == 'Glycine (all)':
        # Uniform across allowed space with mild density weighting
        phi_samples = np.random.uniform(-180, 180, n * 5)
        psi_samples = np.random.uniform(-180, 180, n * 5)
        # Mild weight: accept with probability proportional to a raised density
        density_vals = ramachandran_density(phi_samples, psi_samples)
        weight = 0.3 + 0.7 * (density_vals / MAX_DENSITY)
        accept = np.random.random(n * 5) < weight
        phi_samples = phi_samples[accept][:n]
        psi_samples = psi_samples[accept][:n]
        # Pad if not enough
        while len(phi_samples) < n:
            extra = np.random.uniform(-180, 180, n)
            extra_psi = np.random.uniform(-180, 180, n)
            phi_samples = np.concatenate([phi_samples, extra])
            psi_samples = np.concatenate([psi_samples, extra_psi])
        phi_samples = phi_samples[:n]
        psi_samples = psi_samples[:n]
    else:
        phi_samples = np.random.normal(props['phi_mean'], props['phi_std'], n)
        psi_samples = np.random.normal(props['psi_mean'], props['psi_std'], n)
        # Wrap to [-180, 180]
        phi_samples = ((phi_samples + 180) % 360) - 180
        psi_samples = ((psi_samples + 180) % 360) - 180

    for phi, psi in zip(phi_samples, psi_samples):
        state['residues'].append((float(phi), float(psi), color, 'o'))

    redraw_residues()
    update_info()


def on_clear(event):
    state['residues'].clear()
    last_click_info[0] = None
    redraw_residues()
    info_text_obj.set_text('No residues added yet.\nClick plot or use controls.')
    fig.canvas.draw_idle()


def on_click(event):
    if event.inaxes != ax:
        return
    if event.button != 1:
        return
    phi_c = event.xdata
    psi_c = event.ydata
    if phi_c is None or psi_c is None:
        return

    region, rcol = is_in_region(phi_c, psi_c, DENSITY, phi_vals, psi_vals, MAX_DENSITY)
    state['residues'].append((phi_c, psi_c, rcol, '*'))
    redraw_residues()
    update_info((phi_c, psi_c, region, rcol))


radio.on_clicked(on_radio_change)
btn_add.on_clicked(on_add_residues)
btn_clear.on_clicked(on_clear)
fig.canvas.mpl_connect('button_press_event', on_click)

# ---------------------------------------------------------------------------
# Initial info
# ---------------------------------------------------------------------------
update_info()

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    print(
        "\nRamachandran Plot Explorer"
        "\n=========================="
        "\nTier 1.5 — Bioinformatics: Structural Bioinformatics"
        "\n"
        "\nControls:"
        "\n  Radio buttons  — select residue type (α-Helix, β-Sheet, PPII, Glycine, Loop)"
        "\n  N Residues slider — how many residues to add per click"
        "\n  [Add Residues]  — scatter residues for selected type"
        "\n  [Clear All]     — remove all scattered residues"
        "\n  Click on plot   — add single residue; shows allowed/disallowed classification"
        "\n"
        "\nClose the window to exit.\n"
    )
    plt.show()
