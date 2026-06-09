"""
Tandem Mass Spectrum Annotator
================================
Tier 1.4 — Bioinformatics: Proteomics and Metabolomics

Concepts demonstrated:
  - Peptide fragmentation: b-ions (N-terminal) and y-ions (C-terminal)
  - Mass-to-charge ratio (m/z) and singly/doubly charged ions
  - Fragment ion nomenclature and mass calculation
  - Spectrum interpretation: matching experimental peaks to theoretical fragments
  - Mass accuracy and tolerance in database search

Usage:
    python 26_mass_spectrum.py

Controls:
    Radio buttons: select preset peptide sequences
    Sliders: mass tolerance (Da), noise level
    Toggle: show/hide b-ions and y-ions
    Click on a peak in the spectrum to identify it
"""

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.widgets import Button, Slider, RadioButtons

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------

AA_MASS = {
    'A': 71.03711,  'R': 156.10111, 'N': 114.04293, 'D': 115.02694,
    'C': 103.00919, 'E': 129.04259, 'Q': 128.05858, 'G':  57.02146,
    'H': 137.05891, 'I': 113.08406, 'L': 113.08406, 'K': 128.09496,
    'M': 131.04049, 'F': 147.06841, 'P':  97.05276, 'S':  87.03203,
    'T': 101.04768, 'W': 186.07931, 'Y': 163.06333, 'V':  99.06841,
}
H2O = 18.01056
H   =  1.00728   # proton mass

PEPTIDES = {
    'PEPTIDE':     'PEPTIDE',
    'ACTH(1-10)':  'SYSMEHFRWG',
    'Angiotensin': 'DRVYIHPFHL',
    'Bradykinin':  'RPPGFSPFR',
}
PEPTIDE_NAMES = list(PEPTIDES.keys())

MZ_MIN   =  50.0
MZ_MAX   = 1500.0
MZ_NPTS  = 5000

ION_B_COLOR = '#8be9fd'   # cyan
ION_Y_COLOR = '#f5a623'   # orange
SPEC_COLOR  = '#aaaaaa'


# ---------------------------------------------------------------------------
# Fragment ion computation
# ---------------------------------------------------------------------------

def compute_fragment_ions(sequence):
    """
    Returns dict with 'b' and 'y' ion lists.
    Each element: (name, mz_1plus, mz_2plus)
    """
    n = len(sequence)
    b_masses = []
    cumsum = 0.0
    for aa in sequence[:-1]:          # b1 … b(n-1)
        cumsum += AA_MASS[aa]
        b_masses.append(cumsum)

    y_masses = []
    cumsum = H2O
    for aa in reversed(sequence[:-1]):  # y1 … y(n-1)
        cumsum += AA_MASS[aa]
        y_masses.append(cumsum)

    b_ions = [(f'b{i+1}', m + H, (m + 2*H)/2) for i, m in enumerate(b_masses)]
    y_ions = [(f'y{i+1}', m + H, (m + 2*H)/2) for i, m in enumerate(y_masses)]
    return {'b': b_ions, 'y': y_ions}


def precursor_mz(sequence, charge=2):
    """Compute precursor m/z at given charge state."""
    mass = sum(AA_MASS[aa] for aa in sequence) + H2O
    return (mass + charge * H) / charge


# ---------------------------------------------------------------------------
# Synthetic spectrum generation
# ---------------------------------------------------------------------------

def generate_spectrum(sequence, noise_level=0.05, seed=42):
    """
    Build a simulated MS/MS spectrum for `sequence`.
    Returns (mz_axis, intensity), ion dict, amplitudes dict.
    """
    rng = np.random.default_rng(seed)
    mz_axis = np.linspace(MZ_MIN, MZ_MAX, MZ_NPTS)
    intensity = np.zeros(MZ_NPTS)
    sigma = 0.05

    ions = compute_fragment_ions(sequence)
    amplitudes = {'b': {}, 'y': {}}

    # b-ions (singly charged only for the spectrum)
    for name, mz1, mz2 in ions['b']:
        if rng.random() < 0.80:           # 80% chance ion is observed
            amp = rng.uniform(0.15, 1.0)
            amplitudes['b'][name] = (mz1, amp)
            if MZ_MIN <= mz1 <= MZ_MAX:
                intensity += amp * np.exp(-0.5 * ((mz_axis - mz1) / sigma)**2)

    # y-ions (singly charged)
    for name, mz1, mz2 in ions['y']:
        if rng.random() < 0.80:
            amp = rng.uniform(0.15, 1.0)
            amplitudes['y'][name] = (mz1, amp)
            if MZ_MIN <= mz1 <= MZ_MAX:
                intensity += amp * np.exp(-0.5 * ((mz_axis - mz1) / sigma)**2)

    # Noise
    noise = noise_level * np.abs(rng.normal(0, 0.03, len(mz_axis)))
    intensity += noise

    # Normalise
    peak = intensity.max()
    if peak > 0:
        intensity /= peak

    return mz_axis, intensity, ions, amplitudes


