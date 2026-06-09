# Major Signaling Pathways: Reference Guide

## Overview

The signaling pathways catalogued below are not independent entities — they are overlapping modules in a deeply interconnected network. A cell stimulated with EGF activates not just the MAPK pathway but PI3K, PLCγ, and Src simultaneously; the downstream responses are determined by the integrated pattern of all of these, modulated by feedback between them. The descriptions below abstract each pathway into a clean linear cascade because that abstraction is genuinely useful — it is how drugs are designed and mutations are annotated. But always remember that you are reading a simplification. The purpose of this reference section is to give you the vocabulary and canonical activation logic for each pathway, so that when you encounter them in the research literature, the framework is already in place.

For each pathway, the canonical activation sequence, key regulatory points, and therapeutic relevance are summarized. This is intended as a reference companion to the more detailed mechanistic discussions in earlier sections.

## MAPK/ERK Pathway

**Canonical cascade**: Growth factor → RTK (EGFR, PDGFR, FGFR, etc.) → Grb2:SOS → RAS-GTP → RAF (c-Raf, B-Raf) → MEK1/2 → ERK1/2

**Key regulatory points**:
- RAS GTPase (rate-limiting off-switch): accelerated by GAPs (NF1, RASA1)
- Scaffold protein KSR1/2: organizes RAF:MEK:ERK complex
- Negative feedback: ERK→SOS (inhibitory phosphorylation), ERK→DUSP induction
- Positive feedback: ERK→RAS-GEFs at high concentration

**Major outputs**: proliferation, survival, differentiation (context-dependent, depends on duration)

**Oncogenic mutations**: KRAS G12D/G12V (25% all cancers), BRAF V600E (melanoma 50%, thyroid 40%)

**Approved drugs**: BRAF inhibitors (vemurafenib, dabrafenib), MEK inhibitors (trametinib, cobimetinib), ERK inhibitors (ulixertinib, MK-8353 in trials)

---

## PI3K/AKT/mTOR Pathway

**Canonical cascade**: RTK → IRS-1 → PI3K (p85/p110) → PIP3 → PDK1 + AKT → TSC1/2 → Rheb-GTP → mTORC1 → S6K, 4E-BP1

**Key regulatory points**:
- PTEN: opposes PI3K by dephosphorylating PIP3 (major tumor suppressor)
- AKT T308 (PDK1) and S473 (mTORC2): both required for full activation
- mTORC1 inputs: AKT, AMPK, amino acids (Ragulator), oxygen (REDD1)
- S6K negative feedback: S6K→IRS-1 inhibitory phosphorylation

**Major outputs**: cell growth, protein synthesis, glucose metabolism, survival (BAD phosphorylation), cell cycle (FOXO inactivation)

**Disease relevance**: PIK3CA mutations (breast, endometrial cancer), PTEN loss (glioblastoma, prostate cancer)

**Approved drugs**: mTOR inhibitors (rapamycin analogs: everolimus, temsirolimus), PI3Kδ inhibitor (idelalisib, for CLL), pan-PI3K inhibitors (buparlisib in trials), AKT inhibitors (capivasertib approved 2023)

---

## Wnt/β-catenin Pathway

**Without Wnt**: GSK-3β + CK1 phosphorylate β-catenin → APC:Axin destruction complex → β-TrCP E3 ubiquitin ligase → β-catenin proteasomal degradation

**With Wnt**: Wnt ligand binds Frizzled + LRP5/6 coreceptor → Dishevelled activates → inhibits GSK-3β (via phosphorylation of LRP5/6) → β-catenin escapes degradation → accumulates in nucleus → TCF/LEF + β-catenin → target gene transcription (Axin2, Cyclin D1, c-Myc, CD44)

**Regulation**: DKK1 (secreted antagonist of LRP5/6), Wnt inhibitory factor (WIF), RSPO1-4 (amplify Wnt by inhibiting RNF43 receptor ubiquitination)

**Oncogenic mutations**: APC truncation (colon cancer, >80%), β-catenin activating mutations (CTNNB1; hepatocellular carcinoma), RSPO3 fusions

**Therapeutic attempts**: Porcupine inhibitors (prevent Wnt secretion: WNT974), tankyrase inhibitors (stabilize Axin: research stage)

---

## Notch Pathway

**Mechanism**: Notch receptor (1-4) on signal-receiving cell contacts Delta-like (DLL1, 3, 4) or Jagged (JAG1, 2) ligand on signal-sending cell → proteolytic cleavage of Notch by ADAM metalloprotease (S2 cut) → γ-secretase complex (presenilin) (S3 cut) → NICD (Notch Intracellular Domain) released → translocates to nucleus → displaces HDAC corepressor from RBPJ → activates Hes1, Hey1, and other transcription factor targets

