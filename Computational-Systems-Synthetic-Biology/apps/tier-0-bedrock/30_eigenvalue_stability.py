"""
Eigenvalue / Stability Explorer
=================================
Tier 0.1 — Mathematics: Linear Algebra meets ODE Stability

Concepts demonstrated:
  - Eigenvalues of 2×2 Jacobian matrices
  - Connection between eigenvalues and fixed-point stability
  - Stable/unstable nodes, spirals, saddle points, centers
  - How Jacobian entries map to physical interactions (activation/repression)
  - Trace-determinant stability diagram
  - Biological context: linearisation of nonlinear ODE systems

Usage:
    python 30_eigenvalue_stability.py

Controls:
    Four sliders: entries a, b, c, d of the 2×2 Jacobian J = [[a,b],[c,d]]
    Radio buttons: load a preset (stable spiral, saddle point, center, etc.)
    The phase portrait, eigenvalue plot, and stability class all update live
"""

import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.widgets import Slider, RadioButtons, Button
from scipy.integrate import solve_ivp



# ---------------------------------------------------------------------------
# Presets
# ---------------------------------------------------------------------------
PRESETS = {
    'Stable Node':          (-2,    0,     0,    -1),
    'Stable Spiral':        (-0.5, -2,     2,    -0.5),
    'Unstable Spiral':      ( 0.5, -2,     2,     0.5),
    'Unstable Node':        ( 2,    1,     0,     1),
    'Saddle Point':         ( 1,    0,     0,    -1),
    'Center':               ( 0,   -2,     2,     0),
    'Toggle Switch':        (-1,   -2,    -2,    -1),
    'Activator-Inhibitor':  (-1,    2,    -1,    -2),
}

# ---------------------------------------------------------------------------
# Core analysis
# ---------------------------------------------------------------------------
def analyze_matrix(a, b, c, d):
    J = np.array([[a, b], [c, d]], dtype=float)
    eigvals = np.linalg.eigvals(J)
    trace = a + d
    det = a * d - b * c
    discriminant = trace ** 2 - 4 * det

    eps = 1e-9
    if det < -eps:
        classification = 'Saddle Point'
        color = '#f5a623'
    elif abs(det) < eps:
        classification = 'Non-isolated FP'
        color = '#6272a4'
    elif trace < -eps and det > eps:
        if discriminant > eps:
            classification = 'Stable Node'
            color = '#50fa7b'
        elif discriminant < -eps:
            classification = 'Stable Spiral'
            color = '#8be9fd'
        else:
            classification = 'Stable Star/Degenerate'
            color = '#50fa7b'
    elif trace > eps and det > eps:
        if discriminant > eps:
            classification = 'Unstable Node'
            color = '#e94560'
        elif discriminant < -eps:
            classification = 'Unstable Spiral'
            color = '#ff79c6'
        else:
            classification = 'Unstable Star'
            color = '#e94560'
    elif abs(trace) < eps and det > eps:
        classification = 'Center (pure imaginary λ)'
        color = '#ffb86c'
    else:
        classification = 'Degenerate case'
        color = '#6272a4'

    return J, eigvals, trace, det, classification, color


# ---------------------------------------------------------------------------
# Figure + axes layout
# ---------------------------------------------------------------------------
fig = plt.figure(figsize=(14, 8))
fig.patch.set_facecolor('#1a1a2e')

# Three panels side by side: trace-det | phase portrait | eigenvalue
ax_trace_det = fig.add_axes([0.04,  0.28, 0.27, 0.66])
ax_phase      = fig.add_axes([0.355, 0.28, 0.27, 0.66])
ax_eigen      = fig.add_axes([0.67,  0.28, 0.27, 0.66])

for ax_obj in (ax_trace_det, ax_phase, ax_eigen):
    ax_obj.set_facecolor('#0f3460')
    ax_obj.tick_params(colors='white', labelsize=8)
    for spine in ax_obj.spines.values():
        spine.set_edgecolor('#8be9fd')
        spine.set_linewidth(0.7)

