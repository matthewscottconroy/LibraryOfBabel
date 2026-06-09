# Pan-Genomes

For most of the genomics era, "the human genome" meant a single linear sequence — the reference genome assembled from a handful of individuals and represented as one path through 3.2 billion base pairs. This was an extraordinarily useful fiction. Having a single reference made alignment straightforward, variant calling tractable, and data sharing practical. But it was still a fiction, and the field has been reckoning with its consequences.

The human reference genome, GRCh38, does not contain sequences present in substantial fractions of the human population. It does not represent the genetic diversity of African populations well, since most of its sequence derives from European and Asian donors. It systematically excludes structural variants and insertions that are common in some ancestral backgrounds. When you align reads from a person of Yoruba ancestry to GRCh38, a meaningful fraction of their DNA has no home in the reference — it falls off the edge of the known sequence, or maps poorly, or is misattributed to a different location.

The pan-genome framework emerged as the answer. Instead of one reference, represent the genome of a species as the union of all sequences found in a large, diverse collection of individuals. Not a single linear sequence but a graph of all known paths — a structure that can represent every individual's genome as a specific route through the graph, capturing both shared and private sequences.

A **pan-genome** is the union of all genes found across all strains or individuals of a species. It recognizes a fundamental limitation of single-reference-genome biology: no single genome sequence captures the full genetic diversity of a species. Pan-genome analysis is particularly critical in microbiology (where horizontal gene transfer creates immense within-species diversity) but is increasingly important in plant and human genomics.

## Pan-Genome Structure

The pan-genome of a species is partitioned into:

**Core genome**: genes present in all (or nearly all) strains. These genes are essential for the organism's fundamental biology — core metabolic functions, DNA replication, ribosomal proteins.

**Accessory genome** (shell + cloud):
- **Shell**: genes present in most but not all strains (soft core; present in 95–99% of strains)
- **Cloud**: genes present in few strains (< 15%). Often strain-specific or recently acquired; includes pathogenicity islands, antibiotic resistance genes, phage-derived elements

**Unique genes (singletons)**: genes found in only one strain; often very recently acquired by horizontal gene transfer.

## Open vs. Closed Pan-Genomes

**Open pan-genome**: as more strains are sequenced, new genes continue to be discovered at a rate that doesn't saturate. The pan-genome size grows linearly (or super-linearly) with the number of genomes analyzed. Characteristic of species with extensive horizontal gene transfer (HGT) — *E. coli*, *Streptococcus*, *Staphylococcus*.

Heaps' law models pan-genome growth:

$$P(n) = \kappa \cdot n^\gamma$$

where $P(n)$ is the pan-genome size after sampling $n$ genomes, $\kappa$ is a constant, and $\gamma$ is the growth exponent:
- $\gamma > 0$: open pan-genome ($\gamma$ close to 1: very open)
- $\gamma \approx 0$: closed pan-genome

**Closed pan-genome**: adding more strains contributes few new genes. Characteristic of species with little HGT, small populations, or ecologically constrained niches.

The ecological logic behind open pan-genomes is intuitive once you see it. *E. coli* inhabits dozens of environmental niches — human gut, animal gut, soil, water, hospital surfaces — and each niche selects for different accessory genes. A pathogenic strain has toxin and invasin genes that its commensal relatives lack; a urinary tract pathogen has type 1 fimbriae adhesins; a STEC strain has the Shiga toxin prophage. These are accessory genes that confer competitive advantages in specific niches. Because *E. coli* can acquire these genes by horizontal transfer from distantly related organisms, its pan-genome is essentially unbounded — every new ecological niche explored is a potential source of new accessory genes.

## Bacterial Pan-Genome Tools

### Roary

**Roary** is the standard pan-genome pipeline for bacteria:

```bash
# Input: annotated GFF3 files from Prokka
roary -p 16 -f roary_output/ -e -mafft *.gff

# Outputs:
# core_gene_alignment.aln: alignment of core genes (for phylogenetics)
# gene_presence_absence.csv: presence/absence matrix
# pan_genome_reference.fa: representative sequences for all genes
# summary_statistics.txt: core/soft core/shell/cloud counts

# Visualize with R
# install.packages("ggplot2")
library(ggplot2)
pg <- read.csv("roary_output/gene_presence_absence.csv")
# heatmap of presence/absence across strains
```

### Panaroo

**Panaroo** addresses common artifacts in Roary (fragmented genes, annotation errors) with a graph-based approach:

```bash
panaroo -i annotations/*.gff -o panaroo_output/ \
    --clean-mode strict \
    -t 16 \
    --alignment core \
    --aligner mafft
```

Panaroo's graph representation allows it to:
- Merge fragmented genes (split by annotation errors)
- Resolve paralogs correctly
- Handle partial gene sequences at contig boundaries

## Human Pan-Genome

