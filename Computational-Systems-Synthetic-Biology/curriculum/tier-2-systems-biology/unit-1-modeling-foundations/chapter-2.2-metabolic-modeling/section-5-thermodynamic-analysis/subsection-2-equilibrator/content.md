# eQuilibrator: Thermodynamic Data for Metabolic Reactions

## The Challenge of Biochemical Thermodynamics

Suppose you want to know whether your newly designed biosynthetic pathway is thermodynamically feasible. You need $\Delta_r G'^\circ$ for each step. For a textbook reaction like ATP hydrolysis, the value is in every biochemistry textbook. But your pathway probably includes obscure intermediates — methylmalonyl-CoA, acetoacetyl-ACP, 2-oxo-4-methylpentanoate — for which you would need to search through scattered, inconsistent literature and often find nothing. **eQuilibrator** was built to solve exactly this problem, and it does so elegantly.

Computing $\Delta_r G'$ for a metabolic reaction requires knowing $\Delta_r G'^\circ$ — the standard transformed free energy change under biochemical standard conditions (pH 7, ionic strength 0.25 M, 25°C). Historically, this required finding experimentally measured values in scattered literature, often unavailable for most metabolites. **eQuilibrator** (Flamholz et al. 2012; Beber et al. 2022) solves this problem by providing a unified, programmatically accessible database of biochemical thermodynamic data covering thousands of metabolites.

## The Component Contribution Method

eQuilibrator uses the **component contribution method** (Noor et al. 2013) to estimate $\Delta_f G'^\circ$ (standard formation free energies) for metabolites. The method decomposes each metabolite into structural components (functional groups, molecular fragments) and trains a regression model on all available experimental data simultaneously. This allows:

- **Interpolation**: estimate $\Delta_r G'^\circ$ for reactions involving metabolites with no direct experimental data
- **Uncertainty quantification**: each estimate comes with a standard error
- **Coverage**: over 12,000 compounds from MetaNetX/KEGG, far exceeding what literature mining alone could provide

The pH- and ionic-strength correction uses the Legendre transform to convert from standard chemical conditions to biochemical standard conditions. Magnesium binding is handled explicitly for phosphorylated metabolites (ATP, ADP, GDP, etc.) because Mg²⁺ chelation substantially alters the effective free energy of phosphate groups under physiological conditions.

## Using eQuilibrator: Web Interface

The web interface at `equilibrator.weizmann.ac.il` accepts reaction strings and returns:

- $\Delta_r G'^\circ$: standard transformed free energy change
- $\Delta_r G'$: physiological free energy change at specified concentrations
- Uncertainty estimate (95% confidence interval)
- Links to metabolite structures and source data

**Example workflow** — evaluating the ATP synthase reaction:

```
ADP + Pi → ATP + H2O
```

Enter the reaction string, set pH = 7.4, ionic strength = 0.15 M, pMg = 3.0 (as in typical cytoplasm). The tool returns $\Delta_r G'^\circ \approx +36 \, \text{kJ/mol}$ (endergonic under standard conditions), consistent with the well-known value. Under physiological conditions with [ATP]/[ADP][Pi] reflecting typical cellular ratios, the reverse reaction (ATP hydrolysis) is strongly exergonic ($\approx -54 \, \text{kJ/mol}$).

## The equilibrator-api Python Package

For programmatic use — especially in computational workflows — the `equilibrator-api` package provides the full component contribution database:

```python
from equilibrator_api import ComponentContribution, Q_
import numpy as np

cc = ComponentContribution()

# Phosphoglucose isomerase: G6P → F6P
rxn = cc.parse_reaction_formula(
    "cpd:C00092 = cpd:C00085"  # KEGG IDs for G6P and F6P
)

# Standard ΔrG'° with uncertainty
mu, sigma = cc.standard_dg_prime(rxn)
print(f"ΔrG'° = {mu:.1f} ± {2*sigma:.1f} kJ/mol")

# Physiological ΔrG' given metabolite concentrations
# Set concentrations in Molar using pint quantities
conc = {
    "cpd:C00092": Q_("0.4 mM"),   # G6P
    "cpd:C00085": Q_("0.06 mM"),  # F6P
}
mu_prime, sigma_prime = cc.physiological_dg_prime(rxn, conc)
print(f"ΔrG' = {mu_prime:.1f} ± {2*sigma_prime:.1f} kJ/mol")
```

The uncertainty estimate is crucial: many biochemical values carry substantial uncertainty (±5–20 kJ/mol), and this uncertainty propagates into thermodynamic feasibility conclusions. Reactions near the thermodynamic boundary ($\Delta_r G' \approx 0$) should be flagged as uncertain — a reaction that looks slightly favorable at $-2 \pm 5$ kJ/mol might be infeasible under realistic conditions.

## Batch Analysis of Entire Pathways

A powerful application is computing $\Delta_r G'$ for all reactions in a pathway simultaneously:

```python
import pandas as pd

pathway_reactions = {
    "PGI":  "cpd:C00092 = cpd:C00085",           # G6P → F6P
    "PFK":  "cpd:C00085 + cpd:C00002 = cpd:C05345 + cpd:C00008",  # F6P + ATP → FBP + ADP
    "FBA":  "cpd:C05345 = cpd:C00111 + cpd:C00118", # FBP → DHAP + G3P
}

results = []
for name, formula in pathway_reactions.items():
    rxn = cc.parse_reaction_formula(formula)
    mu, sigma = cc.standard_dg_prime(rxn)
    results.append({"reaction": name, "drg0": mu, "uncertainty": sigma})

df = pd.DataFrame(results)
print(df)
```

This identifies thermodynamically unfavorable steps (positive $\Delta_r G'^\circ$) that may require concentration driving forces or coupling to exergonic reactions.

## Interpreting eQuilibrator Output

**Worked example — the non-oxidative pentose phosphate pathway:**

The transketolase reaction (xylulose-5-phosphate + ribose-5-phosphate → sedoheptulose-7-phosphate + glyceraldehyde-3-phosphate) has $\Delta_r G'^\circ \approx -0.3 \, \text{kJ/mol}$. This is effectively zero — the reaction is near equilibrium. Its flux is determined by mass action, not by kinetic regulation. This tells a metabolic engineer that upregulating the transketolase enzyme may not increase flux through this step; instead, manipulating substrate availability matters more.

## Limitations and Caveats

- **Uncertainty is real**: for some compound classes (lipids, complex cofactors), component contribution estimates have large errors (>20 kJ/mol); treat with caution
- **Temperature**: values are computed at 25°C; biological systems at 37°C will differ by $\sim \!1$ kJ/mol per reaction for most reactions
- **In vivo conditions**: metabolite concentrations inside cells are not precisely known; eQuilibrator analyses are only as good as the concentration inputs

## Why This Matters

eQuilibrator transforms thermodynamic analysis from a literature-intensive chore into a reproducible computational step. Integrating it with FBA (via tools like `pytfa`) produces thermodynamically consistent flux predictions and eliminates energetically impossible loop fluxes — making metabolic models substantially more reliable for biotechnology applications.
