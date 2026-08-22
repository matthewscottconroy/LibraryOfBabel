# Parameter Uncertainty Propagation Across Scales

## The Compounding Uncertainty Problem

Here is a simple but sobering thought: every number in a biological model has an error bar. The $K_M$ you pulled from BRENDA was measured in a different organism at a different temperature. The mRNA half-life came from a microarray experiment that measured dozens of genes simultaneously. The protein synthesis rate was inferred from ribosome profiling data with its own noise floor. In a single-scale model, you can propagate these uncertainties and get a sense of how confident you should be in your predictions.

Now add a second scale. The output of your first model — already uncertain — feeds in as the input of your second model. Its own parameters add more uncertainty on top. Every scale interface multiplies the problem. By the time you are looking at a prediction five levels of abstraction removed from your measurements, you may have very little idea whether your model is telling you something true.

Every model parameter has uncertainty — arising from measurement noise, biological variability, species differences in literature-derived values, or simply lack of data requiring estimation. In a single-scale model, uncertainty in parameters produces uncertainty in model predictions. This can be quantified by standard techniques: Monte Carlo sampling, profile likelihood, or Bayesian inference.

In a **multiscale model**, uncertainty compounds across scale interfaces. If a fine-scale model provides parameters to a coarse-scale model, the uncertainty from the fine-scale model propagates into the coarse-scale predictions. For a chain of three scale interfaces, each adding its own uncertainty, the total uncertainty at the highest scale can be orders of magnitude larger than any single-level uncertainty.

Formally, consider a multiscale model where:
- Level 1 has parameters $\theta_1$ with uncertainty $\sigma_1$
- Level 2 takes output $y_1(\theta_1)$ as its input and has parameters $\theta_2$ with uncertainty $\sigma_2$
- Level 3 takes output $y_2(\theta_2, y_1)$ as its input

The uncertainty in level 3 prediction $y_3$ is approximately:

$$\text{Var}(y_3) \approx \left(\frac{\partial y_3}{\partial y_2}\right)^2 \left(\frac{\partial y_2}{\partial y_1}\right)^2 \text{Var}(y_1) + \left(\frac{\partial y_3}{\partial y_2}\right)^2 \text{Var}(y_2|y_1) + \text{Var}(y_3|y_2)$$

If each level has multiplicative uncertainty factor $f > 1$, total uncertainty scales as $f^n$ for $n$ levels — **exponential compounding**.

## Variance Propagation: The Delta Method

For a differentiable function $y = g(\theta)$, uncertainty in $\theta$ propagates to uncertainty in $y$ via the **delta method** (first-order Taylor expansion):

$$\text{Var}(y) \approx \sum_i \left(\frac{\partial g}{\partial \theta_i}\right)^2 \text{Var}(\theta_i) + \sum_{i \neq j} \frac{\partial g}{\partial \theta_i} \frac{\partial g}{\partial \theta_j} \text{Cov}(\theta_i, \theta_j)$$

For uncorrelated parameters, this simplifies to:

$$\text{Var}(y) \approx \sum_i \left(\frac{\partial g}{\partial \theta_i}\right)^2 \sigma_i^2$$

The gradient terms $\partial g / \partial \theta_i$ are **local sensitivity coefficients** — they quantify how much output changes per unit change in input. The delta method provides a fast, analytical approximation to uncertainty propagation.

```python
import numpy as np
from scipy.optimize import approx_fprime

def propagate_uncertainty_delta_method(model_func, params, param_uncertainties):
    """
    Propagate parameter uncertainty using the delta method.
    
    model_func: function(params) -> scalar output
    params: nominal parameter values (array)
    param_uncertainties: standard deviations of parameters (array)
    Returns: (predicted_output, output_uncertainty)
    """
    y_nominal = model_func(params)
    
    # Compute gradient numerically
    eps = params * 1e-5 + 1e-8  # step sizes
    gradient = approx_fprime(params, model_func, eps)
    
    # Variance propagation
    var_y = np.sum((gradient * param_uncertainties)**2)
    sigma_y = np.sqrt(var_y)
    
    return y_nominal, sigma_y

# Example: uncertainty propagation in Michaelis-Menten
def mm_velocity(params):
    """v = Vmax * S / (Km + S)"""
    Vmax, Km, S = params
    return Vmax * S / (Km + S)

params = np.array([10.0, 0.5, 1.0])  # Vmax, Km, S
uncertainties = np.array([1.0, 0.1, 0.05])  # ±10% on Vmax, ±20% on Km, ±5% on S

v, sigma_v = propagate_uncertainty_delta_method(mm_velocity, params, uncertainties)
print(f"Predicted velocity: {v:.3f} ± {sigma_v:.3f} (1σ)")
print(f"Relative uncertainty: {sigma_v/v*100:.1f}%")
```

## Monte Carlo Uncertainty Propagation

For nonlinear models where the delta method is inaccurate, **Monte Carlo uncertainty propagation** samples parameter distributions and empirically characterizes the output distribution:

