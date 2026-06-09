# LC-MS Metabolomics Workflow

Here is an uncomfortable truth about untargeted metabolomics: the results of your experiment are shaped at least as much by what you do before the mass spectrometer as by what happens inside it. A beautifully calibrated Orbitrap cannot compensate for metabolites that degraded during sample collection, or for enzyme activity that continued to reshape the metabolite pool during extraction, or for systematic matrix effects that made certain lipid classes invisible. The workflow from living cell to mass spectrum is a long chain of steps, and errors at the beginning propagate invisibly through the entire analysis.

The LC-MS metabolomics workflow transforms biological samples (cells, plasma, urine, tissue) into a feature matrix of metabolite intensities across samples. Each step — from sample collection to data processing — introduces potential sources of bias and variability that must be controlled. A rigorous workflow produces reproducible, high-quality data amenable to statistical analysis and biological interpretation.

## Sample Collection and Quenching

Metabolites are chemically reactive and change rapidly after sample collection: enzymes continue to act, metabolite pools equilibrate, and labile modifications are lost. **Quenching** stops this metabolic activity immediately.

For cell cultures: Add cold methanol (−40°C) or fast-transfer cells to liquid nitrogen. Freeze-quenching is the gold standard.
For plasma: Draw blood into appropriate tubes (EDTA for plasma; serum tubes for serum), centrifuge at 4°C within 30 minutes, aliquot, and freeze at −80°C. Avoid repeated freeze-thaw cycles.
For tissue: Flash-freeze in liquid nitrogen immediately after removal; store at −80°C until extraction.

The quenching step is where the biology stops and the chemistry begins. An improperly quenched sample has a metabolome that is a mixture of the biology you want to measure and the ex vivo artefacts you want to avoid. For cell culture metabolomics, studies have shown that even 10 seconds at room temperature before quenching can cause measurable changes in ATP, ADP, AMP, and TCA cycle intermediates — exactly the metabolites that are most biologically interesting and most enzymatically labile.

## Extraction Solvent Selection

The metabolome is chemically diverse — polar (amino acids, organic acids, nucleotides) and non-polar (lipids, steroids, fat-soluble vitamins) — requiring different solvents to extract different classes:

**Polar metabolites**: 80% methanol or 50:50 methanol:acetonitrile (protein precipitation and metabolite extraction in one step). Alternatively, acetonitrile alone provides good coverage of amino acids and TCA cycle intermediates.

**Lipid extraction**: **Bligh-Dyer** or **Folch** extraction — chloroform:methanol:water (2:1:0.8 or 1:2:0.8 by volume). Two phases form: the organic (chloroform) lower phase contains neutral and phospholipids; the aqueous (methanol:water) upper phase contains polar metabolites. This allows simultaneous recovery of both fractions.

**Protein precipitation**: A simple but effective approach — mix sample with 3–4 volumes of cold methanol or acetonitrile, vortex, centrifuge, and take the supernatant. This removes proteins (which would clog the LC column and cause ion suppression) while retaining most small molecule metabolites.

## Chromatographic Separation

Two complementary LC modes are used for metabolomics:

**Reversed-Phase LC (RPLC, C18)**: Separates molecules by hydrophobicity. Polar metabolites elute early (near the void volume), hydrophobic lipids elute late. Ideal for lipids, fatty acids, steroids, and moderately polar metabolites. Uses aqueous→organic gradient (water→acetonitrile or methanol).

**HILIC** (Hydrophilic Interaction Liquid Chromatography): The stationary phase is hydrophilic; the mobile phase starts organic and transitions to aqueous. Polar, charged metabolites that elute in the void of RPLC are well-retained and separated by HILIC: amino acids, sugars, nucleotides, organic acids. HILIC is the method of choice for polar primary metabolites.

In practice, maximizing coverage requires running both RPLC and HILIC, in both positive and negative ionization modes — up to 4 separate analytical runs per sample.

