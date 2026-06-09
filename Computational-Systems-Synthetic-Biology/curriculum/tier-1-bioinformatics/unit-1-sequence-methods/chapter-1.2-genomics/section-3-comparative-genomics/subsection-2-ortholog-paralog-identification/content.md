# Ortholog and Paralog Identification

Here is a question that sounds straightforward but conceals a surprising depth: when you find that a human gene and a yeast gene have similar sequences, what does that tell you about their functions?

The naive answer is "probably the same thing, roughly." And often that is correct. The cytochrome c proteins of humans and yeast are sequence-similar, functionally equivalent, and even partially interchangeable. But consider a different pair: human BRCA1 and human BRCA2. These two proteins share limited sequence similarity, are both involved in DNA repair, and were both discovered as breast cancer susceptibility genes. You might think they are closely related, descended from a common ancestor that functioned in DNA repair. In fact, BRCA1 and BRCA2 are distantly related — they diverged before animal life began — and their similar cellular roles are convergent rather than conserved in the simple sense.

Or consider lactate dehydrogenase (LDH) and malate dehydrogenase (MDH). Both are oxidoreductases that use NAD⁺. Both have similar three-dimensional folds. They are clearly homologous — descended from a common ancestral enzyme. But they evolved different substrate specificities after gene duplication, and using one to infer function of the other would be wrong. The lesson: homology is necessary but not sufficient for functional inference. The key distinction is between orthologs and paralogs, and getting it right matters.

Homologs — genes sharing common ancestry — are the foundation of comparative genomics and functional annotation by homology. Distinguishing **orthologs** (arising by speciation) from **paralogs** (arising by duplication) is critical because these two types of homologs have different functional implications: orthologs typically retain the same function; paralogs frequently diverge.

## Definitions

**Ortholog**: genes in different species descended from a single gene in the last common ancestor. Orthologs are separated by speciation events. They typically (but not always) retain the same function.

**Paralog**: genes in the same or different species derived from gene duplication. Paralogs evolve within a lineage and frequently acquire new functions (neofunctionalization) or divide ancestral function (subfunctionalization).

**Xenolog**: homolog arising from horizontal gene transfer. Common in bacteria; rare but present in eukaryotes.

**Co-ortholog (many-to-many ortholog)**: when a gene in species A is orthologous to multiple genes in species B (due to lineage-specific duplication), each pair is a co-ortholog.

It turns out that the term "ortholog" does not map onto "same function" as cleanly as it might seem. After speciation, orthologs evolve independently in their respective lineages. Some orthologs drift far enough apart in sequence, expression pattern, or binding partners that they no longer perform the same function. And some paralogs, particularly those arising from ancient duplications, have subfunctionalized in ways that make each copy more specifically functional than the ancestor — which means a paralog might actually be a better functional model for a query gene than a distant ortholog. These complications mean that ortholog identification is a starting point for functional inference, not a substitute for experimental validation.

## The Conceptual Challenge

Sequence similarity alone does not distinguish orthologs from paralogs. A human gene BRCA1 may be more similar to a BRCA2 paralog in humans than to the BRCA1 ortholog in yeast — because BRCA1 and BRCA2 diverged more recently than the human-yeast split. Therefore, all-vs-all sequence comparison alone misleads.

The gold standard: **reconcile gene trees with species trees**. If a gene family tree node predates the species divergence, it represents a duplication (producing paralogs); if it matches the speciation event, it represents speciation (producing orthologs).

## Graph-Based Methods: OrthoFinder

**OrthoFinder** is the most widely used ortholog identification tool:

1. All-vs-all DIAMOND (fast protein comparison) for all pairs of species
2. Normalize bit scores for protein length and database size
3. Build a sequence similarity graph
4. **MCL (Markov Cluster algorithm)**: cluster the similarity graph into **orthogroups** — sets of genes descended from a single ancestral gene
5. Build gene trees for each orthogroup
6. Root and reconcile gene trees with the species tree
7. Classify each node as speciation (ortholog) or duplication (paralog)

```bash
# OrthoFinder on a directory of protein FASTA files (one per species)
orthofinder -f proteins_directory/ -t 16 -a 8

# Output:
# Orthogroups.tsv: orthogroup assignments
# Orthogroups/OG000001.fa: sequences in each orthogroup
# Phylogenetic_Hierarchical_Orthogroups/N0.tsv: hierarchical orthogroups
# Gene_Duplication_Events/Duplications.tsv: duplication events per node
```

OrthoFinder's hierarchical orthogroups distinguish between orthologs at different taxonomic levels (e.g., vertebrate-level orthologs vs. mammal-specific orthologs).

The MCL algorithm deserves brief attention because it captures something important about gene family structure. Rather than imposing a fixed similarity threshold, MCL finds natural clusters in the similarity graph by simulating a flow process: connections within a natural cluster are "flooded" while connections between clusters are "dried out." The resulting clusters correspond to gene families that have evolved from a single ancestor — which is exactly what you want for ortholog analysis. This is why OrthoFinder outperforms simpler reciprocal BLAST-based approaches, which tend to fragment large gene families or lump distinct ones together.

