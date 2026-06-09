# Quantitative Proteomics

Finding that a protein is present in your sample is rarely the interesting result. Proteins are the machinery of the cell — the question that drives most proteomics experiments is not whether the machinery exists, but how its operating levels shift when you change the cell's environment. Does this kinase get upregulated when you starve cells of glucose? Does this transcription factor disappear after DNA damage? Does treatment with a candidate drug suppress oncogenic signaling across the proteome, or does it trigger a compensatory response that ultimately allows resistance?

These are quantitative questions. And answering them precisely — reliably, reproducibly, with enough statistical power to distinguish biology from noise — requires thinking carefully about how you assign numbers to protein abundances and how you compare those numbers across samples. Identifying which proteins are present in a sample is only the first step; determining how their abundance changes between conditions (disease vs. healthy, treated vs. untreated) is the primary biological question. Quantitative proteomics encompasses a range of strategies that differ in how they assign numerical values to protein abundance and how they compare across samples.

## Label-Free Quantification (LFQ)

**Label-free quantification** requires no chemical modification of the sample before analysis. Two approaches are used:

**MS1 peak area**: For each identified peptide, the area under the chromatographic peak of the precursor ion (MS1 level) is integrated across the LC gradient. Peptides from more abundant proteins produce higher peak areas. MaxQuant implements LFQ via the MaxLFQ algorithm, which normalizes peptide intensities between samples using the median ratio of shared peptides, producing comparable protein intensities.

**Spectral counting**: The number of MS2 spectra (PSMs) assigned to a protein correlates roughly with its abundance. More abundant proteins produce more peptides that are selected for fragmentation. SpC is less accurate than peak area LFQ but is conceptually simple and works even with unit-resolution instruments.

**Pros of LFQ**: Simple sample preparation, no added cost, unlimited sample number.  
**Cons**: Higher technical variability between samples due to LC/MS run-to-run variation; "missing values" occur when a peptide's peak is detected in some samples but not others (stochastic MS2 sampling in DDA mode).

The missing value problem is not merely a statistical nuisance — it has real biological consequences. Imagine a signaling protein that is expressed in your disease samples but absent or below the detection threshold in controls. This protein will appear as a missing value in some conditions, and naive statistical tests that require complete data will exclude it from analysis entirely. Sophisticated imputation strategies exist, but they introduce assumptions. Understanding why missing values arise in DDA data — stochastic sampling of the most abundant ions at each moment in the LC gradient — motivates the development of DIA (the next subsection), which was designed specifically to eliminate this problem.

## SILAC: Stable Isotope Labeling by Amino Acids in Cell Culture

**SILAC** (Ong et al., 2002) incorporates isotopically labeled amino acids into cellular proteins metabolically. Cells are grown in media containing either "heavy" forms of lysine and arginine ($^{13}$C₆/$^{15}$N₂-Lys = +8.014 Da; $^{13}$C₆-Arg = +6.020 Da) or "light" (standard) amino acids. After 5–6 cell doublings, all proteins are >99% labeled.

Heavy and light cell lysates are then **mixed at a 1:1 ratio** before sample preparation. Since both light and heavy proteins are processed together, all technical variation (digestion, LC, MS) is identical. The ratio of heavy:light peak areas in MS1 directly quantifies the relative protein abundance between the two conditions.

$$\text{Ratio} = \frac{\text{Heavy peak area}}{\text{Light peak area}}$$

A ratio of 2 means the heavy-labeled (e.g., treated) sample has twice the abundance of that protein compared to the light (control).

**Pros**: Gold standard for accuracy; minimal technical variation because samples are pooled pre-analysis.  
**Cons**: Only applicable to cells in culture (cannot label organisms or patient tissues); typically limited to 2 or 3 labels (light, medium-heavy, heavy); expensive isotope-labeled media.

