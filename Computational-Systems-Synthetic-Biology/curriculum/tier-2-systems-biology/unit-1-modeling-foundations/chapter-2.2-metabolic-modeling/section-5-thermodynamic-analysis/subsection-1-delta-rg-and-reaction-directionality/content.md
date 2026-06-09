# ΔrG' and Reaction Directionality

## The Fundamental Constraint on Flux

Here is a question that should bother you about FBA: how does the stoichiometric matrix know which direction a reaction runs? When you write a reversible reaction in the model with bounds $[-1000, +1000]$, you are admitting that the LP solver can push flux in either direction — but thermodynamics says that the actual direction must be determined by the Gibbs free energy. FBA, as usually formulated, ignores this entirely. Thermodynamic analysis puts it back in.

Stoichiometric models like FBA can predict which flux distributions are mass-balanced and capacity-feasible, but they cannot distinguish whether a reaction will actually run forward or backward under cellular conditions. That constraint comes from thermodynamics. A reaction can only carry net flux in the direction that decreases Gibbs free energy — this is not a preference but a physical law.

The **Gibbs free energy change of a reaction** under physiological conditions is:

$$\Delta_r G' = \Delta_r G'^\circ + RT \ln Q$$

where:
- $\Delta_r G'^\circ$ is the standard transformed Gibbs free energy change (at pH 7, ionic strength 0.25 M, 25°C, all concentrations at 1 M)
- $R = 8.314 \, \text{J mol}^{-1} \text{K}^{-1}$ is the gas constant
- $T$ is temperature in Kelvin (typically 310 K for mammalian cells)
- $Q$ is the **reaction quotient**: the ratio of actual product to reactant concentrations, each raised to their stoichiometric coefficients

A reaction proceeds spontaneously in the forward direction only when $\Delta_r G' < 0$. At thermodynamic equilibrium, $\Delta_r G' = 0$ and the ratio $Q = K_{eq}$.

## From Standard to Physiological Conditions

The standard free energy $\Delta_r G'^\circ$ is a fixed property of the reaction chemistry, derivable from the equilibrium constant:

$$\Delta_r G'^\circ = -RT \ln K_{eq}$$

Consider the phosphoglucose isomerase reaction:

$$\text{glucose-6-phosphate} \rightleftharpoons \text{fructose-6-phosphate}$$

The equilibrium constant $K_{eq} \approx 0.29$ at pH 7, giving $\Delta_r G'^\circ \approx +1.7 \, \text{kJ/mol}$ — slightly unfavorable under standard conditions. Yet this reaction runs forward in glycolysis. Why?

Inside a cell, the concentrations of glucose-6-phosphate (G6P) and fructose-6-phosphate (F6P) are not at 1 M each. Typical values are approximately $[\text{G6P}] \approx 0.4 \, \text{mM}$ and $[\text{F6P}] \approx 0.06 \, \text{mM}$:

$$Q = \frac{[\text{F6P}]}{[\text{G6P}]} = \frac{0.06}{0.4} = 0.15$$

$$\Delta_r G' = 1700 + (8.314)(310)\ln(0.15) \approx 1700 - 4800 = -3100 \, \text{J/mol}$$

The reaction is thermodynamically favorable under cellular conditions despite unfavorable standard free energy. This example illustrates a critical lesson: **$\Delta_r G'^\circ$ tells you about equilibrium; $\Delta_r G'$ tells you about the actual direction of flux**.

## The Reaction Quotient in Metabolic Networks

For a general reaction $\sum \nu_i S_i = 0$ (where $\nu_i$ are signed stoichiometric coefficients, positive for products):

$$Q = \prod_i [S_i]^{\nu_i}$$

The concentration-dependent term $RT \ln Q$ can shift reactions that are unfavorable at standard conditions into the feasible range, or conversely render apparently favorable reactions thermodynamically blocked.

**Highly exergonic reactions** (large negative $\Delta_r G'^\circ$, like ATP hydrolysis: $\approx -30 \, \text{kJ/mol}$) are essentially irreversible under physiological conditions. Even if products accumulate substantially, the concentration term is insufficient to make $\Delta_r G' > 0$.

**Near-equilibrium reactions** (small $|\Delta_r G'^\circ|$) are easily pushed in either direction by changes in metabolite concentrations. These reactions are rapidly reversible and their flux direction is determined by local metabolite pool sizes.

## Thermodynamic Feasibility Analysis

Given measured or estimated intracellular metabolite concentrations, one can compute $\Delta_r G'$ for every reaction in a network and check for thermodynamic consistency:

```python
from equilibrator_api import ComponentContribution, Q_
cc = ComponentContribution()

# Define reaction
rxn = cc.parse_reaction_formula("glc__D_c = fru_c")
# Compute standard ΔrG'°
drg0 = cc.standard_dg_prime(rxn)
print(f"ΔrG'° = {drg0:.1f} kJ/mol")

# Adjust for actual concentrations
# Q = [fru]/[glc] = 0.06 mM / 0.4 mM
import numpy as np
RT = 8.314e-3 * 310  # kJ/mol
Q = 0.06 / 0.4
drg_prime = drg0.m + RT * np.log(Q)
print(f"ΔrG' = {drg_prime:.1f} kJ/mol")
```

## Identifying Irreversible and Near-Equilibrium Reactions

Metabolic engineers and systems biologists classify reactions by their thermodynamic regime:

| Category | $\Delta_r G'$ | Implication |
|---|---|---|
| Irreversible (forward) | $\ll 0$ | Cannot carry reverse flux; rate-limiting candidates |
| Near-equilibrium | $\approx 0$ | Flux set by downstream demand; rapid response |
| Thermodynamically infeasible | $> 0$ | Cannot run in assumed direction — model error or missing data |

Reactions with large negative $\Delta_r G'$ represent **committed steps** — the cell has invested thermodynamic potential to drive the network irreversibly forward. These are frequent sites of allosteric regulation (e.g., PFK-1 in glycolysis: $\Delta_r G' \approx -25 \, \text{kJ/mol}$).

## Integrating Thermodynamics with FBA

Standard FBA ignores thermodynamics entirely — it can predict flux distributions where a reaction simultaneously appears to run forward and backward in a loop (thermodynamically infeasible "energy generating cycles"). **Thermodynamic FBA (tFBA)** augments the LP with thermodynamic constraints:

$$\Delta_r G'_j < 0 \iff v_j > 0$$

This eliminates thermodynamically infeasible loops and restricts the feasible flux space to distributions that are simultaneously stoichiometrically and thermodynamically consistent.

## Why This Matters

Understanding $\Delta_r G'$ and concentration-dependent directionality is essential for metabolic engineering: a pathway that looks feasible on paper (based on $\Delta_r G'^\circ$ alone) may be thermodynamically blocked under the metabolite concentrations that accumulate during production. Thermodynamic analysis should be a first-pass check before committing to any heterologous pathway design. Tools like eQuilibrator make this analysis accessible without extensive manual computation.
