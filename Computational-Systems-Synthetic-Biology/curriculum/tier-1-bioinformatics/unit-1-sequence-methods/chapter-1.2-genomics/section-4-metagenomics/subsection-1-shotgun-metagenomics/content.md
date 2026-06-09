# Shotgun Metagenomics

In the early days of microbiology, microbiologists studied microorganisms by growing them in culture — on plates, in broth, under controlled conditions. The method was enormously productive. It produced germ theory, antibiotics, and our fundamental understanding of microbial metabolism. But it had a fundamental limitation that was only recognized when sequencing became cheap: it only worked for the microorganisms that could be grown in the lab.

Estimates now suggest that more than 99% of microbial species in most environments cannot be cultured under standard laboratory conditions. The bacteria and archaea that dominate ocean water, deep soil, human mucosa, and hydrothermal vents have evolved to live in conditions — chemical gradients, syntrophic partnerships, specific nutrient flows — that cannot be replicated in a flask. For a century of microbiology, these organisms were invisible.

Shotgun metagenomics is what changed this. By sequencing all the DNA present in an environmental sample directly — without any culturing step, without amplifying a specific marker gene — it can characterize every organism in a community simultaneously. A gram of human gut feces contains ~10¹¹ microbial cells from hundreds of species; shotgun metagenomics provides taxonomic composition, functional potential, and even genome-scale information for the entire community in a single experiment.

**Shotgun metagenomics** sequences all DNA present in an environmental sample without prior culturing or marker gene amplification.

## Why Shotgun Over 16S?

16S amplicon sequencing (covered in the next section) targets a single marker gene and provides taxonomic information only. Shotgun metagenomics has several advantages:
- **Functional annotation**: identifies metabolic pathway genes, not just taxonomy
- **Strain-level resolution**: strain-specific markers can differentiate strains within species
- **Novel organism discovery**: no primer bias; can detect organisms with divergent 16S
- **Viral and eukaryotic metagenomes**: not limited to bacteria/archaea

Disadvantages: higher cost, more computational complexity, requires careful host decontamination.

The functional advantage deserves emphasis. A 16S study might tell you that a disease state is associated with increased abundance of Bacteroidetes and decreased Firmicutes. This is interesting but mechanistically opaque — which metabolic functions are changing? Shotgun metagenomics can answer this: it reveals which pathway genes are enriched, which biosynthetic gene clusters are present, and which virulence factors are encoded in the community. The difference between taxonomic composition and functional potential is the difference between knowing who is in the room and knowing what they are doing.

## Taxonomic Profiling

### Kraken2: k-mer Classification

**Kraken2** assigns reads to taxa by finding the lowest common ancestor (LCA) of all taxa whose genomes contain each k-mer in the read:

```bash
# Build Kraken2 database (pre-built databases available)
kraken2-build --standard --db kraken2_db --threads 16

# Classify reads
kraken2 --db kraken2_db \
        --paired R1.fastq.gz R2.fastq.gz \
        --output kraken_output.txt \
        --report kraken_report.txt \
        --threads 16

# Estimate relative abundances (adjusts for genome size differences)
bracken -d kraken2_db \
        -i kraken_report.txt \
        -o bracken_abundances.txt \
        -l S \  # Species level
        -t 10   # Minimum reads threshold
```

Kraken2 output: each read assigned to a taxon (or "unclassified"). Bracken re-estimates species abundance by distributing reads assigned to higher taxa.

**Confidence threshold**: the `-confidence` flag (0–1) requires a minimum fraction of k-mers to support the classification. Higher confidence = fewer false positives but more unclassified reads.

### MetaPhlAn4: Marker Gene Profiling

**MetaPhlAn4** uses a curated database of species-specific marker genes:

```bash
metaphlan R1.fastq.gz,R2.fastq.gz \
    --input_type fastq \
    --output_file metaphlan_profile.txt \
    --nproc 16 \
    --bowtie2db metaphlan_db/ \
    --index mpa_vJun23_CHOCOPhlAnSGB_202307

# Merge multiple samples
merge_metaphlan_tables.py sample1_profile.txt sample2_profile.txt \
    > merged_profiles.txt
```

MetaPhlAn profiles:
- More specific than Kraken2 (requires multiple species-specific markers)
- Lower sensitivity for novel species or rare organisms
- Provides relative abundance in percentage, not read counts

Kraken2 and MetaPhlAn4 make different tradeoffs that reflect their different designs. Kraken2 is fast and sensitive because it uses all k-mers from all genomes in its database — it will find matches even for very divergent organisms. MetaPhlAn4 is more specific because it restricts its analysis to marker genes that are truly specific to known species. For well-characterized communities with good database coverage (human gut, mouse gut), MetaPhlAn4's specificity is a virtue. For environmental samples with many novel organisms (soil, ocean), Kraken2's sensitivity may be preferable, though its false positive rate is higher.

## Functional Profiling

### HUMAnN3: Pathway Abundance

**HUMAnN3** identifies which metabolic pathways are present and estimates their relative activity:

