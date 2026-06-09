# Structural Classification Databases

In 1992, Cyrus Chothia estimated that the total number of distinct protein folds in nature might be only a few thousand. It was a bold claim at a time when the PDB contained a few hundred structures. But as the database grew over the following decades, Chothia's estimate proved remarkably prescient. There are currently fewer than 2,000 recognized structural superfamilies. Life has built its molecular machinery almost entirely from a small number of recurring architectural blueprints, combined and recombined in countless ways.

This is one of the deepest insights in all of biology: protein structure space is not infinite and random but is discretely organized around a set of stable folds that evolution has discovered and reused endlessly. The TIM barrel, the Rossmann fold, the immunoglobulin domain, the P-loop NTPase — each appears in dozens to hundreds of proteins across all kingdoms of life, performing wildly different functions from the same structural scaffolding.

Protein structures are classified into hierarchical systems that organize the universe of known protein folds. These classifications reveal evolutionary relationships between proteins that cannot be inferred from sequence alone, because structure is more conserved than sequence over long evolutionary time scales. Three major databases have shaped our understanding of protein structural diversity.

## SCOP: Structural Classification of Proteins

**SCOP** (Structural Classification of Proteins, Murzin et al., 1995; maintained as SCOPe by the Berkeley group) classifies protein domains in a four-level hierarchy:

1. **Class**: The broad category based on secondary structure content:
   - **All-α**: Domains composed almost entirely of α-helices (e.g., globins, cytochrome c).
   - **All-β**: Domains composed of β-sheets (e.g., immunoglobulins, beta-propellers).
   - **α/β**: Alternating α-helices and β-strands, typically mixed topology (e.g., TIM barrels, Rossmann folds).
   - **α+β**: α and β regions largely segregated (e.g., lysozyme, ribonuclease A).
   - **Multi-domain**: Domains that fall into different classes; α and β regions are spatially distinct.

2. **Fold**: Domains with the same overall topology of secondary structure elements — same arrangement in 3D space — regardless of evolutionary relationship. Proteins in the same fold class may have evolved independently (**analogous** rather than **homologous** evolution). Example: The TIM barrel fold appears in dozens of enzymatic superfamilies that likely arose by convergent evolution.

3. **Superfamily**: Domains in the same superfamily are considered to have a common evolutionary origin, evidenced by structural and sometimes functional similarity, even if sequence identity is too low to detect by sequence methods. Example: The "P-loop NTPase" superfamily contains kinases, GTPases, and ATPases with diverse sequences but a common nucleotide-binding core structure.

4. **Family**: Domains within a family have clearly detectable sequence similarity, indicating clear homology. Typically ≥ 30% sequence identity.

SCOPe 2.08 contains ~14,000 PDB entries organized into ~1,200 folds and ~2,500 superfamilies.

The critical distinction between fold and superfamily is one of the richest puzzles in structural biology: two proteins in the same fold may have no detectable shared ancestry — they may have converged on the same architecture because it is an unusually stable or functionally versatile shape. Two proteins in the same superfamily share a fold *and* share ancestry — the fold is a heritable trait, not a coincidence. Disentangling convergent evolution from divergent evolution at the structural level requires careful analysis of functional and mechanistic similarity, not just geometric overlap.

## CATH: Class Architecture Topology Homology

**CATH** (Orengo et al., 1997; continuously updated at cathdb.info) provides an alternative hierarchical classification with some conceptual differences from SCOP:

1. **Class** (C): Same categories as SCOP (mainly-α, mainly-β, α-β, few secondary structures).
2. **Architecture** (A): The overall shape of the domain's secondary structures in 3D space, described qualitatively (e.g., "β-barrel," "orthogonal bundle"). Unlike SCOP fold, architecture ignores the connectivity (order of secondary structures in sequence).
3. **Topology** (T): Secondary structure elements arranged in the same order and orientation — equivalent to SCOP fold (same topology = same fold).
4. **Homology** (H): Evolutionarily related domains — equivalent to SCOP superfamily.

CATH additionally classifies protein domains using sequence clustering at 35%, 60%, and 100% identity levels, providing a more granular sequence-based stratification within each homologous group. This makes CATH particularly useful for non-redundant structure selection — if you want one representative structure per fold at 35% sequence identity, CATH gives you that directly.

## ECOD: Evolutionary Classification of Protein Domains

**ECOD** (Evolutionary Classification Of protein Domains, Cheng et al., 2014) takes a strictly evolutionary perspective, prioritizing phylogenetic relationships over structural topology. ECOD classifies domains in a five-level hierarchy: X (possible homology) → H (homologous superfamily) → T (topology) → F (family) → ECOD domain. The X-group captures very ancient, possibly homologous relationships that SCOP and CATH place in separate fold classes. This makes ECOD particularly valuable for deep evolutionary studies where you suspect common ancestry but the structural similarity has been eroded to near-undetectability.

## Structural Classification Reveals Remote Evolutionary Relationships

The most powerful application of structural classification databases is detecting distant evolutionary relationships invisible in sequence. A classic example: the **ferredoxin fold** (β-α-β-β-α-β topology) appears in proteins with completely unrelated functions (electron transfer, DNA binding, metabolic enzymes) and undetectable sequence similarity (<10% identity). Structural superposition and DALI searches reveal their common fold, suggesting a common ancestor.

Another example: the **RNA Recognition Motif (RRM)**, a small β-α-β-β-α-β domain, turns up in hundreds of RNA-binding proteins across all eukaryotes. The RRM is not just a structural coincidence — it is an ancient module that cells have reused to build an astonishing diversity of RNA-regulatory functions. Without structural classification, the evolutionary unity of this protein family would be completely obscured by sequence divergence.

## Structure-Based Homology Search: Foldseek

**Foldseek** (van Kempen et al., 2023) enables ultra-fast structure-based search of the PDB (and AlphaFold database) at speeds comparable to sequence search (BLAST). It encodes protein structure as a sequence of "3Di" tokens — discrete states representing the local structural context of each residue — and uses sequence alignment on these encoded sequences. A search of the entire PDB takes seconds on a standard computer, making structure-based classification accessible at scale. Foldseek has made the DALI server largely obsolete for large-scale structural database searches.

This is a genuinely transformative development. With AlphaFold2 producing structures for essentially every known protein, and Foldseek making it possible to search those structures in seconds, the structural classification landscape has shifted dramatically. You can now ask "what is the closest structural neighbor of this predicted structure in the AlphaFold database of 200 million proteins" and get an answer before you finish your coffee. The implications for function prediction and evolutionary analysis are still being worked out, but they are profound.

## Using DALI Server for Structural Homolog Search

The classic approach: upload a structure file to the **DALI server** (ekhidna.biocenter.helsinki.fi), which compares the query against the PDB by distance matrix comparison. Results include a list of structural neighbors with Z-scores (>2 = likely similar fold, >8 = certain structural homolog), RMSD, and alignment length. For queries against the PDB alone — where the curated quality and experimental validation of structures matters — DALI remains a valuable tool.

## Why This Matters

Structural classification databases are the foundation for function prediction in structural bioinformatics — when a newly determined structure is classified into a known superfamily, functional inference follows from characterized family members — and they reveal the evolutionary history of protein architecture, showing how a small number of fundamental folds have been reused throughout the tree of life. In the AlphaFold2 era, with structures available for millions of uncharacterized proteins, the structural classification problem has never been more important. The question "what does this protein do?" is now, more than ever, answered by asking "what fold is this, and what do other proteins in that fold do?"