SILAC introduced a conceptually elegant solution to the quantification problem: mix the samples before you do anything else. Every subsequent step — trypsin digestion, LC separation, ionization, fragmentation, detection — is applied equally to both the control and treated samples because they are physically in the same tube. The only difference between a light peptide and its heavy counterpart is their mass, and that mass difference is precisely defined by the isotope substitution. It turns out that this pre-pooling strategy is so effective that SILAC ratios are routinely accurate to within 5–10%, and SILAC is still the gold standard against which newer quantification methods are benchmarked, more than twenty years after Ong's seminal paper.

## TMT/iTRAQ: Isobaric Chemical Labels

**TMT** (Tandem Mass Tags, Thermo) and **iTRAQ** (isobaric tags for relative and absolute quantitation, AB Sciex) are isobaric chemical labels: all labeled samples have the same total mass before fragmentation (so all labeled versions of a peptide appear as a single peak in MS1), but upon HCD fragmentation, TMT tags release **reporter ions** at distinct low-mass m/z values (e.g., TMT 10-plex: reporter ions at m/z 126.1, 127.1, 127.2, ..., 131.1).

Workflow: Each sample is separately digested, labeled with a different TMT channel, then all samples are combined and analyzed in a single LC-MS/MS run. The relative abundance of each sample is read from the reporter ion intensities in each MS2 spectrum.

**TMT16-plex** and **TMTpro 18-plex** allow up to 16 or 18 samples to be analyzed simultaneously in a single run, dramatically increasing throughput.

**Pros**: High multiplexing (16–18 samples), no missing values for detected proteins (all samples analyzed together), compatible with any cell type or tissue.  
**Cons**: **Ratio compression** — co-fragmented precursor ions (from different peptides selected in the same isolation window) contribute their reporter ions to every spectrum, compressing ratios toward 1. MS3 on the Orbitrap Fusion (SPS-MS3) or TMTc methods address this; higher sample complexity can be managed by offline pre-fractionation.

Ratio compression is the dirty secret of TMT quantification. Because DDA selects an isolation window of ±1–2 Da, it inevitably co-isolates other peptides that happen to elute at the same time and have similar masses. Those contaminating peptides also carry TMT tags, and when they fragment they contribute reporter ions to every channel — systematically pushing all ratios toward 1 (because on average the contaminating peptides have equal abundance in all channels). You might expect to see a 5-fold change, and measure a 2-fold change instead. The fix — MS3, which selects a fragment ion from the MS2 spectrum and fragments it again to generate cleaner reporter ions — is effective but halves the throughput. It is a recurring theme in MS: every gain in one dimension comes at a cost in another.

## SILAC vs. TMT vs. LFQ Comparison

| Feature | LFQ | SILAC | TMT/iTRAQ |
|---|---|---|---|
| Sample types | Any | Cell culture only | Any |
| Number of samples | Unlimited | 2–3 | Up to 18 (TMTpro) |
| Sample prep complexity | Low | Low (metabolic) | High (chemical labeling) |
| Cost | Lowest | Medium | High (reagent cost) |
| Missing values | Common | Rare | Rare (all samples co-analyzed) |
| Quantification accuracy | Moderate | High | Moderate (ratio compression) |
| Pre-fractionation benefit | High | Moderate | Essential for depth |

## Normalization Strategies

Regardless of quantification approach, normalization is required to make samples comparable. Common strategies:
- **Median centering**: Shift each sample's log-transformed intensities so the median is 0.
- **Quantile normalization**: Force all samples to have identical intensity distributions.
- **Internal standard normalization**: Spike in a fixed amount of a labelled reference protein (e.g., heavy-labeled proteotypic peptides) for absolute quantification (AQUA proteomics).

## Why This Matters

Quantitative proteomics bridges genomics and cell biology by directly measuring the functional molecules (proteins) rather than their proxies (mRNAs); the choice of quantification strategy has major consequences for experimental throughput, accuracy, and the types of samples that can be studied, making this decision central to any proteomics experimental design. The same biological question — which proteins change after this perturbation? — can be approached with LFQ in a small pilot study, SILAC for a mechanistic follow-up in cell lines, or TMT18-plex for a large clinical cohort. Each approach will tell you something true; each will also leave something out. The art is knowing which trade-offs matter for your specific question.
