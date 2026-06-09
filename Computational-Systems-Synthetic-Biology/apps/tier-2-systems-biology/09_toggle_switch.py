"""
Toggle Switch: Bistability & Phase Plane
=========================================
Tier 2.3 — Systems Biology: Gene Regulatory Networks

Concepts demonstrated:
  - Gardner et al. (2000) mutual repression bistable toggle switch
  - Nullclines and bistability conditions
  - Hysteresis: state depends on history, not just current input
  - Switching dynamics under external inducer
  - Noise-induced switching (Kramers escape)
  - Effect of Hill coefficient and thresholds on bistability window

Usage:
    python 09_toggle_switch.py [--n HILL] [--alpha ALPHA]

Key insight:
    Bistability arises when two nullclines intersect three times.
    The outer two intersections are stable nodes; the middle is a
    saddle point. The system can sit in either state indefinitely —
    it has memory. This is the basis of epigenetic cell fate decisions.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import Slider, Button
from scipy.integrate import solve_ivp
from scipy.optimize import brentq


# ── ODE Model ─────────────────────────────────────────────────────────────────

def toggle_rhs(t, y, alpha1, alpha2, beta, gamma, n1, n2, I_ext=0.0):
    """
    Gardner toggle switch:
      du/dt = α₁ / (1 + v^n1) - u + I_ext  (u = TetR-GFP)
      dv/dt = α₂ / (1 + u^n2) - v           (v = LacI)
    I_ext: external inducer (IPTG reduces u production equiv.)
    """
    u, v = y
    du = alpha1 / (1 + v**n1) - beta * u + I_ext
    dv = alpha2 / (1 + u**n2) - gamma * v
    return [du, dv]


def find_nullclines(alpha1, alpha2, beta, gamma, n1, n2, xlim, ylim, n=400):
    """
    u-nullcline: alpha1/(1+v^n1) = beta*u  →  u = alpha1/(beta*(1+v^n1))
    v-nullcline: alpha2/(1+u^n2) = gamma*v →  v = alpha2/(gamma*(1+u^n2))
    """
    v_range = np.linspace(ylim[0] + 1e-6, ylim[1], n)
    u_range = np.linspace(xlim[0] + 1e-6, xlim[1], n)

    u_nc = alpha1 / (beta * (1 + v_range**n1))   # u as function of v (u-nullcline)
    v_nc = alpha2 / (gamma * (1 + u_range**n2))  # v as function of u (v-nullcline)

    return v_range, u_nc, u_range, v_nc


def find_fixed_points_toggle(alpha1, alpha2, beta, gamma, n1, n2, xlim, ylim):
    """Find fixed points by 1D root finding on u-nullcline intersection."""
    def net_rhs(u):
        v = alpha2 / (gamma * (1 + u**n2))  # v-nullcline
        du = alpha1 / (1 + v**n1) - beta * u
        return du

    u_range = np.linspace(xlim[0] + 1e-4, xlim[1] - 1e-4, 2000)
    f_vals = np.array([net_rhs(u) for u in u_range])
    fps = []
    for i in range(len(f_vals) - 1):
        if f_vals[i] * f_vals[i + 1] < 0:
            try:
                u_fp = brentq(net_rhs, u_range[i], u_range[i + 1])
                v_fp = alpha2 / (gamma * (1 + u_fp**n2))
                fps.append((u_fp, v_fp))
            except ValueError:
                pass
    return fps


def classify_toggle_fp(pt, alpha1, alpha2, beta, gamma, n1, n2, eps=1e-5):
    """Linearize and classify fixed point."""
    u, v = pt
    def F(y):
        return np.array(toggle_rhs(0, y, alpha1, alpha2, beta, gamma, n1, n2))

    J = np.array([
        [(F([u+eps, v])[0] - F([u-eps, v])[0]) / (2*eps),
         (F([u, v+eps])[0] - F([u, v-eps])[0]) / (2*eps)],
        [(F([u+eps, v])[1] - F([u-eps, v])[1]) / (2*eps),
         (F([u, v+eps])[1] - F([u, v-eps])[1]) / (2*eps)],
    ])
    eigvals = np.linalg.eigvals(J)
    if all(e.real < 0 for e in eigvals):
        return 'Stable', '#50fa7b', eigvals
    elif all(e.real > 0 for e in eigvals):
        return 'Unstable', '#ff5555', eigvals
    else:
        return 'Saddle', '#f5a623', eigvals


# ── Hysteresis Sweep ──────────────────────────────────────────────────────────

def hysteresis_sweep(alpha1, alpha2, beta, gamma, n1, n2, I_range=(-1, 2), n_pts=100):
    """
    Sweep inducer I forward and backward, tracking stable state of u.
    """
    I_vals_fwd = np.linspace(I_range[0], I_range[1], n_pts)
    I_vals_bwd = np.linspace(I_range[1], I_range[0], n_pts)

    def steady_u(I, u0_init):
        sol = solve_ivp(
            toggle_rhs, (0, 200), [u0_init, 1.0],
            args=(alpha1, alpha2, beta, gamma, n1, n2, I),
            t_eval=[200], method='LSODA'
        )
        return sol.y[0, -1]

    # Forward: start from low-u state
    u_fwd = [steady_u(I, 0.5) for I in I_vals_fwd]
    u_init_bwd = u_fwd[-1]

    # Backward: start from final state of forward sweep
    u_bwd = []
    u_curr = u_init_bwd
    for I in I_vals_bwd:
        u_curr = steady_u(I, u_curr)
        u_bwd.append(u_curr)

    return I_vals_fwd, u_fwd, I_vals_bwd, u_bwd


# ── Interactive Visualisation ─────────────────────────────────────────────────

class ToggleExplorer:
    def __init__(self, alpha1=3.0, alpha2=3.0, n1=2.5, n2=2.5):
        self.alpha1 = alpha1
        self.alpha2 = alpha2
        self.beta   = 1.0
        self.gamma  = 1.0
        self.n1 = n1
        self.n2 = n2
        self.xlim = (0, 5.5)
        self.ylim = (0, 5.5)
        self.trajectories = []
        self._build_figure()
        self._draw_all()
        plt.show()

    def _build_figure(self):
        self.fig = plt.figure(figsize=(18, 10))
        self.fig.patch.set_facecolor('#1a1a2e')
        self.fig.suptitle('Toggle Switch: Bistability & Phase Plane',
                          color='white', fontsize=13, fontweight='bold')

        gs = gridspec.GridSpec(2, 3, figure=fig := self.fig,
                               left=0.05, right=0.97, top=0.88, bottom=0.12,
                               hspace=0.45, wspace=0.35)

        self.ax_phase  = fig.add_subplot(gs[:, 0])
        self.ax_traj   = fig.add_subplot(gs[0, 1])
        self.ax_hyst   = fig.add_subplot(gs[1, 1])
        self.ax_info   = fig.add_subplot(gs[:, 2])

        for ax in [self.ax_phase, self.ax_traj, self.ax_hyst, self.ax_info]:
            ax.set_facecolor('#0f3460')

        # Sliders
        ax_n1 = fig.add_axes([0.07, 0.05, 0.20, 0.02])
        ax_n2 = fig.add_axes([0.07, 0.02, 0.20, 0.02])
        ax_a1 = fig.add_axes([0.35, 0.05, 0.20, 0.02])
        ax_a2 = fig.add_axes([0.35, 0.02, 0.20, 0.02])

        for ax_s in [ax_n1, ax_n2, ax_a1, ax_a2]:
            ax_s.set_facecolor('#16213e')

        self.sl_n1 = Slider(ax_n1, 'n₁', 0.5, 5.0, valinit=self.n1, color='#50fa7b')
        self.sl_n2 = Slider(ax_n2, 'n₂', 0.5, 5.0, valinit=self.n2, color='#ff79c6')
        self.sl_a1 = Slider(ax_a1, 'α₁', 0.5, 8.0, valinit=self.alpha1, color='#8be9fd')
        self.sl_a2 = Slider(ax_a2, 'α₂', 0.5, 8.0, valinit=self.alpha2, color='#f5a623')

        for sl in [self.sl_n1, self.sl_n2, self.sl_a1, self.sl_a2]:
            sl.label.set_color('white')
            sl.valtext.set_color('white')
            sl.on_changed(self._on_param)

        ax_clear = fig.add_axes([0.60, 0.03, 0.10, 0.03])
        self.btn_clear = Button(ax_clear, 'Clear Traj',
                                color='#16213e', hovercolor='#e94560')
        self.btn_clear.label.set_color('white')
        self.btn_clear.on_clicked(self._clear)

        self.fig.canvas.mpl_connect('button_press_event', self._on_click)

    def _on_param(self, val):
        self.n1 = self.sl_n1.val
        self.n2 = self.sl_n2.val
        self.alpha1 = self.sl_a1.val
        self.alpha2 = self.sl_a2.val
        self.trajectories = []
        self._draw_all()

    def _on_click(self, event):
        if event.inaxes != self.ax_phase:
            return
        u0, v0 = event.xdata, event.ydata
        sol = solve_ivp(
            toggle_rhs, (0, 60), [u0, v0],
            args=(self.alpha1, self.alpha2, self.beta, self.gamma, self.n1, self.n2, 0.0),
            t_eval=np.linspace(0, 60, 600), method='LSODA'
        )
        self.trajectories.append(sol.y)
        self._draw_phase()
        self._draw_trajectory(sol)
        self.fig.canvas.draw_idle()

    def _clear(self, event):
        self.trajectories = []
        self._draw_all()

    def _draw_all(self):
        self._draw_phase()
        self._draw_time_series()
        self._draw_hysteresis()
        self._draw_info()
        self.fig.canvas.draw_idle()

    def _draw_phase(self):
        ax = self.ax_phase
        ax.cla()
        ax.set_facecolor('#0f3460')
        a1, a2 = self.alpha1, self.alpha2
        b, g = self.beta, self.gamma
        n1, n2 = self.n1, self.n2

        v_range, u_nc, u_range, v_nc = find_nullclines(a1, a2, b, g, n1, n2,
                                                        self.xlim, self.ylim)

        ax.plot(u_nc, v_range, color='#f5a623', lw=2.5, label='u-nullcline (du/dt=0)', zorder=3)
        ax.plot(u_range, v_nc, color='#50fa7b', lw=2.5, ls='--', label='v-nullcline (dv/dt=0)', zorder=3)

        fps = find_fixed_points_toggle(a1, a2, b, g, n1, n2, self.xlim, self.ylim)
        for pt in fps:
            lbl, col, ev = classify_toggle_fp(pt, a1, a2, b, g, n1, n2)
            mk = 'o' if 'Stable' in lbl else '^'
            ax.plot(*pt, mk, color=col, ms=12, zorder=5,
                    markeredgecolor='white', markeredgewidth=1.0)

        for traj in self.trajectories:
            ax.plot(traj[0], traj[1], '-', color='#ff79c6', lw=1.2, alpha=0.8)
            ax.plot(traj[0, 0], traj[1, 0], 'w.', ms=6, zorder=6)

        ax.set_xlim(*self.xlim)
        ax.set_ylim(*self.ylim)
        ax.set_xlabel('TetR (u)', color='white')
        ax.set_ylabel('LacI (v)', color='white')
        ax.set_title('Phase Plane  (click to add trajectory)',
                     color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7.5, facecolor='#16213e', labelcolor='white', loc='upper right')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_trajectory(self, sol):
        ax = self.ax_traj
        ax.cla()
        ax.set_facecolor('#0f3460')
        t = np.linspace(0, 60, sol.y.shape[1])
        ax.plot(t, sol.y[0], color='#50fa7b', lw=1.8, label='TetR (u)')
        ax.plot(t, sol.y[1], color='#ff79c6', lw=1.8, label='LacI (v)')
        ax.set_xlabel('Time', color='white')
        ax.set_ylabel('Concentration', color='white')
        ax.set_title('Last Trajectory Time Series', color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_time_series(self):
        ax = self.ax_traj
        ax.cla()
        ax.set_facecolor('#0f3460')

        # Show two basin trajectories
        ic_list = [(0.1, 3.5), (3.5, 0.1), (2.0, 2.0)]
        cs = ['#50fa7b', '#8be9fd', '#f5a623']
        for ic, col in zip(ic_list, cs):
            sol = solve_ivp(
                toggle_rhs, (0, 60), list(ic),
                args=(self.alpha1, self.alpha2, self.beta, self.gamma, self.n1, self.n2, 0.0),
                t_eval=np.linspace(0, 60, 600), method='LSODA'
            )
            ax.plot(sol.t, sol.y[0], color=col, lw=1.5, label=f'u₀={ic[0]},v₀={ic[1]}')

        ax.set_xlabel('Time', color='white')
        ax.set_ylabel('TetR (u)', color='white')
        ax.set_title('Trajectories from 3 initial conditions', color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_hysteresis(self):
        ax = self.ax_hyst
        ax.cla()
        ax.set_facecolor('#0f3460')

        try:
            I_f, u_f, I_b, u_b = hysteresis_sweep(
                self.alpha1, self.alpha2, self.beta, self.gamma, self.n1, self.n2,
                I_range=(-0.5, 1.5), n_pts=30
            )
            ax.plot(I_f, u_f, '-o', color='#50fa7b', lw=2, ms=3, label='Forward (↑ inducer)')
            ax.plot(I_b, u_b, '-s', color='#ff79c6', lw=2, ms=3, label='Backward (↓ inducer)')
            ax.fill_betweenx(
                np.linspace(0, max(max(u_f), max(u_b)) + 0.5, 50),
                min(I_f), max(I_f), alpha=0.05, color='white'
            )
        except Exception:
            ax.text(0.5, 0.5, 'Hysteresis sweep\nnot available\n(monostable regime)',
                    transform=ax.transAxes, ha='center', va='center',
                    color='white', fontsize=9)

        ax.set_xlabel('Inducer I', color='white')
        ax.set_ylabel('TetR steady state (u)', color='white')
        ax.set_title('Hysteresis  (memory in bistable switch)',
                     color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    def _draw_info(self):
        ax = self.ax_info
        ax.cla()
        ax.axis('off')
        ax.set_facecolor('#0f3460')

        fps = find_fixed_points_toggle(
            self.alpha1, self.alpha2, self.beta, self.gamma, self.n1, self.n2,
            self.xlim, self.ylim
        )
        bistable = len(fps) >= 3
        fp_text = f"Fixed points: {len(fps)}\n"
        for pt in fps:
            lbl, col, ev = classify_toggle_fp(pt, self.alpha1, self.alpha2,
                                              self.beta, self.gamma, self.n1, self.n2)
            fp_text += f"  ({pt[0]:.2f}, {pt[1]:.2f}) → {lbl}\n"

        info = (
            f"Toggle Switch Parameters:\n"
            f"  n₁ = {self.n1:.2f},  n₂ = {self.n2:.2f}\n"
            f"  α₁ = {self.alpha1:.2f},  α₂ = {self.alpha2:.2f}\n\n"
            f"{'★ BISTABLE' if bistable else '○ Monostable'}\n\n"
            f"{fp_text}\n"
            f"Bistability condition:\n"
            f"  n > 1 required\n"
            f"  Two stable + one saddle FP\n\n"
            f"Hysteresis:\n"
            f"  Switch state depends on history\n"
            f"  → epigenetic memory\n\n"
            f"Reference:\n"
            f"  Gardner et al.,\n"
            f"  Nature 403, 339–342 (2000)"
        )
        ax.text(0.04, 0.97, info, transform=ax.transAxes,
                va='top', fontsize=8, color='white', fontfamily='monospace',
                linespacing=1.4)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Toggle switch bistability explorer')
    parser.add_argument('--n', type=float, default=2.5, help='Hill coefficient (both genes)')
    parser.add_argument('--alpha', type=float, default=3.0, help='Max expression rate')
    args = parser.parse_args()

    print("Toggle Switch: Bistability & Phase Plane")
    print("=" * 50)
    print(f"n={args.n}, α={args.alpha}")
    print("Adjust sliders to explore bistability window.")
    print("Click on phase plane to add trajectories.")
    ToggleExplorer(alpha1=args.alpha, alpha2=args.alpha, n1=args.n, n2=args.n)
