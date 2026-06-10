# Important Figures

## Augustin-Louis Cauchy (1789–1857)
*Founder of rigorous analysis; introduced the formal definitions of limit, continuity, and convergence.*

Cauchy was born in Paris and educated at the École Polytechnique, where he later taught. He was extraordinarily prolific — his collected works fill 27 volumes — and his influence on 19th-century analysis is without parallel. His personality was polarizing: intensely Catholic and politically conservative, he feuded with contemporaries and famously failed to appreciate Abel's and Galois's work during his lifetime.

His 1821 *Cours d'analyse* and 1823 *Résumé des leçons sur le calcul infinitésimal* introduced rigorous definitions of limit (a sequence $(a_n)$ converges to $L$ if for every $\varepsilon > 0$, there is $N$ such that $|a_n - L| < \varepsilon$ for all $n > N$), continuity of a function, and the definite integral as a limit of Riemann sums. These are the foundational definitions of this chapter. Cauchy also proved the convergence theorem bearing his name: a sequence converges if and only if it is a Cauchy sequence (in a complete space). The concept of Cauchy sequence, the Cauchy criterion for series, and Cauchy's mean value theorem all carry his name; together they form the backbone of the completeness theory developed in Section 2 of this chapter.

Cauchy's influence was immediate and global. Weierstrass, Riemann, and Dedekind all built on his definitions, correcting and sharpening them. The $\varepsilon$-$\delta$ precision that has defined mathematical analysis for 200 years is Cauchy's legacy.

---

## Karl Weierstrass (1815–1897)
*Systematizer of rigorous analysis; introduced the epsilon-delta definition of limits and uniform convergence.*

Weierstrass trained as a teacher and spent many years in secondary school posts in Westphalia before his mathematical work was finally recognized. He joined the University of Berlin at age 41, where he taught until he was nearly 80. His Berlin lectures became legendary: students came from across Europe to transcribe them, and his ideas circulated in handwritten notes before appearing in print.

Weierstrass replaced Cauchy's informal variable-approach language with the fully symbolic $\varepsilon$-$\delta$ definition that appears in every analysis course today: "$f$ is continuous at $a$ if for every $\varepsilon > 0$ there exists $\delta > 0$ such that $|x - a| < \delta$ implies $|f(x) - f(a)| < \varepsilon$." He distinguished sharply between pointwise and uniform continuity, and between pointwise and uniform convergence of sequences of functions — distinctions that resolve paradoxes about term-by-term integration and differentiation. His M-test (the Weierstrass M-test for uniform convergence of series of functions) is a standard tool of analysis. He also provided the first explicit example of a continuous nowhere-differentiable function (1872), demolishing the assumption that continuity and differentiability were nearly equivalent.

In this chapter, the Weierstrass extreme value theorem (a continuous function on a compact set attains its maximum and minimum) and the uniform continuity theorem (a continuous function on a compact metric space is uniformly continuous) are among the most-used results. These theorems, proved rigorously in Weierstrass's style, underpin the applied exercises on numerical methods and signal processing.

---

## Bernhard Riemann (1826–1866)
*Defined the Riemann integral; introduced Riemannian geometry and the topology of surfaces.*

Riemann was born in rural Hanover and studied theology before turning to mathematics at Göttingen under Gauss. He produced a small number of papers — the collected works fill only one volume — but each was transformative. He died of tuberculosis at 39, leaving the Riemann Hypothesis among the unsolved problems he had begun to explore.

His 1854 Habilitationsschrift, *Über die Darstellbarkeit einer Function durch eine trigonometrische Reihe* (On the Representability of a Function by a Trigonometric Series), introduced the Riemann integral as the limit of sums $\sum f(\xi_i)(x_{i+1} - x_i)$ over partitions of an interval. This definition, presented in Section 2 of this chapter, made integration independent of differentiation and extended it to a wide class of discontinuous functions. Riemann also gave necessary and sufficient conditions for Riemann integrability (in terms of the measure of the discontinuity set), precursors to Lebesgue's more general theory.

Riemann's 1854 lecture "Über die Hypothesen, welche der Geometrie zu Grunde liegen" (On the Hypotheses That Lie at the Foundations of Geometry) introduced what we now call Riemannian manifolds — spaces equipped with a smoothly varying inner product — and with them the concept of curvature intrinsic to a surface. This is directly relevant to this chapter's discussion of metric spaces: Riemannian distance is the arc-length metric on a smooth manifold, and the examples of curved metric spaces in Section 1 (the sphere with geodesic distance, hyperbolic space) are Riemannian.

---

## Richard Dedekind (1831–1916)
*Constructed the real numbers via Dedekind cuts; gave the first rigorous foundation for the completeness of the reals.*

Dedekind was Riemann's student at Göttingen, earning his doctorate in 1852. He spent most of his career at the Technische Hochschule in Braunschweig, where he worked in careful isolation from the mathematical mainstream, publishing only when he was fully satisfied with his results. He was one of the creators of algebraic number theory and edited Dirichlet's and Riemann's collected works.

His 1872 monograph *Stetigkeit und irrationale Zahlen* (Continuity and Irrational Numbers) contains the Dedekind cut construction of the real numbers. A Dedekind cut is a pair $(A, B)$ partitioning $\mathbb{Q}$ into a downward-closed set $A$ and an upward-closed set $B$ with no largest element in $A$. Each cut defines a real number — either a rational (when $B$ has a smallest element) or an irrational (when neither $A$ has a largest nor $B$ has a smallest). Dedekind proved that the resulting structure satisfies the least-upper-bound property: every nonempty set of cuts with an upper bound has a supremum. This is the completeness axiom, and it characterizes the real numbers uniquely (up to isomorphism). The construction is presented in Section 6 of this chapter alongside the Cauchy sequence construction.

