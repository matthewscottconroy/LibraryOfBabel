# Data Leakage in Biological Machine Learning

In 2019, a highly-cited paper reported a deep learning model for protein function prediction that achieved AUC scores in the high 0.90s across a wide range of Gene Ontology terms. The numbers were impressive enough to generate substantial follow-on work. Then someone noticed that the train and test sets contained proteins from the same families — in some cases, proteins with over 50% sequence identity that were almost certainly performing identical functions. Re-evaluated with proper sequence-clustered splits, the AUC scores dropped by 10 to 20 percentage points. The model had learned, essentially, to recognize family members from the training set. This pattern has been documented across computational biology repeatedly enough that it deserves a name: it is data leakage, and it is the single most dangerous pitfall in applied machine learning for biology.

Data leakage is the single most dangerous pitfall in applied machine learning for biology. It occurs when information from the test set contaminates the training process, producing models that appear to perform well but fail completely when deployed on truly unseen data. In biological ML, leakage is particularly insidious because biological sequences, structures, and temporal data have non-obvious dependencies that violate the i.i.d. (independent and identically distributed) assumption underlying standard train/test splits.

## What Is Data Leakage?

Formally, leakage occurs when the training data contains information about the target variable that would not be available at prediction time, or when test samples are not independent of training samples. The result is an optimistic bias: reported performance metrics are inflated relative to true generalization performance.

$$\text{Apparent performance} = \text{True generalization} + \underbrace{\text{Leakage bias}}_{\geq 0}$$

Published benchmarks in computational biology frequently suffer from leakage. A 2021 survey found that the majority of protein function prediction papers used random train/test splits, producing AUC estimates inflated by 10-30 percentage points relative to properly clustered splits.

## Sequence Homology Leakage

**Sequence homology leakage** is the dominant failure mode in protein and nucleic acid ML. If a test protein shares high sequence identity with a training protein, the model can achieve high accuracy by effectively memorizing the training set — the model has not learned generalizable sequence-function relationships.

**Why it happens**: A random 80/20 split of a protein dataset will, by chance, place homologous proteins (often from the same family, with >50% sequence identity) in both folds. Standard k-fold cross-validation over protein sequences is almost always leaky.

**Correct approach — sequence clustering before splitting**: Use **CD-HIT** or **MMseqs2** to cluster proteins at a defined identity threshold (typically 30-50%), then assign entire clusters to either training or test sets.

```bash
# CD-HIT: cluster at 30% identity (very stringent; catches distant homologs)
cd-hit -i all_proteins.fasta \
       -o clusters_30.fasta \
       -c 0.30 \
       -n 2 \
       -M 16000 \
       -T 8

# Output: clusters_30.fasta.clstr contains cluster assignments
# Use cluster membership, not individual sequences, for train/test assignment
```

```python
import subprocess
import re
from collections import defaultdict

def parse_cdhit_clstr(clstr_file):
    """Parse CD-HIT .clstr output into {protein_id: cluster_id} dict."""
    clusters = {}
    current_cluster = None
    with open(clstr_file) as f:
        for line in f:
            if line.startswith(">Cluster"):
                current_cluster = int(line.split()[1])
            else:
                # Extract sequence ID from >ID... format
                match = re.search(r">(\S+)\.\.\..*", line)
                if match:
                    clusters[match.group(1)] = current_cluster
    return clusters

def cluster_split(protein_ids, cluster_map, test_fraction=0.2, seed=42):
    """Assign clusters to train/test; return per-protein split assignments."""
    import numpy as np
    rng = np.random.default_rng(seed)

    # Group proteins by cluster
    cluster_to_prots = defaultdict(list)
    for pid in protein_ids:
        cluster_to_prots[cluster_map[pid]].append(pid)

    cluster_ids = list(cluster_to_prots.keys())
    rng.shuffle(cluster_ids)

    n_test_clusters = max(1, int(len(cluster_ids) * test_fraction))
    test_clusters = set(cluster_ids[:n_test_clusters])

    train, test = [], []
    for pid in protein_ids:
        if cluster_map[pid] in test_clusters:
            test.append(pid)
        else:
            train.append(pid)
    return train, test

# MMseqs2 alternative — faster for very large datasets
def cluster_mmseqs2(fasta_in, out_prefix, identity=0.3, threads=8):
    """Run MMseqs2 easy-cluster and return cluster membership."""
    subprocess.run([
        "mmseqs", "easy-cluster",
        fasta_in, out_prefix, "/tmp/mmseqs_tmp",
        "--min-seq-id", str(identity),
        "--threads", str(threads)
    ], check=True)
    # Output: {out_prefix}_cluster.tsv — two columns: rep_seq, member_seq
    import pandas as pd
    clust = pd.read_csv(f"{out_prefix}_cluster.tsv", sep="\t",
                        names=["representative", "member"])
    # Map each member to its representative (cluster label)
    return dict(zip(clust["member"], clust["representative"]))
```

