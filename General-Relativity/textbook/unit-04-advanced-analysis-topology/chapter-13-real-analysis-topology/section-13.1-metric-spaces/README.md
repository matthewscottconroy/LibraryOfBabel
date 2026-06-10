# Section 13.1: Metric Spaces

---

## Section Introduction

A **metric space** is a set $X$ equipped with a **metric** (distance function) $d: X\times X\to\mathbb{R}_{\geq 0}$ satisfying: (1) $d(x,y) = 0$ iff $x = y$; (2) $d(x,y) = d(y,x)$; (3) $d(x,z)\leq d(x,y)+d(y,z)$ (triangle inequality). This abstraction captures the essential structure of distance while applying to spaces far more general than Euclidean space: function spaces, spaces of probability measures, combinatorial graphs, and the space of compact subsets of a metric space (with the Hausdorff metric).

The metric immediately induces topological structure: open balls $B(x,r) = \{y: d(x,y) < r\}$, open sets (unions of open balls), closed sets (complements of open sets), convergence ($x_n\to x$ iff $d(x_n,x)\to 0$), and continuity (the metric-space version of the $\varepsilon$-$\delta$ definition). A map $f: X\to Y$ between metric spaces is **continuous** iff $f^{-1}(U)$ is open in $X$ for every open $U$ in $Y$.

The most important property of a metric space for analysis is **completeness**: every Cauchy sequence converges. $\mathbb{R}^n$ with its usual metric is complete; $\mathbb{Q}$ is not. The function space $C([0,1])$ with the supremum metric $d(f,g) = \sup_{x}|f(x)-g(x)|$ is complete; with the $L^1$ metric $d(f,g) = \int|f-g|\,dx$ it is not (Cauchy sequences in $L^1$ can converge to discontinuous functions). Completeness is what distinguishes the real numbers from the rationals and what makes Hilbert and Banach spaces useful in analysis.

**Compactness** in a metric space has a beautifully concrete characterization: $X$ is compact iff it is complete and **totally bounded** (for every $\varepsilon > 0$, $X$ can be covered by finitely many balls of radius $\varepsilon$). Compact metric spaces are those for which every sequence has a convergent subsequence — the **Bolzano-Weierstrass property**. The Heine-Borel theorem characterizes compact subsets of $\mathbb{R}^n$ as exactly the closed and bounded ones.

---

## Subsections

- [13.1.1: Definition and Examples of Metric Spaces](13.1.1-definition.md)
- [13.1.2: Convergence and Continuity](13.1.2-convergence.md)
- [13.1.3: Completeness and Cauchy Sequences](13.1.3-completeness.md)
- [13.1.4: Compactness in Metric Spaces](13.1.4-compactness.md)
- [13.1.5: Baire Category Theorem](13.1.5-baire.md)
