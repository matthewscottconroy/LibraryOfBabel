# E. coli Extract-Based Cell-Free Systems: Preparation and Properties

Every cell-free experiment ultimately stands or falls on the quality of the extract. You can design the most elegant genetic circuit imaginable, but if your extract was harvested at the wrong growth phase or inadequately dialyzed, the reaction will sputter and the data will mislead you. Crude cell extracts from *E. coli* provide a simpler, cheaper, and higher-yield alternative to the PURE reconstituted system. The extract contains all the molecular machinery present in the cytoplasm — ribosomes, RNA polymerases, translation factors, metabolic enzymes — along with many other proteins whose roles in the extract reaction are unknown or unneeded. Mastering extract preparation is central to operating a productive cell-free synthetic biology laboratory.

## E. coli Strains for Extract Preparation

Not all *E. coli* strains produce equivalent extracts. Key strain choices:

**BL21(DE3)**: the most commonly used strain for extract preparation. Contains T7 RNA polymerase gene under lacUV5 promoter (inducible by IPTG). T7 RNAP can be expressed in the cells and partially retained in the extract, or T7 RNAP can be added separately to the reaction. Good general-purpose extract.

**MG1655**: wild-type *E. coli* K-12; cleaner background for regulatory studies; no T7 system → requires addition of T7 RNAP or sigma-70-based transcription.

**Rosetta2**: contains rare-codon tRNAs; improves translation of genes with rare codons; useful for expressing mammalian or plant genes in cell-free systems.

**Specialized strains**: some extract preparation protocols use strains with knockouts of major proteases (lon, clpP) to improve yields of unstable proteins, or strains overexpressing specific chaperones for difficult-to-fold proteins.

## Extract Preparation Protocol (Standard S12/S30 Method)

The standard protocol (Noireaux lab; Pardee lab; Sun et al. 2013):

### Step 1: Cell Growth

```
1. Inoculate 1 L LB (or 2× YTPG for higher yields) from overnight culture
2. Grow at 37°C to OD₆₀₀ = 0.6–0.8 (mid-log phase)
   Alternative: grow to OD₆₀₀ = 1.5–2.0 (early stationary); use different protocol
3. Chill culture immediately in ice bath
4. Harvest by centrifugation (5,000 × g, 4°C, 15 min)
5. Wash pellet 3× with S30 buffer (10 mM Tris-OAc pH 7.7, 60 mM KOAc, 14 mM Mg(OAc)₂, 1 mM DTT)
```

Growth phase matters: mid-log cells are actively translating with high ribosome content. Stationary phase cells have reduced translational activity.

### Step 2: Cell Lysis

The lysis method significantly affects extract quality:

**Bead-beating** (standard): mix washed cells with 0.1 mm glass beads (1:1 v/v) in a bead-beater; 3 × 45-second pulses at 4°C. Efficient lysis with minimal heat generation. Most commonly used.

**French press**: pass cell suspension through a small orifice at high pressure (~14,000 psi). Very efficient lysis; reproducible; requires French press instrument.

**Sonication**: sonicate cell suspension with a probe tip sonicator; multiple pulses on ice. Less reproducible than bead-beating or French press; risk of heat damage.

### Step 3: Clearing Centrifugation

```
4. Centrifuge lysate at 12,000 × g for 10 min at 4°C (removes cell debris and unlysed cells)
   OR centrifuge at 30,000 × g for 30 min (S30 extract — removes most membranes)
   OR centrifuge at 100,000 × g for 2 hours (S100 extract — removes ribosomes too, for specialized uses)
```

The choice of centrifugation speed determines which fraction is retained:
- **S12 extract**: 12,000 × g pellet removed; ribosomes and membranes retained. High ribosome content; often highest translational activity.
- **S30 extract**: 30,000 × g pellet removed; ribosomes retained; most membranes removed. Standard for cell-free protein synthesis.

