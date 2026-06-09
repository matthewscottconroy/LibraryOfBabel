# Model Selection

Here is an uncomfortable truth about phylogenetic models: you never know which one is correct, because the true process of molecular evolution is far more complex than any model you can write down. What you can do is find the model that best approximates reality for your specific dataset — one that captures the most important sources of variation without wasting parameters on structure the data cannot support. This is model selection, and it is not a formality you run before the "real" analysis. It is part of the analysis itself.

Choosing an appropriate substitution model is a critical step in maximum likelihood phylogenetics. An under-parameterized model (e.g., JC69 when the data clearly have unequal base frequencies and rate variation) produces biased branch length estimates and potentially incorrect topologies. An over-parameterized model wastes degrees of freedom on parameters that cannot be reliably estimated from the data, increasing variance without improving accuracy. Model selection provides a principled, data-driven approach to finding the best-fitting model for a given dataset.

## Information Criteria

The two dominant criteria for model comparison in phylogenetics are **AIC** (Akaike Information Criterion) and **BIC** (Bayesian Information Criterion):

$$\text{AIC} = 2k - 2\ln L$$
$$\text{BIC} = k \ln n - 2\ln L$$

where $k$ = number of free parameters in the model (substitution rates + frequencies + branch lengths), $n$ = number of sites (observations), $\ln L$ = log-likelihood of the fitted model.

Both criteria penalize model complexity (higher $k$) to avoid overfitting, but with different penalty magnitudes. For large $n$ (genomic datasets), BIC penalizes complexity more heavily than AIC. **Lower AIC or BIC = better model**.

Key difference in model selection behavior:
- **AIC** selects for predictive accuracy — prefers models that best predict new observations. Tends to choose slightly more complex models.
- **BIC** selects for parsimony — prefers the most parsimonious explanation. Approximates the Bayesian marginal likelihood under flat priors. Tends to select simpler models for large datasets.

For phylogenetics with large modern datasets (>100 sites, >10 taxa), BIC is generally preferred because the large $n$ term in BIC appropriately penalizes the many branch length parameters.

## Likelihood Ratio Test (LRT) for Nested Models

When one model is a special case of another (**nested models**), the likelihood ratio test provides a formal hypothesis test:

$$\Lambda = 2(\ln L_1 - \ln L_0) \approx \chi^2_{\text{df}}$$

where $\text{df} = k_1 - k_0$ (difference in number of parameters). Example: JC69 (1 free parameter) vs. K2P (2 free parameters, adds κ). If $\Lambda > \chi^2_{\text{df}=1, \alpha=0.05} = 3.84$, reject JC69 in favor of K2P.

The LRT is valid only for **nested models** (one is a special case of the other). GTR and K2P are nested (K2P is GTR with equal frequencies and equal transversions). But GTR and HKY are not comparably nested with mixture models (C20), so LRT cannot directly compare these — you need AIC or BIC when models are not nested.

## ModelTest-NG and IQ-TREE ModelFinder

**ModelTest-NG** (Darriba et al., 2020) and **IQ-TREE ModelFinder** are the current standard tools for automated phylogenetic model selection. They:

1. Fit a comprehensive set of substitution models (up to 88 nucleotide models, or hundreds of amino acid models) to the alignment using maximum likelihood.
2. Compute AIC, AICc (corrected AIC for small $n/k$ ratios), and BIC for each model.
3. Report the best-fitting model under each criterion.

```bash
# IQ-TREE2 ModelFinder (nucleotide data)
iqtree2 -s alignment.fasta -m TEST -msub nuclear -AIC -BIC

# Output includes:
# Best-fit model: GTR+F+G4 chosen according to BIC
# Model                   LogL         AIC         BIC
# JC                   -12345.6   24691.2   24695.4
# HKY+F+G4            -11234.7   22473.4   22485.8
# GTR+F+G4            -11198.3   22408.6   22431.0
```

Looking at this output, you can see exactly how much the likelihood improves as models become more complex, and how the BIC penalty eventually reverses the preference when models become too parameter-rich. The difference in log-likelihood between JC and GTR+F+G4 here is over 1,147 units — a dramatically better fit. The BIC confirms this improvement is real and not just overfitting.

## Practical Advice: Safe Defaults

For most analyses, before running formal model selection, use these safe defaults:

**Nucleotide data**: **GTR+Γ** (or GTR+G4 in IQ-TREE notation, using 4 discrete gamma categories). GTR is the most general time-reversible model; +Γ handles rate heterogeneity. Adding +I (invariable sites) is usually unnecessary and can cause identifiability issues with +Γ.

**Protein data**: **LG+Γ+F** (LG exchangeabilities, gamma rate variation, frequencies estimated from the data). For deep evolutionary analyses or diverse datasets, consider C20+Γ or C60+Γ.

**Codon data**: For dN/dS analysis: GY94 (Goldman-Yang 1994) codon model.

## The Cost of Overparameterization

Adding parameters always increases the log-likelihood (or leaves it unchanged). AIC and BIC penalize this to prevent overfitting. In practice, the greatest risk of overparameterization in phylogenetics is not in the substitution model parameters (which are well-penalized) but in the **number of partitions** (see partitioned analysis). Having one partition with its own GTR+Γ model for every gene in a 1,000-gene phylogenomics dataset is almost certainly overparameterized; PartitionFinder or IQ-TREE's ModelFinder with partition merging addresses this.

It turns out that a common mistake is to run model selection once and never revisit it. Model selection should be re-run if you significantly change your alignment (add taxa, trim sequences) because the best model depends on the data. The model selected for a 50-taxon, 5,000-site dataset may not be appropriate for the 5-taxon, 500-site subset you use for a specific hypothesis test.

## Why This Matters

Model selection is not a formality — using JC69 instead of GTR+Γ for a dataset with clear rate variation produces systematically short branches and potentially incorrect topologies for long-diverged taxa; the widespread availability of automated tools like ModelFinder makes correct model selection straightforward and should be a routine first step in any maximum likelihood phylogenetic analysis. The few minutes it takes to run ModelFinder are worth it: the best-fit model is almost always more informative than the default, and the improvement in tree accuracy can be substantial.
