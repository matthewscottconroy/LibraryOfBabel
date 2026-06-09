# Support Vector Machines

Before deep learning took over biological sequence analysis, Support Vector Machines were the method of choice for predicting whether a DNA sequence contained a splice site, a transcription factor binding motif, or a promoter element. Researchers spent years hand-crafting features — k-mer frequencies, position weight matrix scores, dinucleotide biases — and then SVMs were the classifiers that drew the decision boundary. Understanding why SVMs were so effective, and what they do geometrically, will teach you something that no amount of deep learning practice can substitute: the intuition that a good classifier is one that keeps its distance.

**Support Vector Machines (SVMs)** are maximum-margin classifiers: they find the hyperplane that separates two classes with the widest possible margin, where the margin is the distance between the hyperplane and the nearest training points (the **support vectors**). The combination of the kernel trick — which implicitly maps data to very high-dimensional feature spaces — and robust performance on small, high-dimensional datasets makes SVMs particularly well-suited to biological classification problems like splice site prediction, binding site classification, and TF binding specificity.

## The Maximum Margin Hyperplane

For linearly separable binary classification data $\{(\mathbf{x}_i, y_i)\}$ with $y_i \in \{-1, +1\}$, the SVM finds $(\mathbf{w}, b)$ such that:

$$y_i(\mathbf{w} \cdot \mathbf{x}_i - b) \geq 1 \quad \forall i$$

and the margin $2/\|\mathbf{w}\|$ is maximized, equivalent to minimizing $\|\mathbf{w}\|^2/2$.

For non-separable data, **slack variables** $\xi_i \geq 0$ allow some misclassification:

$$\text{minimize} \quad \frac{1}{2}\|\mathbf{w}\|^2 + C \sum_i \xi_i$$
$$\text{subject to} \quad y_i(\mathbf{w} \cdot \mathbf{x}_i - b) \geq 1 - \xi_i$$

The **cost parameter** $C$ controls the tradeoff between margin width and training errors: large $C$ penalizes misclassification heavily (small margin, complex boundary); small $C$ allows more misclassifications (large margin, simpler boundary).

## The Kernel Trick

For nonlinearly separable data, SVMs use the **kernel trick**: rather than explicitly mapping inputs to a high-dimensional feature space $\phi(\mathbf{x})$, the kernel function $K(\mathbf{x}_i, \mathbf{x}_j) = \phi(\mathbf{x}_i) \cdot \phi(\mathbf{x}_j)$ computes the inner product in that space directly. The decision function becomes:

$$f(\mathbf{x}) = \text{sign}\left(\sum_{i \in \text{SVs}} \alpha_i y_i K(\mathbf{x}_i, \mathbf{x}) - b\right)$$

**Common kernels for biology:**

| Kernel | Formula | Biological use |
|--------|---------|----------------|
| RBF (Gaussian) | $\exp(-\gamma\|\mathbf{x}-\mathbf{z}\|^2)$ | Expression data, general |
| Polynomial | $(\mathbf{x} \cdot \mathbf{z} + r)^d$ | Combinatorial features |
| String kernel | count common substrings | Raw DNA/protein sequences |
| Linear | $\mathbf{x} \cdot \mathbf{z}$ | High-dimensional, sparse data |

The **RBF kernel** effectively projects data into an infinite-dimensional feature space and works well when the Euclidean distance between samples is meaningful — which is generally the case for standardized gene expression data.

## Application: Transcription Factor Binding Site Classification

SVMs were the dominant method for TF binding site prediction before deep learning:

```python
from sklearn.svm import SVC, SVR
from sklearn.model_selection import GridSearchCV, cross_val_score
from sklearn.preprocessing import StandardScaler
from sklearn.pipeline import Pipeline
import numpy as np

# Feature engineering: DNA sequence to numerical features
# Simple k-mer frequency approach
from itertools import product

def kmer_features(sequence, k=3):
    """Extract k-mer frequency features from a DNA sequence."""
    alphabet = 'ACGT'
    kmers = [''.join(p) for p in product(alphabet, repeat=k)]
    kmer_to_idx = {km: i for i, km in enumerate(kmers)}
    
    features = np.zeros(len(kmers))
    for i in range(len(sequence) - k + 1):
        kmer = sequence[i:i+k]
        if kmer in kmer_to_idx:
            features[kmer_to_idx[kmer]] += 1
    
    # Normalize to frequency
    total = features.sum()
    if total > 0:
        features /= total
    return features

# Simulate dataset: positive = TATA box sequences, negative = random
def generate_tata_data(n_pos=500, n_neg=500, seq_len=200, seed=42):
    rng = np.random.default_rng(seed)
    bases = list('ACGT')
    
    sequences = []
    labels = []
    
    # Positive: contain TATAAA motif
    for _ in range(n_pos):
        seq = rng.choice(bases, size=seq_len).tolist()
        pos = rng.integers(80, 120)
        seq[pos:pos+6] = list('TATAAA')
        sequences.append(''.join(seq))
        labels.append(1)
    
    # Negative: random sequences
    for _ in range(n_neg):
        seq = ''.join(rng.choice(bases, size=seq_len))
        sequences.append(seq)
        labels.append(0)
    
    return sequences, np.array(labels)

sequences, y = generate_tata_data()

# Extract 3-mer features
X = np.array([kmer_features(seq, k=3) for seq in sequences])
print(f"Feature matrix: {X.shape} (samples × 3-mers)")

# SVM with RBF kernel — the standard pipeline
svm_pipeline = Pipeline([
    ('scaler', StandardScaler()),
    ('svm', SVC(kernel='rbf', probability=True))
])

# Critical: tune C and gamma jointly with grid search
param_grid = {
    'svm__C': [0.1, 1, 10, 100],
    'svm__gamma': ['scale', 'auto', 0.01, 0.001]
}

grid_search = GridSearchCV(
    svm_pipeline, param_grid,
    cv=5, scoring='roc_auc',
    n_jobs=-1, verbose=1
)
grid_search.fit(X, y)

print(f"\nBest AUC: {grid_search.best_score_:.3f}")
print(f"Best params: {grid_search.best_params_}")

# Inspect support vectors
best_svm = grid_search.best_estimator_['svm']
print(f"Number of support vectors: {sum(best_svm.n_support_)}")
print(f"Fraction of training points that are SVs: "
      f"{sum(best_svm.n_support_)/len(y):.2%}")
```

## Support Vector Regression for Binding Affinity

SVMs extend to regression via **Support Vector Regression (SVR)**, which finds a function within an $\epsilon$-tube of the training data. For predicting binding affinity ($K_d$) from sequence features:

```python
from sklearn.svm import SVR
from sklearn.metrics import r2_score
import numpy as np

# Simulate binding affinity data: log(Kd) as continuous target
n = 400
X_reg = np.random.standard_normal((n, 64))  # 4-mer features
y_reg = X_reg[:, :5].sum(axis=1) + 0.5 * np.random.standard_normal(n)

svr_pipeline = Pipeline([
    ('scaler', StandardScaler()),
    ('svr', SVR(kernel='rbf', C=10, gamma='scale', epsilon=0.1))
])

from sklearn.model_selection import cross_val_predict
y_pred = cross_val_predict(svr_pipeline, X_reg, y_reg, cv=5)
r2 = r2_score(y_reg, y_pred)
print(f"SVR cross-validated R²: {r2:.3f}")
```

## The C and Gamma Parameters: A Practical Guide

The two most important hyperparameters for RBF SVMs:

- **C (regularization):** Controls the penalty for misclassification. Start with a log-uniform grid: `[0.01, 0.1, 1, 10, 100, 1000]`. Large C → complex boundary → overfitting risk; small C → large margin → underfitting risk.

- **gamma (bandwidth):** Controls the width of the RBF kernel, i.e., how far the influence of a single training example extends. `gamma='scale'` (default: $1/(p \cdot \text{Var}(X))$) is a good starting point. Small gamma → broad influence → smooth decision boundary; large gamma → narrow influence → complex, wiggly boundary.

The grid `C × gamma` must be searched jointly because the optimal values are correlated:

```python
import matplotlib.pyplot as plt
from sklearn.model_selection import GridSearchCV

# Visualize the C × gamma grid search results
cv_results = pd.DataFrame(grid_search.cv_results_)
scores = cv_results.pivot_table(
    index='param_svm__C',
    columns='param_svm__gamma',
    values='mean_test_score'
)

fig, ax = plt.subplots(figsize=(6, 5))
im = ax.imshow(scores, cmap='viridis', aspect='auto')
ax.set_xticks(range(scores.shape[1]))
ax.set_xticklabels(scores.columns, rotation=45)
ax.set_yticks(range(scores.shape[0]))
ax.set_yticklabels(scores.index)
ax.set_xlabel('gamma')
ax.set_ylabel('C')
ax.set_title('Cross-validated AUC')
plt.colorbar(im, ax=ax)
plt.tight_layout()
plt.savefig('svm_grid.pdf')
```

## Why This Matters

SVMs remain the workhorse for sequence-level classification tasks in bioinformatics — splice site detection, promoter recognition, intrinsically disordered region prediction — precisely because they excel on high-dimensional feature spaces derived from sequences (k-mer counts, PWM scores, structural features). The string kernel variant, which computes similarity directly from raw sequences without explicit feature extraction, was the state-of-the-art method for protein function prediction before the deep learning era and still provides competitive performance on small datasets where deep learning overfits.
