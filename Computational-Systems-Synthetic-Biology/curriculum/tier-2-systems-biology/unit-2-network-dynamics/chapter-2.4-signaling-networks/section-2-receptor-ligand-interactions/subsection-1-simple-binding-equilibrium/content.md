# Simple Binding Equilibrium

## The Law of Mass Action for Receptor-Ligand Binding

Before a cell can respond to insulin, EGF, or any other hormone, it must first catch the molecule. That act of catching — a ligand drifting through solution and colliding with its receptor — is the founding event of all cell signaling, and it obeys the law of mass action with satisfying simplicity. The simplest model of receptor-ligand interaction treats it as a bimolecular reversible reaction:

$$R + L \underset{k_{-1}}{\overset{k_1}{\rightleftharpoons}} RL$$

where $R$ is free receptor, $L$ is free ligand, $RL$ is the bound complex, $k_1$ is the association rate constant (units: M⁻¹s⁻¹), and $k_{-1}$ is the dissociation rate constant (units: s⁻¹).

The equilibrium **dissociation constant** is:

$$K_d = \frac{k_{-1}}{k_1} = \frac{[R][L]}{[RL]}$$

A smaller $K_d$ means tighter binding — the complex dissociates less readily.

## Receptor Occupancy and the Dose-Response Curve

Assuming total receptor concentration $R_T = [R] + [RL]$ is constant, and ligand concentration $[L]$ is not depleted by binding (valid when $[L] \gg R_T$), the fractional receptor occupancy at equilibrium is:

$$\theta = \frac{[RL]}{R_T} = \frac{[L]}{K_d + [L]}$$

This is the **Hill-Langmuir equation** (Hill coefficient $n=1$, hyperbolic). It has several important properties:

- At $[L] = K_d$: $\theta = 0.5$ — exactly half of receptors are occupied
- At $[L] \ll K_d$: $\theta \approx [L]/K_d$ (linear response)
- At $[L] \gg K_d$: $\theta \approx 1$ (saturation)
- EC50 = $K_d$ (the ligand concentration producing 50% response)

```python
import numpy as np
import matplotlib.pyplot as plt

def receptor_occupancy(L, Kd):
    """Fractional receptor occupancy as function of ligand concentration."""
    return L / (Kd + L)

Kd = 1e-9  # 1 nM (typical high-affinity receptor)
L = np.logspace(-12, -6, 200)  # 1 pM to 1 µM

theta = receptor_occupancy(L, Kd)

# Compute EC50 and Hill coefficient region
ec50_idx = np.argmin(np.abs(theta - 0.5))
print(f"EC50 = {L[ec50_idx]*1e9:.2f} nM (should equal Kd = {Kd*1e9:.1f} nM)")
print(f"Hill coefficient = 1 (hyperbolic response)")
```

## Cooperative Binding: Hill Equation

For receptors with multiple binding sites that exhibit cooperativity (binding of one ligand facilitates binding of subsequent ligands), the dose-response curve becomes sigmoidal:

$$\theta = \frac{[L]^n}{K_d^n + [L]^n}$$

where $n$ is the Hill coefficient. For $n > 1$: positive cooperativity (sigmoidal, switch-like). For $n < 1$: negative cooperativity. For $n = 1$: no cooperativity (hyperbolic).

The Hill coefficient characterizes the steepness of the dose-response curve. The EC10-to-EC90 range spans:
- $n = 1$: 81-fold concentration range
- $n = 2$: 9-fold range
- $n = 4$: 3-fold range
- $n \to \infty$: step function (perfect switch)

## Kinetics of Binding

At equilibrium, occupancy is described by $K_d$ alone. But the time to reach equilibrium depends on both rate constants and the ligand concentration:

$$\frac{d[RL]}{dt} = k_1 [R][L] - k_{-1}[RL]$$

The relaxation rate (rate of approach to equilibrium) is:

$$k_{\text{obs}} = k_1 [L] + k_{-1}$$

At high ligand concentrations: $k_{\text{obs}} \approx k_1 [L]$ — binding is fast and approximately first-order in $[L]$.
At low ligand concentrations: $k_{\text{obs}} \approx k_{-1}$ — the off-rate dominates and equilibration is slow.

The half-time for equilibration: $t_{1/2} = \ln 2 / k_{\text{obs}}$.

**Worked example** — Epidermal Growth Factor (EGF) binding to EGFR:
- $k_1 \approx 3 \times 10^6 \, \text{M}^{-1}\text{s}^{-1}$
- $k_{-1} \approx 3 \times 10^{-3} \, \text{s}^{-1}$
- $K_d = k_{-1}/k_1 \approx 1 \, \text{nM}$
- At $[\text{EGF}] = 1 \, \text{nM}$: $k_{\text{obs}} = 3 \times 10^{-3} + 3 \times 10^{-3} = 6 \times 10^{-3} \, \text{s}^{-1}$, $t_{1/2} \approx 115 \, \text{s}$

## Receptor Internalization

In reality, the bound RL complex is often internalized (endocytosed) rather than existing at equilibrium at the cell surface:

$$R + L \to RL \to \text{internalized} \to R_{\text{recycled}} \text{ or } R_{\text{degraded}}$$

This creates a pseudo-steady state rather than a true equilibrium. The apparent $K_d$ for cell-level responses can differ substantially from the in vitro binding $K_d$ because:
- Receptor recycling (resensitization) affects the effective receptor pool
- Internalized complexes may still signal (from endosomes)
- Receptor downregulation (ligand-induced degradation) depletes receptors at high chronic ligand concentrations

## Measuring Binding: Experimental Methods

**Radioligand binding**: incubate cells with ¹²⁵I-labeled ligand at multiple concentrations; measure total and non-specific binding; Scatchard analysis yields $K_d$ and $B_{\max}$ (total receptor number).

**Surface plasmon resonance (SPR/Biacore)**: flow ligand over immobilized receptor; measure real-time binding kinetics; directly yields $k_1$, $k_{-1}$, and $K_d$.

**Flow cytometry with fluorescent ligand**: measure cell-bound fluorescence across ligand concentrations; yields $K_d$ directly in cell-based format.

**FRET/BRET assays**: report ligand-induced receptor conformational changes or dimerization in live cells.

## Spare Receptors and Non-Linear Stimulus-Response

Biological responses often begin at receptor occupancies much below 100%. If a half-maximal response is achieved at $\theta = 0.01$ (1% receptor occupancy), the remaining 99% of receptors are "spare receptors." This spare receptor pool:
- Increases sensitivity to ligand (response is 50% maximal at $[L] = K_d/99 \approx 0.01 K_d$)
- Provides a safety margin against partial receptor loss
- Means the operational EC50 of the drug response is much less than $K_d$

This distinction between the biochemical $K_d$ (receptor binding) and the pharmacological EC50 (physiological response) is fundamental to understanding dose-response relationships in pharmacology.

## Why This Matters

The simple binding equilibrium is the foundation of quantitative pharmacology. Every drug that targets a receptor must be understood in terms of its $K_d$ (or $K_i$ for inhibitors), cooperativity, and kinetics. Understanding these relationships predicts drug occupancy at therapeutic plasma concentrations, receptor selectivity, and the time course of drug action. More importantly, deviations from simple binding (cooperativity, spare receptors, biased agonism) explain why drugs with similar binding affinities can have dramatically different physiological effects.
