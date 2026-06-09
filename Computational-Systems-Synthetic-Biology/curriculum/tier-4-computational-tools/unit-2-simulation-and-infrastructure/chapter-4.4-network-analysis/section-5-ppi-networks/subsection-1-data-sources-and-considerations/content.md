# PPI Networks: Data Sources and Considerations

Every interaction in a protein-protein interaction database began its life as an experiment. Someone transfected a yeast two-hybrid construct, ran an affinity purification, or added fluorescent tags to two proteins and watched them co-localize. Each of these experiments asks a slightly different question, detects a slightly different kind of interaction, and introduces a slightly different kind of noise. When you download a PPI network from BioGRID and start computing centrality scores or drug targets, you are working with the accumulated output of thousands of such experiments — with all their individual biases overlaid on top of each other. Understanding those biases is not a footnote to the analysis. It is the analysis.

Protein-protein interaction (PPI) networks are the backbone of interactome biology, but interpreting them correctly requires understanding how the data were generated and what biases each experimental method introduces. No single PPI dataset is complete or unbiased; network analyses built on uncritical use of a single database will propagate those biases into all downstream conclusions.

## Major PPI Databases

### BioGRID

The **Biological General Repository for Interaction Datasets (BioGRID)** is the largest manually curated interaction database, drawing from yeast-two-hybrid (Y2H), affinity purification mass spectrometry (AP-MS), protein complementation assays, and other experimental methods.

```python
import requests
import pandas as pd
import networkx as nx

def download_biogrid_ppi(organism="Saccharomyces cerevisiae",
                          interaction_type="physical",
                          version="4.4.236"):
    """
    Download PPI data from BioGRID REST API.
    Returns a NetworkX graph of physical interactions.
    """
    # BioGRID API
    url = "https://webservice.thebiogrid.org/interactions/"
    params = {
        "accessKey": "YOUR_API_KEY",  # register at biogrid.org
        "format": "json",
        "interSpeciesExcluded": "true",
        "selfInteractionsExcluded": "true",
        "geneList": "",          # empty = all genes for organism
        "taxId": "559292",       # S. cerevisiae
        "includeInteractors": "false"
    }
    # In practice: download the flat file from downloads.thebiogrid.org

    # Simulate with synthetic data for illustration
    print("BioGRID data structure:")
    print("  #BioGRID-ID  Entrez Gene Interactor A  Entrez Gene Interactor B")
    print("  Official Symbol A  Official Symbol B  Organism A  Organism B")
    print("  Experimental System  Experimental System Type  Author  PMID  Source Database")

def load_biogrid_flatfile(tsv_file, min_score=None, exp_types=None):
    """
    Load BioGRID flat file and return a filtered NetworkX graph.
    exp_types: list of experimental types to include
               e.g., ["Two-hybrid", "Affinity Capture-MS", "Co-crystal Structure"]
    """
    df = pd.read_csv(tsv_file, sep="\t", comment="#")
    # Standard BioGRID columns:
    # Official Symbol Interactor A, Official Symbol Interactor B,
    # Experimental System, Experimental System Type, Organism ID Interactor A

    if exp_types:
        df = df[df["Experimental System"].isin(exp_types)]

    G = nx.Graph()
    for _, row in df.iterrows():
        gene_a = row["Official Symbol Interactor A"]
        gene_b = row["Official Symbol Interactor B"]
        exp    = row["Experimental System"]
        if gene_a != gene_b:  # exclude self-interactions
            if G.has_edge(gene_a, gene_b):
                # Multiple evidence: increment weight
                G[gene_a][gene_b]["evidence_count"] += 1
                G[gene_a][gene_b]["experiments"].add(exp)
            else:
                G.add_edge(gene_a, gene_b, evidence_count=1,
                           experiments={exp})
    return G
```

### STRING

**STRING** (Search Tool for the Retrieval of Interacting Genes/Proteins) aggregates evidence from multiple sources — experiments, text mining, co-expression, genomic context (gene fusion, neighborhood, co-occurrence) — and assigns a composite score per interaction.

```python
def load_string_network(string_file, min_score=700, species=9606):
    """
    Load STRING protein links file.
    string_file: e.g., 9606.protein.links.v12.0.txt.gz
    min_score: 0-1000; 400=low, 700=medium, 900=high confidence
    """
    df = pd.read_csv(string_file, sep=" ")
    # Columns: protein1, protein2, combined_score
    # Protein IDs are Ensembl protein IDs: e.g., 9606.ENSP00000356969

    filtered = df[df["combined_score"] >= min_score].copy()
    # Strip species prefix
    filtered["protein1"] = filtered["protein1"].str.replace(f"{species}.", "", regex=False)
    filtered["protein2"] = filtered["protein2"].str.replace(f"{species}.", "", regex=False)

    G = nx.Graph()
    for _, row in filtered.iterrows():
        G.add_edge(row["protein1"], row["protein2"],
                   weight=row["combined_score"] / 1000.0)

    print(f"STRING network (score ≥ {min_score}):")
    print(f"  Proteins: {G.number_of_nodes()}")
    print(f"  Interactions: {G.number_of_edges()}")
    return G

def string_score_composition(detail_file, gene_pair=("TP53", "MDM2")):
    """
    Break down STRING composite score into evidence channels.
    detail_file: 9606.protein.links.detailed.v12.0.txt.gz
    """
    df = pd.read_csv(detail_file, sep=" ")
    channels = ["neighborhood", "neighborhood_transferred", "fusion",
                "cooccurence", "coexpression", "coexpression_transferred",
                "experimentally_determined_interaction",
                "experimentally_determined_interaction_transferred",
                "database_annotated", "database_annotated_transferred",
                "textmining", "textmining_transferred", "combined_score"]
    print(f"\nSTRING score breakdown for {gene_pair[0]}–{gene_pair[1]}:")
    print("  (scores 0-1000; 0 = no evidence for this channel)")
```

