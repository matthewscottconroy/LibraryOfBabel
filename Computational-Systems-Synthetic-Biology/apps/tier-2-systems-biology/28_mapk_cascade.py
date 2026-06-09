"""
MAPK Signaling Cascade: Ultrasensitivity Explorer
===================================================
Tier 2.4 — Systems Biology: Signaling Networks

Concepts demonstrated:
  - Phosphorylation cascades: kinase/phosphatase cycling
  - Michaelis-Menten kinetics in zero-order ultrasensitivity (Goldbeter-Koshland)
  - Signal amplification through cascade tiers
  - Ultrasensitivity: sigmoidal (switch-like) dose-response
  - Hill coefficient as a measure of cooperativity/ultrasensitivity
  - Time-course dynamics: transient vs sustained signaling
  - How saturation of phosphatases creates switch-like behaviour

Usage:
    python 28_mapk_cascade.py

Controls:
    Sliders: total protein concentrations (KKK, KK, K),
             input kinase rate V1, phosphatase Km values
    Toggle: "Dose-Response" vs "Time Course" view
    Radio: show KKK / KK / K activity (cascade tier)
"""

import numpy as np
import matplotlib
try:
    matplotlib.use('TkAgg')
except Exception:
    pass  # fall back to whatever backend is available
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
from matplotlib.widgets import Slider, Button
from scipy.integrate import solve_ivp

# ---------------------------------------------------------------------------
# Default parameters
# ---------------------------------------------------------------------------
DEFAULT_PARAMS = {
    'T1': 1.0, 'T2': 1.0, 'T3': 1.0,   # total concentrations
    'V1': 2.5, 'V2': 1.0,               # KKK kinase/phosphatase Vmax
    'V3': 2.5, 'V4': 1.0,               # KK kinase/phosphatase Vmax
    'V5': 2.5, 'V6': 1.0,               # K kinase/phosphatase Vmax
    'Km1': 0.1, 'Km2': 0.1,             # KKK cycle Km
    'Km3': 0.1, 'Km4': 0.1,             # KK cycle Km
    'Km5': 0.1, 'Km6': 0.1,             # K cycle Km
}

# ---------------------------------------------------------------------------
# ODE system
# ---------------------------------------------------------------------------
def mapk_odes(t, y, S, params):
    """Three-tier MAPK cascade ODEs (Goldbeter-Koshland phosphorylation cycles)."""
    a_KKK, a_KK, a_K = y
    p = params

    T1, T2, T3 = p['T1'], p['T2'], p['T3']

    # Clamp active fractions to [0, T_i] for numerical safety
    a_KKK = np.clip(a_KKK, 0.0, T1)
    a_KK  = np.clip(a_KK,  0.0, T2)
    a_K   = np.clip(a_K,   0.0, T3)

    eps = 1e-12

    # Tier 1: KKK
    phos1   = p['V1'] * S    * (T1 - a_KKK) / (p['Km1'] + (T1 - a_KKK) + eps)
    dephos1 = p['V2']        * a_KKK         / (p['Km2'] + a_KKK          + eps)
    da_KKK  = phos1 - dephos1

    # Tier 2: KK
    phos2   = p['V3'] * a_KKK * (T2 - a_KK) / (p['Km3'] + (T2 - a_KK) + eps)
    dephos2 = p['V4']          * a_KK         / (p['Km4'] + a_KK          + eps)
    da_KK   = phos2 - dephos2

    # Tier 3: K
    phos3   = p['V5'] * a_KK  * (T3 - a_K)  / (p['Km5'] + (T3 - a_K)  + eps)
    dephos3 = p['V6']          * a_K          / (p['Km6'] + a_K           + eps)
    da_K    = phos3 - dephos3

    return [da_KKK, da_KK, da_K]


def steady_state(S, params):
    """Integrate to steady state for a given signal S."""
    y0 = [0.01, 0.01, 0.01]
    sol = solve_ivp(
        mapk_odes, (0, 500), y0,
        args=(S, params),
        method='Radau',
        dense_output=False,
        rtol=1e-8, atol=1e-10,
    )
    return sol.y[:, -1]