## Tree-Based Reconciliation

For rigorous ortholog identification, gene trees must be reconciled with the species tree. Three events can explain discordance:
- **Duplication**: a gene copies before speciation
- **Loss**: a gene is lost in one lineage (creates apparent "missing" orthologs)
- **Horizontal transfer (xenolog)**

**ETE Toolkit** provides Python API for tree reconciliation:

```python
from ete3 import Tree, PhyloTree

gene_tree = PhyloTree("gene_family.nwk", sp_naming_function=lambda n: n.split("_")[0])
species_tree = Tree("species.nwk")

# Reconcile gene tree with species tree
recon_tree, events = gene_tree.reconcile(species_tree)

for node in recon_tree.traverse():
    if hasattr(node, "evoltype"):
        if node.evoltype == "D":
            print(f"Duplication at: {node.name}")
        elif node.evoltype == "S":
            print(f"Speciation at: {node.name}")
```

## BUSCO: Universal Single-Copy Orthologs

**BUSCO** identifies universal single-copy orthologs — genes present in exactly one copy in (nearly) all species within a lineage:

```bash
# Assessment using vertebrate BUSCO database
busco -i proteome.faa -l vertebrata_odb10 -m protein -o busco_output/ -c 16

# Output:
# C:98.5%[S:97.8%,D:0.7%],F:0.5%,M:1.0%,n:3354
# C = Complete, S = Single-copy, D = Duplicated, F = Fragmented, M = Missing
```

BUSCO's vertebrata_odb10 database contains 3,354 genes expected to be single-copy in all vertebrates. High completeness (> 95% C) indicates a nearly complete genome assembly and annotation.

## OMA and PANTHER: Alternative Resources

**OMA (Orthologous Matrix)**: a comprehensive database of orthologs across 2,700+ species. Implements a pairwise evolutionary distance approach with Smith-Waterman alignment. Available as web tool and command-line.

**PANTHER**: classifies genes into evolutionary families and subfamilies. PANTHER's ortholog database supports functional annotation and pathway analysis.

**OrthoDB**: hierarchically nested orthogroups at multiple taxonomic levels. Level-specific orthogroups distinguish arthropod-level from insect-level orthologs.

## Inparalogs vs. Outparalogs

**Inparalogs**: paralogs that arose by duplication after the speciation event separating two species. These are species-specific duplicates and are co-orthologs relative to single-copy genes in the other species.

**Outparalogs**: paralogs that arose before the speciation event. These are non-orthologous between species.

This distinction matters for functional annotation: an inparalog pair (A1 and A2 in species X, both orthologous to A in species Y) may each have subfunctionalized relative to A.

## Practical Example: Olfactory Receptor Orthologs

Olfactory receptor (OR) genes are a large gene family with extensive lineage-specific duplication:
- Human: ~400 functional OR genes
- Mouse: ~1,200 OR genes
- Dog: ~800 OR genes

Most ORs are **outparalogs** (duplicated before mammalian diversification) and **inparalogs** (lineage-specific duplicates). OrthoFinder will group human/mouse/dog ORs into orthogroups, but the many-to-many ortholog relationships make functional annotation by homology less reliable than for single-copy genes.

The olfactory receptor example captures the complexity of real gene family evolution. A mouse OR gene that is 80% identical to a human OR gene might detect completely different odorous compounds, because olfactory receptor specificity is determined by a small number of hypervariable positions in the binding pocket that can differ substantially even at high overall sequence identity. Functional annotation by sequence similarity in this gene family is genuinely unreliable — a lesson that applies broadly to receptor gene families and other proteins where a small number of amino acids determine specificity.

## Why This Matters

Ortholog identification underlies every claim of "functional conservation" in comparative biology. Using paralogs instead of orthologs to infer function is a systematic error: lactate dehydrogenase and malate dehydrogenase are paralogs (common ancestor underwent duplication before animal diversification) with related but distinct functions. In clinical genomics, gene copy number and paralog structure affect variant interpretation. In synthetic biology, designing biosynthetic pathways requires identifying true orthologs of enzyme-coding genes, not paralogs with different substrate specificities. OrthoFinder and OMA provide the rigorous ortholog assignments needed for reliable comparative analysis.

In the context of synthetic biology, ortholog identification is not just an academic exercise. If you want to introduce the violacein biosynthetic pathway from Chromobacterium violaceum into E. coli, you need to know whether the pathway enzymes have orthologs in E. coli that might compete with your introduced pathway, and whether any of the C. violaceum enzymes share substrate overlap with essential E. coli metabolic enzymes. Getting these ortholog relationships wrong leads to metabolic interference, toxicity, or pathway failure. The rigor that OrthoFinder brings to comparative genomics is directly useful for engineering organisms.
