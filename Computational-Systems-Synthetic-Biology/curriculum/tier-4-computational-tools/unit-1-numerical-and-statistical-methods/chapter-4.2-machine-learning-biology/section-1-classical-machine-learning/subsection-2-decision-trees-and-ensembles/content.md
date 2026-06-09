# Decision Trees and Ensemble Methods

Imagine you are trying to classify tumor biopsies as drug-resistant or drug-sensitive, and you have expression measurements for a thousand genes. You could try to write down rules by hand — "if EGFR is high and PTEN is low, then resistant" — but human intuition runs out fast in a thousand-dimensional space. A decision tree does exactly this, but automatically and exhaustively, recursively partitioning the feature space to find the rules that best separate your classes. Alone, a single tree is fragile, prone to memorizing noise. But ensemble these trees in the right way — either by averaging many independently randomized trees, or by building each new tree to correct the errors of those before it — and you get the most reliable predictive tools available for biological tabular data.

Decision trees and their ensemble extensions — random forests and gradient boosted trees — are among the most broadly applicable machine learning methods in biology. They require minimal preprocessing, handle mixed data types naturally, are relatively robust to outliers, and provide interpretable feature importance measures that translate directly into biological hypotheses.

## Decision Trees: The Building Block

A **decision tree** recursively partitions the feature space by choosing a feature and threshold that maximally separates the outcome variable. For a binary classification problem, at each node the algorithm chooses the split that maximizes the **information gain** (or equivalently, minimizes the weighted **Gini impurity** of the resulting child nodes):

$$\text{Gini}(t) = 1 - \sum_{k=1}^{K} p_k^2$$

where $p_k$ is the fraction of samples in class $k$ at node $t$.

A fully grown decision tree **overfits** — it memorizes the training data. The maximum depth hyperparameter (`max_depth`) limits tree complexity and is the primary regularizer.

## Random Forests

A **random forest** is an ensemble of $B$ decision trees, each trained on a **bootstrap sample** (sampling with replacement) of the training data, using a **random subset of $m$ features** (typically $m = \sqrt{p}$ for classification, $m = p/3$ for regression) at each split. Predictions are aggregated by majority vote (classification) or averaging (regression).

Two sources of randomness — bootstrap sampling and feature subsampling — ensure that the trees are **decorrelated**: each tree makes different errors, and averaging reduces variance without increasing bias.

**Out-of-Bag (OOB) error:** For each training sample, approximately 1/3 of trees did not include it in their bootstrap sample. These trees provide a "held-out" prediction for free — the OOB error estimate is a reliable cross-validation proxy requiring no explicit train/test split.

**Feature Importance:** The mean decrease in impurity at splits using each feature, averaged across all trees, gives a **feature importance score**. Higher scores indicate features that consistently improve predictions across many trees.

```python
from sklearn.ensemble import RandomForestClassifier
from sklearn.model_selection import cross_val_score, GridSearchCV
import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

# Example: predict drug resistance from gene expression
# X: (n_samples, n_genes) expression matrix
# y: binary resistance label

rng = np.random.default_rng(42)
n_samples, n_genes = 300, 1000

# Simulate: 30 genes are truly informative
X = rng.standard_normal((n_samples, n_genes))
true_gene_idx = rng.integers(0, n_genes, 30)
y = (X[:, true_gene_idx].mean(axis=1) + rng.standard_normal(n_samples) > 0).astype(int)
gene_names = [f"GENE_{i:04d}" for i in range(n_genes)]

# Step 1: Quick baseline with OOB error
rf_base = RandomForestClassifier(
    n_estimators=500,
    max_features='sqrt',
    n_jobs=-1,
    oob_score=True,          # compute OOB error during training
    random_state=42
)
rf_base.fit(X, y)
print(f"OOB accuracy: {rf_base.oob_score_:.3f}")

# Step 2: Hyperparameter tuning with cross-validation
param_grid = {
    'n_estimators': [200, 500],
    'max_depth': [None, 10, 20],
    'min_samples_leaf': [1, 5, 10],
    'max_features': ['sqrt', 0.2]
}

rf_cv = RandomForestClassifier(n_jobs=-1, random_state=42)
grid_search = GridSearchCV(
    rf_cv, param_grid,
    cv=5, scoring='roc_auc',
    n_jobs=-1, verbose=0
)
grid_search.fit(X, y)
print(f"\nBest AUC: {grid_search.best_score_:.3f}")
print(f"Best params: {grid_search.best_params_}")

# Step 3: Feature importance analysis
best_rf = grid_search.best_estimator_
importances = pd.Series(best_rf.feature_importances_, index=gene_names)
top_genes = importances.sort_values(ascending=False)[:20]

fig, ax = plt.subplots(figsize=(8, 5))
top_genes.plot.bar(ax=ax)
ax.set_ylabel('Mean decrease in Gini impurity')
ax.set_title('Top 20 genes by feature importance')
plt.tight_layout()
plt.savefig('feature_importance.pdf')

# How many of the top 20 are truly informative?
top_20_idx = np.argsort(best_rf.feature_importances_)[-20:]
true_discovery = len(set(top_20_idx) & set(true_gene_idx))
print(f"\nTrue genes recovered in top 20: {true_discovery}/20")
```

