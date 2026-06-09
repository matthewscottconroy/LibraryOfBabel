# Amino Acid Substitution Models

For nucleotide sequences, the GTR model has 5 free exchangeability parameters — a tractable number that can be estimated from alignments of a few thousand sites. Scale this up to amino acids with 20 states, and the fully general time-reversible model has 190 exchangeability parameters. No single protein alignment carries enough information to estimate all of those independently. You need a different approach.

Amino acid substitution models describe the rates at which one amino acid is replaced by another over evolutionary time. Unlike nucleotide models where the 12 parameters are directly interpretable as chemical rates, amino acid models must handle 20 states — yielding $20 \times 19 / 2 = 190$ independent exchangeability parameters in the most general model. With typical datasets, these cannot all be reliably estimated; instead, **empirical models** derived from large collections of protein alignments are used.

## Why Empirical Models?

The General Time Reversible model for amino acids (AA-GTR) has 190 exchangeability parameters + 19 frequency parameters = 209 parameters (plus branch lengths). Fitting 190 independent rate parameters from a single protein alignment of a few hundred sites is impossible — the data are insufficient. Instead, empirical models are derived by fitting these parameters once to a massive curated collection of thousands of protein alignments representing diverse evolutionary histories, then treating the resulting rates as fixed constants when analyzing any new dataset.

This is analogous to using BLOSUM/PAM substitution matrices derived from large databases, but for phylogenetic purposes (where time-reversibility and explicit rate models are required).

It turns out that the complexity of amino acid substitution patterns reflects real biochemistry: whether a substitution is accepted depends simultaneously on amino acid size, charge, hydrophobicity, polarity, and structural role. A glutamate → aspartate substitution (similar charge, slightly smaller) is far more common than a glutamate → tryptophan substitution (opposite chemical character). Capturing all of these dependencies in a parameterized model would require a mechanistic theory of protein evolution that doesn't yet exist. Empirical models sidestep this by learning the rates from data.

## JTT: Jones, Taylor, and Thornton (1992)

**JTT** was derived from 59 alignment blocks of globular proteins from the PIR database, totaling ~59,000 observed amino acid substitutions. The 190 exchangeability parameters were estimated by maximum likelihood. JTT was the standard model for protein phylogenetics through the 1990s-2000s.

## WAG: Whelan and Goldman (2001)

**WAG** updated JTT using a larger dataset (~6,800 pairs of aligned protein sequences from ~182 protein families from Swiss-Prot/TrEMBL) and a more sophisticated maximum likelihood estimation procedure. WAG provides better model fit than JTT on most real datasets and became the new standard.

## LG: Le and Gascuel (2008)

**LG** is currently the most widely used general protein evolution model. Derived from an even larger database (~4,000 alignments, >67,000 sequences from Pfam), LG systematically outperforms both JTT and WAG in likelihood on independent benchmark datasets. LG is the default model in many phylogenetic programs (PhyML, IQ-TREE for protein data without model selection).

**LG+F**: A variant where the amino acid frequencies are estimated from the specific dataset being analyzed rather than using the frequencies from the training set. This improves fit when the dataset has unusual amino acid composition. You might think the training-set frequencies are always best — after all, they were estimated from thousands of alignments — but for a dataset of, say, extreme thermophile proteins with strong compositional bias, the specific dataset's frequencies are more informative than the global average.

## Why AA Models Are Empirical vs. Parameterized

For nucleotides, 12 rate parameters can be reliably estimated from typical alignments (1,000–10,000 sites). Chemical mechanisms suggest specific parameter constraints (Ti/Tv bias, base frequency effects) that further reduce the effective parameter count. The biological intuition is tractable.

For amino acids, 190 parameters + 19 frequencies = 209 parameters require far more data. Additionally, the physicochemical constraints are complex (amino acid substitutions depend on side chain volume, charge, hydrophobicity, polarity, and secondary structure propensity simultaneously). Empirical models capture these complex dependencies implicitly from evolutionary data without requiring an explicit mechanistic theory of why certain substitutions occur at certain rates.

## PAM vs. BLOSUM: Historical Context (Not for Phylogenetics)

**PAM** (Point Accepted Mutation) matrices and **BLOSUM** (BLOcks SUbstitution Matrix) are amino acid substitution matrices used in sequence alignment scoring (BLAST, local alignment), not phylogenetic models. They are not time-reversible and don't have the form required for the matrix exponential calculation in phylogenetics. Confusing PAM/BLOSUM with LG/WAG/JTT is a common error. If you are doing sequence similarity searches, you use BLOSUM62. If you are building a phylogenetic tree from protein sequences, you use LG or WAG. These are different mathematical objects designed for different purposes.

## Mixture Models: C60 and UL3

A single empirical model assumes all amino acid sites across all proteins evolve according to the same rate matrix. In reality, different structural and functional environments (buried hydrophobic core, solvent-exposed loop, active site) may have very different amino acid substitution patterns. **Mixture models** (C10, C20, C60) use 10, 20, or 60 different rate matrices (each representing a different site-specific evolutionary environment) and assign each site to a mixture component. C60 substantially improves model fit for large protein datasets and reduces phylogenetic artifacts from heterogeneous site evolution.

**UL3** (Ultrafast site-heterogeneous model) and **GHOST** are mixture model implementations available in IQ-TREE that provide dramatically better fit than LG alone for many datasets.

The intuition behind mixture models is compelling: a buried hydrophobic residue in a protein core can only accept other hydrophobic residues (very constrained, slow evolution), while a solvent-exposed loop residue may accept almost anything (fast evolution with many accessible states). These two situations have fundamentally different rate matrices. Treating them the same produces a model that poorly fits both. Mixture models learn these different regimes from the data, producing substantially better likelihood scores and, importantly, more accurate topology estimates for distantly related proteins.

## Model Selection for Amino Acid Data

**ModelTest-NG** and **IQ-TREE's ModelFinder** test protein evolution models by AIC/BIC to select the best-fitting model from a comprehensive list. For most analyses:

- **LG+Γ+F**: A reliable default for diverse protein datasets.
- **WAG+Γ**: Legacy default; now mostly superseded by LG.
- **Mixture models (C20, C60)**: For large datasets where computational cost is manageable; substantially improves topology accuracy for distantly related sequences.

```bash
# IQ-TREE2 model selection for protein data
iqtree2 -s protein_alignment.faa -m TEST -mset LG,WAG,JTT,C20,C60 --msub nuclear
```

## Why This Matters

Amino acid substitution models are the bridge between raw protein sequence data and phylogenetic inference — choosing LG vs. JTT vs. a mixture model can meaningfully affect topology at deep evolutionary divergences where model fit matters most; understanding why empirical models are used (data limitation) and when mixture models are advantageous (heterogeneous sites) directly informs phylogenomic analysis quality. For ancient evolutionary questions — the origin of eukaryotes, the deep branching of major metazoan phyla — the choice of protein model is often as consequential as the choice of taxa included.
