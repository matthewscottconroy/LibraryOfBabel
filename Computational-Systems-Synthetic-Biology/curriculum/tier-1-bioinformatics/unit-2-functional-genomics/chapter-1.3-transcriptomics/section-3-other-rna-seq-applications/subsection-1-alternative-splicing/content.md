# Alternative Splicing Analysis

Consider this number: the human genome encodes roughly 20,000 protein-coding genes, yet the human proteome contains hundreds of thousands of distinct protein variants. How? The answer is largely alternative splicing. The same gene — the same stretch of DNA — can produce fundamentally different proteins depending on which combination of exons gets included in the final mRNA. The neural cell adhesion molecule *NCAM* can be produced in at least 27 isoforms with distinct adhesion properties. The *Dscam* gene in *Drosophila* can theoretically generate 38,016 different mRNA combinations through mutually exclusive exon choices, giving each neuron a unique molecular identity that guides axon wiring. These are not minor variations — they are functionally distinct proteins with distinct interactors and distinct roles.

Pre-mRNA splicing is not a deterministic process — the same pre-mRNA can be spliced in multiple ways to produce different mature mRNA isoforms that encode different protein variants. **Alternative splicing** is estimated to affect >95% of multi-exon human genes, making it a major mechanism for proteome diversity. RNA-seq can capture this complexity, but requires specific experimental design and dedicated analysis tools.

## Types of Splicing Events

Five canonical types of alternative splicing events are recognized, and each has distinct biological logic:

1. **Exon skipping (ES)**: The most common type in mammals. An internal exon is excluded from the mature mRNA, producing a shorter isoform. Example: *Bcl-x* can skip exon 2 to produce the pro-apoptotic Bcl-xS isoform. The decision to skip this one exon determines whether the protein promotes cell survival or cell death.

2. **Intron retention (IR)**: An intron is not removed, producing a transcript with extra sequence. Common in plants and yeast; in mammals, often associated with gene regulation in neuronal and immune cells, and with pre-mRNA that is retained in the nucleus. Intron retention can function as a "hold" mechanism — keeping transcripts nuclear until a signal triggers their splicing and cytoplasmic export.

3. **Alternative 5' splice site (A5SS)**: The upstream (5') edge of an exon is shifted, using a different donor splice site. Results in a shorter or longer version of the exon.

4. **Alternative 3' splice site (A3SS)**: The downstream (3') edge of an exon is shifted, using a different acceptor splice site.

5. **Mutually exclusive exons (MXE)**: Two exons are never both included; one or the other is chosen. Example: *DSCAM* in Drosophila has mutually exclusive exons that can theoretically generate >38,000 different protein isoforms.

## The ψ (PSI) Metric: Percent Spliced In

To quantify splicing changes, the standard metric is **ψ (psi)**, the **percent spliced in** (PSI), which measures the fraction of transcripts that include a given exon:

$$\psi = \frac{\text{reads supporting inclusion}}{\text{reads supporting inclusion} + \text{reads supporting exclusion}}$$

For an exon skipping event, inclusion reads are those spanning the upstream exon-exon junction or the downstream exon-exon junction that include the cassette exon; exclusion reads span the junction from the upstream exon directly to the downstream exon, skipping the cassette.

ψ ranges from 0 (exon never included) to 1 (exon always included). A differential splicing event is reported as Δψ between conditions. A |Δψ| > 0.1 with FDR < 0.05 is a common threshold for biological relevance.

Note the difference between PSI and differential expression. A gene might have the same total transcript abundance in two conditions but dramatically different isoform usage. Standard DE analysis, which simply counts all reads from a gene regardless of their exon composition, would call this gene unchanged. Only splicing-specific analysis reveals the regulatory event. These are distinct biological phenomena — transcriptional regulation controls how much of a gene is expressed; splicing regulation controls what kind of protein is made from that expression.

## rMATS: Differential Splicing from Short Reads

**rMATS** (replicate Multivariate Analysis of Transcript Splicing) is the standard tool for detecting differential alternative splicing between two conditions using paired-end RNA-seq data. It requires stranded libraries and high sequencing depth (≥100M reads per sample for reliable junction coverage).

rMATS counts reads for each of the five splicing event types from the BAM files, models ψ values using a beta distribution, and tests for differential ψ between conditions using a likelihood ratio test. The key output includes:

- Splicing event coordinates
- ψ estimates for each sample and condition
- IncLevel and SkipLevel counts
- P-value and FDR

```bash
rmats.py --b1 control_bam_list.txt \
         --b2 treatment_bam_list.txt \
         --gtf annotation.gtf \
         --od output_dir/ \
         -t paired \
         --readLength 150 \
         --nthread 8
```

The depth requirement (≥100M reads) explains why splicing analysis was not routinely done in early RNA-seq studies — standard experiments were designed for differential expression, not splicing. A dedicated splicing experiment needs to be designed from the start with sufficient depth to see junction-spanning reads at low-abundance exons.

## MAJIQ: De Novo Splicing Graph

**MAJIQ** (Modeling Alternative Junction Inclusion Quantification) takes a de novo approach: rather than testing pre-defined event categories, it builds a **splicing graph** from the data, identifying all observed splice junctions (including annotated and novel ones). It then defines **Local Splicing Variations (LSVs)** — complex splicing patterns involving multiple junctions. This makes MAJIQ more sensitive to complex and novel splicing events but requires more computational resources.

You might expect that using the known annotation — testing only the five canonical event types — would be sufficient. It turns out that many disease-relevant splicing events involve non-canonical or compound events not easily described by the five canonical categories. A single mutation can create a novel splice site that adds a new micro-exon or creates a cryptic splice acceptor deep within an intron. MAJIQ's de novo approach finds these events where rMATS would miss them.

## Long-Read Isoform Sequencing

Short-read data can only infer isoforms indirectly from junction-spanning reads and never observes a complete isoform in one read. If a gene has five alternative exons, short reads tell you about each junction independently, but cannot tell you which combinations of exons co-occur in the same molecule. **Long-read sequencing** (PacBio HiFi reads up to 20 kb; Oxford Nanopore reads up to hundreds of kb) can sequence full-length mRNA transcripts in a single read, directly identifying which combination of exons constitutes each isoform.

**PacBio Iso-seq** and **Nanopore direct RNA sequencing** have revealed that the true isoform diversity of human genes is substantially greater than previously appreciated — including many tissue-specific isoforms not present in current annotations. Tools like **FLAMES** and **TALON** analyze long-read transcript data to characterize novel isoforms.

The combination of short-read data for quantification (high depth, statistically robust) and long-read data for isoform discovery (full-length structure, lower depth) is becoming the gold standard for comprehensive splicing analysis.

## Why This Matters

Alternative splicing creates a proteome far richer than the ~20,000 human protein-coding genes would suggest. Beyond basic biology, it has direct clinical relevance. Hundreds of disease-causing mutations disrupt splice sites or splicing regulatory elements (exonic splicing enhancers, silencers), altering isoform ratios in ways that produce disease. Spinal muscular atrophy is caused by reduced inclusion of exon 7 in the *SMN2* gene; the clinically approved drug nusinersen corrects this splicing defect using an antisense oligonucleotide. Splicing analysis is not just an academic exercise — it is a direct path to understanding and correcting genetic disease mechanisms.
