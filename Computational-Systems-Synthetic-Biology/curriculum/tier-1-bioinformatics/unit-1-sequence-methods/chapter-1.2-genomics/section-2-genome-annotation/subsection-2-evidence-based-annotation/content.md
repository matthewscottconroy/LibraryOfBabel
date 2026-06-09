# Evidence-Based Genome Annotation

Ab initio prediction, as the previous section described, can find genes in a naked genome sequence — but imperfectly. It misses small exons, gets splice boundaries wrong in complex regions, and produces a single isoform per locus when the actual gene may produce dozens. The reason for these failures is fundamental: the predictor is reasoning from statistical tendencies, from the general signature of coding sequence, but it has no knowledge of which specific sequences in this genome are actually transcribed.

The solution is to look. To sequence the RNA that the organism actually produces, under real biological conditions, and use those observations as direct evidence for where genes begin and end, which exons are included in which transcript, and which alternative isoforms exist. This is evidence-based annotation — and when you combine it with ab initio prediction and protein homology, the resulting gene models become substantially more accurate and complete than any single approach could achieve alone.

**Evidence-based annotation** incorporates experimental data — RNA-seq reads, protein homology, expressed sequence tags — to guide and validate gene models. It overcomes the key weaknesses of ab initio prediction by grounding gene structures in observed biological evidence. Modern genome annotation pipelines combine multiple evidence types for maximum accuracy and completeness.

## Types of Evidence

**Transcriptomic evidence** (RNA-seq):
- Direct observation of expressed sequences
- Identifies actual splice junctions used in the organism
- Can discover novel transcripts and isoforms
- Quality depends on developmental stages and conditions sampled

**Protein homology evidence**:
- Aligned proteins from closely related organisms indicate coding potential
- More conserved than nucleotide sequence; detects exons without RNA evidence
- Particularly valuable for genes expressed in inaccessible tissues or rare conditions

**EST/cDNA data**:
- Historically important; now largely superseded by RNA-seq
- cDNA libraries provide full-length transcript evidence

Each evidence type has its characteristic blind spots. RNA-seq misses genes that are not expressed in the sampled conditions — if you sequence liver RNA, you will miss testis-specific genes. Protein homology misses genes that are genuinely novel with no relatives in the database — lineage-specific innovations are common. Ab initio prediction misses poorly-signaled genes. The power of combining all three is that each compensates for the others' weaknesses: a gene with no RNA-seq coverage but strong protein homology can still be annotated accurately; a gene with no protein homology but strong RNA-seq signal can still be captured.

## RNA-seq-Based Annotation

### STAR + StringTie: Reference-Guided Assembly

```bash
# Step 1: Align RNA-seq to genome with STAR
STAR --genomeDir star_index/ \
     --readFilesIn R1.fastq.gz R2.fastq.gz \
     --readFilesCommand zcat \
     --outSAMtype BAM SortedByCoordinate \
     --outFileNamePrefix sample1_ \
     --runThreadN 8

# Step 2: Assemble transcripts with StringTie
stringtie sample1_Aligned.sortedByCoord.out.bam \
    -o sample1_transcripts.gtf \
    -G existing_annotation.gtf \
    -p 8

# Step 3: Merge transcriptomes from multiple samples
stringtie --merge -G existing_annotation.gtf \
    -o merged_transcriptome.gtf \
    sample1_transcripts.gtf sample2_transcripts.gtf sample3_transcripts.gtf
```

StringTie uses a network flow algorithm to assemble the most parsimonious set of transcripts consistent with the read alignments. The merged transcriptome contains both known and novel transcripts.

The power of the merge step is underappreciated. Any single RNA-seq sample will have unique coverage biases and will miss transcripts that are rare, lowly expressed, or cell-type-specific. By merging transcriptomes from multiple samples — ideally spanning diverse tissues, developmental stages, and conditions — you capture a much more complete view of the transcriptome. The Ensembl annotation for human uses hundreds of RNA-seq samples precisely because transcript discovery saturates slowly; new samples keep revealing new splice variants and novel transcripts even after thousands of experiments.

### Trinity: De Novo Transcriptome Assembly

When no reference genome is available, **Trinity** assembles transcripts directly from RNA-seq reads using de Bruijn graphs:

```bash
Trinity --seqType fq \
        --left R1.fastq.gz --right R2.fastq.gz \
        --max_memory 64G --CPU 16 \
        --output trinity_output/
```

Trinity output: a FASTA file of assembled transcripts. These can be used for:
- Protein prediction with TransDecoder (finds ORFs in assembled transcripts)
- Annotation via BLAST against known proteins
- Expression quantification with Salmon/kallisto

```bash
# Find ORFs in Trinity transcripts
TransDecoder.LongOrfs -t trinity_output/Trinity.fasta
TransDecoder.Predict -t trinity_output/Trinity.fasta
TransDecoder.Util.gtf_to_alignment_gff3.pl Trinity.fasta.transdecoder.gff3 \
    > predicted_cds.gff3
```

Trinity de novo assembly is the only option when you have no reference genome, and it has transformed non-model organism biology. Before Trinity, studying the transcriptome of a non-model organism required either a reference genome or Sanger-sequenced EST libraries. Trinity allows you to characterize the expressed gene content from RNA-seq alone — a capability that opened up thousands of ecologically and evolutionarily interesting organisms that would otherwise have been inaccessible.

