# Classification Readouts for Reservoir Computing

## Classification as a Readout Problem

Reservoir computing is naturally suited to regression: the readout maps reservoir states to real-valued outputs via a learned linear combination. Classification tasks, by contrast, require assigning discrete labels to inputs. The reservoir state $\mathbf{x}_t \in \mathbb{R}^N$ still serves as the feature representation, but the readout must now implement a decision function rather than a continuous approximation. This section surveys the principal classification readout architectures, their training procedures, and their comparative strengths [Duda et al. 2001].

## Softmax Readout with Cross-Entropy Training

For $K$-class classification, the softmax readout assigns a probability to each class:

$$p(y = k \mid \mathbf{x}) = \frac{\exp(\mathbf{w}_k^\top \mathbf{x} + b_k)}{\sum_{j=1}^{K} \exp(\mathbf{w}_j^\top \mathbf{x} + b_j)}, \quad k = 1, \ldots, K,$$

where $\mathbf{w}_k \in \mathbb{R}^N$ is the weight vector for class $k$ and $b_k$ is its bias. The weight matrix $\mathbf{W} = [\mathbf{w}_1 \mid \cdots \mid \mathbf{w}_K]^\top \in \mathbb{R}^{K \times N}$ is trained by minimizing the cross-entropy loss:

$$\mathcal{L}(\mathbf{W}) = -\frac{1}{T} \sum_{t=1}^{T} \sum_{k=1}^{K} y_{t,k}^* \log p(y = k \mid \mathbf{x}_t),$$

where $y_{t,k}^* \in \{0,1\}$ is the one-hot target. Because the softmax is a composition of a linear function with a fixed nonlinearity, the cross-entropy loss is convex in $\mathbf{W}$ when $\mathbf{x}_t$ is fixed [Duda et al. 2001]. Gradient descent converges to the global minimum, and $L_2$ regularization (adding $\lambda \|\mathbf{W}\|_F^2$ to $\mathcal{L}$) yields a well-posed problem even when $N > T$.

## Linear Discriminant Analysis

Linear discriminant analysis (LDA) seeks a projection direction $\mathbf{w}$ that maximizes between-class variance while minimizing within-class variance, formalized through Fisher's criterion:

$$J(\mathbf{w}) = \frac{\mathbf{w}^\top \mathbf{S}_B \mathbf{w}}{\mathbf{w}^\top \mathbf{S}_W \mathbf{w}},$$

where $\mathbf{S}_B = \sum_{k=1}^K n_k (\boldsymbol{\mu}_k - \boldsymbol{\mu})(\boldsymbol{\mu}_k - \boldsymbol{\mu})^\top$ is the between-class scatter matrix and $\mathbf{S}_W = \sum_{k=1}^K \sum_{t: y_t = k} (\mathbf{x}_t - \boldsymbol{\mu}_k)(\mathbf{x}_t - \boldsymbol{\mu}_k)^\top$ is the within-class scatter matrix. The optimal projection directions are the leading eigenvectors of $\mathbf{S}_W^{-1} \mathbf{S}_B$.

For reservoir classification, LDA provides up to $K-1$ discriminant dimensions, which is often sufficient. Its closed-form solution is computationally efficient and statistically well-grounded under Gaussian class-conditional densities. The homoscedasticity assumption (equal covariance across classes) is rarely satisfied in practice, but LDA remains robust in many reservoir state settings [Duda et al. 2001].

## Support Vector Machines on Reservoir States

The support vector machine (SVM) finds the maximum-margin hyperplane separating two classes:

$$\min_{\mathbf{w}, b, \boldsymbol{\xi}} \frac{1}{2}\|\mathbf{w}\|^2 + C \sum_{t=1}^T \xi_t \quad \text{subject to} \quad y_t(\mathbf{w}^\top \mathbf{x}_t + b) \geq 1 - \xi_t, \; \xi_t \geq 0,$$

where $y_t \in \{-1, +1\}$ and $C > 0$ is the regularization parameter [Vapnik 1995]. The dual formulation involves only inner products $\mathbf{x}_i^\top \mathbf{x}_j$, enabling the kernel trick: replace $\mathbf{x}_i^\top \mathbf{x}_j$ with $k(\mathbf{x}_i, \mathbf{x}_j)$. The radial basis function kernel $k(\mathbf{x}, \mathbf{x}') = \exp(-\gamma \|\mathbf{x} - \mathbf{x}'\|^2)$ is the standard choice.

SVMs are particularly effective for reservoir classification when training sets are small: the margin maximization principle provides strong generalization even with $T \ll N$, a regime that defeats poorly regularized methods [Vapnik 1995].

## One-vs-Rest vs. One-vs-One for Multiclass

Multiclass SVM requires decomposing the $K$-class problem into binary subproblems. Two standard strategies exist.

**One-vs-rest (OvR):** Train $K$ binary SVMs, each distinguishing class $k$ from all others. Predict the class with the highest margin score. OvR requires $K$ SVMs and $K \cdot T$ training samples (with class imbalance in each subproblem).

**One-vs-one (OvO):** Train $\binom{K}{2}$ binary SVMs on all pairs of classes. Predict by majority vote. OvO trains on balanced subsets of size $2T/K$ each, which often produces better margins, at the cost of $O(K^2)$ classifiers.

Empirically, OvO tends to outperform OvR for reservoir state classification when the number of classes is moderate ($K \lesssim 20$), because the smaller, more balanced training sets exploit the high-dimensional reservoir feature space more effectively [Duda et al. 2001].

## Class-Weighted Ridge Regression

For imbalanced datasets — where class sizes $n_k$ differ substantially — standard ridge regression minimizes a loss dominated by the majority class. The remedy is class-weighted ridge regression:

$$\hat{\mathbf{W}} = \underset{\mathbf{W}}{\arg\min} \sum_{t=1}^T w_t \|\mathbf{W} \mathbf{x}_t - \mathbf{y}_t^*\|^2 + \lambda \|\mathbf{W}\|_F^2,$$

where the sample weight $w_t = 1/n_{y_t}$ upweights minority-class examples. This is equivalent to reweighting the design matrix: let $\mathbf{D} = \text{diag}(w_1, \ldots, w_T)$, then

$$\hat{\mathbf{W}} = \mathbf{Y}^{*\top} \mathbf{D} \mathbf{X} (\mathbf{X}^\top \mathbf{D} \mathbf{X} + \lambda \mathbf{I})^{-1},$$

which has the same $O(N^3)$ cost as standard ridge but corrects the class imbalance [Duda et al. 2001].

## Comparative Summary

The choice of classification readout depends primarily on dataset size and structure. SVMs are consistently strong for small datasets ($T < 1000$) and high-dimensional reservoir states, as margin maximization provides the most reliable generalization. Softmax with cross-entropy is preferred when $T$ is large and streaming training (via SGD) is needed. LDA is fast and interpretable, ideal when normality assumptions hold or when computational budget is tight. Class-weighted ridge regression is the recommended baseline for imbalanced classification in the streaming setting, where its online extension (RLS with sample weights) is directly applicable.

---

## References

- Duda, R. O., Hart, P. E., & Stork, D. G. (2001). *Pattern Classification* (2nd ed.). Wiley-Interscience.
- Vapnik, V. N. (1995). *The Nature of Statistical Learning Theory*. Springer.
