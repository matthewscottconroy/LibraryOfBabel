# Chapter 20 — Information Geometry

> *The space of probability distributions is a manifold. Information geometry studies its intrinsic geometry — the Riemannian structure given by the Fisher information metric, and the pair of dual connections that replace the Levi-Civita connection.*

**Prerequisites:** Chapter 3 (differential geometry, Riemannian manifolds), Chapter 16 (KL divergence, information inequalities).

---

Probability distributions can be added (if you average them), scaled, and smoothly deformed — in short, they live in a space with geometric structure. Information geometry is the study of that geometry. The central insight, due to Rao (1945) and later developed by Amari into a systematic theory, is that the right Riemannian metric on the space of distributions is the *Fisher information metric*, and that KL divergence plays the role of a "geodesic distance" under a pair of dual connections.

This geometric perspective is not just aesthetically pleasing — it is operationally meaningful. The Fisher information metric directly bounds statistical estimation via the Cramér-Rao inequality. The KL divergence's Pythagorean theorem characterizes optimal statistical inference algorithms. The exponential family — Gaussians, Bernoullis, Poissons — are exactly the "flat" submanifolds under the information-geometric connections. And the EM algorithm for maximum likelihood estimation is, geometrically, alternating projections in the information-geometric sense.

Information geometry connects to several other threads in this book. The Fisher-Rao metric gives one natural Riemannian structure on probability space; the Wasserstein metric from optimal transport (Chapter 11) gives another. Both appear in the Fokker-Planck equation and its gradient flow structure. Natural gradient descent in machine learning is steepest descent in the Fisher metric. Hypothesis testing error exponents are geodesic distances.

This chapter develops the geometric framework, derives its main results, and shows where they connect to the rest of the story.

**What this chapter builds:**
- Statistical manifolds and the Fisher information matrix
- The Cramér-Rao lower bound and MLE efficiency
- Exponential families as flat submanifolds
- Dual connections: the $e$-connection and $m$-connection
- The Pythagorean theorem for KL divergence
- The EM algorithm as alternating projections
- Fisher information and optimal transport (Otto calculus)
- Natural gradient descent
- Hypothesis testing, Stein's lemma, and Chernoff information

**Sections:**
- [20.1 Statistical Manifolds](statistical-manifolds.md)
- [20.2 Exponential Families](exponential-families.md)
- [20.3 Dual Connections and Dually Flat Geometry](dual-connections.md)
- [20.4 KL Divergence and the Pythagorean Theorem](pythagorean-theorem.md)
- [20.5 Fisher Information and Optimal Transport](fisher-information-optimal-transport.md)
- [20.6 Natural Gradient](natural-gradient.md)
- [20.7 Hypothesis Testing and Information Geometry](hypothesis-testing.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
