"""
Variant Consequence Annotator
==============================
Tier 1.2 — Bioinformatics: Genomics

Concepts demonstrated:
  - Gene structure: exons, introns, UTRs, splice sites
  - Variant types: SNP, insertion, deletion
  - Consequence classification: synonymous, missense, nonsense, frameshift,
    splice-site, intronic, UTR variants
  - Functional impact scoring (CADD-like qualitative scale)
  - Variant filtering by consequence class

Usage:
    python 25_variant_annotator.py

Controls:
    Click a variant flag in the genome browser to view its details
    Toggle consequence filter buttons to show/hide variant classes
    Use the zoom slider to expand regions of the genome
"""

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.widgets import Button, Slider
from matplotlib.lines import Line2D

# ---------------------------------------------------------------------------
# Gene model
# ---------------------------------------------------------------------------

GENE = {
    'name': 'BRCA2-toy',
    'chrom': 'chr1',
    'start': 200,
    'end': 1800,
    'strand': '+',
    'cds_start': 310,
    'regions': [
        {'name': "5'UTR",    'start': 200,  'end': 299,  'type': 'utr5'},
        {'name': 'Exon1',    'start': 300,  'end': 450,  'type': 'exon'},
        {'name': 'Intron1',  'start': 451,  'end': 649,  'type': 'intron'},
        {'name': 'Exon2',    'start': 650,  'end': 800,  'type': 'exon'},
        {'name': 'Intron2',  'start': 801,  'end': 999,  'type': 'intron'},
        {'name': 'Exon3',    'start': 1000, 'end': 1150, 'type': 'exon'},
        {'name': 'Intron3',  'start': 1151, 'end': 1349, 'type': 'intron'},
        {'name': 'Exon4',    'start': 1350, 'end': 1600, 'type': 'exon'},
        {'name': "3'UTR",    'start': 1601, 'end': 1800, 'type': 'utr3'},
    ],
}

# ---------------------------------------------------------------------------
# Variants
# ---------------------------------------------------------------------------

VARIANTS = [
    {'pos': 250,  'ref': 'A',   'alt': 'G',  'type': 'SNP',       'consequence': 'UTR_5prime',      'description': "5' UTR variant — may affect translation initiation"},
    {'pos': 340,  'ref': 'C',   'alt': 'T',  'type': 'SNP',       'consequence': 'synonymous',      'description': 'Synonymous: codon change but same amino acid (silent)'},
    {'pos': 380,  'ref': 'G',   'alt': 'A',  'type': 'SNP',       'consequence': 'missense',        'description': 'Missense: amino acid change (moderate functional impact)'},
    {'pos': 420,  'ref': 'C',   'alt': 'T',  'type': 'SNP',       'consequence': 'nonsense',        'description': 'Nonsense: premature stop codon introduced (high impact)'},
    {'pos': 451,  'ref': 'GT',  'alt': 'AT', 'type': 'SNP',       'consequence': 'splice_donor',    'description': 'Splice donor disrupted — likely exon skipping'},
    {'pos': 500,  'ref': 'T',   'alt': 'A',  'type': 'SNP',       'consequence': 'intronic',        'description': 'Deep intronic variant — usually benign'},
    {'pos': 648,  'ref': 'AG',  'alt': 'GG', 'type': 'SNP',       'consequence': 'splice_acceptor', 'description': 'Splice acceptor disrupted (high impact)'},
    {'pos': 690,  'ref': 'T',   'alt': 'TA', 'type': 'insertion', 'consequence': 'frameshift',      'description': 'Single-base insertion → frameshift (high impact)'},
    {'pos': 720,  'ref': 'GA',  'alt': 'G',  'type': 'deletion',  'consequence': 'frameshift',      'description': 'Single-base deletion → frameshift (high impact)'},
    {'pos': 760,  'ref': 'A',   'alt': 'G',  'type': 'SNP',       'consequence': 'missense',        'description': 'Missense variant in Exon 2'},
    {'pos': 900,  'ref': 'C',   'alt': 'T',  'type': 'SNP',       'consequence': 'intronic',        'description': 'Intronic variant (Intron 2)'},
    {'pos': 1050, 'ref': 'G',   'alt': 'C',  'type': 'SNP',       'consequence': 'synonymous',      'description': 'Synonymous variant in Exon 3'},
    {'pos': 1100, 'ref': 'A',   'alt': 'T',  'type': 'SNP',       'consequence': 'missense',        'description': 'Missense in Exon 3'},
    {'pos': 1200, 'ref': 'C',   'alt': 'T',  'type': 'SNP',       'consequence': 'intronic',        'description': 'Intronic (Intron 3)'},
    {'pos': 1400, 'ref': 'T',   'alt': 'A',  'type': 'SNP',       'consequence': 'missense',        'description': 'Missense in Exon 4'},
    {'pos': 1450, 'ref': 'G',   'alt': 'T',  'type': 'SNP',       'consequence': 'nonsense',        'description': 'Nonsense variant in Exon 4 — introduces stop codon'},
    {'pos': 1500, 'ref': 'ACC', 'alt': 'A',  'type': 'deletion',  'consequence': 'frameshift',      'description': '2-bp deletion → frameshift in Exon 4'},
    {'pos': 1700, 'ref': 'T',   'alt': 'G',  'type': 'SNP',       'consequence': 'UTR_3prime',      'description': "3' UTR variant — may affect mRNA stability"},
]

