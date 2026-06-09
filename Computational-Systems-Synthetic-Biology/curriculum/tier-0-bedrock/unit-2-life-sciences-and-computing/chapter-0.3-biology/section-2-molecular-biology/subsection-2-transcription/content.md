# Transcription

If DNA is the archive and proteins are the workers, then transcription is the act of reading the archive and generating instructions. It is the first step in gene expression and — crucially for us — the primary step at which regulation is exerted. When a cell decides to change its behavior in response to a new nutrient, a stress signal, or a developmental cue, it does so mostly by changing which genes are transcribed and at what rate. Understanding transcription quantitatively — promoter recognition, elongation kinetics, termination signals — is necessary for modeling gene circuits, interpreting RNA-seq data, and engineering synthetic promoters.

Transcription is the synthesis of RNA from a DNA template. It is the first and most heavily regulated step in gene expression.

## RNA Polymerase: The Core Machine

**RNA polymerase (RNAP)** synthesizes RNA in the 5'→3' direction using the 3'→5' DNA strand as template. Unlike DNA polymerase, RNAP:
- Does not require a primer — it initiates synthesis de novo
- Has no proofreading 3'→5' exonuclease; error rate is $\sim 10^{-5}$ per base (acceptable because RNA is transient and many copies are made)
- Uses ribonucleoside triphosphates (NTPs) rather than dNTPs

In *E. coli*, RNAP is a ~480 kDa multi-subunit complex: $\alpha_2 \beta \beta' \omega$ as the core enzyme. The core is catalytically active but cannot bind promoters without a **sigma (σ) factor**.

## Bacterial Transcription: Sigma Factors and Promoter Recognition

The **σ factor** dissociates from the core enzyme after promoter escape, making it available to assist another RNAP. This allows a single limiting pool of σ factors to program large numbers of RNAP molecules.

**σ70** is the primary sigma factor in *E. coli* (the housekeeping sigma). It recognizes two conserved elements:

- **-35 box**: consensus sequence `TTGACA`, centered 35 bp upstream of the transcription start site (+1)
- **-10 box**: consensus sequence `TATAAT`, centered 10 bp upstream of +1 (the "Pribnow box")

The spacing between -35 and -10 (optimally 17 bp) is critical; it positions the two elements on the same face of the DNA helix for simultaneous contact with σ70 subdomains.

**Promoter strength** correlates with proximity to consensus:
- Strong promoters (e.g., *rrnB* P1): near-consensus -35 and -10; transcription rate can reach ~80 transcripts/min
- Weak promoters: one or both boxes deviate from consensus; initiation rates of 1 transcript/min or less

Alternative sigma factors enable rapid global reprogramming of transcription:

| Sigma factor | Regulon | Signal |
|---|---|---|
| σ70 (σD) | Housekeeping genes | None (constitutive) |
| σ32 (σH) | Heat shock genes | High temperature, unfolded proteins |
| σ54 (σN) | Nitrogen assimilation | Nitrogen starvation |
| σ38 (σS) | Stationary phase | Starvation, stress |
| σ28 (σF) | Flagellar genes | Developmental signal |

## Transcription Cycle: Initiation, Elongation, Termination

**Initiation** proceeds through defined intermediates:
1. **Closed complex (RPc)**: RNAP-σ binds duplex DNA at the promoter; reversible
2. **Open complex (RPo)**: DNA is unwound ~13 bp around the -10 box; slow, rate-limiting step; now irreversible
3. **Abortive initiation**: short RNAs (2–9 nt) are synthesized and released repeatedly before promoter clearance
4. **Promoter escape**: after synthesizing ~10 nt, σ is released and the elongation complex (EC) becomes fully processive

**Elongation**: The *E. coli* RNAP elongates at a mean rate of **~40–80 nt/s** under optimal conditions, with significant pausing at specific sequences. The transcription bubble (unwound region) is ~12 bp; ~8 bp of RNA:DNA hybrid exists behind the active site.

