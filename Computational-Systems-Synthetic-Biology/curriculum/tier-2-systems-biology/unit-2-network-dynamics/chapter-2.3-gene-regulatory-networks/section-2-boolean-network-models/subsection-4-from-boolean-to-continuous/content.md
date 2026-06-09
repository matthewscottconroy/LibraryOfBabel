# From Boolean to Continuous: Bridging Formalisms

## The Translation Problem

Here is the practical bind you find yourself in when doing computational biology. Boolean models are easy to build — you need only the topology of regulatory interactions, which can often be read directly from the experimental literature. But your collaborators hand you RNA-seq time courses, single-cell expression matrices, and quantitative reporter assays. How do you compare a model that lives in a world of 0s and 1s to data that lives in a world of continuous measurements?

Boolean network models provide qualitative predictions about attractors and transitions but discard all quantitative information about expression levels, timing, and gradedness. Continuous ODE models provide this quantitative detail but require parameters that are often unavailable. A family of methods bridges these formalisms, using Boolean models as structural scaffolding to generate continuous dynamical systems.

## Quasi-Boolean (Sigmoidal) Relaxation

The simplest approach: replace each Boolean variable $g_i \in \{0,1\}$ with a continuous variable $x_i \in [0,1]$ and each Boolean function $F_i$ with its continuous relaxation using sigmoidal functions.

For a Boolean rule $g_3 = g_1 \text{ AND NOT } g_2$, the continuous relaxation is:

$$x_3^* = \sigma^+(x_1) \cdot \sigma^-(x_2)$$

where $\sigma^+(x) = x^n/(h^n + x^n)$ and $\sigma^-(x) = h^n/(h^n + x^n)$ are sigmoidal functions, and $h$ is the threshold (typically 0.5) and $n$ is the steepness parameter (high $n$ → more Boolean-like).

The full ODE system is:

$$\frac{dx_i}{dt} = \frac{1}{\tau_i}\left(F_i^{\text{cont}}(\mathbf{x}) - x_i\right)$$

where $\tau_i$ is a relaxation timescale (equal for all genes in the simplest version) and $F_i^{\text{cont}}$ is the continuous relaxation of the Boolean update function.

**Key property**: as $n \to \infty$ (steepness increases), the continuous system has the same attractor structure as the Boolean system. For finite $n$, the attractors are "smeared out" into continuous regions, allowing visualization as two-dimensional projections.

This is a beautiful result. It tells you that the Boolean model is not a coarse approximation to something else — it is the limiting case of a whole family of continuous models, all of which share the same qualitative attractor structure. As you decrease the steepness $n$, you gradually relax from the hard Boolean world into the soft continuous one, while the attractors remain where they are.

## SQUAD: Standardized Qualitative Dynamical Systems

**SQUAD** (di Cara et al. 2007) implements this quasi-Boolean approach with a standardized normalization:

1. Parse Boolean rules into a signed interaction matrix
2. Construct a sigmoidal ODE for each gene
3. Integrate from multiple initial conditions
4. Report attractors and basin boundaries

SQUAD is particularly useful for:
- Generating continuous trajectories compatible with microarray or RNA-seq data
- Predicting perturbation responses in qualitative units (fold change direction)
- Visualizing the attractor landscape in low-dimensional projections

```python
# Quasi-Boolean ODE implementation
import numpy as np
from scipy.integrate import solve_ivp

def sigmoid_pos(x, threshold=0.5, n=5):
    """Sigmoidal activation function."""
    return x**n / (threshold**n + x**n)

def sigmoid_neg(x, threshold=0.5, n=5):
    """Sigmoidal repression function."""
    return threshold**n / (threshold**n + x**n)

def quasi_boolean_odes(t, state, interactions, tau=1.0):
    """
    interactions: dict mapping gene index to continuous rule function
    """
    n = len(state)
    dydt = np.zeros(n)
    for i in range(n):
        target = interactions[i](state)  # continuous rule evaluation
        dydt[i] = (target - state[i]) / tau
    return dydt

# Define toggle switch: A = NOT B, B = NOT A
interactions = {
    0: lambda s: sigmoid_neg(s[1]),  # A = NOT B
    1: lambda s: sigmoid_neg(s[0])   # B = NOT A
}

# Integrate from two different initial conditions
for x0 in [[0.9, 0.1], [0.1, 0.9]]:
    sol = solve_ivp(quasi_boolean_odes, [0, 20], x0, 
                    args=(interactions,), dense_output=True)
    print(f"Starting at {x0}, final state: {sol.y[:, -1].round(2)}")
# Output: two different stable states — bistability confirmed
```

## Piecewise-Linear Differential Equations

A mathematically more tractable approach uses **piecewise-linear (PL) differential equations** (Glass & Kauffman 1973; de Jong et al. 2004):

$$\frac{dx_i}{dt} = \sum_j \kappa_{ij} b_j(\mathbf{x}) - \gamma_i x_i$$

where $b_j(\mathbf{x}) \in \{0,1\}$ are step functions (Boolean-like switching functions of concentrations) and $\kappa_{ij}, \gamma_i$ are kinetic parameters. The step functions partition state space into rectangular regions (boxes) within each of which the system is linear. Trajectories can be computed analytically within boxes and matched at boundaries.

PL systems have the same attractor topology as Boolean networks but allow:
- Exact computation of trajectories (no numerical integration needed within boxes)
- Formal proofs of attractor existence and stability
- Natural inclusion of regulatory thresholds as parameters

The software **GNA (Genetic Network Analyzer)** implements PL differential equation analysis.

## Fitting Boolean-Derived ODEs to Data

Once a Boolean-derived continuous model is constructed, its parameters ($n$, $\tau_i$, thresholds) can be fit to experimental data:

```python
from scipy.optimize import minimize

def model_error(params, observed_trajectories, interactions):
    """Compute sum of squared residuals for parameter fitting."""
    tau, n = params
    total_error = 0
    for x0, observed in observed_trajectories:
        sol = solve_ivp(quasi_boolean_odes, [0, 100], x0,
                       args=(interactions, tau), dense_output=True)
        predicted = sol.sol(observed['timepoints'])
        total_error += np.sum((predicted - observed['values'])**2)
    return total_error

# Optimize parameters given time-course data
result = minimize(model_error, x0=[1.0, 3.0], 
                  args=(data, interactions),
                  method='Nelder-Mead')
```

## Comparing Attractor Structures

A crucial validation step is confirming that the continuous model preserves the Boolean model's attractors:

- **Fixed-point correspondence**: every fixed point of the Boolean model should correspond to a stable fixed point of the continuous model (for large enough $n$)
- **Limit cycle correspondence**: Boolean limit cycles should appear as limit cycles in the continuous system
- **Basin topology**: the relative sizes of attraction basins should be preserved qualitatively

Mismatches indicate that the continuous relaxation is not faithful to the Boolean topology, which may require adjusting the sigmoidal form or threshold values.

## Why This Matters

The Boolean-to-continuous translation solves a practical bottleneck in systems biology: Boolean models are easy to build from qualitative regulatory knowledge, but continuous models are needed for quantitative comparison with gene expression data. Quasi-Boolean ODE systems provide a middle path — they make the attractor structure of Boolean models accessible to continuous data analysis and allow parameter fitting without requiring detailed kinetic measurements. This bridge is increasingly used in stem cell and cancer biology to interpret single-cell RNA-seq data in terms of attractor landscape models.