# ---------------------------------------------------------------------------
# Color / impact maps
# ---------------------------------------------------------------------------

CONSEQUENCE_COLORS = {
    'synonymous':      '#50fa7b',
    'missense':        '#f5a623',
    'nonsense':        '#e94560',
    'frameshift':      '#ff0000',
    'splice_donor':    '#ff79c6',
    'splice_acceptor': '#ff79c6',
    'intronic':        '#6272a4',
    'UTR_5prime':      '#8be9fd',
    'UTR_3prime':      '#8be9fd',
}

IMPACT_MAP = {
    'frameshift':      ('HIGH',     '#ff0000'),
    'nonsense':        ('HIGH',     '#e94560'),
    'splice_donor':    ('HIGH',     '#ff79c6'),
    'splice_acceptor': ('HIGH',     '#ff79c6'),
    'missense':        ('MODERATE', '#f5a623'),
    'synonymous':      ('LOW',      '#50fa7b'),
    'intronic':        ('MODIFIER', '#6272a4'),
    'UTR_5prime':      ('MODIFIER', '#8be9fd'),
    'UTR_3prime':      ('MODIFIER', '#8be9fd'),
}

# Short display labels for toggle buttons
CONSEQUENCE_LABELS = [
    'synonymous', 'missense', 'nonsense', 'frameshift',
    'splice_donor', 'splice_acceptor', 'intronic', 'UTR_5prime', 'UTR_3prime',
]

# Toy reference sequence (repeating pattern for illustration)
_BASES = 'ACGTACGTACGT'
def toy_seq(pos, length=1):
    """Return a toy reference sequence string at genomic position `pos`."""
    return ''.join(_BASES[(pos + i) % len(_BASES)] for i in range(length))


# ---------------------------------------------------------------------------
# App state
# ---------------------------------------------------------------------------

class AppState:
    def __init__(self):
        self.selected_variant = None
        self.visible_consequences = {c: True for c in CONSEQUENCE_LABELS}
        self.zoom_start = 1
        self.zoom_end = 2000
        # variant artists: list of (patch/line artist, variant index)
        self.variant_artists = []
        self.toggle_states = {c: True for c in CONSEQUENCE_LABELS}


STATE = AppState()

# ---------------------------------------------------------------------------
# Drawing helpers
# ---------------------------------------------------------------------------