## Gradient Boosting: XGBoost and LightGBM

Where random forests build trees in parallel and average them, **gradient boosting** builds an ensemble sequentially: each new tree corrects the **residual errors** of the current ensemble. The intuition is that each tree is a doctor of last resort — it sees only the cases the previous ensemble got wrong and focuses its diagnostic attention there.

Formally, we minimize a loss $\mathcal{L}$ by adding functions from a hypothesis class (regression trees) using gradient descent in function space:

$$F_m(x) = F_{m-1}(x) + \eta \cdot h_m(x)$$

where $h_m$ is the tree that best fits the **negative gradient** of $\mathcal{L}$ evaluated at $F_{m-1}$, and $\eta$ is the learning rate (shrinkage).

**XGBoost** adds regularization (L1/L2 on leaf weights) and uses second-order Taylor approximations for faster convergence. **LightGBM** uses histogram-based splits and leaf-wise (rather than depth-wise) tree growth, making it 10–100× faster than XGBoost for large datasets.

```python
from xgboost import XGBClassifier
from lightgbm import LGBMClassifier
from sklearn.model_selection import cross_val_score

# XGBoost: excellent baseline for tabular biological data
xgb = XGBClassifier(
    n_estimators=500,
    learning_rate=0.05,
    max_depth=6,
    subsample=0.8,
    colsample_bytree=0.8,
    eval_metric='logloss',
    use_label_encoder=False,
    random_state=42,
    n_jobs=-1
)

# LightGBM: faster, comparable accuracy
lgb = LGBMClassifier(
    n_estimators=500,
    learning_rate=0.05,
    num_leaves=31,
    subsample=0.8,
    colsample_bytree=0.8,
    random_state=42,
    n_jobs=-1,
    verbose=-1
)

for name, model in [('XGBoost', xgb), ('LightGBM', lgb)]:
    scores = cross_val_score(model, X, y, cv=5, scoring='roc_auc', n_jobs=-1)
    print(f"{name}: AUC = {scores.mean():.3f} ± {scores.std():.3f}")

# Early stopping with a validation set (prevents overfitting)
from sklearn.model_selection import train_test_split

X_tr, X_val, y_tr, y_val = train_test_split(X, y, test_size=0.2, random_state=42)
xgb_es = XGBClassifier(n_estimators=2000, learning_rate=0.05, 
                        max_depth=6, random_state=42)
xgb_es.fit(X_tr, y_tr,
           eval_set=[(X_val, y_val)],
           early_stopping_rounds=50,
           verbose=100)
print(f"Best iteration: {xgb_es.best_iteration}")
```

## Permutation Importance: Unbiased Feature Ranking

The built-in impurity-based importance **overestimates** high-cardinality and correlated features. **Permutation importance** — which measures the drop in validation accuracy when a feature is randomly shuffled — is more reliable because it directly asks the question you care about: "how much worse does the model perform when this feature is made uninformative?"

```python
from sklearn.inspection import permutation_importance

perm_imp = permutation_importance(
    best_rf, X, y,
    n_repeats=20,
    scoring='roc_auc',
    random_state=42,
    n_jobs=-1
)

perm_series = pd.Series(
    perm_imp.importances_mean,
    index=gene_names
).sort_values(ascending=False)

# Compare impurity vs permutation importance
print("Top 10 genes (permutation importance):")
print(perm_series[:10])
```

## Calibration: Ensuring Probability Outputs Are Reliable

For clinical applications where the predicted probability matters (not just the rank ordering), tree ensemble probabilities often need calibration:

```python
from sklearn.calibration import CalibratedClassifierCV, calibration_curve

# Calibrate random forest probabilities using Platt scaling
rf_calibrated = CalibratedClassifierCV(best_rf, cv=5, method='sigmoid')
rf_calibrated.fit(X, y)

# Check calibration
proba = rf_calibrated.predict_proba(X)[:, 1]
fraction_pos, mean_pred_prob = calibration_curve(y, proba, n_bins=10)
# Perfect calibration: fraction_pos ≈ mean_pred_prob
```

## Why This Matters

Random forests and gradient boosted trees are the most practically important ML methods for tabular biological data. Feature importance from these models has directly guided biological discovery: identifying driver mutations in cancer genomics, ranking metabolites associated with disease phenotypes, and discovering predictive biomarkers in clinical trials. Their combination of performance, interpretability, and robustness makes them the default tool for any supervised learning task on biological tabular data.
