"""
Enzyme Kinetics Lab
====================
Tier 0.2 — Chemistry: Enzyme kinetics

Concepts demonstrated:
  - Michaelis-Menten kinetics: full mechanism vs. QSS approximation
  - Km and Vmax: measuring affinity and maximum rate
  - kcat (turnover number) and catalytic efficiency kcat/Km
  - Competitive, uncompetitive, and noncompetitive inhibition
  - Lineweaver-Burk double-reciprocal plots
  - Hill equation: cooperativity and ultrasensitivity

Usage:
    python 03_enzyme_kinetics.py

Controls:
    Use sliders to explore parameter space.
    Radio buttons select kinetic model.
"""

import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from matplotlib.widgets import Slider, RadioButtons
from scipy.integrate import solve_ivp


# ── Kinetic Models ────────────────────────────────────────────────────────────

def michaelis_menten(S, Vmax, Km):
    """Classic MM: v = Vmax[S]/(Km+[S])"""
    return Vmax * S / (Km + S)


def hill_equation(S, Vmax, K, n):
    """Cooperative binding: v = Vmax·[S]^n / (K^n + [S]^n)"""
    return Vmax * S**n / (K**n + S**n)


def competitive_inhibition(S, Vmax, Km, I, Ki):
    """Inhibitor competes with substrate. Apparent Km increases; Vmax unchanged."""
    Km_app = Km * (1 + I / Ki)
    return Vmax * S / (Km_app + S)


def uncompetitive_inhibition(S, Vmax, Km, I, Ki):
    """Inhibitor binds ES. Both apparent Km and Vmax decrease."""
    Vmax_app = Vmax / (1 + I / Ki)
    Km_app = Km / (1 + I / Ki)
    return Vmax_app * S / (Km_app + S)


def noncompetitive_inhibition(S, Vmax, Km, I, Ki):
    """Inhibitor binds E or ES. Vmax decreases; Km unchanged."""
    Vmax_app = Vmax / (1 + I / Ki)
    return Vmax_app * S / (Km + S)


def full_mechanism_odes(t, y, k1, km1, kcat, Et):
    """Full E+S ⇌ ES → E+P mechanism (4 ODEs).
    y = [S, E, ES, P]"""
    S, E, ES, P = y
    dS  = -k1 * E * S + km1 * ES
    dE  =  -k1 * E * S + km1 * ES + kcat * ES
    dES =   k1 * E * S - km1 * ES - kcat * ES
    dP  =   kcat * ES
    return [dS, dE, dES, dP]


# ── Main App ──────────────────────────────────────────────────────────────────

