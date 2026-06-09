# Coalescent-Based Species Tree Methods

The previous section established that concatenating all genes into a single super-alignment can give the wrong species tree when ILS is prevalent. This is not a theoretical concern — it is a proven statistical inconsistency. As you add more genes to a concatenated analysis in the anomaly zone, the tree you infer converges to the wrong tree with increasing confidence. More data makes things worse. The solution is to change the inference framework entirely: use methods that explicitly model the fact that different genes have different genealogical histories.

Concatenation-based approaches (treating all genes as a single super-alignment) produce a single "super-gene tree" that ignores the fact that different genes have different genealogical histories. When ILS is prevalent, concatenation is statistically inconsistent — it can converge to the wrong species tree even with infinite data. **Coalescent-based species tree methods** explicitly model the multispecies coalescent, accounting for gene tree discordance due to ILS.

## The Multispecies Coalescent (MSC) Model

The **multispecies coalescent** (Rannala & Yang, 2003; Degnan & Rosenberg, 2006) provides the probabilistic model connecting gene trees and species trees. Under MSC:

1. The species tree has branches with lengths in **coalescent units** $\tau = t/(4N_e)$ (where $t$ = branch duration in generations, $N_e$ = ancestral effective population size).
2. Gene lineages from different species coalesce (find their common ancestor) according to a Kingman coalescent process within the species tree branches.
3. When a lineage fails to coalesce within an ancestral population branch before the next speciation event, it is carried into the deeper ancestral population — this is deep coalescence, producing ILS.
4. The probability of a given gene tree topology, given the species tree topology and branch lengths in coalescent units, is derived from coalescence theory.

The coalescent units $\tau = t/(4N_e)$ make explicit the two parameters that drive ILS: the duration of the ancestral population ($t$, proportional to branch length in the species tree) and the effective population size ($N_e$). Short $t$ or large $N_e$ both increase the probability of ILS. Rapid speciation with large populations — exactly what occurs in adaptive radiations — is the ILS danger zone.

## ASTRAL: Quartet-Based Summary Method

**ASTRAL** (Accurate Species TRees ALgorithm, Zhang et al., 2018; ASTRAL-III) is the most widely used coalescent-based species tree method. It takes a collection of input gene trees (one per gene locus) and finds the species tree topology that maximizes the number of quartet topologies shared between the species tree and the gene trees.

**Algorithm**: 
1. Infer ML gene trees from individual gene alignments (using IQ-TREE).
2. Extract all possible quartets (sets of 4 taxa) from each gene tree.
3. Find the species tree that maximizes the **quartet support score** — the total number of gene tree quartets that agree with the corresponding quartet in the species tree.

ASTRAL is proven to be **statistically consistent** under the MSC: given enough gene trees, ASTRAL converges to the true species tree even in the anomaly zone. It runs in $O(n^2 k)$ time where $n$ = taxa and $k$ = gene trees, making it practical for hundreds of taxa and thousands of genes.

```bash
# Typical ASTRAL workflow:
# 1. Infer individual gene trees with IQ-TREE
for gene in genes/; do
    iqtree2 -s ${gene}.fasta -m GTR+G4 -T 4 --prefix ${gene}
done

# 2. Concatenate all gene trees
cat genes/*.treefile > all_gene_trees.trees

# 3. Run ASTRAL
java -jar astral.5.7.8.jar \
    -i all_gene_trees.trees \
    -o species_tree_astral.treefile \
    2> astral.log
```

ASTRAL outputs local posterior probabilities for each branch (an analog of bootstrap support calibrated under the MSC). Values ≥ 0.95 are considered strong support.

## SVDquartets: Site-Based Quartet Method

**SVDquartets** (Chifman & Kubatko, 2014) is a coalescent-based method that operates directly on site patterns from the aligned sequences — not from pre-estimated gene trees. For each quartet of taxa, SVDquartets identifies the correct quartet topology using the singular value decomposition of the site pattern frequency matrix (exploiting algebraic properties of the MSC that make certain site pattern combinations more frequent under the correct topology).

SVDquartets is implemented in **PAUP*** and does not require separate gene tree estimation. Its advantage is bypassing gene tree estimation error (gene trees from short loci are noisy); its limitation is that it does not use model-based substitution model fitting.

## Full Bayesian Multispecies Coalescent: *BEAST (StarBEAST3)

**\*BEAST** (implemented as the StarBEAST3 package in BEAST2) performs full Bayesian inference under the MSC: it jointly estimates gene trees, the species tree, substitution model parameters, and population sizes from the aligned sequence data in a single MCMC analysis. This is the most statistically principled approach but is computationally intensive — practical for datasets of ~10–50 taxa and 10–100 loci.

The *BEAST framework has a compelling payoff: by jointly estimating gene trees and the species tree, it correctly propagates uncertainty from gene tree estimation into species tree estimation. ASTRAL, as a summary method, uses fixed gene trees (ignoring gene tree estimation uncertainty), which can be a problem for short loci with few informative sites.

## STAR and MP-EST

**STAR** (Species Tree from Average Ranks of Coalescences) and **MP-EST** (Maximum Pseudo-likelihood Estimation of Species Trees) are older summary methods that estimate species trees from gene trees using different objective functions. They are now largely superseded by ASTRAL in practice, but remain useful as comparison methods.

## Practical Recommendations

**For large datasets (>50 taxa, >50 genes)**: Use ASTRAL. Its speed makes it practical for phylogenomic scale analyses. Bootstrap gene trees for uncertainty quantification.

**For small datasets with population structure**: Use *BEAST (StarBEAST3). It properly integrates over gene tree uncertainty and estimates population sizes.

**For detecting reticulation**: Compute gCF/sCF concordance factors alongside ASTRAL. Large gCF values indicate broad support; low gCF despite high bootstrap suggests widespread discordance (ILS or hybridization).

**Concatenation + ASTRAL comparison**: If both produce the same tree, confidence is high. If they disagree, the ASTRAL result is more likely to be correct when ILS is suspected; the concatenation result may be more reliable when ILS is minimal but substitution model misspecification affects gene trees.

## Why This Matters

Coalescent-based methods have resolved long-standing controversies in phylogenomics — including the placement of turtles within reptiles, the early-branching positions within Neoaviae, and the relationship between hominin species — that concatenation approaches failed to resolve correctly due to abundant ILS; these methods are now essential tools for any phylogenomic analysis of recently diverged organisms. If you are working on any clade that underwent a rapid radiation, ASTRAL is not optional — it is the appropriate method of choice.
