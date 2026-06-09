# Genome Organization

The human genome contains approximately 3.2 billion base pairs — enough, if printed in standard book format, to fill about 3,000 volumes of 1,000 pages each. Yet only 1.5% of it encodes protein. When this statistic was first being appreciated in the 1970s, the non-coding majority was casually labeled "junk DNA." We now know that this dismissal was hasty: the non-coding genome contains thousands of regulatory elements, structural domains, non-coding RNA genes, and repetitive elements whose contributions to biology range from architectural (centromeres, telomeres) to regulatory (enhancers, insulators) to evolutionary (mobile elements that have been co-opted as exons, promoters, and signaling molecules). Understanding genome organization means understanding what kinds of sequences exist, in what proportions, and how they contribute to function — prerequisite knowledge for interpreting genome-scale data and for genome engineering projects that must consider off-target effects across the full sequence landscape.

## Gene Density and the Protein-Coding Landscape

The human genome encodes approximately **~20,000–25,000 protein-coding genes**, occupying only **~1.5% of the genome** (exonic sequence). However, when intronic sequence is included, protein-coding genes span ~25–33% of the genome (introns are large in humans — the median intron length is ~3 kb, and *WWOX* has an intron >780 kb).

Gene density is highly non-uniform:
- **Gene-rich chromosomes**: chromosomes 19 and 22 have high gene density (~14 genes/Mb)
- **Gene-poor regions**: chromosomes 4 and 18 are relatively sparse; pericentromeric regions and constitutive heterochromatin have few functional genes
- **Gene deserts**: megabase-scale regions with no protein-coding genes, often containing long-range enhancers for developmental genes

Genome annotation (RefSeq, Ensembl/GENCODE) assigns features to each position. The latest GENCODE release (v44) annotates ~20,000 protein-coding genes plus ~20,000 lncRNA genes and thousands of pseudogenes, miRNA genes, snoRNA genes, and others.

## Introns, Exons, and the Pre-mRNA Landscape

**Exons** are the sequences that remain in the mature mRNA. The typical human gene has ~9 exons; median exon length is ~170 bp; median coding sequence is ~1,300 bp (433 codons → ~49 kDa average protein).

**Introns** are removed during splicing. The size distribution is extremely broad: most are 100 bp to 10 kb, but some (e.g., within *DMD*, the dystrophin gene at 2.3 Mb) are >100 kb. Large introns slow pre-mRNA maturation and increase the time between gene activation and protein production.

**Alternative splicing** means the relationship between gene count and protein count is not 1:1. Conservative estimates suggest ~95% of multi-exon human genes undergo alternative splicing, producing on average 2–3 distinct transcripts per locus. The true number of distinct protein-coding isoforms is likely >100,000.

## Repetitive Elements: The Repetitive Half of the Genome

~50% of the human genome is composed of repetitive elements. These fall into two broad classes:

### Transposable Elements

**Class I — Retrotransposons (copy-and-paste)**: transpose via an RNA intermediate that is reverse-transcribed and integrated at a new locus.

- **LINEs (Long Interspersed Nuclear Elements)**: ~6 kb, encode reverse transcriptase and endonuclease; autonomous. L1 (LINE-1) is the dominant human LINE (~17% of genome). Most L1 copies are truncated and inactive; ~80–100 copies remain actively transposable in humans.
- **SINEs (Short Interspersed Nuclear Elements)**: ~100–300 bp, non-autonomous (rely on LINE machinery for transposition). **Alu** elements are the most abundant SINE in humans (~1.1 million copies; ~11% of genome; ~280 bp each). Alu elements are primate-specific and can be exapted as regulatory elements, splice sites, or coding exons. SVA elements are younger composite SINEs.

**Class II — DNA transposons (cut-and-paste)**: encode transposase that excises and reinserts the element. In humans, DNA transposons are mostly extinct and immobile (~2% of genome). In *Drosophila* and plants, many are still active.

### Tandem Repeats

- **Satellite DNA**: highly repetitive (>10⁵ copies) arrays at centromeres and pericentromeric regions. Alpha-satellite (centromeric) repeats are ~171 bp monomers; highly variable between individuals.
- **Minisatellites**: 10–60 bp repeat units; hypervariable; used in DNA fingerprinting
- **Microsatellites (STRs, simple tandem repeats)**: 1–6 bp repeat units; ~700,000 loci in the human genome; highly variable due to replication slippage; used in population genetics, forensics, and linkage analysis. Expansion of certain STRs causes disease (Huntington's: CAG repeats in *HTT* > 36 copies; Fragile X: CGG repeats in *FMR1* > 200 copies).

## Non-Coding RNA Genes

Beyond protein-coding genes, the human genome encodes thousands of non-coding RNA genes:

| Type | Count (approx.) | Size | Function |
|---|---|---|---|
| rRNA | 400 (clustered at NOR loci) | 18S, 28S, 5.8S, 5S | Translation machinery |
| tRNA | ~500 genes | 73–93 nt | Amino acid adaptors |
| snRNA | ~100 active | 100–200 nt | Splicing (U1, U2, U4, U5, U6) |
| snoRNA | ~500 | 60–300 nt | rRNA modification (pseudouridylation, 2'-O-methylation) |
| miRNA | ~2,600 (miRbase v22) | ~22 nt (mature) | Post-transcriptional repression via RISC |
| lncRNA | ~20,000+ | >200 nt | Diverse; chromatin regulation, scaffolding, decoy |

**miRNAs** repress translation and/or promote mRNA degradation by binding complementary sequences in the 3' UTR. Each miRNA can target hundreds of mRNAs; each mRNA 3' UTR typically contains ~1–5 miRNA binding sites. The RISC (RNA-Induced Silencing Complex) centered on Argonaute proteins executes repression.

**lncRNAs** are functionally heterogeneous. Some (XIST, HOTAIR, NEAT1) are well characterized; most have no known function. A key challenge is distinguishing functional lncRNAs from transcriptional noise.

## Synteny: Comparative Genome Organization

**Synteny** refers to the conservation of gene order between homologous chromosomal regions across species. Human and mouse genomes (~80 Mb divergence time) share ~450 conserved synteny blocks of >100 kb. Human and *Drosophila* share almost no synteny at the chromosomal level, though individual genes are clearly homologous.

Synteny conservation highlights functionally constrained regions: genes that must remain co-regulated (sharing long-range regulatory elements) tend to be syntenic. Breaks in synteny — inversions, translocations — can disrupt gene regulation and are often associated with reproductive isolation between species.

## Why This Matters for Computational Biology

Genome organization is the reference context for every sequencing analysis. Short-read aligners (BWA-MEM, Bowtie2) must handle multi-mapping reads in repetitive regions; long-read sequencing (PacBio HiFi, Oxford Nanopore) is resolving previously collapsed repetitive regions. Gene annotation quality determines the accuracy of RNA-seq quantification. Transposable element activity contributes to somatic mosaicism and disease (L1 insertions in cancer; Alu-mediated deletions via non-allelic homologous recombination). In synthetic biology, the choice of genomic integration site for synthetic constructs requires knowing the local chromatin environment — inserting into heterochromatin silences the construct; inserting near an oncogene may activate it. Safe harbor loci (human *AAVS1*, *H11*; mouse *Rosa26*) are constitutively open chromatin sites used for reliable transgene expression.
