# Eukaryotic Promoters: Architecture, Diversity, and Engineering

When synthetic biologists first began engineering gene expression in bacteria, they could specify a promoter with two six-base words and a spacer. Moving to eukaryotic cells is a different world entirely. A human gene promoter may integrate regulatory inputs from dozens of transcription factors, respond to signals arriving from enhancers a million base pairs away, and be silenced overnight by the spreading of chromatin marks from a neighboring genomic region. The additional complexity is not arbitrary: it reflects hundreds of millions of years of evolution toward fine-grained developmental control. But for an engineer trying to express a therapeutic protein or build a logic gate in a mammalian cell, that complexity means that intuitions developed in bacteria will routinely fail you. Understanding the architecture of eukaryotic promoters is not optional — it is the prerequisite for rational gene expression design outside the microbial world.

Eukaryotic transcription is vastly more complex than bacterial transcription. Where a bacterial promoter can be functionally characterized by two hexamers and a spacer, a eukaryotic promoter may integrate signals from dozens of transcription factors acting over distances of kilobases.

## Core Promoter Architecture

The **core promoter** is the minimal DNA sequence sufficient to direct accurate transcription initiation by RNA Polymerase II (Pol II). It encompasses roughly −40 to +40 relative to the transcription start site (TSS) and contains a subset of the following elements:

- **TATA box**: consensus TATAAA, centered at ~−30; present in ~10–20% of human promoters; binds TATA-binding protein (TBP), the foundation of the Pol II preinitiation complex (PIC)
- **Initiator (Inr)**: consensus YYANWYY (Y = pyrimidine, W = A/T, N = any) spanning the TSS; recognized by TAF1 and TAF2 subunits of TFIID
- **Downstream Promoter Element (DPE)**: at +28 to +34; common in *Drosophila*; compensates when TATA is absent
- **BRE (TFIIB Recognition Element)**: flanks the TATA box; fine-tunes TFIIB positioning

Most human promoters lack a TATA box and instead rely on Inr and DPE, or on **CpG islands**—regions of high CpG dinucleotide density that nucleate transcription factor binding and chromatin remodeling.

## Enhancers and Distal Regulatory Elements

Unlike bacteria, eukaryotic promoters are rarely self-contained. **Enhancers** are cis-regulatory elements that:
- Can act at distances of 1 Mb or more from the promoter
- Function in either orientation
- Loop to contact the promoter through 3D chromatin organization (detected by Hi-C, ChIA-PET)
- Integrate signals from multiple transcription factors via their enhanceosome-like architecture

For synthetic biology, this means that heterologous transgenes may behave unpredictably depending on their genomic insertion site—adjacent chromatin can activate or silence expression through enhancer interference or heterochromatin spreading.

## Commonly Used Constitutive Promoters for Synthetic Biology

### Yeast (*Saccharomyces cerevisiae*)

| Promoter | Strength | Notes |
|---|---|---|
| TEF1 (pTEF1) | Strong | Translation elongation factor 1α; very commonly used |
| GPD (pGPD/pTDH3) | Very strong | Glyceraldehyde-3-phosphate dehydrogenase; highest in glucose |
| ADH1 (pADH1) | Moderate | Alcohol dehydrogenase; lower than GPD |
| CYC1 | Weak | Cytochrome c; useful as minimal promoter for inducible systems |

### Mammalian Cells

| Promoter | Strength | Notes |
|---|---|---|
| CMV (Cytomegalovirus) | Very strong | Widely used; can silence in stem cells and some primary cells |
| EF1α | Strong | Human elongation factor 1α; active in most cell types; more stable than CMV |
| PGK | Moderate | Phosphoglycerate kinase; ubiquitous; often used for selection markers |
| UBC | Moderate | Ubiquitin C; stable, broad expression |
| CAG | Very strong | CMV enhancer + chicken β-actin promoter + rabbit β-globin splice acceptor; very stable |

The **CAG promoter** is particularly notable for its resistance to silencing. Many early viral promoters (CMV, RSV) are silenced in pluripotent stem cells and during development, but CAG maintains activity through the CpG island-rich β-actin core.

## Inducible Systems in Mammalian Cells

### Tet-On / Tet-Off

The **Tet system** is the workhorse of mammalian inducible expression. It uses a chimeric protein—the reverse tetracycline transactivator (rtTA) or tetracycline transactivator (tTA)—composed of TetR fused to the VP16 activation domain.

- **Tet-Off (tTA)**: activates a TRE (Tet-responsive element = 7x tetO repeats) promoter in the absence of doxycycline (Dox); Dox addition turns expression OFF
- **Tet-On (rtTA)**: activates TRE only when Dox is present; the default state is OFF

Tet-On is generally preferred because the ON state requires actively adding Dox (easier to control), while Tet-Off requires maintaining Dox to suppress expression—leaky at trace Dox concentrations.

Third-generation rtTA variants (rtTA-M2, rtTA3) have reduced Dox requirements and lower leakiness.

### Light-Inducible Systems

Optogenetic tools allow spatially and temporally precise control of gene expression using light:

- **CRY2-CIB1 (blue light, 450 nm)**: CRY2 and CIB1 heterodimerize within seconds upon blue light illumination. Fusion of CIB1 to a DNA-binding domain and CRY2 to an activation domain creates a light-dependent transcription factor.
- **LightOn (Vivid-based, 450 nm)**: *Neurospora crassa* Vivid protein homodimerizes under blue light; used to reconstitute split transcription factors.
- **PhyB-PIF (660/730 nm)**: red/far-red switchable; PhyB-PIF dimerize under red light and dissociate under far-red light; allows reversible control.

Light-inducible systems offer millisecond-to-second temporal resolution and micrometer spatial resolution that no chemical system can match.

### Chemical Dimerization Systems

- **Rapamycin-induced FKBP-FRB dimerization**: rapamycin bridges FKBP and FRB domains; fusing each domain to a split transcription factor half reconstitutes activity only with rapamycin
- **Gibberellin (GA)-induced GAI-GID1 dimerization**: plant hormone; orthogonal to mammalian systems

## Synthetic Minimal Promoters and Modular Design

For many applications, full-length natural promoters carry baggage: endogenous regulatory inputs, variable strength across cell types, complex chromatin interactions. **Synthetic minimal promoters** strip out all regulatory elements except the core, then add back only the desired response elements.

A modular design workflow:

```
[n × TF binding sites] -- [Spacer] -- [Minimal core promoter] -- [TSS] -- [Gene]
```

Example: 5× GATA-binding sites upstream of a minimal CYC1 core drives expression specifically in GATA-factor-expressing erythroid cells, with minimal activity elsewhere.

This logic underpins **cell-type-specific promoters** used in gene therapy: AAV vectors carrying therapeutic genes under synthetic promoters that respond only to transcription factors abundant in the target cell type (e.g., synapsin promoter for neurons, liver-specific APOA1 enhancer).

## Why This Matters

Eukaryotic promoters control not just how much RNA is made but where, when, and in response to what signals. For metabolic engineering in yeast, the choice between GPD and TEF1 promoters can determine whether your pathway enzyme is expressed at levels that saturate cofactors or create toxic intermediates. For mammalian cell engineering, promoter silencing in stem cells can render a therapeutic construct inactive by the time it reaches its target. Understanding the architectural principles of eukaryotic promoters—and the engineering tools available to tune them—is foundational for any practitioner working outside bacteria.
