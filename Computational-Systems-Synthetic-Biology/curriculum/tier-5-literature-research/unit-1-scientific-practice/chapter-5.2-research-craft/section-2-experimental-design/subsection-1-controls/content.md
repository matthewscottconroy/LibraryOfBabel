# Controls in Experimental Biology

Here is a scenario that plays out in biology labs more often than anyone likes to admit. A graduate student spends three weeks showing that a small molecule inhibitor dramatically reduces expression of a target gene. The Western blots are clean, the qPCR is consistent, the dose-response curve looks beautiful. Then a senior lab member asks: "What did you use to dissolve the inhibitor?" DMSO. "Did you run a DMSO vehicle control?" Silence. The inhibitor was dissolved in 0.5% DMSO — a concentration known to alter gene expression in the cell line being used. Three weeks of work reduced to a question that cannot be answered. A result without appropriate controls is not a result — it is an observation that cannot be interpreted. This section systematically covers the types of controls required in biological experiments, why each is necessary, and what specific failure modes each type of control prevents.

## What Controls Are and Why They Are Necessary

An experiment answers a question by comparing two or more conditions. The experimental condition is the one of interest; the control conditions define what the outcome looks like in the absence of the experimental variable. Without controls, you cannot distinguish:

- A real biological effect from a measurement artifact
- A specific drug effect from the effect of its solvent
- A gene function from a confounding genetic background difference
- A true positive from the detection limit of your assay

Controls are not overhead — they are the mechanism by which experiments produce interpretable results.

## Positive Controls

**Definition:** A condition known to produce the expected result, included in the same experiment as the test condition.

**Purpose:** Confirms that the assay is functioning correctly and can detect the effect being studied. If the positive control does not work, any negative result from the experimental condition is uninterpretable — was there no effect, or did the assay fail?

**Examples:**

- In a Western blot for phospho-Akt, include lysate from cells treated with EGF (which reliably activates Akt phosphorylation)
- In a GFP reporter assay for promoter activity, include a constitutive promoter construct
- In a CRISPR editing experiment, include a guide RNA targeting a well-validated gene (e.g., AAVS1 safe harbor)
- In a differential expression analysis, include spike-in RNA controls (ERCC) at known concentrations

**What happens without a positive control:** If your experimental condition gives no signal, you cannot determine whether the gene/drug/condition has no effect (true negative) or whether your assay failed to detect a real effect (false negative). A positive control resolves this ambiguity.

**When positive controls are most critical:** Any assay that detects a rare event (low transfection efficiency, rare mutation) or that requires a working detection reagent (antibody, probe) needs a positive control in every experiment, not just during assay development.

## Negative Controls

**Definition:** A condition known to produce no effect; the baseline measurement.

**Purpose:** Establishes the background level of signal in the absence of the effect being tested. Without a negative control, you cannot determine whether observed signal represents a real effect or background noise.

**Examples:**

- In a reporter assay with an inducible promoter, include a sample without inducer
- In a cell migration assay, include untreated cells (no chemokine gradient)
- In a pull-down assay, include beads without the bait protein
- In a colony-forming assay with an antibiotic, include plates without antibiotic (to verify cells are viable) and plates with antibiotic and without the resistance gene

**Non-template controls in PCR:** Every PCR run should include a no-template control (NTC) — a reaction with water instead of sample. Amplification in the NTC indicates contamination.

**Empty vector controls:** When overexpressing a gene of interest in a plasmid, the negative control is the same plasmid backbone without the gene insert. This controls for the effect of transfection, antibiotic selection, and plasmid backbone on cell behavior.

## Vehicle Controls

**Definition:** Treatment with the carrier substance used to dissolve the experimental compound, without the compound itself.

**Purpose:** Many small molecule compounds are dissolved in organic solvents (most commonly DMSO, but also ethanol, PEG, or methanol). DMSO at concentrations above 0.1–0.5% has measurable effects on mammalian cells, including changes in differentiation, membrane permeability, and gene expression. Without a vehicle control, it is impossible to determine whether an observed effect is due to the compound or its solvent.