class EnzymeKineticsLab:
    MODES = ["Michaelis-Menten", "Hill Equation", "Inhibition Modes",
             "Full Mechanism (ODE)", "Lineweaver-Burk"]

    def __init__(self):
        self.mode = "Michaelis-Menten"
        self._build_figure()
        self._draw()
        plt.show()

    def _build_figure(self):
        self.fig = plt.figure(figsize=(16, 9))
        self.fig.patch.set_facecolor('#1a1a2e')
        self.fig.suptitle('Enzyme Kinetics Lab', color='white',
                          fontsize=14, fontweight='bold')

        gs = gridspec.GridSpec(2, 2, figure=self.fig,
                               left=0.06, right=0.97, top=0.88, bottom=0.25,
                               hspace=0.45, wspace=0.3)

        self.ax_main  = self.fig.add_subplot(gs[0, 0])
        self.ax_aux   = self.fig.add_subplot(gs[0, 1])
        self.ax_mech  = self.fig.add_subplot(gs[1, 0])
        self.ax_info  = self.fig.add_subplot(gs[1, 1])

        for ax in [self.ax_main, self.ax_aux, self.ax_mech, self.ax_info]:
            ax.set_facecolor('#0f3460')

        # Sliders
        slider_specs = [
            (0.07, 0.18, 'Vmax', 0.1, 10.0, 5.0),
            (0.07, 0.14, 'Km',   0.01, 5.0, 1.0),
            (0.07, 0.10, 'n (Hill)', 0.5, 6.0, 1.0),
            (0.07, 0.06, '[I] (Inhibitor)', 0.0, 5.0, 0.0),
            (0.07, 0.02, 'Ki', 0.1, 5.0, 1.0),
        ]
        self.sliders = {}
        for x, y, label, vmin, vmax, vinit in slider_specs:
            ax_s = self.fig.add_axes([x, y, 0.30, 0.022])
            ax_s.set_facecolor('#16213e')
            sl = Slider(ax_s, label, vmin, vmax, valinit=vinit, color='#e94560')
            sl.label.set_color('white')
            sl.valtext.set_color('white')
            sl.on_changed(lambda val: self._draw())
            self.sliders[label] = sl

        # Radio
        ax_radio = self.fig.add_axes([0.42, 0.01, 0.18, 0.20])
        ax_radio.set_facecolor('#16213e')
        self.radio = RadioButtons(ax_radio, self.MODES, activecolor='#e94560')
        self.radio.on_clicked(self._on_mode)

    def _on_mode(self, label):
        self.mode = label
        self._draw()

    @property
    def p(self):
        return {k: s.val for k, s in self.sliders.items()}

    def _draw(self):
        for ax in [self.ax_main, self.ax_aux, self.ax_mech, self.ax_info]:
            ax.cla()
            ax.set_facecolor('#0f3460')
        self.ax_info.axis('off')

        S = np.linspace(0, 10, 500)
        p = self.p
        Vmax, Km, n = p['Vmax'], p['Km'], p['n (Hill)']
        I, Ki = p['[I] (Inhibitor)'], p['Ki']

        if self.mode == "Michaelis-Menten":
            self._draw_mm(S, Vmax, Km)
        elif self.mode == "Hill Equation":
            self._draw_hill(S, Vmax, Km, n)
        elif self.mode == "Inhibition Modes":
            self._draw_inhibition(S, Vmax, Km, I, Ki)
        elif self.mode == "Full Mechanism (ODE)":
            self._draw_full_mechanism(Vmax, Km)
        elif self.mode == "Lineweaver-Burk":
            self._draw_lineweaver_burk(S, Vmax, Km, I, Ki)

        self.fig.canvas.draw_idle()

    def _draw_mm(self, S, Vmax, Km):
        v = michaelis_menten(S, Vmax, Km)
        ax = self.ax_main
        ax.plot(S, v, color='#50fa7b', lw=2.5, label='v(S)')
        ax.axhline(Vmax, ls='--', color='#f5a623', lw=1.5, label=f'Vmax = {Vmax:.2f}')
        ax.axhline(Vmax / 2, ls=':', color='#8be9fd', lw=1.2, label=f'Vmax/2 = {Vmax/2:.2f}')
        ax.axvline(Km, ls=':', color='#ff79c6', lw=1.2, label=f'Km = {Km:.2f}')
        ax.plot(Km, Vmax / 2, 'wo', ms=8, zorder=5)
        ax.set_xlabel('[S]', color='white')
        ax.set_ylabel('v (reaction rate)', color='white')
        ax.set_title('Michaelis-Menten Curve', color='white')
        ax.legend(facecolor='#16213e', labelcolor='white', fontsize=8)

        # Efficiency
        ax2 = self.ax_aux
        kcat = Vmax  # normalized per unit enzyme
        eff = kcat / Km
        data_labels = ['Km', 'Vmax', 'kcat/Km']
        vals = [Km, Vmax, eff]
        colors = ['#ff79c6', '#50fa7b', '#f5a623']
        bars = ax2.bar(data_labels, vals, color=colors, edgecolor='white', width=0.5)
        for bar, val in zip(bars, vals):
            ax2.text(bar.get_x() + bar.get_width() / 2, bar.get_height() * 1.02,
                     f'{val:.3f}', ha='center', color='white', fontsize=9)
        ax2.set_title('Kinetic Parameters', color='white')
        ax2.set_ylabel('Value', color='white')

        # Mechanism text
        self.ax_mech.text(0.05, 0.9,
            "Michaelis-Menten Mechanism:\n\n"
            "  E + S  ⇌  ES  →  E + P\n\n"
            "QSS (Briggs-Haldane):\n"
            "  Km = (k₋₁ + kcat) / k₁\n"
            "  v = Vmax·[S] / (Km + [S])\n\n"
            "Km interpretation:\n"
            "  [S] at half-maximal velocity\n"
            "  Proxy for enzyme-substrate affinity\n\n"
            "Catalytic efficiency:\n"
            "  kcat/Km = specificity constant\n"
            "  Upper limit: diffusion-controlled\n"
            "  (~10⁸–10⁹ M⁻¹s⁻¹)",
            transform=self.ax_mech.transAxes,
            va='top', fontsize=8.5, color='white', fontfamily='monospace')

        info = (
            f"Current parameters:\n"
            f"  Vmax = {Vmax:.2f}\n"
            f"  Km   = {Km:.3f}\n"
            f"  v(Km)= {michaelis_menten(Km, Vmax, Km):.3f}\n\n"
            f"At [S]=Km: v = Vmax/2 ✓\n\n"
            f"At [S]=10×Km:\n"
            f"  v = {michaelis_menten(10*Km, Vmax, Km):.3f}\n"
            f"  = {100*michaelis_menten(10*Km, Vmax, Km)/Vmax:.1f}% of Vmax"
        )
        self.ax_info.text(0.05, 0.95, info, transform=self.ax_info.transAxes,
                          va='top', fontsize=9, color='white', fontfamily='monospace')

        for ax in [self.ax_main, self.ax_aux, self.ax_mech]:
            ax.tick_params(colors='white')
            for sp in ax.spines.values():
                sp.set_edgecolor('#888')

    def _draw_hill(self, S, Vmax, K, n_vals_extra=None):
        ax = self.ax_main
        ax2 = self.ax_aux

        for n_test in [0.5, 1.0, 2.0, 4.0]:
            v = hill_equation(S, Vmax, K, n_test)
            ls = '-' if abs(n_test - self.p['n (Hill)']) < 0.1 else '--'
            alpha = 1.0 if ls == '-' else 0.45
            ax.plot(S, v, lw=2 if ls=='-' else 1, alpha=alpha,
                    label=f'n={n_test}')

        ax.axhline(Vmax / 2, ls=':', color='#888', lw=1)
        ax.axvline(K, ls=':', color='#888', lw=1)
        ax.set_xlabel('[S]', color='white')
        ax.set_ylabel('v', color='white')
        ax.set_title('Hill Equation — Cooperativity', color='white')
        ax.legend(facecolor='#16213e', labelcolor='white', fontsize=8)

        # Sensitivity metric
        n_range = np.linspace(0.5, 8, 200)
        # EC10 and EC90 for each n
        EC10_arr = K * (0.1 / 0.9)**(1 / n_range)
        EC90_arr = K * (0.9 / 0.1)**(1 / n_range)
        ratio = EC90_arr / EC10_arr
        ax2.semilogy(n_range, ratio, color='#ff79c6', lw=2)
        n_cur = self.p['n (Hill)']
        EC10 = K * (0.1 / 0.9)**(1 / n_cur)
        EC90 = K * (0.9 / 0.1)**(1 / n_cur)
        ax2.axvline(n_cur, color='white', ls='--', lw=1)
        ax2.set_xlabel('Hill coefficient n', color='white')
        ax2.set_ylabel('EC90 / EC10 (ultrasensitivity)', color='white')
        ax2.set_title('Ultrasensitivity vs. n', color='white')

        self.ax_mech.text(0.05, 0.9,
            "Hill Equation:\n\n"
            "  v = Vmax·[S]ⁿ / (Kⁿ + [S]ⁿ)\n\n"
            "n = 1  : Michaelis-Menten\n"
            "n > 1  : Cooperative (sigmoidal)\n"
            "n → ∞  : Perfect switch (step)\n\n"
            "Physical basis:\n"
            "  n cooperative binding sites\n"
            "  All-or-none binding at high n\n\n"
            "Ultrasensitivity index:\n"
            "  EC90/EC10 → small when n is large\n"
            "  n=1: ratio=81; n=4: ratio≈3",
            transform=self.ax_mech.transAxes,
            va='top', fontsize=8.5, color='white', fontfamily='monospace')

        info = (
            f"n = {n_cur:.2f}\n"
            f"K50 = {K:.3f}\n"
            f"EC10 = {EC10:.3f}\n"
            f"EC90 = {EC90:.3f}\n"
            f"EC90/EC10 = {EC90/EC10:.2f}\n\n"
            f"n=1 (MM): ratio=81\n"
            f"n=2:      ratio≈9\n"
            f"n=4:      ratio≈3"
        )
        self.ax_info.text(0.05, 0.95, info, transform=self.ax_info.transAxes,
                          va='top', fontsize=9, color='white', fontfamily='monospace')

        for ax in [ax, ax2, self.ax_mech]:
            ax.tick_params(colors='white')
            for sp in ax.spines.values():
                sp.set_edgecolor('#888')

    def _draw_inhibition(self, S, Vmax, Km, I, Ki):
        ax = self.ax_main
        ax2 = self.ax_aux

        v_no_inh  = michaelis_menten(S, Vmax, Km)
        v_comp    = competitive_inhibition(S, Vmax, Km, I, Ki)
        v_uncomp  = uncompetitive_inhibition(S, Vmax, Km, I, Ki)
        v_noncomp = noncompetitive_inhibition(S, Vmax, Km, I, Ki)

        ax.plot(S, v_no_inh,  '-',  color='#50fa7b', lw=2.5, label='No inhibitor')
        ax.plot(S, v_comp,    '--', color='#ff5555', lw=2,   label=f'Competitive (I={I:.1f})')
        ax.plot(S, v_uncomp,  '-.',  color='#f5a623', lw=2,   label='Uncompetitive')
        ax.plot(S, v_noncomp, ':',  color='#8be9fd', lw=2,   label='Noncompetitive')
        ax.axhline(Vmax, color='#50fa7b', ls=':', lw=0.8, alpha=0.4)
        ax.set_xlabel('[S]', color='white')
        ax.set_ylabel('v', color='white')
        ax.set_title('Inhibition Modes', color='white')
        ax.legend(facecolor='#16213e', labelcolor='white', fontsize=7.5)

        # Table of apparent Km, Vmax
        rows = {
            'None':       (Km, Vmax),
            'Competitive': (Km*(1+I/Ki), Vmax),
            'Uncompetitive': (Km/(1+I/Ki), Vmax/(1+I/Ki)),
            'Noncompetitive': (Km, Vmax/(1+I/Ki)),
        }
        ax2.axis('off')
        table_data = [['Mode', "Km'", "Vmax'"]]
        for mode, (km_app, vmax_app) in rows.items():
            table_data.append([mode, f'{km_app:.3f}', f'{vmax_app:.3f}'])
        tbl = ax2.table(cellText=table_data[1:], colLabels=table_data[0],
                        loc='center', cellLoc='center')
        tbl.auto_set_font_size(False)
        tbl.set_fontsize(9)
        for key, cell in tbl.get_celld().items():
            cell.set_facecolor('#0f3460')
            cell.set_edgecolor('#888')
            cell.set_text_props(color='white')
        ax2.set_title('Apparent Parameters Summary', color='white')

        for ax in [self.ax_main, self.ax_mech]:
            ax.tick_params(colors='white')
            for sp in ax.spines.values():
                sp.set_edgecolor('#888')

    def _draw_full_mechanism(self, Vmax, Km):
        """Simulate the full enzyme mechanism ODE and compare to QSS."""
        # Derive rate constants: Km = (km1+kcat)/k1; assume km1 = 10*kcat
        kcat = Vmax / 0.1  # Et = 0.1
        Et = 0.1
        k1  = 100.0
        kcat = 1.0
        km1  = Km * k1 - kcat
        if km1 < 0:
            km1 = 0.5

        S0 = 5.0
        y0 = [S0, Et, 0, 0]  # S, E, ES, P
        t_eval = np.linspace(0, 40, 1000)

        sol = solve_ivp(full_mechanism_odes, (0, 40), y0, t_eval=t_eval,
                        args=(k1, km1, kcat, Et), method='BDF', rtol=1e-10)

        S_t, E_t, ES_t, P_t = sol.y

        ax = self.ax_main
        ax.plot(sol.t, S_t,  color='#8be9fd', lw=1.8, label='[S] substrate')
        ax.plot(sol.t, E_t,  color='#50fa7b', lw=1.8, label='[E] free enzyme')
        ax.plot(sol.t, ES_t, color='#f5a623', lw=1.8, label='[ES] complex')
        ax.plot(sol.t, P_t,  color='#ff79c6', lw=1.8, label='[P] product')
        ax.set_xlabel('Time', color='white')
        ax.set_ylabel('Concentration', color='white')
        ax.set_title('Full Mechanism ODE Simulation', color='white')
        ax.legend(facecolor='#16213e', labelcolor='white', fontsize=8)

        # QSS phase
        ax2 = self.ax_aux
        S_range = np.linspace(0, S0, 300)
        v_qss = (kcat * Et) * S_range / (Km + S_range)

        # Actual instantaneous rate from simulation
        dP_dt = kcat * ES_t
        ax2.plot(S_t, dP_dt, color='#e94560', lw=2, label='v(t) — exact')
        ax2.plot(S_range, v_qss, '--', color='#50fa7b', lw=2, label='v — QSS')
        ax2.set_xlabel('[S]', color='white')
        ax2.set_ylabel('v = d[P]/dt', color='white')
        ax2.set_title('QSS Approximation vs. Exact', color='white')
        ax2.legend(facecolor='#16213e', labelcolor='white', fontsize=8)

        for ax in [self.ax_main, self.ax_aux, self.ax_mech]:
            ax.tick_params(colors='white')
            for sp in ax.spines.values():
                sp.set_edgecolor('#888')

    def _draw_lineweaver_burk(self, S, Vmax, Km, I, Ki):
        """Double-reciprocal plot showing how inhibition modes look different."""
        ax = self.ax_main
        S_pos = S[S > 0.1]
        inv_S = 1 / S_pos

        def lb(v_arr):
            return 1 / (v_arr + 1e-12)

        ax.plot(inv_S, lb(michaelis_menten(S_pos, Vmax, Km)),
                color='#50fa7b', lw=2.5, label='No inhibitor')
        ax.plot(inv_S, lb(competitive_inhibition(S_pos, Vmax, Km, I, Ki)),
                '--', color='#ff5555', lw=2, label=f'Competitive (I={I:.1f})')
        ax.plot(inv_S, lb(uncompetitive_inhibition(S_pos, Vmax, Km, I, Ki)),
                '-.', color='#f5a623', lw=2, label='Uncompetitive')
        ax.plot(inv_S, lb(noncompetitive_inhibition(S_pos, Vmax, Km, I, Ki)),
                ':', color='#8be9fd', lw=2, label='Noncompetitive')

        ax.axhline(0, color='#888', lw=0.5)
        ax.axvline(0, color='#888', lw=0.5)
        ax.set_xlabel('1/[S]  →  [S] increases left', color='white')
        ax.set_ylabel('1/v', color='white')
        ax.set_title('Lineweaver-Burk Double-Reciprocal Plot', color='white')
        ax.legend(facecolor='#16213e', labelcolor='white', fontsize=8)
        ax.set_ylim(bottom=0)

        self.ax_mech.text(0.05, 0.9,
            "Lineweaver-Burk Plot:\n"
            "  1/v = (Km/Vmax)·(1/[S]) + 1/Vmax\n\n"
            "y-intercept: 1/Vmax\n"
            "x-intercept: -1/Km\n"
            "slope: Km/Vmax\n\n"
            "Inhibition diagnosis:\n"
            "  Competitive: lines cross y-axis\n"
            "  Uncompetitive: parallel lines\n"
            "  Noncompetitive: lines cross x-axis\n\n"
            "(LB plots are historically important\n"
            "but amplify error at low 1/[S];\n"
            "use nonlinear fitting in practice)",
            transform=self.ax_mech.transAxes,
            va='top', fontsize=8.5, color='white', fontfamily='monospace')

        for ax in [self.ax_main, self.ax_mech]:
            ax.tick_params(colors='white')
            for sp in ax.spines.values():
                sp.set_edgecolor('#888')


if __name__ == '__main__':
    print("Enzyme Kinetics Lab")
    print("=" * 40)
    print("Modes: Michaelis-Menten | Hill Equation | Inhibition | Full ODE | Lineweaver-Burk")
    print()
    EnzymeKineticsLab()