# ---------------------------------------------------------------------------
# Slider axes (below panels)
# ---------------------------------------------------------------------------
SLIDER_LEFT   = 0.12
SLIDER_WIDTH  = 0.75
SLIDER_H      = 0.028
SLIDER_BOTTOMS = [0.175, 0.13, 0.085, 0.040]
SLIDER_LABELS  = ['a  (J[0,0])', 'b  (J[0,1])', 'c  (J[1,0])', 'd  (J[1,1])']
SLIDER_INIT    = [-0.5, -2.0, 2.0, -0.5]   # Stable Spiral default

slider_axes = []
sliders     = []
for i, (bot, lbl, init) in enumerate(zip(SLIDER_BOTTOMS, SLIDER_LABELS, SLIDER_INIT)):
    sax = fig.add_axes([SLIDER_LEFT, bot, SLIDER_WIDTH, SLIDER_H])
    sax.set_facecolor('#16213e')
    sl = Slider(sax, lbl, valmin=-4.0, valmax=4.0, valinit=init, color='#e94560')
    sl.label.set_color('white')
    sl.label.set_fontsize(8.5)
    sl.valtext.set_color('white')
    slider_axes.append(sax)
    sliders.append(sl)

sl_a, sl_b, sl_c, sl_d = sliders

# ---------------------------------------------------------------------------
# RadioButtons for presets
# ---------------------------------------------------------------------------
radio_ax = fig.add_axes([0.00, 0.28, 0.035, 0.66])
radio_ax.set_facecolor('#16213e')
radio_ax.set_visible(False)   # invisible frame; we place the actual widget

# We want the radio on far left — use a slightly wider rect
radio_ax2 = fig.add_axes([0.000, 0.28, 0.04, 0.66])
radio_ax2.set_facecolor('#16213e')
for sp in radio_ax2.spines.values():
    sp.set_edgecolor('#8be9fd')
    sp.set_linewidth(0.5)
radio_ax2.set_xticks([])
radio_ax2.set_yticks([])

radio = RadioButtons(
    radio_ax2,
    list(PRESETS.keys()),
    active=1,   # Stable Spiral
    activecolor='#e94560',
)
for lbl in radio.labels:
    lbl.set_color('white')
    lbl.set_fontsize(7.5)

fig.text(0.001, 0.955, 'Presets', color='#8be9fd', fontsize=8, fontweight='bold')

# ---------------------------------------------------------------------------
# Panel: Trace-Determinant stability diagram (left)
# ---------------------------------------------------------------------------
TAU_LIM = (-4, 4)
DET_LIM = (-4, 8)