### IntAct and Human Protein Reference Database

**IntAct** (EMBL-EBI): high-confidence, manually curated; all interactions are literature-derived. Smaller than BioGRID but higher precision.

**HuRI** (Human Reference Interactome): systematic Y2H screening of ~17,000 human proteins. Represents the most comprehensive binary interaction map; ~53,000 interactions.

## Experimental Methods and Their Biases

Understanding which interactions are detected by which method is essential for interpreting the resulting network.

| Method | What it detects | Biases | False positive rate |
|---|---|---|---|
| Y2H | Binary direct interactions | Nuclear-localized proteins favored; misses membrane proteins; artificial fusions | ~50–80% |
| AP-MS | Co-complex interactions | Detects both direct and indirect (within-complex) interactions; transient interactions missed | ~40–60% |
| Co-fractionation | Protein complexes | Low resolution; many false positives for abundant proteins | ~60–80% |
| FRET/BiFC | Proximity in cells | True binary interaction; requires compatible fluorophore geometries | ~20–30% |
| Co-IP + WB | Physical association | Usually confirms specific known interactions; not high-throughput | Low |

**Critical points:**
- A Y2H interaction means two proteins can bind when fused to artificial domains in a yeast nucleus — not necessarily that they interact in their native cellular context
- AP-MS gives co-complex membership, not binary interaction; protein A and protein B may both be in a 20-protein complex without ever directly touching
- High-confidence STRING interactions (score > 900) still have ~30% false positives

## Network Bias: The Popularity Problem

The most pervasive bias in PPI databases is the **sampling bias** toward well-studied proteins. Proteins like TP53, EGFR, and AKT1 are extensively studied; every new study reports more interactions with these proteins. A low-degree protein in the interactome database may simply be understudied.

```python
def assess_degree_study_bias(G, publication_counts):
    """
    Test whether network degree correlates with number of publications per gene.
    If yes: high-degree nodes may simply be better-studied, not genuinely more connected.
    publication_counts: dict {gene: n_publications}
    """
    import scipy.stats as stats

    degrees = {node: G.degree(node) for node in G.nodes() if node in publication_counts}
    deg_vals = [degrees[n] for n in degrees]
    pub_vals = [publication_counts[n] for n in degrees]

    rho, pval = stats.spearmanr(deg_vals, pub_vals)
    print(f"Spearman ρ(degree, publications): {rho:.3f}, p = {pval:.2e}")
    if rho > 0.5:
        print("  WARNING: Strong publication bias detected in this PPI network")
        print("  Hub proteins may be hubs due to study bias, not biology")
    else:
        print("  Moderate publication bias; proceed with appropriate caution")
    return rho, pval

# Mitigation: use Y2H-based networks (HuRI) that systematically screen all protein pairs
# or apply degree-aware normalization in downstream analyses
```

## Building a High-Confidence Network

```python
def build_high_confidence_ppi(biogrid_df, string_df, min_biogrid_evidence=2,
                               min_string_score=800):
    """
    Build a high-confidence PPI network by requiring interactions to be:
    1. Supported by >= min_biogrid_evidence independent experiments in BioGRID, OR
    2. High confidence in STRING (score >= min_string_score), AND
    3. Present in both databases (optional: intersection)
    """
    G_biogrid = nx.Graph()
    for _, row in biogrid_df.iterrows():
        a, b = row["Official Symbol Interactor A"], row["Official Symbol Interactor B"]
        if a != b:
            if G_biogrid.has_edge(a, b):
                G_biogrid[a][b]["count"] += 1
            else:
                G_biogrid.add_edge(a, b, count=1)

    # Filter BioGRID: keep only edges with >= n evidence
    high_conf_biogrid = {(a, b) for a, b, data in G_biogrid.edges(data=True)
                          if data["count"] >= min_biogrid_evidence}

    # High confidence STRING edges
    high_conf_string = {
        (row["protein1"], row["protein2"])
        for _, row in string_df.iterrows()
        if row["combined_score"] >= min_string_score
    }
    # Normalize protein IDs in both sets before comparison

    # Union or intersection
    union_interactions = high_conf_biogrid | high_conf_string
    intersection = high_conf_biogrid & high_conf_string

    G_final = nx.Graph()
    G_final.add_edges_from(union_interactions)

    print(f"High-confidence PPI network:")
    print(f"  BioGRID (≥{min_biogrid_evidence} evidence): {len(high_conf_biogrid)} edges")
    print(f"  STRING (score ≥{min_string_score}): {len(high_conf_string)} edges")
    print(f"  Union: {G_final.number_of_edges()} edges")
    print(f"  Intersection: {len(intersection)} edges")
    return G_final
```

## Why This Matters

The quality of a PPI network analysis is bounded by the quality of the underlying data. Using BioGRID without filtering for evidence count produces a noisy network where up to 80% of edges may be false positives — all downstream centrality, community detection, and drug target analyses are then operating on garbage. Conversely, using only the highest-confidence STRING interactions (score > 900) sacrifices recall — many true interactions are missed. The correct approach is to understand the biology of your system, choose appropriate databases and evidence filters, and acknowledge the limitations in your analysis. Network biology papers that ignore experimental biases are not reproducible and should not be trusted.