def dose_response_curves(params, n_pts=120):
    """Sweep signal S and return steady-state activity for each tier."""
    signals = np.linspace(0, 3.0, n_pts)
    kkk_ss = np.zeros(n_pts)
    kk_ss  = np.zeros(n_pts)
    k_ss   = np.zeros(n_pts)
    for i, S in enumerate(signals):
        ss = steady_state(S, params)
        kkk_ss[i] = np.clip(ss[0] / params['T1'], 0, 1)
        kk_ss[i]  = np.clip(ss[1] / params['T2'], 0, 1)
        k_ss[i]   = np.clip(ss[2] / params['T3'], 0, 1)
    return signals, kkk_ss, kk_ss, k_ss


def time_course(params, S_low=0.0, S_high=1.5, step_t=10.0, t_end=100.0, n_pts=500):
    """Simulate step input: S=S_low for t<step_t, S=S_high after."""
    t_eval = np.linspace(0, t_end, n_pts)

    # Phase 1: basal
    y0 = [0.01, 0.01, 0.01]
    t1_eval = t_eval[t_eval <= step_t]
    sol1 = solve_ivp(
        mapk_odes, (0, step_t), y0,
        args=(S_low, params),
        method='Radau',
        t_eval=t1_eval,
        rtol=1e-8, atol=1e-10,
    )
    y_at_step = sol1.y[:, -1]

    # Phase 2: stimulated
    t2_eval = t_eval[t_eval > step_t]
    sol2 = solve_ivp(
        mapk_odes, (step_t, t_end), y_at_step,
        args=(S_high, params),
        method='Radau',
        t_eval=t2_eval,
        rtol=1e-8, atol=1e-10,
    )

    t_full = np.concatenate([sol1.t, sol2.t])
    y_full = np.concatenate([sol1.y, sol2.y], axis=1)
    return t_full, y_full


def compute_hill_coefficient(signals, k_ss):
    """Estimate Hill coefficient from dose-response curve of final tier K."""
    max_act = np.max(k_ss)
    if max_act < 1e-6:
        return 1.0
    norm = k_ss / max_act
    # Find S10 and S90 by linear interpolation
    try:
        idx10 = np.where(norm >= 0.10)[0][0]
        idx90 = np.where(norm >= 0.90)[0][0]
        if idx10 == 0:
            s10 = signals[0] + 1e-6
        else:
            f = (0.10 - norm[idx10 - 1]) / (norm[idx10] - norm[idx10 - 1] + 1e-12)
            s10 = signals[idx10 - 1] + f * (signals[idx10] - signals[idx10 - 1])
        f = (0.90 - norm[idx90 - 1]) / (norm[idx90] - norm[idx90 - 1] + 1e-12)
        s90 = signals[idx90 - 1] + f * (signals[idx90] - signals[idx90 - 1])
        if s10 <= 0 or s90 <= s10:
            return 1.0
        n_H = np.log(81.0) / np.log(s90 / s10)
        return float(np.clip(n_H, 0.1, 30.0))
    except (IndexError, ZeroDivisionError):
        return 1.0


