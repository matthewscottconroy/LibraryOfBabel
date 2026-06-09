# Synthetic Lethality Networks

In 1998, Mark O'Connor and colleagues at the ICRF in London published a paper proposing that BRCA1/BRCA2-mutant cancer cells — deficient in homologous recombination — might be selectively killed by inhibiting a second DNA repair pathway. The idea was elegant in its logic: cancer cells have already lost one repair route (the mutation they carry), so you kill them by closing the other. Normal cells, with both routes intact, survive. The concept had a name — synthetic lethality — borrowed from the classical genetics literature, where double mutants die while each single mutant survives. What nobody fully anticipated in 1998 was that this genetic logic would translate into some of the most successful anticancer drugs of the following two decades. PARP inhibitors, approved for BRCA-mutant ovarian and breast cancers, are the direct clinical implementation of a network genetics principle. Understanding how to find more such interactions — computationally, at scale — is now one of the most active areas in precision oncology.

**Synthetic lethality** occurs when two gene knockouts are individually viable but lethal in combination. In cancer, where one gene is already inactivated by mutation, its synthetic lethal partner becomes an essential drug target: killing only cancer cells (which lack the first gene) while leaving normal cells unharmed.

## Definition and Genetic Interaction Score

Two genes $A$ and $B$ exhibit a **genetic interaction** if the double mutant fitness $W_{AB}$ deviates from the expectation based on single mutant fitnesses:

$$\varepsilon_{AB} = W_{AB} - W_A \cdot W_B$$

where fitness $W$ is typically measured as colony growth rate relative to wild type (0 = lethal, 1 = wild type, > 1 = growth advantage).

- **Synthetic lethality**: $\varepsilon < 0$, $W_A > 0$, $W_B > 0$, $W_{AB} \approx 0$
- **Synthetic sick**: $\varepsilon \ll 0$ but $W_{AB} > 0$
- **Buffering (positive genetic interaction)**: $\varepsilon > 0$; double mutant grows better than expected (redundant pathways)

```python
import numpy as np
import pandas as pd
import networkx as nx
import matplotlib.pyplot as plt

def compute_epistasis(W_A, W_B, W_AB):
    """
    Compute epistasis score for a gene pair.
    Multiplicative model: expected W_AB = W_A * W_B
    """
    expected = W_A * W_B
    epsilon = W_AB - expected

    interaction_type = "neutral"
    if epsilon < -0.08 and W_AB < 0.3:
        interaction_type = "synthetic lethal"
    elif epsilon < -0.08:
        interaction_type = "synthetic sick"
    elif epsilon > 0.08:
        interaction_type = "buffering"

    return {"epsilon": epsilon, "expected": expected, "observed": W_AB,
            "type": interaction_type}

# Example: BRCA1 and PARP1 synthetic lethality
example = compute_epistasis(W_A=0.85, W_B=0.90, W_AB=0.02)
print(f"BRCA1 × PARP1 example:")
print(f"  Observed double mutant fitness: {example['observed']:.2f}")
print(f"  Expected (W_A × W_B):         {example['expected']:.2f}")
print(f"  Epistasis ε:                   {example['epsilon']:.2f}")
print(f"  Interaction type:              {example['type']}")
```

## The Global Genetic Interaction Network

The **Costanzo et al. (2010, 2016)** yeast genetic interaction network is the most comprehensive global measurement of genetic interactions:
- ~23 million gene pairs measured in *S. cerevisiae*
- ~550,000 significant interactions (|ε| > 0.08, p < 0.05)
- Reveals the functional architecture of the yeast genome