### Step 4: Run-Off Reaction (Critical)

**Run-off reaction**: incubate the cleared lysate at 37°C for 80 minutes without DNA template. This step depletes endogenous mRNA:
- Endogenous mRNAs are translated and degraded by natural RNases
- After run-off, the ribosome pool is free of endogenous mRNA → fully available for exogenous circuit DNA

Without run-off, endogenous mRNA competes with the synthetic circuit DNA for ribosomes, dramatically reducing circuit expression levels.

### Step 5: Dialysis

After run-off, dialyze extract against S30 buffer to remove small molecules: degradation products, amino acids, nucleotides from endogenous mRNA degradation, DTT from the run-off incubation.

### Step 6: Flash Freeze and Store

Aliquot extract in 50–200 µL volumes; flash freeze in liquid nitrogen; store at -80°C. Properly prepared extracts maintain activity for 1–2 years.

## Extract Properties and Quality Assessment

**Protein concentration**: 20–60 mg/mL total protein in the extract. Ribosome concentration: ~5–15 µM (equivalent to ~30–90 µM ribosomal subunits).

**Translation activity test**: add a control reporter (GFP or deGFP mRNA or plasmid) to extract + energy/amino acid supplement; measure fluorescence at 37°C over 8 hours. Good extract: GFP signal reaches plateau within 4 hours; typical yield 200–500 µg/mL deGFP equivalent.

**Batch-to-batch variability**: extract quality can vary between batches due to subtle differences in growth phase, lysis efficiency, or dialysis. Always test a new batch against the previous batch with the same reference construct before using in experiments.

## Energy Regeneration Systems

The choice of energy system significantly affects both yield and reaction duration:

**3-Phosphoglycerate (3-PGA)**: most commonly used in Noireaux-lab-style extracts. 3-PGA is converted to ATP through the glycolytic enzymes present in the extract:
$$\text{3-PGA} \rightarrow \text{2-PGA} \rightarrow \text{PEP} \xrightarrow{\text{PK}} \text{ATP}$$
Provides 6–8 hours of activity. Inexpensive.

**Creatine phosphate/creatine kinase (CP/CK)**: simpler chemistry; more reproducible; CK must be supplemented. 3–4 hours of activity. More expensive than 3-PGA.

**Maltose/maltodextrin**: uses the maltose phosphorylase system; longer duration (12+ hours); compatible with continuous-exchange cell-free format.

**Phosphoenolpyruvate (PEP)**: very clean energy system; 2–3 hours. Most expensive.

## Supplement Formulation

The extract must be supplemented with components that are present in the cell but diluted by lysis:

Standard supplement for 10 µL extract-based reaction:
```
- Amino acids: 2 mM each of all 20
- ATP: 1.5 mM
- GTP: 1.25 mM  
- CTP + UTP: 0.85 mM each
- tRNA (E. coli total): 170 µg/mL
- Magnesium glutamate: 4 mM
- Potassium glutamate: 80 mM
- 3-PGA: 33 mM (energy source)
- CoA: 0.26 mM (for acetyl-CoA regeneration)
- NAD: 0.33 mM
- cAMP: 0.75 mM (prevents catabolite repression effects)
- Folinic acid: 0.068 mM (for formylmethionine)
- Putrescine: 1 mM
- Spermidine: 1.5 mM
- DTT: 2 mM
```

## Why This Matters

Extract preparation is a laboratory skill that directly determines the quality and reproducibility of all cell-free experiments. A poorly prepared extract — wrong growth phase, inadequate run-off, improper dialysis — produces low yields, high variability, and misleading results that cannot be replicated. The investment in understanding and optimizing extract preparation is paid back through reliable, high-quality data in downstream experiments. For synthetic biology laboratories that use cell-free systems routinely (for circuit prototyping, part characterization, or production), the extract preparation is the foundational process on which all other work depends — in the same way that a good DNA miniprep protocol is foundational to molecular cloning.
