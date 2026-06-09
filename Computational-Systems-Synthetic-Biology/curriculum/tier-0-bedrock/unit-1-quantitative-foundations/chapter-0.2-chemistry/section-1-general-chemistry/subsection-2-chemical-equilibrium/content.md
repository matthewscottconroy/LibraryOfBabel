# Chemical Equilibrium

Imagine watching a test tube in which you've mixed two proteins that bind each other. Before your eyes, the solution appears to reach a stable state — the turbidity stops changing, the fluorescence levels off. You might conclude that the reaction has stopped. But it hasn't. Molecules are still binding and unbinding constantly; it's just that the rates of the two directions have become equal. This is chemical equilibrium, and the distinction between "stopped" and "balanced" turns out to matter enormously.

Chemical equilibrium is the state a reaction system reaches when the forward and reverse reaction rates are equal — the macroscopic concentrations stop changing, not because the reactions have stopped, but because they proceed at the same rate in both directions. Equilibrium is central to understanding protein-ligand binding, enzyme-substrate interactions, pH buffering, and the driving force for metabolic reactions. Every $K_d$ you will encounter in the biochemical literature, every binding affinity measured by isothermal titration calorimetry, every Hill coefficient in a gene expression model — all of these connect back to the thermodynamics of equilibrium developed in this section.

## The Equilibrium Constant

For a general reaction $aA + bB \rightleftharpoons cC + dD$, the **equilibrium constant** is:

$$K_{eq} = \frac{[C]^c [D]^d}{[A]^a [B]^b}$$

where concentrations are evaluated at equilibrium. $K_{eq}$ is dimensionless (strictly, it uses activities rather than concentrations, but for dilute solutions these are equivalent).

**Relationship to Gibbs free energy:**

$$\Delta G^\circ = -RT \ln K_{eq}$$

or equivalently: $K_{eq} = e^{-\Delta G^\circ / RT}$.

Large $K_{eq}$ ($K_{eq} \gg 1$): products strongly favored, $\Delta G^\circ \ll 0$.
Small $K_{eq}$ ($K_{eq} \ll 1$): reactants strongly favored, $\Delta G^\circ \gg 0$.
$K_{eq} = 1$: $\Delta G^\circ = 0$, equal concentrations of products and reactants at standard conditions.

The exponential relationship between $K_{eq}$ and $\Delta G^\circ$ is worth internalizing: every 5.7 kJ/mol (~1.4 kcal/mol) of free energy corresponds to a factor of 10 in the equilibrium constant at 37°C. A reaction that is 17 kJ/mol favorable has $K_{eq} \approx 1000$, meaning at equilibrium products outnumber reactants a thousandfold. These are the conversions you need to build chemical intuition.

**Worked example — transcription factor binding:**

A transcription factor TF binds DNA at a specific site: $\text{TF} + \text{DNA} \rightleftharpoons \text{TF:DNA}$. The binding constant $K_a$ (association constant) and dissociation constant $K_d$ are:

$$K_a = \frac{[\text{TF:DNA}]}{[\text{TF}][\text{DNA}]}, \qquad K_d = \frac{1}{K_a} = \frac{[\text{TF}][\text{DNA}]}{[\text{TF:DNA}]}$$

$K_d$ has units of concentration (typically nM). At $[\text{TF}] = K_d$, exactly half of the DNA sites are occupied — this is the definition of the dissociation constant and explains why $K_d$ is the most useful measure of binding affinity. Typical transcription factor $K_d$ values range from 1–100 nM.

## Reaction Quotient and Direction of Reaction

The **reaction quotient** $Q$ is computed exactly like $K_{eq}$, but using current (non-equilibrium) concentrations:

$$Q = \frac{[\text{products}]_{\text{current}}}{[\text{reactants}]_{\text{current}}}$$

- If $Q < K_{eq}$: $\Delta G < 0$, reaction proceeds forward (toward products)
- If $Q > K_{eq}$: $\Delta G > 0$, reaction proceeds in reverse
- If $Q = K_{eq}$: at equilibrium

This provides a quantitative criterion for the direction of any biochemical reaction under physiological conditions, even when the standard free energy suggests it should go the other way (or vice versa).

**Example:** The phosphoglucose isomerase reaction (glycolysis): G6P $\rightleftharpoons$ F6P, $\Delta G^{\circ'} = +1.67$ kJ/mol, $K_{eq} = 0.51$. This looks unfavorable ($K_{eq} < 1$), but in the cell, $[\text{G6P}] \approx 0.1$ mM and $[\text{F6P}] \approx 0.02$ mM, so $Q = 0.2 < 0.51$. The reaction proceeds forward despite $\Delta G^{\circ'} > 0$.

This is not an exception — it is the rule. Cells are experts at maintaining their metabolite concentrations far from equilibrium, and this displacement is what provides the thermodynamic driving force for reactions that would otherwise stall. The tools for computing $\Delta G$ under actual cellular concentrations (as in the code at the end of Section 1.1) are essential for identifying which reactions in a metabolic network are far from equilibrium (irreversible, effectively) and which are near equilibrium (potentially reversible).

## Le Chatelier's Principle

**Le Chatelier's principle:** When a system at equilibrium is disturbed, it shifts in the direction that partially counteracts the disturbance.