```python
import numpy as np
from scipy.integrate import solve_ivp

def mc_uncertainty_propagation(model, param_distributions, n_samples=1000):
    """
    Monte Carlo uncertainty propagation.
    
    model: function(params) -> predicted output
    param_distributions: list of scipy.stats distributions for each parameter
    n_samples: number of Monte Carlo samples
    """
    outputs = []
    samples = np.column_stack([dist.rvs(n_samples) 
                                for dist in param_distributions])
    
    for sample in samples:
        try:
            y = model(sample)
            outputs.append(y)
        except Exception:
            pass  # skip failed model evaluations
    
    outputs = np.array(outputs)
    return {
        'mean': np.mean(outputs),
        'std': np.std(outputs),
        'percentile_5': np.percentile(outputs, 5),
        'percentile_95': np.percentile(outputs, 95),
        'samples': outputs
    }

# Apply to a two-level multiscale model
from scipy import stats

# Level 1: molecular dynamics provides Km with uncertainty
Km_distribution = stats.lognormal(s=0.3, scale=0.5)  # log-normal, CV=30%

# Level 2: kinetic model uses Km to predict flux, which determines growth rate
def growth_rate_model(params):
    """Two-level model: Km → enzyme flux → growth rate"""
    Km, Vmax, substrate = params
    flux = Vmax * substrate / (Km + substrate)
    # Simplified: growth rate proportional to flux
    mu = 0.3 * flux / (0.01 + flux)  # additional saturation
    return mu

param_dists = [
    stats.lognormal(s=0.3, scale=0.5),   # Km
    stats.lognormal(s=0.2, scale=10.0),  # Vmax
    stats.uniform(0.5, 0.5)              # substrate concentration range
]

results = mc_uncertainty_propagation(growth_rate_model, param_dists, n_samples=5000)
print(f"Growth rate: {results['mean']:.4f} h⁻¹")
print(f"90% CI: [{results['percentile_5']:.4f}, {results['percentile_95']:.4f}]")
print(f"CV: {results['std']/results['mean']*100:.1f}%")
```

## Global Sensitivity Analysis

When many parameters contribute to output uncertainty, identifying the **most important parameters** guides experimental prioritization. **Global sensitivity analysis (GSA)** quantifies each parameter's contribution to total output variance across the full parameter space (not just at the nominal values).

**Sobol indices** (Saltelli et al. 2010) decompose output variance into contributions from each parameter and their interactions:

$$\text{Var}(Y) = \sum_i V_i + \sum_{i<j} V_{ij} + \ldots$$

where $V_i = \text{Var}_{X_i}(\mathbb{E}[Y|X_i])$ is the first-order Sobol index for parameter $i$, and $V_{ij}$ captures interaction effects.

```python
from SALib.sample import saltelli
from SALib.analyze import sobol

# Define parameter ranges for Sobol analysis
problem = {
    'num_vars': 4,
    'names': ['Km', 'Vmax', 'k_deg', 'substrate'],
    'bounds': [[0.01, 2.0],    # Km range (mM)
               [1.0, 100.0],   # Vmax range
               [0.01, 1.0],    # degradation rate
               [0.1, 10.0]]    # substrate concentration
}

# Generate Saltelli samples
param_values = saltelli.sample(problem, 1024)

# Evaluate model at each sample
outputs = np.array([growth_rate_model_4param(pv) for pv in param_values])

# Compute Sobol indices
Si = sobol.analyze(problem, outputs, print_to_console=False)

print("First-order Sobol indices:")
for name, S1 in zip(problem['names'], Si['S1']):
    print(f"  {name}: {S1:.3f}")
print("\nTotal-order Sobol indices:")
for name, ST in zip(problem['names'], Si['ST']):
    print(f"  {name}: {ST:.3f}")
# High S1 → parameter is individually important
# ST >> S1 → parameter is important mainly through interactions
```

## Uncertainty Reduction Strategy

GSA identifies which parameters most strongly drive output uncertainty. This provides a **prioritized experimental agenda**:

1. Parameters with highest Sobol indices → most important to measure precisely
2. Parameters with low Sobol indices → additional precision not cost-effective
3. High-interaction parameters (ST >> S1) → must be measured in combination with other key parameters

For a multiscale cell biology model, typical findings:
- A few "super-sensitive" parameters (often rate-limiting kinetic constants, key regulatory thresholds) dominate predictions
- Many parameters (far from rate-limiting steps, operating in linear regime) can vary over orders of magnitude without affecting predictions
- Parameter interactions are common at regulatory branch points

## Practical Approaches to Managing Uncertainty

**Ensemble modeling**: maintain a population of parameter sets consistent with experimental data; report ensemble-averaged predictions with confidence bounds.

**Bayesian model updating**: as new data arrives, update parameter distributions using Bayes' theorem. Narrows uncertainty where data constrains it; preserves uncertainty where data is absent.

**Experimental design for uncertainty reduction**: use uncertainty/sensitivity analysis to identify the single experiment that most reduces total output uncertainty — optimal experimental design.

## Why This Matters

Uncertainty quantification distinguishes computational biology that makes reliable predictions from analyses that produce precise-looking numbers without meaningful confidence. In pharmaceutical development, the cost of a failed clinical trial (\$500M+) is justified in part by the expectation that computational models will provide reliable predictions of drug effects. Those predictions are only reliable if their uncertainty is properly quantified and traced to specific mechanistic assumptions. Developing facility with uncertainty propagation methods is as important as developing facility with the models themselves — the uncertainty is where the science lives.