Dedekind also wrote the foundational 1888 monograph *Was sind und was sollen die Zahlen?* (What Are Numbers and What Should They Be?), which gave a set-theoretic foundation for the natural numbers via the notion of a simply infinite system (essentially what we now call a Dedekind-Peano system). This work connects directly to the foundational concerns of Chapter 1 of this curriculum.

---

## Henri Lebesgue (1875–1941)
*Developed Lebesgue measure and the Lebesgue integral, extending integration beyond Riemann's framework.*

Lebesgue studied at the École Normale Supérieure in Paris and completed his doctorate in 1902 with his landmark thesis *Intégrale, longueur, aire* (Integral, Length, Area). He held positions at several French universities and was elected to the Académie des sciences in 1922. He was known as a gifted and accessible lecturer and wrote extensively on the philosophy of mathematics.

Lebesgue's 1902 thesis introduced measure theory and the Lebesgue integral, a generalization of Riemann's integral that handles a vastly larger class of functions. The key idea is to measure the preimage of a function's values rather than partitioning the domain: instead of asking "how large is $f$ on the small interval $[x_i, x_{i+1}]$?", Lebesgue asks "how large is the set of points where $f$ takes values in $[y_j, y_{j+1}]$?" The resulting integral (using the Lebesgue measure of these preimage sets) agrees with the Riemann integral where the latter is defined, but extends to functions that are Riemann non-integrable.

The $L^p$ spaces — $L^1$ (integrable functions), $L^2$ (square-integrable functions), $L^\infty$ (essentially bounded functions) — that appear in the applied exercises on Fourier series and signal processing are Lebesgue's legacy. The completeness of $L^2$ under its natural norm (the Riesz-Fischer theorem) is the analytic fact that makes Hilbert space theory work. The Lebesgue dominated convergence theorem and the monotone convergence theorem are the workhorses of modern analysis; their role is visible in the proofs of interchange of limits and integrals throughout this chapter.

---

## Georg Cantor (1845–1918)
*Invented set theory; gave the Cauchy sequence construction of the real numbers; proved the uncountability of the reals.*

Cantor was born in St. Petersburg and trained at Berlin under Weierstrass and Kronecker. He spent his career at the University of Halle. His mathematical life was marked by fierce opposition from Kronecker, who considered his transfinite set theory dangerous and philosophically unsound, and by repeated episodes of serious depression that led to hospitalisations. He was vindicated posthumously: Hilbert called his theory "the finest product of mathematical genius and one of the supreme achievements of purely intellectual human activity."

For this chapter, Cantor's most direct contributions are the Cauchy sequence construction of the real numbers (1872) and the proof that the real numbers are uncountable (1874, 1891). The diagonal argument for uncountability — constructing, given any list of real numbers, a real number not on the list — is one of the most elegant proofs in mathematics and appears in the real numbers section of this chapter. Cantor's construction of the reals as equivalence classes of Cauchy sequences of rationals (two sequences being equivalent if their difference converges to zero) is an alternative to Dedekind cuts that is particularly well-adapted to the metric space framework: it is a special case of the general completion of a metric space.

Cantor's broader set-theoretic work — ordinal and cardinal numbers, the continuum hypothesis — is foundational for this curriculum even where it is not invoked explicitly. The uniqueness-up-to-isomorphism of the real numbers (which the chapter proves) motivates the Univalence Axiom: in ZFC, Dedekind's reals and Cantor's reals are different sets related by an isomorphism; in HoTT, the Univalence Axiom makes isomorphic types literally equal.

---

## Henri Poincaré (1854–1912)
*Founder of topology and homotopy theory; defined the fundamental group; pioneered the study of paths and homotopies.*

Poincaré studied at the École Polytechnique and the École des Mines and held a chair in mathematical physics at the Sorbonne from 1881. He was a universalist of extraordinary breadth, contributing to complex analysis, celestial mechanics, mathematical physics (he came close to special relativity independently of Einstein), and philosophy of science, in addition to founding algebraic topology. His popular books on the philosophy of mathematics are still widely read.

For this chapter, Poincaré's fundamental contributions are the definitions of *path*, *homotopy*, and *fundamental group* from his 1895 *Analysis Situs* and its five supplements (1899–1904). A path in a topological space $X$ is a continuous map $\gamma : [0,1] \to X$; two paths are homotopic (with fixed endpoints) if one can be continuously deformed into the other. The *fundamental group* $\pi_1(X, x_0)$ — the group of homotopy classes of loops at a basepoint — is the first homotopy invariant of a space. Poincaré computed $\pi_1(S^1) = \mathbb{Z}$ and $\pi_1(\text{torus}) = \mathbb{Z}^2$. He stated, but could not prove, the Poincaré conjecture (that every simply connected closed 3-manifold is homeomorphic to $S^3$), which was resolved by Perelman in 2003.

The concepts developed in Section 7 of this chapter — paths as continuous maps, homotopy as continuous deformation, the groupoid structure of path composition up to homotopy — are all Poincaré's inventions. In HoTT, the identity type $\mathrm{Id}_A(a,b)$ is interpreted as the type of paths from $a$ to $b$; the proof that the identity type has a groupoid structure is precisely the proof that paths and homotopies behave as Poincaré observed. The computation $\pi_1(S^1) = \mathbb{Z}$ is a theorem of HoTT proved using higher inductive types, and it is the direct HoTT-internal analogue of Poincaré's original computation.
