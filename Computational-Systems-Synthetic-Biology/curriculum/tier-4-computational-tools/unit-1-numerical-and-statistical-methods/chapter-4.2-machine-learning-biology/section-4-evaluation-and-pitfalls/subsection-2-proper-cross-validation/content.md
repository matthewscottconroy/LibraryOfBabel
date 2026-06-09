# Proper Cross-Validation for Biological Data

Suppose you have built a model that predicts cancer drug sensitivity from gene expression, and your cross-validation shows AUC = 0.89. That is a number worth examining carefully before you celebrate. How did you split the data? If patients from the same clinical cohort appear in both training and test folds — or if cell lines from the same laboratory appear on both sides of the split — you may be measuring how well your model recognizes familiar examples, not how well it generalizes to new patients. The question "does this model work?" is only as good as the experimental design you use to answer it.

Cross-validation is the standard technique for estimating model generalization, but applying it naively to biological data produces unreliable estimates. The key insight is that biological datasets have structure — sequence families, experimental batches, phylogenetic relationships, temporal ordering — that must be respected when partitioning data into folds. Ignoring this structure is equivalent to the data leakage described in the previous section, but manifests specifically during the model selection and evaluation pipeline.

## Why Standard k-Fold Cross-Validation Fails in Biology

Standard k-fold CV randomly partitions samples into $k$ equally sized subsets, trains on $k-1$ folds, and evaluates on the held-out fold, rotating through all $k$ possibilities. The variance estimate is valid only when samples are i.i.d. — which biological samples almost never are.

**Problem**: If protein sequences from the same family appear in both train and test folds, the model sees near-identical examples during training before being "tested" on them. The CV estimate is optimistically biased, and the standard error of the CV estimate is too small (the folds are not independent).

## Stratified K-Fold Cross-Validation

**Stratified k-fold** ensures that each fold has the same class distribution as the full dataset. This is critical when one class is rare (e.g., active compounds in a screen, pathogenic variants in a dataset dominated by benign variants).

```python
import numpy as np
from sklearn.model_selection import StratifiedKFold, cross_val_score
from sklearn.ensemble import GradientBoostingClassifier

# X: (n_samples, n_features) — e.g., molecular fingerprints
# y: binary labels (1 = active, 0 = inactive); heavily imbalanced
print(f"Class balance: {y.mean():.3f} positive")

skf = StratifiedKFold(n_splits=5, shuffle=True, random_state=42)

# Evaluate with AUC-ROC (appropriate for imbalanced data; not accuracy)
model = GradientBoostingClassifier(n_estimators=200, max_depth=4, random_state=42)
auc_scores = cross_val_score(model, X, y, cv=skf, scoring="roc_auc", n_jobs=-1)

print(f"AUC: {auc_scores.mean():.3f} ± {auc_scores.std():.3f}")
# Report both mean and std; a large std indicates high variance across folds
```

Stratified k-fold is appropriate when:
- The classification task has imbalanced classes
- Samples are independent (e.g., small molecules from different chemical scaffolds, distinct patients)

## Group K-Fold: Biological Replicates and Batch Effects

**Group k-fold** ensures that all samples from the same group appear in the same fold. Use this when:
- You have biological replicates (multiple measurements from the same cell line, patient, or organism)
- Data was collected in batches (different experimental runs that must not be split)
- Proteins from the same family must stay together

```python
from sklearn.model_selection import GroupKFold, cross_val_score
import pandas as pd

# Example: gene expression dataset with multiple samples per patient
# Leaking patient-level information is common and inflates apparent accuracy
df = pd.read_csv("expression_data.csv")
X = df.drop(columns=["label", "patient_id"]).values
y = df["label"].values
groups = df["patient_id"].values  # group identifier

gkf = GroupKFold(n_splits=5)

# cross_val_score with groups argument
from sklearn.linear_model import LogisticRegression
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler

pipe = Pipeline([("scaler", StandardScaler()), ("clf", LogisticRegression(C=1.0))])
scores = cross_val_score(pipe, X, y, cv=gkf.split(X, y, groups),
                          scoring="roc_auc", n_jobs=-1)
print(f"Patient-stratified AUC: {scores.mean():.3f} ± {scores.std():.3f}")
```

## Leave-One-Group-Out (LOGO) Cross-Validation

**Leave-one-group-out (LOGO)** CV holds out all samples from one group as the test set in each fold. It is the most conservative (and most realistic) estimate when you want to know: "How well does the model generalize to an entirely new group (cell line, patient, species, protein family)?"

```python
from sklearn.model_selection import LeaveOneGroupOut

logo = LeaveOneGroupOut()
# Number of folds = number of unique groups (can be large!)
n_groups = len(np.unique(groups))
print(f"LOGO CV: {n_groups} folds")

logo_scores = []
for train_idx, test_idx in logo.split(X, y, groups):
    pipe.fit(X[train_idx], y[train_idx])
    pred = pipe.predict_proba(X[test_idx])[:, 1]
    from sklearn.metrics import roc_auc_score
    try:
        logo_scores.append(roc_auc_score(y[test_idx], pred))
    except ValueError:
        pass  # Skip folds with only one class in test set

print(f"LOGO AUC: {np.mean(logo_scores):.3f} ± {np.std(logo_scores):.3f}")
```

## Cluster-Based Cross-Validation for Sequence Data