def draw_trace_det_background(ax_td):
    ax_td.cla()
    ax_td.set_facecolor('#0f3460')
    ax_td.tick_params(colors='white', labelsize=8)
    for sp in ax_td.spines.values():
        sp.set_edgecolor('#8be9fd')
        sp.set_linewidth(0.7)

    tau_arr = np.linspace(TAU_LIM[0], TAU_LIM[1], 400)
    det_arr = np.linspace(DET_LIM[0], DET_LIM[1], 400)
    TAU, DET = np.meshgrid(tau_arr, det_arr)
    DISC = TAU ** 2 - 4 * DET

    # Region masks
    saddle      = DET < 0
    stab_spiral = (DET > 0) & (TAU < 0) & (DISC < 0)
    stab_node   = (DET > 0) & (TAU < 0) & (DISC >= 0)
    unst_spiral = (DET > 0) & (TAU > 0) & (DISC < 0)
    unst_node   = (DET > 0) & (TAU > 0) & (DISC >= 0)

    def fill_region(mask, color, alpha=0.18):
        z = np.zeros_like(TAU)
        z[mask] = 1.0
        ax_td.contourf(tau_arr, det_arr, z, levels=[0.5, 1.5],
                       colors=[color], alpha=alpha)

    fill_region(saddle,       '#f5a623', alpha=0.22)
    fill_region(stab_spiral,  '#8be9fd', alpha=0.18)
    fill_region(stab_node,    '#50fa7b', alpha=0.18)
    fill_region(unst_spiral,  '#ff79c6', alpha=0.18)
    fill_region(unst_node,    '#e94560', alpha=0.18)

    # Parabola τ²=4Δ
    tau_p = np.linspace(TAU_LIM[0], TAU_LIM[1], 400)
    det_p = tau_p ** 2 / 4
    ax_td.plot(tau_p, det_p, color='white', linewidth=1.2, alpha=0.6, zorder=3,
               label='τ²=4Δ (node/spiral)')

    # Reference lines
    ax_td.axhline(0, color='white', alpha=0.3, linewidth=0.8, zorder=2)
    ax_td.axvline(0, color='white', alpha=0.3, linewidth=0.8, zorder=2)

    # Region text annotations
    ax_td.text(-2.8, -2.5, 'Saddle',       color='#f5a623', fontsize=7.5, alpha=0.9, ha='center')
    ax_td.text(-2.5,  5.5, 'Stable\nSpiral', color='#8be9fd', fontsize=7,   alpha=0.9, ha='center')
    ax_td.text(-2.5,  2.0, 'Stable\nNode',   color='#50fa7b', fontsize=7,   alpha=0.9, ha='center')
    ax_td.text( 2.5,  5.5, 'Unstable\nSpiral', color='#ff79c6', fontsize=7, alpha=0.9, ha='center')
    ax_td.text( 2.5,  2.0, 'Unstable\nNode',   color='#e94560', fontsize=7, alpha=0.9, ha='center')

    ax_td.set_xlim(*TAU_LIM)
    ax_td.set_ylim(*DET_LIM)
    ax_td.set_xlabel('Trace  τ = a+d', color='white', fontsize=9)
    ax_td.set_ylabel('Determinant  Δ = ad−bc', color='white', fontsize=9)
    ax_td.set_title('τ-Δ Stability Diagram', color='white', fontsize=10, pad=5)
    ax_td.legend(fontsize=7, loc='upper right', framealpha=0.2,
                 facecolor='#16213e', edgecolor='#8be9fd', labelcolor='white')


# ---------------------------------------------------------------------------
# Panel: Phase portrait (center)
# ---------------------------------------------------------------------------
PHASE_LIM = 3.0
T_SPAN    = (0, 12)
TRAJ_COLORS = plt.cm.cool(np.linspace(0, 1, 8))

IC_ANGLES  = np.linspace(0, 2 * np.pi, 9)[:-1]
IC_RADIUS  = 2.2


def integrate_trajectory(J, x0, y0, t_max=12.0, n_pts=400):
    """Integrate ẋ = Jx from (x0,y0). Stop if norm blows up."""
    def ode(t, xy):
        return J @ xy

    lim = 20.0

    def blow_up(t, xy):
        return lim - np.linalg.norm(xy)
    blow_up.terminal  = True
    blow_up.direction = -1

    sol = solve_ivp(
        ode, [0, t_max], [x0, y0],
        max_step=0.05,
        t_eval=np.linspace(0, t_max, n_pts),
        events=blow_up,
        rtol=1e-6, atol=1e-9,
    )
    return sol.y[0], sol.y[1]


