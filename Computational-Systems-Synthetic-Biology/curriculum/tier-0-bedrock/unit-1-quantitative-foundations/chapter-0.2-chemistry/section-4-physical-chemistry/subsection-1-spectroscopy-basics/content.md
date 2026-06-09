# Spectroscopy Basics

In 1944, Erwin Schrödinger published *What is Life?*, a book that helped catalyze the molecular biology revolution. But long before physicists turned their attention to biology, chemists had developed an extraordinary toolkit for interrogating matter without touching it: spectroscopy. Shine light of the right frequency on a molecule, and it will absorb, scatter, or emit in a way that is a precise fingerprint of its chemical structure, concentration, and physical state.

Today, every quantitative experiment in biochemistry relies on spectroscopy in some form. You cannot measure protein concentration without Beer-Lambert law; you cannot do fluorescence microscopy without understanding Stokes shifts; you cannot interpret NMR metabolomics data without knowing what chemical shifts mean. And perhaps most relevantly for computational biology: every data point from a spectroscopic experiment carries implicit assumptions about physics that you must understand to know whether your numbers are valid.

Spectroscopic methods are the experimental foundation of biochemistry — they allow us to quantify concentrations, characterize molecular structure, measure binding affinities, and track conformational changes without destructive interference. Understanding the physical principles underlying spectroscopy enables you to interpret data correctly, recognize artifacts, and choose the right technique for a given problem.

## UV-Vis Absorbance Spectroscopy

Molecules absorb light when a photon's energy matches the energy gap between electronic states. UV-visible spectroscopy (200–700 nm) measures this absorbance.

**Beer-Lambert Law:** The absorbance $A$ of a solution is proportional to the concentration $c$ and path length $l$:

$$A = \varepsilon \cdot c \cdot l$$

where $\varepsilon$ is the **molar extinction coefficient** (units: M$^{-1}$ cm$^{-1}$, also called molar absorptivity). $A$ is dimensionless; $A = -\log_{10}(I/I_0)$ where $I_0$ is incident and $I$ is transmitted intensity.

**Key chromophores in biochemistry:**
- **Nucleic acids:** $A_{260}$ — purine and pyrimidine bases absorb strongly at 260 nm. $\varepsilon_{260} \approx 10,000$ M$^{-1}$cm$^{-1}$ per nucleotide. Rule of thumb: $A_{260} = 1.0$ corresponds to ~50 µg/mL double-stranded DNA.
- **Proteins:** $A_{280}$ — Trp ($\varepsilon \approx 5500$), Tyr ($\varepsilon \approx 1490$), and disulfide bonds ($\varepsilon \approx 125$) contribute. $\varepsilon_{280}$ calculated from sequence using the Pace equation.
- **NADH:** $A_{340}$, $\varepsilon = 6220$ M$^{-1}$cm$^{-1}$ — used to monitor enzyme reactions that produce or consume NADH
- **Heme proteins:** Soret band at ~415 nm ($\varepsilon > 100,000$) — extremely sensitive

**$A_{260}/A_{280}$ ratio for nucleic acid purity:** Pure DNA: ~1.8; pure RNA: ~2.0. Lower values indicate protein contamination; values above 2.0 indicate RNA contamination in a DNA prep. $A_{260}/A_{230} \approx 2.0$ for pure samples (contamination by phenol, guanidinium gives abnormally low $A_{260}/A_{230}$).

**Worked example — enzyme assay:** Lactate dehydrogenase activity is measured by the rate of NADH consumption ($A_{340}$ decrease). From Beer-Lambert: $\Delta[\text{NADH}] = \Delta A_{340} / (\varepsilon \cdot l) = 0.1 / (6220 \times 0.001\ \text{L/cm} \times 1\ \text{cm}) = 0.016\ \text{mM}$ consumed per unit $A_{340}$ change per mL. Enzyme activity in nmol/min is computed from the slope of $A_{340}$ vs. time.

## Fluorescence Spectroscopy

Fluorescence occurs when an excited molecule emits a photon as it relaxes from $S_1$ to $S_0$. The emitted photon has lower energy (longer wavelength) than the absorbed photon — this energy loss is the **Stokes shift**, enabling separation of excitation and emission.

**Intrinsic fluorophores:**
- **Tryptophan (Trp):** Excited at 280–300 nm; emission at 330–350 nm in nonpolar environments, shifting to 350–360 nm in polar environments. Used to monitor protein folding/unfolding.
- **NADH:** Excited at 340 nm; emission at 460 nm. Allows monitoring metabolic state.
- **Flavins (FAD, FMN):** Excited at 450 nm; emission at 525 nm.