The gold standard for protein or DNA sequence models: cluster sequences by identity with CD-HIT or MMseqs2, then use cluster membership as the group label for group k-fold.

```python
from sklearn.model_selection import GroupKFold
import subprocess
import pandas as pd

def cluster_based_cv(X, y, sequences, identity=0.3, n_splits=5):
    """
    Perform cluster-based CV: sequences clustered at `identity`,
    cluster membership used as group label.
    """
    # Write sequences to temp FASTA
    with open("/tmp/seqs.fasta", "w") as f:
        for i, seq in enumerate(sequences):
            f.write(f">seq{i}\n{seq}\n")

    # Run CD-HIT
    subprocess.run([
        "cd-hit", "-i", "/tmp/seqs.fasta", "-o", "/tmp/clusters.fasta",
        "-c", str(identity), "-n", "2", "-T", "4", "-M", "4000"
    ], check=True, capture_output=True)

    # Parse cluster assignments
    groups = np.zeros(len(sequences), dtype=int)
    current_cluster = -1
    with open("/tmp/clusters.fasta.clstr") as f:
        for line in f:
            if line.startswith(">Cluster"):
                current_cluster += 1
            elif ">" in line:
                idx = int(line.split(">seq")[1].split("...")[0])
                groups[idx] = current_cluster

    print(f"Sequences: {len(sequences)}, Clusters at {identity:.0%}: {current_cluster+1}")

    # Group k-fold on cluster labels
    gkf = GroupKFold(n_splits=n_splits)
    return list(gkf.split(X, y, groups)), groups
```

## Nested Cross-Validation for Hyperparameter Selection

When you want to both select hyperparameters and estimate generalization performance, you need **nested CV**: an outer loop for evaluation and an inner loop for hyperparameter search. Using the same CV loop for both produces an optimistically biased estimate.

```python
from sklearn.model_selection import GridSearchCV, GroupKFold, cross_val_score
from sklearn.svm import SVC

# Inner CV: hyperparameter selection (e.g., 3-fold)
inner_cv = GroupKFold(n_splits=3)

# Outer CV: unbiased generalization estimate (e.g., 5-fold)
outer_cv = GroupKFold(n_splits=5)

# Hyperparameter grid
param_grid = {"C": [0.1, 1.0, 10.0], "gamma": ["scale", "auto"]}

# GridSearchCV wraps inner CV for hyperparameter selection
clf = GridSearchCV(SVC(kernel="rbf", probability=True),
                   param_grid, cv=inner_cv, scoring="roc_auc", n_jobs=-1)

# Outer CV gives an honest generalization estimate
nested_scores = []
for outer_train, outer_test in outer_cv.split(X, y, groups):
    clf.fit(X[outer_train], y[outer_train], groups=groups[outer_train])
    pred = clf.predict_proba(X[outer_test])[:, 1]
    nested_scores.append(roc_auc_score(y[outer_test], pred))
    print(f"  Best params: {clf.best_params_}, fold AUC: {nested_scores[-1]:.3f}")

print(f"\nNested CV AUC: {np.mean(nested_scores):.3f} ± {np.std(nested_scores):.3f}")
```

The difference between standard CV (with hyperparameter tuning on the same folds) and nested CV is the **optimism bias** — how much the selected model has overfit to the CV procedure itself.

## Reporting Standards

A correct evaluation report should specify:
1. The splitting strategy (random / stratified / group / cluster-based / temporal)
2. The grouping criterion (patient ID, protein cluster at X% identity, batch ID)
3. The CV scheme (k-fold, LOGO, nested)
4. All metrics reported (mean ± std over folds, or 95% CI)
5. The metric choice and its appropriateness (AUC-ROC for imbalanced; AUPR when positive class prevalence is very low; Spearman $\rho$ or MSE for regression)

```python
from scipy.stats import spearmanr

def regression_cv_report(model, X, y, groups, n_splits=5):
    """Report Spearman r, Pearson r, and RMSE from group k-fold CV."""
    gkf = GroupKFold(n_splits=n_splits)
    spearman_scores, pearson_scores, rmse_scores = [], [], []

    for train_idx, test_idx in gkf.split(X, y, groups):
        model.fit(X[train_idx], y[train_idx])
        pred = model.predict(X[test_idx])
        spearman_scores.append(spearmanr(y[test_idx], pred).correlation)
        from scipy.stats import pearsonr
        pearson_scores.append(pearsonr(y[test_idx], pred)[0])
        rmse_scores.append(np.sqrt(np.mean((y[test_idx] - pred)**2)))

    print(f"Spearman r: {np.mean(spearman_scores):.3f} ± {np.std(spearman_scores):.3f}")
    print(f"Pearson r:  {np.mean(pearson_scores):.3f} ± {np.std(pearson_scores):.3f}")
    print(f"RMSE:       {np.mean(rmse_scores):.3f} ± {np.std(rmse_scores):.3f}")
```

## Why This Matters

The choice of cross-validation strategy can shift reported AUC by 0.05–0.30 in protein function prediction tasks. Models that appear publication-worthy under random CV may perform no better than a baseline when evaluated with proper cluster-based CV. As a practitioner, choosing the right CV scheme is as important as choosing the right model architecture. It is also the primary way reviewers will scrutinize your methodology: a model with modest performance under rigorous evaluation is more trustworthy than an apparently impressive model with questionable CV design.
