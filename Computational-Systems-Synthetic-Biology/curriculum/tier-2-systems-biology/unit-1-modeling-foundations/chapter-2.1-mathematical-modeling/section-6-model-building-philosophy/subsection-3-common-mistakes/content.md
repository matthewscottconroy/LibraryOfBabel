# Common Mistakes in Biological Modeling

## Why Enumerate Mistakes?

Understanding common pitfalls is as important as understanding correct methods. Mistakes in biological modeling are often subtle — the equations look correct, the simulation runs, and the output resembles experimental data — yet the model is fundamentally wrong. The insidiousness of modeling mistakes lies in their invisibility: unlike a failed experiment that obviously fails, a model with the wrong mechanism can produce output that looks entirely reasonable, fit data that happens to be consistent with its predictions, and lead the scientist confidently in the wrong direction.

The mistakes below are drawn from the literature and from common student errors. Each is presented with its mechanism, its consequence, and its remedy. Learning to recognize these failure modes, and to detect them before they undermine a scientific conclusion, is among the most practically valuable skills in quantitative biology.

## Mistake 1: Over-Parameterization

**What it is**: including more free parameters than the data can constrain. A model with $k$ parameters and $n < k$ independent data points can always be made to fit the data exactly — but it explains nothing.

**How it happens**: starting with a complex model because the biological system is complex; including mechanistic details "just to be safe"; treating all parameters as free even when literature values are available.

**Consequence**: the model fits training data perfectly but generalizes poorly. Parameters are non-identifiable (multiple parameter sets give identical fits). The model is unfalsifiable: it can accommodate almost any experimental outcome.

**Remedy**: count the number of free parameters versus independent data points. Apply AIC (Akaike Information Criterion) or BIC (Bayesian Information Criterion) to compare models of different complexity. Fix literature-constrained parameters at their known values rather than re-estimating them.

$$\text{AIC} = 2k - 2\ln(\hat{L}), \quad \text{BIC} = k\ln(n) - 2\ln(\hat{L})$$

where $k$ is the number of parameters, $n$ is the number of data points, and $\hat{L}$ is the maximum likelihood. Lower values indicate better models, penalized for complexity.

## Mistake 2: Ignoring Identifiability

**What it is**: fitting parameters that cannot be uniquely determined from the available data (structural or practical non-identifiability, Section 2.1.5.2).

**How it happens**: fitting without checking whether the best-fit minimum is a unique global minimum; using point estimates from a single optimization run without exploring the parameter landscape.

**Consequence**: reported parameters appear precise (narrow confidence intervals from Hessian-based methods) but are actually undetermined — the likelihood is flat in certain parameter directions. The model "works" for training data but makes different predictions depending on which non-identifiable parameter is chosen.

**Remedy**: compute profile likelihoods for all parameters. Perform structural identifiability analysis (DAISY, SIAN) before fitting. Report confidence intervals using profile likelihood, not just Hessian approximations.

## Mistake 3: Unstated Assumptions

**What it is**: implicit assumptions embedded in the model equations that are not stated, tested, or justified.

**Examples**:
- QSSA applied when $[E]_0 \gg [S]_0$ (violating the QSSA condition $[E]_0 \ll [S]_0$)
- Treating a highly expressed gene as constitutively expressed when single-cell data show bimodal distributions
- Using dimensionless concentrations without specifying the reference state
- Assuming negligible mRNA dynamics when mRNA half-life is comparable to protein half-life

**Consequence**: the model gives wrong predictions in regimes where the unstated assumption fails, without any warning. Because the assumption was never stated, readers cannot identify when the model should fail.

**Remedy**: explicitly state every assumption in the model description. Test each assumption quantitatively. Report the conditions under which the model is expected to be valid. A model with explicit, tested assumptions is not "less rigorous" — it is more trustworthy precisely because you know exactly where it might break down.

## Mistake 4: Units and Dimensional Inconsistency

**What it is**: mixing incompatible units (per second vs. per minute; µM vs. mM; concentrations vs. molecule counts).