RNAP can pause, arrest, or backtrack:
- **Pause sites** are sequence-encoded; the RNA folds into a hairpin that contacts RNAP, causing ~1–10 s pauses
- **Arrested complexes** require **GreA/GreB** (bacterial) cleavage factors to rescue — these cleave the 3' end of the backtracked RNA to regenerate an active 3'-OH

**Termination** in bacteria occurs by two mechanisms:

1. **Intrinsic (Rho-independent) termination**: The nascent RNA folds into a G+C-rich hairpin (ΔG $\approx$ -10 to -20 kcal/mol) followed by a U-rich tract (~8 nt). The hairpin destabilizes the RNA:DNA hybrid (since rU:dA pairs are weak), causing RNAP dissociation.

2. **Rho-dependent termination**: The **Rho** factor (a hexameric RNA-dependent ATPase) binds a C-rich, unstructured region of the nascent RNA (**rut site**) and translocates in the 5'→3' direction to catch up to a paused RNAP, inducing its release. Rho-dependent terminators control ~50% of *E. coli* transcription units.

## Eukaryotic Transcription: Three Polymerases and GTFs

Eukaryotes have three nuclear RNA polymerases with non-overlapping roles:

| Polymerase | Product | Location |
|---|---|---|
| RNA Pol I | rRNA (28S, 18S, 5.8S) | Nucleolus |
| RNA Pol II | mRNA, most lncRNA, snRNA | Nucleoplasm |
| RNA Pol III | tRNA, 5S rRNA, small RNAs | Nucleoplasm |

**RNA Pol II** requires **general transcription factors (GTFs)** — TFIIA, B, D, E, F, H — to assemble the **pre-initiation complex (PIC)** at the promoter. The key elements recognized are:
- **TATA box**: `TATAAA`, ~25 bp upstream of +1; recognized by TBP (TATA-binding protein), a subunit of TFIID
- **Initiator (Inr)**: spans the +1 site
- **Downstream promoter element (DPE)**: ~+30, particularly important in Drosophila

The **C-terminal domain (CTD)** of the Pol II large subunit has 52 heptapeptide repeats (YSPTSPS) in humans. Phosphorylation state of Ser2/Ser5/Ser7 coordinates the transcription cycle with co-transcriptional events:
- Ser5-P: marks early elongation; recruits 5' capping machinery
- Ser2-P: marks productive elongation; recruits splicing and 3' processing factors

## A Worked Example: Estimating Transcript Number Per Cell

Given: Strong *E. coli* promoter, transcription rate $k_{tx} = 1\ \text{transcript/min}$, mRNA half-life $t_{1/2} = 2\ \text{min}$, degradation rate $\gamma = \ln 2 / t_{1/2} = 0.347\ \text{min}^{-1}$.

At steady state, production = degradation:

$$k_{tx} = \gamma \cdot [\text{mRNA}]_{ss}$$

$$[\text{mRNA}]_{ss} = \frac{k_{tx}}{\gamma} = \frac{1\ \text{min}^{-1}}{0.347\ \text{min}^{-1}} \approx 2.9 \text{ molecules per cell}$$

This is why single-cell measurements reveal substantial cell-to-cell variability in mRNA levels — with only 2–3 molecules per cell, Poisson noise alone gives a coefficient of variation $CV = 1/\sqrt{n} \approx 0.58$, i.e., 58% variability. The takeaway is profound: gene expression noise is not a defect or an artifact, but an inescapable consequence of the small copy numbers involved. Any quantitative model of gene expression that ignores stochasticity is missing something real and biologically important.

## Why This Matters for Computational Biology

Transcription rate is the primary control point for gene expression and therefore the primary target of synthetic biology design. Promoter strength is tunable by mutation, and libraries of promoters with quantified activities (in promoter units) are available for circuit design. Sigma factor competition creates a global coupling between circuits — high expression of one gene can sequester RNAP, reducing expression of others, an effect called **resource competition** or **cellular burden**. Models of gene circuits must account for finite RNAP pools, elongation rate-limited transcription time, and the kinetics of promoter escape. RNA-seq measures the steady-state mRNA level: interpreting it requires knowing degradation rates as well as synthesis rates.
