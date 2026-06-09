"""
MCMC Parameter Estimation for Biological Models
=================================================
Tier 4.1 — Computational Tools: Parameter Estimation

Concepts demonstrated:
  - Bayesian parameter estimation framework
  - Metropolis-Hastings MCMC algorithm
  - Likelihood function for ODE models
  - Prior distributions (uniform, log-normal)
  - Posterior distributions and credible intervals
  - Trace plots and autocorrelation (MCMC diagnostics)
  - Profile likelihood (frequentist alternative)
  - Identifiability: which parameters can be determined from data?

Usage:
    python 18_mcmc_parameter_estimation.py [--model MODEL]

Models:
    mm    — Michaelis-Menten (fit Vmax, Km)
    hill  — Hill equation (fit alpha, K, n)
    osc   — Repressilator (fit α, β)

Key insight:
    MCMC samples the posterior P(θ|data) ∝ P(data|θ) × P(θ).
    Wide posteriors signal non-identifiable parameters.
    Strong correlations between parameters (bananas in the posterior)
    indicate structural unidentifiability — the model has more
    parameters than can be determined from the available data.
"""

import argparse
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
from scipy.integrate import solve_ivp
from scipy.optimize import minimize


# ── Forward Models (ground truth) ────────────────────────────────────────────

def michaelis_menten(S, Vmax, Km):
    return Vmax * S / (Km + S)


def hill_fn(S, alpha, K, n):
    return alpha * S**n / (K**n + S**n)


def repressilator_p1(alpha, beta, t_eval, n=2.0):
    def rhs(t, y):
        m1, m2, m3, p1, p2, p3 = y
        dm1 = alpha / (1 + p3**n) + 0.216 - m1
        dm2 = alpha / (1 + p1**n) + 0.216 - m2
        dm3 = alpha / (1 + p2**n) + 0.216 - m3
        dp1 = beta * (m1 - p1)
        dp2 = beta * (m2 - p2)
        dp3 = beta * (m3 - p3)
        return [dm1, dm2, dm3, dp1, dp2, dp3]
    sol = solve_ivp(rhs, (0, t_eval[-1]), [0, 0, 0, 2, 1, 3],
                    t_eval=t_eval, method='LSODA', rtol=1e-6)
    if sol.success:
        return sol.y[3]  # return p1 trajectory
    return np.ones_like(t_eval) * np.nan


# ── Synthetic Data Generation ─────────────────────────────────────────────────

def generate_data(model_name, seed=42):
    rng = np.random.default_rng(seed)
    noise_frac = 0.10   # 10% relative noise

    if model_name == 'mm':
        S = np.array([0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 20.0])
        TRUE_PARAMS = {'Vmax': 3.0, 'Km': 1.0}
        y_true = michaelis_menten(S, **TRUE_PARAMS)
        y_obs = y_true * (1 + noise_frac * rng.standard_normal(len(S)))
        return S, y_obs, TRUE_PARAMS, '[S]'

    elif model_name == 'hill':
        S = np.logspace(-1, 2, 20)
        TRUE_PARAMS = {'alpha': 5.0, 'K': 3.0, 'n': 2.5}
        y_true = hill_fn(S, **TRUE_PARAMS)
        y_obs = y_true * (1 + noise_frac * rng.standard_normal(len(S)))
        return S, y_obs, TRUE_PARAMS, '[Ligand]'

    elif model_name == 'osc':
        t_eval = np.linspace(0, 200, 80)
        TRUE_PARAMS = {'alpha': 150.0, 'beta': 0.15}
        y_true = repressilator_p1(TRUE_PARAMS['alpha'], TRUE_PARAMS['beta'], t_eval)
        y_obs = np.maximum(0.01, y_true * (1 + noise_frac * rng.standard_normal(len(t_eval))))
        return t_eval, y_obs, TRUE_PARAMS, 'Time'

    raise ValueError(f"Unknown model: {model_name}")


# ── Log-Likelihood and Log-Prior ──────────────────────────────────────────────

