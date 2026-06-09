# Flux Variability Analysis

## The Problem of Alternative Optima

Suppose FBA tells you that the optimal growth rate is 0.87 h⁻¹ and the phosphofructokinase flux is 7.2 mmol/gDW/h. You measure PFK flux in your experiment and get 6.5. Is the model wrong? Not necessarily — and understanding why requires confronting one of the most important subtleties in FBA: the optimal flux distribution is rarely unique.

Standard FBA identifies **one** flux distribution that achieves the optimal objective value. However, for large metabolic networks, there are typically many flux distributions that achieve the same maximum growth rate — the optimal solution is not unique. This degeneracy arises because metabolic networks contain redundant pathways and reversible reactions that can be combined in multiple ways to achieve the same net flux balance.

**Flux Variability Analysis (FVA)** systematically explores this degeneracy by computing, for each reaction $j$, the minimum and maximum flux that is compatible with a given (usually optimal) objective value:

$$v_j^\text{min} = \min v_j \quad \text{subject to:} \quad \mathbf{S} \cdot \mathbf{v} = \mathbf{0},\; \mathbf{v}_\text{min} \leq \mathbf{v} \leq \mathbf{v}_\text{max},\; \mathbf{c}^\top \mathbf{v} \geq f \cdot \mu^*$$

$$v_j^\text{max} = \max v_j \quad \text{subject to the same constraints}$$

where $f \in [0, 1]$ is the **fraction of optimum** parameter (typically 0.9–1.0 — requiring the flux distribution to achieve at least $f \times$ the maximum growth rate).

## Implementation

```python
import cobra
from cobra.flux_analysis import flux_variability_analysis
from cobra.io import read_sbml_model

model = read_sbml_model('iJO1366.xml')

# FVA at 100% optimal growth
fva_100 = flux_variability_analysis(model, fraction_of_optimum=1.0)

# FVA at 90% optimal growth (allows 10% suboptimality)
fva_90 = flux_variability_analysis(model, fraction_of_optimum=0.9)

print("FVA results (first 10 reactions):")
print(fva_100.head(10))

# Reactions with zero range: fixed flux (essential or perfectly coupled)
fixed_reactions = fva_100[fva_100['maximum'] - fva_100['minimum'] < 1e-6]
print(f"\nFixed reactions (no flexibility): {len(fixed_reactions)}")

# Reactions with nonzero range: metabolically flexible
flexible = fva_100[(fva_100['maximum'] - fva_100['minimum']) > 1e-3]
print(f"Flexible reactions: {len(flexible)}")

# Reactions that are never used (max = min = 0)
blocked = fva_100[(fva_100['maximum'] < 1e-10) & (fva_100['minimum'] > -1e-10)]
print(f"Blocked reactions: {len(blocked)}")
```

## Interpreting FVA Results

**Fixed reactions** (range ≈ 0): these carry the same flux in every optimal solution. They are unambiguously part of the optimal metabolic state. Fixed reactions with nonzero flux are **essential** for growth — knocking them out would reduce growth rate. Fixed reactions at zero are **blocked** — stoichiometrically impossible to carry flux given the current constraints.

**Flexible reactions** (large range): these can carry anywhere from minimum to maximum flux while maintaining optimal growth. This indicates metabolic redundancy — multiple pathways can substitute for each other without affecting the objective. When you see a large range in an FVA result, it is telling you something deep about the biology: stoichiometry alone cannot determine this flux. You need kinetics, regulation, or direct measurement to pin it down.

**Blocked reactions** (both min and max ≈ 0 regardless of $f$): cannot carry any flux at optimal growth. These may represent:
- Missing gene annotations (enzyme present but not captured in GPR)
- Incomplete network topology (substrate not provided in current medium)
- Thermodynamically infeasible fluxes given constraints

## Flux Coupling Analysis

FVA reveals **flux coupling** between reactions: if reactions $i$ and $j$ always carry proportional fluxes across all optimal solutions, they are coupled. Types of coupling:

**Fully coupled** ($v_i = c \cdot v_j$ always): the two reactions always carry fixed-ratio fluxes. This occurs when they are in the same linear pathway with no branch points.

**Directionally coupled**: if $v_i > 0$ then $v_j > 0$, but the ratio can vary. Partial coupling.

**Uncoupled**: fluxes are completely independent.

Flux coupling analysis (as implemented in CoPE-FBA) uses FVA systematically to determine coupling classes and is valuable for:
- Identifying gene pairs that must be co-expressed for growth
- Reducing model complexity by merging fully coupled reactions
- Understanding metabolic architecture

## Application to Metabolic Engineering

FVA is particularly valuable for **identifying strain design targets**:

```python
import pandas as pd
from cobra.flux_analysis import flux_variability_analysis

# Find reactions where ethanol production is compatible with high growth
with model:
    # Set minimum ethanol production target
    model.reactions.get_by_id('EX_etoh_e').lower_bound = 5.0  # mmol/gDW/h

    # Check maximum achievable growth under this constraint
    sol = model.optimize()
    max_growth_with_etoh = sol.objective_value
    print(f"Max growth with ethanol production ≥ 5: {max_growth_with_etoh:.4f} h⁻¹")

    if sol.status == 'optimal':
        # FVA to find all reactions compatible with this constraint
        fva_etoh = flux_variability_analysis(
            model, fraction_of_optimum=0.95
        )
        # Reactions with zero range are obligately active or obligately zero
        # These are targets for overexpression or knockout
        obligate_active = fva_etoh[
            (fva_etoh['minimum'] > 0.01) & 
            (fva_etoh['maximum'] - fva_etoh['minimum'] < 0.01)
        ]
        print(f"Obligately active reactions in ethanol-producing condition: {len(obligate_active)}")
        print(obligate_active.head(10))
```

## Yield Analysis with FVA

FVA can compute the theoretical yield range for any product:

```python
from cobra.flux_analysis import production_envelope

# Compute the production envelope for succinate vs. growth
prod_envelope = production_envelope(
    model,
    reactions=['EX_succ_e'],  # product secretion
    objective='biomass_reaction',
    carbon_sources=['EX_glc__D_e']
)

# Plot: growth rate on x-axis, succinate flux on y-axis
# Shows the trade-off between growth rate and product formation
```

The **production envelope** (or phenotype phase plane) shows the achievable combinations of growth rate and product yield — it defines the fundamental trade-off between cell growth and product formation inherent in the stoichiometric network. Before building any strain, this is the map you draw first: the production envelope tells you whether your target yield is stoichiometrically achievable at all.

## Why This Matters

FVA transforms FBA from a single-point predictor into a characterization of the entire optimal space. The range of flexibility revealed by FVA has direct experimental consequences: a reaction with a wide range means that measuring its flux experimentally could yield many different values without contradicting the model — a warning that stoichiometry alone does not predict this flux. A reaction with zero range means that any two experiments at the same growth rate should give the same flux — a strong, testable prediction. For metabolic engineering, the FVA production envelope is the computational blueprint that defines what yields are physically possible before any strain is constructed.