```python
def load_genetic_interaction_network(tsv_file, epsilon_threshold=0.08,
                                      fdr_threshold=0.05):
    """
    Load yeast genetic interaction data (Costanzo et al. format).
    Builds separate positive and negative interaction networks.
    """
    df = pd.read_csv(tsv_file, sep="\t")
    # Columns: Query gene, Array gene, epsilon score, p-value, FDR

    # Filter for significant interactions
    sig = df[(df["FDR"] <= fdr_threshold) &
             (df["epsilon"].abs() >= epsilon_threshold)]

    print(f"Significant genetic interactions: {len(sig)}")
    print(f"  Synthetic lethal (ε < -{epsilon_threshold}): "
          f"{(sig['epsilon'] < -epsilon_threshold).sum()}")
    print(f"  Buffering (ε > {epsilon_threshold}): "
          f"{(sig['epsilon'] > epsilon_threshold).sum()}")

    # Build signed genetic interaction network
    G_neg = nx.Graph()  # negative (synthetic sick/lethal)
    G_pos = nx.Graph()  # positive (buffering)

    for _, row in sig.iterrows():
        gene_a = row["Query Gene"]
        gene_b = row["Array Gene"]
        eps = row["epsilon"]

        if eps < 0:
            G_neg.add_edge(gene_a, gene_b, weight=abs(eps))
        else:
            G_pos.add_edge(gene_a, gene_b, weight=eps)

    print(f"\nNegative network (synthetic lethal/sick):")
    print(f"  {G_neg.number_of_nodes()} genes, {G_neg.number_of_edges()} edges")
    print(f"Positive network (buffering):")
    print(f"  {G_pos.number_of_nodes()} genes, {G_pos.number_of_edges()} edges")

    return G_neg, G_pos
```

## Structural Properties of the Genetic Interaction Network

The genetic interaction network has a distinct topology from PPI networks:

```python
def analyze_gi_network_structure(G_neg, G_pos):
    """
    Analyze the structure of genetic interaction networks.
    Key property: negative GI networks have a 'spoke' pattern around functional modules.
    """
    # Negative interactions: 'alleviating' pattern within pathways
    # Positive interactions: 'aggravating' pattern across pathways

    # Community detection on negative network
    from networkx.algorithms.community import louvain_communities
    communities = louvain_communities(G_neg, seed=42, resolution=1.0)
    print(f"Negative GI network: {len(communities)} communities")

    # Check: are members of the same pathway positively interacting?
    # (redundant paralogs within a pathway buffer each other)

    # Degree distribution in GI network
    degrees_neg = [d for _, d in G_neg.degree()]
    print(f"\nNegative GI degree statistics:")
    print(f"  Mean: {np.mean(degrees_neg):.1f}")
    print(f"  Max:  {max(degrees_neg)}")
    print(f"  Top hubs (most synthetic sick interactions):")

    top_neg = sorted(G_neg.degree(), key=lambda x: -x[1])[:5]
    for gene, k in top_neg:
        print(f"    {gene}: {k} negative interactions")

    return communities
```

## Cancer-Specific Synthetic Lethality

In cancer, the therapeutic goal is to identify a gene $B$ such that:
1. Gene $A$ is mutated/deleted in the cancer but not in normal tissue
2. Inhibiting $B$ is lethal in cancer (which lacks $A$) but tolerable in normal cells (which have $A$)

```python
def identify_cancer_synthetic_lethals(gi_network, cancer_driver_genes,
                                       essential_gene_threshold=0.5):
    """
    Identify synthetic lethal partners of cancer driver genes.
    
    gi_network: negative genetic interaction network
    cancer_driver_genes: list of genes inactivated in cancer
    essential_gene_threshold: minimum epistasis score to consider
    Returns: ranked candidate drug targets
    """
    candidates = {}

    for driver in cancer_driver_genes:
        if driver not in gi_network:
            continue
        # Get all synthetic lethal partners
        for partner, data in gi_network[driver].items():
            eps = data.get("weight", 0)
            if eps >= essential_gene_threshold:
                candidates[partner] = candidates.get(partner, 0) + eps

    # Rank by total synthetic lethality score across all drivers
    ranked = sorted(candidates.items(), key=lambda x: -x[1])

    print("Candidate synthetic lethal drug targets:")
    print(f"  (Partners of: {', '.join(cancer_driver_genes[:5])}...)")
    print(f"\n{'Gene':<20} {'SL score':>12} {'Notes'}")
    print("-" * 50)
    for gene, score in ranked[:10]:
        print(f"  {gene:<20} {score:>12.3f}")

    return ranked

# Classic example: BRCA1/BRCA2 → PARP1/PARP2 synthetic lethality
# Forms the basis for olaparib, niraparib, rucaparib (FDA approved)
cancer_drivers = ["BRCA1", "BRCA2", "TP53", "ATM"]  # HR pathway mutants
print("Synthetic lethality basis of PARP inhibitors:")
print("  BRCA1/2 mutation → defective homologous recombination (HR)")
print("  PARP inhibition → single-strand break repair failure")
print("  In BRCA-mutant cells: BOTH repair pathways lost → synthetic lethality")
print("  In normal cells: HR intact → PARP inhibition tolerated")
```