The decision to run up to four separate LC-MS analyses per sample is expensive in instrument time and reagents. But consider what you lose if you cut corners: RPLC misses amino acids and nucleotides entirely (they elute in the void). Positive mode alone misses organic acids and bile acids that ionize preferentially as [M-H]⁻. Each analytical mode is a window on a different chemical dimension of the metabolome, and no single window shows the whole landscape.

## Ionization Mode

**Positive ionization mode** ([M+H]⁺, [M+Na]⁺, [M+NH₄]⁺): Better for basic compounds, amines, lipids with phosphocholine headgroups, vitamins.
**Negative ionization mode** ([M-H]⁻, [M+Cl]⁻, [M+COOH]⁻): Better for acidic compounds — organic acids, nucleotides, phospholipids with acidic headgroups, bile acids.

Some metabolites ionize well in both modes (nucleosides, amino acids); others are strongly mode-dependent (glucose ionizes poorly in both, often detected as [M+NH₄]⁺ or by derivatization).

## Data Processing Pipeline

Raw LC-MS data (typically .mzML, .raw, or .wiff format) must be processed to extract and align features across samples:

**XCMS** (in R) and **MZmine** (Java) are the standard open-source tools:

1. **Peak picking**: Detect chromatographic peaks above background in each sample. The XCMS `centWave` algorithm uses continuous wavelet transform to identify peaks by their characteristic width and m/z profile.
2. **Feature alignment**: Retention times shift slightly between injections (due to column aging, pressure variations). The `obiwarp` or `groupChromPeaks` algorithm aligns features across samples by warping retention time axes.
3. **Gap filling**: If a feature was detected in 5/6 samples, the intensity in the missing sample is estimated by integrating the raw data at the expected retention time and m/z (even if no peak was formally detected).

The output is a **feature matrix**: rows = samples, columns = features (defined by m/z and retention time), values = peak areas.

## Missing Value Imputation

Missing values arise from features below the limit of detection in some samples. Random imputation approaches:
- **Half minimum**: Replace missing values with half the minimum observed intensity for that feature.
- **KNN imputation**: Use k-nearest-neighbor samples to estimate the missing value.
- **MICE or multiple imputation**: More sophisticated statistical approaches.

## Normalization

Normalization corrects for systematic variation in total metabolite concentration between samples (due to pipetting errors, matrix effects, or biological variation in sample amount):

- **Total ion count (TIC) normalization**: Divide each feature's intensity by the total sum of all feature intensities in that sample, then multiply by the median TIC across samples.
- **QC-sample normalization**: Inject pooled QC samples (made by mixing equal aliquots of all study samples) every 10–15 injections throughout the run. Fit a LOESS curve to the QC intensities over injection order for each feature; use this curve to correct for signal drift.
- **Internal standard normalization**: For targeted assays, use stable-isotope-labeled internal standards added at a known concentration before extraction to normalize for extraction efficiency and matrix effects.

Signal drift — the gradual change in instrument response over the course of a long analytical batch — is one of the most insidious sources of artifact in metabolomics. Without correction, samples analyzed on day 1 of a 3-day run will appear systematically different from samples analyzed on day 3, not because the biology is different but because the instrument changed. Pooled QC samples, injected regularly throughout the batch, make this drift visible and correctable. They are not an optional extra in a well-designed metabolomics study; they are a prerequisite for drawing any conclusions at all.

## Why This Matters

The quality of LC-MS metabolomics data depends critically on every step of this workflow — from flash-freezing samples to QC normalization — because metabolomics features are not self-documenting like genomic sequences; a poorly controlled workflow produces artifactual patterns that cannot be distinguished from genuine biology without careful experimental design and QC monitoring. When you read a metabolomics paper, look for: Was quenching described? Were QC samples used? Were missing value rates reported? Were RPLC and HILIC run separately? These details determine whether the biological signals in the data are real or are artifacts of the workflow — and they are often buried in the supplementary methods.
