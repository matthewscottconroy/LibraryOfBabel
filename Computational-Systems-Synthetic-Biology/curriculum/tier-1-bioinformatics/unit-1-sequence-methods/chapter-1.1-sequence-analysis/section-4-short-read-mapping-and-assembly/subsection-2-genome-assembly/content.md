# Genome Assembly

Genome assembly is the computational process of reconstructing the original genomic sequence from millions of short, overlapping sequencing reads. Unlike read mapping (which requires a reference), assembly operates de novo — it must infer the complete sequence from overlapping fragment evidence alone. Assembly is among the most computationally challenging problems in bioinformatics.

The scale of the achievement represented by a complete genome assembly is easy to underestimate. The first human reference genome took a decade of coordinated international effort and several hundred million dollars to assemble. Today, a reasonable human genome assembly can be produced in a few days for a few hundred dollars, using algorithms that had not been invented when the Human Genome Project began. The dramatic improvement came not from faster computers alone, but from new sequencing technologies — especially PacBio HiFi and Oxford Nanopore — that produce reads long enough to span the repetitive regions that defeat short-read assemblers. Understanding assembly helps you appreciate both the extraordinary quality of modern genomes and the real limitations that remain.

## The Assembly Problem

Given $N$ reads of length $L$ with average depth of coverage $C$ over a genome of size $G$:

$$C = \frac{N \cdot L}{G}$$

At 30× coverage of the human genome ($G = 3.2$ Gb) with 150 bp reads: $N = 3.2 \times 10^9 \times 30 / 150 = 640$ million reads.

The challenge: reconstruct the genome from these overlapping fragments. Complications include:
- **Repetitive regions**: any repeat longer than the read length creates ambiguity in assembly
- **Sequencing errors**: ~0.1% error rate in Illumina reads
- **Heterozygosity**: diploid genomes have two haplotypes; assemblers must handle or phase them
- **Coverage non-uniformity**: GC-rich and GC-poor regions may be under-sequenced

## de Bruijn Graph Assembly

The dominant algorithm for short-read assembly is the **de Bruijn graph** approach.

### K-mers and the Graph

1. Break every read into overlapping **k-mers** (substrings of length $k$)
2. Create a graph where **nodes** are $(k-1)$-mers and **edges** are k-mers
3. Each k-mer represents a directed edge from its $(k-1)$-mer prefix to its $(k-1)$-mer suffix
4. Find an **Eulerian path** (visiting every edge exactly once) through the graph

**Example** with k = 3 and reads `ACGTCA`, `CGTCAG`:

K-mers: ACG, CGT, GTC, TCA, CAG

Nodes (2-mers): AC, CG, GT, TC, CA, AG

Edges:
- ACG: AC → CG
- CGT: CG → GT
- GTC: GT → TC
- TCA: TC → CA
- CAG: CA → AG

Eulerian path: AC → CG → GT → TC → CA → AG, spelling `ACGTCAG`

### Why Eulerian Not Hamiltonian?

An **Eulerian path** (visiting every edge once) can be found in polynomial time $O(E)$ using Hierholzer's algorithm. A **Hamiltonian path** (visiting every node once) is NP-hard. By representing reads as edges rather than nodes, de Bruijn graphs convert the assembly problem from Hamiltonian to Eulerian — a critical algorithmic insight.

This reformulation is one of the most elegant ideas in computational biology. The genome assembly problem seems intractable at first glance — you are trying to find the path through a massive graph that visits each read exactly once (a Hamiltonian path, which is NP-hard). By transforming reads into edges and (k-1)-mers into nodes, you convert the problem to finding an Eulerian path — which is solvable in linear time. The same sequence data, a different graph representation, and suddenly the problem is tractable.

### K-mer Choice

The choice of $k$ critically affects assembly:
- **Small k**: better connectivity (more paths in graph) but more ambiguity from repeats (more bubbles)
- **Large k**: better resolution of repeats but requires higher coverage and long reads

Tools like SPAdes automatically use multiple k values and combine results.

### K-mer Frequency and Error Correction

```python
import collections

def count_kmers(reads, k):
    kmers = collections.Counter()
    for read in reads:
        for i in range(len(read) - k + 1):
            kmers[read[i:i+k]] += 1
    return kmers

# K-mer frequency histogram
kmers = count_kmers(all_reads, k=21)
# Plot: x = frequency, y = number of k-mers with that frequency
# Error k-mers appear at low frequency (1-3x)
# True genomic k-mers appear around coverage peak
```

