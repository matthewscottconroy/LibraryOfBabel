# CRISPR-Cas in Native Biology

Here is a story that illustrates how science actually works: CRISPR sequences were first noticed in *E. coli* in 1987 by Yoshizumi Ishino, who dismissed them as an interesting but unexplained curiosity. For the next two decades, microbiologists catalogued similar repeat arrays in dozens of species without understanding their function. Then in 2007, Rodolphe Barrangou and colleagues at Danisco (a cheese company — biology happens in unexpected places) showed that *Streptococcus thermophilus* incorporates DNA from phage it has survived into its CRISPR arrays, and that this incorporation provides immunity to future infection. That discovery transformed a genomic oddity into the most important tool in modern genetics. But to truly understand the tools derived from CRISPR — to know why certain target sequences work and others don't, why off-target cutting happens, and how to make the system safer — you need to understand the native biology.

Before CRISPR became a genome editing tool, it was a bacterial immune system. Understanding its native function — how bacteria detect, remember, and destroy invading phage DNA — is essential for appreciating the logic of the tools derived from it, for understanding its limitations and off-target effects, and for recognizing that CRISPR variation across organisms represents a rich toolkit waiting to be engineered.

## What CRISPR Is: Adaptive Immunity in Prokaryotes

**CRISPR** (Clustered Regularly Interspaced Short Palindromic Repeats) arrays were noticed in bacterial genomes as early as 1987 (Ishino et al.) but their function as an immune system was not recognized until 2007 (Barrangou et al., in *Streptococcus thermophilus*).

The CRISPR-Cas system is a form of **adaptive immunity** that provides sequence-specific memory of past infections:

1. **Acquisition (adaptation)**: When a phage infects a bacterium, the Cas1-Cas2 complex cleaves the phage DNA and integrates a short fragment (~30 bp) — a **spacer** — into the CRISPR array at the **leader end** (between repeats). The flanked repeat sequence is called a **direct repeat** (~20–40 bp, partially palindromic, species-specific). The spacer is derived from the protospacer — a target sequence adjacent to a **PAM (protospacer adjacent motif)** in the phage genome.

2. **Biogenesis (processing)**: The CRISPR array is transcribed as a long **pre-crRNA**. This is processed (by Cas6, RNase III, or intrinsic processing depending on the type) into individual **crRNAs**, each containing one spacer sequence flanked by repeat sequences. In Type II systems, a **tracrRNA** (trans-activating crRNA) base-pairs with the repeat portion of pre-crRNA; RNase III cleavage produces mature dual-guide RNA, which is further trimmed.

3. **Interference**: crRNA (or in engineered systems, a single-guide RNA — sgRNA — fusing crRNA + tracrRNA) guides Cas effector nucleases to the complementary target DNA. The effector verifies the PAM sequence (a 2–5 nt sequence adjacent to the protospacer, not in the spacer sequence), unwinds the DNA, checks for crRNA-DNA complementarity, and cleaves.

## Classification of CRISPR-Cas Systems

CRISPR-Cas systems are extraordinarily diverse. The current classification (Makarova et al. 2020) divides them into **2 classes, 6 types, and 33 subtypes**:

**Class 1** (multi-protein effector complexes):
- **Type I**: Most abundant; Cas3 is the effector nuclease/helicase; cascades (Cascade complex) guide Cas3. PAM recognition by Cas8 or Cas5.
- **Type III**: RNA-guided; targets both ssDNA and ssRNA; uses Cas10 complex; activated by RNA:DNA R-loops; unusual in targeting RNA.
- **Type IV**: Poorly characterized; may target plasmids.

**Class 2** (single-protein effectors — the basis of most genome editing tools):
- **Type II**: Cas9 is the effector; cleaves both DNA strands with two catalytic domains (RuvC and HNH); requires PAM (NGG for *SpCas9*). This is the most widely used for genome editing.
- **Type V**: Cas12a/Cpf1 and relatives; single RuvC domain; staggered cuts (5' overhang); self-processing of pre-crRNA. PAM is typically 5'-TTTN for Cas12a.
- **Type VI**: Cas13; targets RNA, not DNA; used for RNA knockdown and diagnostics (SHERLOCK platform).

### PAM Sequences: Molecular Self vs. Non-Self Discrimination

The PAM is a critical self/non-self discrimination element. The spacer sequence is derived from phage DNA; if the CRISPR array also contains sequences matching the spacer, the repeat region would be a valid target — but repeats do not have a PAM, preventing autoimmune targeting of the CRISPR array itself. PAM recognition is thus essential for both immune protection (only PAM-containing sequences in invader DNA are targeted) and for avoiding self-targeting.

Different Cas9 orthologs have different PAMs:
- *SpCas9* (from *S. pyogenes*): NGG
- *SaCas9* (from *S. aureus*): NNGRRT
- *NmCas9* (from *N. meningitidis*): NNNNGATT
- *AsCas12a*: TTTN

## Spacer Acquisition: Primed Adaptation

Initial spacer acquisition (**naive adaptation**) is slow and rare (~$10^{-7}$ events per phage infection per cell). However, when a cell already has a partial-match spacer for a phage, **primed adaptation** dramatically accelerates spacer acquisition from that phage (~$10^{-4}$ events). Primed adaptation allows bacteria to rapidly diversify their spacer collection when facing a previously-encountered phage, analogous to a memory B cell response in adaptive immunity.

Phages escape CRISPR immunity by:
- **Mutating the PAM**: a single PAM mutation eliminates targeting
- **Mutating the protospacer seed region** (1–5 nt adjacent to PAM): critical for crRNA-DNA base pairing
- **Anti-CRISPR proteins (Acrs)**: phage-encoded proteins that block Cas proteins; >45 Acr families identified; work by mimicking DNA (blocking the crRNA-binding cleft), degrading the PAM-interacting domain, or inhibiting other steps

## CRISPR Population Dynamics

The evolutionary arms race between CRISPR and phage creates interesting population dynamics. A bacterium acquires a spacer and becomes immune to phage → phage escapes by PAM/seed mutation → bacterium acquires a new spacer → repeat. In large natural populations, this arms race maintains diverse spacer repertoires and phage sequence diversity.

Mathematically, this can be modeled as a predator-prey system with frequency-dependent selection:

$$\frac{dB}{dt} = rB - \phi V B + \sum_i \alpha_i S_i$$
$$\frac{dV}{dt} = \beta \phi V B (1 - I) - \delta V$$

where $B$ = bacterial density, $V$ = phage density, $I$ = immune fraction (depends on spacer-phage matches), $\phi$ = infection rate, $r$ = bacterial growth rate, $\beta$ = burst size.

## Why This Matters for Computational Biology

The native CRISPR system is the biological context that explains the molecular details of the most important tool in modern genetics. PAM sequence diversity across Cas orthologs defines what genomic targets are accessible — a critical constraint for genome editing target design. Computational tools for guide RNA design (Benchling, CRISPRscan, CRISPOR) compute on-target efficiency scores from sequence features and off-target scores by counting near-matches elsewhere in the genome. Anti-CRISPR proteins are actively explored as safety mechanisms for gene drive systems. Understanding primed adaptation provides intuition for why partial complementarity can still lead to activity — a background needed for assessing off-target risk. The diversity of Type VI (RNA-targeting) CRISPR systems underlies RNA knockdown and RNA base editing approaches that are expanding the toolkit for eukaryotic gene regulation studies.
