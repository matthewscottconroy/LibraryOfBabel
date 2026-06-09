# Delivery Methods: RNP and Other Strategies

In November 2021, a 58-year-old man with transthyretin amyloidosis — a disease caused by misfolded liver protein accumulating in the heart and nerves — received an infusion of lipid nanoparticles carrying Cas9 mRNA and a guide RNA targeting the TTR gene. Six months later, his transthyretin levels had dropped by 87%. This was the first reported in vivo CRISPR editing in a human patient, and it worked not because the guide RNA was particularly clever, but because the delivery system was. The lipid nanoparticle dissolved in the bloodstream, found the liver, entered the hepatocytes, released its cargo, and cleared within days. The biology of delivery — how you get the molecular machinery from a vial into the right cells in the right tissue without triggering an immune response or leaving a trail of off-target edits — is often what separates a promising CRISPR application from one that actually reaches patients.

Even a perfectly designed CRISPR system is useless if it cannot enter the target cell. Delivery is often the limiting factor in genome editing, particularly for in vivo therapeutic applications where the cell type, tissue accessibility, cargo size, and immune context all constrain the available options. This section surveys delivery strategies across contexts, emphasizing the trade-offs that determine which approach is appropriate.

## The Delivery Problem

A functional CRISPR system requires delivery of:
- The Cas9 protein (or its encoding nucleic acid)
- The sgRNA (or its encoding DNA)
- Optionally: a donor template for HDR, or a pegRNA for prime editing

These components can be packaged and delivered in three formats:
1. **DNA** (plasmid): persistent expression, higher risk of genomic integration
2. **RNA** (mRNA + synthetic sgRNA): transient expression, no integration risk
3. **Ribonucleoprotein (RNP)**: protein + RNA, fastest clearance, lowest off-target

## Ribonucleoprotein (RNP) Delivery

RNP delivery has become the preferred method for most research and therapeutic applications. Cas9 protein is produced recombinantly in *E. coli* or human cells, purified, and complexed with chemically synthesized or in vitro-transcribed sgRNA before delivery.

**Advantages**:
- **Fastest clearance**: Cas9 protein is degraded by cellular proteases within 4–24 hours. This brief exposure reduces cumulative off-target cleavage and immune activation.
- **No genomic integration risk**: unlike plasmid or lentiviral delivery, protein cannot integrate.
- **Highest efficiency in primary cells**: electroporation of RNP achieves 60–95% editing in primary T cells, HSCs, and NK cells — cell types that are refractory to plasmid transfection.
- **Compatible with 2′OMe-PS-modified sgRNAs**: chemical modifications that reduce immunogenicity and off-target activity work the same way in RNP format.

**Delivery method**: electroporation is most effective for primary cells. Parameters (voltage, pulse duration, buffer composition) are cell-type specific and must be optimized empirically. Commercial systems (Lonza Nucleofector, MaxCyte, Bio-Rad Gene Pulser) provide cell-type-specific protocols.

**Example**: ex vivo editing of hematopoietic stem cells (HSCs) for sickle cell disease therapy:
1. Mobilize HSCs from bone marrow with G-CSF + plerixafor
2. Collect HSCs by apheresis
3. Electroporate RNP (Cas9 + BCL11A enhancer sgRNA) to reactivate fetal hemoglobin
4. Infuse edited HSCs back into patient after myeloablative conditioning
5. Editing efficiency in engrafted cells: 70–95%
This is the approach used in FDA-approved Casgevy (exagamglogene autotemcel).

## Lipid Nanoparticle (LNP) Delivery

LNPs are the most advanced platform for systemic in vivo delivery, particularly to the liver. They consist of a lipid bilayer-like shell encapsulating nucleic acid cargo (mRNA or siRNA).

**Why LNPs work well for liver**: after intravenous injection, LNPs are taken up by hepatocytes (which express the ApoE receptor that binds LNPs) with high efficiency. Hepatocyte targeting is near-complete at optimized doses.

**CRISPR applications**:
- LNP-delivered Cas9 mRNA + sgRNA for transthyretin amyloidosis (clinical trials: Intellia NTLA-2001, 2021): >90% knockdown of transthyretin in liver in clinical trial Phase 1
- LNP-delivered base editor mRNA + sgRNA for PCSK9 knockdown (lipid-lowering therapy)

