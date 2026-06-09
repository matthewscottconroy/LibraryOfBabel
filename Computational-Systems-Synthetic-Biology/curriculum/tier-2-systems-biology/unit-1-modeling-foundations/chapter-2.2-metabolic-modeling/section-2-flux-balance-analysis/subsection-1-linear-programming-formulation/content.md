# Flux Balance Analysis: Linear Programming Formulation

## The FBA Optimization Problem

Evolution is an optimizer. Over billions of years, natural selection has pressured microorganisms to grow as fast as possible in the environments they encounter — and that pressure is reflected in the metabolic networks they carry. Flux Balance Analysis (FBA) makes this insight into a precise mathematical hypothesis: the cell is assumed to operate at the flux distribution that maximizes its growth rate, subject to stoichiometric and capacity constraints. That hypothesis, when formalized as a linear program, turns out to be remarkably predictive.

**Flux Balance Analysis (FBA)** identifies the flux distribution within the feasible polytope that maximizes (or minimizes) a chosen **objective function**. The complete mathematical formulation is:

$$\underset{\mathbf{v}}{\text{maximize}} \quad \mathbf{c}^\top \mathbf{v}$$

$$\text{subject to:} \quad \mathbf{S} \cdot \mathbf{v} = \mathbf{0}$$

$$\quad \mathbf{v}_\text{min} \leq \mathbf{v} \leq \mathbf{v}_\text{max}$$

This is a **linear program (LP)**: both the objective $\mathbf{c}^\top \mathbf{v}$ and all constraints are linear in the decision variables $\mathbf{v}$.

The vector $\mathbf{c}$ is the **objective coefficient vector**: $c_j = 1$ for the reaction(s) being maximized and $c_j = 0$ for all others. Most commonly:

- **Maximize biomass**: $c_j = 1$ for the biomass reaction, $c_j = 0$ otherwise.
- **Maximize ATP**: $c_j = 1$ for the ATP maintenance (ATPM) reaction.
- **Minimize total flux** (parsimonious FBA): $\mathbf{c} = -\mathbf{1}$ (all reactions); equivalent to Occam's razor.
- **Maximize product secretion**: $c_j = 1$ for the product exchange reaction.

## Why Linear Programming?

LP is the tool of choice because:

1. **Global optimality**: LP solvers find the global optimum — there are no local optima to worry about. This is a major advantage over nonlinear optimization.
2. **Computational efficiency**: LP solvers (simplex method, interior-point methods) scale well to thousands of variables and constraints. Genome-scale models with 2,000–10,000 reactions are solved in milliseconds.
3. **Duality theory**: LP provides **shadow prices** (dual variables) as a byproduct of the solution, revealing which constraints are limiting.

## The Biomass Reaction

The most biologically motivated objective is **growth rate maximization**. This assumes that evolution has selected for maximum growth rate under the given nutritional conditions — a reasonable approximation for exponentially growing cells in rich medium.

The **biomass reaction** encodes the stoichiometric costs of cell duplication:

$$\sum_i c_i^\text{dry mass} \cdot \text{Metabolite}_i \rightarrow \text{Biomass}$$

where $c_i^\text{dry mass}$ are the stoichiometric coefficients derived from measurements of cell composition (amino acid content, lipid fractions, nucleotide pools, cofactor concentrations). The flux through this reaction (in units of g dry weight / g dry weight / hour = h$^{-1}$) is the growth rate $\mu$.

FBA then asks: given the available nutrients, what is the maximum $\mu$ achievable while satisfying all mass balance and capacity constraints? It is the same question a metabolic engineer asks when trying to maximize yield, or an evolutionary biologist asks when predicting which phenotype selection will favor.

## Shadow Prices and Dual Variables

Every LP has a **dual problem**. The dual variables $\boldsymbol{\lambda}$ (shadow prices) associated with the steady-state constraints $\mathbf{S} \cdot \mathbf{v} = \mathbf{0}$ represent the marginal value of each metabolite:

$$\lambda_i = \frac{\partial (\text{objective})}{\partial b_i}$$

where $b_i$ would be the right-hand side of the balance equation (if it were not zero). $\lambda_i$ answers: by how much does the optimal growth rate increase if metabolite $i$ is made available from an external source at infinitesimal rate?

Metabolites with large positive shadow prices are **limiting** — supplying them externally would significantly improve growth. Shadow prices are widely used to identify:
- Which nutrients are limiting in the current medium
- Which internal intermediates are bottlenecks
- Which reactions are most valuable to derepress

## Worked Example: Glucose vs. Glycerol

```python
import cobra
from cobra.io import read_sbml_model
import pandas as pd

model = read_sbml_model('iJO1366.xml')

# Define media conditions
media_conditions = {
    'Glucose aerobic': {'EX_glc__D_e': -10, 'EX_o2_e': -20},
    'Glycerol aerobic': {'EX_glyc_e': -10, 'EX_o2_e': -20},
    'Glucose anaerobic': {'EX_glc__D_e': -10, 'EX_o2_e': 0},
}

results = []
for condition, bounds in media_conditions.items():
    with model:
        for rxn_id, lb in bounds.items():
            model.reactions.get_by_id(rxn_id).lower_bound = lb
        sol = model.optimize()
        results.append({
            'Condition': condition,
            'Growth rate (h⁻¹)': sol.objective_value,
            'Status': sol.status
        })

df = pd.DataFrame(results)
print(df.to_string(index=False))

# Typical results:
# Glucose aerobic:   ~0.873 h⁻¹
# Glycerol aerobic:  ~0.641 h⁻¹  (glycerol is a less efficient carbon source)
# Glucose anaerobic: ~0.211 h⁻¹  (fermentation only)
```

## Reduced Costs

The **reduced cost** of a reaction $j$ is:

$$\bar{c}_j = c_j - \boldsymbol{\lambda}^\top \mathbf{S}_j$$

where $\mathbf{S}_j$ is the $j$-th column of $\mathbf{S}$. If $\bar{c}_j < 0$ (for a maximization problem): reaction $j$ carries zero flux at optimum, and increasing its lower bound would *decrease* the objective. If $\bar{c}_j = 0$: the reaction is at its bound and is part of the optimal solution.

Reduced costs help identify reactions that are not used at optimum and why — they have unfavorable cost-benefit ratios from the perspective of the objective.

## Limits of the LP Framework

FBA optimizes a linear objective, but real metabolism is not a linear optimizer:

- Cells do not always maximize growth (especially under stress, stationary phase, or cooperative conditions)
- LP gives a unique objective value but not necessarily a unique flux distribution — multiple optima may exist (Section 2.2.2.4 on FVA)
- The biomass composition is measured as a population average; individual cells may differ

Despite these limitations, FBA accurately predicts growth rates and gene essentiality for many organisms under many conditions — a remarkable achievement for a model with no kinetic parameters.

## Why This Matters

The linear programming formulation of FBA is one of the most successful examples of mathematical optimization applied to biology. It transforms the question "what does this cell do?" into a precisely posed optimization problem with an unambiguous, globally optimal answer. The quality of the prediction depends on the quality of the stoichiometric matrix and the validity of the growth-maximization assumption — both of which can be tested. For metabolic engineering, FBA provides a computational platform to identify gene deletion and overexpression strategies for maximizing desired product yields before any experiment is performed.
