# Nucleotide Substitution Models

Every maximum likelihood phylogenetic analysis requires you to specify a model of how nucleotides change over time. This model is not a peripheral technical detail — it is the core of the analysis. The model determines what the likelihood function looks like, which tree topology maximizes it, and how long the branches are. A misspecified model produces biased branch lengths, potentially incorrect topologies, and unreliable tests for selection or molecular clock rates. Getting the model right matters.

A substitution model describes the probabilistic process by which one nucleotide changes to another over evolutionary time. The model is parameterized by an **instantaneous rate matrix Q**, from which the probability of transitioning between states over branch length $t$ is computed as $P(t) = e^{Qt}$ (the matrix exponential). More complex models have more parameters and can better approximate the true substitution process — but also risk overfitting sparse data.

## The Rate Matrix Q

For nucleotides {A, C, G, T}, the instantaneous rate matrix $Q$ has off-diagonal entries $q_{ij} = r_{ij} \pi_j$ (rate of substitution from state $i$ to state $j$, proportional to the equilibrium frequency $\pi_j$ of the target state — **time-reversibility** assumption). The diagonal entries are defined so rows sum to zero: $q_{ii} = -\sum_{j \neq i} q_{ij}$.

The branch length $t$ is measured in expected substitutions per site: $t = -\sum_i \pi_i q_{ii}$ (the expected number of changes per site per unit time, scaled so that $t = 1$ means 1 expected change per site).

The time-reversibility assumption ($\pi_i q_{ij} = \pi_j q_{ji}$) is a mathematical convenience that makes the matrix diagonalizable and the computation tractable. It means the process looks the same whether you read the tree from root to tip or from tip to root — a biologically approximate but computationally essential property of most phylogenetic models.

## JC69: The Equal-Rates Model

**Jukes-Cantor (JC69)** assumes all 12 off-diagonal rate parameters are equal and all four base frequencies are 1/4:

$$Q^{JC} = \begin{pmatrix} -3\alpha & \alpha & \alpha & \alpha \\ \alpha & -3\alpha & \alpha & \alpha \\ \alpha & \alpha & -3\alpha & \alpha \\ \alpha & \alpha & \alpha & -3\alpha \end{pmatrix}$$

