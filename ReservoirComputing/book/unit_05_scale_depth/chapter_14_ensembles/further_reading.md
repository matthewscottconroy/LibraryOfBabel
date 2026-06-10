# Chapter 14 — Further Reading and References

---

## Essential References

### [Breiman1996]

**Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.**

The founding paper for bagging. Remarkably accessible for a theoretical statistics paper — Breiman motivates the idea with intuition before the formal analysis, then proves the variance reduction theorem. The empirical results on decision trees and neural networks demonstrate the practical impact. Required reading.

### [Geman1992]

**Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.**

The paper that established the bias-variance framework for neural network analysis. Long (58 pages) but worth reading carefully. Contains both the formal decomposition and a rich empirical analysis of how bias and variance change with network architecture and training.

### [Krogh1995]

**Krogh, A. & Vedelsby, J. (1995). Neural network ensembles, cross validation, and active learning. *Advances in Neural Information Processing Systems*, 7.**

A short paper with a large impact: proves the ambiguity decomposition $E^{avg} = \bar{E} - \bar{A}$. This result is sharper and more actionable than the Breiman variance reduction result, and it directly motivates negative correlation learning.

---

## Ensemble Theory

### [Hastie2009, Chapter 8]

**Hastie, T., Tibshirani, R., & Friedman, J. (2009). *The Elements of Statistical Learning*, 2nd ed. Springer. Chapter 8: Model Inference and Averaging.**

The standard statistical learning theory reference for bias-variance and ensemble methods. Chapter 8 covers bagging, boosting, and the statistical foundations in a unified framework. Available free online.

### [Zhou2012]

**Zhou, Z.H. (2012). *Ensemble Methods: Foundations and Algorithms*. CRC Press.**

The most comprehensive book on ensemble methods. Covers bagging, boosting, random forests, mixture of experts, stacking, and diversity measures. Chapter 2 (on the diversity-accuracy tradeoff) is directly relevant to reservoir ensembles.

### [Dietterich2000]

**Dietterich, T.G. (2000). Ensemble methods in machine learning. In *Multiple Classifier Systems*, Lecture Notes in Computer Science, 1857. Springer. 1–15.**

A clear survey of ensemble methods with a focus on the diversity-accuracy tradeoff. Defines and analyzes four types of ensemble diversity and gives practical guidance on when each is effective.

---

## Mixture of Experts

### [Jacobs1991]

**Jacobs, R.A., Jordan, M.I., Nowlan, S.J., & Hinton, G.E. (1991). Adaptive mixtures of local experts. *Neural Computation*, 3(1), 79–87.**

The founding paper. Proposes the MoE architecture, the softmax gating network, and the EM training algorithm. Historically important and still highly readable.

### [Jordan1994]

**Jordan, M.I. & Jacobs, R.A. (1994). Hierarchical mixtures of experts and the EM algorithm. *Neural Computation*, 6(2), 181–214.**

The theoretical follow-up: derives the EM algorithm for MoE rigorously, establishes convergence guarantees, and introduces the hierarchical MoE. The probabilistic interpretation of MoE as a Gaussian mixture model is developed here.

### [Shazeer2017]

**Shazeer, N. et al. (2017). Outrageously large neural networks: The sparsely-gated mixture-of-experts layer. *ICLR 2017*.**

A modern application of MoE at enormous scale (up to 137 billion parameters). While the setting (language modeling) is far from reservoir computing, this paper demonstrates that the MoE idea from 1991 scales to the frontier of AI. The comparison is instructive: small MoE reservoirs are a principled approximation of the same idea at a fraction of the computational cost.

---

## Negative Correlation Learning

### [Liu1999]

**Liu, Y. & Yao, X. (1999). Ensemble learning via negative correlation. *Neural Networks*, 12(10), 1399–1404.**

Introduces the negative correlation (NC) learning framework: adding a diversity penalty to individual learner objectives. The paper proves that NC learning is equivalent to a particular stochastic approximation to the ambiguity-maximizing ensemble. Directly applicable to linear readout training in reservoir ensembles.

---

## Reservoir-Specific Ensemble Work

### [Strauss2012]

**Strauss, T., Wustlich, W., & Labahn, R. (2012). Design strategies for weight matrices of echo state networks. *Neural Computation*, 24(12), 3246–3276.**

Analyzes how structural choices in weight matrix design affect ESN performance. While not directly about ensembles, the analysis of how different matrix structures produce different representational properties provides the theoretical basis for understanding diversity in reservoir ensembles.

### [Lukosevicius2009]

**Lukoševičius, M. & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.**

The comprehensive review of reservoir computing. The section on model selection includes a discussion of ensemble-like approaches and hyperparameter search strategies that relates to ensemble diversity.
