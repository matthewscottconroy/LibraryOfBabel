"""
Gillespie SSA: Stochastic Gene Expression
==========================================
Tier 2.1 — Systems Biology: Stochastic Modeling

Concepts demonstrated:
  - Gillespie Direct Method (exact stochastic simulation)
  - Two-state promoter (bursting) gene expression model
  - mRNA/protein number distributions
  - Fano factor and noise (σ²/μ) vs. deterministic prediction
  - Comparison of multiple SSA trajectories
  - Effect of burst size and burst frequency on noise

Usage:
    python 10_gillespie_ssa.py [--model MODEL] [--n_traj N]

Models:
    simple   — constitutive expression (mRNA + protein)
    bursting — two-state promoter (on/off switching)
    feedback — autoactivation (positive feedback)

Key insight:
    Stochastic gene expression is not just a perturbation of the mean.
    Low-copy-number effects produce distributions that can be far from
    Gaussian. Two-state promoters (bursty transcription) produce
    super-Poisson noise — Fano factor > 1.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.stats import poisson, nbinom


# ── Reaction Networks ─────────────────────────────────────────────────────────

MODELS = {
    "simple": {
        "species": ["mRNA", "Protein"],
        "description": (
            "Constitutive expression:\n"
            "∅ →(k_tx) mRNA\n"
            "mRNA →(k_deg_m) ∅\n"
            "mRNA →(k_tl) mRNA + Protein\n"
            "Protein →(k_deg_p) ∅\n\n"
            "Mean mRNA = k_tx / k_deg_m\n"
            "Mean protein = k_tl·mean_m / k_deg_p\n"
            "Both Poisson distributed\n"
            "(Fano = 1)"
        ),
        "params": {
            "k_tx":    1.0,     # transcription rate (mRNA/min)
            "k_deg_m": 0.2,     # mRNA degradation rate
            "k_tl":    5.0,     # translation rate (per mRNA, protein/min)
            "k_deg_p": 0.05,    # protein degradation rate
        },
    },
    "bursting": {
        "species": ["Promoter_ON", "mRNA", "Protein"],
        "description": (
            "Two-state promoter (bursty transcription):\n"
            "ON ←→ OFF  (k_on, k_off)\n"
            "ON →(k_tx) ON + mRNA\n"
            "mRNA →(k_deg_m) ∅\n"
            "mRNA →(k_tl) mRNA + Protein\n"
            "Protein →(k_deg_p) ∅\n\n"
            "Burst frequency: k_on / (k_on + k_off)\n"
            "Burst size: k_tx / k_off\n"
            "Fano > 1 (super-Poisson)"
        ),
        "params": {
            "k_on":    0.2,     # promoter activation
            "k_off":   0.5,     # promoter inactivation
            "k_tx":    5.0,     # TX rate when ON
            "k_deg_m": 0.5,
            "k_tl":    5.0,
            "k_deg_p": 0.05,
        },
    },
    "feedback": {
        "species": ["mRNA", "Protein"],
        "description": (
            "Autoactivation (positive feedback):\n"
            "∅ →(k0 + k1·P/(K+P)) mRNA\n"
            "mRNA →(k_deg_m) ∅\n"
            "mRNA →(k_tl) mRNA + Protein\n"
            "Protein →(k_deg_p) ∅\n\n"
            "Creates bimodal distribution\n"
            "when feedback is strong.\n"
            "Models: cell fate switching."
        ),
        "params": {
            "k0":      0.2,
            "k1":      4.0,
            "K":       20.0,    # activation threshold
            "k_deg_m": 0.5,
            "k_tl":    5.0,
            "k_deg_p": 0.05,
        },
    },
}


# ── Gillespie SSA ─────────────────────────────────────────────────────────────

def gillespie_simple(params, t_max=1000, seed=0):
    rng = np.random.default_rng(seed)
    k_tx = params['k_tx']
    k_deg_m = params['k_deg_m']
    k_tl = params['k_tl']
    k_deg_p = params['k_deg_p']

    m, p = 0, 0
    times = [0.0]
    m_traj = [m]
    p_traj = [p]
    t = 0.0

    while t < t_max:
        rates = np.array([
            k_tx,
            k_deg_m * m,
            k_tl * m,
            k_deg_p * p,
        ], dtype=float)
        R = rates.sum()
        if R < 1e-12:
            t = t_max
            break
        dt = rng.exponential(1.0 / R)
        t += dt
        u = rng.uniform(0, R)
        rxn = np.searchsorted(np.cumsum(rates), u)
        if rxn == 0:   m += 1
        elif rxn == 1: m = max(0, m - 1)
        elif rxn == 2: p += 1
        elif rxn == 3: p = max(0, p - 1)
        times.append(t)
        m_traj.append(m)
        p_traj.append(p)

    return np.array(times), np.array(m_traj), np.array(p_traj)


def gillespie_bursting(params, t_max=1000, seed=0):
    rng = np.random.default_rng(seed)
    k_on = params['k_on']
    k_off = params['k_off']
    k_tx = params['k_tx']
    k_deg_m = params['k_deg_m']
    k_tl = params['k_tl']
    k_deg_p = params['k_deg_p']

    on = 0   # promoter state: 0=OFF, 1=ON
    m, p = 0, 0
    times = [0.0]
    m_traj = [m]
    p_traj = [p]
    t = 0.0

    while t < t_max:
        rates = np.array([
            k_on * (1 - on),       # OFF→ON
            k_off * on,             # ON→OFF
            k_tx * on,              # TX when ON
            k_deg_m * m,
            k_tl * m,
            k_deg_p * p,
        ], dtype=float)
        R = rates.sum()
        if R < 1e-12:
            break
        dt = rng.exponential(1.0 / R)
        t += dt
        u = rng.uniform(0, R)
        rxn = np.searchsorted(np.cumsum(rates), u)
        if rxn == 0:   on = 1
        elif rxn == 1: on = 0
        elif rxn == 2: m += 1
        elif rxn == 3: m = max(0, m - 1)
        elif rxn == 4: p += 1
        elif rxn == 5: p = max(0, p - 1)
        times.append(t)
        m_traj.append(m)
        p_traj.append(p)

    return np.array(times), np.array(m_traj), np.array(p_traj)


def gillespie_feedback(params, t_max=1000, seed=0):
    rng = np.random.default_rng(seed)
    k0 = params['k0']
    k1 = params['k1']
    K  = params['K']
    k_deg_m = params['k_deg_m']
    k_tl = params['k_tl']
    k_deg_p = params['k_deg_p']

    m, p = 0, 0
    times = [0.0]
    m_traj = [m]
    p_traj = [p]
    t = 0.0

    while t < t_max:
        k_eff = k0 + k1 * p / (K + p)
        rates = np.array([k_eff, k_deg_m * m, k_tl * m, k_deg_p * p], dtype=float)
        R = rates.sum()
        if R < 1e-12:
            break
        dt = rng.exponential(1.0 / R)
        t += dt
        u = rng.uniform(0, R)
        rxn = np.searchsorted(np.cumsum(rates), u)
        if rxn == 0:   m += 1
        elif rxn == 1: m = max(0, m - 1)
        elif rxn == 2: p += 1
        elif rxn == 3: p = max(0, p - 1)
        times.append(t)
        m_traj.append(m)
        p_traj.append(p)

    return np.array(times), np.array(m_traj), np.array(p_traj)


GILLESPIE_FNS = {
    'simple': gillespie_simple,
    'bursting': gillespie_bursting,
    'feedback': gillespie_feedback,
}


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(model_name='simple', n_traj=20, t_max=1000):
    model = MODELS[model_name]
    params = model['params']
    gillespie_fn = GILLESPIE_FNS[model_name]

    print(f"Running {n_traj} SSA trajectories for model: {model_name}")
    trajs = [gillespie_fn(params, t_max=t_max, seed=i) for i in range(n_traj)]

    # Sample protein counts at t_max/2 and t_max (steady state)
    def sample_at_time(trajs, t_sample):
        counts = []
        for (times, m_traj, p_traj) in trajs:
            idx = np.searchsorted(times, t_sample) - 1
            idx = max(0, min(idx, len(p_traj) - 1))
            counts.append(p_traj[idx])
        return np.array(counts)

    p_samples = sample_at_time(trajs, t_max * 0.8)
    m_samples = np.array([
        trajs[i][1][np.searchsorted(trajs[i][0], t_max * 0.8) - 1]
        for i in range(n_traj)
    ])

    mean_p = p_samples.mean()
    var_p  = p_samples.var()
    fano_p = var_p / mean_p if mean_p > 0 else 0
    cv_p   = np.sqrt(var_p) / mean_p if mean_p > 0 else 0

    # ── Figure ────────────────────────────────────────────────────────────────
    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'Gillespie SSA — {model_name.capitalize()} Gene Expression',
                 color='white', fontsize=13, fontweight='bold')

    gs = gridspec.GridSpec(2, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.50, wspace=0.35)

    ax_traj   = fig.add_subplot(gs[0, :2])
    ax_p_dist = fig.add_subplot(gs[1, 0])
    ax_m_dist = fig.add_subplot(gs[1, 1])
    ax_info   = fig.add_subplot(gs[:, 2])

    for ax in [ax_traj, ax_p_dist, ax_m_dist, ax_info]:
        ax.set_facecolor('#0f3460')

    # ── Time series (multiple trajectories) ───────────────────────────────────
    cmap = plt.cm.viridis
    for i, (times, m_traj, p_traj) in enumerate(trajs[:min(10, n_traj)]):
        alpha = 0.6 if n_traj <= 5 else 0.35
        lw = 1.2 if n_traj <= 5 else 0.7
        ax_traj.step(times, p_traj, where='post',
                     color=cmap(i / 10), lw=lw, alpha=alpha)

    # Mean trajectory via interpolation
    t_grid = np.linspace(0, t_max, 1000)
    p_interp = np.zeros((n_traj, 1000))
    for i, (times, m_traj, p_traj) in enumerate(trajs):
        idx = np.searchsorted(times, t_grid) - 1
        idx = np.clip(idx, 0, len(p_traj) - 1)
        p_interp[i] = p_traj[idx]

    mean_traj = p_interp.mean(axis=0)
    ax_traj.plot(t_grid, mean_traj, color='white', lw=2.5, alpha=0.9,
                 label=f'Mean (n={n_traj} trajs)', zorder=5)

    ax_traj.set_xlabel('Time (a.u.)', color='white')
    ax_traj.set_ylabel('Protein molecules', color='white')
    ax_traj.set_title(f'SSA Trajectories  (each line = one cell)',
                      color='white', fontsize=9)
    ax_traj.tick_params(colors='white')
    ax_traj.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
    for sp in ax_traj.spines.values():
        sp.set_edgecolor('#888')

    # ── Protein distribution ──────────────────────────────────────────────────
    max_p = max(p_samples.max(), 1)
    bins = np.arange(0, max_p + 2)
    counts, _ = np.histogram(p_samples, bins=bins)
    ax_p_dist.bar(bins[:-1], counts / len(p_samples),
                  color='#8be9fd', edgecolor='white', width=0.8, alpha=0.85,
                  label='SSA distribution')

    # Poisson reference
    if model_name == 'simple':
        pois_mean = mean_p
        pois_k = np.arange(0, max_p + 2)
        pois_pmf = poisson.pmf(pois_k, pois_mean)
        ax_p_dist.plot(pois_k, pois_pmf, 'o-', color='#f5a623', lw=1.5, ms=4,
                       label=f'Poisson(μ={pois_mean:.1f})')

    ax_p_dist.axvline(mean_p, color='#ff79c6', lw=1.5, ls='--',
                      label=f'Mean={mean_p:.1f}')
    ax_p_dist.set_xlabel('Protein count', color='white')
    ax_p_dist.set_ylabel('Probability', color='white')
    ax_p_dist.set_title(f'Protein Distribution\n(Fano={fano_p:.2f}, CV={cv_p:.2f})',
                        color='white', fontsize=9)
    ax_p_dist.tick_params(colors='white')
    ax_p_dist.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_p_dist.spines.values():
        sp.set_edgecolor('#888')

    # ── mRNA distribution ─────────────────────────────────────────────────────
    max_m = max(m_samples.max(), 1)
    bins_m = np.arange(0, max_m + 2)
    counts_m, _ = np.histogram(m_samples, bins=bins_m)
    ax_m_dist.bar(bins_m[:-1], counts_m / len(m_samples),
                  color='#50fa7b', edgecolor='white', width=0.8, alpha=0.85,
                  label='SSA distribution')
    mean_m = m_samples.mean()
    var_m = m_samples.var()
    fano_m = var_m / mean_m if mean_m > 0 else 0

    # Poisson reference for mRNA
    pois_k_m = np.arange(0, max_m + 2)
    ax_m_dist.plot(pois_k_m, poisson.pmf(pois_k_m, mean_m), 'o-',
                   color='#f5a623', lw=1.5, ms=4,
                   label=f'Poisson(μ={mean_m:.1f})')
    ax_m_dist.axvline(mean_m, color='#ff79c6', lw=1.5, ls='--',
                      label=f'Mean={mean_m:.1f}')

    ax_m_dist.set_xlabel('mRNA count', color='white')
    ax_m_dist.set_ylabel('Probability', color='white')
    ax_m_dist.set_title(f'mRNA Distribution\n(Fano={fano_m:.2f})',
                        color='white', fontsize=9)
    ax_m_dist.tick_params(colors='white')
    ax_m_dist.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
    for sp in ax_m_dist.spines.values():
        sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    det_mean_m = (params.get('k_tx', params.get('k0', 0.5)) /
                  params.get('k_deg_m', 0.5))
    det_mean_p = (det_mean_m * params.get('k_tl', 5.0) /
                  params.get('k_deg_p', 0.05))

    info = (
        f"Model: {model_name}\n\n"
        f"{model['description']}\n\n"
        f"Measured statistics:\n"
        f"  Protein:\n"
        f"    mean  = {mean_p:.1f}\n"
        f"    var   = {var_p:.1f}\n"
        f"    Fano  = {fano_p:.2f}  (1=Poisson)\n"
        f"    CV    = {cv_p:.2f}\n\n"
        f"  mRNA:\n"
        f"    mean  = {mean_m:.1f}\n"
        f"    Fano  = {fano_m:.2f}\n\n"
        f"Deterministic prediction:\n"
        f"  mRNA mean  ≈ {det_mean_m:.1f}\n"
        f"  Prot mean  ≈ {det_mean_p:.1f}\n\n"
        f"SSA: n_traj = {n_traj}\n"
        f"     t_max  = {t_max}"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=8, color='white', fontfamily='monospace',
                 linespacing=1.45)

    plt.show()

    print(f"\nModel: {model_name}")
    print(f"Protein: mean={mean_p:.1f}, var={var_p:.1f}, Fano={fano_p:.2f}, CV={cv_p:.2f}")
    print(f"mRNA: mean={mean_m:.1f}, Fano={fano_m:.2f}")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Gillespie SSA gene expression')
    parser.add_argument('--model', default='bursting',
                        choices=['simple', 'bursting', 'feedback'])
    parser.add_argument('--n_traj', type=int, default=50)
    parser.add_argument('--t_max', type=float, default=800)
    args = parser.parse_args()

    print("Gillespie SSA: Stochastic Gene Expression")
    print("=" * 50)
    run(model_name=args.model, n_traj=args.n_traj, t_max=args.t_max)