**Biological functions**: lateral inhibition (ensures single cell fate in equivalent cell population), boundary formation, stem cell maintenance, T cell lineage commitment

**Disease**: Notch1 activating mutations (T-ALL, ~50%), NOTCH1 or NOTCH2 mutations (diffuse large B cell lymphoma), Jagged1 mutations (Alagille syndrome, bile duct malformations)

**Therapeutic**: γ-secretase inhibitors (GSIs) block NICD release; clinical success limited by gut toxicity (goblet cell hyperplasia from Notch inhibition in intestine)

---

## Hedgehog Pathway

**Without Hh**: Patched (PTCH1) receptor inhibits Smoothened (SMO) → SMO inactive → SUFU sequesters GLI transcription factors → GLI3 proteolytic processing to repressor form → target genes suppressed

**With Sonic Hedgehog (SHH)**: SHH binds PTCH1 → relieves SMO inhibition → SMO activates GLI2 activator form → Hh target genes (PTCH1, Gli1, Snail, Cyclin D1)

**Disease**: Gorlin syndrome (PTCH1 germ-line loss → basal cell carcinomas, medulloblastoma), SMO mutations (sporadic BCC)

**Drugs**: SMO inhibitors: vismodegib (approved for BCC and medulloblastoma), sonidegib

---

## JAK/STAT Pathway

**Canonical**: Cytokine → cytokine receptor (JAK1/2/3-associated) → receptor dimerization → JAK transautophosphorylation → receptor phosphorylation → STAT (1-6) recruitment via SH2 domain → JAK phosphorylates STAT → STAT dimerizes → nuclear translocation → GAS element target genes

**Pathway outputs by STAT**:
- STAT1: interferon response, antiviral genes
- STAT3: IL-6, oncogenesis (anti-apoptotic, proliferative targets)
- STAT5: hematopoiesis, prolactin signaling
- STAT6: IL-4/IL-13, Th2 differentiation, IgE class switch

**Negative regulation**: SOCS proteins (STAT targets that inhibit JAKs → negative feedback), PIAS (protein inhibitors of activated STATs → nuclear SUMO-mediated inhibition)

**Oncogenic mutations**: JAK2 V617F (polycythemia vera, 97%), JAK1/3 mutations (T-ALL), STAT3/5 activating mutations (large granular lymphocyte leukemia)

**Drugs**: Ruxolitinib (JAK1/2, approved for myelofibrosis, PV, GvHD), tofacitinib (JAK1/3, approved for RA, IBD), baricitinib (JAK1/2, RA, COVID-19)

---

## NF-κB Pathway

**Classical activation**: TNF → TNFR1 → TRADD/RIP1 → IKK complex (IKKα+IKKβ+NEMO) → IκBα phosphorylation (S32/S36) → β-TrCP ubiquitination → proteasomal degradation → NF-κB (p65/p50) released → nuclear translocation → NF-κB target genes (IL-6, IL-8, BCL-2, XIAP, Cyclin D1)

**Negative feedback**: IκBα is itself an NF-κB target gene → delayed negative feedback → oscillatory NF-κB dynamics

**Alternative pathway**: RANKL, TWEAK, CD40L → NIK kinase → IKKα homodimer → p100 processing to p52 → RelB/p52 → target genes (CXCL13, BAFF, lymphoid organogenesis)

**Disease**: Multiple myeloma (NF-κB constitutive activation via TRAF3 deletion, NIK mutation), diffuse large B-cell lymphoma (ABC subtype, MYD88 L265P → constitutive NF-κB)

**Drugs**: Bortezomib (proteasome inhibitor → prevents IκBα degradation; multiple myeloma); IKKβ inhibitors in clinical development

---

## cAMP/PKA Pathway

**Activation**: β-agonist or other Gαs-coupled GPCR agonist → adenylyl cyclase (AC) activation → ATP → cAMP → PKA R-subunit dissociation → free PKA C-subunits → phosphorylate CREB (transcription), phospholamban (cardiac), glycogen phosphorylase kinase, HSL (lipolysis)

**Termination**: PDE (cyclic nucleotide phosphodiesterase, types 1–11) hydrolyzes cAMP → AMP

**AKAP proteins (A-kinase anchoring proteins)**: localize PKA to specific organelles (mitochondria, ER, plasma membrane) → spatial specificity

**Disease relevance**: McCune-Albright syndrome (GNAS activating mutation → constitutive cAMP → fibrous dysplasia); carney complex (PRKAR1A mutation → constitutive PKA)

---

## Why This Matters

Each of these pathways has been targeted therapeutically, and the list of approved drugs continues to grow. A systems biology perspective recognizes that these "pathways" are abstractions — they are not independent but form an interconnected network with extensive crosstalk, feedback, and context-dependence. The references in this table are starting points for understanding the major regulatory modules; the systems-level picture emerges only when multiple pathways are considered together.
