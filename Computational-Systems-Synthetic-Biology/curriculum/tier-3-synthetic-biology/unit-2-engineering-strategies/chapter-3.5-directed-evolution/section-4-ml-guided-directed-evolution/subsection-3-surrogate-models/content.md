# Surrogate Models for Fitness Prediction

Once you have a set of measured variants — sequences with attached fitness values — you face a modeling problem. Your training set might be a few hundred data points. The space you are trying to predict over might be tens of millions of possible sequences. The model you build to bridge these two realities is called the surrogate model, and it is the intellectual heart of machine learning-guided directed evolution. A good surrogate correctly predicts which unmeasured sequences have high fitness; a bad one sends you down blind alleys, spending expensive experimental budget on sequences that underperform. Understanding what makes a surrogate model succeed or fail in this particular context — small data, structured input, high-dimensional output space, strong epistasis — is what separates principled MLDE from wishful machine learning.

A surrogate model (also called a fitness model or oracle approximation) learns the mapping from protein sequence (encoded as a numerical representation) to fitness value from a set of measured training examples. The quality of the surrogate model determines the efficiency of machine learning-guided directed evolution: a better model proposes better sequences, requiring fewer experimental measurements to find the optimum.

## The Training Problem

Given $N$ training examples $\{(x_i, y_i)\}_{i=1}^N$ where $x_i$ is the sequence embedding and $y_i$ is the measured fitness (kcat, Tm, binding Kd, or any scalar property), the surrogate model learns a function $f: x \rightarrow y$ that minimizes prediction error on unseen sequences.

Specific challenges in the directed evolution context:
- **Small $N$**: typically 50–2000 measurements; deep neural networks overfit
- **Structured input**: sequences are not i.i.d. samples — they are related by evolutionary distance and mutational proximity
- **Epistatic interactions**: the correct model must capture how combinations of substitutions jointly affect fitness
- **Out-of-distribution generalization**: the model must predict fitness for sequences more distant from training data than any pair of training sequences

## Gaussian Process Regression

The **Gaussian process (GP)** is the most principled surrogate model for MLDE because it provides not only fitness predictions but also **uncertainty estimates** — essential for Bayesian optimization acquisition functions.

### Model