**Limitations**:
- Primarily hepatocyte-specific without active targeting modifications
- mRNA is less stable than plasmid DNA; must be coformulated under cold conditions
- Repeated dosing may trigger immune response to LNP components

**Ionizable lipids**: the key chemistry innovation in LNPs. Ionizable lipids (pKa ~6.2–6.5) are neutral at physiological pH (preventing membrane toxicity during circulation) but protonated at endosomal pH (~5.5), enabling endosomal escape and release of cargo into the cytoplasm.

## Adeno-Associated Virus (AAV) Delivery

AAV is the primary vector for in vivo delivery to tissues beyond the liver — particularly muscle, retina, CNS, and heart.

**Properties**:
- Single-stranded DNA genome, ~4.7 kb packaging limit
- Does not integrate (remains episomal; lost in dividing cells but persists in post-mitotic cells)
- Serotypes differ in tropism: AAV5/8/9 for liver; AAV9 for CNS; AAV2 for retina; AAV6 for muscle

**Size constraint problem**: SpCas9 CDS alone is 4.2 kb, leaving insufficient space for promoter, sgRNA, and poly-A signal in a single AAV.

**Solutions**:
- **SaCas9** (3.2 kb): fits with promoter and sgRNA in AAV; used in in vivo liver editing
- **Dual AAV with split intein**: Cas9 is split into N-terminal and C-terminal halves; each half delivered by a separate AAV; split intein domains in each half splice together in transduced cells to reconstitute full-length Cas9
- **Helper-dependent AAV**: larger capacity (up to 8 kb), but no endogenous viral genes; complex to produce

**In vivo AAV editing results**:
- Liver (AAV8-SaCas9 + sgRNA targeting PCSK9): 50–60% knockout of PCSK9 in mouse liver; durable (>1 year)
- Retina (subretinal AAV2-SpCas9): 20–30% editing in photoreceptors; proof-of-concept for Leber's congenital amaurosis

## Electroporation for Cell Lines and Bacteria

For cell lines and bacteria in culture, electroporation is the most efficient delivery method for all formats (DNA, RNA, RNP):

**Principle**: brief high-voltage electrical pulse transiently creates pores in the cell membrane, allowing entry of macromolecules.

**Bacterial electroporation**: 1.8–2.5 kV/cm; 5–8 ms pulse; standard for delivering plasmids and RNPs to *E. coli*, *Bacillus*, and other Gram-negative/positive bacteria. Efficiency: 10⁶–10⁹ transformants per µg plasmid.

**Mammalian cell electroporation (Nucleofection)**: lower voltage, longer pulse; specially formulated buffer + cell-specific programs. Key advantage over lipofection: works efficiently for hard-to-transfect cell types (primary T cells, neurons, HSCs).

## Lipofection for Dividing Cell Lines

Cationic lipid formulations (Lipofectamine, jetPEI) complex with negatively charged nucleic acids, forming particles that fuse with cellular membranes. Effective for dividing transformed cell lines (HEK293, Jurkat, HeLa). Inefficient for primary cells and difficult-to-transfect lines.

**Use case**: standard plasmid delivery for initial sgRNA testing in HEK293T before optimizing delivery for the actual target cell type.

## Comparison of Delivery Methods

| Method | Cell Types | Format | Off-Target Risk | In Vivo? | Cargo Size Limit |
|--------|-----------|--------|----------------|---------|----------------|
| RNP electroporation | Primary cells, lines | Protein + RNA | Lowest | No | None |
| LNP | Liver (systemic) | mRNA + sgRNA | Low | Yes | ~5 kb |
| AAV | Many tissues | DNA | Medium | Yes | ~4.7 kb |
| Lentivirus | Dividing cells | DNA (integrating) | Medium | Limited | ~8 kb |
| Lipofection | Dividing lines | DNA | High | No | None |
| Electroporation (cells) | Lines, primary cells | All formats | Low–medium | No | None |

## Why This Matters

Delivery is not a secondary consideration — it is often what determines whether a CRISPR application is feasible. The approval of the first CRISPR therapeutic (Casgevy) was enabled by efficient ex vivo RNP delivery to HSCs; LNP delivery enabled the first in vivo CRISPR clinical success (NTLA-2001 for transthyretin amyloidosis). The size constraint of AAV is driving both protein engineering (smaller Cas variants) and chemistry (split-intein dual-AAV systems). Understanding delivery options and their trade-offs is essential for translating CRISPR designs from bench experiments to applications that function in the intended biological context.