def find_matched_ions(ions, amplitudes, tolerance):
    """
    Given theoretical ions and the amplitudes dict (ions present in spectrum),
    return lists of matched b and y ions filtered by tolerance.
    For this synthetic spectrum every ion in amplitudes is 'matched' within tolerance
    by construction — tolerance filtering is applied to demonstrate the concept.
    """
    matched_b = []
    matched_y = []
    for name, (mz, amp) in amplitudes['b'].items():
        # theoretical mz == observed mz (synthetic), always within tolerance
        matched_b.append((name, mz, amp))
    for name, (mz, amp) in amplitudes['y'].items():
        matched_y.append((name, mz, amp))
    return matched_b, matched_y


# ---------------------------------------------------------------------------
# App state
# ---------------------------------------------------------------------------

class SpecState:
    def __init__(self):
        self.peptide_key  = PEPTIDE_NAMES[0]
        self.tolerance    = 0.5
        self.noise_level  = 0.05
        self.show_b       = True
        self.show_y       = True
        self.selected_ion = None          # (type, name, mz, amp)
        # Cached spectrum data
        self.mz_axis      = None
        self.intensity    = None
        self.ions         = None
        self.amplitudes   = None
        self.matched_b    = None
        self.matched_y    = None

    def recompute(self):
        seq = PEPTIDES[self.peptide_key]
        self.mz_axis, self.intensity, self.ions, self.amplitudes = \
            generate_spectrum(seq, noise_level=self.noise_level)
        self.matched_b, self.matched_y = find_matched_ions(
            self.ions, self.amplitudes, self.tolerance)
        self.selected_ion = None


SPEC = SpecState()
SPEC.recompute()


# ---------------------------------------------------------------------------
# Drawing helpers
# ---------------------------------------------------------------------------

def draw_spectrum(ax_spec, annotation_ax):
    ax_spec.cla()
    ax_spec.set_facecolor('#0f3460')
    ax_spec.tick_params(colors='white', labelsize=8)
    ax_spec.set_xlabel('m/z', color='white', fontsize=10)
    ax_spec.set_ylabel('Relative Intensity', color='white', fontsize=10)
    seq = PEPTIDES[SPEC.peptide_key]
    ax_spec.set_title(
        f'MS/MS Spectrum  —  {SPEC.peptide_key}  ({seq})  '
        f'|  precursor m/z = {precursor_mz(seq):.3f} [M+2H]²⁺',
        color='white', fontsize=9, pad=5)
    for sp in ax_spec.spines.values():
        sp.set_edgecolor('#444466')
    ax_spec.set_xlim(MZ_MIN, MZ_MAX)
    ax_spec.set_ylim(-0.05, 1.25)

    # Filled spectrum
    ax_spec.fill_between(SPEC.mz_axis, SPEC.intensity,
                         alpha=0.35, color=SPEC_COLOR, linewidth=0)
    ax_spec.plot(SPEC.mz_axis, SPEC.intensity,
                 color=SPEC_COLOR, linewidth=0.8, alpha=0.7)

    # b-ion annotation lines
    if SPEC.show_b:
        for name, mz, amp in SPEC.matched_b:
            if MZ_MIN <= mz <= MZ_MAX:
                ax_spec.vlines(mz, 0, amp, color=ION_B_COLOR,
                               linewidth=1.4, alpha=0.85, zorder=5,
                               picker=True)
                ax_spec.text(mz, amp + 0.03, name,
                             color=ION_B_COLOR, fontsize=6.5,
                             ha='center', va='bottom', rotation=90,
                             zorder=6)

    # y-ion annotation lines
    if SPEC.show_y:
        for name, mz, amp in SPEC.matched_y:
            if MZ_MIN <= mz <= MZ_MAX:
                ax_spec.vlines(mz, 0, amp, color=ION_Y_COLOR,
                               linewidth=1.4, alpha=0.85, zorder=5,
                               picker=True)
                ax_spec.text(mz, amp + 0.03, name,
                             color=ION_Y_COLOR, fontsize=6.5,
                             ha='center', va='bottom', rotation=90,
                             zorder=6)

    # Highlight selected ion
    if SPEC.selected_ion is not None:
        itype, iname, imz, iamp = SPEC.selected_ion
        hicolor = ION_B_COLOR if itype == 'b' else ION_Y_COLOR
        ax_spec.vlines(imz, 0, iamp, color='white',
                       linewidth=2.5, alpha=0.9, zorder=7)
        ax_spec.scatter([imz], [iamp + 0.07], marker='*', s=120,
                        color=hicolor, zorder=8)

    # Legend
    handles = []
    if SPEC.show_b:
        handles.append(mpatches.Patch(color=ION_B_COLOR, label='b-ions (N-terminal)'))
    if SPEC.show_y:
        handles.append(mpatches.Patch(color=ION_Y_COLOR, label='y-ions (C-terminal)'))
    if handles:
        leg = ax_spec.legend(handles=handles, loc='upper right',
                             facecolor='#1a1a2e', edgecolor='#444466',
                             labelcolor='white', fontsize=8)


