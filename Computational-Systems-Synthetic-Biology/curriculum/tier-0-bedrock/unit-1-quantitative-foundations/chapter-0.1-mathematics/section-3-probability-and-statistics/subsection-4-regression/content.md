# Regression

Every genome-wide association study, at its heart, is a regression problem. You have a phenotype (disease status, blood pressure, height) that you are trying to explain with a predictor (genotype at a particular SNP). You do this simultaneously for millions of SNPs, each time fitting a model, estimating a coefficient, and computing a p-value. The sophistication of modern GWAS — controlling for population structure, modeling linkage disequilibrium, accounting for covariates like age and sex — is all built on the foundation of linear regression.

Regression is the statistical framework for modeling the relationship between a response variable and one or more predictor variables. In computational biology, regression underlies gene-trait associations in GWAS, differential expression analysis, feature selection in machine learning, and network inference. Mastering regression — from simple linear models to regularized variants — is one of the highest-return investments in statistical education.

## Linear Regression

**Simple linear regression** models a linear relationship between predictor $X$ and response $Y$:

$$Y_i = \beta_0 + \beta_1 X_i + \varepsilon_i, \quad \varepsilon_i \sim N(0, \sigma^2)$$

The **ordinary least squares (OLS)** estimators minimize $\text{RSS} = \sum_i (y_i - \hat{y}_i)^2$:

$$\hat{\beta}_1 = \frac{\sum_i (x_i - \bar{x})(y_i - \bar{y})}{\sum_i (x_i - \bar{x})^2} = \frac{\text{Cov}(X, Y)}{\text{Var}(X)}, \quad \hat{\beta}_0 = \bar{y} - \hat{\beta}_1 \bar{x}$$

**Multiple linear regression** extends this to $p$ predictors:

$$\mathbf{y} = X\boldsymbol{\beta} + \boldsymbol{\varepsilon}, \quad \boldsymbol{\varepsilon} \sim N(0, \sigma^2 I)$$

The OLS estimator is $\hat{\boldsymbol{\beta}} = (X^T X)^{-1} X^T \mathbf{y}$ — the solution to the **normal equations** $X^T X \hat{\boldsymbol{\beta}} = X^T \mathbf{y}$.

**Interpretation of coefficients:** $\hat{\beta}_j$ is the expected change in $Y$ per unit increase in $X_j$, holding all other predictors constant. This interpretation assumes the model is correctly specified — a strong assumption in biology, where unmeasured confounders lurk everywhere and the true relationship between genotype and phenotype is rarely linear.

**Model diagnostics:** Always check:
- Residual plots ($\hat{\varepsilon}_i$ vs. $\hat{y}_i$): should show no pattern
- Q-Q plot of residuals: should be approximately normal
- Leverage and Cook's distance: identify influential observations

## Logistic Regression

For a **binary outcome** $Y \in \{0, 1\}$ (disease/no disease, high/low expression), logistic regression models the log-odds:

$$\log\frac{P(Y=1 | \mathbf{x})}{P(Y=0 | \mathbf{x})} = \beta_0 + \beta_1 x_1 + \cdots + \beta_p x_p$$

The probability of the positive class is the **sigmoid function**:

$$P(Y=1 | \mathbf{x}) = \sigma(\mathbf{x}^T \boldsymbol{\beta}) = \frac{1}{1 + e^{-\mathbf{x}^T \boldsymbol{\beta}}}$$

Parameters are estimated by MLE (there is no closed form — numerical optimization is required). The **odds ratio** $e^{\hat{\beta}_j}$ represents the multiplicative change in odds for a unit increase in $X_j$ — the standard measure of effect size in GWAS logistic regression analysis. An odds ratio of 1.3 for a particular SNP means carrying that variant multiplies your odds of disease by 1.3 — a modest but detectable effect, which is characteristic of common complex disease variants.

## Generalized Linear Models

**Generalized linear models (GLMs)** extend regression to non-normal response distributions via:
1. A probability distribution from the exponential family (Poisson, binomial, gamma, negative binomial...)
2. A linear predictor $\eta = X\boldsymbol{\beta}$
3. A **link function** $g$ relating mean to linear predictor: $g(E[Y]) = \eta$

| Distribution | Link function | Biological use |
|---|---|---|
| Binomial | logit: $\log(\mu/(1-\mu))$ | Binary outcomes, allele counts |
| Poisson | log: $\log(\mu)$ | Count data, RNA-seq (simple) |
| Negative binomial | log | RNA-seq (overdispersed counts) |
| Gamma | log or inverse | Continuous positive quantities |

