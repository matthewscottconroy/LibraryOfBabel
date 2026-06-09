# Flux Sampling

Flux Balance Analysis finds the single flux distribution that maximizes growth. It is enormously useful, but it answers only one question: what does the optimal cell do? It cannot tell you what a real cell does, or what range of behaviors is consistent with the constraints. Real cells do not strictly maximize growth rate. They regulate for robustness, for stress response, for secondary metabolism. A bacterium growing on minimal media in steady state occupies some point in the feasible flux space — and FBA tells you the extreme point of that space, not where the cell actually sits. Flux sampling takes a different approach: instead of optimizing, it asks what does the entire feasible space look like? By drawing random points from the polytope of all steady-state-consistent flux distributions, you get a statistical picture of metabolic flexibility — which reactions are essentially fixed by stoichiometry, which can vary across a wide range, and how different reactions co-vary. This is the metabolic equivalent of asking not "what is the best possible experiment?" but "what outcomes are physically possible?"

**Flux sampling** is a constraint-based method that generates random steady-state flux distributions from the feasible solution space of a metabolic model. Unlike Flux Balance Analysis (FBA), which finds a single optimal point, flux sampling characterizes the full geometric space of all metabolically feasible states — revealing which reactions are highly constrained, which are flexible, and how different reactions co-vary.

## The Feasible Flux Polytope

Given a metabolic model with stoichiometric matrix $S$ and flux bounds, the set of all feasible steady-state fluxes is a convex polytope:

$$\mathcal{F} = \left\{ \mathbf{v} \in \mathbb{R}^n : S\mathbf{v} = \mathbf{0},\; \mathbf{lb} \leq \mathbf{v} \leq \mathbf{ub} \right\}$$

Flux sampling draws uniformly from $\mathcal{F}$. The uniform distribution is the maximally uninformative prior — it does not assume any particular objective function, making it suitable for exploring the full metabolic space without optimality assumptions.

## Sampling Algorithms

### Hit-and-Run (HR) Sampling

The classic algorithm for uniform sampling from convex polytopes. Starting from a feasible point:
1. Choose a random direction $\mathbf{d}$ uniformly from the unit sphere
2. Find the line segment $\mathbf{v} + t\mathbf{d}$ within $\mathcal{F}$: compute $t_\text{min}$ and $t_\text{max}$
3. Sample $t$ uniformly on $[t_\text{min}, t_\text{max}]$
4. Update current point: $\mathbf{v} \leftarrow \mathbf{v} + t\mathbf{d}$
5. Repeat

HR produces a Markov chain that converges to the uniform distribution.

### Optimized GP (OptGP) Sampling

**OptGP** (Megchelenbrink et al., 2014) improves on hit-and-run by:
- Using multiple parallel chains (warm-up via LP)
- Exploiting the network structure to propose better directions
- Implementing "thinning" to reduce autocorrelation

```python
import cobra
from cobra.sampling import sample, OptGPSampler, ACHRSampler
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

# Load model and set up constraints
model = cobra.io.read_sbml_model("iJO1366.xml")  # E. coli K-12

# Optional: add constraint (e.g., minimal glucose uptake for growth)
model.reactions.get_by_id("EX_glc__D_e").lower_bound = -10  # mmol/gDW/h glucose

# FBA first: confirm model is feasible
with model:
    solution = model.optimize()
    print(f"FBA optimal growth: {solution.objective_value:.4f} h⁻¹")

# Flux sampling with OptGP
print("\nRunning OptGP flux sampling (1000 samples, 8 processes)...")
flux_samples = sample(model, n=1000, method="optgp",
                      thinning=100, processes=8, seed=42)

print(f"Sample shape: {flux_samples.shape}")
print(f"  Rows: samples, Columns: reactions")
print(f"\nSample statistics for key reactions:")
key_rxns = ["PFK", "PGI", "PGL", "GND", "PPC", "CS", "ACONTa", "ICDHyr"]
print(flux_samples[key_rxns].describe().T[["mean", "std", "min", "max"]].to_string())
```

