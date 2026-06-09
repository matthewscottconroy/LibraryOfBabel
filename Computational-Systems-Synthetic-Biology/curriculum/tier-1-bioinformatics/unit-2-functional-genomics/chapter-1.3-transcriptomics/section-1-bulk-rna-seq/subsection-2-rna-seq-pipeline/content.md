# The RNA-seq Pipeline

Imagine holding a FASTQ file — a text file containing tens of millions of short, 150-nucleotide sequences, each one a fragment of some RNA molecule that existed in a cell. By itself, this file tells you nothing about which genes were active. The RNA-seq pipeline is the sequence of computational transformations that converts these raw reads into biologically interpretable numbers: a count matrix, where each entry tells you how many times a given gene was sampled. Every step involves substantive choices, and errors here propagate silently through every downstream analysis.

Once reads are quality-controlled and trimmed, the pipeline converts FASTQ files into a count matrix — the quantitative representation of gene expression across samples. Getting this right requires understanding not just what each tool does, but why it does it.

## Splice-Aware Alignment with STAR

Here is a problem that seems simple until you think about it: you have a 150 nt sequencing read from a human mRNA, and you want to find where in the 3-billion-base genome it came from. For genomic DNA, this is relatively straightforward. But mRNA has been spliced. A read that starts in exon 7 and ends in exon 8 spans an intron that might be 50,000 bases long in the genome. A standard aligner looking for a contiguous match will fail completely — it will either not find the read or map it to the wrong location.

Unlike genomic DNA, mRNA has been spliced: reads can span exon-exon junctions that are hundreds of kilobases apart in the genome. A standard aligner like BWA cannot handle this. **STAR** (Spliced Transcripts Alignment to a Reference) builds a compressed suffix array index of the genome and uses a maximal mappable prefix seed strategy to anchor reads before extending across splice junctions.

**Two-pass mode** is STAR's key feature for novel splice junction discovery. In the first pass, STAR aligns all reads and identifies splice junctions not in the annotation. In the second pass, these novel junctions are added to the index before re-aligning. This improves sensitivity for novel isoforms.

```bash
# STAR two-pass alignment
STAR --runMode alignReads \
     --genomeDir /path/to/star_index \
     --readFilesIn sample_R1.fastq.gz sample_R2.fastq.gz \
     --readFilesCommand zcat \
     --outSAMtype BAM SortedByCoordinate \
     --twopassMode Basic \
     --outSJtype Standard \
     --quantMode GeneCounts \
     --outFileNamePrefix results/sample_
```

A typical alignment rate for a human sample is 90–95% uniquely mapped reads. Rates below 70% warrant investigation: possible causes include wrong genome/annotation, rRNA contamination, or library preparation artifacts. A low mapping rate is one of the clearest signals that something went wrong upstream.

## Read Summarization with featureCounts

Alignment places each read somewhere in the genome. Summarization answers a different question: which gene does each aligned read belong to? This requires intersecting the genomic coordinates of each aligned read with a reference annotation (a GTF file listing the genomic coordinates of all exons and genes).

After alignment, reads must be assigned to genomic features (genes or exons). **featureCounts** (from the Subread package) intersects BAM alignments with a GTF annotation file and counts how many reads (or read pairs, for PE data) overlap each feature.

Key parameters:
- `-p` for paired-end data (counts fragments, not individual reads)
- `-s 1` or `-s 2` for stranded libraries (must match the library protocol)
- `-T` for multi-threading

The output is a tab-delimited count matrix with genes as rows and samples as columns. Alternative quantification tools (**kallisto**, **Salmon**) use pseudoalignment or quasi-mapping against the transcriptome rather than the genome, providing faster quantification with similar accuracy and including uncertainty in isoform assignment. Salmon in particular has become widely adopted for its speed and its ability to estimate per-transcript abundances while accounting for multi-mapping reads probabilistically.

## Expression Units: TPM, FPKM, and Raw Counts