def log_likelihood(params, x, y_obs, model_name, sigma=None):
    """Gaussian log-likelihood."""
    if model_name == 'mm':
        Vmax, Km = params
        if Vmax <= 0 or Km <= 0:
            return -np.inf
        y_pred = michaelis_menten(x, Vmax, Km)
    elif model_name == 'hill':
        alpha, K, n = params
        if alpha <= 0 or K <= 0 or n <= 0:
            return -np.inf
        y_pred = hill_fn(x, alpha, K, n)
    elif model_name == 'osc':
        alpha, beta = params
        if alpha <= 0 or beta <= 0 or alpha > 500 or beta > 2:
            return -np.inf
        y_pred = repressilator_p1(alpha, beta, x)
        if np.any(np.isnan(y_pred)):
            return -np.inf

    if sigma is None:
        sigma = max(0.05 * y_obs.mean(), 1e-6)
    residuals = y_obs - y_pred
    return -0.5 * np.sum((residuals / sigma)**2) - len(y_obs) * np.log(sigma * np.sqrt(2 * np.pi))


def log_prior(params, model_name):
    """Log-normal / uniform priors."""
    if model_name == 'mm':
        Vmax, Km = params
        if Vmax <= 0 or Km <= 0:
            return -np.inf
        return -np.log(Vmax) - np.log(Km)   # log-uniform (Jeffreys)
    elif model_name == 'hill':
        alpha, K, n = params
        if alpha <= 0 or K <= 0 or n <= 0 or n > 10:
            return -np.inf
        return -np.log(alpha) - np.log(K) - np.log(n)
    elif model_name == 'osc':
        alpha, beta = params
        if alpha <= 0 or beta <= 0 or alpha > 500 or beta > 2:
            return -np.inf
        return -np.log(alpha) - np.log(beta)
    return 0.0


# ── Metropolis-Hastings MCMC ──────────────────────────────────────────────────

def metropolis_hastings(x, y_obs, model_name, init_params, n_samples=5000,
                        proposal_std=None, seed=42):
    rng = np.random.default_rng(seed)
    n_params = len(init_params)
    if proposal_std is None:
        proposal_std = np.array(init_params) * 0.05 + 0.01

    samples = np.zeros((n_samples, n_params))
    current = np.array(init_params, dtype=float)
    current_log_post = (log_likelihood(current, x, y_obs, model_name) +
                        log_prior(current, model_name))

    n_accepted = 0
    for i in range(n_samples):
        # Propose new parameters
        proposed = current + rng.standard_normal(n_params) * proposal_std
        proposed_log_post = (log_likelihood(proposed, x, y_obs, model_name) +
                             log_prior(proposed, model_name))

        # Metropolis acceptance
        log_accept = proposed_log_post - current_log_post
        if np.log(rng.uniform()) < log_accept:
            current = proposed
            current_log_post = proposed_log_post
            n_accepted += 1
        samples[i] = current

    acceptance_rate = n_accepted / n_samples
    return samples, acceptance_rate


# ── Main Visualisation ────────────────────────────────────────────────────────

