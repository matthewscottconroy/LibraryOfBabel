# Constrained Optimization and Linear Programming

Suppose you are trying to predict how fast *E. coli* will grow on a given carbon source. You know the full set of metabolic reactions — glycolysis, the TCA cycle, the electron transport chain, biosynthesis of all the building blocks. You know the stoichiometry: how many molecules of ATP are produced per glucose consumed, how many NADH molecules feed into oxidative phosphorylation. What you want to know is: given all these reactions and all their constraints, what combination of fluxes maximizes the rate of biomass production?

This is not an unconstrained optimization problem. Metabolic fluxes must be non-negative (reactions run in one direction). They must satisfy stoichiometric balance — for every intermediate metabolite, production must equal consumption at steady state. And they must lie within measured bounds on enzyme capacity. The answer to your question is the solution to a linear program: an optimization problem with a linear objective and linear constraints. It can be solved exactly, in seconds, for models with thousands of reactions.

Biological optimization problems are rarely unconstrained. Metabolic fluxes must be non-negative, satisfy stoichiometric balance at steady state, and lie within measured capacity limits. Protein designs must satisfy thermodynamic stability constraints. Clinical dosing must respect toxicity thresholds. **Constrained optimization** — and its most computationally mature special case, **linear programming (LP)** — provides the mathematical framework for these problems.

## Linear Programming Fundamentals

A **linear program (LP)** minimizes a linear objective subject to linear equality and inequality constraints:

$$\text{minimize} \quad \mathbf{c}^T \mathbf{x}$$
$$\text{subject to} \quad A_{\text{eq}} \mathbf{x} = \mathbf{b}_{\text{eq}}$$
$$\quad\quad\quad\quad\quad\quad A_{\text{ub}} \mathbf{x} \leq \mathbf{b}_{\text{ub}}$$
$$\quad\quad\quad\quad\quad\quad \ell \leq \mathbf{x} \leq u$$

The feasible set is a **convex polytope** (intersection of half-spaces). The optimal solution lies at a **vertex** of this polytope (unless the problem is unbounded or infeasible). The **Simplex algorithm** moves along edges of the polytope from vertex to vertex, always decreasing the objective. **Interior point methods** traverse the interior of the polytope and are more efficient for large LPs.

## Flux Balance Analysis as a Linear Program

**Flux Balance Analysis (FBA)** is the most important application of LP in biology. Given a genome-scale metabolic model with $m$ metabolites and $n$ reactions, the **stoichiometric matrix** $S \in \mathbb{R}^{m \times n}$ encodes how each reaction changes metabolite concentrations.

At steady state, production equals consumption for each metabolite:

$$S \mathbf{v} = \mathbf{0}$$

Subject to flux bounds $\mathbf{v}_{\min} \leq \mathbf{v} \leq \mathbf{v}_{\max}$ (thermodynamic and measured constraints), we maximize a biological objective — typically biomass production or ATP yield:

$$\text{maximize} \quad \mathbf{c}^T \mathbf{v}$$
$$\text{subject to} \quad S\mathbf{v} = \mathbf{0}, \quad \mathbf{v}_{\min} \leq \mathbf{v} \leq \mathbf{v}_{\max}$$

```python
import numpy as np
from scipy.optimize import linprog

# Minimal 3-reaction FBA example
# Reactions: R1: A -> B (uptake), R2: B -> C (conversion), R3: B -> (waste), R4: C -> biomass
# Metabolites: A, B, C

# Stoichiometric matrix: S[metabolite, reaction]
# Reactions: v1 (A->B), v2 (B->C), v3 (B->waste), v4 (C->biomass)
S = np.array([
    [-1,  0,  0,  0],   # A: consumed by v1
    [ 1, -1, -1,  0],   # B: produced by v1, consumed by v2, v3
    [ 0,  1,  0, -1],   # C: produced by v2, consumed by v4
])

m, n = S.shape
print(f"Stoichiometric matrix: {m} metabolites x {n} reactions")

# Objective: maximize biomass flux v4 (linprog minimizes, so negate)
c = np.array([0, 0, 0, -1])  # -v4 (biomass)

# Equality constraints: Sv = 0 (steady state)
A_eq = S
b_eq = np.zeros(m)

# Flux bounds
v_min = np.array([0, 0, 0, 0])    # all irreversible (non-negative)
v_max = np.array([10, 100, 100, 100])  # substrate uptake limited to 10

bounds = list(zip(v_min, v_max))

result = linprog(
    c=c,
    A_eq=A_eq,
    b_eq=b_eq,
    bounds=bounds,
    method='highs'   # HiGHS: modern, fast LP solver
)

print(f"Optimal biomass flux: {-result.fun:.4f}")
print(f"Flux distribution: v = {result.x}")
print(f"Status: {result.message}")
```