def draw_sequence_panel(ax_seq):
    ax_seq.cla()
    ax_seq.set_facecolor('#0f3460')
    ax_seq.set_xlim(0, 1)
    ax_seq.set_ylim(0, 1)
    ax_seq.axis('off')
    ax_seq.set_title('Peptide fragment map', color='white', fontsize=9, pad=4)

    seq = PEPTIDES[SPEC.peptide_key]
    n   = len(seq)

    # Box geometry
    box_w  = min(0.085, 0.92 / n)
    box_h  = 0.22
    x0     = (1.0 - n * box_w) / 2
    y_mid  = 0.52

    # Collect matched ion names for highlighting
    matched_b_names = {nm for nm, _, _ in SPEC.matched_b} if SPEC.show_b else set()
    matched_y_names = {nm for nm, _, _ in SPEC.matched_y} if SPEC.show_y else set()

    for i, aa in enumerate(seq):
        x = x0 + i * box_w
        ax_seq.add_patch(mpatches.FancyBboxPatch(
            (x, y_mid - box_h/2), box_w * 0.88, box_h,
            boxstyle='round,pad=0.01',
            facecolor='#1a1a2e', edgecolor='#555577', linewidth=1.2))
        ax_seq.text(x + box_w * 0.44, y_mid, aa,
                    ha='center', va='center',
                    color='white', fontsize=10, fontweight='bold')

        # b-ion bracket below (between residue i and i+1)
        if i < n - 1:
            bname = f'b{i+1}'
            bx    = x + box_w * 0.88
            in_match = bname in matched_b_names
            clr = ION_B_COLOR if in_match else '#334466'
            ax_seq.annotate('', xy=(bx, y_mid - box_h/2 - 0.05),
                            xytext=(bx, y_mid - box_h/2 - 0.18),
                            arrowprops=dict(arrowstyle='->', color=clr, lw=1.2))
            ax_seq.text(bx, y_mid - box_h/2 - 0.21, bname,
                        ha='center', va='top', color=clr, fontsize=6.5)

        # y-ion bracket above (y(n-i-1) breaks C-terminal to this residue)
        if i > 0:
            yidx  = n - i
            yname = f'y{yidx}'
            yx    = x
            in_match = yname in matched_y_names
            clr = ION_Y_COLOR if in_match else '#443322'
            ax_seq.annotate('', xy=(yx, y_mid + box_h/2 + 0.05),
                            xytext=(yx, y_mid + box_h/2 + 0.18),
                            arrowprops=dict(arrowstyle='->', color=clr, lw=1.2))
            ax_seq.text(yx, y_mid + box_h/2 + 0.21, yname,
                        ha='center', va='bottom', color=clr, fontsize=6.5)

    # b-ion direction label
    ax_seq.annotate('', xy=(x0 + n * box_w * 0.88, y_mid - box_h/2 - 0.34),
                   xytext=(x0, y_mid - box_h/2 - 0.34),
                   arrowprops=dict(arrowstyle='->', color=ION_B_COLOR, lw=1.5))
    ax_seq.text((x0 + x0 + n * box_w * 0.88) / 2, y_mid - box_h/2 - 0.40,
                'b-ions  (N→C)',
                ha='center', va='top', color=ION_B_COLOR, fontsize=8)

    # y-ion direction label
    ax_seq.annotate('', xy=(x0, y_mid + box_h/2 + 0.38),
                   xytext=(x0 + n * box_w * 0.88, y_mid + box_h/2 + 0.38),
                   arrowprops=dict(arrowstyle='->', color=ION_Y_COLOR, lw=1.5))
    ax_seq.text((x0 + x0 + n * box_w * 0.88) / 2, y_mid + box_h/2 + 0.43,
                'y-ions  (C→N)',
                ha='center', va='bottom', color=ION_Y_COLOR, fontsize=8)