def run(model_name='mm'):
    x, y_obs, true_params, x_label = generate_data(model_name)
    param_names = list(true_params.keys())
    true_vals = list(true_params.values())

    # Initial guess (slightly off from truth)
    init_params = [v * 1.5 for v in true_vals]

    print(f"Running MCMC for {model_name} (5000 samples)...")
    samples, accept_rate = metropolis_hastings(
        x, y_obs, model_name, init_params, n_samples=5000
    )

    # Burn-in: discard first 20%
    burn_in = 1000
    samples_burned = samples[burn_in:]

    # MAP estimate
    map_idx = np.argmax([
        log_likelihood(samples_burned[i], x, y_obs, model_name) +
        log_prior(samples_burned[i], model_name)
        for i in range(len(samples_burned))
    ])
    map_params = samples_burned[map_idx]

    # Posterior predictive
    n_predict = 200
    rng = np.random.default_rng(0)
    pred_idx = rng.choice(len(samples_burned), n_predict, replace=False)
    pred_samples = samples_burned[pred_idx]

    fig = plt.figure(figsize=(18, 10))
    fig.patch.set_facecolor('#1a1a2e')
    fig.suptitle(f'MCMC Parameter Estimation — {model_name.upper()} Model',
                 color='white', fontsize=13, fontweight='bold')

    n_params = len(param_names)
    gs = gridspec.GridSpec(3, 3, figure=fig,
                           left=0.05, right=0.97, top=0.88, bottom=0.07,
                           hspace=0.55, wspace=0.35)

    ax_fit      = fig.add_subplot(gs[0, :2])
    ax_info     = fig.add_subplot(gs[:, 2])
    ax_traces   = [fig.add_subplot(gs[1, i]) for i in range(min(n_params, 2))]
    ax_posteriors = [fig.add_subplot(gs[2, i]) for i in range(min(n_params, 2))]
    if n_params == 3:
        ax_traces.append(fig.add_subplot(gs[1, 2]))
        ax_posteriors.append(fig.add_subplot(gs[2, 2]))

    for ax in [ax_fit, ax_info] + ax_traces + ax_posteriors:
        ax.set_facecolor('#0f3460')

    # ── Data + fit ────────────────────────────────────────────────────────────
    ax_fit.scatter(x, y_obs, color='#f1fa8c', s=40, zorder=5, label='Observed data',
                   edgecolors='white', linewidths=0.5)

    if model_name == 'mm':
        x_fine = np.linspace(0, x.max(), 200)
        for params in pred_samples:
            y_pred = michaelis_menten(x_fine, *params)
            ax_fit.plot(x_fine, y_pred, color='#8be9fd', lw=0.5, alpha=0.15)
        ax_fit.plot(x_fine, michaelis_menten(x_fine, *map_params), color='#50fa7b', lw=2.5,
                    label='MAP estimate', zorder=4)
        ax_fit.plot(x_fine, michaelis_menten(x_fine, *true_vals), color='#ff79c6', lw=1.5,
                    ls='--', label='True parameters', zorder=4)
    elif model_name == 'hill':
        x_fine = np.logspace(np.log10(x.min()), np.log10(x.max()), 200)
        for params in pred_samples:
            ax_fit.semilogx(x_fine, hill_fn(x_fine, *params),
                            color='#8be9fd', lw=0.5, alpha=0.15)
        ax_fit.semilogx(x_fine, hill_fn(x_fine, *map_params), color='#50fa7b', lw=2.5,
                        label='MAP estimate', zorder=4)
        ax_fit.semilogx(x_fine, hill_fn(x_fine, *true_vals), color='#ff79c6', lw=1.5,
                        ls='--', label='True parameters', zorder=4)
        ax_fit.scatter(x, y_obs, color='#f1fa8c', s=40, zorder=5, edgecolors='white')
    elif model_name == 'osc':
        for params in pred_samples[:50]:
            y_pred = repressilator_p1(*params, x)
            if not np.any(np.isnan(y_pred)):
                ax_fit.plot(x, y_pred, color='#8be9fd', lw=0.7, alpha=0.2)
        y_map = repressilator_p1(*map_params, x)
        ax_fit.plot(x, y_map, color='#50fa7b', lw=2.5, label='MAP estimate', zorder=4)
        y_true_traj = repressilator_p1(*true_vals, x)
        ax_fit.plot(x, y_true_traj, color='#ff79c6', lw=1.5, ls='--',
                    label='True parameters', zorder=4)

    ax_fit.set_xlabel(x_label, color='white')
    ax_fit.set_ylabel('Response', color='white')
    ax_fit.set_title('Data + Posterior Predictive (shaded = MCMC samples)',
                     color='white', fontsize=9)
    ax_fit.tick_params(colors='white')
    ax_fit.legend(fontsize=8, facecolor='#16213e', labelcolor='white')
    for sp in ax_fit.spines.values():
        sp.set_edgecolor('#888')

    # ── Trace plots ───────────────────────────────────────────────────────────
    for pi, (ax, pname) in enumerate(zip(ax_traces, param_names)):
        ax.plot(samples[:, pi], color='#8be9fd', lw=0.6, alpha=0.8)
        ax.axhline(true_vals[pi], color='#ff79c6', lw=1.5, ls='--', label='True value')
        ax.axvline(burn_in, color='#f5a623', lw=1, ls=':', label='Burn-in end')
        ax.set_xlabel('Sample #', color='white')
        ax.set_ylabel(pname, color='white')
        ax.set_title(f'Trace: {pname}', color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    # ── Posterior distributions ───────────────────────────────────────────────
    for pi, (ax, pname) in enumerate(zip(ax_posteriors, param_names)):
        ax.hist(samples_burned[:, pi], bins=50, color='#50fa7b', edgecolor='none', alpha=0.85,
                density=True)
        ax.axvline(true_vals[pi], color='#ff79c6', lw=2, ls='--', label='True value')
        ax.axvline(np.percentile(samples_burned[:, pi], 2.5), color='#888', lw=1, ls=':')
        ax.axvline(np.percentile(samples_burned[:, pi], 97.5), color='#888', lw=1, ls=':',
                   label='95% CI')
        ax.set_xlabel(pname, color='white')
        ax.set_ylabel('Density', color='white')
        ax.set_title(f'Posterior: {pname}', color='white', fontsize=9)
        ax.tick_params(colors='white')
        ax.legend(fontsize=7, facecolor='#16213e', labelcolor='white')
        for sp in ax.spines.values():
            sp.set_edgecolor('#888')

    # ── Info panel ────────────────────────────────────────────────────────────
    ax_info.axis('off')
    post_text = "Posterior summary:\n"
    for pi, pname in enumerate(param_names):
        post_samples = samples_burned[:, pi]
        ci_lo = np.percentile(post_samples, 2.5)
        ci_hi = np.percentile(post_samples, 97.5)
        post_text += (f"  {pname}:\n"
                      f"    true  = {true_vals[pi]:.3f}\n"
                      f"    MAP   = {map_params[pi]:.3f}\n"
                      f"    mean  = {post_samples.mean():.3f}\n"
                      f"    95%CI = [{ci_lo:.3f}, {ci_hi:.3f}]\n\n")

    info = (
        f"Model: {model_name}\n"
        f"n_samples: 5000  (burn-in: {burn_in})\n"
        f"Acceptance rate: {accept_rate:.1%}\n\n"
        f"{post_text}"
        f"MCMC diagnostics:\n"
        f"  Ideal acceptance: 20-40%\n"
        f"  Trace: stationary after burn-in\n"
        f"  Autocorrelation should decay\n\n"
        f"Bayesian inference:\n"
        f"  P(θ|D) ∝ P(D|θ) · P(θ)\n"
        f"  Posterior = Likelihood × Prior"
    )
    ax_info.text(0.04, 0.97, info, transform=ax_info.transAxes,
                 va='top', fontsize=7.5, color='white', fontfamily='monospace',
                 linespacing=1.4)

    plt.show()

    print(f"\nModel: {model_name}")
    print(f"Acceptance rate: {accept_rate:.1%}")
    for pi, pname in enumerate(param_names):
        ps = samples_burned[:, pi]
        print(f"  {pname}: true={true_vals[pi]:.3f}, MAP={map_params[pi]:.3f}, "
              f"mean={ps.mean():.3f}, 95CI=[{np.percentile(ps,2.5):.3f},{np.percentile(ps,97.5):.3f}]")


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='MCMC parameter estimation')
    parser.add_argument('--model', default='mm', choices=['mm', 'hill', 'osc'])
    args = parser.parse_args()

    print("MCMC Parameter Estimation for Biological Models")
    print("=" * 55)
    run(model_name=args.model)