```bash
humann --input reads.fastq.gz \
       --output humann_output/ \
       --nucleotide-database chocophlan/ \
       --protein-database uniref/ \
       --threads 16

# Outputs:
# *_genefamilies.tsv: UniRef90 gene family abundances (RPK)
# *_pathabundance.tsv: MetaCyc pathway abundances
# *_pathcoverage.tsv: pathway coverage (0-1, fraction of genes present)

# Normalize to relative abundance
humann_renorm_table --input sample_pathabundance.tsv \
    --output sample_pathabundance_relab.tsv \
    --units relab
```

HUMAnN3 pipeline:
1. Taxonomic profiling (MetaPhlAn4)
2. Nucleotide-level search against species-specific pangenomes
3. Translated search against UniRef90 for unclassified reads
4. Map gene families to MetaCyc pathways

## Metagenome-Assembled Genomes (MAGs)

From a metagenomic dataset, individual microbial genomes can be assembled and binned:

```bash
# Step 1: Assembly
megahit -1 R1.fastq.gz -2 R2.fastq.gz \
        -o megahit_assembly/ -t 16 \
        --min-contig-len 1000

# Step 2: Map reads back to assembly for coverage
bowtie2-build megahit_assembly/final.contigs.fa assembly_index
bowtie2 -x assembly_index -1 R1.fastq.gz -2 R2.fastq.gz \
    | samtools sort -o assembly_reads.bam
samtools index assembly_reads.bam

# Step 3: Bin contigs into MAGs
# MetaBAT2: uses tetranucleotide frequencies + coverage
jgi_summarize_bam_contig_depths --outputDepth depth.txt assembly_reads.bam
metabat2 -i megahit_assembly/final.contigs.fa \
         -a depth.txt \
         -o bins/bin -t 16

# Step 4: Quality assessment
checkm lineage_wf bins/ checkm_output/ -t 16 -x fa
# Reports: completeness, contamination, strain heterogeneity per bin
```

**MAG quality thresholds** (MIMAG standards):
- High-quality: completeness > 90%, contamination < 5%
- Medium-quality: completeness ≥ 50%, contamination < 10%
- Low-quality: completeness < 50%

MAG binning is remarkable: it reconstructs complete microbial genomes from a mixed community without ever culturing a single cell. The key insight is that contigs originating from the same organism share two properties: similar tetranucleotide frequency (a function of the organism's codon usage and genome composition) and similar coverage depth (all contigs from a given organism should be covered proportionally to that organism's abundance in the sample). MetaBAT2 clusters contigs using both signals simultaneously, producing bins that often correspond to single organisms. CheckM then evaluates completeness (are the universal single-copy marker genes present?) and contamination (are there duplicate copies suggesting bin contamination with a second organism?).

## Statistical Analysis

```python
import pandas as pd
from scipy import stats
import numpy as np

# Load taxonomic profile (samples as columns, taxa as rows)
profiles = pd.read_csv("merged_profiles.txt", sep='\t', index_col=0)

# Alpha diversity: Shannon index
def shannon(row):
    p = row[row > 0] / row.sum()
    return -(p * np.log(p)).sum()

alpha_div = profiles.apply(shannon, axis=0)

# Beta diversity: Bray-Curtis dissimilarity
from sklearn.metrics import pairwise_distances
bc_matrix = pairwise_distances(profiles.T, metric='braycurtis')

# Ordination (PCoA)
from sklearn.manifold import MDS
mds = MDS(n_components=2, dissimilarity='precomputed', random_state=42)
coords = mds.fit_transform(bc_matrix)
```

## Host Decontamination

For human microbiome studies, reads mapping to the human genome must be removed:

```bash
bowtie2 -x human_genome_index \
    -1 R1.fastq.gz -2 R2.fastq.gz \
    --un-conc-gz non_human_R%.fastq.gz \
    -S /dev/null --threads 16
# Use non_human_R1/R2.fastq.gz for metagenomic analysis
```

## Why This Matters

Shotgun metagenomics has transformed our understanding of the microbiome in health and disease, environmental microbiology, and biotechnology. It revealed that > 99% of environmental microorganisms cannot be cultured, that the human gut harbors > 500 microbial species encoding > 3 million unique genes, and that microbial community composition is associated with conditions from obesity to cancer to neurological disease. For biotechnology, metagenomics is a discovery engine for novel enzymes (polymerases, proteases, biosynthetic gene clusters) from environmental samples. Understanding the pipeline — from taxonomic profiling to MAG assembly to functional annotation — is essential for interpreting metagenomic data in any applied context.

The discovery angle is worth dwelling on. The first thermostable DNA polymerase used for PCR was isolated from a hot spring bacterium, Thermus aquaticus — a discovery that required culturing. But the explosion of novel enzymes discovered by metagenomic mining has no such requirement. Functional screens of metagenomic libraries, or computational identification of biosynthetic gene clusters in MAGs from novel environments, have produced novel antibiotics, novel restriction enzymes, novel CRISPR systems (Cas9 was not the first — it was just the one that worked best, identified from a comparative analysis of bacterial defense systems across hundreds of sequenced genomes), and countless industrial enzymes. The uncultured majority of the microbial world is, among other things, a library of molecular tools waiting to be discovered.