def draw_phase_portrait(ax_ph, J, classification, cls_color, eigvals):
    ax_ph.cla()
    ax_ph.set_facecolor('#0f3460')
    ax_ph.tick_params(colors='white', labelsize=8)
    for sp in ax_ph.spines.values():
        sp.set_edgecolor('#8be9fd')
        sp.set_linewidth(0.7)

    # Vector field (quiver)
    n_q = 18
    qx = np.linspace(-PHASE_LIM, PHASE_LIM, n_q)
    qy = np.linspace(-PHASE_LIM, PHASE_LIM, n_q)
    QX, QY = np.meshgrid(qx, qy)
    UV = J @ np.vstack([QX.ravel(), QY.ravel()])
    U, V = UV[0].reshape(n_q, n_q), UV[1].reshape(n_q, n_q)
    NORM = np.sqrt(U**2 + V**2) + 1e-12
    ax_ph.quiver(QX, QY, U / NORM, V / NORM,
                 color='white', alpha=0.18, scale=28, width=0.003,
                 zorder=2)

    # Trajectories
    for i, theta in enumerate(IC_ANGLES):
        x0 = IC_RADIUS * np.cos(theta)
        y0 = IC_RADIUS * np.sin(theta)
        xs, ys = integrate_trajectory(J, x0, y0)
        c = TRAJ_COLORS[i % len(TRAJ_COLORS)]
        ax_ph.plot(xs, ys, color=c, linewidth=1.1, alpha=0.75, zorder=3)
        # Arrowhead near end
        if len(xs) > 5:
            ax_ph.annotate(
                '', xy=(xs[-1], ys[-1]), xytext=(xs[-3], ys[-3]),
                arrowprops=dict(arrowstyle='->', color=c, lw=1.0),
                zorder=4,
            )

    # Fixed point at origin
    ax_ph.scatter([0], [0], s=100, c=cls_color, edgecolors='white',
                  linewidths=1.0, zorder=6)

    ev_str = ', '.join(
        f'λ{i+1}={lv.real:+.2f}{"" if abs(lv.imag)<1e-6 else f"{lv.imag:+.2f}i"}'
        for i, lv in enumerate(eigvals)
    )
    ax_ph.set_xlim(-PHASE_LIM, PHASE_LIM)
    ax_ph.set_ylim(-PHASE_LIM, PHASE_LIM)
    ax_ph.axhline(0, color='white', alpha=0.2, linewidth=0.6)
    ax_ph.axvline(0, color='white', alpha=0.2, linewidth=0.6)
    ax_ph.set_xlabel('x', color='white', fontsize=9)
    ax_ph.set_ylabel('y', color='white', fontsize=9)
    ax_ph.set_title(
        f'Phase Portrait\n{classification}\n{ev_str}',
        color=cls_color, fontsize=9, pad=4,
        fontweight='bold',
    )


# ---------------------------------------------------------------------------
# Panel: Eigenvalue complex plane (right)
# ---------------------------------------------------------------------------
EIG_LIM = 4.0

def draw_eigenvalue_plane(ax_ev, eigvals, cls_color):
    ax_ev.cla()
    ax_ev.set_facecolor('#0f3460')
    ax_ev.tick_params(colors='white', labelsize=8)
    for sp in ax_ev.spines.values():
        sp.set_edgecolor('#8be9fd')
        sp.set_linewidth(0.7)

    # Stable/unstable half-plane tints
    ax_ev.axvspan(-EIG_LIM, 0, alpha=0.10, color='#50fa7b', zorder=0)
    ax_ev.axvspan(0, EIG_LIM, alpha=0.10, color='#e94560', zorder=0)

    # Axes
    ax_ev.axhline(0, color='white', alpha=0.35, linewidth=0.8, zorder=1)
    ax_ev.axvline(0, color='white', alpha=0.35, linewidth=0.8, zorder=1)

    # Labels
    ax_ev.text(-EIG_LIM * 0.55, EIG_LIM * 0.88, 'stable',
               color='#50fa7b', alpha=0.6, fontsize=8, ha='center')
    ax_ev.text( EIG_LIM * 0.55, EIG_LIM * 0.88, 'unstable',
               color='#e94560', alpha=0.6, fontsize=8, ha='center')

    # Conjugate pair line
    if len(eigvals) == 2 and abs(eigvals[0].imag) > 1e-6:
        ax_ev.plot(
            [eigvals[0].real, eigvals[1].real],
            [eigvals[0].imag, eigvals[1].imag],
            color='white', alpha=0.3, linewidth=1.0, zorder=2,
            linestyle='--',
        )

    # Plot eigenvalues
    for i, lv in enumerate(eigvals):
        re, im = lv.real, lv.imag
        if re < -1e-9:
            ec = '#50fa7b'
        elif re > 1e-9:
            ec = '#e94560'
        else:
            ec = '#ffb86c'

        ax_ev.scatter([re], [im], s=160, c=ec,
                      edgecolors='white', linewidths=1.2, zorder=5)

        im_str = '' if abs(im) < 1e-6 else f'{im:+.2f}i'
        label  = f'λ{i+1} = {re:+.2f}{im_str}'
        offset_y = 0.35 if im >= 0 else -0.45
        ax_ev.text(re + 0.15, im + offset_y, label,
                   color='white', fontsize=8, zorder=6,
                   bbox=dict(facecolor='#1a1a2e', alpha=0.55, boxstyle='round,pad=0.2',
                             edgecolor='none'))

    ax_ev.set_xlim(-EIG_LIM, EIG_LIM)
    ax_ev.set_ylim(-EIG_LIM, EIG_LIM)
    ax_ev.set_xlabel('Re(λ)', color='white', fontsize=9)
    ax_ev.set_ylabel('Im(λ)', color='white', fontsize=9)
    ax_ev.set_title('Eigenvalue Plane', color='white', fontsize=10, pad=5)
    ax_ev.set_aspect('equal', adjustable='box')