## Validating Sample Quality

Samples must be verified to actually lie in the feasible polytope and to be mixing well (not stuck in one region):

```python
def validate_flux_samples(samples, model):
    """
    Validate that flux samples satisfy steady-state and bounds constraints.
    """
    import numpy as np

    S = cobra.util.create_stoichiometric_matrix(model, array_type="dense")
    rxn_ids = [rxn.id for rxn in model.reactions]
    lb = np.array([rxn.lower_bound for rxn in model.reactions])
    ub = np.array([rxn.upper_bound for rxn in model.reactions])

    # Check steady-state constraint: S·v ≈ 0 for each sample
    fluxes = samples[rxn_ids].values
    residuals = S @ fluxes.T  # (n_metabolites, n_samples)
    max_residual = np.abs(residuals).max()
    print(f"Steady-state validation:")
    print(f"  Max |S·v| = {max_residual:.2e} (should be < 1e-6)")

    # Check bounds
    bounds_violation = ((fluxes < lb - 1e-6) | (fluxes > ub + 1e-6)).any(axis=1)
    print(f"  Samples violating bounds: {bounds_violation.sum()}")

    return max_residual < 1e-4 and bounds_violation.sum() == 0

valid = validate_flux_samples(flux_samples, model)
print(f"Sample validation: {'PASS' if valid else 'FAIL'}")
```

## Analyzing Flux Distributions

```python
def flux_variability_from_samples(samples, model):
    """
    Compute flux range and flexibility index from samples.
    Comparable to FVA (Flux Variability Analysis) but distribution-based.
    """
    stats = samples.describe().T
    stats["range"] = stats["max"] - stats["min"]
    stats["cv"] = stats["std"] / (stats["mean"].abs() + 1e-10)  # coefficient of variation

    # Flexibility index: 0 = completely fixed, 1 = maximally variable
    lb_arr = np.array([model.reactions.get_by_id(r).lower_bound
                       for r in samples.columns if r in model.reactions])
    ub_arr = np.array([model.reactions.get_by_id(r).upper_bound
                       for r in samples.columns if r in model.reactions])
    possible_range = ub_arr - lb_arr
    possible_range[possible_range == 0] = 1  # avoid division by zero

    stats["flexibility"] = stats["range"] / possible_range

    # Fixed reactions: nearly constant flux (std < 1% of range)
    fixed = stats[stats["flexibility"] < 0.01]
    flexible = stats[stats["flexibility"] > 0.5]

    print(f"Reaction flexibility analysis ({len(samples.columns)} reactions):")
    print(f"  Fixed reactions (flexibility < 1%):  {len(fixed)}")
    print(f"  Flexible reactions (flexibility > 50%): {len(flexible)}")
    print(f"\nMost flexible reactions:")
    print(flexible["flexibility"].nlargest(10).to_string())
    return stats

flux_stats = flux_variability_from_samples(flux_samples, model)
```

## Flux Correlation Analysis

Pairs of reactions with strongly correlated fluxes across samples are **coupled reactions** — they must operate together or not at all. This reveals functional modules:

```python
def flux_coupling_analysis(samples, threshold=0.9):
    """
    Identify coupled reaction pairs from flux sample correlations.
    Spearman correlation used (robust to non-normal flux distributions).
    """
    from scipy.stats import spearmanr

    n_rxns = len(samples.columns)
    rxn_ids = samples.columns.tolist()
    corr_matrix = samples.corr(method="spearman")

    # Find highly correlated pairs
    high_corr = []
    for i in range(n_rxns):
        for j in range(i+1, n_rxns):
            rho = corr_matrix.iloc[i, j]
            if abs(rho) >= threshold:
                high_corr.append({
                    "reaction_1": rxn_ids[i],
                    "reaction_2": rxn_ids[j],
                    "spearman_rho": rho,
                    "type": "coupled" if rho > 0 else "anti-coupled"
                })

    df_coupled = pd.DataFrame(high_corr).sort_values("spearman_rho", ascending=False)
    print(f"Flux-coupled reaction pairs (|ρ| ≥ {threshold}): {len(df_coupled)}")
    print(df_coupled.head(10).to_string(index=False))

    # Visualize correlation heatmap for key reactions
    key_rxns = ["PFK", "PGI", "PYK", "CS", "ACONTa", "ICDHyr",
                "SUCOAS", "SUCD1", "FUM", "MDH"]
    available = [r for r in key_rxns if r in samples.columns]
    if available:
        fig, ax = plt.subplots(figsize=(8, 7))
        corr_sub = samples[available].corr(method="spearman")
        im = ax.imshow(corr_sub, cmap="RdBu_r", vmin=-1, vmax=1)
        ax.set_xticks(range(len(available)))
        ax.set_yticks(range(len(available)))
        ax.set_xticklabels(available, rotation=45, ha="right")
        ax.set_yticklabels(available)
        plt.colorbar(im, ax=ax, label="Spearman ρ")
        ax.set_title("Flux correlation: central carbon metabolism")
        plt.tight_layout()

    return df_coupled

coupled = flux_coupling_analysis(flux_samples)
```

## Comparing Metabolic States

Flux sampling is powerful for comparing metabolic states between conditions (wild-type vs. mutant, nutrient-replete vs. starved, cancer vs. normal):

```python
def compare_metabolic_states(model_wt, model_mutant, n_samples=500):
    """
    Compare flux distributions between two conditions using flux sampling.
    Tests for significant differences using Mann-Whitney U test.
    """
    from scipy.stats import mannwhitneyu
    from statsmodels.stats.multitest import multipletests

    samples_wt  = sample(model_wt, n=n_samples, method="optgp", seed=42)
    samples_mut = sample(model_mutant, n=n_samples, method="optgp", seed=42)

    # Test each reaction for differential flux
    results = []
    common_rxns = [r for r in samples_wt.columns if r in samples_mut.columns]

    for rxn_id in common_rxns:
        wt_flux  = samples_wt[rxn_id].values
        mut_flux = samples_mut[rxn_id].values
        stat, pval = mannwhitneyu(wt_flux, mut_flux, alternative="two-sided")
        results.append({
            "reaction": rxn_id,
            "wt_mean": wt_flux.mean(),
            "mut_mean": mut_flux.mean(),
            "delta_flux": mut_flux.mean() - wt_flux.mean(),
            "p_value": pval
        })

    df = pd.DataFrame(results)
    _, q_values, _, _ = multipletests(df["p_value"], method="fdr_bh")
    df["q_value"] = q_values

    significant = df[df["q_value"] < 0.05].sort_values("delta_flux", key=abs, ascending=False)
    print(f"Differentially active reactions (FDR < 5%): {len(significant)}")
    print(significant[["reaction", "wt_mean", "mut_mean", "delta_flux", "q_value"]].head(15).to_string())
    return df
```

## Why This Matters

Flux sampling bridges the gap between constraint-based modeling (which tells us what is feasible) and a statistical description of metabolic behavior (which tells us what is typical, variable, or constrained). Unlike FBA, which assumes a single objective function (growth rate maximization) that may not apply to all conditions or organisms, flux sampling makes no assumptions about objective. It reveals the full metabolic flexibility of an organism — which pathways are metabolically equivalent (highly correlated fluxes) and which are independently regulated. In cancer metabolism research, comparing flux samples from normal vs. oncogene-expressing cells identifies the metabolic reprogramming that supports tumor growth and reveals metabolic vulnerabilities specific to the cancer state. In metabolic engineering, flux variability analysis from samples identifies reactions that cannot be changed without disrupting growth (fixed reactions = essential reactions = poor engineering targets) vs. reactions that can be freely modulated (flexible reactions = candidate engineering points).
