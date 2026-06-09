# Sensitivity Analysis: Quantifying Parameter Influence

## Purpose and Types

Not all parameters in a biological model are equally important. In a model of gene expression, the transcription rate and the mRNA degradation rate might together control 90% of the variance in steady-state protein levels, while the translation rate and protein degradation rate are nearly irrelevant. Knowing this is valuable — it tells you where to focus experimental effort, which parameters need to be precisely characterized, and which can be loosely estimated without sacrificing predictive accuracy.

**Sensitivity analysis** quantifies how model outputs change in response to changes in model inputs (parameters, initial conditions). It answers the question: which parameters most influence the behavior I care about? This is valuable for:

- **Model understanding**: identify the mechanistically important parameters.
- **Experimental design**: prioritize measurements of the most influential parameters.
- **Robustness assessment**: determine which parameters must be tightly controlled versus which can vary without affecting function.
- **Uncertainty quantification**: propagate parameter uncertainty into output uncertainty.

Two fundamentally different approaches exist: **local** sensitivity (derivatives at a single point in parameter space) and **global** sensitivity (averaging over the full parameter space).

## Local Sensitivity Analysis

**Local sensitivity coefficients** are partial derivatives of outputs with respect to parameters, evaluated at the nominal parameter set $\boldsymbol{\theta}^*$:

$$s_{ij} = \frac{\partial y_i}{\partial \theta_j}\bigg|_{\boldsymbol{\theta}^*}$$

**Logarithmic (normalized) sensitivity** is more interpretable — it gives the fractional change in output per fractional change in parameter:

$$\hat{s}_{ij} = \frac{\partial \ln y_i}{\partial \ln \theta_j} = \frac{\theta_j}{y_i} \cdot \frac{\partial y_i}{\partial \theta_j}$$

A value of $\hat{s}_{ij} = 2$ means: doubling parameter $\theta_j$ approximately doubles output $y_i$ (for a 100% change in $\theta_j$, expect a ~200% change in $y_i$). A value near zero means the output is insensitive to that parameter.

For the constitutive gene expression model $m^* = \alpha/\delta$, the log-sensitivities are exactly $\hat{s}_{\alpha} = +1$ and $\hat{s}_{\delta} = -1$: a 10% increase in transcription rate causes a 10% increase in mRNA; a 10% increase in degradation causes a 10% decrease. Both parameters have equal influence, which you might have guessed from the symmetry of the formula. For more complex nonlinear models, the sensitivities can be far less obvious.

**Computing sensitivities**: For ODE models, sensitivities satisfy their own ODE system:

$$\frac{d s_{ij}}{dt} = \frac{\partial f_i}{\partial \theta_j} + \sum_k J_{ik} s_{kj}$$

where $J_{ik} = \partial f_i / \partial x_k$ is the Jacobian matrix. This can be solved simultaneously with the model ODEs (**forward sensitivity equations**) without finite-differencing, using the AMICI library in Python.

```python
import numpy as np
from scipy.integrate import solve_ivp
import matplotlib.pyplot as plt

def sensitivity_gene_expression(alpha, delta, t_span, t_eval, eps=1e-6):
    """Compute local log-sensitivities by finite differences."""
    def rhs(t, y, a, d):
        return [a - d * y[0]]
    
    # Nominal solution
    sol0 = solve_ivp(rhs, t_span, [0.0], args=(alpha, delta), t_eval=t_eval)
    m_nom = sol0.y[0]
    
    # Perturb alpha
    sol_a = solve_ivp(rhs, t_span, [0.0], args=(alpha*(1+eps), delta), t_eval=t_eval)
    s_alpha = (np.log(sol_a.y[0] + 1e-12) - np.log(m_nom + 1e-12)) / np.log(1 + eps)
    
    # Perturb delta
    sol_d = solve_ivp(rhs, t_span, [0.0], args=(alpha, delta*(1+eps)), t_eval=t_eval)
    s_delta = (np.log(sol_d.y[0] + 1e-12) - np.log(m_nom + 1e-12)) / np.log(1 + eps)
    
    return t_eval, m_nom, s_alpha, s_delta

t, m, s_a, s_d = sensitivity_gene_expression(5.0, 0.5, (0, 20), np.linspace(0, 20, 100))
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 6))
ax1.plot(t, m, 'steelblue'); ax1.set_ylabel('[mRNA]')
ax2.plot(t, s_a, label='∂ln m/∂ln α'); ax2.plot(t, s_d, label='∂ln m/∂ln δ')
ax2.set_ylabel('Log-sensitivity'); ax2.legend()
plt.xlabel('Time'); plt.tight_layout()
```

