# Microbial Genomics

There is a thought experiment that illuminates the strangeness of microbial genomes: take two strains of *E. coli*. One is the familiar K-12 laboratory strain that has been used in thousands of experiments since the 1920s. The other is the O157:H7 strain that causes severe hemorrhagic colitis and kidney failure. They share a common ancestor less than five million years ago — a geological eyeblink — and yet one is harmless and one can kill you. The difference is not primarily in the core genes they share. The difference is in the accessory genome: pathogenicity islands, toxin genes, and adhesion factors that were acquired horizontally, often in large blocks, from entirely unrelated organisms. Microbial genomics is the discipline that makes sense of this kind of genomic diversity.

Microbial genomes are compact, efficient, and highly dynamic. Bacteria pack more function per megabase than any eukaryote: their genomes are gene-dense, their regulatory elements are small, and horizontal gene transfer allows rapid acquisition of entirely new metabolic capabilities. Microbial genomics is the foundation of metagenomics, comparative genomics, and metabolic engineering — understanding how bacterial genomes are organized and how they change is essential for both interpreting sequencing data and designing synthetic biology chassis.

## Bacterial Genome Architecture

The typical bacterial genome is:
- **Size**: 1–10 Mb (median ~4 Mb for free-living bacteria; ~0.5 Mb for obligate intracellular parasites)
- **Single circular chromosome** (with some exceptions: *Vibrio cholerae* has two chromosomes; some bacteria have linear chromosomes)
- **Gene density**: ~85–95% coding (vs. ~25% in humans including introns); ~1 gene per kb
- **Polycistronic mRNAs**: multiple functionally related genes are transcribed as a single mRNA from an operon
- **No introns** in protein-coding genes (rare exceptions in some archaea and group I/II self-splicing introns in some bacteria)

Gene order in bacteria is not random. Genes encoding functions that must be expressed coordinately are co-localized in **operons**. The lac operon (*lacZYA*) is the paradigm: *lacZ* (β-galactosidase), *lacY* (permease), and *lacA* (transacetylase) are co-regulated by the lac repressor (LacI) and catabolite activator protein (CAP/CRP), and co-transcribed as a single polycistronic mRNA ~5.4 kb long.

## Plasmids: Extrachromosomal Genetic Elements

**Plasmids** are circular (usually) extrachromosomal DNA elements, typically 1 kb to >100 kb, that replicate autonomously. Key features:

- **Origin of replication**: each plasmid has its own ori; copy number is determined by the replication control system
  - **High copy**: pUC (500–700 copies/cell), pBR322 (~15–20 copies/cell)
  - **Low copy**: F plasmid (1–2 copies/cell)
- **Selectable marker**: typically antibiotic resistance (AmpR, KanR, CmR)
- **Incompatibility groups**: two plasmids with the same replication machinery cannot coexist stably in the same cell; they compete for limiting replication factors and one is stochastically lost during division

Plasmid stability affects synthetic biology circuit design: high-copy plasmids impose greater metabolic burden but ensure high gene dosage; low-copy or chromosomal integration provides stable, lower-expression cassettes.

## Sigma Factors as Global Transcriptional Regulators

Unlike eukaryotes, where hundreds of sequence-specific transcription factors control individual genes, bacteria use **alternative sigma factors** to reprogram transcription globally in response to stress or developmental cues. A single sigma factor swap changes thousands of promoter specificities simultaneously:

| Condition | Alternative σ | Regulon size (*E. coli*) |
|---|---|---|
| Heat shock | σ32 (rpoH) | ~100 genes |
| Stationary phase | σ38 (rpoS) | ~500 genes |
| Flagella | σ28 (fliA) | ~40 genes |
| Nitrogen limitation | σ54 (rpoN) | ~100 genes |

The relative concentrations of different sigma factors in the cell reflect the cell's physiological state and constitute a major regulatory layer. This is computationally tractable: sigma factor binding energies to promoter sequences can be calculated from position weight matrices, and sigma factor competition for core RNAP has been modeled as a resource allocation problem.

## Horizontal Gene Transfer and the Pangenome

The **pangenome** concept captures the true scope of genetic diversity within a species:
- **Core genome**: genes present in all strains of a species (~3,100 genes for *E. coli*)
- **Accessory genome**: genes present in some but not all strains (pathogenicity islands, phage remnants, novel metabolic genes)
- **Unique genes**: genes found in only one strain

*E. coli* has a pangenome of >80,000 genes — the core genome of ~3,100 represents only a fraction of any single strain's ~4,400 genes. This extraordinary diversity is maintained by **horizontal gene transfer (HGT)**: transformation, transduction, and conjugation continuously shuffle genes between strains (addressed more fully in the evolutionary biology section).

**Genomic islands** — large (10–200 kb) insertions with atypical nucleotide composition (GC%, codon usage) relative to the rest of the chromosome — are evidence of recent HGT. They are often flanked by tRNA genes (insertion sites for phage/integrating elements) and repeat sequences. Pathogenicity islands (PAIs) in *Salmonella*, *E. coli* O157:H7, and others carry virulence genes acquired by HGT.

## Comparative Microbial Genomics

Comparing multiple genomes from the same species or genus reveals evolutionary dynamics:

**Genome synteny**: Gene order is often conserved within genera but rearranged between genera. Inversions within the chromosome (frequently about the replication axis) are the most common rearrangements; they can be detected computationally from dotplot comparisons of two genome sequences.

**Ortholog inference**: Bidirectional best hits (BBH) or more sophisticated methods (OrthoFinder, DIAMOND+clustering) identify orthologous genes across genomes. Ortholog tables are the basis for comparative function assignment and phylogenetics.

**Core genome phylogenetics**: Building trees from concatenated alignments of core genes is more robust than single-gene phylogenetics and is the standard approach for bacterial species trees.

## Metagenomics: Reading Microbial Community Genomes

In environmental and clinical microbiology, DNA is extracted from community samples (soil, gut, ocean) and sequenced — producing a mixture of genomes from all organisms present. **Metagenomics** analysis involves:

1. **Quality filtering and host decontamination**: Remove human reads (for clinical samples) using reference alignment
2. **Assembly**: MEGAHIT or metaSPAdes assembles reads into contigs
3. **Binning**: Group contigs into putative genomes (MAGs — Metagenome-Assembled Genomes) using tetranucleotide frequency and coverage (MetaBAT2, CONCOCT)
4. **Quality assessment**: CheckM estimates completeness and contamination from marker gene content
5. **Annotation**: Prodigal calls ORFs; KEGG/COG databases annotate function

16S rRNA amplicon sequencing is a lower-cost alternative for taxonomic profiling only: the hypervariable V3-V4 region (~400 bp) is amplified and sequenced; OTU/ASV clustering (QIIME2, DADA2) assigns taxonomic identity.

## Why This Matters for Computational Biology

Microbial genomics is the primary data type for metagenomics, comparative genomics, and metabolic engineering. The tools of genome assembly, annotation, and comparison are the core of microbial bioinformatics. Understanding operon structure is essential for designing multi-gene synthetic constructs — polycistronic mRNAs are an efficient way to express multiple proteins in bacteria, but ribosome reinitiation efficiency at internal start codons must be engineered carefully. The pangenome concept matters for selecting representative strains for experiments and for interpreting strain-specific phenotypes. Antibiotic resistance spread via HGT is a critical public health challenge modeled computationally using phylogenetics and network epidemiology tools.