## COBRApy for Genome-Scale FBA

For realistic genome-scale metabolic models (GEMs) with thousands of reactions:

```python
import cobra
from cobra.io import read_sbml_model

# Load E. coli core model (iJO1366: 2251 reactions, 1136 metabolites)
model = read_sbml_model('e_coli_core.xml')

# Inspect model
print(f"Reactions: {len(model.reactions)}")
print(f"Metabolites: {len(model.metabolites)}")
print(f"Genes: {len(model.genes)}")

# Default objective: maximize biomass
with model:
    solution = model.optimize()
    print(f"\nOptimal growth rate: {solution.objective_value:.4f} h^-1")
    print(f"Glucose uptake: {solution.fluxes['EX_glc__D_e']:.4f} mmol/gDW/h")
    print(f"Oxygen uptake: {solution.fluxes['EX_o2_e']:.4f} mmol/gDW/h")

# Gene knockout: disable phosphoglucose isomerase (pgi)
with model:
    model.genes.b4025.knock_out()
    ko_solution = model.optimize()
    print(f"\nΔpgi growth rate: {ko_solution.objective_value:.4f} h^-1")

# Flux Variability Analysis: find the range of each flux at optimal growth
from cobra.flux_analysis import flux_variability_analysis
fva_result = flux_variability_analysis(
    model, 
    fraction_of_optimum=0.9,  # allow 10% suboptimal growth
    processes=4
)
print(f"\nFlux ranges (first 5 reactions):")
print(fva_result.head())
```

## Quadratic and Nonlinear Programming

When the objective or constraints are quadratic or nonlinear, more general solvers are needed:

**Quadratic programming (QP):** objective is quadratic, constraints linear. Solvers: `scipy.optimize.minimize(method='SLSQP')`, `quadprog`, `osqp`.

**Nonlinear programming (NLP):** general nonlinear objective and constraints. Solver: IPOPT (Interior Point OPTimizer, via `cyipopt` Python wrapper) or `scipy.optimize.minimize(method='SLSQP')`.

```python
from scipy.optimize import minimize

# Constrained parameter optimization: fit rates subject to thermodynamic constraints
# Constraint: detailed balance (K_eq product of rate ratios = 1 for each cycle)

def objective(params):
    # ODE fit objective
    return sum_of_squares(params, data)

def detailed_balance_constraint(params):
    """Enforce thermodynamic consistency: product of equilibrium constants around each cycle = 1."""
    k1f, k1r, k2f, k2r = params[:4]
    K_eq_cycle = (k1f * k2f) / (k1r * k2r)
    return K_eq_cycle - 1.0  # must equal zero

constraints = [
    {'type': 'eq', 'fun': detailed_balance_constraint},
    {'type': 'ineq', 'fun': lambda p: p}  # all params >= 0
]

bounds = [(0, None)] * 8

result = minimize(objective, x0, method='SLSQP',
                 constraints=constraints, bounds=bounds,
                 options={'maxiter': 500, 'ftol': 1e-10})
```

## The Dual Problem and Shadow Prices

Every LP has a **dual problem** whose solution provides **shadow prices** (dual variables) — the sensitivity of the optimal objective to changes in constraints. In FBA, the shadow price of the $i$-th metabolite is:

$$\lambda_i = \frac{\partial (\text{optimal objective})}{\partial b_i}$$

where $b_i$ is the right-hand side of the steady-state constraint for metabolite $i$. A positive shadow price means adding more of this metabolite to the system would increase growth — it is a limiting resource.

```python
with model:
    solution = model.optimize()
    # Shadow prices: metabolite-level growth sensitivity
    shadow_prices = solution.shadow_prices
    # Identify the top limiting metabolites
    top_limiting = shadow_prices.abs().sort_values(ascending=False).head(10)
    print("Top growth-limiting metabolites (by shadow price):")
    print(top_limiting)
```

## Why This Matters

Linear programming is the engine of constraint-based metabolic modeling. The entire field of genome-scale metabolic modeling — which has been used to design metabolic engineering strategies, predict essential genes, and understand drug-nutrient interactions — rests on LP. Understanding both the mathematics (simplex method, LP duality) and the practical tools (scipy.optimize.linprog, COBRApy, commercial solvers) gives you direct access to one of the most powerful quantitative frameworks in modern biology.