One free parameter: $\alpha$ (the overall substitution rate, which scales the branch lengths but doesn't affect topology). JC69 is appropriate for short sequences where parameter estimation is unreliable, or as a starting point. In practice, almost every real DNA dataset has unequal base frequencies and shows transition-transversion bias, so JC69 is usually the wrong model — but it is the foundation from which all more complex models are built, and understanding it is prerequisite to understanding everything else.

## K80/K2P: Transition-Transversion Model

**Kimura 2-parameter (K80/K2P)** distinguishes between **transitions** (pyrimidine↔pyrimidine: C↔T; purine↔purine: A↔G, rate $\alpha$) and **transversions** (purine↔pyrimidine, rate $\beta$). Base frequencies remain equal (1/4). Two free parameters: $\kappa = \alpha/\beta$ (the transition:transversion ratio, **Ti/Tv ratio**) and an overall rate scalar.

In real sequences, transitions are more frequent than transversions (biologically, transitions involve less structural change in the purine/pyrimidine ring), so $\kappa > 1$ (typically 2–10). K2P is more realistic than JC69 for most real datasets.

## HKY85: Unequal Frequencies + Ti/Tv

**HKY85** (Hasegawa, Kishino & Yano, 1985) adds **unequal base frequencies** ($\pi_A, \pi_C, \pi_G, \pi_T$, summing to 1) to K2P's transition-transversion distinction. Four free parameters: κ (Ti/Tv ratio) and three independent base frequencies (four sum to 1, so three are free). HKY85 is a commonly used default model for many datasets.

The addition of unequal frequencies turns out to matter considerably for many biological sequences. Mammalian genomes, for example, tend to have low GC content; bacterial genomes vary enormously in GC content from below 25% to above 75%. Assuming equal frequencies (as JC69 and K2P do) when the true frequencies are strongly skewed produces a model that misrepresents the equilibrium state the sequences are evolving toward.

## GTR: The Most General Time-Reversible Model

**GTR** (General Time Reversible) is the most parameter-rich time-reversible nucleotide model:
- **6 exchangeability parameters** ($r_{AC}, r_{AG}, r_{AT}, r_{CG}, r_{CT}, r_{GT}$, one set to 1 for scaling = 5 free)
- **4 base frequencies** (3 free, as they sum to 1)
- **Total free parameters: 8** (plus branch lengths)

GTR subsumes all simpler models (JC, K2P, HKY, TrN, etc.) as special cases. It is the standard choice for reliable analyses when sufficient data are available.

## Rate Variation Across Sites: +Γ

A major biological reality is that different positions in a sequence evolve at very different rates: active site residues are strongly conserved (slow); synonymous positions and less constrained loops evolve rapidly. Ignoring rate variation leads to underestimation of branch lengths (averaging fast and slow sites biases the distance estimate).

The **+Γ** (plus Gamma) rate variation model assumes that rates across sites follow a **discrete gamma distribution** approximated by $K$ rate categories (usually $K = 4$). The gamma distribution has shape parameter $\alpha$ (not the substitution rate — confusingly reusing Greek letters!):

- Small $\alpha$ (< 1): Highly heterogeneous — many invariant or near-invariant sites mixed with very fast sites.
- Large $\alpha$ (> 5): Nearly homogeneous rates.

The four discrete rate categories and their weights are computed from the gamma distribution's quartiles. The site likelihood is the weighted sum over all four rate categories.

Rate variation is not a minor correction — it is often the most important component of the model. A site that has changed 5 times carries very different information about the tree than a site that has changed 0 times, and treating them identically as a uniform-rate model does produces badly biased branch lengths. The +Γ model gives fast sites and slow sites their appropriate weights.

## +I: Invariable Sites

The **+I** (invariable sites) parameter allows a proportion $p_{\text{inv}}$ of sites to be completely invariable (never change), modeling conserved positions that cannot substitute. The site likelihood becomes:

$$P(D_c) = p_{\text{inv}} \cdot \mathbb{1}[\text{site is constant}] + (1 - p_{\text{inv}}) \cdot P_\Gamma(D_c)$$

**Warning**: +Γ+I models are often redundant — the gamma distribution already allows very low rates (near-invariant sites), and adding the +I component can create statistical identifiability problems. Many researchers now prefer +Γ alone with a sufficient number of rate categories.

## GTR+Γ+I: The Full Model

The **GTR+Γ+I** model combines all the above: general time-reversible substitution rates + gamma rate variation + a proportion of invariable sites. With 5 exchangeabilities + 3 frequencies + 1 shape + 1 inv + branch lengths, this is a highly parameterized model appropriate for datasets with thousands of sites. For shorter alignments (< 500 bp), GTR+Γ may be better to avoid overfitting.

## Why +Γ Matters

An alignment analysis without rate variation will estimate branch lengths that are too short (because fast-evolving sites saturate, contributing less information than expected under a uniform-rate model). Rate-heterogeneous models better fit the data (higher likelihood), give longer and more accurate branch length estimates for fast-evolving lineages, and are substantially more resistant to long-branch attraction artifacts. This connects back to the core failure of parsimony: long-branch attraction occurs because the model (implicit or explicit) fails to account for convergent evolution at fast-evolving sites. The +Γ model, by assigning these sites appropriate rates, substantially reduces the probability of mistaking convergence for shared ancestry.

## Why This Matters

Nucleotide substitution models are the mathematical language in which all likelihood-based phylogenetics is written — choosing an appropriate model (GTR+Γ for most analyses) vs. an oversimplified one (JC) directly affects the accuracy of tree topology, branch lengths, and divergence time estimates; understanding what each parameter represents enables interpretation of model selection results and appropriate use of each model's assumptions.