## Protein Homology Integration

Aligning proteins from related species to the genome reveals exon positions even without RNA evidence:

```bash
# Using Miniprot: protein-to-genome aligner (faster than older tools)
miniprot -t 8 genome.fa proteins.faa > protein_alignments.paf

# Or using GenomeThreader
gth -genomic genome.fa -protein proteins.faa \
    -species arabidopsis -o gth_alignments.gff3
```

**Homology-based annotation**: conserved proteins from related species are aligned to the genome; aligned positions indicate exon locations. Particularly useful for genes with low expression in sampled conditions.

The key insight behind protein homology as evidence is that protein sequences evolve much more slowly than nucleotide sequences — even between species separated by hundreds of millions of years, conserved exons remain alignable at the protein level. A fish protein aligned to a mammalian genome will still land on the correct exons despite 400 million years of sequence divergence, because those exons encode amino acids that are functionally constrained. This makes protein homology uniquely valuable for detecting exons in genes that are expressed only in developmental windows not captured by RNA-seq.

## MAKER: Multi-Evidence Annotation

**MAKER** is a widely used pipeline that integrates all evidence types:

```bash
# MAKER configuration files
maker -CTL  # Generates maker_opts.ctl, maker_bopts.ctl, maker_exe.ctl

# Edit maker_opts.ctl:
# genome=genome.fa
# est=rnaseq_transcripts.fasta
# protein=related_proteins.faa
# est_gff=stringtie_merged.gff3
# augustus_species=closest_species

# Run MAKER
maker -base annotation genome.fa maker_opts.ctl
```

MAKER's pipeline:
1. Repeat masking (RepeatMasker)
2. EST/RNA-seq alignment (BLAST, GMAP)
3. Protein alignment (BLAST, Exonerate)
4. Ab initio prediction (AUGUSTUS, SNAP, GeneMark)
5. Evidence weighting: AED (Annotation Edit Distance) measures how well each model fits the evidence

**AED (Annotation Edit Distance)**: a quality metric ranging 0–1:
- AED = 0: gene model perfectly supported by evidence
- AED = 1: no evidence supports the gene model
- Threshold: typically keep models with AED ≤ 0.5

The AED score is a useful concept to internalize: it is not a p-value, not a fold-change, but a measure of how much you would need to edit the gene model to make it consistent with the evidence. A gene with AED = 0 is exactly what the RNA-seq and protein alignments say it should be. A gene with AED = 0.8 was mostly predicted ab initio and is poorly supported by experimental observation. When you use a MAKER annotation for functional analysis, the AED distribution tells you how much of the annotation you can trust.

## BRAKER2: Automated Pipeline

BRAKER2 automates the combination of RNA-seq, protein homology, and AUGUSTUS:

```bash
braker.pl \
    --genome=genome.fa \
    --bam=rnaseq.bam \
    --prot_seq=proteins.faa \
    --species=myspecies \
    --softmasking \
    --cores=16 \
    --workingdir=braker_output/
```

BRAKER2 uses RNA-seq to train AUGUSTUS on the target organism, then runs AUGUSTUS with both RNA-seq and protein homology as hints. This is particularly powerful for organisms where no trained AUGUSTUS model exists.

## Evaluating Annotation Quality

```bash
# BUSCO completeness assessment
busco -i annotation_proteins.faa -l embryophyta_odb10 \
      -m protein -o busco_annotation/ -c 16

# Count features in annotation
awk '$3 == "gene"' annotation.gff3 | wc -l   # Number of genes
awk '$3 == "mRNA"' annotation.gff3 | wc -l   # Number of transcripts
awk '$3 == "exon"' annotation.gff3 | wc -l   # Number of exons

# Compare gene counts to closely related species (sanity check)
```

Expected metrics for a good eukaryotic genome annotation:
- BUSCO completeness: > 90%
- Number of genes: consistent with related species (human: ~20,000 protein-coding)
- Average exons per gene: 8–10 for vertebrates, fewer for compact genomes
- Low AED fraction: > 60% of models with AED < 0.5

BUSCO completeness is the single most widely reported annotation quality metric, and for good reason: it is interpretable, it is comparable across species, and it is anchored to a biologically meaningful question (are the conserved core genes present?). An annotation with 30% BUSCO completeness is not just incomplete — it suggests a serious problem with the assembly, the annotation pipeline, or both. An annotation with 98% BUSCO completeness gives you confidence that the major functional repertoire is captured.

## Why This Matters

Evidence-based annotation is responsible for the high quality of well-annotated genome databases (Ensembl, NCBI RefSeq, Wormbase, FlyBase). The quality of these annotations directly affects all downstream analyses: differential expression analysis, variant effect prediction, ortholog identification, and pathway analysis all depend on correct exon boundaries and gene models. For non-model organisms, the annotation quality may be the primary bottleneck in using genomic data. Understanding how evidence is weighted and integrated — and what MAKER's AED score means — enables critical evaluation of genome annotations encountered in research.

Every time you download a GTF file from Ensembl and pipe it into your differential expression analysis, you are silently trusting thousands of annotation decisions made by the MAKER/BRAKER/GNOMON pipelines described here. The genes you call differentially expressed are only as accurate as the exon boundaries you used to count reads. Understanding how those boundaries were determined — and what evidence supports them — is not just academic; it is the prerequisite for knowing whether to trust your results.