**The DMSO problem:** Many published cell biology papers use 0.1% DMSO as the vehicle for drugs or small molecules. If the DMSO vehicle control is omitted, and DMSO is producing a phenotype, the conclusion that the experimental compound is responsible is incorrect. The vehicle control must be at the same DMSO concentration as the treatment condition.

**Rule:** Whenever your experimental condition contains a substance not present in native biology (organic solvent, carrier lipid, polyethylene glycol), include a vehicle control at the same concentration.

## Isogenic Controls

**Definition:** Control strains or cell lines that are genetically identical to the experimental condition except for the single variable being tested.

**Purpose:** Genetic background has large effects on phenotype. If you compare a knockout strain to an unrelated wild-type strain, any phenotypic difference could be due to the knockout or to any of the thousands of other genetic differences between the strains.

**Examples:**

- A CRISPR knockout cell line must be compared to the same parental cell line without the edit (or, better, to a clonal line with a confirmed non-editing control guide RNA)
- A gene overexpression experiment must be compared to the same strain with an empty vector
- A transposon insertion mutant (Tn-seq) must be compared to the parent strain

**The isogenic requirement in clinical-translational research:** Patient-derived iPSC experiments require isogenic controls — either an unedited clone from the same patient or a clone in which the causal mutation has been corrected. Using a healthy donor as the "control" is inadequate because genetic background differences between donors can produce phenotypic differences larger than the effect of the mutation being studied.

## Technical Controls for Specific Assays

**Flow cytometry:** Single-color controls for compensation; fluorescence-minus-one (FMO) controls for gating; isotype controls for antibody specificity (though FMO controls are preferred over isotype controls in most modern flow cytometry workflows).

**Microscopy and image analysis:** Include a background image (no-cell field) to calibrate background subtraction; include a no-antibody or no-fluorophore control to measure autofluorescence; use the same imaging settings across all conditions.

**RNA-seq:** Include spike-in RNA standards (ERCC or SIRV) for technical normalization; confirm RNA quality by RIN (RNA Integrity Number) before library preparation; include a replicate of a standard condition across different batches to assess batch effects.

**Mass spectrometry (proteomics/metabolomics):** Include pooled QC samples (aliquots of all samples mixed together) injected at regular intervals to monitor instrument drift; include labeled standards (isotopically labeled peptides or metabolites) for absolute quantification.

## The Hierarchy of Controls

Not all controls are equally important. In planning an experiment, prioritize in this order:

1. **Positive control** — always include; without it, negative results are uninterpretable
2. **Negative control / vehicle control** — always include; defines baseline and prevents solvent artifacts
3. **Isogenic control** — required for any genetic manipulation experiment
4. **Technical assay controls** — required for any quantitative measurement (calibration curves, blanks, internal standards)

## Common Errors in Control Design

**Including controls in a different experiment:** Controls must be in the same experiment as the test conditions, processed at the same time. A positive control from a previous week does not validate today's experiment.

**Insufficient biological replicates for controls:** If your experimental condition has n=3 biological replicates, your control should too. Comparing one experimental condition to one control is not powered to detect variability.

**Using historical controls without validation:** "This result matches historical control values from previous experiments" is acceptable for preliminary data, not for publication. Controls must be concurrent.

**Conflating positive control with experimental condition:** The positive control must use a known, validated positive signal — not the experimental compound at a higher dose. If you are testing a new drug, the positive control is a different drug with established activity, not more of the new drug.

## Takeaway

Controls define what an experiment can and cannot conclude. Positive controls validate that the assay works; negative and vehicle controls establish baseline; isogenic controls eliminate confounding genetic background; assay-specific technical controls ensure measurement accuracy. A missing control does not merely weaken a result — in many cases it makes the result uninterpretable. The discipline of always designing experiments with complete controls is one of the most important habits a scientist can develop.