- **Adding product:** Equilibrium shifts toward reactants
- **Removing product:** Equilibrium shifts toward products
- **Increasing temperature:** Shifts toward the endothermic direction
- **Increasing pressure:** Shifts toward the side with fewer moles of gas

In cellular biochemistry, Le Chatelier's principle is the mechanism by which enzymatic reactions are driven forward: metabolic pathways remove products as fast as they are made (via downstream reactions), maintaining $Q \ll K_{eq}$ and ensuring thermodynamic driving force throughout the pathway. This is why the overall $\Delta G$ of glycolysis is $\sim -80$ kJ/mol even though individual steps have small $\Delta G$.

It turns out that this product-removal trick is exactly what makes metabolic pathways effective. Each enzyme in glycolysis pulls product away from the previous enzyme. The first enzyme's product is the second enzyme's substrate, and so on. The pathway acts like a thermodynamic cascade, each step slightly downhill, the entire chain driven by the enormous favorability of glucose oxidation at the pathway's end.

## Multiple Equilibria and Linkage

Many biological systems involve multiple coupled equilibria. A classic example is the oxygen binding to hemoglobin, which involves:
1. Four sequential binding events with different affinities (cooperativity)
2. pH-dependent binding (Bohr effect: lower pH at tissue decreases O$_2$ affinity)
3. CO$_2$ binding (shifts O$_2$ affinity)
4. BPG (2,3-bisphosphoglycerate) binding (decreases O$_2$ affinity in red blood cells)

The binding of one ligand affects the binding of another — this is **thermodynamic linkage**. Formally: if two ligands X and Y bind the same protein, $\partial \ln K_X / \partial \ln [Y] = \partial \ln K_Y / \partial \ln [X]$ — the effect of Y on X-binding equals the effect of X on Y-binding. This cycle rule derives purely from thermodynamics and is independent of mechanism.

Thermodynamic linkage is one of those quietly powerful results that keeps appearing in unexpected places. It tells you that if you can measure the effect of oxygen on proton binding, you automatically know the effect of protons on oxygen binding — with no additional experiments required. The thermodynamic cycle enforces it.

## Temperature Dependence: Van't Hoff Analysis

A plot of $\ln K_{eq}$ vs. $1/T$ (a **Van't Hoff plot**) gives a straight line with slope $-\Delta H^\circ / R$ and intercept $\Delta S^\circ / R$:

$$\ln K_{eq} = -\frac{\Delta H^\circ}{R} \cdot \frac{1}{T} + \frac{\Delta S^\circ}{R}$$

This allows you to determine $\Delta H^\circ$ and $\Delta S^\circ$ from equilibrium measurements at multiple temperatures — without directly measuring heat. Widely used for characterizing DNA melting, protein unfolding, and ligand binding thermodynamics.

## Why This Matters for Computational Biology

In every ODE model of a biochemical network, the equilibrium constants determine the ratio of forward and reverse rate constants: $K_{eq} = k_f / k_r$. The far-from-equilibrium driving force of reactions determines whether a simplified "irreversible" rate law is valid or whether reverse reactions must be included. In thermodynamic constraint-based modeling (such as the TFBA approach), reaction equilibria are used to eliminate thermodynamically infeasible flux distributions. In drug discovery, $K_d$ values are the primary measure of binding affinity and are used to rank candidate compounds. Understanding equilibrium is understanding why biological systems are poised where they are.

```python
import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import fsolve

# Transcription factor binding equilibrium
def occupancy(TF_total, DNA_total, Kd):
    """Fraction of DNA sites occupied as a function of total TF concentration."""
    # Full binding equation (quadratic): accounts for TF depletion by binding
    # [TF:DNA] = ([TF_tot + DNA_tot + Kd] - sqrt(...))/2
    a = 1
    b = -(TF_total + DNA_total + Kd)
    c = TF_total * DNA_total
    discriminant = b**2 - 4*a*c
    complex_conc = (-b - np.sqrt(discriminant)) / (2*a)
    return complex_conc / DNA_total

# Compare simple Langmuir isotherm vs. full solution
Kd = 10e-9  # 10 nM
DNA_total = 1e-9  # 1 nM (in dilute ChIP experiment)
TF_range = np.logspace(-11, -6, 200)  # 0.01 nM to 1000 nM

# Simple (assumes free [TF] ≈ total [TF])
occ_simple = TF_range / (TF_range + Kd)

# Full quadratic solution
occ_full = np.array([occupancy(tf, DNA_total, Kd) for tf in TF_range])

plt.figure(figsize=(8, 4))
plt.semilogx(TF_range*1e9, occ_simple, label='Simple (Langmuir)', linestyle='--')
plt.semilogx(TF_range*1e9, occ_full, label='Full (quadratic)')
plt.axvline(Kd*1e9, linestyle=':', color='gray', label=f'Kd = {Kd*1e9:.0f} nM')
plt.xlabel('[TF] total (nM)')
plt.ylabel('Fraction DNA occupied')
plt.title('Transcription Factor Binding Isotherm')
plt.legend()
plt.tight_layout()
print(f"At [TF] = Kd = {Kd*1e9} nM, occupancy (simple) = {Kd/(Kd+Kd):.2f}")
```