# ---------------------------------------------------------------------------
# Cascade cartoon drawing
# ---------------------------------------------------------------------------
def draw_cascade(ax, activities, labels=('Raf (KKK)', 'MEK (KK)', 'ERK (K)')):
    """Draw the 3-tier cascade cartoon with activity-coloured boxes."""
    ax.cla()
    ax.set_facecolor('#0f3460')
    ax.set_xlim(0, 1)
    ax.set_ylim(0, 1)
    ax.axis('off')
    ax.set_title('MAPK Cascade', color='white', fontsize=12, fontweight='bold', pad=6)

    box_x = 0.25
    box_w = 0.50
    box_h = 0.13
    y_centers = [0.82, 0.52, 0.22]
    tier_colors_active = ['#8be9fd', '#f5a623', '#50fa7b']

    # Signal arrow
    ax.annotate(
        'Signal (S)', xy=(0.5, 0.97), xytext=(0.5, 0.97),
        fontsize=9, color='#ff79c6', ha='center',
    )
    ax.annotate(
        '', xy=(0.5, y_centers[0] + box_h / 2 + 0.01),
        xytext=(0.5, 0.94),
        arrowprops=dict(arrowstyle='->', color='#ff79c6', lw=2),
    )

    for i, (yc, act) in enumerate(zip(y_centers, activities)):
        frac = float(np.clip(act, 0, 1))
        # Interpolate box colour: dark blue → tier accent
        base = np.array([0.06, 0.13, 0.25])
        accent_hex = tier_colors_active[i].lstrip('#')
        accent = np.array([int(accent_hex[j:j+2], 16) / 255.0 for j in (0, 2, 4)])
        box_rgb = base + frac * (accent - base)
        box_color = tuple(box_rgb)

        rect = mpatches.FancyBboxPatch(
            (box_x, yc - box_h / 2), box_w, box_h,
            boxstyle='round,pad=0.02',
            facecolor=box_color,
            edgecolor=tier_colors_active[i],
            linewidth=2,
        )
        ax.add_patch(rect)
        ax.text(
            0.5, yc + 0.01, labels[i],
            ha='center', va='center', color='white',
            fontsize=10, fontweight='bold',
        )
        ax.text(
            0.5, yc - 0.035, f'{frac * 100:.1f}% active',
            ha='center', va='center', color='white', fontsize=8,
        )

        # Arrow to next tier
        if i < 2:
            ax.annotate(
                '', xy=(0.5, y_centers[i + 1] + box_h / 2 + 0.01),
                xytext=(0.5, yc - box_h / 2 - 0.01),
                arrowprops=dict(arrowstyle='->', color=tier_colors_active[i], lw=2),
            )

    # Phosphatase labels (right side)
    for i, yc in enumerate(y_centers):
        ax.text(0.82, yc, 'PP2A\n(phosphatase)',
                ha='left', va='center', color='#e94560', fontsize=7)
        ax.annotate(
            '', xy=(box_x + box_w + 0.01, yc),
            xytext=(0.80, yc),
            arrowprops=dict(arrowstyle='<-', color='#e94560', lw=1.2),
        )



