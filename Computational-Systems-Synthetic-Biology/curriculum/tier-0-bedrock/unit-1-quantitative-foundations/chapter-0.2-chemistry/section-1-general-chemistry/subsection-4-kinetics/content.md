# Chemical Kinetics

A match head contains sulfur, potassium chlorate, and a little phosphorus — all the thermodynamic ingredients for an exothermic combustion reaction. Yet a match can sit in your pocket for years without igniting. Thermodynamics tells you the reaction is favorable; it says nothing about how quickly the match will burn. That is the province of kinetics.

While thermodynamics tells you whether a reaction can occur, kinetics tells you how fast it occurs. A reaction may be thermodynamically favorable (large negative $\Delta G$) yet proceed vanishingly slowly under physiological conditions — understanding kinetics is what separates a thermodynamicist from a biochemist. Chemical kinetics underpins all of enzyme kinetics, ODE-based modeling of biochemical systems, and the quantitative analysis of how quickly cells respond to signals.

This distinction is not merely academic. Diamond is thermodynamically less stable than graphite at ambient conditions, yet we do not worry about our engagement rings spontaneously turning into pencil lead. The hydrolysis of the peptide bond is thermodynamically favorable yet requires harsh acid or base — or a protease enzyme — to proceed on any reasonable timescale. Biology exploits kinetic barriers constantly, using enzymes to selectively accelerate only those reactions that should proceed and leaving others kinetically "silenced."

## Reaction Rates and Rate Laws

The **reaction rate** $v$ is the change in concentration of a product (or negative change in reactant) per unit time:

$$v = \frac{d[\text{product}]}{dt} = -\frac{d[\text{reactant}]}{dt}$$

A **rate law** expresses the rate as a function of reactant concentrations and a rate constant $k$. For a reaction $A + B \to C$:

$$v = k[A]^m [B]^n$$

where $m$ and $n$ are the **reaction orders** with respect to each reactant (not necessarily the stoichiometric coefficients — these are determined experimentally). The **overall order** is $m + n$.

**Zero-order reaction:** $v = k$ (rate independent of concentration). Occurs when an enzyme is saturated with substrate — the enzyme's active site is the rate-limiting factor, not substrate availability. Many biosynthetic reactions in cells run near zero-order with respect to substrate because substrate concentrations far exceed the $K_m$.

**First-order reaction:** $v = k[A]$. The most common order in biology. Radioactive decay, mRNA degradation, protein turnover, and many signaling events are first-order. Solution: $[A](t) = [A]_0 e^{-kt}$, with half-life $t_{1/2} = \ln 2 / k$.

**Second-order reaction:** $v = k[A][B]$ (bimolecular) or $v = k[A]^2$. Bimolecular association reactions (TF binding DNA, antibody-antigen binding, protein dimerization) are second-order. The rate constant $k_{\text{on}}$ for diffusion-limited binding is $\sim 10^8 - 10^9$ M$^{-1}$s$^{-1}$.

The prevalence of first-order kinetics in biology is not a coincidence. Degradation reactions — mRNA turnover, protein proteolysis — are typically first-order because the rate-limiting step is recognition and cleavage by a protease or nuclease, not substrate availability. This means that the half-life of an mRNA is an intrinsic property of that molecule, independent of its current concentration. That's a useful design property: cells can tune the rate of response to a signal by changing the stability of the response molecules.

## The Arrhenius Equation

The temperature dependence of reaction rate constants follows the **Arrhenius equation**:

$$k = A \cdot e^{-E_a / RT}$$

where:
- $A$ is the **pre-exponential factor** (frequency of molecular collisions with correct orientation)
- $E_a$ is the **activation energy** (energy barrier the reactants must overcome)
- $R = 8.314$ J/(mol·K), $T$ is absolute temperature

Taking logarithms: $\ln k = \ln A - E_a / (RT)$. An **Arrhenius plot** ($\ln k$ vs. $1/T$) gives a straight line with slope $-E_a/R$.

**The Q$_{10}$ rule:** For many biological reactions, the rate roughly doubles for each 10°C temperature increase ($Q_{10} \approx 2$). This corresponds to $E_a \approx 50$ kJ/mol.

Enzymes dramatically lower $E_a$ — from typically ~100 kJ/mol for the uncatalyzed reaction to ~10–50 kJ/mol for the enzyme-catalyzed reaction. This can increase rates by factors of $10^{10}$ or more. **Catalysts do not change $\Delta G$ or the equilibrium position** — they only lower the kinetic barrier to reaching equilibrium.

## Transition State Theory

**Transition state theory** (Eyring equation) connects kinetics to thermodynamics:

$$k = \frac{k_B T}{h} e^{-\Delta G^\ddagger / RT} = \frac{k_B T}{h} e^{\Delta S^\ddagger / R} e^{-\Delta H^\ddagger / RT}$$

where $\Delta G^\ddagger$ is the **free energy of activation** (the free energy difference between the ground state and the transition state), $k_B$ is Boltzmann's constant, and $h$ is Planck's constant.

This decomposition into $\Delta H^\ddagger$ (activation enthalpy) and $\Delta S^\ddagger$ (activation entropy) is useful for understanding enzyme mechanism. Enzymes achieve rate acceleration through:
- **Transition state stabilization:** Binding the transition state more tightly than the ground state (lowers $\Delta G^\ddagger$)
- **Entropy reduction:** Holding substrates in the correct orientation eliminates the translational/rotational entropy cost of bringing them together (increases $\Delta S^\ddagger$)
- **Acid-base catalysis:** Proton transfers that stabilize charged transition states

