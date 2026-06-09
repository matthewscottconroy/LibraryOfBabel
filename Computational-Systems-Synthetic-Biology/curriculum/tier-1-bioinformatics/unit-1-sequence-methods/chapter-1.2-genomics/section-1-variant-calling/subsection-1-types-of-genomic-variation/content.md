# Types of Genomic Variation

Here is a question that drove a decade of human genetics research: why do two people — both human, both reading this sentence, both with 3.2 billion base pairs of DNA — nonetheless differ in their risk of developing cancer, their response to a blood pressure drug, or the color of their irises? The answer lives in the differences between those 3.2 billion base pairs. Genomic variation is the substrate of evolution and the molecular basis of heritable disease. Virtually every phenotypic difference between individuals — from eye color to drug response to disease risk — can be traced to differences in DNA sequence. Understanding the spectrum of genomic variation, its mechanistic origins, and its consequences for gene function is prerequisite to all variant-calling analyses.

What makes this intellectually rich is that variation is not one thing. It spans six orders of magnitude in size, it arises by completely different molecular mechanisms, and it requires completely different detection strategies. An analyst who knows SNPs but not structural variants is like a geologist who knows sediment types but not faults — they can describe the surface but they will miss the forces that shaped it.

## The Spectrum of Variant Size

Genomic variants span six orders of magnitude in size, from single nucleotide changes to entire chromosome duplications. The detection method, downstream consequences, and clinical significance differ substantially across this size spectrum.

## Single Nucleotide Polymorphisms (SNPs)

**SNPs** are changes in a single nucleotide position that are present in at least 1% of the population (variants below this threshold are called **rare variants** or **mutations**). They are the most common form of human genetic variation, with approximately 4–5 million SNPs distinguishing any two human genomes.

You might find that number surprisingly large or surprisingly small, depending on your perspective. It is large enough to make every human genome unique. But it is small relative to the 3.2 billion base pair total — meaning any two people share more than 99.9% of their sequence. We are an extraordinarily similar species, a fact that reflects our recent common origin in Africa roughly 100,000–200,000 years ago.

**Transition vs. transversion**:
- **Transitions (Ts)**: purine↔purine (A↔G) or pyrimidine↔pyrimidine (C↔T); 2× more common than transversions
- **Transversions (Tv)**: purine↔pyrimidine (A/G ↔ C/T)
- Ti/Tv ratio: ~2.1 for whole genome, ~3.0 for whole exome (used as QC metric)

The biological reason transitions are more common: spontaneous deamination of cytosine produces uracil (which reads as thymine), making C→T transitions — especially at methylated CpG sites — the single most common mutation in the human genome. This is not mere biochemical trivia; it forms the basis of quality control metrics that flag suspect variant calls before they contaminate your analysis.

**Functional consequences of SNPs** (in coding regions):
- **Synonymous (silent)**: nucleotide change but same amino acid (e.g., AAA → AAG, both Lys)
- **Missense**: amino acid change (e.g., p.Arg175His in TP53 — common cancer driver)
- **Nonsense**: amino acid → stop codon; truncates protein (e.g., p.Gln58*)
- **Splice site**: disrupts canonical splice signals (GT...AG) → aberrant splicing

It turns out that "synonymous" mutations are not always truly silent — they can affect splicing, mRNA stability, translational speed, and protein folding. But for the purposes of variant effect prediction, the distinction between synonymous and nonsynonymous variants is a useful first filter.

## Small Insertions and Deletions (Indels)

**Indels** are insertions or deletions of 1–50 bp. They are the second most common form of human variation (~1 million in a typical genome).

**Consequences in coding regions**:
- If indel length is a **multiple of 3**: in-frame insertion or deletion (adds or removes amino acids)
- If indel length is **not a multiple of 3**: **frameshift** — changes every downstream codon, usually creating a premature stop codon

**Example frameshift**:
```
Original: ATG AAA GGC TTC → Met Lys Gly Phe
+1 insert: ATG AAA GGG CTT C... → Met Lys Gly Leu [frameshift]
```

Frameshifts in exons are usually loss-of-function and subject to nonsense-mediated decay (NMD). The cell has evolved NMD precisely to degrade these aberrant transcripts before they can produce toxic truncated proteins — a surveillance system that runs continuously in every cell, quietly discarding the molecular wreckage of replication errors.

## Structural Variants (SVs)

**SVs** are variants > 50 bp. They include:

| Type | Description | Detection method |
|------|-------------|-----------------|
| Deletion | Loss of sequence | Split reads, discordant pairs |
| Insertion | Novel sequence insertion | Split reads, long reads |
| Duplication (tandem) | Segment repeated in tandem | Read depth, split reads |
| Inversion | Segment reversed in orientation | Discordant read pairs |
| Translocation | Segment moves to different chromosome | Discordant pairs, split reads |
| Mobile element insertion | Alu, LINE, SINE insertion | Soft-clipped reads |