## CRISPR Screens for Synthetic Lethality Discovery

Genome-wide CRISPR screens in cancer cell lines with known driver mutations identify synthetic lethals at scale:

```python
def analyze_crispr_sl_screen(lfc_file, background_gene_set=None,
                              fdr_threshold=0.05, lfc_threshold=-1.0):
    """
    Analyze a CRISPR negative selection screen in cancer cell lines.
    Genes with strong depletion (negative LFC) in mutant but not WT lines
    are synthetic lethal with the cancer driver.
    
    lfc_file: DataFrame with columns [gene, LFC_mutant, LFC_WT]
    """
    df = pd.read_csv(lfc_file)

    # Differential essentiality: depleted in mutant but not WT
    df["delta_LFC"] = df["LFC_mutant"] - df["LFC_WT"]

    # Synthetic lethal candidates: depleted in mutant, not in WT
    sl_candidates = df[
        (df["LFC_mutant"] < lfc_threshold) &
        (df["LFC_WT"] > lfc_threshold - 0.5) &
        (df["delta_LFC"] < -1.0)
    ].sort_values("delta_LFC")

    print(f"CRISPR screen: {len(df)} genes screened")
    print(f"Synthetic lethal candidates (LFC_mutant < {lfc_threshold}, "
          f"selective): {len(sl_candidates)}")
    print(sl_candidates[["gene", "LFC_mutant", "LFC_WT", "delta_LFC"]].head(10).to_string())
    return sl_candidates

# Simulated CRISPR screen data for illustration
np.random.seed(42)
n_genes = 5000
crispr_data = pd.DataFrame({
    "gene": [f"GENE{i}" for i in range(n_genes)],
    "LFC_mutant": np.random.normal(-0.2, 0.8, n_genes),
    "LFC_WT": np.random.normal(-0.05, 0.6, n_genes)
})
# Inject 50 true synthetic lethals
sl_genes = np.random.choice(n_genes, 50, replace=False)
crispr_data.loc[sl_genes, "LFC_mutant"] -= 2.0

# Save temporarily for analysis
crispr_data.to_csv("/tmp/crispr_screen.csv", index=False)
result = analyze_crispr_sl_screen("/tmp/crispr_screen.csv")
```

## Network-Predicted vs. Experimentally Validated SL

```python
def compare_network_prediction_to_experiment(predicted_sl, experimental_sl,
                                              all_genes):
    """
    Evaluate how well network-based SL prediction matches CRISPR screen results.
    """
    from sklearn.metrics import roc_auc_score, average_precision_score

    predicted_set = set(predicted_sl)
    experimental_set = set(experimental_sl)

    # Overlap analysis
    overlap = predicted_set & experimental_set
    precision = len(overlap) / len(predicted_set) if predicted_set else 0
    recall = len(overlap) / len(experimental_set) if experimental_set else 0

    if precision + recall > 0:
        f1 = 2 * precision * recall / (precision + recall)
    else:
        f1 = 0

    print(f"Network prediction vs. CRISPR screen:")
    print(f"  Predicted SL: {len(predicted_set)}")
    print(f"  Experimentally validated: {len(experimental_set)}")
    print(f"  Overlap: {len(overlap)}")
    print(f"  Precision: {precision:.3f}")
    print(f"  Recall: {recall:.3f}")
    print(f"  F1: {f1:.3f}")
    print(f"  Enrichment: {precision/(len(experimental_set)/len(all_genes)):.1f}×")
    return {"precision": precision, "recall": recall, "F1": f1, "overlap": overlap}
```

## Why This Matters

Synthetic lethality is the most clinically validated concept from network genetics. PARP inhibitors — which target the PARP1/PARP2 synthetic lethal interaction with BRCA1/BRCA2 — are now standard of care for BRCA-mutant breast, ovarian, and prostate cancers. The success of PARP inhibitors inspired a systematic search for other cancer-specific synthetic lethal interactions, fueled by genome-wide CRISPR screens in hundreds of cancer cell lines. The Cancer Dependency Map (DepMap) project has characterized essential genes across ~1,000 cancer cell lines, creating a resource for identifying genetic vulnerabilities specific to cancer subtypes defined by their driver mutations. Network analysis of genetic interaction data accelerates this discovery: instead of measuring all $10^9$ possible gene pairs, network topology predicts which pairs are most likely to be synthetic lethal — dramatically reducing the experimental search space and guiding toward the most therapeutically promising targets.
