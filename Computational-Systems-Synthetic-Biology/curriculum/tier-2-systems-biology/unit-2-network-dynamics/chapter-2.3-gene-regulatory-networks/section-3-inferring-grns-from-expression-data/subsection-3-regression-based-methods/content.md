# Regression-Based Methods for GRN Inference

## The Regression Reformulation

Consider what a target gene is really doing in the cell: its expression level is a function — typically nonlinear, possibly cooperative — of the activities of all the TFs that regulate it. This is the core of gene regulation. The question is whether you can reverse-engineer that function from expression data. Regression-based methods take this observation seriously and reframe GRN inference as a supervised machine learning problem: for each target gene $j$, predict its expression from the expression of all transcription factors (TFs), and use the model's feature importance scores to rank TF-target relationships.

This formulation is statistically natural because it directly models the biological process: the expression level of a target gene is determined by the activities of its regulators. Importantly, we typically restrict the set of possible regulators to known TFs (human: ~1,800 TFs), reducing the dimensionality of each regression problem from $n$ genes to $n_{\text{TF}}$ TFs.

## GENIE3: Random Forest Regression

**GENIE3** (Gene Network Inference with Ensemble of trees; Huynh-Thu et al. 2010) is one of the most consistently high-performing GRN inference methods across benchmark evaluations.

**Algorithm**:
For each target gene $j$:
1. Train a random forest regressor to predict $x_j$ from all TF expressions $\{x_i : i \in \text{TFs}\}$
2. Extract **feature importances**: the reduction in variance attributable to each TF $i$ when predicting $j$
3. Feature importance $w_{ij}$ becomes the edge weight in the inferred network

The final GRN is a ranked list of TF-target pairs sorted by importance weight.

**Why random forests?**
- No assumptions about functional form (linear, polynomial, etc.)
- Handles nonlinear and interaction effects naturally
- Resistant to overfitting through bootstrap aggregation
- Produces stable importance scores across runs
- Scalable to genome-wide datasets

The last point has a deeper implication. Because random forests make no assumptions about the functional form of regulation, GENIE3 is agnostic to whether a particular TF activates, represses, or produces a more complex nonlinear effect on its target. The model learns the relationship from data, not from prior assumptions about regulatory logic.

```python
from sklearn.ensemble import RandomForestRegressor
import numpy as np
import pandas as pd

def genie3(expression_matrix, tf_names, n_estimators=500, n_jobs=-1):
    """
    GENIE3 implementation.
    
    expression_matrix: genes x samples numpy array
    tf_names: list of TF indices or names
    Returns: DataFrame with [TF, target, importance] columns
    """
    gene_names = list(range(expression_matrix.shape[0]))
    tf_indices = [i for i, g in enumerate(gene_names) if g in tf_names]
    tf_expression = expression_matrix[tf_indices, :].T  # samples x TFs
    
    edges = []
    for target_idx in range(expression_matrix.shape[0]):
        target_expr = expression_matrix[target_idx, :]
        
        # Remove target from TF list if it's a TF itself
        other_tfs = [i for i in tf_indices if i != target_idx]
        X = expression_matrix[other_tfs, :].T
        
        # Fit random forest
        rf = RandomForestRegressor(n_estimators=n_estimators, 
                                   n_jobs=n_jobs, random_state=42)
        rf.fit(X, target_expr)
        
        # Collect feature importances
        for local_i, global_tf_i in enumerate(other_tfs):
            edges.append({
                'TF': gene_names[global_tf_i],
                'target': gene_names[target_idx],
                'importance': rf.feature_importances_[local_i]
            })
    
    return pd.DataFrame(edges).sort_values('importance', ascending=False)

# Usage
network = genie3(expr_matrix, tf_names=['TP53', 'MYC', 'E2F1', ...])
print(network.head(20))
```

## GRNBoost2: Gradient Boosting for Speed

**GRNBoost2** (Moerman et al. 2019), part of the `arboreto` package, replaces the random forest regressor with gradient boosted trees (XGBoost). This achieves approximately 10× speedup while maintaining comparable accuracy.

Key differences from GENIE3:
- XGBoost uses sequential tree building (gradient boosting) instead of parallel bagging
- Early stopping prevents overfitting without requiring large numbers of trees
- Importance scores are computed differently (gain-based vs. impurity-based)

GRNBoost2 is the preferred method for single-cell RNA-seq analysis where datasets may contain millions of cells.

```python
from arboreto.algo import grnboost2

# expression_data: cells x genes DataFrame
network_df = grnboost2(
    expression_data=expr_df,
    tf_names=tf_list,
    verbose=True,
    seed=42
)
# Returns ranked edge list: TF, target, importance
```

## LASSO Regression for Sparse Networks

**LASSO (L1-regularized) regression** enforces sparsity — each target gene is regulated by only a small number of TFs. For target gene $j$:

$$\min_{\beta} \frac{1}{2n} \|x_j - X_{\text{TF}} \beta\|_2^2 + \lambda \|\beta\|_1$$

The L1 penalty drives most coefficients to exactly zero, selecting only the most important TFs. The sign of each coefficient indicates the direction (activating vs. repressing) of the interaction — a key advantage over tree-based methods that provide importance magnitude only.

This sign information is something tree-based methods cannot give you. If you need to know not just which TFs regulate a gene but whether they activate or repress it, LASSO is a natural starting point — even if its performance on highly nonlinear relationships is lower than GENIE3.

**TIGRESS** (Haury et al. 2012) uses stability selection with LASSO: repeatedly apply LASSO to bootstrap subsamples and select edges that appear consistently. This reduces false positives substantially.

```python
from sklearn.linear_model import LassoCV
import numpy as np

def lasso_grn(expr_matrix, tf_indices, alpha=None):
    """Infer GRN using LASSO for each target gene."""
    n_genes = expr_matrix.shape[0]
    edges = []
    
    for target in range(n_genes):
        X = expr_matrix[tf_indices, :].T  # samples x TFs
        y = expr_matrix[target, :]
        
        if alpha is None:
            lasso = LassoCV(cv=5, max_iter=5000)
        else:
            from sklearn.linear_model import Lasso
            lasso = Lasso(alpha=alpha, max_iter=5000)
        
        lasso.fit(X, y)
        
        for i, coef in enumerate(lasso.coef_):
            if abs(coef) > 1e-6:
                edges.append({
                    'TF': tf_indices[i],
                    'target': target,
                    'coefficient': coef,
                    'sign': '+' if coef > 0 else '-'
                })
    
    return pd.DataFrame(edges)
```

## Comparative Performance

From DREAM benchmark evaluations:

| Method | Bulk RNA-seq | scRNA-seq | Runtime |
|---|---|---|---|
| GENIE3 | High | High | Slow |
| GRNBoost2 | High | High | Fast |
| LASSO | Medium | Low | Fast |
| ARACNE | Medium | Medium | Medium |
| CLR | Medium | Low | Fast |

Ensemble combinations (e.g., averaging GENIE3 + CLR + LASSO rankings) consistently outperform individual methods.

## Why This Matters

Regression-based methods, especially GENIE3 and GRNBoost2, represent the current state-of-the-art for GRN inference from expression data. Their success comes from the flexibility of tree-based models to capture complex, nonlinear regulatory relationships without requiring prior specification of the functional form. Understanding these methods is prerequisite to interpreting SCENIC output (which builds on GRNBoost2) and to designing computational experiments that combine expression-based inference with chromatin and binding data for maximum regulatory insight.
