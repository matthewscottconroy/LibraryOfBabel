# MrBayes and BEAST2

Bayesian phylogenetics has two principal software implementations, and choosing between them is not arbitrary — they are designed for different questions. If you want the best estimate of the tree topology and want to quantify your uncertainty about it, MrBayes is your tool. If you want to know when the divergences happened — when lineages split, how fast a virus is evolving, how the effective population size of a species changed through time — BEAST2 is your tool. Understanding what each software is actually doing, and when to use each one, is as important as knowing the underlying mathematics.

Two software packages dominate Bayesian phylogenetics: **MrBayes** for topology estimation with fixed time assumptions, and **BEAST2** for divergence time estimation with calibrated molecular clocks. While both use MCMC sampling from the posterior distribution, they differ substantially in their models, inputs, and intended questions.

## MrBayes: Fixed Topology Estimation

**MrBayes** (Ronquist & Huelsenbeck, 2003; Ronquist et al., 2012) is the standard Bayesian phylogenetic software for estimating tree topologies and testing phylogenetic hypotheses. It focuses on accurately recovering the tree topology and branch lengths in units of substitutions per site.

**Key features**:

**Partitioned analysis**: MrBayes natively supports partitioned models where different data blocks (genes, codon positions, morphological characters) each have their own substitution model. Different partitions can have unlinked substitution parameters but share the same topology.

**Mixed models**: Different substitution models can be mixed across partitions within a single analysis.

**Marginal model comparison**: MrBayes implements stepping-stone sampling and thermodynamic integration to estimate the marginal likelihood for model comparison (Bayes factors), which is more principled than AIC/BIC for Bayesian model selection.

**Summary tree**: After MCMC, the `sumt` command computes the **Maximum Clade Credibility (MCC) tree** — the tree topology with the highest product of posterior probabilities across all clades. The consensus tree annotated with posterior probabilities is the standard output.

```bash
# MrBayes nexus block:
begin mrbayes;
    set autoclose=yes;
    lset nst=6 rates=gamma;  # GTR+G
    mcmcp ngen=1000000 samplefreq=1000 printfreq=1000;
    mcmc;
    sumt burnin=250;  # 25% burn-in
    sump burnin=250;
end;
```

Running this analysis produces a consensus tree where each node is annotated with its posterior probability. A node with PP = 0.98 means 98% of the trees sampled after burn-in contained that clade. Reading the `sump` output in Tracer to verify ESS > 200 for all parameters is an essential step before trusting the results.

## BEAST2: Divergence Time Estimation

**BEAST2** (Bouckaert et al., 2014, 2019) is designed for **molecular clock dating** — estimating not just the tree topology but also the absolute ages of divergence events (in years or millions of years). BEAST2 integrates the phylogenetic model with molecular clock models and demographic models, enabling joint inference of tree topology, branch lengths (in time units), and population parameters.

**BEAUti**: BEAST2 does not take command-line parameters; instead, analysis configuration is done through the **BEAUti** graphical interface, which generates an XML configuration file that BEAST2 reads. BEAUti allows the user to: import alignment, assign partitions, set substitution models, configure clock models, set calibration priors, and choose tree priors.

**Molecular clock models** (see Subsection 1, Section 7):
- **Strict clock**: All branches evolve at the same rate (single rate parameter).
- **Uncorrelated lognormal (UCLN)**: Each branch has an independent rate drawn from a lognormal distribution. Most flexible; preferred when strict clock is rejected.

**Tree priors (demographic models)**:
- **Yule process**: Pure birth process (no extinction); appropriate for species phylogenies.
- **Birth-death**: Speciation and extinction rates; more realistic for macroevolution.
- **Coalescent**: For intraspecific population-level data.
- **BDSKY** (Birth-Death Skyline): Time-varying birth-death rates for phylodynamics (virus epidemics).

**Tip dating**: For rapidly evolving organisms (RNA viruses), the sampling dates of sequences serve as calibration points. BEAST2 uses the sample collection dates (provided in the XML) as fixed time anchors, enabling estimation of substitution rates and divergence times without fossil calibrations.

**TreeAnnotator**: Post-analysis, **TreeAnnotator** processes the BEAST2 output (a file of sampled trees) to create the Maximum Clade Credibility tree with posterior probability and node age (95% HPD interval) annotations. The MCC tree is the primary phylogenetic result.

**Tracer**: Run **Tracer** on the BEAST2 log file to verify convergence (ESS > 200 for all continuous parameters, flat traces, adequate mixing) before interpreting results. This step is not optional — interpreting results from a non-converged BEAST2 run is a common and consequential error in the literature.

## When to Use BEAST2

BEAST2 is the method of choice when:
1. **Divergence time estimation** is needed (molecular clock dating with fossil calibrations or tip dates).
2. **Phylodynamics**: Estimating viral reproductive numbers ($R_0$), epidemic growth rates, or SARS-CoV-2 variant emergence timing.
3. **Population-level analyses**: Estimating effective population size through time (Bayesian Skyline Plot).
4. **Co-phylogenetics**: Jointly inferring host-parasite coevolution timelines.

MrBayes is preferred when: (1) only topology and branch lengths (in substitutions/site) are needed; (2) large partitioned datasets where BEAST2's integrated clock model would be computationally prohibitive.

## BEAST2 Package Ecosystem

BEAST2's functionality is substantially extended through a package system (installed via the BEAUti Package Manager):
- **BDSKY**: Birth-death skyline model for phylodynamics.
- **BEASTLabs**: Utilities and additional operators.
- **TreeStat2**: Tree statistic computation from BEAST2 output.
- **SNAPP**: SNP-based species tree inference.
- **StarBEAST3**: *BEAST model for multispecies coalescent (species trees with gene trees).

## Why This Matters

MrBayes and BEAST2 represent the gold standard for Bayesian phylogenetic analysis across the full spectrum of evolutionary questions — from tree topology uncertainty quantification (MrBayes) to viral epidemic dating and ancestral population size reconstruction (BEAST2) — making proficiency with these tools essential for modern evolutionary genomics, phylodynamics, and molecular paleovirology. Together, these two tools answer the two fundamental questions of phylogenetics: what is the tree, and when did it happen?