K-mers with frequency 1 are likely sequencing errors. Error correction (removing/correcting low-frequency k-mers) before assembly dramatically improves quality.

## Practical Assembly Tools

### SPAdes (Short-read, Versatile)

SPAdes is the most widely used short-read assembler:

```bash
# Standard WGS assembly
spades.py -1 reads_R1.fastq.gz -2 reads_R2.fastq.gz \
          -o spades_output/ -t 16 -m 64

# Metagenome assembly
spades.py --meta -1 R1.fastq -2 R2.fastq -o meta_output/

# RNA-seq assembly (de novo transcriptome)
spades.py --rna -1 R1.fastq -2 R2.fastq -o rnaspades_output/
```

SPAdes uses multiple k values (21, 33, 55, 77 by default) and a hybrid approach combining de Bruijn graphs with read-pair information.

### Hifiasm (Long-read HiFi)

For PacBio HiFi reads (10–25 kb, < 0.1% error):

```bash
hifiasm -o assembly -t 16 hifi_reads.fastq.gz
# Outputs: assembly.bp.hap1.p_ctg.gfa, assembly.bp.hap2.p_ctg.gfa (phased)

# Convert GFA to FASTA
awk '/^S/{print ">"$2; print $3}' assembly.bp.p_ctg.gfa > assembly.fasta
```

Hifiasm produces near-complete chromosome-scale assemblies with a diploid-aware algorithm that phases haplotypes using heterozygous variant information.

### Flye (Nanopore and PacBio CLR)

```bash
flye --nano-raw reads.fastq.gz --out-dir flye_output/ --threads 16

# For HiFi reads
flye --pacbio-hifi reads.fastq.gz --out-dir flye_hifi/ --threads 16
```

## Assembly Quality Metrics

### N50

The **N50** is the most commonly reported assembly quality metric. Sort all contigs by length (longest first). Walk down the list accumulating total length. The N50 is the length of the contig at which the cumulative sum reaches 50% of the total assembly length.

**Interpretation**: larger N50 = more contiguous assembly. An N50 of 1 Mb means half the assembly is in contigs $\geq$ 1 Mb.

**NG50**: same as N50 but the 50% is calculated relative to the estimated genome size (not total assembly length). More useful when assembly is incomplete.

### BUSCO

**BUSCO (Benchmarking Universal Single-Copy Orthologs)** measures assembly completeness by searching for expected single-copy genes:

```bash
busco -i assembly.fasta -l vertebrata_odb10 -o busco_output -m genome -c 16
# Reports: Complete (C), Fragmented (F), Missing (M) BUSCOs
# Good assembly: >95% complete BUSCOs
```

A BUSCO score of 98.5% (C) means 98.5% of expected conserved genes were found intact.

### Assembly Statistics

```bash
# Using seqkit
seqkit stats -a assembly.fasta
# Output: N50, L50, max contig length, GC%, total bases

# Using assembly-stats
assembly-stats assembly.fasta
```

**Example good human assembly stats**:
- Contig N50: 50–150 Mb (chromosome-scale with HiFi + Hi-C)
- BUSCO: > 95% complete
- Total size: 3.0–3.2 Gb
- GC content: ~41%

## Hybrid Assembly

Combining technologies leverages complementary strengths:

| Technology | Strength | Weakness |
|-----------|----------|---------|
| Illumina short reads | High accuracy (>99.9%) | Short reads (150 bp) |
| PacBio HiFi | Long reads (15–25 kb), high accuracy | Lower throughput, higher cost |
| Oxford Nanopore | Ultra-long reads (up to Mb) | Higher error rate (5–15%) |
| Hi-C | Chromosome-scale scaffolding | Does not improve local accuracy |

```bash
# Polish a Flye nanopore assembly with Illumina reads
medaka_consensus -i nanopore.fastq.gz -d flye_assembly.fasta -o polished/
# Then short-read polish with Pilon:
pilon --genome polished/consensus.fasta --frags illumina.bam \
      --output pilon_polished --threads 8
```

## Why This Matters

Genome assembly is the foundation of reference genome construction — the reference sequences used by all downstream analyses. The quality of a reference genome directly limits all subsequent analyses: a fragmented assembly with many gaps misses genes, misrepresents repeat distributions, and makes structural variant detection impossible. The transition from short-read assemblies (N50 of kilobases) to long-read HiFi assemblies (N50 of tens of megabases, near-complete chromosomes) has transformed human and model organism genomics. Understanding de Bruijn graphs and assembly quality metrics enables critical evaluation of genome assemblies used in research.