# ---------------------------------------------------------------------------
# Matrix display text
# ---------------------------------------------------------------------------
mat_text = fig.text(
    0.5, 0.965,
    'J = [[a, b], [c, d]]',
    color='white', fontsize=11, ha='center', va='top',
    fontfamily='monospace',
    bbox=dict(facecolor='#16213e', alpha=0.7, boxstyle='round,pad=0.4',
              edgecolor='#8be9fd', linewidth=0.8),
)

# ---------------------------------------------------------------------------
# Main update function
# ---------------------------------------------------------------------------
def update(val=None):
    a = sl_a.val
    b = sl_b.val
    c = sl_c.val
    d = sl_d.val

    J, eigvals, trace, det, classification, cls_color = analyze_matrix(a, b, c, d)

    # Matrix display
    mat_text.set_text(
        f'J = [[{a:+.2f}, {b:+.2f}],       Trace = {trace:+.3f}'
        f'\n     [{c:+.2f}, {d:+.2f}]]      Det   = {det:+.3f}'
        f'       →  {classification}'
    )
    mat_text.set_color(cls_color)

    # Left: trace-det diagram
    draw_trace_det_background(ax_trace_det)
    ax_trace_det.scatter([trace], [det], s=160, c=cls_color,
                         edgecolors='white', linewidths=1.2, zorder=10)

    # Center: phase portrait
    draw_phase_portrait(ax_phase, J, classification, cls_color, eigvals)

    # Right: eigenvalue plane
    draw_eigenvalue_plane(ax_eigen, eigvals, cls_color)

    fig.canvas.draw_idle()


# ---------------------------------------------------------------------------
# Preset callback
# ---------------------------------------------------------------------------
_ignore_radio = [False]   # prevent recursive callbacks

def on_preset(label):
    if _ignore_radio[0]:
        return
    a, b, c, d = PRESETS[label]
    _ignore_radio[0] = True
    sl_a.set_val(a)
    sl_b.set_val(b)
    sl_c.set_val(c)
    sl_d.set_val(d)
    _ignore_radio[0] = False
    update()


# ---------------------------------------------------------------------------
# Wire up callbacks
# ---------------------------------------------------------------------------
for sl in sliders:
    sl.on_changed(update)

radio.on_clicked(on_preset)

# ---------------------------------------------------------------------------
# Initial draw
# ---------------------------------------------------------------------------
update()

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
if __name__ == '__main__':
    print(
        "\nEigenvalue / Stability Explorer"
        "\n================================"
        "\nTier 0.1 — Mathematics: Linear Algebra meets ODE Stability"
        "\n"
        "\nControls:"
        "\n  Sliders a, b, c, d  — set entries of 2×2 Jacobian J = [[a,b],[c,d]]"
        "\n  Preset radio buttons — load classic systems (stable spiral, saddle, etc.)"
        "\n"
        "\nPanels (left → right):"
        "\n  τ-Δ Diagram   — trace-determinant stability regions; current point shown"
        "\n  Phase Portrait — vector field + trajectories for ẋ = Jx"
        "\n  Eigenvalue Plane — Re/Im of λ₁, λ₂; green=stable, red=unstable"
        "\n"
        "\nClose the window to exit.\n"
    )
    plt.show()
