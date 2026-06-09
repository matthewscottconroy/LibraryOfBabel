# Key Phylogenetic Terminology

Before you can read a phylogenetic tree — or spot errors in one — you need to be fluent in its vocabulary. This is not just taxonomic pedantry. Confusing a soft polytomy with evidence of simultaneous radiation, misidentifying the root, or calling a paraphyletic group a clade are all errors that propagate into conclusions. Phylogenetics has a precise vocabulary that is essential for reading the literature and correctly interpreting trees. Many terms are straightforward geometric descriptions of tree topology; others have subtle evolutionary implications. Mastery of this terminology prevents common errors in tree reading and analysis.

## The Tree Structure

A phylogenetic tree consists of **nodes** connected by **branches** (also called edges). Two types of nodes:

**Leaf nodes** (terminal nodes, **taxa**): Represent the observed sequences or organisms being studied. Also called OTUs (**Operational Taxonomic Units**) in ecology and microbiology — a pragmatic grouping of similar sequences that may or may not correspond to biological species.

**Internal nodes**: Represent inferred common ancestors — hypothetical ancestral entities that gave rise to all the descendant taxa in the subtree rooted at that node. Each internal node represents a speciation, gene duplication, or viral divergence event. When you read a phylogenetic paper and see a node annotated with a posterior probability of 0.98, that node is a statement about an ancestor that actually lived — the probability that this particular grouping of descendants reflects a real historical common ancestry event.

**Branch lengths** quantify evolutionary change. In substitution-based models, branch length is measured in **expected substitutions per site** — for example, a branch of length 0.05 means 5% of sites are expected to have changed along that lineage. For ultrametric trees (time-calibrated), branch lengths represent time units (millions of years, calendar dates for rapidly evolving organisms).

**Root**: The unique internal node that represents the most recent common ancestor of all taxa in the tree. The root defines the directionality (past to present) of the tree. Without a root, a tree has no "direction of time" — you know the branching pattern but not which end is ancient and which is recent.

## Rooted vs. Unrooted Trees

An **unrooted tree** depicts relationships among taxa without implying a direction of evolutionary time. The topology shows branching patterns only. An unrooted tree with 4 taxa has 3 possible topologies.

A **rooted tree** has a specified root node, which allows directionality: ancestors are "above" (or "toward the root from") descendants. Rooting requires external information — the sequences themselves do not tell you which end is ancestral.

**Outgroup rooting**: The most common rooting method. An organism known to be outside the group of interest (**outgroup**) is included, and the root is placed on the branch separating the outgroup from the remaining taxa (**ingroup**). The outgroup must be more distantly related to all ingroup members than any ingroup members are to each other. Example: To root a mammal phylogeny, a fish or bird serves as an outgroup. If the outgroup is incorrectly chosen (too closely or too distantly related), the root placement will be wrong — and a misplaced root can reverse the apparent direction of evolutionary change for every clade in the tree.

## Clades and Monophyletic Groups

A **clade** (or **monophyletic group**) consists of an ancestor and ALL of its descendants. Clades are the natural units of phylogenetic classification: any clade in a rooted tree represents a group that has shared a common evolutionary history since the node at the base of the clade.

In contrast, a **paraphyletic group** includes an ancestor and SOME but not all descendants. "Reptiles" is paraphyletic because it excludes birds, which are the descendants of dinosaurs. A **polyphyletic group** includes organisms from different clades that are grouped by convergent traits — "flying vertebrates" (birds, bats, pterosaurs) is polyphyletic.

It turns out that many traditional taxonomic groups are paraphyletic — "fish" excludes tetrapods even though tetrapods evolved from fish ancestors; "prokaryotes" excludes eukaryotes even though the deepest archaeal lineages are more closely related to eukaryotes than to bacteria. Phylogenetics has forced the reclassification of many familiar groups by insisting on monophyly as the criterion for natural grouping, which is not mere pedantry but reflects a genuine commitment to grouping organisms by shared evolutionary history rather than superficial similarity.

## Bifurcating vs. Polytomous Trees

A **fully bifurcating** (dichotomous) tree has all internal nodes with exactly 2 descendants — each ancestral node splits into exactly 2 lineages. Most phylogenetic methods produce fully bifurcating trees.

A **polytomy** (multifurcation) is an internal node with 3 or more descendants, representing either: (1) a **hard polytomy** — genuinely simultaneous divergence of multiple lineages (a rapid adaptive radiation where speciation events are so close in time that they cannot be resolved); or (2) a **soft polytomy** — insufficient data to resolve the branching order (the true tree is bifurcating, but statistical support for any specific resolution is too low).

Polytomies in published trees should always be examined carefully — most represent soft polytomies (unresolved relationships) rather than biological simultaneous divergence. You might expect that with modern genome-scale data, all polytomies would be resolved, but it turns out that some rapid radiations — the Neoavian bird radiation at the K-Pg boundary, the early placental mammal diversification — happened so quickly that even whole genomes cannot fully resolve them, because the internal branches connecting the relevant nodes are simply too short to accumulate informative substitutions.

## Topology vs. Branch Lengths

**Topology** refers to the branching pattern — which taxa are connected to which nodes — independent of branch lengths. Two trees can have identical topologies but different branch lengths (representing different amounts of evolution along each lineage).

**Congruence/incongruence**: Two trees are topologically **congruent** if they have the same branching pattern. Gene tree/species tree incongruence (a major topic in Section 8) means the topology of the gene tree differs from the topology of the species tree — a phenomenon that turns out to be far more common than previously appreciated, with profound consequences for how phylogenomics is done.

## Ultrametric Trees and Time Calibration

In an **ultrametric tree**, all tips (leaves) are equidistant from the root — they all exist at the same point in time (the present). Ultrametric trees arise from strict molecular clock models where all lineages evolve at the same rate. Time-calibrated trees (from BEAST2 with fossil calibrations) are ultrametric.

Under a **relaxed molecular clock**, different lineages evolve at different rates, and the tips may not all be equidistant from the root when measured in substitutions — but after time calibration, the tips are aligned to the same time point by using rate variation to convert branch lengths to time.

## Why This Matters

Correct interpretation of phylogenetic trees requires fluency with this vocabulary — confusing a soft polytomy with a hard polytomy misrepresents biological uncertainty, misidentifying the root inverts evolutionary directions, and failing to recognize paraphyletic groups leads to systematic errors in comparative analyses that assume monophyly. These are not abstract concerns: every paper in comparative genomics, ancestral sequence reconstruction, and diversification rate analysis depends on reading trees correctly. Getting the terminology right is the prerequisite for everything that follows.