**How it happens**: assembling parameters from different literature sources with different conventions; using dimensionless model formulations without tracking the dimensional correspondence; copying bimolecular rate constants without accounting for volume.

**Consequence**: model predictions are off by orders of magnitude; parameters appear implausibly large or small. Simulations may still run without error, producing numerically sensible output that is biologically nonsensical. This mistake is particularly hard to detect because the simulation does not crash — it just gives wrong numbers that happen to look plausible.

**Remedy**: always write units for every parameter and every term in every equation. Check that both sides of each ODE have consistent units (concentration / time = sum of rate terms with units concentration / time). Use unit-tracking libraries (Pint in Python) for complex models.

## Mistake 5: Confusing Correlation with Mechanism

**What it is**: assuming that a model "explains" a phenomenon because it reproduces the data, without establishing that the assumed mechanism is the actual cause.

**Example**: a positive feedback loop produces bistability. Bistability is observed. Conclusion: the system has a positive feedback loop. This is affirming the consequent — many other mechanisms can also produce bistability (double negative feedback, cooperative binding with sufficient nonlinearity, etc.).

**Consequence**: the model correctly predicts existing data for the wrong reasons. Novel perturbation predictions fail because the mechanism is wrong. You commit resources to testing or engineering the wrong mechanism.

**Remedy**: make predictions that specifically depend on the mechanistic assumption, not just the qualitative behavior. If the system is bistable due to positive feedback, specific perturbations of the feedback strength should shift the bifurcation in predictable ways. Test these mechanism-specific predictions. A model that fits the data is suggestive; a model whose mechanism-specific predictions are also correct is compelling.

## Mistake 6: Applying Deterministic Models to Stochastic Systems

**What it is**: using an ODE model to interpret data from systems where molecule copy numbers are low (mRNA, transcription factors, signaling molecules near threshold).

**Consequence**: the model predicts a sharp threshold response where the actual response is graded (due to noise); the model predicts a single stable state where cells show bimodal behavior; the model underestimates cell-to-cell variability.

**Remedy**: assess whether the system is in the stochastic regime (Section 2.1.3.1). If so, use a stochastic model and analyze distributions, not means.

## Mistake 7: Training = Validation Data

**What it is**: using the same data to fit the model and to evaluate how well it fits. Reporting the fit to training data as evidence of model validity.

**Consequence**: any flexible model can be made to fit training data; this is not evidence of mechanistic correctness or predictive ability.

**Remedy**: reserve a portion of the data as a held-out validation set. Use the fitted model to predict the validation set without re-optimization. Alternatively, design and perform new experiments that test model predictions. The gold standard is to make a prediction from the model before collecting the new data, then compare.

## Mistake 8: Single Optimal Trajectory

**What it is**: running the model at a single "best-fit" parameter set and reporting the resulting trajectory as if it were the model prediction, without reporting uncertainty.

**Consequence**: overstates confidence in the model; suppresses the range of predictions that is actually consistent with the data.

**Remedy**: report trajectories sampled from the posterior distribution or the profile-likelihood-based confidence region. Show that qualitative conclusions (e.g., "the system is bistable") hold throughout the uncertainty range. A conclusion that depends on a single parameter set is fragile; one that holds across the full confidence region is robust.

## Why This Matters

Mistakes in biological modeling have consequences beyond the individual publication: wrong mechanistic models shape experimental intuition, guide drug target selection, and influence how graduate students are trained to think about biological systems. The field has gradually developed institutional practices to reduce common mistakes — sharing model code, reporting sensitivity analyses, standardizing formats — and these practices exist precisely because the mistakes described here are common and consequential.

Awareness of pitfalls is not pessimism; it is the foundation of rigorous, trustworthy quantitative biology. Every mistake in this list has a straightforward remedy. The hard part is not knowing the remedy — it is building the habits that make you apply the remedy every time, not just when you suspect a problem. Robust modeling practice means applying these checks routinely, as part of normal workflow, regardless of whether any particular model seems suspicious. That discipline is what separates quantitative biology that advances knowledge from quantitative biology that generates equations.