def draw_gene_model(ax):
    """Draw the BRCA2-toy gene structure on ax."""
    ax.cla()
    ax.set_facecolor('#16213e')
    ax.set_xlim(STATE.zoom_start, STATE.zoom_end)
    ax.set_ylim(-1.5, 3.5)
    ax.tick_params(colors='white', labelsize=8)
    ax.set_xlabel('Genomic position (chr1)', color='white', fontsize=9)
    ax.set_title('BRCA2-toy Gene Browser  —  click a variant flag for details',
                 color='white', fontsize=10, pad=6)
    for spine in ax.spines.values():
        spine.set_edgecolor('#444466')

    # Gene backbone line (introns)
    ax.hlines(0, GENE['start'], GENE['end'], color='#8888aa', linewidth=1.5, zorder=2)

    # Region rectangles
    for region in GENE['regions']:
        rtype = region['type']
        rstart = region['start']
        rend = region['end']
        width = rend - rstart
        if rtype == 'exon':
            rect = mpatches.FancyBboxPatch(
                (rstart, -0.35), width, 0.7,
                boxstyle='round,pad=0.01',
                facecolor='#8be9fd', edgecolor='#44ccff', linewidth=1, zorder=3)
            ax.add_patch(rect)
            ax.text(rstart + width / 2, -0.65, region['name'],
                    ha='center', va='top', color='#8be9fd', fontsize=6.5, zorder=4)
        elif rtype in ('utr5', 'utr3'):
            rect = mpatches.FancyBboxPatch(
                (rstart, -0.22), width, 0.44,
                boxstyle='round,pad=0.01',
                facecolor='#6272a4', edgecolor='#8888cc', linewidth=0.8, zorder=3)
            ax.add_patch(rect)
            label = "5'UTR" if rtype == 'utr5' else "3'UTR"
            ax.text(rstart + width / 2, -0.5, label,
                    ha='center', va='top', color='#9999bb', fontsize=6, zorder=4)

    # Arrow indicating strand direction
    ax.annotate('', xy=(GENE['end'] + 20, 0), xytext=(GENE['end'], 0),
                arrowprops=dict(arrowstyle='->', color='#aaaacc', lw=1.2))
    ax.text(GENE['end'] + 25, 0, "+", color='#aaaacc', fontsize=8, va='center')

    # --- Variants ---
    STATE.variant_artists = []
    for vi, v in enumerate(VARIANTS):
        csq = v['consequence']
        if not STATE.visible_consequences.get(csq, True):
            continue
        color = CONSEQUENCE_COLORS.get(csq, '#ffffff')
        pos = v['pos']
        flag_y = 1.0
        # Stagger flags a little to reduce overlap
        flag_y += 0.25 * (vi % 3)

        # Vertical stem
        stem, = ax.plot([pos, pos], [0.4, flag_y + 0.15],
                        color=color, linewidth=1.2, zorder=5,
                        picker=True, pickradius=6)
        stem._variant_index = vi

        # Triangle marker
        tri = ax.scatter([pos], [flag_y + 0.2], marker='^',
                         s=40, color=color, zorder=6,
                         picker=True)
        tri._variant_index = vi

        # Label (tiny, rotated)
        short = csq.replace('_', ' ')[:12]
        ax.text(pos + 3, flag_y + 0.25, short,
                color=color, fontsize=5, rotation=45,
                va='bottom', ha='left', zorder=7)

        STATE.variant_artists.append((stem, vi))
        STATE.variant_artists.append((tri, vi))

    # Highlight selected variant
    if STATE.selected_variant is not None:
        sv = VARIANTS[STATE.selected_variant]
        ax.axvline(sv['pos'], color='white', linewidth=1.5,
                   linestyle='--', alpha=0.6, zorder=8)


