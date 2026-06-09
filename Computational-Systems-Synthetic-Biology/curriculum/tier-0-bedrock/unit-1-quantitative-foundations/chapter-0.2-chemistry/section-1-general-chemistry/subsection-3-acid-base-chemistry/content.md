# Acid-Base Chemistry

There is a question you can ask of almost any enzyme that will instantly reveal something deep about its mechanism: what happens to your activity when you change the pH by 0.3 units? The answer, almost invariably, is "a lot." Enzymes are machines made of ionizable groups — aspartates, glutamates, histidines, lysines — whose charge states are exquisitely tuned to their catalytic roles. A histidine must be neutral to act as a proton acceptor; a glutamate must be charged to stabilize a transition state. Change the pH by half a unit, and you shift the equilibrium that controls these ionization states, and the enzyme grinds to a halt.

The intracellular pH is exquisitely controlled at ~7.2–7.4 in most eukaryotic cells, and deviations of even 0.1 pH unit can dramatically alter enzyme activities, protein conformations, and signaling. Acid-base chemistry governs the charge states of amino acids, the binding properties of enzymes, the stability of nucleic acids, and the design of biological buffers. For computational biologists, understanding protonation equilibria is essential for protein electrostatics modeling, drug design, and any simulation involving ionizable groups.

## Brønsted-Lowry Acids and Bases

A **Brønsted-Lowry acid** is a proton ($H^+$) donor; a **base** is a proton acceptor. Every acid-base reaction involves a conjugate acid-base pair:

$$\text{HA} \rightleftharpoons \text{H}^+ + \text{A}^-$$

Acid: HA, conjugate base: A$^-$.

**pH** is the negative log of hydrogen ion activity (approximately equal to concentration in dilute solutions):

$$\text{pH} = -\log_{10}[\text{H}^+]$$

At 37°C, neutral pH $\approx 6.8$ (slightly lower than the 25°C value of 7.0 because water self-ionization increases with temperature). Cellular cytoplasm is typically pH 7.2–7.4.

## The pKa and Protonation State

The **acid dissociation constant** $K_a$ and its negative log $\text{pK}_a$:

$$K_a = \frac{[\text{H}^+][\text{A}^-]}{[\text{HA}]}, \qquad \text{pK}_a = -\log_{10} K_a$$

At pH = pKa, exactly 50% of the molecules are protonated (HA) and 50% are deprotonated (A$^-$). **Lower pKa = stronger acid** (donates protons more readily).

**Critical amino acid pKa values** (approximate values in free amino acid; context in a protein can shift these by 2–4 units):

| Amino acid | Ionizable group | pKa |
|---|---|---|
| Asp (D) | $\beta$-carboxylate | 3.9 |
| Glu (E) | $\gamma$-carboxylate | 4.1 |
| His (H) | imidazole | 6.0 |
| Cys (C) | thiol | 8.3 |
| Lys (K) | $\varepsilon$-amino | 10.5 |
| Arg (R) | guanidinium | 12.5 |
| Tyr (Y) | phenol | 10.1 |

At physiological pH 7.4: Asp, Glu are negative; Lys, Arg are positive; His is ~96% neutral (its pKa near 6 makes it uniquely suited as an acid-base catalyst that is poised between protonated and unprotonated at neutral pH).

The key insight about histidine is worth dwelling on. Its pKa sits just below physiological pH — which means that at pH 7.4, histidine is almost entirely neutral, but barely so. A small change in the local protein environment — a nearby charged residue, a hydrogen bond partner — can shift its effective pKa up into the physiological range, making it an excellent general acid or base. This tunability is why histidine appears in the active sites of serine proteases, phosphatases, and dozens of other enzyme families.

## Henderson-Hasselbalch Equation

The **Henderson-Hasselbalch equation** relates pH, pKa, and the ratio of conjugate base to acid:

$$\text{pH} = \text{pK}_a + \log_{10} \frac{[\text{A}^-]}{[\text{HA}]}$$

**Applications:**
1. **Finding protonation fraction at a given pH:** $f_{\text{prot}} = \frac{1}{1 + 10^{\text{pH} - \text{pK}_a}}$

2. **Buffer design:** A buffer is most effective within ±1 pH unit of the pKa. Common biological buffers: HEPES (pKa 7.55), PIPES (pKa 6.8), phosphate (pKa 7.2 for H$_2$PO$_4^-$/HPO$_4^{2-}$), bicarbonate (pKa 6.1 in physiological CO$_2$/HCO$_3^-$ system).

3. **pI calculation:** The isoelectric point is the pH at which a protein has zero net charge. For a protein with multiple ionizable groups: find the pH at which positive charges exactly balance negative charges.

