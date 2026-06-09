# Phosphoproteomics of Signaling Networks

## Mapping Signaling State Globally

For decades, cell signaling was studied one pathway at a time. You would stimulate cells with EGF, blot for phospho-ERK, perhaps phospho-AKT, call it a day. This approach built extraordinary knowledge — the canonical pathway maps we use today largely emerged from exactly this kind of careful, focused biochemistry. But it has a fundamental limitation: you can only see what you already know to look for. And signaling networks, it turns out, are far more interconnected, far more context-dependent, and far more surprising than any pathway map drawn in advance can capture.

Individual pathway studies identify how specific kinases activate specific substrates. But cells activate dozens of kinases simultaneously, and each kinase has hundreds of potential substrates. Understanding how a stimulus actually reshapes the **global phosphorylation state** of the cell requires a genome-scale measurement approach.

**Phosphoproteomics** — mass spectrometry-based quantification of protein phosphorylation at thousands of sites simultaneously — provides this global view. A modern phosphoproteomic experiment can measure 10,000–30,000 phosphorylation sites in a single experiment, revealing the coordinated changes in the cellular signaling network in response to any stimulus.

## Experimental Workflow

The standard phosphoproteomic workflow:

**1. Cell treatment and lysis**
Treat cells with stimulus (growth factor, drug, environmental stress). Quench signaling rapidly (typically 30–60 second cold PBS wash + 8M urea lysis buffer or boiling SDS). Time-course experiments capture signaling dynamics.

**2. Protein digestion**
Denature proteins (urea or SDS) → reduce and alkylate cysteines → trypsin digestion (cleaves after Arg/Lys) → produce peptides of 8-25 amino acids.

**3. Phosphopeptide enrichment**
Phosphopeptides represent ~0.1% of total peptides. Enrichment is required:
- **TiO₂ (titanium dioxide)**: most common; selective for pSer/pThr/pTyr
- **Fe-IMAC (immobilized metal affinity chromatography)**: also selective for phosphopeptides
- **Anti-pTyr antibody immunoprecipitation**: selective for tyrosine phosphorylation (relevant for RTK signaling)

**4. LC-MS/MS analysis**
Nano-HPLC separation → electrospray ionization → fragmentation (HCD, ETD) → mass measurement. Database search (MaxQuant, Proteome Discoverer) assigns phosphopeptide sequences and localizes phosphosite to specific residue.

**5. Quantification**
- **Label-free quantification (LFQ)**: compare intensities across samples
- **TMT/iTRAQ isobaric labeling**: chemically label peptides with different mass tags → multiplex 6–18 samples in single MS run
- **SILAC (Stable Isotope Labeling by Amino Acids in Cell Culture)**: metabolic labeling with ¹³C/¹⁵N amino acids → defined mass shift between conditions

## Key Computational Analyses

### 1. Kinase-Substrate Enrichment Analysis (KSEA)

Given a list of differentially phosphorylated sites, which kinases are responsible?

**KSEA** (Casado et al. 2013) uses curated kinase-substrate databases (PhosphoSitePlus, KinBase) to identify kinases whose known substrates are coordinately up- or down-regulated:

```python
import pandas as pd
import numpy as np
from scipy import stats

def ksea(phosphosite_fold_changes, kinase_substrate_db, min_substrates=5):
    """
    Kinase Substrate Enrichment Analysis.
    
    phosphosite_fold_changes: dict {site_id: log2_FC}
    kinase_substrate_db: dict {kinase: [list of substrate sites]}
    Returns: DataFrame with kinase, mean_substrate_FC, p_value
    """
    all_fc = np.array(list(phosphosite_fold_changes.values()))
    results = []
    
    for kinase, substrates in kinase_substrate_db.items():
        # Find fold changes for this kinase's substrates
        substrate_fc = [phosphosite_fold_changes[s] for s in substrates 
                       if s in phosphosite_fold_changes]
        
        if len(substrate_fc) < min_substrates:
            continue
        
        substrate_fc = np.array(substrate_fc)
        mean_fc = np.mean(substrate_fc)
        
        # Mann-Whitney U test: are substrate FCs different from background?
        statistic, pval = stats.mannwhitneyu(substrate_fc, all_fc, 
                                              alternative='two-sided')
        
        results.append({
            'kinase': kinase,
            'n_substrates': len(substrate_fc),
            'mean_substrate_FC': mean_fc,
            'p_value': pval,
            'enrichment_score': mean_fc / np.std(all_fc)
        })
    
    df = pd.DataFrame(results)
    df['FDR'] = df['p_value'] * len(df) / (df.index + 1)  # Benjamini-Hochberg
    return df.sort_values('enrichment_score', ascending=False)
```