# ---------------------------------------------------------------------------
# Main application
# ---------------------------------------------------------------------------
def build_app():
    params = dict(DEFAULT_PARAMS)

    # --- Figure & axes layout ---
    fig = plt.figure(figsize=(16, 9))
    fig.patch.set_facecolor('#1a1a2e')

    # GridSpec: [cascade | main plot | controls]
    from matplotlib.gridspec import GridSpec
    gs = GridSpec(
        1, 3,
        figure=fig,
        left=0.03, right=0.97,
        top=0.93, bottom=0.28,
        wspace=0.08,
        width_ratios=[1, 2.2, 1],
    )

    ax_cascade  = fig.add_subplot(gs[0, 0])
    ax_main     = fig.add_subplot(gs[0, 1])
    ax_controls = fig.add_subplot(gs[0, 2])

    for ax in [ax_cascade, ax_main, ax_controls]:
        ax.set_facecolor('#0f3460')

    ax_controls.axis('off')

    # ----- Slider axes (bottom strip) -----
    sl_color = '#16213e'
    ax_sl_S    = fig.add_axes([0.12, 0.17, 0.35, 0.025]);  ax_sl_S.set_facecolor(sl_color)
    ax_sl_Km   = fig.add_axes([0.12, 0.12, 0.35, 0.025]);  ax_sl_Km.set_facecolor(sl_color)
    ax_sl_Vmax = fig.add_axes([0.12, 0.07, 0.35, 0.025]);  ax_sl_Vmax.set_facecolor(sl_color)

    sl_S    = Slider(ax_sl_S,    'Signal S',           0.0,  3.0,   valinit=0.5,  color='#e94560')
    sl_Km   = Slider(ax_sl_Km,  'Phosphatase Km',     0.001, 0.5,  valinit=0.1,  color='#8be9fd')
    sl_Vmax = Slider(ax_sl_Vmax,'Kinase Vmax ratio',  1.0,  5.0,   valinit=2.5,  color='#50fa7b')

    for sl in [sl_S, sl_Km, sl_Vmax]:
        sl.label.set_color('white')
        sl.valtext.set_color('white')

    # ----- Toggle button -----
    ax_btn = fig.add_axes([0.55, 0.12, 0.18, 0.06])
    btn_toggle = Button(ax_btn, 'View: Dose-Response',
                        color='#16213e', hovercolor='#e94560')
    btn_toggle.label.set_color('white')
    btn_toggle.label.set_fontsize(9)

    # ----- Info text axes -----
    ax_info = fig.add_axes([0.75, 0.04, 0.22, 0.20])
    ax_info.set_facecolor('#0f3460')
    ax_info.axis('off')
    info_text = ax_info.text(
        0.05, 0.95, '',
        transform=ax_info.transAxes,
        color='white', fontsize=9, va='top', family='monospace',
    )

    # State
    state = {'view': 'dose_response'}  # 'dose_response' or 'time_course'

    # ------------------------------------------------------------------ #
    def get_params():
        p = dict(DEFAULT_PARAMS)
        km_val   = sl_Km.val
        vmax_val = sl_Vmax.val
        p['Km2'] = km_val; p['Km4'] = km_val; p['Km6'] = km_val
        p['V1']  = vmax_val; p['V3'] = vmax_val; p['V5'] = vmax_val
        return p

    def update_main_plot():
        p   = get_params()
        S   = sl_S.val
        ss  = steady_state(S, p)
        act = [ss[0] / p['T1'], ss[1] / p['T2'], ss[2] / p['T3']]
        act = [float(np.clip(a, 0, 1)) for a in act]

        draw_cascade(ax_cascade, act)

        ax_main.cla()
        ax_main.set_facecolor('#16213e')
        for spine in ax_main.spines.values():
            spine.set_color('#8be9fd')
        ax_main.tick_params(colors='white')
        ax_main.xaxis.label.set_color('white')
        ax_main.yaxis.label.set_color('white')
        ax_main.title.set_color('white')

        if state['view'] == 'dose_response':
            signals, kkk_ss, kk_ss, k_ss = dose_response_curves(p)
            n_H = compute_hill_coefficient(signals, k_ss)

            ax_main.plot(signals, kkk_ss, color='#8be9fd', lw=2.0, label='Raf (KKK)')
            ax_main.plot(signals, kk_ss,  color='#f5a623', lw=2.0, label='MEK (KK)')
            ax_main.plot(signals, k_ss,   color='#50fa7b', lw=2.5, label='ERK (K)')
            ax_main.axvline(S, color='#ff79c6', lw=1.5, ls='--', alpha=0.8, label=f'S = {S:.2f}')
            ax_main.set_xlabel('Signal (S)', fontsize=11)
            ax_main.set_ylabel('Fraction active', fontsize=11)
            ax_main.set_title('Dose-Response Curves', fontsize=13, fontweight='bold')
            ax_main.set_xlim(0, 3)
            ax_main.set_ylim(-0.05, 1.15)
            ax_main.legend(framealpha=0.3, facecolor='#0f3460', labelcolor='white', fontsize=9)
            ax_main.text(
                0.98, 0.10,
                f'ERK Hill coefficient: {n_H:.2f}',
                transform=ax_main.transAxes,
                color='#50fa7b', fontsize=11, fontweight='bold',
                ha='right', va='bottom',
                bbox=dict(boxstyle='round,pad=0.3', facecolor='#0f3460', alpha=0.7),
            )
            info_str = (
                f'Signal S   : {S:.2f}\n'
                f'KKK active : {act[0]*100:.1f}%\n'
                f'KK  active : {act[1]*100:.1f}%\n'
                f'K   active : {act[2]*100:.1f}%\n'
                f'Hill coeff : {n_H:.2f}\n'
                f'Phosph. Km : {sl_Km.val:.3f}\n'
                f'Kinase Vmax: {sl_Vmax.val:.2f}\n'
            )

        else:  # time_course
            t_arr, y_arr = time_course(p, S_low=0.0, S_high=max(S, 0.05))
            kkk_t = np.clip(y_arr[0] / p['T1'], 0, 1)
            kk_t  = np.clip(y_arr[1] / p['T2'], 0, 1)
            k_t   = np.clip(y_arr[2] / p['T3'], 0, 1)

            ax_main.plot(t_arr, kkk_t, color='#8be9fd', lw=2.0, label='Raf (KKK)')
            ax_main.plot(t_arr, kk_t,  color='#f5a623', lw=2.0, label='MEK (KK)')
            ax_main.plot(t_arr, k_t,   color='#50fa7b', lw=2.5, label='ERK (K)')
            ax_main.axvline(10.0, color='#ff79c6', lw=1.5, ls='--', alpha=0.8, label='Step onset')
            ax_main.text(
                10.5, 0.95, f'S → {max(S, 0.05):.2f}',
                color='#ff79c6', fontsize=9, va='top',
            )
            ax_main.set_xlabel('Time', fontsize=11)
            ax_main.set_ylabel('Fraction active', fontsize=11)
            ax_main.set_title('Time Course (Step Input)', fontsize=13, fontweight='bold')
            ax_main.set_xlim(0, 100)
            ax_main.set_ylim(-0.05, 1.15)
            ax_main.legend(framealpha=0.3, facecolor='#0f3460', labelcolor='white', fontsize=9)
            signals, _, _, k_ss = dose_response_curves(p, n_pts=80)
            n_H = compute_hill_coefficient(signals, k_ss)
            info_str = (
                f'Signal S   : {S:.2f}\n'
                f'KKK final  : {float(kkk_t[-1])*100:.1f}%\n'
                f'KK  final  : {float(kk_t[-1])*100:.1f}%\n'
                f'K   final  : {float(k_t[-1])*100:.1f}%\n'
                f'Hill coeff : {n_H:.2f}\n'
                f'Phosph. Km : {sl_Km.val:.3f}\n'
                f'Kinase Vmax: {sl_Vmax.val:.2f}\n'
            )

        info_text.set_text(info_str)
        fig.canvas.draw_idle()

    # ------------------------------------------------------------------ #
    def on_slider_change(val):
        update_main_plot()

    def on_toggle(event):
        if state['view'] == 'dose_response':
            state['view'] = 'time_course'
            btn_toggle.label.set_text('View: Time Course')
        else:
            state['view'] = 'dose_response'
            btn_toggle.label.set_text('View: Dose-Response')
        update_main_plot()

    sl_S.on_changed(on_slider_change)
    sl_Km.on_changed(on_slider_change)
    sl_Vmax.on_changed(on_slider_change)
    btn_toggle.on_clicked(on_toggle)

    # ----- Title -----
    fig.text(
        0.5, 0.97,
        'MAPK Signaling Cascade — Ultrasensitivity Explorer',
        ha='center', va='top', color='white',
        fontsize=15, fontweight='bold',
    )
    fig.text(
        0.5, 0.945,
        'Tier 2.4  |  Goldbeter-Koshland zero-order ultrasensitivity  |  3-tier phosphorylation cascade',
        ha='center', va='top', color='#8be9fd', fontsize=9,
    )

    # Initial draw
    update_main_plot()
    return fig


# ---------------------------------------------------------------------------
if __name__ == '__main__':
    print("=" * 60)
    print("MAPK Signaling Cascade: Ultrasensitivity Explorer")
    print("=" * 60)
    print()
    print("Controls:")
    print("  Signal S slider      — sets external signal strength")
    print("  Phosphatase Km       — lower Km → more ultrasensitivity")
    print("  Kinase Vmax ratio    — upstream kinase activity")
    print("  Toggle button        — switch dose-response / time-course")
    print()
    print("Key concept:")
    print("  When phosphatase Km << total protein, the enzyme is")
    print("  saturated → zero-order kinetics → Hill coeff >> 1")
    print("  (ultrasensitive, switch-like response)")
    print()
    fig = build_app()
    plt.show()
