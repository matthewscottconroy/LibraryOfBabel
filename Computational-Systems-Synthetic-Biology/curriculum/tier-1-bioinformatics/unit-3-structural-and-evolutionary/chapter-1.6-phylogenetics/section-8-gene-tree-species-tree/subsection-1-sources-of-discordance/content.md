# Sources of Gene Tree/Species Tree Discordance

Here is a fact that was deeply surprising when it emerged from the first wave of comparative genomics: if you pick any two genes from the same set of species and build phylogenetic trees from each, you will often get different trees. Not slightly different — sometimes completely contradictory. The gene for cytochrome b might say that species A and B are sisters, while the BRCA1 ortholog says species A and C are sisters, and the ribosomal protein gene tells a third story. This is not sequencing error. It is a fundamental property of how genomes evolve.

A **fundamental discovery of the genomics era** is that different genes in the same set of organisms frequently tell different evolutionary stories — the gene tree for one locus may have a different topology than the gene tree for another. This **gene tree/species tree discordance** is pervasive in phylogenomics and has profound implications for how we infer evolutionary relationships. Understanding its causes is essential for choosing appropriate methods and interpreting conflicting phylogenomic signal.

## The Core Problem

A **species tree** represents the evolutionary history of lineages (populations or species). A **gene tree** represents the evolutionary history of a specific gene locus within those lineages. Because genes evolve within populations that undergo complex processes (polymorphism, selection, transfer, duplication), the gene tree need not — and often does not — match the species tree.

## Incomplete Lineage Sorting (ILS)

**Incomplete lineage sorting** (ILS) is the most important and most biologically common cause of gene tree discordance. It arises from **deep coalescence**: the persistence of ancestral polymorphism through one or more speciation events.

Consider three species A, B, C where A and B diverged from their common ancestor more recently than B and C. In the ancestral population of A+B+C, multiple alleles exist at each locus. When the A+B+C population splits (speciation event 1), some ancestral alleles may not have had time to reach fixation within the A/B lineage before the next speciation event (A/B split). The result: gene copies from species A may be more similar to gene copies from species C than to gene copies from species B — a gene tree that contradicts the species tree.

**The anomaly zone**: For certain combinations of population size and divergence time, some gene tree topologies are actually *more probable* than the species tree topology under ILS alone. This "anomaly zone" occurs when internal branches are very short (rapid successive speciation with large ancestral population sizes). ILS-based gene tree discordance is particularly prominent in:
- **Birds**: Neoavian radiation after the K-Pg boundary (~66 Ma) was extremely rapid; many relationships remain uncertain.
- **Drosophila**: *melanogaster* subgroup divergences occurred rapidly with large population sizes.
- **Mammals**: Placental mammal radiation; rapid speciation with large ancestral populations.

**Measurement**: The multispecies coalescent model (MSC) predicts the probability of each gene tree topology given the species tree and branch lengths (in coalescent units = $t/4N_e$, where $t$ = branch length in generations and $N_e$ = effective population size). ASTRAL and SVDquartets use this theory to infer species trees from gene trees while accounting for ILS.

It turns out that the anomaly zone is not a rare edge case — many of the most important and contentious phylogenetic problems in evolutionary biology fall squarely in it. The relationships among major orders of birds, the placement of turtles within reptiles, the early branching of placental mammals — all involve rapid radiations with short internal branches and large ancestral populations. This is why concatenation-based phylogenomics, which ignores ILS, was systematically misleading for these problems.

## Horizontal Gene Transfer (HGT)

**HGT** (also called lateral gene transfer, LGT) moves genetic material between organisms by non-reproductive mechanisms: conjugation, transduction, transformation, or viral mediated transfer. HGT is especially common in bacteria and archaea but also occurs across kingdoms (agrobacterium to plants; integrative elements in eukaryotes).

In a phylogeny, HGT produces a gene tree where the transferred gene clusters with the recipient's relatives rather than the donor's relatives. Systematic detection: identify genes where the gene tree conflicts with the majority-rule species tree in a way consistent with a specific transfer event (the gene clusters with a different taxonomic group in a phylogenetically plausible way).

## Gene Duplication and Loss

When a gene duplicates before the speciation event of interest, paralogs end up in daughter species. If paralogy is not recognized (see Section 1.3 on homology), the gene tree inferred from the paralogs will appear discordant with the species tree — but the discordance is artifactual, caused by comparing the wrong gene copies.

Gene loss compounds this: if one paralog is lost in some species but retained in others, the sampling of paralogs across species may be incomplete, further obscuring the relationship between gene tree and species tree.

## Hybridization and Introgression

**Hybridization** produces organisms with genomes derived from two distinct lineages. In plants (where hybridization is common), entire genomes can merge (**allopolyploidy**). In animals, **introgression** (gene flow between differentiated populations) introduces foreign alleles into one lineage from another. Introgressed gene copies may cluster in gene trees with the donor lineage rather than the recipient lineage, producing reticulate (network) rather than tree-like history.

Ancient introgression between modern humans and Neanderthals (1–4% of non-African human genomes) produces human gene trees where some loci cluster with Neanderthal sequences — initially interpreted as sequencing error, now understood as genuine introgression. This discovery, enabled by ancient DNA sequencing and careful phylogenetic analysis of introgression patterns, fundamentally changed our understanding of human evolution.

## The Anomaly Zone in Detail

When speciation events occur rapidly with large ancestral populations, the probability of any single gene tree topology supporting the "true" species tree can fall below 1/3 (the random expectation for three possible resolutions of a four-taxon problem). In this **anomaly zone**, the most common gene tree is NOT the species tree — making concatenation-based phylogenetics (which assumes the gene tree = species tree) systematically misleading. Coalescent-based methods (ASTRAL) remain statistically consistent in the anomaly zone.

## Why This Matters

Gene tree/species tree discordance is not a complication to be ignored — it is a fundamental property of eukaryotic and prokaryotic genomes that reflects real biological processes (ILS, HGT, hybridization) with important biological implications; correctly attributing discordance to its cause is essential for accurate phylogenomics, for detecting hybridization, and for understanding the evolutionary processes that shape genome evolution. A phylogenomics analysis that treats every gene as telling the same story is ignoring most of what the data have to say.
