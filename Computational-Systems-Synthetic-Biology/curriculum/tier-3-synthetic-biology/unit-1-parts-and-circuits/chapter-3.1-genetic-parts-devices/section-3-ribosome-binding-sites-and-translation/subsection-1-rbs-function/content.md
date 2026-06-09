# Ribosome Binding Sites: Function and the Determinants of Translation Initiation

Here is a striking fact: two genes driven by identical promoters in the same cell can differ in protein output by over 100-fold, simply because of differences in a short sequence upstream of the start codon. That sequence — the ribosome binding site — controls whether an mRNA is translated efficiently or ignored. You might spend considerable effort optimizing a promoter to drive strong transcription, only to discover that weak translation is the actual bottleneck. Appreciating the RBS as an independent, equally powerful control point is one of the conceptual shifts that separates naive from sophisticated gene expression engineering.

The **ribosome binding site (RBS)**—also called the Shine-Dalgarno sequence in bacteria—is the mRNA element that recruits the small ribosomal subunit to the correct position for translation initiation. While promoters control how much mRNA is made, the RBS controls how efficiently each mRNA molecule is translated. For a given promoter-RBS combination, protein output is approximately proportional to the product of transcription rate and translation initiation rate (TIR):

$$[\text{Protein}]_{ss} \approx \frac{k_{tx} \cdot k_{tl}}{\delta_{mRNA} \cdot \delta_{protein}}$$

where $k_{tx}$ is transcription rate, $k_{tl}$ is translation initiation rate, and $\delta$ terms are degradation rates. This means the RBS is as powerful a determinant of protein levels as the promoter itself.

## The Shine-Dalgarno Sequence

In *E. coli* and most bacteria, the 30S ribosomal subunit is recruited to mRNA through base-pairing between the **Shine-Dalgarno (SD) sequence** in the mRNA and the **anti-Shine-Dalgarno (aSD)** sequence at the 3' end of the 16S rRNA.

The 16S rRNA 3' tail is:
```
...AUUCCUCCACUAG-3'  (3' end of 16S rRNA in E. coli)
```

The complementary SD consensus in mRNA is:
```
5'-AAGG-3' (core) or 5'-AGGAGG-3' (extended Shine-Dalgarno)
```

The SD sequence typically lies **5–10 nucleotides upstream of the AUG start codon**, with 7–8 nt being optimal. The spacing reflects the physical distance between the SD:aSD pairing site and the ribosomal P-site where the initiator tRNA (fMet-tRNA_f^Met) must position over the AUG.

## Key Determinants of Translation Initiation Rate

Translation initiation is the rate-limiting step in most bacterial translation, and its efficiency depends on several features:

### 1. SD Complementarity
The number and quality of base pairs between the SD sequence and the aSD determines binding energy. A perfect 9-nt complement yields maximum initiation; single mismatches reduce TIR by 2–10-fold each.

### 2. SD-AUG Spacing
The spacing between the last nucleotide of the SD and the A of AUG must be precisely 5–10 nt for optimal positioning. Shorter spacing (< 4 nt) creates steric clash; longer spacing (> 12 nt) weakens interaction:

| Spacing (nt) | Relative TIR |
|---|---|
| 4 | ~0.2 |
| 5–6 | ~0.5 |
| 7–8 | ~1.0 (optimal) |
| 9–10 | ~0.7 |
| 12 | ~0.1 |

### 3. Secondary Structure Around the RBS
If the RBS or start codon is sequestered in a stem-loop structure, ribosomes cannot access it. The free energy of the mRNA structure at the translation initiation region (TIR) has a large effect on TIR:

$$\text{TIR} \propto e^{-\Delta G_{SD:aSD}/RT} \cdot e^{+\Delta G_{structure}/RT}$$

A stable hairpin at the RBS ($\Delta G_{structure}$ very negative) suppresses translation. This is the basis of regulatory riboswitches (see section 5.2) and also a frequent source of unintended variation: changing the coding sequence of a gene changes the secondary structure of the 5' end of the mRNA, altering TIR in ways that cannot be predicted from the RBS sequence alone.

### 4. Downstream Sequence Context (Standby Site)
Beyond the immediate RBS, the ~30 nt upstream of the SD can act as a **standby site** for the 30S subunit, increasing the local concentration of ribosomes near the initiation complex. A purine-rich standby region improves TIR.

### 5. Start Codon Identity
- AUG: most efficient (used by ~92% of *E. coli* genes)
- GUG: ~80% efficiency of AUG
- UUG: ~60% efficiency of AUG
- AUU, AUA: rare; very low efficiency

## Measuring Translation Initiation Rate

Direct measurement of TIR requires separating it from transcription. The standard approach:

1. Express a reporter (GFP or luciferase) from an identical promoter with different RBS variants
2. Simultaneously measure mRNA levels (RT-qPCR) and protein levels (fluorescence or luminescence)
3. TIR is proportional to protein output divided by mRNA level:

$$\text{TIR} \propto \frac{[\text{protein}]}{[\text{mRNA}]}$$

This normalization removes transcriptional variation and isolates the translational contribution.

## Worked Example: Comparing Two RBS Variants

Suppose you have two RBS variants upstream of GFP:

**RBS-A**: `AAAGAAGGAGATATACAT` (strong SD: AAGGAG, spacing: 8 nt)
**RBS-B**: `AAAGAAATAGATATACAT` (weak SD: AAATAG, spacing: 8 nt)

Measured in *E. coli* MG1655 at mid-log phase:
- RBS-A: 10,000 fluorescence units/OD; mRNA = 50 au → TIR_A = 200
- RBS-B: 1,000 fluorescence units/OD; mRNA = 48 au → TIR_B = 20.8

RBS-A is ~10× stronger than RBS-B, consistent with the stronger SD complementarity. Note that the mRNA levels are nearly identical (same promoter), confirming the difference is purely translational.

## Why This Matters

The RBS is the second knob—after the promoter—for controlling protein levels, and it is frequently underappreciated. Many failed metabolic engineering experiments can be traced to pathway enzyme expression levels that are either insufficient (too-weak RBS) or so high they create a burden or toxic intermediate accumulation (too-strong RBS). The thermodynamic model underlying the Salis RBS Calculator makes it possible to design RBS sequences for target TIRs with reasonable accuracy, shifting this knob from trial-and-error toward rational design.
