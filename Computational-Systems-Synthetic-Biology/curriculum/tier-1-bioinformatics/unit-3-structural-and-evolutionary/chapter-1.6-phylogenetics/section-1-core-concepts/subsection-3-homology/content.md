# Homology in Phylogenetics

In 1843, Richard Owen defined homology as the "same organ in different animals under every variety of form and function." He meant that a human arm and a bat wing are homologous — not because they look alike or do the same thing, but because they are the same structure, transformed by evolution. Darwin later supplied the explanation: homology means common ancestry. This conceptual foundation has never changed, but the molecular era revealed that the word "homology" covers several distinct evolutionary relationships that look superficially similar but have very different implications for phylogenetic analysis. Getting them confused is one of the most consequential mistakes in comparative genomics.

**Homology** — the sharing of common ancestry — is the conceptual foundation of comparative biology and molecular phylogenetics. However, the word "homology" encompasses several distinct evolutionary relationships that can produce superficially similar but evolutionarily distinct patterns. Conflating them leads to incorrect phylogenetic analyses and incorrect functional inferences.

## Three Types of Sequence Homology

**Orthology**: Two genes are **orthologs** if they are related by a **speciation event** — they are the "same" gene in two different species, derived from a single ancestral gene present in the common ancestor. Orthologs typically (but not always) perform the same function in their respective organisms. BRCA1 in human and the mouse BRCA1 gene are orthologs.

**Paralogy**: Two genes are **paralogs** if they are related by a **gene duplication event**. After duplication, each copy can evolve independently and may acquire new or modified functions. The hemoglobin α and β subunit genes are paralogs — they arose by duplication from a single ancestral globin gene. All human HOX genes are paralogs (arising from tandem duplications of an ancestral Hox gene). You might expect that gene duplicates would quickly diverge until they are unrecognizable — but it turns out that many paralogs retain enough sequence similarity that they are easily confused with orthologs, especially in large gene families like kinases or GPCRs, where dozens of closely related family members exist in every genome.

**Xenology**: Two genes are **xenologs** if their relationship involves **horizontal gene transfer (HGT)** — the gene was transferred between organisms by non-reproductive means. Antibiotic resistance genes transferred between bacterial species on plasmids are xenologs in the recipient species. The transferred gene may be highly similar to its donor-species counterpart, making it look like a close phylogenetic relationship when the actual connection is not evolutionary relatedness but lateral transfer.

## Synapomorphy vs. Symplesiomorphy for Tree Reconstruction

In morphological (and by analogy, molecular) phylogenetics, shared characters are categorized by their evolutionary significance:

**Synapomorphy**: A derived character state shared by a group of taxa due to inheritance from a common ancestor in which the character originated. Synapomorphies define clades (monophyletic groups) and are phylogenetically informative.

**Symplesiomorphy**: A primitive (ancestral) character state shared by multiple taxa because they all retained it from a distant ancestor. Symplesiomorphies do not define a meaningful clade — they only show that the taxa haven't changed. Feathers are a synapomorphy of birds; a notochord at some stage of development is a symplesiomorphy of chordates.

In molecular phylogenetics (maximum likelihood, Bayesian), the substitution model explicitly distinguishes shared ancestral states from shared derived states, avoiding the conceptual confusion that plagued early phenetic clustering methods. This is one of the key advantages of model-based over similarity-based approaches: the model provides a framework for evaluating whether a shared nucleotide is a synapomorphy or a symplesiomorphy.

## The Danger of Paralogs in Phylogenetics

This is perhaps the most important practical implication of homology types: **using paralogs instead of orthologs in a species phylogeny analysis produces incorrect tree topologies**.

If a gene duplicated in the ancestor of clade A (producing gene1 and gene2), and you include gene1 from species A with gene2 from species B (without realizing they are paralogs), the resulting tree will show species A and B as more divergent than they truly are — because you're comparing copies that diverged at the duplication event (predating the speciation), not at the speciation event itself.

This paralog contamination problem is especially severe for:
- Gene families (kinases, GPCRs, olfactory receptors) where many paralogs exist
- Ancient gene duplications (the two rounds of vertebrate whole-genome duplication)
- Partial genome assemblies where gene model quality is poor

It turns out that early molecular phylogenetic studies were sometimes compromised by exactly this problem. The two rounds of vertebrate whole-genome duplication mean that every vertebrate has up to four copies of most ancestral genes, and identifying which copy in species A is truly orthologous to which copy in species B requires careful phylogenetic analysis of the entire gene family — you cannot simply BLAST a sequence and take the top hit.

## Ortholog Identification Tools

**OrthoFinder** (Emms & Kelly, 2019) is the standard tool for genome-scale ortholog inference. It:
1. Performs all-vs-all BLAST between input proteomes.
2. Constructs a gene similarity graph.
3. Clusters orthogroups using an algorithm that identifies groups of genes all derived from a single ancestral gene.
4. Infers orthology relationships (1:1 orthologs, co-orthologs) within each orthogroup using gene trees.

OrthoFinder outputs: orthogroups (clusters of all homologs), one-to-one ortholog pairs, and phylogenetic trees for each orthogroup. The gene-tree-based approach is key: rather than defining orthologs by sequence similarity thresholds alone (which conflates orthologs and paralogs), OrthoFinder reconciles gene trees with the species tree to identify where duplications vs. speciations occurred.

**BUSCO** (Benchmarking Universal Single-Copy Orthologs) uses a curated set of single-copy orthologs expected to be present once in all genomes of a given clade (e.g., Mammalia BUSCO set). BUSCO scores are used to assess genome assembly completeness, and BUSCO genes (being single-copy orthologs) are ideal for phylogenomics precisely because they avoid the paralog problem: if a gene is consistently single-copy across all members of a clade, there is no ambiguity about which copy to use.

## Ortholog Databases

**OMA** (Orthologous MAtrix): Pairwise ortholog database computed from >2,500 proteomes. Provides hierarchical orthogroups (HOGs) that respect the species tree topology.

**eggNOG** (evolutionary genealogy of genes: Non-supervised Orthologous Groups): Contains functional annotation (Gene Ontology terms, KEGG pathways) for orthologous groups across >5,000 organisms. **eggNOG-mapper** rapidly annotates a new genome by assigning each protein to an eggNOG orthologous group and transferring its functional annotations. This is one of the most powerful practical applications of phylogenetics to genomics: by identifying which orthologous group a protein belongs to, you inherit all the functional characterization work done on any member of that group.

## Why This Matters

Correctly distinguishing orthologs from paralogs is fundamental to both phylogenomics (species tree reconstruction) and functional genomics (function inference by similarity) — using the wrong type of homolog in a phylogenetic analysis produces incorrect evolutionary conclusions, while using paralogs for function inference assumes functional conservation that may not exist. The entire machinery of comparative genomics — identifying conserved genes, inferring function by homology, detecting positive selection in gene families — rests on this distinction. If you build a phylogeny from paralogs thinking they are orthologs, the topology is meaningless; if you infer protein function from a paralog, the functional annotation may be wrong. Orthology is not a technical detail. It is the conceptual foundation on which all comparative molecular biology stands.
