"""
ODE Phase Plane Explorer
========================
Tier 0.1 — Mathematics: ODEs, stability analysis, bifurcation theory

Concepts demonstrated:
  - Vector fields and nullclines in 2D phase space
  - Fixed point detection via nullcline intersections
  - Stability via Jacobian eigenvalues
  - Trajectory simulation from multiple initial conditions
  - Biological examples: toggle switch, predator-prey, activator-inhibitor

Usage:
    python 01_ode_phase_plane.py

Controls:
    Use the radio buttons to switch between built-in biological systems.
    Click anywhere on the phase plane to add a trajectory from that point.
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import RadioButtons, Button, Slider
from scipy.integrate import solve_ivp
from scipy.optimize import fsolve


# ── Biological ODE Systems ────────────────────────────────────────────────────

SYSTEMS = {}

def register(name, xlim, ylim, xlabel, ylabel, description):
    """Decorator that registers a (rhs, params) factory as a named system."""
    def decorator(fn):
        SYSTEMS[name] = dict(fn=fn, xlim=xlim, ylim=ylim,
                             xlabel=xlabel, ylabel=ylabel, description=description)
        return fn
    return decorator


@register("Toggle Switch", (0, 5), (0, 5), "[Protein A]", "[Protein B]",
          "Mutual repression bistable switch (Gardner et al. 2000)\n"
          "du/dt = α/(1+vⁿ) – u,   dv/dt = α/(1+uⁿ) – v")
def toggle_switch(x, y, alpha=3.0, n=2.5):
    du = alpha / (1 + y**n) - x
    dv = alpha / (1 + x**n) - y
    return du, dv


@register("Lotka-Volterra", (0, 8), (0, 6), "[Prey]", "[Predator]",
          "Predator-prey oscillations\n"
          "dx/dt = ax – bxy,   dy/dt = –cy + dxy")
def lotka_volterra(x, y, a=1.0, b=0.2, c=1.0, d=0.15):
    dx = a*x - b*x*y
    dy = -c*y + d*x*y
    return dx, dy


@register("Activator-Inhibitor", (0, 4), (0, 4), "[Activator]", "[Inhibitor]",
          "Positive-negative feedback (Gierer-Meinhardt skeleton)\n"
          "du/dt = u²/v – u + σ,   dv/dt = u² – v")
def activator_inhibitor(x, y, sigma=0.5):
    dx = x**2 / (y + 1e-9) - x + sigma
    dy = x**2 - y
    return dx, dy


@register("FitzHugh-Nagumo", (-2.5, 2.5), (-1.0, 2.0), "Voltage (v)", "Recovery (w)",
          "Excitable cell / action potential skeleton\n"
          "dv/dt = v – v³/3 – w + I,   dw/dt = ε(v + a – bw)")
def fitzhugh_nagumo(x, y, I=0.5, eps=0.08, a=0.7, b=0.8):
    dv = x - x**3/3 - y + I
    dw = eps * (x + a - b*y)
    return dv, dw


@register("Hopf Oscillator", (-3, 3), (-3, 3), "x", "y",
          "Supercritical Hopf bifurcation — stable limit cycle\n"
          "dx/dt = μx – y – x(x²+y²),   dy/dt = x + μy – y(x²+y²)")
def hopf_oscillator(x, y, mu=1.0):
    r2 = x**2 + y**2
    dx = mu*x - y - x*r2
    dy = x + mu*y - y*r2
    return dx, dy


# ── Core Analysis Functions ───────────────────────────────────────────────────

def compute_vector_field(system_name, n_grid=24):
    """Return meshgrid + vector field for the selected system."""
    s = SYSTEMS[system_name]
    x = np.linspace(*s['xlim'], n_grid)
    y = np.linspace(*s['ylim'], n_grid)
    X, Y = np.meshgrid(x, y)
    U, V = s['fn'](X, Y)
    speed = np.hypot(U, V)
    speed[speed == 0] = 1e-12
    return X, Y, U / speed, V / speed, speed


def compute_nullclines(system_name, n=400):
    """Compute x-nullcline (du/dt=0) and y-nullcline (dv/dt=0) curves."""
    s = SYSTEMS[system_name]
    xs = np.linspace(*s['xlim'], n)
    ys = np.linspace(*s['ylim'], n)
    X, Y = np.meshgrid(xs, ys)
    U, V = s['fn'](X, Y)
    return xs, ys, U, V


def find_fixed_points(system_name, n_starts=8):
    """Locate fixed points by solving RHS=0 from multiple starting points."""
    s = SYSTEMS[system_name]
    fixed_pts = []
    tried = set()
    xl, xh = s['xlim']
    yl, yh = s['ylim']

    def rhs(p):
        u, v = s['fn'](p[0], p[1])
        return [u, v]

    for xi in np.linspace(xl + 0.1, xh - 0.1, n_starts):
        for yi in np.linspace(yl + 0.1, yh - 0.1, n_starts):
            try:
                sol = fsolve(rhs, [xi, yi], full_output=True)
                pt = tuple(np.round(sol[0], 5))
                if sol[2] == 1 and xl <= pt[0] <= xh and yl <= pt[1] <= yh:
                    if pt not in tried:
                        tried.add(pt)
                        fixed_pts.append(sol[0])
            except Exception:
                pass
    return fixed_pts


def classify_fixed_point(system_name, pt, eps=1e-6):
    """Classify stability via Jacobian eigenvalues at a fixed point."""
    s = SYSTEMS[system_name]
    x, y = pt

    def F(p):
        u, v = s['fn'](p[0], p[1])
        return np.array([u, v])

    J = np.array([
        [(F([x+eps, y])[0] - F([x-eps, y])[0]) / (2*eps),
         (F([x, y+eps])[0] - F([x, y-eps])[0]) / (2*eps)],
        [(F([x+eps, y])[1] - F([x-eps, y])[1]) / (2*eps),
         (F([x, y+eps])[1] - F([x, y-eps])[1]) / (2*eps)],
    ])
    eigvals = np.linalg.eigvals(J)
    real_parts = eigvals.real

    if all(r < 0 for r in real_parts):
        if any(abs(v.imag) > 1e-6 for v in eigvals):
            return "Stable Spiral", "green", eigvals
        return "Stable Node", "blue", eigvals
    elif all(r > 0 for r in real_parts):
        if any(abs(v.imag) > 1e-6 for v in eigvals):
            return "Unstable Spiral", "red", eigvals
        return "Unstable Node", "red", eigvals
    elif np.prod(real_parts) < 0:
        return "Saddle Point", "orange", eigvals
    return "Unknown", "gray", eigvals


def integrate_trajectory(system_name, x0, y0, t_max=50, n_pts=3000):
    """Integrate ODE forward and backward from initial condition."""
    s = SYSTEMS[system_name]

    def rhs(t, y):
        u, v = s['fn'](y[0], y[1])
        return [u, v]

    xl, xh = s['xlim']
    yl, yh = s['ylim']

    def hit_boundary(t, y):
        return min(y[0]-xl, xh-y[0], y[1]-yl, yh-y[1])
    hit_boundary.terminal = True

    t_span = (0, t_max)
    t_eval = np.linspace(0, t_max, n_pts)
    sol = solve_ivp(rhs, t_span, [x0, y0], t_eval=t_eval,
                    method='RK45', rtol=1e-8, events=hit_boundary)
    return sol.y[0], sol.y[1]


# ── Plotting ──────────────────────────────────────────────────────────────────

class PhasePlaneExplorer:
    def __init__(self):
        self.current = list(SYSTEMS.keys())[0]
        self.trajectories = []

        self.fig = plt.figure(figsize=(14, 8))
        self.fig.patch.set_facecolor('#1a1a2e')
        gs = gridspec.GridSpec(1, 3, width_ratios=[1, 3, 1.2], wspace=0.3)

        # Left: radio + info
        ax_radio = self.fig.add_axes([0.01, 0.45, 0.15, 0.4])
        ax_radio.set_facecolor('#16213e')
        self.radio = RadioButtons(ax_radio, list(SYSTEMS.keys()),
                                  activecolor='#e94560')
        self.radio.on_clicked(self._on_system_change)

        ax_clear = self.fig.add_axes([0.02, 0.35, 0.13, 0.06])
        self.btn_clear = Button(ax_clear, 'Clear Trajectories',
                                color='#16213e', hovercolor='#e94560')
        self.btn_clear.on_clicked(self._clear_trajectories)

        # Center: phase plane
        self.ax = self.fig.add_subplot(gs[1])
        self.ax.set_facecolor('#0f3460')

        # Right: info panel
        self.ax_info = self.fig.add_subplot(gs[2])
        self.ax_info.set_facecolor('#16213e')
        self.ax_info.axis('off')

        self.fig.canvas.mpl_connect('button_press_event', self._on_click)
        self.fig.suptitle('ODE Phase Plane Explorer', color='white',
                          fontsize=14, fontweight='bold')

        self.draw()
        plt.show()

    def draw(self):
        self.ax.cla()
        self.ax_info.cla()
        self.ax_info.axis('off')
        s = SYSTEMS[self.current]

        # Vector field
        X, Y, U, V, speed = compute_vector_field(self.current)
        self.ax.quiver(X, Y, U, V,
                       color=plt.cm.cool(speed / speed.max()),
                       alpha=0.55, scale=28, width=0.003)

        # Nullclines
        xs, ys, Uf, Vf = compute_nullclines(self.current)
        Xg, Yg = np.meshgrid(xs, ys)
        self.ax.contour(Xg, Yg, Uf, levels=[0], colors=['#f5a623'], linewidths=2,
                        linestyles='--', alpha=0.85)
        self.ax.contour(Xg, Yg, Vf, levels=[0], colors=['#50fa7b'], linewidths=2,
                        linestyles=':', alpha=0.85)

        # Fixed points
        fps = find_fixed_points(self.current)
        info_lines = [f"System: {self.current}\n",
                      s['description'] + "\n\n",
                      "Fixed Points:\n"]
        for pt in fps:
            label, color, eigvals = classify_fixed_point(self.current, pt)
            marker = 'o' if 'Stable' in label else ('^' if 'Saddle' in label else 's')
            self.ax.plot(*pt, marker, color=color, markersize=11, zorder=5,
                         markeredgecolor='white', markeredgewidth=0.8)
            info_lines.append(
                f"  ({pt[0]:.2f}, {pt[1]:.2f})\n"
                f"  {label}\n"
                f"  λ = {eigvals[0]:.3f}, {eigvals[1]:.3f}\n\n"
            )

        # Stored trajectories
        for tx, ty in self.trajectories:
            self.ax.plot(tx, ty, '-', color='#ff79c6', lw=1.2, alpha=0.8)
            self.ax.plot(tx[0], ty[0], 'w.', markersize=5)

        # Legend
        from matplotlib.lines import Line2D
        legend_elements = [
            Line2D([0], [0], color='#f5a623', lw=2, ls='--', label='x-nullcline (du/dt=0)'),
            Line2D([0], [0], color='#50fa7b', lw=2, ls=':', label='y-nullcline (dv/dt=0)'),
            Line2D([0], [0], color='#ff79c6', lw=1.5, label='Trajectories'),
        ]
        self.ax.legend(handles=legend_elements, loc='upper right',
                       facecolor='#16213e', edgecolor='gray',
                       labelcolor='white', fontsize=8)

        self.ax.set_xlim(*s['xlim'])
        self.ax.set_ylim(*s['ylim'])
        self.ax.set_xlabel(s['xlabel'], color='white')
        self.ax.set_ylabel(s['ylabel'], color='white')
        self.ax.set_title(f'{self.current} — Click to add trajectories',
                          color='white', fontsize=10)
        self.ax.tick_params(colors='white')
        for sp in self.ax.spines.values():
            sp.set_edgecolor('#888')

        # Info panel
        info_text = ''.join(info_lines)
        self.ax_info.text(0.05, 0.97, info_text, transform=self.ax_info.transAxes,
                          va='top', fontsize=7.5, color='white',
                          fontfamily='monospace', wrap=True)
        self.ax_info.text(0.05, 0.12,
                          "Legend:\n"
                          "● Blue = Stable Node\n"
                          "● Green = Stable Spiral\n"
                          "▲ Orange = Saddle Point\n"
                          "■ Red = Unstable",
                          transform=self.ax_info.transAxes,
                          va='bottom', fontsize=7.5, color='#ccc')

        self.fig.canvas.draw_idle()

    def _on_system_change(self, label):
        self.current = label
        self.trajectories = []
        self.draw()

    def _on_click(self, event):
        if event.inaxes != self.ax:
            return
        x0, y0 = event.xdata, event.ydata
        tx, ty = integrate_trajectory(self.current, x0, y0)
        self.trajectories.append((tx, ty))
        self.draw()

    def _clear_trajectories(self, event):
        self.trajectories = []
        self.draw()


if __name__ == '__main__':
    print("ODE Phase Plane Explorer")
    print("=" * 40)
    print("Click on the phase plane to launch trajectories.")
    print("Use the radio buttons on the left to switch systems.")
    print()
    for name, s in SYSTEMS.items():
        print(f"  {name}: {s['description'].split(chr(10))[0]}")
    print()
    PhasePlaneExplorer()