## Temporal Leakage

**Temporal leakage** occurs when a model trained on data from one time period is tested on data from the same period, even though the model will be deployed to predict future outcomes. This is critical for:

- **Variant effect prediction**: if variants discovered in 2023 appear in both training and test sets, but the model will be deployed to interpret novel variants from 2024
- **Drug repurposing**: training on known drug-disease associations and testing on held-out associations from the same publication database
- **Protein structure prediction benchmarks**: CASP targets (released post-2020) must not overlap with PDB structures used in training

**Correct approach**: Always split by time. If your data has timestamps (submission date, publication year, assay date), use a chronological cutoff.

```python
import pandas as pd

def temporal_split(df, date_col="date", test_cutoff="2023-01-01"):
    """Split dataframe using a strict temporal cutoff."""
    df[date_col] = pd.to_datetime(df[date_col])
    cutoff = pd.to_datetime(test_cutoff)
    train = df[df[date_col] < cutoff].copy()
    test  = df[df[date_col] >= cutoff].copy()
    print(f"Train: {len(train)} samples (before {test_cutoff})")
    print(f"Test:  {len(test)} samples (after {test_cutoff})")
    return train, test

# Worked example: UniProt submission dates
proteins = pd.read_csv("uniprot_proteins.csv")  # columns: id, sequence, function, reviewed_date
train_prots, test_prots = temporal_split(proteins, date_col="reviewed_date",
                                          test_cutoff="2022-01-01")
```

## Feature Leakage

**Feature leakage** occurs when features computed from the full dataset — including test samples — are used in training. Common examples:

- Normalizing expression values using the mean/variance of the entire dataset (including test)
- Computing PCA projections on the full dataset, then splitting
- Using global sequence statistics (e.g., amino acid frequencies) computed over all data

**Correct approach**: fit all preprocessing transformers on training data only, then apply (transform) to test data.

```python
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler
from sklearn.ensemble import RandomForestClassifier

# WRONG: fit scaler on all data
from sklearn.preprocessing import StandardScaler
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X)  # leaks test statistics into scaling
X_train_scaled, X_test_scaled = X_scaled[train_idx], X_scaled[test_idx]

# CORRECT: fit only on training data, transform both
scaler = StandardScaler()
X_train_scaled = scaler.fit_transform(X[train_idx])   # fit + transform train
X_test_scaled  = scaler.transform(X[test_idx])        # transform only test

# Best practice: use sklearn Pipeline to prevent this mistake automatically
pipe = Pipeline([
    ("scaler", StandardScaler()),
    ("clf", RandomForestClassifier(n_estimators=500, random_state=42))
])
# cross_val_score with Pipeline: scaler is re-fit inside each CV fold
from sklearn.model_selection import cross_val_score
scores = cross_val_score(pipe, X, y, cv=5)
```

## Diagnosing Leakage: A Checklist

```python
def leakage_audit(train_seqs, test_seqs, identity_threshold=0.5):
    """
    Quick audit: compute pairwise sequence identity between test and train.
    Flag any test sequence with >threshold identity to a training sequence.
    """
    from Bio import pairwise2
    flags = []
    for test_seq in test_seqs[:100]:  # sample for speed
        for train_seq in train_seqs:
            alignment = pairwise2.align.globalxx(test_seq, train_seq,
                                                  score_only=True)
            max_len = max(len(test_seq), len(train_seq))
            identity = alignment / max_len
            if identity > identity_threshold:
                flags.append((test_seq[:20], identity))
                break
    if flags:
        print(f"WARNING: {len(flags)}/100 test sequences have >{identity_threshold:.0%}"
              f" identity to training sequences")
    else:
        print("No high-identity pairs detected in sample.")
    return flags
```

## Why This Matters

The gap between reported benchmark performance and real-world deployment performance in biological ML is often explained entirely by data leakage. A protein function predictor that reports 95% accuracy using random splits may drop to 70% when tested on sequences from novel protein families — a difference that matters enormously when the model is used to guide experimental decisions. Understanding and controlling for leakage is not a methodological nicety but a scientific necessity. Results from leaky evaluations are not reproducible, not useful, and waste experimental resources when acted upon.
