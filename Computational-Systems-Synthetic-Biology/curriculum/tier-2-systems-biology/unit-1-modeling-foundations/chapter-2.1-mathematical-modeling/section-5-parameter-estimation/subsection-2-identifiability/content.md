# Identifiability: Can Parameters Be Determined from Data?

## The Identifiability Problem

Imagine you have built a model of a signaling pathway with ten parameters and you have carefully measured the output protein level over time with good precision. You run your optimizer, it converges to a beautiful fit, and you report the ten parameter values. But here is the disturbing possibility: maybe six of those ten parameters cannot actually be determined from your data, no matter how precise your measurements or how long your optimizer runs. Their values could each vary by orders of magnitude in different combinations while producing identical model predictions.

A parameter is **identifiable** if it can, in principle, be uniquely determined from experimental data. Non-identifiability is a fundamental limitation: even with perfect, noise-free data and unlimited computational resources, non-identifiable parameters cannot be estimated. This is not an experimental shortcoming — it is a structural property of the model and what is observed.

Recognizing non-identifiability before fitting a model saves enormous effort and prevents the false confidence of well-fitting parameters that are actually underdetermined. It is one of the most commonly neglected checks in published biological modeling studies.

## Structural vs. Practical Identifiability

**Structural identifiability** asks: can the parameters be uniquely determined from *ideal* data (infinite precision, all observables measurable, no noise)? This is a mathematical property of the model equations and observation function.

**Practical identifiability** asks: can the parameters be estimated with acceptable precision from *real* experimental data (finite number of noisy measurements)? Even a structurally identifiable parameter may be practically non-identifiable if the data are too sparse or noisy to constrain it.

These are conceptually different questions, and confusing them leads to different errors. Structural non-identifiability is an absolute limitation — more data cannot fix it, only better experimental design (measuring different observables) can. Practical non-identifiability can potentially be addressed by collecting more data or reducing measurement noise.

## Structural Identifiability Analysis

**Differential algebra method**: Eliminate unobserved state variables from the model equations to obtain input-output relations involving only observables and parameters. If two distinct parameter sets give identical input-output relations, the model is not structurally identifiable.

**Example**: Consider a two-compartment pharmacokinetic model:

$$\frac{dx_1}{dt} = -k_{10} x_1 - k_{12} x_1 + k_{21} x_2 + u(t)$$

$$\frac{dx_2}{dt} = k_{12} x_1 - k_{21} x_2$$

If only $x_1$ is observed, it can be shown by computing the transfer function that the three parameters $k_{10}$, $k_{12}$, $k_{21}$ are structurally identifiable from $x_1$ data with known input $u$.

**Tools**:
- **DAISY** (MATLAB): differential algebra approach; applies to polynomial ODE systems
- **SIAN** (Julia): efficient for large systems; parallelizable
- **StructuralIdentifiability.jl** (Julia): state-of-the-art for large models

```python
# Conceptual: Check identifiability by comparing predictions at two parameter sets
import numpy as np
from scipy.integrate import solve_ivp

def two_state_model(t, y, k_on, k_off, alpha, delta):
    """Two-state promoter: [m_ON, m_OFF, mRNA_count]"""
    # Simple check: can k_on and k_off be separately identified from mRNA data?
    m = y[0]  # mRNA count
    # If only mRNA is observed, effective production rate is alpha*k_on/(k_on+k_off)
    # and k_on, k_off appear only as their ratio -> NOT identifiable separately
    # from steady-state data alone; dynamics could disambiguate them
    eff_alpha = alpha * k_on / (k_on + k_off)
    return [eff_alpha - delta * m]

# Test: two parameter sets with same ratio k_on/k_off
params_A = dict(k_on=0.1, k_off=0.9, alpha=10, delta=1)  # k_on/(k_on+k_off) = 0.1
params_B = dict(k_on=0.2, k_off=1.8, alpha=10, delta=1)  # same ratio

# Both predict identical steady-state mRNA = 1.0
for name, p in [('A', params_A), ('B', params_B)]:
    ss = p['alpha'] * p['k_on'] / (p['delta'] * (p['k_on'] + p['k_off']))
    print(f"Set {name}: steady-state mRNA = {ss:.2f}")
# -> steady-state data alone cannot distinguish k_on from k_off
# Dynamic (switching) data would be required
```

