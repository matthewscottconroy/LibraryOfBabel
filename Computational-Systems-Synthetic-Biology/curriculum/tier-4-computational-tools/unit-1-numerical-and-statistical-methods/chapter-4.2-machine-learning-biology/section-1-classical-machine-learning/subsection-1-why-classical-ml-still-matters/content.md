# Why Classical Machine Learning Still Matters in Biology

Here is a question worth sitting with: if deep learning is so powerful, why do the papers that predict patient drug response from proteomics data still use gradient boosted trees? The narrative around machine learning in 2025 is dominated by transformers, diffusion models, and protein language models. These are genuinely transformative technologies. But the biological data that most researchers actually work with — clinical metabolomics tables, quantitative proteomics, patient-level omics with phenotype labels, single-cell QC metrics — is often tabular, relatively small ($n < 10^4$ samples), and high-dimensional in interpretable ways. For these datasets, classical machine learning methods consistently match or outperform deep learning while offering dramatic advantages in compute cost, training time, interpretability, and robustness.

This is not a matter of preference or tradition. It reflects a deep truth about the relationship between model complexity and data structure: the right tool depends on the geometry of your problem, not on what happens to be fashionable.

## The Tabular Data Landscape in Biology

**Tabular biological datasets** include:
- Proteomics: 5,000–10,000 protein intensities × 50–500 samples
- Metabolomics: 500–5,000 metabolite measurements × 100–1,000 samples
- Clinical genomics: variant burden scores, polygenic risk scores × patient cohorts
- scRNA-seq metadata: cell-level QC metrics, clustering labels, pseudotime
- Drug sensitivity: IC50 measurements × 100–1,000 cell lines

On these datasets, empirical benchmarks (e.g., Grinsztajn et al. 2022, NeurIPS) consistently show that **gradient boosted trees (XGBoost, LightGBM) outperform deep learning** on tabular data, for reasons related to inductive bias, sample efficiency, and the absence of spatial or sequential structure.

## The Compute Argument

| Model | Training time (typical) | Hardware needed |
|-------|------------------------|-----------------|
| Random forest (500 trees) | 30 seconds | Laptop CPU |
| XGBoost | 2 minutes | Laptop CPU |
| Logistic regression | 10 seconds | Laptop CPU |
| Small neural network | 10 minutes | GPU helpful |
| BERT fine-tuning | 2 hours | GPU required |
| ESM-2 fine-tuning | 8 hours | Multi-GPU |

For exploratory analysis, rapid iteration, and hypothesis generation, classical ML runs in seconds. For a researcher testing 50 different feature sets or clinical hypotheses, deep learning is impractical and classical ML is essential.

## Interpretability: The Biological Advantage

The most important advantage of classical ML for biology is **interpretability**. When you fit a random forest to predict drug resistance from gene expression, feature importances rank genes by their contribution to the prediction — directly generating biological hypotheses. When you use SHAP values, you get sample-level explanations: "this patient's high EGFR expression and low PTEN expression contributed +2.3 to the resistance score."

Deep neural networks compress information into millions of distributed parameters, making post-hoc interpretation difficult, error-prone, and sometimes misleading. For biomarker discovery, clinical interpretation, and mechanistic hypothesis generation, this matters enormously.

```python
from sklearn.ensemble import RandomForestClassifier, GradientBoostingClassifier
from sklearn.linear_model import LogisticRegression
from sklearn.svm import SVC
from sklearn.model_selection import cross_val_score, StratifiedKFold
from sklearn.preprocessing import StandardScaler
from sklearn.pipeline import Pipeline
import numpy as np
import pandas as pd

# Simulated proteomics dataset: 200 samples, 500 proteins
# Binary outcome: drug-sensitive (0) vs drug-resistant (1)
rng = np.random.default_rng(42)
n_samples, n_features = 200, 500
X = rng.standard_normal((n_samples, n_features))
# Add signal: first 20 features are truly informative
y = (X[:, :20].mean(axis=1) + 0.5 * rng.standard_normal(n_samples) > 0).astype(int)

feature_names = [f"protein_{i:03d}" for i in range(n_features)]
cv = StratifiedKFold(n_splits=5, shuffle=True, random_state=42)

# Compare classical ML methods
models = {
    'Logistic (L2)': Pipeline([
        ('scaler', StandardScaler()),
        ('clf', LogisticRegression(C=1.0, max_iter=1000))
    ]),
    'Random Forest': RandomForestClassifier(
        n_estimators=200, max_features='sqrt', n_jobs=-1, random_state=42
    ),
    'SVM (RBF)': Pipeline([
        ('scaler', StandardScaler()),
        ('clf', SVC(kernel='rbf', C=1.0, probability=True))
    ]),
}

print("Model comparison (5-fold stratified CV AUC):")
print("-" * 50)
for name, model in models.items():
    scores = cross_val_score(model, X, y, cv=cv, scoring='roc_auc')
    print(f"  {name:25s}: {scores.mean():.3f} ± {scores.std():.3f}")
```

## Sample Efficiency: Classical ML Wins on Small Data

Deep learning requires large datasets to learn general representations. Classical methods, especially regularized linear models and tree ensembles, have strong inductive biases that work well with $n < 1,000$ samples. The principle is:

**Use the simplest model that fits your data well.** Start with:
1. Logistic/linear regression (interpretable baseline)
2. Random forest or gradient boosting (non-linear, robust)
3. SVM with RBF kernel (good for medium-dimensional spaces)
4. Neural network only if the above underfit

```python
from sklearn.model_selection import learning_curve
import matplotlib.pyplot as plt

# Learning curves: how does performance change with training set size?
train_sizes, train_scores, val_scores = learning_curve(
    RandomForestClassifier(n_estimators=100, random_state=42),
    X, y,
    train_sizes=np.linspace(0.1, 1.0, 10),
    cv=5, scoring='roc_auc', n_jobs=-1
)

fig, ax = plt.subplots(figsize=(7, 4))
ax.fill_between(train_sizes,
                train_scores.mean(1) - train_scores.std(1),
                train_scores.mean(1) + train_scores.std(1), alpha=0.2, color='C0')
ax.fill_between(train_sizes,
                val_scores.mean(1) - val_scores.std(1),
                val_scores.mean(1) + val_scores.std(1), alpha=0.2, color='C1')
ax.plot(train_sizes, train_scores.mean(1), 'C0-', label='Training AUC')
ax.plot(train_sizes, val_scores.mean(1), 'C1-', label='Validation AUC')
ax.set_xlabel('Training set size')
ax.set_ylabel('AUC-ROC')
ax.legend()
plt.tight_layout()
plt.savefig('learning_curve.pdf')
```

## Robustness and Reproducibility

Classical ML models are:
- **Deterministic** (for fixed seeds)
- **Fast to retrain** from scratch
- **Low variance** across implementations (sklearn, R caret, Julia MLJ all give similar results)
- **Well-understood** in terms of failure modes

Neural networks require extensive hyperparameter tuning (architecture, learning rate, regularization, data augmentation), are sensitive to random initialization, and can fail silently (training loss decreases, validation performance plateaus). For clinical or regulatory applications where reproducibility is mandatory, classical methods are strongly preferred.

## Why This Matters

Classical machine learning is not a stepping stone to deep learning — it is a permanent tool in the computational biologist's toolkit, optimized for the data regime that most biological experiments actually produce. Every researcher who understands random forests, regularized regression, and SVMs has a complete toolkit for the majority of supervised learning problems they will encounter in practice. Deep learning expertise, while increasingly important, builds on top of this foundation.