**Extrinsic fluorophores:** Fluorescent dyes attached to proteins or nucleic acids: GFP family, FITC, Cy3/Cy5, SYBR Green (DNA intercalator).

**Förster Resonance Energy Transfer (FRET):** Energy transfer from a donor fluorophore to an acceptor when they are within ~1–10 nm. FRET efficiency:

$$E = \frac{1}{1 + (r/R_0)^6}$$

where $r$ is the donor-acceptor distance and $R_0$ is the Förster radius (typically 2–7 nm, where efficiency = 50%). FRET is a **molecular ruler** — it reports distances at the nanometer scale, far below the diffraction limit of light microscopy. Applications: protein conformational changes, protein-protein interactions, single-molecule dynamics.

The steep sixth-power dependence of FRET efficiency on distance is both its strength and its limitation. It makes FRET exquisitely sensitive to small distance changes in the 2–8 nm range — perfectly suited for detecting whether a protein has opened or closed, or whether two proteins have come together. But it also means that FRET reports almost nothing outside this window. Knowing this, you can immediately assess whether a FRET experiment is designed to measure what its designers claim.

## NMR Spectroscopy (Essentials)

**Nuclear Magnetic Resonance (NMR)** detects the resonant absorption of radiofrequency radiation by atomic nuclei in a magnetic field. Biologically relevant nuclei: $^1$H (proton, most sensitive), $^{13}$C, $^{15}$N, $^{31}$P.

**Chemical shift** ($\delta$, in ppm): The resonance frequency of a nucleus, shifted from a reference by the local electronic environment. Electron-withdrawing groups deshield nuclei (shift downfield); electron-donating groups shield them (shift upfield). In $^1$H NMR: aliphatic H at 0–3 ppm, amide H at 6–9 ppm, aromatic H at 7–9 ppm. Chemical shifts are sensitive to protein folding state — NMR "fingerprints" (2D $^1$H-$^{15}$N HSQC) change when a protein binds a ligand.

**J-coupling:** Spin-spin coupling through bonds; creates multiplet patterns that reveal connectivity (adjacent protons) and can measure torsion angles.

**Applications in computational biology:**
- **Metabolomics:** $^1$H NMR of cell extracts or biofluids provides metabolite fingerprints without prior knowledge of what's present — complementary to mass spectrometry
- **Protein structure determination:** Multidimensional NMR ($^{13}$C/$^{15}$N labeled proteins) determines solution structures of proteins $< 40$ kDa
- **Fragment-based drug discovery:** NMR detects weak binding ($K_d$ ~mM) that other methods miss

## Why This Matters for Computational Biology

Spectroscopy provides the experimental data that feed computational analyses. A_{260} measurements calibrate RNA-seq library preparations. FRET measurements constrain protein conformational ensemble models. NMR chemical shifts validate molecular dynamics simulation force fields and structures. Fluorescence microscopy images, analyzed by computational methods, track protein localization and dynamics in living cells. Understanding these methods is necessary for interpreting the raw data, recognizing quality control failures, and understanding what physical quantity each data point actually measures.

```python
import numpy as np
import matplotlib.pyplot as plt

# Beer-Lambert law: calculate concentration from absorbance
def concentration_from_abs(absorbance, epsilon, path_length=1.0):
    """Calculate concentration (M) from absorbance, extinction coeff (M^-1 cm^-1), path (cm)."""
    return absorbance / (epsilon * path_length)

# FRET efficiency as function of distance
def fret_efficiency(r, R0):
    return 1 / (1 + (r/R0)**6)

# Example calculations
print("Beer-Lambert examples:")
print(f"  DNA at A260=0.8: {concentration_from_abs(0.8, 10000)*1e6:.1f} µM nucleotides")
print(f"  NADH at A340=0.3: {concentration_from_abs(0.3, 6220)*1e6:.1f} µM NADH")

# FRET ruler
R0 = 5.0  # nm (typical Cy3-Cy5 Forster radius)
distances = np.linspace(1, 15, 200)
E = fret_efficiency(distances, R0)

fig, ax = plt.subplots(figsize=(7, 4))
ax.plot(distances, E * 100)
ax.axhline(50, linestyle='--', color='red', label=f'50% efficiency at R₀ = {R0} nm')
ax.axvline(R0, linestyle='--', color='red')
ax.set_xlabel('Donor-Acceptor Distance (nm)')
ax.set_ylabel('FRET Efficiency (%)')
ax.set_title('FRET as a Molecular Ruler')
ax.legend()
plt.tight_layout()
print(f"\nFRET efficiency at distances:")
for d in [2, 5, 7, 10]:
    print(f"  r = {d} nm: E = {fret_efficiency(d, R0)*100:.1f}%")
```
