"""
Bifurcation Diagram Builder
============================
Tier 0.1 — Mathematics: Bifurcation theory

Concepts demonstrated:
  - Saddle-node bifurcation (bistability appears / disappears)
  - Pitchfork bifurcation (symmetry breaking)
  - Hopf bifurcation (oscillations emerge)
  - Hysteresis: sweeping parameter forward vs. backward
  - How bistable genetic switches arise from nonlinear feedback

Usage:
    python 02_bifurcation_diagram.py

Controls:
    Left panel: 1D bifurcation diagrams with parameter slider for 3 canonical systems.
    Right panel: live phase-line portrait updates as you change the parameter.

Biological relevance:
    Bistable toggle switches undergo saddle-node bifurcations.
    Cell cycle checkpoints use Hopf-like transitions.
    Gene regulatory circuits near bifurcation points show ultrasensitivity.
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import Slider, RadioButtons
from scipy.integrate import solve_ivp
from scipy.optimize import brentq


# ── 1-D Dynamical Systems ─────────────────────────────────────────────────────

def saddle_node(x, r):
    """f(x) = r - x²  →  two fixed pts for r>0, none for r<0, saddle at r=0."""
    return r - x**2


def pitchfork(x, r):
    """f(x) = rx - x³  →  stable at 0 for r<0; two stables + one unstable for r>0."""
    return r * x - x**3


def bistable_switch(x, r, n=3, K=1.0):
    """Gene regulatory switch: activation + degradation.
    dx/dt = r·xⁿ/(Kⁿ+xⁿ) - x
    Bistable for r large enough, monostable otherwise."""
    return r * x**n / (K**n + x**n) - x


def hopf_radius(r2, mu):
    """Hopf normal form: dr²/dt = 2μr² - 2r⁴  →  limit cycle at r²=μ for μ>0."""
    return 2 * mu * r2 - 2 * r2**2


SYSTEMS = {
    "Saddle-Node": {
        "fn": saddle_node,
        "r_range": (-1.5, 1.5),
        "x_range": (-2.0, 2.0),
        "r_label": "parameter r",
        "x_label": "x",
        "description": (
            "Saddle-Node Bifurcation\n"
            "f(x) = r – x²\n\n"
            "r < 0 : No fixed points\n"
            "r = 0 : Bifurcation point\n"
            "r > 0 : Two fixed points\n"
            "        (one stable, one unstable)\n\n"
            "Biology: toggles, memory devices\n"
            "appear via this mechanism."
        ),
    },
    "Pitchfork": {
        "fn": pitchfork,
        "r_range": (-2.0, 2.0),
        "x_range": (-2.0, 2.0),
        "r_label": "parameter r",
        "x_label": "x",
        "description": (
            "Pitchfork Bifurcation\n"
            "f(x) = rx – x³\n\n"
            "r < 0 : Single stable fixed point\n"
            "r = 0 : Flat top at x=0\n"
            "r > 0 : x=0 unstable; two\n"
            "        new stables at ±√r\n\n"
            "Biology: symmetry-breaking\n"
            "events, cell polarization."
        ),
    },
    "Bistable Switch": {
        "fn": bistable_switch,
        "r_range": (0.5, 4.0),
        "x_range": (0.0, 3.5),
        "r_label": "synthesis rate r",
        "x_label": "[Protein]",
        "description": (
            "Bistable Gene Switch\n"
            "dx/dt = r·x³/(1+x³) – x\n\n"
            "Low r  : Single stable OFF state\n"
            "r_lower: Saddle-node — ON state appears\n"
            "r_upper: Saddle-node — OFF state disappears\n\n"
            "Hysteresis loop between the two\n"
            "saddle-node bifurcation points.\n\n"
            "Biology: GATA/PU.1 switch,\n"
            "lac operon bistability."
        ),
    },
}


def trace_bifurcation_diagram(system_name, n_r=600):
    """
    Trace stable and unstable branches by sweeping parameter r.
    Returns r_vals, stable_branches [(r, x)...], unstable_branches [(r, x)...]
    """
    s = SYSTEMS[system_name]
    fn = s["fn"]
    r_vals = np.linspace(*s["r_range"], n_r)
    x_vals = np.linspace(*s["x_range"], 2000)

    stable = []
    unstable = []

    for r in r_vals:
        f_vals = fn(x_vals, r)
        # Find sign changes → fixed points
        for i in range(len(f_vals) - 1):
            if f_vals[i] * f_vals[i + 1] < 0:
                x_fp = brentq(lambda x: fn(x, r), x_vals[i], x_vals[i + 1])
                # Stability: df/dx < 0 → stable
                dx = 1e-6
                slope = (fn(x_fp + dx, r) - fn(x_fp - dx, r)) / (2 * dx)
                if slope < 0:
                    stable.append((r, x_fp))
                else:
                    unstable.append((r, x_fp))

    return r_vals, stable, unstable


# ── Plotting ──────────────────────────────────────────────────────────────────

class BifurcationExplorer:
    def __init__(self):
        self.current = "Saddle-Node"
        self._build_figure()
        self._draw_all()
        plt.show()

    def _build_figure(self):
        self.fig = plt.figure(figsize=(16, 9))
        self.fig.patch.set_facecolor('#1a1a2e')
        self.fig.suptitle('Bifurcation Diagram Builder', color='white',
                          fontsize=14, fontweight='bold')

        # Layout
        gs = gridspec.GridSpec(2, 3, figure=self.fig,
                               left=0.05, right=0.97, top=0.88, bottom=0.12,
                               hspace=0.4, wspace=0.35)

        self.ax_bif   = self.fig.add_subplot(gs[0, :2])  # bifurcation diagram
        self.ax_phase = self.fig.add_subplot(gs[1, :2])  # phase-line portrait
        self.ax_info  = self.fig.add_subplot(gs[:, 2])   # text info

        for ax in [self.ax_bif, self.ax_phase, self.ax_info]:
            ax.set_facecolor('#0f3460')

        self.ax_info.axis('off')

        # Radio buttons
        ax_radio = self.fig.add_axes([0.01, 0.7, 0.10, 0.20])
        ax_radio.set_facecolor('#16213e')
        self.radio = RadioButtons(ax_radio, list(SYSTEMS.keys()), activecolor='#e94560')
        self.radio.on_clicked(self._on_system)

        # Parameter slider
        ax_slider = self.fig.add_axes([0.07, 0.04, 0.55, 0.025])
        ax_slider.set_facecolor('#16213e')
        s = SYSTEMS[self.current]
        self.slider = Slider(ax_slider, 'r  →', *s['r_range'],
                             valinit=(s['r_range'][0] + s['r_range'][1]) / 2,
                             color='#e94560')
        self.slider.label.set_color('white')
        self.slider.valtext.set_color('white')
        self.slider.on_changed(self._on_slider)

    def _on_system(self, label):
        self.current = label
        s = SYSTEMS[label]
        self.slider.valmin = s['r_range'][0]
        self.slider.valmax = s['r_range'][1]
        self.slider.val = (s['r_range'][0] + s['r_range'][1]) / 2
        self.slider.ax.set_xlim(*s['r_range'])
        self._draw_all()

    def _on_slider(self, val):
        self._draw_phase(val)
        self.fig.canvas.draw_idle()

    def _draw_all(self):
        self._draw_bifurcation()
        self._draw_phase(self.slider.val)
        self._draw_info()
        self.fig.canvas.draw_idle()

    def _draw_bifurcation(self):
        ax = self.ax_bif
        ax.cla()
        ax.set_facecolor('#0f3460')
        s = SYSTEMS[self.current]

        r_vals, stable, unstable = trace_bifurcation_diagram(self.current)

        if stable:
            rs, xs = zip(*stable)
            ax.plot(rs, xs, '.', color='#50fa7b', ms=2, label='Stable fixed point')
        if unstable:
            ru, xu = zip(*unstable)
            ax.plot(ru, xu, '.', color='#ff5555', ms=2, label='Unstable fixed point')

        # Mark current r
        ax.axvline(self.slider.val, color='#f8f8f2', lw=1.2, alpha=0.6, ls='--')

        ax.set_xlabel(s['r_label'], color='white')
        ax.set_ylabel(s['x_label'], color='white')
        ax.set_title('Bifurcation Diagram  (stable=green, unstable=red)',
                     color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_phase(self, r_val):
        ax = self.ax_phase
        ax.cla()
        ax.set_facecolor('#0f3460')
        s = SYSTEMS[self.current]
        fn = s['fn']

        x_vals = np.linspace(*s['x_range'], 600)
        f_vals = fn(x_vals, r_val)

        ax.axhline(0, color='#888', lw=0.8, zorder=0)
        ax.plot(x_vals, f_vals, color='#8be9fd', lw=2, label=f'f(x) at r={r_val:.3f}')
        ax.fill_between(x_vals, 0, f_vals, where=f_vals > 0,
                        color='#50fa7b', alpha=0.25, label='dx/dt > 0 (→)')
        ax.fill_between(x_vals, 0, f_vals, where=f_vals < 0,
                        color='#ff5555', alpha=0.25, label='dx/dt < 0 (←)')

        # Arrows showing direction
        for xi in np.linspace(*s['x_range'], 18):
            fi = fn(xi, r_val)
            sign = np.sign(fi)
            if abs(fi) > 1e-4:
                ax.annotate('', xy=(xi + 0.08 * sign, 0), xytext=(xi, 0),
                            arrowprops=dict(arrowstyle='->', color='white', lw=0.7))

        # Fixed points
        for i in range(len(x_vals) - 1):
            if f_vals[i] * f_vals[i + 1] < 0:
                x_fp = brentq(fn, x_vals[i], x_vals[i + 1], args=(r_val,))
                dx = 1e-6
                slope = (fn(x_fp + dx, r_val) - fn(x_fp - dx, r_val)) / (2 * dx)
                color = '#50fa7b' if slope < 0 else '#ff5555'
                marker = 'o' if slope < 0 else 's'
                ax.plot(x_fp, 0, marker, color=color, ms=10,
                        markeredgecolor='white', markeredgewidth=0.8, zorder=5)

        ax.set_xlabel(s['x_label'], color='white')
        ax.set_ylabel('dx/dt', color='white')
        ax.set_title(f'Phase-line portrait at {s["r_label"]} = {r_val:.3f}',
                     color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7, facecolor='#16213e', labelcolor='white', loc='upper right')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_info(self):
        ax = self.ax_info
        ax.cla()
        ax.axis('off')
        ax.set_facecolor('#0f3460')
        text = SYSTEMS[self.current]['description']
        ax.text(0.05, 0.95, text, transform=ax.transAxes,
                va='top', fontsize=9, color='white', fontfamily='monospace',
                linespacing=1.6)
        ax.text(0.05, 0.08,
                "Drag the r slider below to\n"
                "move through the bifurcation.\n\n"
                "● Green = stable\n"
                "■ Red  = unstable",
                transform=ax.transAxes, va='bottom', fontsize=8.5,
                color='#aaa')


if __name__ == '__main__':
    print("Bifurcation Diagram Builder")
    print("=" * 40)
    print("Explore three canonical 1D bifurcations:")
    for name in SYSTEMS:
        print(f"  {name}: {SYSTEMS[name]['description'].split(chr(10))[0]}")
    print()
    print("Use the slider to move through parameter space.")
    print("Watch how fixed points appear, disappear, and change stability.")
    print()
    BifurcationExplorer()