## Cellular pH Control

Cells maintain pH through multiple mechanisms:
- **Bicarbonate buffer system** (blood): $\text{CO}_2 + \text{H}_2\text{O} \rightleftharpoons \text{H}_2\text{CO}_3 \rightleftharpoons \text{H}^+ + \text{HCO}_3^-$. The lungs control CO$_2$ (and hence pH) by adjusting breathing rate.
- **Phosphate buffer** (intracellular): H$_2$PO$_4^-$/HPO$_4^{2-}$, pKa 7.2.
- **Protein buffering**: Histidine residues on proteins provide intracellular buffering capacity near physiological pH.
- **Proton pumps**: V-type ATPases acidify lysosomes (pH ~5) and Golgi lumen; plasma membrane H$^+$-ATPases in plants; mitochondrial H$^+$ gradient drives ATP synthesis.

The sophistication of these pH control systems reflects how deeply pH is integrated into cellular biochemistry. The fact that the mitochondrial proton gradient — the very engine of ATP synthesis — is also a major source of cellular acidification means that pH control and energy metabolism are inextricably linked. When cells are deprived of oxygen and switch to lactic acid fermentation, the rapid acidification of cytoplasm is one of the first signals of metabolic distress.

## Protonation States in Protein Modeling

In computational protein modeling (molecular dynamics, docking, electrostatics calculations), you must assign protonation states to every ionizable residue. This is non-trivial because the protein environment shifts pKa values dramatically:

- **Buried charged residues** have anomalously shifted pKa values (a Asp buried in a hydrophobic core may have pKa > 8)
- **Salt bridges** between Asp/Glu and Lys/Arg stabilize both ionized forms
- **Catalytic residues** are precisely tuned by their environment: the catalytic His in serine proteases is stabilized as a neutral imidazole by a nearby Asp, lowering its effective pKa

Tools like PROPKA, H++ server, and continuum electrostatics calculations (Poisson-Boltzmann equation) estimate environment-adjusted pKa values computationally.

## Why This Matters for Computational Biology

pH is a critical parameter in every molecular simulation and model. In ODE models of enzyme kinetics, enzyme activity is often a function of pH — including a pH-dependent activity term is essential for accurate prediction over physiological pH ranges. In protein structure prediction and design, charge states must be correctly assigned. In metabolic modeling with thermodynamic constraints, the actual $\Delta G'$ of reactions depends on pH (especially for reactions consuming or producing H$^+$). In synthetic biology, the charge state of DNA, RNA, and proteins affects electrostatic interactions, nuclear localization signals, and membrane translocation. Even in simple PCR primer design, the effect of pH on annealing temperature (via its effect on the phosphate backbone's charge) is relevant.

```python
import numpy as np
import matplotlib.pyplot as plt

def protonation_fraction(pH, pKa):
    """Fraction in protonated (acid) form."""
    return 1 / (1 + 10**(pH - pKa))

# Plot protonation curves for key amino acids
pH_range = np.linspace(0, 14, 300)
residues = {
    'Asp (pKa 3.9)': 3.9,
    'Glu (pKa 4.1)': 4.1,
    'His (pKa 6.0)': 6.0,
    'Cys (pKa 8.3)': 8.3,
    'Lys (pKa 10.5)': 10.5,
}

fig, ax = plt.subplots(figsize=(9, 5))
for label, pKa in residues.items():
    f_prot = protonation_fraction(pH_range, pKa)
    ax.plot(pH_range, f_prot * 100, label=label)

ax.axvline(7.4, linestyle='--', color='black', label='Physiological pH = 7.4')
ax.set_xlabel('pH'); ax.set_ylabel('% Protonated')
ax.set_title('Amino Acid Protonation States vs. pH')
ax.legend(fontsize=8)
ax.grid(True, alpha=0.3)
plt.tight_layout()

# Henderson-Hasselbalch: fraction protonated at physiological pH
print("Protonation at pH 7.4:")
for label, pKa in residues.items():
    f = protonation_fraction(7.4, pKa) * 100
    print(f"  {label}: {f:.1f}% protonated")

# Buffer pH calculation: phosphate buffer
# Mix H2PO4- (pKa 7.2) and HPO4^2- in 4:6 ratio
ratio = 4 / 6  # [HA]/[A-] = [H2PO4-]/[HPO4^2-]
pH_buffer = 7.2 + np.log10(1/ratio)
print(f"\nPhosphate buffer (40% H2PO4-, 60% HPO4^2-): pH = {pH_buffer:.2f}")
```
