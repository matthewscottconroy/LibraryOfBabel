# Chapter 14 — Key Researchers

---

## Leo Breiman

**Affiliation:** University of California, Berkeley (Department of Statistics; 1928–2005)

**Contributions:** Breiman invented bagging [Breiman1996] and random forests [Breiman2001], two of the most influential ensemble methods in machine learning. The bagging variance-reduction proof (Section 14.2) is directly from [Breiman1996]. Breiman's insight that averaging independent learners reduces variance without changing bias — and that diversity, not accuracy, is what distinguishes useful ensemble members — is the conceptual foundation of this entire chapter.

His later work on random forests showed that introducing randomness into the base learner (random subsets of features) can reduce correlation among ensemble members at a small cost in individual accuracy, yielding large ensemble improvements. This is precisely the logic behind varying reservoir hyperparameters across ensemble members.

**Selected publications:**
- Breiman, L. (1996). Bagging predictors. *Machine Learning*, 24(2), 123–140.
- Breiman, L. (2001). Random forests. *Machine Learning*, 45(1), 5–32.

---

## Stuart Geman

**Affiliation:** Brown University, Division of Applied Mathematics

**Contributions:** Co-author (with Bienenstock and Doursat) of the 1992 paper [Geman1992] that gave the bias-variance decomposition its canonical form and embedded it within the neural network discourse. Before this paper, bias and variance were known separately in statistics, but their joint application to the neural network generalization problem was Geman et al.'s contribution. The paper's title — "Neural Networks and the Bias/Variance Dilemma" — framed a generation's understanding of why deep networks overfit and why ensemble methods help.

**Selected publications:**
- Geman, S., Bienenstock, E., & Doursat, R. (1992). Neural networks and the bias/variance dilemma. *Neural Computation*, 4(1), 1–58.

---

## Anders Krogh

**Affiliation:** University of Copenhagen, Niels Bohr Institute (also previously Santa Fe Institute)

**Contributions:** Krogh and Vedelsby [Krogh1995] proved the ambiguity decomposition — the identity $E^{avg} = \bar{E} - \bar{A}$ — which gives the sharpest formal characterization of why diversity helps in ensembles. This paper is a landmark in ensemble theory and provides a tighter and more actionable analysis than the original Breiman variance-reduction argument.

**Selected publications:**
- Krogh, A. & Vedelsby, J. (1995). Neural network ensembles, cross validation, and active learning. In *Advances in Neural Information Processing Systems*, 7.
- Krogh, A. & Hertz, J. (1992). A simple weight decay can improve generalization. In *Advances in Neural Information Processing Systems*, 4.

---

## Robert Jacobs

**Affiliation:** University of Rochester, Department of Brain and Cognitive Sciences

**Contributions:** Jacobs, together with Michael Jordan, Geoffrey Hinton, and Steven Nowlan, introduced the mixture of experts framework [Jacobs1991]. This paper proposed both the softmax gating architecture and the EM training algorithm for MoE models, establishing the framework that Chapter 14 Section 14.4 applies to reservoir ensembles. The mixture-of-experts idea has had enormous influence in machine learning and AI, culminating recently in its use in trillion-parameter language models.

**Selected publications:**
- Jacobs, R.A., Jordan, M.I., Nowlan, S.J., & Hinton, G.E. (1991). Adaptive mixtures of local experts. *Neural Computation*, 3(1), 79–87.

---

## Michael Jordan

**Affiliation:** University of California, Berkeley, Departments of Statistics and Electrical Engineering and Computer Sciences

**Contributions:** Jordan's collaboration with Jacobs on the mixture of experts [Jacobs1991, Jordan1994] gave the framework its probabilistic foundation and established the EM algorithm as the appropriate training method. Jordan's broader work on graphical models, variational inference, and probabilistic machine learning provides the theoretical language in which the mixture of experts is most precisely understood.

**Selected publications:**
- Jordan, M.I. & Jacobs, R.A. (1994). Hierarchical mixtures of experts and the EM algorithm. *Neural Computation*, 6(2), 181–214.