A GP assumes the fitness function $f(x)$ is a sample from a Gaussian process prior:
$$f(x) \sim \mathcal{GP}(m(x), k(x, x'))$$

Where $m(x)$ is the mean function (typically set to 0) and $k(x, x')$ is the **kernel function** defining the covariance between fitness values at sequences $x$ and $x'$.

**Posterior predictions**: given training data $\{(x_i, y_i)\}$, the GP posterior at a new sequence $x^*$ is a Gaussian distribution:
$$f(x^*) | \{(x_i, y_i)\} \sim \mathcal{N}(\mu^*, (\sigma^*)^2)$$

Where:
$$\mu^* = K(x^*, X)[K(X, X) + \sigma_n^2 I]^{-1} y$$
$$(\sigma^*)^2 = k(x^*, x^*) - K(x^*, X)[K(X, X) + \sigma_n^2 I]^{-1} K(X, x^*)$$

$K(X, X)$ is the $N \times N$ kernel matrix between training sequences, $\sigma_n^2$ is noise variance.

**Kernel choice for sequences**: the **string kernel** (measuring sequence similarity), radial basis function (RBF) kernel on ESM embeddings, or Matern kernel. The kernel defines the smoothness assumption of the fitness landscape.

**Advantage**: uncertainty estimates $\sigma^*$ are used directly in acquisition functions (UCB, EI). High uncertainty = model is unsure = explore this region.

**Disadvantage**: GP computation scales as $O(N^3)$ (matrix inversion), making it slow for $N > 5000$. Sparse GP approximations (inducing point methods) reduce this to $O(NM^2)$ for $M$ inducing points.

## Random Forest

A **random forest** is an ensemble of decision trees trained on random subsets of the training data and features. Predictions are averaged across trees.

**Advantages for MLDE**:
- Robust to small training sets (typically performs better than neural networks for $N < 500$)
- Feature importance estimates: identifies which positions or embedding dimensions most predict fitness
- Non-parametric: no assumption about the functional form of the fitness landscape

**Uncertainty estimation**: predict with all trees; the variance across tree predictions approximates uncertainty. Less principled than GP uncertainty but computationally tractable for large sequences.

**Performance**: on GB1 benchmark with 200 training sequences, random forest + one-hot encoding achieves Spearman $\rho \approx 0.55$; random forest + ESM embedding achieves $\rho \approx 0.68$.

## Linear Models: Ridge Regression and Lasso

For interpretability and when epistasis is limited:

**Ridge regression** (L2 regularization):
$$\hat{\beta} = \arg\min_\beta \|y - X\beta\|^2 + \lambda\|\beta\|^2$$

With one-hot encoding, the coefficients $\beta$ directly report the additive effect of each amino acid at each position. This is essentially the **additive model of fitness** — the assumption that individual substitution effects are independent and sum linearly.

**When does the additive model work?**: empirically, ~50–70% of the fitness variation in real landscapes is explained by additive effects (measured in global mutagenesis studies). The remainder requires epistatic terms.

**Lasso** (L1 regularization): selects sparse sets of important positions; useful for identifying which positions are most critical.

**Epistatic extensions**: add pairwise interaction terms (polynomial features) to the one-hot matrix:
$$X_{pairwise} = [x_i \otimes x_j \text{ for all pairs } (i, j)]$$

Each column represents the joint presence of a specific amino acid at position $i$ AND position $j$. This captures pairwise epistasis at the cost of $O(L^2 \times 20^2)$ features — feasible only for short proteins or with heavy L1 regularization.

## Neural Networks for Larger Datasets

**Feed-forward neural network**: fully connected layers with nonlinear activation (ReLU, GELU). Learns non-linear sequence-fitness relationships including higher-order epistasis.

```python
import torch.nn as nn

class FitnessModel(nn.Module):
    def __init__(self, input_dim=1280, hidden=256):
        super().__init__()
        self.net = nn.Sequential(
            nn.Linear(input_dim, hidden),
            nn.ReLU(),
            nn.Dropout(0.2),
            nn.Linear(hidden, hidden // 2),
            nn.ReLU(),
            nn.Linear(hidden // 2, 1)
        )
    
    def forward(self, x):
        return self.net(x).squeeze(-1)
```

**When to use**: $N > 1000$. For smaller datasets, neural networks typically overfit despite regularization.

**Uncertainty estimation for NNs**: Monte Carlo dropout (use dropout at inference time; predict many times; variance across predictions = uncertainty) or deep ensembles (train multiple models; variance across model predictions = uncertainty).

**Ensembles + active learning**: training 5–10 models with different random seeds; using ensemble disagreement as the uncertainty metric is the most robust practical approach for MLDE.

## Model Selection and Cross-Validation

With limited training data, model selection must use cross-validation, not a held-out test set:

**k-fold cross-validation**: split training data into k folds (k = 5 is standard). Train on k-1 folds, evaluate on 1 fold. Rotate through all k possibilities. Average test error across folds = estimate of generalization error.

**Metrics**: Spearman rank correlation (how well the model ranks sequences by fitness, more relevant than R² for identifying top variants), RMSE (absolute prediction accuracy), Top-k recovery (fraction of the true top-k sequences that the model places in its predicted top-k).

**Model comparison**: always compare the ML model to a baseline of random sequence proposal. If MLDE doesn't outperform random proposal after $N$ measurements, the model is not providing useful guidance.

## Why This Matters

The surrogate model is the bottleneck of MLDE: if the model cannot accurately predict which unobserved sequences have high fitness, proposing sequences based on its predictions is no better than random. Choosing the right model for the available data size, the sequence representation, and the expected complexity of the fitness landscape (level of epistasis) is the central modeling challenge. The field has largely converged on using Gaussian processes or random forests for small datasets ($N < 500$), neural networks for larger datasets, and ESM-2 embeddings as the default representation. But the best combination for any specific protein and fitness property must be validated by empirical model comparison — there is no universal best surrogate model for directed evolution.