SVs tend to have larger functional effects than SNPs because they can:
- Delete or duplicate entire exons or genes
- Disrupt regulatory elements (enhancers, CTCF binding sites)
- Create gene fusions (especially in cancer via translocations)

**Detection**: short reads detect many SVs through discordant read pairs (unexpected orientation/distance) and split reads (reads spanning SV breakpoints). Long reads (minimap2 + PBSV, Sniffles2) detect SVs with much higher sensitivity and precision.

The key insight here is that structural variants are not exotic rarities — they are present in every human genome, and their contribution to genetic disease has historically been underappreciated because the standard short-read workflows miss many of them. The genomics field is in the middle of a reckoning with this blind spot, driven by the falling cost of long-read sequencing.

## Copy Number Variants (CNVs)

**CNVs** are gains or losses of large genomic segments (typically > 1 kb), detectable as changes in read depth. They represent the second-largest category of genetic diversity, with each human genome containing ~1,000 CNVs affecting ~0.8% of the genome.

Depth of coverage at position $i$:

$$\text{Copy number} \approx 2 \times \frac{\text{observed depth}_i}{\text{genome-wide mean depth}}$$

(for a diploid sample; × 1 for hemizygous deletion, × 3 for single-copy gain)

**Tools**: CNVkit, GATK gCNV, Control-FREEC

**Clinical examples**:
- Chromosome 22q11.2 deletion: DiGeorge syndrome
- 17p11.2 duplication: Potocki-Lupski syndrome
- Somatic amplification of *ERBB2* (HER2): breast cancer driver

The ERBB2 example is worth pausing on. In roughly 15–20% of breast cancers, the gene encoding the HER2 receptor is amplified from its normal 2 copies to 20 or more. This amplification — detectable as a dramatic read-depth elevation — is what makes HER2-positive breast cancer both more aggressive and more treatable: the HER2 protein is an excellent drug target for trastuzumab (Herceptin), a therapy that transformed outcomes for this subtype. Copy number calling is not academic; it directly informs treatment decisions.

## Short Tandem Repeats (STRs)

**STRs** (microsatellites) are repeated motifs of 1–6 bp with highly variable repeat counts:
- `(CA)n` dinucleotide repeats are most common in the human genome
- Mutation rate ~10⁻⁴ per generation (much higher than SNP rate of 10⁻⁸)
- Most STRs are neutral; some cause disease when expanded

**Triplet repeat expansion disorders**: Huntington disease (CAG repeats in HTT), fragile X syndrome (CGG in FMR1), myotonic dystrophy (CTG in DMPK). Long-read sequencing (ONT, PacBio) is required to accurately genotype expanded repeats.

The molecular pathology of repeat expansions is fascinating: normal individuals have 10–35 CAG repeats in HTT, while Huntington disease requires 36 or more. The CAG codon encodes glutamine, so the huntingtin protein acquires a polyglutamine tract that causes it to misfold and aggregate in neurons. The repeat length determines age of onset with grim precision — longer repeats predict earlier onset, creating a molecular countdown that clinicians can now read from a blood test.

## Variant Classification in Clinical Genetics

The ACMG/AMP guidelines (2015) classify variants in five categories:
1. **Pathogenic**: causes disease
2. **Likely pathogenic**: > 90% confidence
3. **Variant of Uncertain Significance (VUS)**: insufficient evidence
4. **Likely benign**: < 10% probability of pathogenicity
5. **Benign**: does not cause disease

Most variants identified in a sequenced genome are common (MAF > 1%) and benign. Identifying the rare causative variant in a diagnostic context requires filtering by population frequency, functional prediction, and segregation with phenotype.

The "Variant of Uncertain Significance" category deserves special attention. VUS classification is not a scientific failure — it is an honest acknowledgment that evidence is currently insufficient. The challenge is that VUS results create anxiety for patients and uncertainty for clinicians. A large fraction of the field's clinical interpretation work involves re-classifying VUS variants as more data accumulates from large-scale studies like gnomAD and ClinVar. This is active, ongoing science, not a solved problem.

## Why This Matters

Understanding variant types is prerequisite to selecting appropriate detection methods, interpreting variant effect predictions, and explaining genotype-phenotype relationships. An analyst who conflates indels with SNPs will use incorrect variant calling parameters; one who overlooks structural variants will miss large deletions causing Mendelian disease. The variant spectrum from SNPs to chromosome-scale CNVs represents the full range of natural diversity and disease-causing mutations — comprehensive genomic characterization requires methods appropriate for each variant class.

More broadly: the map from genotype to phenotype is not read by scanning a single type of variation. It requires accounting for the full spectrum — the point mutations that alter a single amino acid, the small indels that disrupt reading frames, the large deletions that eliminate entire exons, and the copy number amplifications that flood cells with the wrong protein. Each class requires its own detection methodology, its own quality metrics, and its own interpretation framework. The rest of this chapter builds the technical vocabulary you need to handle all of them.