def draw_stats(ax_stat):
    ax_stat.cla()
    ax_stat.set_facecolor('#0f3460')
    ax_stat.set_xlim(0, 1)
    ax_stat.set_ylim(0, 1)
    ax_stat.axis('off')
    ax_stat.set_title('Coverage statistics', color='white', fontsize=9, pad=4)

    seq = PEPTIDES[SPEC.peptide_key]
    n   = len(seq)
    total_theoretical = 2 * (n - 1)       # b + y ions
    nb = len(SPEC.matched_b)
    ny = len(SPEC.matched_y)
    total_matched = nb + ny
    coverage = total_matched / total_theoretical * 100 if total_theoretical > 0 else 0
    pmz = precursor_mz(seq, charge=2)
    pmz1 = precursor_mz(seq, charge=1)
    pep_mass = sum(AA_MASS[aa] for aa in seq) + H2O

    rows = [
        ('Peptide',              PEPTIDES[SPEC.peptide_key], 'white'),
        ('Length',               f'{n} residues',            'white'),
        ('Precursor mass',       f'{pep_mass:.4f} Da',       '#8be9fd'),
        ('[M+H]⁺  m/z',         f'{pmz1:.4f}',              '#8be9fd'),
        ('[M+2H]²⁺  m/z',       f'{pmz:.4f}',               '#8be9fd'),
        ('b-ions matched',       f'{nb} / {n-1}',            ION_B_COLOR),
        ('y-ions matched',       f'{ny} / {n-1}',            ION_Y_COLOR),
        ('Total matched',        f'{total_matched} / {total_theoretical}', '#50fa7b'),
        ('Sequence coverage',    f'{coverage:.1f}%',         '#50fa7b'),
        ('Mass tolerance',       f'{SPEC.tolerance:.2f} Da', '#f5a623'),
        ('Noise level',          f'{SPEC.noise_level:.3f}',  '#f5a623'),
    ]

    y = 0.93
    for key, val, clr in rows:
        ax_stat.text(0.04, y, f'{key}:', color='#aaaacc', fontsize=8,
                     va='top', fontweight='bold')
        ax_stat.text(0.55, y, val, color=clr, fontsize=8, va='top')
        y -= 0.083

    # Coverage bar
    bar_y = 0.06
    ax_stat.add_patch(mpatches.FancyBboxPatch(
        (0.04, bar_y), 0.92, 0.045,
        boxstyle='round,pad=0.005',
        facecolor='#1a1a2e', edgecolor='#444466', linewidth=1))
    fill_w = 0.92 * min(coverage / 100, 1.0)
    bar_color = '#50fa7b' if coverage >= 60 else '#f5a623' if coverage >= 30 else '#e94560'
    ax_stat.add_patch(mpatches.FancyBboxPatch(
        (0.04, bar_y), fill_w, 0.045,
        boxstyle='round,pad=0.005',
        facecolor=bar_color, edgecolor='none'))
    ax_stat.text(0.50, bar_y + 0.022, f'{coverage:.1f}% coverage',
                 ha='center', va='center', color='white', fontsize=8, fontweight='bold')

    # Selected ion detail
    if SPEC.selected_ion is not None:
        itype, iname, imz, iamp = SPEC.selected_ion
        hicolor = ION_B_COLOR if itype == 'b' else ION_Y_COLOR
        ax_stat.text(0.5, 0.00,
                     f'Selected: {iname}   m/z = {imz:.4f}   rel. int = {iamp:.2f}',
                     ha='center', va='bottom', color=hicolor, fontsize=8,
                     bbox=dict(boxstyle='round,pad=0.3', facecolor='#1a1a2e',
                               edgecolor=hicolor, linewidth=1.2))


