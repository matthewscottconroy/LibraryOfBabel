# Max-Min Driving Force Optimization

## The Problem: Jointly Feasible Thermodynamics

Checking whether each reaction in a pathway is thermodynamically favorable seems straightforward — compute $\Delta_r G'$ for each step using eQuilibrator. But there is a subtlety that makes this reaction-by-reaction approach insufficient: the metabolite concentrations are shared. Making reaction A highly favorable by keeping its substrate at high concentration simultaneously affects reaction B, which might consume that same substrate. You cannot optimize each reaction independently; you need a consistent set of concentrations that makes every reaction in the pathway thermodynamically feasible at once. That is what Max-Min Driving Force analysis provides.

**Max-Min Driving Force (MDF)** (Noor et al. 2014) is a linear programming method that finds the metabolite concentrations maximizing the minimum thermodynamic driving force across all reactions in a pathway, subject to realistic concentration bounds.

## Mathematical Formulation

Given a set of $n$ reactions with stoichiometric matrix $\mathbf{S}$ and a flux direction vector $\mathbf{v}$ (known from FBA or measured ¹³C MFA), define:

$$\Delta_r G'_j = \Delta_r G'^\circ_j + RT \sum_i S_{ij} \ln[x_i]$$

where $[x_i]$ is the concentration of metabolite $i$.

The MDF optimization problem maximizes the worst-case driving force $B$ (the "bottleneck"):

$$\text{maximize} \quad B$$
$$\text{subject to:} \quad \text{sign}(v_j) \cdot \Delta_r G'_j \leq -B \quad \forall j$$
$$\quad \ln x_i^{\min} \leq \ln x_i \leq \ln x_i^{\max} \quad \forall i$$

Here the sign of $v_j$ ensures we require $\Delta_r G'_j < 0$ in the direction of net flux. The decision variables are $\ln x_i$ (log concentrations), making the problem linear in $\ln[\text{concentrations}]$.

The **MDF value** $B^*$ (in kJ/mol) is the maximum achievable minimum driving force:
- $B^* > 0$: pathway is thermodynamically feasible; there exists a concentration assignment consistent with all flux directions
- $B^* \leq 0$: pathway is thermodynamically infeasible regardless of concentrations — the pathway cannot carry flux in the specified direction

## Setting Concentration Bounds

Physiologically realistic bounds constrain the optimization. For most cytoplasmic metabolites:

$$10^{-6} \, \text{M} \leq [x_i] \leq 10^{-2} \, \text{M}$$

Tighter bounds can be imposed for specific metabolites based on experimental data:
- ATP: 1–10 mM (energetic currency must remain high)
- NADH/NAD⁺ ratio: fixed at physiological value (~0.001 in cytoplasm)
- CO₂: fixed based on bicarbonate buffering

Fixed concentrations (set $x_i^{\min} = x_i^{\max}$) effectively remove degrees of freedom and can tighten the MDF constraint.

## Worked Example: Glycolysis vs. Gluconeogenesis

Consider the upper glycolysis segment: glucose → glucose-6-phosphate → fructose-6-phosphate → fructose-1,6-bisphosphate.

For the glycolytic direction:
- PGI ($G6P \to F6P$): $\Delta_r G'^\circ \approx +1.7$ kJ/mol — needs concentration driving force
- PFK ($F6P + ATP \to FBP + ADP$): $\Delta_r G'^\circ \approx -14.2$ kJ/mol — highly favorable

Running MDF on this segment with standard concentration bounds returns $B^* \approx 3.5$ kJ/mol, achieved at $[G6P]/[F6P] \approx 6$. This concentration ratio drives PGI forward even though it is endergonic at standard conditions — the MDF tells us precisely how much concentration "gradient" is needed.

For gluconeogenesis (running PGI in reverse), MDF would instead optimize $[F6P]/[G6P] > 1$, shifting the constraint.

## Implementation

```python
import numpy as np
from scipy.optimize import linprog

def compute_mdf(drg0, S, v, x_min, x_max, RT=2.479):
    """
    drg0: array of standard ΔrG'° values (kJ/mol), length n_rxns
    S: stoichiometric matrix (n_mets x n_rxns), only pathway metabolites
    v: flux direction (+1 or -1) for each reaction
    x_min, x_max: log concentration bounds for each metabolite
    RT: R*T in kJ/mol (default: 25°C)
    Returns: MDF value and optimal log concentrations
    """
    n_rxns, n_mets = len(drg0), S.shape[0]
    # Variables: [ln(x_1), ..., ln(x_n), B]
    # Minimize -B (i.e., maximize B)
    c = np.zeros(n_mets + 1)
    c[-1] = -1  # maximize B
    
    # Constraint: v_j * (drg0_j + RT * S[:,j]^T * ln(x)) <= -B
    # => v_j * RT * S[:,j]^T * ln(x) + B <= -v_j * drg0_j
    A_ub = np.zeros((n_rxns, n_mets + 1))
    for j in range(n_rxns):
        A_ub[j, :n_mets] = v[j] * RT * S[:, j]
        A_ub[j, -1] = 1  # +B term
    b_ub = -v * drg0
    
    bounds = [(x_min[i], x_max[i]) for i in range(n_mets)] + [(None, None)]
    result = linprog(c, A_ub=A_ub, b_ub=b_ub, bounds=bounds, method='highs')
    
    mdf = -result.fun
    ln_conc = result.x[:n_mets]
    return mdf, np.exp(ln_conc)
```

## Identifying the Thermodynamic Bottleneck

The reaction with the smallest driving force at the MDF optimum is the **thermodynamic bottleneck**. This reaction limits the overall pathway's thermodynamic efficiency — it operates with the minimum possible $|\Delta_r G'|$. Engineers targeting improved pathway performance should focus on:

1. Relieving concentration constraints on bottleneck metabolites (e.g., via transporter engineering)
2. Replacing the bottleneck enzyme with one that uses a more favorable co-substrate
3. Coupling the bottleneck step to an exergonic driving reaction

## Connection to Pathway Design

MDF analysis is especially powerful for **novel biosynthetic pathway design**. When comparing multiple routes from a precursor to a target compound, MDF ranks pathways by thermodynamic viability. A pathway with higher MDF value requires less metabolite concentration adjustment to operate — it is more thermodynamically robust.

The `pytfa` package integrates MDF analysis directly into COBRA-based FBA workflows, enabling thermodynamic FBA at genome scale.

## Why This Matters

MDF moves thermodynamic analysis from a check performed on individual reactions to a global consistency constraint on entire pathways. Experimentally, MDF predictions can be validated by measuring intracellular metabolite concentrations — if measured concentrations are close to the MDF-optimal values, it suggests that thermodynamic driving force is indeed a binding constraint on the cell's metabolism, not just a loose bound.