The human reference genome (GRCh38) is derived from a small number of individuals and fails to represent ~5–10% of sequences present in diverse human populations. The **Human Pangenome Reference Consortium (HPRC)** has assembled 47 high-quality haplotype-resolved genomes from diverse ancestries, revealing:

- ~300 Mb of sequences not present in GRCh38
- ~10,000–100,000 structural variants per genome
- Hundreds of gene-level copy number variants

**Graph genome representation**: instead of a linear reference, a graph genome encodes all known variation as a directed graph. Nodes = sequence segments; edges = connections (either reference or alternate paths).

Tools: **vg (variation graph)**, **PGGB (PanGenome Graph Builder)**:

```bash
# Build a pangenome graph with PGGB
pggb -i genomes.fasta -s 5000 -p 90 -n 47 -o pggb_output/ -t 16

# Map reads to variation graph
vg map -f reads.fastq.gz -x graph.xg -g graph.gcsa > aligned.gam

# Call variants in graph space
vg call graph.xg -s sample_name aligned.gam > variants.vcf
```

The 300 Mb of sequence absent from GRCh38 deserves emphasis. That is roughly 10% of the size of a haploid human genome — not a trivial omission. Much of this missing sequence is copy number variable, or present in certain ancestral backgrounds and absent from others, or embedded in medically important regions like the HLA locus (which is so polymorphic that representing it as a graph rather than a linear sequence is almost mandatory for accurate typing). When you perform a GWAS, an eQTL study, or a clinical diagnostic test using GRCh38 as your reference, you are missing these sequences. The pan-genome reference does not just expand the completeness of the reference — it changes the fundamental architecture of how we represent human genetic variation.

## Plant Pan-Genomes

Pan-genome thinking is revolutionizing plant genomics:

**Maize pan-genome**: only ~60% of genes are present in all 26 sequenced lines. 30–40% of genes are absent in any given line. This accessory genome includes genes for pest resistance, drought tolerance, and metabolic specialization.

**Rice pan-genome**: analysis of 66 diverse rice lines revealed a core genome of ~27,000 genes (present in > 90% of lines) and a dispensable genome of > 10,000 additional genes.

**Tomato pan-genome**: 725 wild and cultivated tomato accessions; identified core + accessory genes including novel disease-resistance genes absent from the reference.

The maize numbers are striking: nearly half the genes in a given maize line are absent from other lines. This is not a failure of annotation — it reflects genuine biological diversity. The genes in the accessory genome are real, they are expressed, and they encode functions that vary across the maize population. When plant breeders select for disease resistance or stress tolerance, they are often selecting for accessory genome genes that happen to be present in their breeding lines. The reference-based approach to maize genomics was, therefore, systematically missing a large part of the genetic diversity that breeders were actually working with.

## Functional Enrichment of Core vs. Accessory Genes

```python
import pandas as pd
from scipy import stats

# Load gene presence/absence matrix
presence = pd.read_csv("gene_presence_absence.csv", index_col=0)

# Classify genes
n_strains = presence.shape[1]
gene_freq = presence.notna().sum(axis=1) / n_strains

core_genes = gene_freq[gene_freq >= 0.99].index
accessory_genes = gene_freq[(gene_freq >= 0.15) & (gene_freq < 0.99)].index
cloud_genes = gene_freq[gene_freq < 0.15].index

print(f"Core: {len(core_genes)}, Accessory: {len(accessory_genes)}, Cloud: {len(cloud_genes)}")

# GO enrichment of core vs. accessory
# Core: enriched for housekeeping functions
# Accessory: enriched for mobile elements, toxin-antitoxin, antibiotic resistance
```

## Why This Matters

Single-reference-genome biology systematically misses between 5% (humans) and 40% (bacteria, plants) of the biologically relevant sequence diversity in a species. Pan-genome approaches capture this diversity, revealing genes responsible for clinically critical traits (antibiotic resistance, virulence), agricultural traits (disease resistance, yield), and evolutionary dynamics (HGT, adaptation). The shift from linear reference genomes to graph-based pan-genomes is one of the most significant current transitions in the field, with implications for all forms of genomic analysis — variant calling, expression quantification, and association studies — all of which improve in accuracy when performed against a pan-genome representation.

For synthetic biology, the pan-genome concept matters in a specific and practical way. When you choose a chassis organism for engineering, you are choosing one genome from a population with a pan-genome. If your chassis strain lacks accessory genes that are present in closely related strains — genes for stress tolerance, metal resistance, or metabolic flexibility — you might not need to engineer these capabilities from scratch. They might already exist in the pan-genome of your species, and horizontal transfer is a time-tested mechanism for importing them. Conversely, if your engineered chassis acquires mobile elements or accessory genes from environmental bacteria during fermentation, those acquisitions could disrupt your designed circuits. Understanding the pan-genome of your chassis organism is, in this sense, part of understanding the engineering substrate.