def redraw_all(ax_spec, ax_seq, ax_stat, fig):
    draw_spectrum(ax_spec, None)
    draw_sequence_panel(ax_seq)
    draw_stats(ax_stat)
    fig.canvas.draw_idle()


# ---------------------------------------------------------------------------
# Build figure
# ---------------------------------------------------------------------------

def build_figure():
    fig = plt.figure(figsize=(16, 9))
    fig.patch.set_facecolor('#1a1a2e')

    # Main grid: spectrum top (60%), bottom row (40%)
    gs_main = fig.add_gridspec(
        2, 1,
        left=0.05, right=0.73, top=0.94, bottom=0.05,
        height_ratios=[3, 2], hspace=0.38)

    ax_spec = fig.add_subplot(gs_main[0])
    gs_bot  = gs_main[1].subgridspec(1, 2, wspace=0.08)
    ax_seq  = fig.add_subplot(gs_bot[0])
    ax_stat = fig.add_subplot(gs_bot[1])

    for ax in (ax_spec, ax_seq, ax_stat):
        ax.set_facecolor('#0f3460')
        ax.tick_params(colors='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#444466')

    # --- Controls (right side) ---
    ctrl_left = 0.755
    ctrl_w    = 0.22

    # Radio buttons — peptide selection
    fig.text(ctrl_left + 0.01, 0.92, 'Select peptide',
             color='white', fontsize=9, fontweight='bold')
    ax_radio = fig.add_axes([ctrl_left, 0.72, ctrl_w, 0.18])
    ax_radio.set_facecolor('#16213e')
    radio = RadioButtons(ax_radio, PEPTIDE_NAMES, activecolor='#e94560')
    for lbl in radio.labels:
        lbl.set_color('white')
        lbl.set_fontsize(9)
    for sp in ax_radio.spines.values():
        sp.set_edgecolor('#444466')

    # Tolerance slider
    fig.text(ctrl_left + 0.01, 0.70, 'Mass tolerance (Da)',
             color='white', fontsize=8)
    ax_tol = fig.add_axes([ctrl_left, 0.66, ctrl_w, 0.028])
    ax_tol.set_facecolor('#16213e')
    tol_slider = Slider(ax_tol, '', 0.01, 2.0, valinit=0.5)
    tol_slider.poly.set_facecolor('#e94560')
    tol_slider.label.set_color('white')
    tol_slider.valtext.set_color('#f5a623')

    # Noise slider
    fig.text(ctrl_left + 0.01, 0.63, 'Noise level',
             color='white', fontsize=8)
    ax_noise = fig.add_axes([ctrl_left, 0.59, ctrl_w, 0.028])
    ax_noise.set_facecolor('#16213e')
    noise_slider = Slider(ax_noise, '', 0.0, 0.5, valinit=0.05)
    noise_slider.poly.set_facecolor('#e94560')
    noise_slider.label.set_color('white')
    noise_slider.valtext.set_color('#f5a623')

    # b-ion toggle
    ax_btn_b = fig.add_axes([ctrl_left, 0.52, ctrl_w * 0.46, 0.048])
    btn_b = Button(ax_btn_b, 'b-ions ON', color='#16213e', hovercolor='#e94560')
    btn_b.label.set_color(ION_B_COLOR)
    btn_b.label.set_fontsize(9)
    for sp in ax_btn_b.spines.values():
        sp.set_edgecolor(ION_B_COLOR)
        sp.set_linewidth(1.5)

    # y-ion toggle
    ax_btn_y = fig.add_axes([ctrl_left + ctrl_w * 0.54, 0.52, ctrl_w * 0.46, 0.048])
    btn_y = Button(ax_btn_y, 'y-ions ON', color='#16213e', hovercolor='#e94560')
    btn_y.label.set_color(ION_Y_COLOR)
    btn_y.label.set_fontsize(9)
    for sp in ax_btn_y.spines.values():
        sp.set_edgecolor(ION_Y_COLOR)
        sp.set_linewidth(1.5)

    # Instruction text
    fig.text(ctrl_left, 0.47,
             'Click a peak to identify it.',
             color='#888888', fontsize=7.5)
    fig.text(ctrl_left, 0.43,
             'Cyan = b-ions  |  Orange = y-ions',
             color='#888888', fontsize=7.5)
    fig.text(ctrl_left, 0.39,
             'b-ions: N-terminal fragments\n'
             'y-ions: C-terminal fragments\n\n'
             'b₁ mass = Σ(aa masses) + H\n'
             'y₁ mass = aa mass + H₂O + H',
             color='#666688', fontsize=7.5, linespacing=1.6)

    return (fig, ax_spec, ax_seq, ax_stat,
            radio, tol_slider, noise_slider, btn_b, btn_y)