This example illustrates a general principle: when parameters appear only as a ratio in the observables, they are not individually identifiable from that observable. Here, $k_\text{on}$ and $k_\text{off}$ appear only as $k_\text{on}/(k_\text{on}+k_\text{off})$ in the steady-state mRNA level — so doubling both rates while keeping their ratio constant produces identical predictions. To separately identify them, you would need to measure the switching dynamics directly (e.g., single-cell time-lapse microscopy showing individual switching events).

## Practical Identifiability: Profile Likelihood

The **profile likelihood** is the most principled tool for assessing practical identifiability. For each parameter $\theta_i$ of interest:

1. Fix $\theta_i$ at a series of values $\theta_i^{(k)}$ spanning its plausible range.
2. For each fixed $\theta_i^{(k)}$, optimize all other parameters: $J_\text{profile}(\theta_i^{(k)}) = \min_{\boldsymbol{\theta}_{-i}} J(\boldsymbol{\theta})$.
3. Plot $J_\text{profile}(\theta_i)$ as a function of $\theta_i$.

**Interpretation**:
- **Identifiable parameter**: profile has a clear minimum; the likelihood drops steeply on both sides → finite confidence interval.
- **Non-identifiable parameter (flat profile)**: the likelihood does not increase no matter how far $\theta_i$ is varied in one or both directions → the parameter cannot be constrained by this data.
- **Practically non-identifiable**: profile has a minimum but the curvature is very low → wide confidence interval.

Confidence intervals from profile likelihood use the likelihood ratio test: the 95% CI contains all $\theta_i$ values where $J_\text{profile}(\theta_i) - J(\boldsymbol{\theta}^*) \leq \chi^2_{1,0.95} = 3.84$.

```python
from scipy.optimize import minimize
import matplotlib.pyplot as plt

def profile_likelihood(param_idx, param_values, theta_opt, obj_func, n_params):
    """Compute profile likelihood by fixing one parameter and optimizing others."""
    profile = []
    for val in param_values:
        def constrained_obj(theta_free):
            theta_full = theta_opt.copy()
            free_idx = [i for i in range(n_params) if i != param_idx]
            for k, idx in enumerate(free_idx):
                theta_full[idx] = theta_free[k]
            theta_full[param_idx] = val
            return obj_func(theta_full)
        
        theta0_free = theta_opt[[i for i in range(n_params) if i != param_idx]]
        res = minimize(constrained_obj, theta0_free, method='Nelder-Mead')
        profile.append(res.fun)
    
    return np.array(profile)
```

The profile likelihood is more reliable than the Hessian-based (Fisher information) confidence interval for nonlinear models. The Hessian approximation assumes the objective function is locally quadratic (parabolic) around the optimum, which is often a poor assumption for biological ODE models with correlated parameters.

## Structural Non-Identifiability: Common Causes

**Parameter ratios only appear in data**: if only the ratio $\theta_1/\theta_2$ appears in the observable, the individual parameters are not separately identifiable.

**Hidden compartments**: unobserved state variables introduce structural non-identifiability. The states can be reparameterized without affecting observables.

**Redundant pathways**: if two parallel reactions produce the same product, only their sum (total flux) is identifiable, not the individual rates.

**Solutions**:
- **Fix non-identifiable parameters** at literature values; report this assumption explicitly.
- **Reparameterize**: work with identifiable combinations (e.g., $\alpha/\delta$ rather than $\alpha$ and $\delta$ separately when only steady state is observed).
- **Design informative experiments**: add time points, change observables, or apply perturbations that break the non-identifiability. This is the most powerful solution — it transforms a model that cannot be estimated into one that can, by choosing experiments that contain information about the previously underdetermined parameters.

## Why This Matters

Identifiability analysis is a prerequisite for trustworthy parameter estimation. A model that appears to fit data well with a unique optimum may be concealing parameter combinations that are fundamentally underdetermined — and will fail to generalize when the model is used to predict responses to novel perturbations.

In practice, many published ODE models of biological systems are structurally or practically non-identifiable, meaning that the reported "best-fit" parameters should be interpreted with caution. The problem is particularly acute for large models with many parameters: a 50-parameter ODE model fit to 30 time points is almost certainly non-identifiable in several parameter combinations, and reporting 50 "best-fit" values without identifiability analysis is scientifically misleading.

Building identifiability analysis into the model-building workflow — not as an afterthought — is the mark of rigorous quantitative biology. It forces you to ask, before you fit, whether the data you have can actually answer the question you are trying to answer. Often the answer is "no — you need a different experiment," which is exactly the insight that moves the science forward.