Three main expression units are used in RNA-seq, and the choice matters critically for what analysis follows. Using the wrong unit for the wrong analysis is one of the most common mistakes in the field.

**RPKM/FPKM** (Reads/Fragments Per Kilobase of transcript per Million mapped reads) normalizes by both sequencing depth and gene length:

$$\text{FPKM}_g = \frac{C_g}{L_g \cdot N} \times 10^9$$

where $C_g$ = read count for gene $g$, $L_g$ = gene length in bp, $N$ = total mapped reads.

**TPM** (Transcripts Per Million) first normalizes by gene length, then scales so all values sum to $10^6$. TPM values are comparable between samples — the sum is always constant — unlike FPKM where the sum can vary. For this reason, TPM has largely replaced FPKM. When you see a gene expressed at 50 TPM in sample A and 48 TPM in sample B, you can directly interpret those numbers as proportions of the transcriptome.

**Raw counts** (integers from featureCounts) are the input required for differential expression tools like DESeq2 and edgeR. These tools incorporate normalization internally using methods designed for the statistical properties of count data. **Never use TPM or FPKM as input to DESeq2** — they are pre-normalized and violate the statistical model assumptions. This is not a minor technical quibble; inputting normalized values to DESeq2 produces invalid p-values and inflated false discovery rates.

## Normalization Methods

Even raw counts are not directly comparable across samples without normalization, because samples differ in total sequencing depth. If sample A has 30 million total reads and sample B has 50 million, a gene with 3,000 reads in A and 5,000 reads in B is expressed at the same level — not differentially expressed.

**TMM** (Trimmed Mean of M-values, used by edgeR) calculates a per-sample scaling factor by taking the weighted mean of log fold changes between each sample and a reference, after trimming the top and bottom percentiles of genes. This handles samples with very different total counts.

**DESeq2 median of ratios** normalization calculates a geometric mean expression for each gene across all samples, then computes the ratio of each sample's counts to this reference. The **size factor** for each sample is the median of these ratios across all genes:

$$s_j = \text{median}_g \left( \frac{K_{gj}}{\left(\prod_{i=1}^{m} K_{gi}\right)^{1/m}} \right)$$

This approach is robust to extreme outlier genes because the median is insensitive to a small number of highly expressed genes dominating the library. You might expect that simply dividing by total read counts would suffice. It turns out that if one gene is extremely highly expressed in one condition (a common scenario in, say, interferon-stimulated cells), normalizing by total counts would artificially scale down all other genes and create false-positive downregulation across the entire genome.

## Quality Metrics for Alignment

You have aligned your reads and counted them. But how do you know the experiment actually worked? Several quality metrics collectively diagnose whether the data is reliable:

- **% uniquely mapped reads**: Should be ≥80% for human RNA-seq; lower values indicate contamination, wrong genome, or technical issues.
- **Gene body coverage**: Calculated by RSeQC; a strong 3' bias (signal concentrated at 3' end) indicates RNA degradation or poly-A selection from low-quality RNA.
- **% reads in features**: The fraction of mapped reads assigned to annotated genes; 60–80% is typical for poly-A libraries.
- **Duplication rate**: ≤50% for whole-transcriptome; very high rates suggest low input.
- **rRNA contamination**: If ribo-depletion was used, check that rRNA read fraction is <1%.

MultiQC aggregates all these metrics across samples into a single QC dashboard for efficient review. Running MultiQC before proceeding to differential expression analysis is not optional — it is the checkpoint that catches experimental failures before they become expensive mistakes.

## Why This Matters

Every downstream biological conclusion — whether a gene is differentially expressed, which isoform is dominant, or how a pathway is regulated — depends on accurate alignment, appropriate expression unit selection, and correct normalization. Errors here propagate silently through the entire analysis pipeline. The pipeline is not just plumbing between your sequencer and your results; it is where the raw physics of sequencing gets translated into the language of gene regulation, and small errors in translation compound into large errors in interpretation.