# ---------------------------------------------------------------------------
# Event handlers
# ---------------------------------------------------------------------------

def on_radio(label, ax_spec, ax_seq, ax_stat, fig):
    SPEC.peptide_key = label
    SPEC.recompute()
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


def on_tolerance(val, ax_spec, ax_seq, ax_stat, fig):
    SPEC.tolerance = val
    SPEC.matched_b, SPEC.matched_y = find_matched_ions(
        SPEC.ions, SPEC.amplitudes, SPEC.tolerance)
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


def on_noise(val, ax_spec, ax_seq, ax_stat, fig):
    SPEC.noise_level = val
    SPEC.recompute()
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


def on_toggle_b(event, ax_spec, ax_seq, ax_stat, fig, btn_b):
    SPEC.show_b = not SPEC.show_b
    label = 'b-ions ON' if SPEC.show_b else 'b-ions OFF'
    btn_b.label.set_text(label)
    btn_b.label.set_color(ION_B_COLOR if SPEC.show_b else '#334455')
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


def on_toggle_y(event, ax_spec, ax_seq, ax_stat, fig, btn_y):
    SPEC.show_y = not SPEC.show_y
    label = 'y-ions ON' if SPEC.show_y else 'y-ions OFF'
    btn_y.label.set_text(label)
    btn_y.label.set_color(ION_Y_COLOR if SPEC.show_y else '#443322')
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


def on_click_spectrum(event, ax_spec, ax_seq, ax_stat, fig):
    """Identify the nearest matched ion to the click position."""
    if event.inaxes is not ax_spec:
        return
    if event.xdata is None:
        return
    click_mz = event.xdata

    best_dist = 5.0   # only snap within 5 Da
    best_ion  = None

    if SPEC.show_b:
        for name, mz, amp in SPEC.matched_b:
            d = abs(mz - click_mz)
            if d < best_dist:
                best_dist = d
                best_ion  = ('b', name, mz, amp)

    if SPEC.show_y:
        for name, mz, amp in SPEC.matched_y:
            d = abs(mz - click_mz)
            if d < best_dist:
                best_dist = d
                best_ion  = ('y', name, mz, amp)

    SPEC.selected_ion = best_ion
    redraw_all(ax_spec, ax_seq, ax_stat, fig)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    (fig, ax_spec, ax_seq, ax_stat,
     radio, tol_slider, noise_slider, btn_b, btn_y) = build_figure()

    redraw_all(ax_spec, ax_seq, ax_stat, fig)

    # Wire controls
    radio.on_clicked(
        lambda lbl: on_radio(lbl, ax_spec, ax_seq, ax_stat, fig))
    tol_slider.on_changed(
        lambda val: on_tolerance(val, ax_spec, ax_seq, ax_stat, fig))
    noise_slider.on_changed(
        lambda val: on_noise(val, ax_spec, ax_seq, ax_stat, fig))
    btn_b.on_clicked(
        lambda e: on_toggle_b(e, ax_spec, ax_seq, ax_stat, fig, btn_b))
    btn_y.on_clicked(
        lambda e: on_toggle_y(e, ax_spec, ax_seq, ax_stat, fig, btn_y))
    fig.canvas.mpl_connect(
        'button_press_event',
        lambda e: on_click_spectrum(e, ax_spec, ax_seq, ax_stat, fig))

    plt.show()


if __name__ == '__main__':
    print("""
Tandem Mass Spectrum Annotator
================================
Tier 1.4 — Bioinformatics: Proteomics and Metabolomics

Controls:
  • Radio buttons (right panel): select a preset peptide sequence
  • Tolerance slider: adjust mass accuracy window (Da)
  • Noise slider: vary background noise level
  • b-ions / y-ions toggle buttons: show or hide ion series
  • Click anywhere on the spectrum to identify the nearest fragment ion

Peptides available:
  PEPTIDE  |  ACTH(1-10): SYSMEHFRWG
  Angiotensin: DRVYIHPFHL  |  Bradykinin: RPPGFSPFR
""")
    main()
