# Isotopologue Labeling and Mass Isotopologue Distributions

## Isotopomers vs. Isotopologues

When a ¹³C-labeled glucose molecule enters glycolysis, each subsequent reaction reshuffles which specific carbon positions carry the label. Pyruvate kinase moves carbon differently from phosphoglucose isomerase; the two halves of fructose-1,6-bisphosphate go their separate ways at aldolase. Tracking this positional shuffling with full precision would require distinguishing [1-¹³C]pyruvate from [2-¹³C]pyruvate from [3-¹³C]pyruvate — and that is what the distinction between isotopomers and isotopologues is about.

Two related but distinct concepts are central to ¹³C MFA:

- **Isotopomers**: molecules that differ in the specific *positions* of ¹³C atoms (e.g., [1-¹³C]pyruvate vs. [2-¹³C]pyruvate vs. [3-¹³C]pyruvate). For a molecule with $n$ carbons, there are $2^n$ isotopomers (each carbon can be ¹²C or ¹³C).
- **Isotopologues**: molecules that differ in the *number* of ¹³C atoms but not necessarily position. For pyruvate ($n=3$), there are $n+1 = 4$ isotopologues: M+0, M+1, M+2, M+3.

Mass spectrometry (GC-MS, LC-MS/MS) measures the **mass isotopologue distribution (MID)**: the fractional abundance of each mass isotopologue:

$$\text{MID} = [m_0, m_1, m_2, \ldots, m_n]$$

where $m_k$ is the fraction of molecules with exactly $k$ ¹³C atoms and $\sum_k m_k = 1$.

Mass spectrometry cannot resolve positional isotopomers — it only measures total mass. NMR can resolve positional labeling (¹³C NMR gives site-specific enrichment) but with lower sensitivity.

## Natural Abundance Correction

Every molecule contains some ¹³C from natural abundance (1.1% per carbon) even without tracer. Before using MIDs for flux calculation, measured spectra must be corrected:

$$\mathbf{m}^{\text{corrected}} = \mathbf{C}^{-1} \mathbf{m}^{\text{measured}}$$

where $\mathbf{C}$ is the correction matrix accounting for the binomial distribution of natural ¹³C, ¹⁸O, ²H, ¹⁵N, and ³⁴S contributions. For a fragment ion with $c$ carbons and $o$ oxygens, the correction matrix element $C_{kl}$ is:

$$C_{kl} = \binom{c}{l-k} \left(p_{^{13}C}\right)^{l-k} \left(1-p_{^{13}C}\right)^{c-(l-k)} \times (\text{similar terms for O, H, N, S})$$

where $p_{^{13}C} = 0.011$. The `IsoCor` software automates this correction for GC-MS data.

```python
# Simplified natural abundance correction for a 3-carbon fragment
import numpy as np

def natural_abundance_correction(measured_MID, n_carbons, p13C=0.011):
    """Correct for natural abundance ¹³C contribution."""
    from scipy.special import comb
    n = len(measured_MID)
    C = np.zeros((n, n))
    for i in range(n):
        for j in range(i, n):
            k = j - i  # number of natural 13C atoms
            C[j, i] = comb(n_carbons - i, k, exact=True) * \
                       (p13C**k) * ((1-p13C)**(n_carbons - i - k))
    return np.linalg.solve(C, measured_MID)

# Example: correct measured MID of pyruvate (3C)
measured = np.array([0.62, 0.29, 0.07, 0.02])  # M+0, M+1, M+2, M+3
corrected = natural_abundance_correction(measured, n_carbons=3)
print("Corrected MID:", corrected)
```

## How Metabolic Reactions Mix Labeling Patterns

Each metabolic reaction transforms isotopologue distributions according to the **carbon atom transitions** it performs. Consider alanine aminotransferase:

$$\text{pyruvate} + \text{glutamate} \rightleftharpoons \text{alanine} + \alpha\text{-ketoglutarate}$$

Alanine has the same carbon skeleton as pyruvate (carbons 1, 2, 3 of pyruvate become carbons 1, 2, 3 of alanine). Therefore:

$$\text{MID}_{\text{alanine}} = \text{MID}_{\text{pyruvate}}$$

This makes alanine a convenient proxy for cytoplasmic pyruvate — they are in rapid isotopic equilibrium. In contrast, consider the citrate synthase reaction:

$$\text{acetyl-CoA (2C)} + \text{oxaloacetate (4C)} \rightarrow \text{citrate (6C)}$$

The MID of citrate is the convolution of the MIDs of acetyl-CoA and OAA:

$$m_k^{\text{citrate}} = \sum_{i+j=k} m_i^{\text{acetyl-CoA}} \cdot m_j^{\text{OAA}}$$

This convolution is the fundamental mathematical operation in isotopologue balancing. For a network with many condensation and cleavage reactions, the set of all MID equations becomes large but tractable with the EMU framework.

## Worked Example: Glycolysis vs. PPP with [U-¹³C₆]glucose

Feed cells with 50% [U-¹³C₆]glucose (fully labeled, M+6) and 50% natural ¹²C glucose (M+0).

**Glycolysis only**: Each glucose molecule is either fully labeled (M+6) or unlabeled (M+0). After cleavage at aldolase, each 3-carbon product is either fully labeled (M+3) or unlabeled (M+0). Pyruvate: $[m_0, m_1, m_2, m_3] = [0.5, 0, 0, 0.5]$.

**Pentose phosphate pathway with re-entry**: The carbon rearrangements in transketolase and transaldolase mix labeled and unlabeled carbons, producing M+1 and M+2 species in the re-entering fragments. Pyruvate: $[m_0, m_1, m_2, m_3] \approx [0.5, 0.05, 0.1, 0.35]$ (illustrative values).

The difference in M+1 and M+2 fractions is diagnostic of PPP activity. A flux of 25% through PPP would produce an intermediate pattern. By fitting the full model to measured MIDs of pyruvate, serine, alanine, and TCA intermediates, the PPP contribution can be resolved to ±3% accuracy.

## Fragment Ions and Partial Carbon Coverage

GC-MS measures fragment ions — portions of the molecule after electron ionization fragmentation. Different fragments report on different carbons of the same molecule:

**Glutamate (5 carbons) fragments:**
- Fragment at m/z 198: carbons 1-2 (2-carbon fragment → M+0, M+1, M+2)
- Fragment at m/z 152: carbons 2-5 (4-carbon fragment → M+0 through M+4)

Multiple fragments from the same metabolite provide complementary information about labeling at different positions, improving the resolution of flux estimates.

## Why This Matters

Understanding how isotopologue distributions encode and propagate through metabolic reactions is prerequisite to interpreting ¹³C MFA results. The key insight is that MIDs are not arbitrary numbers — they are constrained by carbon atom transitions that are stoichiometrically determined by the reaction network. This constraint is what allows flux calculation: only specific combinations of fluxes can produce the observed labeling patterns.