DESeq2 fits a negative binomial GLM for each gene: the linear predictor includes terms for experimental conditions and covariates (batch effects), and the log link ensures counts are positive. The connection between this carefully chosen distributional model and the actual biology of transcriptional variability is what makes DESeq2's results interpretable and well-calibrated.

## Regularization

When $p$ is large (many predictors) relative to $n$ (samples), OLS overfits. **Regularization** adds a penalty to the loss function to shrink coefficients toward zero:

**Ridge regression (L2 penalty):**

$$\hat{\boldsymbol{\beta}}_{\text{ridge}} = \arg\min_{\boldsymbol{\beta}} \left\{ \|y - X\boldsymbol{\beta}\|^2 + \lambda \|\boldsymbol{\beta}\|^2 \right\} = (X^T X + \lambda I)^{-1} X^T \mathbf{y}$$

Ridge shrinks all coefficients toward zero but keeps all of them nonzero. The $\lambda I$ term regularizes the matrix inversion — crucial when $X^T X$ is nearly singular (multicollinear predictors). In genetics, ridge regression is used for polygenic score estimation.

**LASSO (L1 penalty):**

$$\hat{\boldsymbol{\beta}}_{\text{lasso}} = \arg\min_{\boldsymbol{\beta}} \left\{ \|y - X\boldsymbol{\beta}\|^2 + \lambda \|\boldsymbol{\beta}\|_1 \right\}$$

The L1 penalty produces **sparse solutions** — many coefficients exactly zero. LASSO performs automatic feature selection, retaining only the most informative predictors. This is enormously useful in network inference (LASSO regression on expression data to infer regulatory connections) and high-dimensional association studies. The sparsity is not just computational convenience — it reflects the biological intuition that any one gene's expression is influenced by a small subset of the thousands of other genes, not by all of them equally.

**Elastic net:** Combines L1 and L2 penalties:

$$\lambda\left[\alpha \|\boldsymbol{\beta}\|_1 + \frac{1-\alpha}{2}\|\boldsymbol{\beta}\|^2\right]$$

Elastic net handles correlated predictors better than LASSO alone (LASSO arbitrarily picks one from a correlated group; elastic net tends to include all or none).

**Choosing $\lambda$:** Use cross-validation — the value of $\lambda$ that minimizes prediction error on held-out data. In Python, `sklearn.linear_model.LassoCV` and `RidgeCV` do this automatically.

## Why This Matters for Computational Biology

Regression is the most-used statistical tool in quantitative biology. GWAS associates genotype (predictor) with phenotype (response) using logistic or linear regression for millions of SNPs. DESeq2 and edgeR fit negative binomial GLMs to identify differentially expressed genes. Network inference algorithms like ARACNE and GENIE3 use regularized regression to infer regulatory relationships from expression data. Dimensionality reduction methods like PCA are closely related to linear regression. Understanding the assumptions, diagnostics, and limitations of regression models allows you to design experiments intelligently, interpret results correctly, and recognize when a model is misspecified.

```python
import numpy as np
from sklearn.linear_model import LassoCV, Ridge
from sklearn.preprocessing import StandardScaler
from sklearn.model_selection import train_test_split
import matplotlib.pyplot as plt

# Simulate gene expression network inference via LASSO
np.random.seed(42)
n_samples = 100
n_genes = 50  # total genes
n_regulators = 5  # true regulators of gene 0

# True regulatory coefficients (sparse)
true_betas = np.zeros(n_genes - 1)
true_betas[:n_regulators] = np.random.uniform(0.5, 2.0, n_regulators)
true_betas[:n_regulators] *= np.random.choice([-1, 1], n_regulators)

# Generate expression data
X_all = np.random.randn(n_samples, n_genes - 1)  # regulator expression
y = X_all @ true_betas + 0.5 * np.random.randn(n_samples)  # target expression

# Standardize
scaler = StandardScaler()
X_scaled = scaler.fit_transform(X_all)

# LASSO with cross-validation
lasso = LassoCV(cv=5, random_state=42)
lasso.fit(X_scaled, y)

# Identify non-zero coefficients (predicted regulators)
predicted_regulators = np.where(lasso.coef_ != 0)[0]
true_regulators = np.where(true_betas != 0)[0]

print(f"LASSO selected lambda = {lasso.alpha_:.4f}")
print(f"True regulators: {true_regulators}")
print(f"Predicted regulators: {predicted_regulators}")
precision = len(set(predicted_regulators) & set(true_regulators)) / max(len(predicted_regulators), 1)
recall = len(set(predicted_regulators) & set(true_regulators)) / len(true_regulators)
print(f"Precision: {precision:.2f}, Recall: {recall:.2f}")
```