### 2. Network-Based Analysis with OmniPath

**OmniPath** is a comprehensive resource of signaling pathway data (protein-protein interactions, kinase-substrate relationships, TF-target relationships) that enables network analysis of phosphoproteomic data:

```python
import omnipath as op
import networkx as nx

# Get kinase-substrate interactions
ksubs = op.interactions.KinaseExtra.get()
# Get all signaling interactions
signaling = op.interactions.AllInteractions.get(datasets=['signaling'])

# Build directed signaling network
G = nx.from_pandas_edgelist(signaling, 'source', 'target', 
                              edge_attr=['type', 'references'],
                              create_using=nx.DiGraph())

# Find paths from EGFR to a downstream transcription factor
paths = list(nx.all_simple_paths(G, 'EGFR', 'ELK1', cutoff=5))
print(f"Found {len(paths)} paths from EGFR to ELK1")
```

### 3. Phosphoproteomic Time-Course Analysis

Measuring phosphorylation changes at multiple time points after stimulus reveals the kinetics of pathway activation and feedback:

| Time point | Dominant changes |
|---|---|
| 1-5 min | RTK autophosphorylation, RAS-GTP, PI3K, immediate ERK |
| 5-30 min | Peak ERK, AKT, S6K activation |
| 30-120 min | Negative feedback (DUSP activation, ERK attenuation), delayed responses |
| 2-24 h | Transcription factor phosphorylation (CREB, ELK1), epigenetic changes |

## Identifying Signaling Signatures

Phosphoproteomic signatures can identify active signaling pathways without prior knowledge of the upstream stimulus:

**Application in cancer diagnostics**: phosphoproteomic profiling of tumors reveals which signaling pathways are hyperactivated, even when the driving mutation is not in an obvious kinase (e.g., mutation in a scaffold protein that constitutively activates downstream kinases). This guides kinase inhibitor selection beyond what genomic sequencing alone reveals.

**Reverse Phase Protein Arrays (RPPA)**: an alternative technology that measures ~500 proteins/phosphoproteins using antibody arrays — lower throughput than MS-based phosphoproteomics but higher throughput at the sample level (thousands of samples). Used in TCGA cancer proteomics.

## Integrating Phosphoproteomics with Other Data Types

Maximum insight comes from combining phosphoproteomics with:
- **Transcriptomics**: which gene expression changes are downstream of the phosphorylation events?
- **Genomics**: which mutations activate which kinases?
- **Drug response**: which phosphoproteomic features predict sensitivity to kinase inhibitors?

**CARNIVAL (CAusal Reasoning for Network Identification using Integer VALue programming)** integrates TF activities (from transcriptomics) with phosphoproteomic kinase activities to infer the signaling network topology active in a specific context — connecting the two data types through a causal reasoning framework.

## Why This Matters

Phosphoproteomics has transformed signaling biology from a pathway-centric view (studying one pathway at a time) to a systems-level view (measuring the full signaling state simultaneously). This global perspective reveals:
- Unexpected crosstalk between pathways
- Context-dependent rewiring of signaling networks
- Drug-induced feedback responses that drive resistance
- Biomarkers of pathway activity for patient stratification

As MS technology improves and phosphoproteomics becomes more routine, it will increasingly complement genomics as a readout of cellular signaling state — providing the mechanistic layer between mutation and phenotype.