Notice that the sensitivities are time-dependent during the transient approach to steady state: early in the simulation, sensitivity to $\alpha$ is near 1 (production dominates) and to $\delta$ is near 0 (nothing has degraded yet). At steady state, both reach their equilibrium values of $\pm 1$. This time-dependence of sensitivities is invisible from steady-state analysis alone and matters for understanding how quickly the system responds to parameter perturbations.

**Limitation of local sensitivity**: it is only valid near the nominal parameter values. If the model is highly nonlinear, sensitivities may change dramatically in different regions of parameter space.

## Global Sensitivity Analysis

Global methods explore the full parameter space (typically by sampling) to compute average sensitivities that account for parameter uncertainty and nonlinearity.

### Morris Screening Method

The **Morris method** (also called Elementary Effects method) provides a computationally efficient screen to rank parameters by importance:

1. Sample $r$ trajectories through parameter space; each trajectory changes one parameter at a time by a fixed step $\Delta$.
2. Compute the elementary effect of parameter $j$ for each trajectory: $EE_j = (y(\ldots, \theta_j + \Delta, \ldots) - y(\ldots, \theta_j, \ldots)) / \Delta$.
3. Summary statistics: $\mu^* = $ mean of $|EE_j|$ (overall importance); $\sigma = $ std of $EE_j$ (nonlinearity/interaction).

Parameters with high $\mu^*$ are influential. Parameters with high $\sigma$ have interactions with other parameters (their effect depends on what other parameters are set to).

### Sobol Variance-Based Indices

**Sobol indices** provide the most rigorous global sensitivity analysis by decomposing total output variance into contributions from each parameter and their interactions:

$$V(Y) = \sum_i V_i + \sum_{i<j} V_{ij} + \ldots$$

**First-order index** $S_i = V_i / V(Y)$: fraction of output variance due to parameter $i$ alone.

**Total-order index** $T_i = (V_{\sim i}) / V(Y)$: total contribution of parameter $i$, including all interactions.

$T_i - S_i$ measures interaction effects. Parameters with $T_i \approx S_i$ act independently.

```python
from SALib.sample import sobol as sobol_sample
from SALib.analyze import sobol
import numpy as np

def model_steady_state(params):
    """Output: steady-state mRNA = alpha/delta"""
    results = []
    for row in params:
        alpha, delta, beta, gamma = row
        m_ss = alpha / delta
        p_ss = beta * m_ss / gamma
        results.append(p_ss)
    return np.array(results)

problem = {
    'num_vars': 4,
    'names': ['alpha', 'delta', 'beta', 'gamma'],
    'bounds': [[1, 20], [0.1, 2], [0.5, 5], [0.05, 0.5]]
}

# Generate Saltelli samples (required for Sobol indices)
param_values = sobol_sample.sample(problem, 1024)
Y = model_steady_state(param_values)

# Compute Sobol indices
Si = sobol.analyze(problem, Y, print_to_console=True)
# Si['S1']: first-order indices
# Si['ST']: total-order indices
```

For the gene expression model above, you will find that $\alpha$ and $\delta$ together account for essentially all of the variance in mRNA (since $m^* = \alpha/\delta$), while $\beta$ and $\gamma$ have high Sobol indices for protein but zero index for mRNA. Global sensitivity analysis makes this kind of structural insight quantitative and automatic.

## Practical Recommendations

**Use local sensitivity first**: fast, gives intuition about which parameters matter at the operating point of interest. Essential when preparing for experiments — local sensitivities tell you directly which parameters to measure most precisely.

**Use Morris for screening large parameter spaces**: with $> 20$ parameters, Sobol indices require $> 10^4$ model evaluations. Morris can screen 50 parameters with $\sim 500$ evaluations.

**Use Sobol when accuracy matters**: for final results or regulatory submissions, Sobol indices provide the most rigorous characterization.

**Connect to identifiability**: parameters with zero (or very low) sensitivity are practically non-identifiable — the data contain no information about them. Profile likelihood and sensitivity analysis tell the same story from different directions.

## Why This Matters

Sensitivity analysis transforms a fitted model from a black box into a mechanistic explanation. Knowing that model output is 90% controlled by two parameters — and nearly insensitive to the remaining eight — focuses experimental effort and provides insight into which cellular variables are most important for the biological function being modeled.

In synthetic biology design, sensitivity analysis identifies which circuit parameters need precise tuning (tight sensitivity) and which can be loosely specified (low sensitivity), directly informing the level of precision required in component characterization. A toggle switch with high sensitivity to the Hill coefficient of the repression function tells you that you need a repressor with high cooperativity; a switch with low sensitivity to the degradation rate tells you that you don't need to engineer a precise protease. Sensitivity analysis turns abstract model parameters into actionable design specifications.