def draw_detail(ax, variant_index):
    """Render detail panel for a chosen variant."""
    ax.cla()
    ax.set_facecolor('#0f3460')
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis('off')

    if variant_index is None:
        ax.text(0.5, 0.5,
                'Click a variant flag above to view details',
                color='#888888', fontsize=11, ha='center', va='center',
                style='italic')
        return

    v = VARIANTS[variant_index]
    csq = v['consequence']
    color = CONSEQUENCE_COLORS.get(csq, '#ffffff')
    impact_label, impact_color = IMPACT_MAP.get(csq, ('UNKNOWN', '#ffffff'))

    # Title
    ax.text(0.02, 0.93, f"Variant at chr1:{v['pos']}",
            color='white', fontsize=11, fontweight='bold', va='top')

    # Impact badge
    ax.text(0.98, 0.93, f'IMPACT: {impact_label}',
            color=impact_color, fontsize=11, fontweight='bold',
            ha='right', va='top',
            bbox=dict(boxstyle='round,pad=0.3', facecolor='#1a1a2e',
                      edgecolor=impact_color, linewidth=1.5))

    # Key-value rows
    rows = [
        ('Position',     f"chr1:{v['pos']}"),
        ('Change',       f"{v['ref']}  →  {v['alt']}"),
        ('Variant type', v['type'].upper()),
        ('Consequence',  csq.replace('_', ' ').title()),
        ('Description',  v['description']),
    ]

    y = 0.76
    for key, val in rows:
        ax.text(0.02, y, f"{key}:", color='#aaaacc', fontsize=9,
                va='top', fontweight='bold')
        # For description, allow wrapping via a narrower width
        if key == 'Description':
            ax.text(0.22, y, val, color='white', fontsize=9,
                    va='top', wrap=True,
                    bbox=dict(boxstyle='round,pad=0.2',
                              facecolor='#1a1a2e', edgecolor='none'))
        else:
            ax.text(0.22, y, val, color=color if key == 'Consequence' else 'white',
                    fontsize=9, va='top')
        y -= 0.13

    # Sequence context strip
    ctx_start = max(1, v['pos'] - 8)
    ctx_end = ctx_start + 17
    seq = toy_seq(ctx_start, 17)
    rel = v['pos'] - ctx_start  # index of variant in context string

    ax.text(0.02, 0.08, 'Sequence context:', color='#aaaacc', fontsize=8, va='bottom')
    x_off = 0.22
    for i, base in enumerate(seq):
        face = '#1a1a2e'
        bcolor = '#888888'
        if i == rel:
            face = color
            bcolor = color
        ax.text(x_off + i * 0.038, 0.08, base,
                color='white' if i == rel else '#aaaacc',
                fontsize=8, va='bottom', ha='center',
                fontfamily='monospace',
                bbox=dict(boxstyle='round,pad=0.15',
                          facecolor=face, edgecolor=bcolor, linewidth=0.8))

    # Position numbers
    ax.text(0.22, 0.01, str(ctx_start), color='#555577', fontsize=6.5, va='bottom')
    ax.text(0.22 + (16) * 0.038, 0.01, str(ctx_end),
            color='#555577', fontsize=6.5, va='bottom', ha='right')


# ---------------------------------------------------------------------------
# Build figure
# ---------------------------------------------------------------------------

def build_figure():
    fig = plt.figure(figsize=(16, 9))
    fig.patch.set_facecolor('#1a1a2e')

    # Grid: browser (top 60%), detail (bottom 40%), toggle column (right)
    gs = fig.add_gridspec(
        2, 2,
        left=0.05, right=0.78, top=0.93, bottom=0.05,
        height_ratios=[3, 2], hspace=0.35, wspace=0.08,
    )
    ax_browser = fig.add_subplot(gs[0, :])
    ax_detail  = fig.add_subplot(gs[1, :])

    ax_browser.set_facecolor('#16213e')
    ax_detail.set_facecolor('#0f3460')

    for ax in (ax_browser, ax_detail):
        ax.tick_params(colors='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#444466')

    # --- Toggle buttons (right column) ---
    toggle_axes = {}
    btn_objects = {}
    n_csq = len(CONSEQUENCE_LABELS)
    btn_height = 0.052
    btn_gap    = 0.008
    total_h    = n_csq * (btn_height + btn_gap)
    top_y      = 0.93
    left_x     = 0.80

    fig.text(left_x + 0.01, top_y + 0.015, 'Filter by consequence',
             color='white', fontsize=8, fontweight='bold')

    for i, csq in enumerate(CONSEQUENCE_LABELS):
        y_pos = top_y - (i + 1) * (btn_height + btn_gap)
        bax = fig.add_axes([left_x, y_pos, 0.18, btn_height])
        color = CONSEQUENCE_COLORS.get(csq, '#ffffff')
        label = csq.replace('_', ' ')
        btn = Button(bax, label, color='#16213e', hovercolor='#e94560')
        btn.label.set_color(color)
        btn.label.set_fontsize(8)
        # Mark border with consequence color
        for sp in bax.spines.values():
            sp.set_edgecolor(color)
            sp.set_linewidth(1.5)
        toggle_axes[csq] = bax
        btn_objects[csq] = btn

    # Zoom slider
    ax_zoom = fig.add_axes([0.07, 0.01, 0.55, 0.025])
    ax_zoom.set_facecolor('#16213e')
    zoom_slider = Slider(ax_zoom, 'Zoom', 1, 2000,
                         valinit=1, valstep=1)
    zoom_slider.poly.set_facecolor('#e94560')
    zoom_slider.label.set_color('white')
    zoom_slider.valtext.set_color('white')

    return fig, ax_browser, ax_detail, btn_objects, zoom_slider