The transition state concept also explains why transition state analogs are such powerful enzyme inhibitors. If an enzyme is designed by evolution to bind the transition state tightly, then a stable molecule that mimics the transition state geometry will bind extremely tightly — often with $K_d$ values in the picomolar range. This principle has led to some of the most potent enzyme inhibitors known, including HIV protease inhibitors and the influenza drug oseltamivir.

## Pre-Steady-State vs. Steady-State Kinetics

Most enzyme kinetic data is collected under **steady-state conditions** — where enzyme-substrate complex ES has reached a pseudo-steady state and the substrate is in great excess over enzyme. This is the basis of the Michaelis-Menten equation (see Section 3.2).

**Pre-steady-state kinetics** (stopped-flow, rapid quench methods) captures the initial transient phases before steady state is established. These measurements reveal elementary rate constants ($k_{\text{on}}$, $k_{\text{off}}$, $k_{\text{cat}}$) individually, rather than the composite parameters $K_m$ and $V_{\text{max}}$. Pre-steady-state analysis is essential for understanding the detailed kinetic mechanism of enzymes, including nucleotide incorporation fidelity of DNA polymerases.

## Worked Example: mRNA Degradation Kinetics

In a simple gene expression model, mRNA is produced at rate $\beta_m$ and degraded first-order at rate $\delta_m$:

$$\frac{d[m]}{dt} = \beta_m - \delta_m [m]$$

At steady state: $[m]^* = \beta_m / \delta_m$.

The half-life is $t_{1/2} = \ln 2 / \delta_m$. **Typical values:** *E. coli* mRNA $t_{1/2} \approx 2$ min ($\delta_m \approx 0.35$ min$^{-1}$); mammalian mRNA $t_{1/2} \approx 1-10$ h. This difference is why bacterial cells respond much faster to environmental changes — they can rapidly degrade old transcripts and replace them with new ones, while mammalian cells have longer mRNA half-lives that smooth out rapid fluctuations.

This simple model illustrates something profound: the timescale of response is set not by the production rate but by the degradation rate. A cell can increase protein production instantly by increasing transcription, but the new steady-state level of mRNA won't be reached until enough time has elapsed for the old mRNAs to turn over. The half-life is the fundamental timescale of the system.

## Why This Matters for Computational Biology

Every rate constant in an ODE model of a gene circuit or metabolic pathway comes from kinetics. Understanding how rate constants depend on concentration (order of reaction), temperature (Arrhenius), and enzyme saturation is essential for choosing the right rate law. The difference between a linear first-order degradation term ($\delta [P]$) and a Michaelis-Menten degradation term ($V_{\text{max}} [P] / (K_m + [P])$) can qualitatively change the dynamics of a gene circuit — saturable degradation can produce pulsatile behaviors that linear degradation cannot. Knowing when each approximation is valid requires understanding kinetics at a mechanistic level.

```python
import numpy as np
import matplotlib.pyplot as plt

# Arrhenius equation: rate constant vs temperature
R = 8.314  # J/(mol·K)

def arrhenius_k(T_celsius, A, Ea_kJmol):
    T = T_celsius + 273.15
    Ea = Ea_kJmol * 1000  # convert to J/mol
    return A * np.exp(-Ea / (R * T))

T_range = np.linspace(4, 60, 100)  # 4°C to 60°C

# Two enzymes: moderate vs high activation energy
k1 = arrhenius_k(T_range, 1e10, 50)   # Ea = 50 kJ/mol
k2 = arrhenius_k(T_range, 1e12, 80)   # Ea = 80 kJ/mol

fig, axes = plt.subplots(1, 2, figsize=(12, 4))

axes[0].plot(T_range, k1/k1.max(), label='Ea = 50 kJ/mol')
axes[0].plot(T_range, k2/k2.max(), label='Ea = 80 kJ/mol')
axes[0].set_xlabel('Temperature (°C)'); axes[0].set_ylabel('Relative rate constant')
axes[0].set_title('Arrhenius: Rate vs Temperature')
axes[0].legend()

# Arrhenius plot (linear form)
T_K = T_range + 273.15
axes[1].plot(1/T_K * 1000, np.log(k1), label='Ea = 50 kJ/mol')
axes[1].plot(1/T_K * 1000, np.log(k2), label='Ea = 80 kJ/mol')
axes[1].set_xlabel('1000/T (K⁻¹)'); axes[1].set_ylabel('ln k')
axes[1].set_title('Arrhenius Plot (ln k vs 1/T)')
axes[1].legend()

plt.tight_layout()

# Q10 calculation
k_20 = arrhenius_k(20, 1e10, 50)
k_30 = arrhenius_k(30, 1e10, 50)
Q10 = k_30 / k_20
print(f"Q10 for Ea=50 kJ/mol: {Q10:.2f}")

# mRNA degradation half-life
delta_m_ecoli = np.log(2) / 2  # 2 min half-life in E. coli
delta_m_human = np.log(2) / 120  # 2 hour half-life in human
print(f"\nE. coli mRNA degradation rate: {delta_m_ecoli:.3f} min⁻¹ (t½ = 2 min)")
print(f"Human mRNA degradation rate: {delta_m_human:.5f} min⁻¹ (t½ = 2 h)")
```
