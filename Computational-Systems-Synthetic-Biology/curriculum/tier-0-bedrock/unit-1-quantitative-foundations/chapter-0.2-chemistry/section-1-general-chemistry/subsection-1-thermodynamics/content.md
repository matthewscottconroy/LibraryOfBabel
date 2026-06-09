# Chemical Thermodynamics

Here is the question that biology spent two billion years learning to exploit: *how do you make something happen that wouldn't happen on its own?*

A muscle fiber pulling against a load, a ribosome threading an amino acid onto a growing chain, a membrane pump pushing sodium ions uphill against their concentration gradient — none of these events are spontaneous. Left to themselves, muscles relax, proteins fall apart, ion gradients dissipate. Yet life builds order continuously, and it does so by coupling these unfavorable processes to others that release energy. The discipline that gives you the tools to reason about all of this quantitatively is thermodynamics.

Thermodynamics governs which chemical reactions can happen spontaneously and which require energy input. Every metabolic reaction, every protein folding event, every molecular motor step is subject to thermodynamic constraints. Understanding these constraints is not optional for computational biologists — they are the physical law layer beneath every biochemical model. You can think of thermodynamics as the set of constraints your model must respect before you've written a single differential equation.

## The Laws of Thermodynamics

**First Law (Conservation of Energy):** The total energy of an isolated system is constant. Energy can be converted between forms (chemical bond energy, heat, mechanical work) but cannot be created or destroyed. The internal energy change of a system: $\Delta U = q + w$ where $q$ is heat absorbed and $w$ is work done on the system.

**Enthalpy** is defined as $H = U + PV$ (where $P$ is pressure and $V$ is volume). At constant pressure (the usual condition for biochemical reactions in cells): $\Delta H = q_P$ — the heat absorbed at constant pressure. Bond-breaking reactions are endothermic ($\Delta H > 0$); bond-forming reactions are exothermic ($\Delta H < 0$).

**Second Law:** In any spontaneous process, the total entropy of the universe increases: $\Delta S_{\text{universe}} = \Delta S_{\text{system}} + \Delta S_{\text{surroundings}} \geq 0$.

**Entropy** $S$ measures disorder or the number of microstates $\Omega$ accessible to a system: $S = k_B \ln \Omega$ (Boltzmann). Processes that increase disorder (unfolding a protein, mixing solutions) are entropically favorable.

It's worth pausing on what entropy really means for biology. When a protein folds, it becomes more ordered — so how can folding be spontaneous? The answer is that folding buries hydrophobic residues in the protein's interior, releasing ordered water molecules that surrounded them in the unfolded state. The entropy of those released water molecules increases more than the entropy of the protein decreases. The *universe's* entropy increases, even as the protein becomes more structured. This hydrophobic effect is the dominant driving force for protein folding, and it is fundamentally an entropic phenomenon.

## Gibbs Free Energy

The **Gibbs free energy** $G = H - TS$ combines enthalpy and entropy into a single criterion for spontaneity at constant temperature and pressure:

$$\Delta G = \Delta H - T \Delta S$$

This is the master equation for biochemical thermodynamics:
- $\Delta G < 0$: **exergonic** — spontaneous
- $\Delta G > 0$: **endergonic** — non-spontaneous (requires energy input)
- $\Delta G = 0$: at **equilibrium**

The beauty of Gibbs free energy is that it handles both the enthalpic contribution (whether bonds break or form) and the entropic contribution (whether disorder increases or decreases) in a single number. A reaction can have unfavorable enthalpy but be driven by favorable entropy, or vice versa. Temperature determines which wins.

