# Computing Pairwise Distances

Imagine you have aligned two DNA sequences and you count up how many positions differ. Fifteen of a hundred sites are different, so you write down 0.15 and move on to the next pair. Simple enough — and for closely related sequences, a reasonable first approximation. But here is the problem: that number is almost certainly too small. The more diverged two sequences are, the more the observed fraction of differences underestimates the true number of evolutionary events that have occurred. Sites can change more than once. A mutation can be reversed. Two independent mutations can arrive at the same destination. You are trying to count footsteps, but you can only see the endpoint, not the path.

Distance-based phylogenetic methods convert aligned sequences into a matrix of pairwise distances (one number for each pair of sequences representing how different they are), then use this matrix to infer a tree. The key challenge is that the **observed divergence** (fraction of sites that differ) systematically underestimates the **true evolutionary distance** (total number of substitutions per site, including multiple substitutions at the same site that cancel each other out). Distance models correct for this underestimation.

## Hamming Distance: The Observed Divergence

The simplest pairwise distance is the **Hamming distance** (also called the **p-distance** or **observed divergence**):

$$p = \frac{\text{number of differing sites}}{L}$$

where $L$ is the total alignment length. For two sequences that differ at 15 of 100 aligned sites, $p = 0.15$.

The p-distance is biologically interpretable (it's directly observable) but is a biased estimator of evolutionary distance because it doesn't account for **multiple substitutions** — when a site has changed more than once over evolutionary time, only the final state is observed. A site that evolved A → C → A appears identical in the two sequences, but actually contributed 2 substitutions. As divergence increases, p saturates (approaches a maximum of ~0.75 for equal base frequencies), making it unreliable for highly divergent sequences.

## Jukes-Cantor (JC69) Correction

The **Jukes-Cantor model** (1969) is the simplest nucleotide substitution model: all four nucleotides are equally frequent, and all 12 possible substitutions (A→C, A→G, A→T, C→A, ...) occur at the same rate $\alpha$.

Under this model, the true evolutionary distance $d$ (in expected substitutions per site) relates to the observed proportion of differences $p$ by:

$$d = -\frac{3}{4} \ln\left(1 - \frac{4}{3}p\right)$$

**Derivation intuition**: At any site, the probability of NOT observing a difference after $d$ substitutions per site decays exponentially. For equal rates and frequencies, this gives the characteristic $-\frac{3}{4} \ln(\cdot)$ form. As $p \to 0.75$, $d \to \infty$ (infinite divergence corresponds to random sequence similarity).

**Worked example**: Two sequences differ at $p = 0.30$ of sites.

$$d = -\frac{3}{4} \ln\left(1 - \frac{4}{3}(0.30)\right) = -0.75 \ln(0.60) = -0.75 \times (-0.511) = 0.383 \text{ substitutions/site}$$

The corrected distance (0.383) is 28% larger than the observed divergence (0.30), reflecting the back-substitution correction. This may seem like a modest correction, but for more diverged sequences — say $p = 0.50$ — the correction becomes enormous: $d = -0.75 \ln(1 - 4/3 \times 0.50) = -0.75 \ln(-0.167)$, which is undefined, indicating that saturation has been reached and the observed divergence carries almost no information about the true evolutionary distance.

## Kimura 2-Parameter (K2P) Model

The **Kimura 2-parameter (K2P or K80) model** (1980) improves on JC69 by recognizing that transitions (pyrimidine↔pyrimidine: C↔T; or purine↔purine: A↔G) typically occur at a higher rate than transversions (pyrimidine↔purine: A/G↔C/T). K2P has two parameters: the transition rate $\alpha$ and the transversion rate $\beta$, with the transition:transversion ratio **κ = α/(2β)** (often κ ≈ 2–10 for typical DNA sequences).

The K2P distance is:

$$d = -\frac{1}{2}\ln\left(1 - 2P - Q\right) - \frac{1}{4}\ln\left(1 - 2Q\right)$$

where $P$ = proportion of transition differences and $Q$ = proportion of transversion differences. K2P is the default distance model in many phylogenetic programs and in the original MEGA implementation.

Why does the transition/transversion distinction matter? Biochemically, transitions involve changing one purine to another (A↔G) or one pyrimidine to another (C↔T), which requires less structural rearrangement at the DNA level than swapping a purine for a pyrimidine. In practice, this means that observing the same nucleotide at a site in two sequences is a weaker indicator of shared ancestry than you might think — it could reflect a transition that happened and then reversed, or a series of transitions back and forth. Separating these two rates makes the correction more accurate.

## Why Observed Divergence Underestimates True Distance: Saturation

As the true evolutionary distance increases, sites accumulate multiple substitutions. For highly diverged sequences (distant relatives), many sites will have changed 2, 3, or more times. Beyond a certain divergence level (**saturation**), the observed divergence no longer increases even as true evolutionary change continues — the alignment appears to "saturate" because the number of observable differences approaches the random expectation.

For nucleotides, saturation typically occurs around $p \approx 0.5–0.75$ (depending on base composition). For amino acids (20 states), saturation occurs at higher p-distances. Saturation is why third codon position sequences (which evolve fastest) are often excluded or heavily downweighted in deep phylogenetic analyses, and why protein sequences are preferred over nucleotides for ancient divergence studies.

**Saturation plot**: Plot pairwise $p$-distances vs. JC-corrected distances for all sequence pairs. If points follow a straight line, there is no saturation. If points bend below the diagonal, saturation is occurring. This is a practical diagnostic you should run before any deep phylogenetic analysis — if you see saturation, the simplest distance models are inadequate, and you need either more sophisticated correction models or a shift to model-based likelihood methods.

## The Distance Matrix

Computing all pairwise corrected distances produces a **distance matrix** $D$ where $D_{ij}$ is the estimated evolutionary distance between sequences $i$ and $j$. This symmetric matrix is the input for distance-based tree-building algorithms (UPGMA, Neighbor-Joining).

Note that some distances may be negative (when observed divergence is very low and sampling noise creates negative log terms) — this is a numerical artifact that indicates insufficient data for reliable distance estimation. If you encounter negative distances, the sequences are too similar for the distance model to give meaningful corrections — either use the raw p-distance, or recognize that the sequences are so similar that any tree topology will have nearly zero branch lengths anyway.

## Why This Matters

Understanding evolutionary distance models — and why the naive observed divergence underestimates true evolutionary change — is the conceptual foundation for all phylogenetic distance methods, and the saturation problem motivates the use of more sophisticated likelihood-based methods that explicitly model the substitution process for highly diverged sequences. Every distance-based tree you build encodes assumptions about how sequences evolve; understanding JC69 and K2P is the first step to understanding why those assumptions matter and when they fail.