# ---------------------------------------------------------------------------
# Event handlers
# ---------------------------------------------------------------------------

def on_pick(event, ax_browser, ax_detail, fig):
    artist = event.artist
    if hasattr(artist, '_variant_index'):
        vi = artist._variant_index
        STATE.selected_variant = vi
        draw_gene_model(ax_browser)
        draw_detail(ax_detail, vi)
        fig.canvas.draw_idle()


def make_toggle_callback(csq, ax_browser, ax_detail, fig, btn_objects):
    def callback(event):
        STATE.visible_consequences[csq] = not STATE.visible_consequences[csq]
        btn = btn_objects[csq]
        color = CONSEQUENCE_COLORS.get(csq, '#ffffff')
        if STATE.visible_consequences[csq]:
            btn.label.set_color(color)
            btn.ax.set_facecolor('#16213e')
        else:
            btn.label.set_color('#444455')
            btn.ax.set_facecolor('#0a0a18')
        draw_gene_model(ax_browser)
        # If selected variant is now hidden, clear detail
        if STATE.selected_variant is not None:
            sv_csq = VARIANTS[STATE.selected_variant]['consequence']
            if not STATE.visible_consequences.get(sv_csq, True):
                STATE.selected_variant = None
                draw_detail(ax_detail, None)
        fig.canvas.draw_idle()
    return callback


def make_zoom_callback(ax_browser, fig):
    def callback(val):
        zstart = int(val)
        zend   = min(2000, zstart + int(2000 * 0.5))
        if zend - zstart < 200:
            zend = min(2000, zstart + 200)
        STATE.zoom_start = zstart
        STATE.zoom_end   = zend
        ax_browser.set_xlim(STATE.zoom_start, STATE.zoom_end)
        draw_gene_model(ax_browser)
        fig.canvas.draw_idle()
    return callback


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    fig, ax_browser, ax_detail, btn_objects, zoom_slider = build_figure()

    draw_gene_model(ax_browser)
    draw_detail(ax_detail, None)

    # Wire toggle buttons
    for csq in CONSEQUENCE_LABELS:
        btn_objects[csq].on_clicked(
            make_toggle_callback(csq, ax_browser, ax_detail, fig, btn_objects))

    # Wire zoom slider
    zoom_slider.on_changed(make_zoom_callback(ax_browser, fig))

    # Wire pick events
    fig.canvas.mpl_connect(
        'pick_event',
        lambda e: on_pick(e, ax_browser, ax_detail, fig))

    plt.show()


if __name__ == '__main__':
    print("""
Variant Consequence Annotator
==============================
Tier 1.2 — Bioinformatics: Genomics

Controls:
  • Click a variant flag (triangle) in the genome browser to view its details
  • Toggle consequence filter buttons on the right to show/hide variant classes
  • Use the zoom slider at the bottom to expand regions of the genome
""")
    main()