**Standard Gibbs free energy** $\Delta G^\circ$ is the free energy change under standard conditions (1 M concentrations, 25°C, 1 atm). In biochemistry, the convention is $\Delta G^{\circ'}$: standard conditions at pH 7 (with water at 55.5 M activity = 1, $H^+$ at $10^{-7}$ M = 1).

The relationship to equilibrium: $\Delta G^{\circ'} = -RT \ln K_{eq}$

**Under non-standard conditions:**

$$\Delta G = \Delta G^{\circ'} + RT \ln Q$$

where $Q = [\text{products}]/[\text{reactants}]$ is the reaction quotient. When $Q < K_{eq}$, $\Delta G < 0$ and the reaction proceeds forward. This distinction between $\Delta G^\circ$ and $\Delta G$ is one of the most important in biochemistry — a reaction can have a positive $\Delta G^\circ$ (unfavorable at standard conditions) yet proceed spontaneously inside a cell because the cell maintains concentrations far from equilibrium. We will return to this point repeatedly.

## Coupled Reactions and ATP

A crucial feature of cellular biochemistry is **reaction coupling** — thermodynamically unfavorable reactions can be driven forward by coupling them to favorable ones:

$$\Delta G_{\text{coupled}} = \Delta G_1 + \Delta G_2$$

If $\Delta G_1 > 0$ (unfavorable) and $\Delta G_2 < 0$ (favorable), the coupled reaction proceeds spontaneously if $\Delta G_1 + \Delta G_2 < 0$.

**ATP hydrolysis** is the universal coupling agent:

$$\text{ATP} + H_2O \to \text{ADP} + P_i \quad \Delta G^{\circ'} = -30.5\ \text{kJ/mol}$$

Under cellular conditions ($[\text{ATP}]/[\text{ADP}] \gg 1$, $[P_i] \approx 1-5$ mM), the actual $\Delta G$ is even more negative: $\approx -50$ to $-60$ kJ/mol.

This energy is used to drive unfavorable reactions: biosynthetic reactions ($\Delta G^{\circ'} > 0$), concentration gradients (maintaining $[Na^+]$ gradient across cell membrane costs ~30 kJ/mol per cycle of the Na/K ATPase), and mechanical work (myosin motor step: ~25 kJ/mol).

You might think of ATP as a kind of molecular voucher — it stores a promise of free energy that can be redeemed to drive otherwise unfavorable chemistry. The cell earns ATP by oxidizing glucose and other fuels, then spends it on everything from muscle contraction to active transport to biosynthesis. The budget must balance: the total ATP production from catabolism must equal the total ATP consumption by biosynthesis and mechanical work. This balance is captured explicitly in the stoichiometric matrix of flux balance analysis.

## Worked Example: Glucose Oxidation

The complete oxidation of glucose to CO$_2$ and H$_2$O:

$$\text{C}_6\text{H}_{12}\text{O}_6 + 6\text{O}_2 \to 6\text{CO}_2 + 6\text{H}_2\text{O} \quad \Delta G^{\circ'} = -2870\ \text{kJ/mol}$$

This enormous free energy release drives the synthesis of ~30 ATP per glucose in oxidative phosphorylation. Glycolysis alone (anaerobic) yields only 2 ATP ($\Delta G \approx -60$ kJ/mol consumed per ATP synthesis: $2 \times 50 = 100$ kJ captured out of ~190 kJ released in glycolysis alone — roughly 50% efficiency).

## Temperature Dependence and the Van't Hoff Equation

The equilibrium constant's dependence on temperature:

$$\frac{d \ln K_{eq}}{dT} = \frac{\Delta H^\circ}{RT^2}$$

Integrated form (**van't Hoff equation**):

$$\ln\frac{K_2}{K_1} = -\frac{\Delta H^\circ}{R}\left(\frac{1}{T_2} - \frac{1}{T_1}\right)$$

For exothermic reactions ($\Delta H < 0$): $K_{eq}$ decreases with temperature — equilibrium shifts toward reactants at higher $T$. For endothermic reactions ($\Delta H > 0$): $K_{eq}$ increases with temperature. This is why enzyme activities are typically maximized near the organism's physiological temperature and decrease above it (though kinetics increase monotonically with $T$ until denaturation).

## Why This Matters for Computational Biology

Thermodynamics provides the constraints that no biological system can violate. In flux balance analysis (FBA), thermodynamic feasibility constraints (requiring $\Delta G < 0$ for all reactions in the direction they run) dramatically reduce the feasible flux space and improve predictions. In protein structure prediction, the native fold is the minimum free energy structure. In synthetic biology, designing efficient metabolic pathways requires understanding the thermodynamic driving force of each step — a pathway with endergonic steps requires coupling to ATP hydrolysis or maintaining far-from-equilibrium concentrations. In RNA secondary structure prediction (mfold, RNAfold), free energy minimization is the objective. Thermodynamics is not just theoretical background — it is a computational tool.

```python
import numpy as np

# Compute actual Gibbs free energy under cellular conditions
R = 8.314e-3  # kJ/(mol·K)
T = 310.0     # Kelvin (37°C)

def actual_delta_G(delta_G0_prime, products_conc, reactants_conc):
    """Compute actual ΔG given concentrations."""
    Q = products_conc / reactants_conc
    return delta_G0_prime + R * T * np.log(Q)

# ATP hydrolysis under cellular conditions
# ATP ~5 mM, ADP ~0.5 mM, Pi ~1 mM
delta_G0_atp = -30.5  # kJ/mol (standard biochemical)
Q_atp = (0.5e-3 * 1e-3) / (5e-3)  # [ADP][Pi]/[ATP]
delta_G_atp = actual_delta_G(delta_G0_atp, 0.5e-3 * 1e-3, 5e-3)
print(f"ATP hydrolysis:")
print(f"  ΔG°' = {delta_G0_atp:.1f} kJ/mol")
print(f"  ΔG (cellular) = {delta_G_atp:.1f} kJ/mol")

# Example: phosphofructokinase reaction (glycolysis)
# F6P + ATP -> F1,6BP + ADP  ΔG°' = -14.2 kJ/mol
delta_G0_pfk = -14.2
# Typical concentrations in E. coli
Q_pfk = (0.02e-3 * 0.5e-3) / (0.1e-3 * 5e-3)
delta_G_pfk = actual_delta_G(delta_G0_pfk, Q_pfk, 1.0)  # Q already computed
print(f"\nPhosphofructokinase (glycolysis):")
print(f"  ΔG°' = {delta_G0_pfk:.1f} kJ/mol")
print(f"  ΔG (cellular) ≈ {delta_G0_pfk + R*T*np.log(Q_pfk):.1f} kJ/mol")
print(f"  Reaction is {'favorable' if delta_G0_pfk + R*T*np.log(Q_pfk) < 0 else 'unfavorable'}")
```
