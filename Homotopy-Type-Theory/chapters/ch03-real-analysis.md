# Chapter 3: Metric Spaces and the Topology of Analysis

## Introduction

Real analysis provides the foundational models that homotopy theory abstracts. Before we can understand homotopy between continuous maps, we need to understand *continuity* itself. Before we can understand path spaces and loop spaces, we need to understand what a *path* is in a topological space. And before topology, we need the cleaner, more metric-grounded world of analysis.

This chapter is deliberately compressed — a full analysis course would take a semester. We focus on the concepts that feed directly into topology (Chapter 13) and homotopy theory (Chapter 14): metric spaces, completeness, compactness, and continuous functions. We include enough detail to internalize these ideas, with pointers to what gets generalized in topology.

---

## 1. Metric Spaces

### 1.1 The Definition

**Definition 3.1 (Metric Space).** A *metric space* is a pair $(X, d)$ where $X$ is a set and $d : X \times X \to \mathbb{R}_{\geq 0}$ is a *metric* (or *distance function*) satisfying:
1. **Non-degeneracy:** $d(x, y) = 0 \Leftrightarrow x = y$.
2. **Symmetry:** $d(x, y) = d(y, x)$ for all $x, y$.
3. **Triangle inequality:** $d(x, z) \leq d(x, y) + d(y, z)$ for all $x, y, z$.

**Example 3.2.**
- $(\mathbb{R}^n, d_{\text{Euclid}})$ with $d(x, y) = \sqrt{\sum_i (x_i - y_i)^2}$.
- $(\mathbb{R}, |{-}|)$ with $d(x, y) = |x - y|$.
- Any set $X$ with the *discrete metric*: $d(x, y) = 0$ if $x = y$, else $1$.
- The *French railway metric* on $\mathbb{R}^2$: $d(x, y) = |x| + |y|$ if $x \neq y$, else $0$ (all travel goes through Paris).
- $C([0,1], \mathbb{R})$ with $d(f, g) = \sup_{t \in [0,1]} |f(t) - g(t)|$ (the *uniform metric*).

The last example is especially important: it is a metric space of *functions*, showing that the abstract setting of metric spaces encompasses far more than just familiar Euclidean space.

### 1.2 Open and Closed Sets

**Definition 3.3.** In a metric space $(X, d)$:
- The *open ball* $B(x, r) = \{y \in X \mid d(x, y) < r\}$ for $x \in X$, $r > 0$.
- A set $U \subseteq X$ is *open* if for every $x \in U$ there exists $r > 0$ with $B(x, r) \subseteq U$.
- A set $C \subseteq X$ is *closed* if its complement $X \setminus C$ is open.

**Theorem 3.4.** Arbitrary unions of open sets are open. Finite intersections of open sets are open. (And dually for closed sets: arbitrary intersections are closed, finite unions are closed.)

*Proof.* For unions: if $x \in \bigcup_\alpha U_\alpha$, then $x \in U_\alpha$ for some $\alpha$, so some ball $B(x, r) \subseteq U_\alpha \subseteq \bigcup U_\alpha$. For finite intersections: if $x \in U_1 \cap \cdots \cap U_n$, each $U_i$ contains a ball $B(x, r_i)$; then $B(x, \min_i r_i) \subseteq \bigcap U_i$. $\square$

**Remark 3.5.** This theorem motivates the abstract definition of a *topological space* (Chapter 13): a set with a designated collection of "open" sets satisfying exactly these closure properties, without any reference to a metric.

---

## 2. Convergence and Completeness

### 2.1 Sequences and Convergence

**Definition 3.6 (Convergence).** A sequence $(x_n)_{n=1}^\infty$ in a metric space $(X, d)$ *converges* to $x \in X$ (written $x_n \to x$ or $\lim_{n \to \infty} x_n = x$) if:
$$\forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall n \geq N,\ d(x_n, x) < \varepsilon.$$

Limits in metric spaces are unique (since if $x_n \to x$ and $x_n \to y$, then $d(x, y) \leq d(x, x_n) + d(x_n, y) \to 0$).

**Definition 3.7 (Cauchy Sequence).** A sequence $(x_n)$ is *Cauchy* if:
$$\forall \varepsilon > 0,\ \exists N \in \mathbb{N},\ \forall m, n \geq N,\ d(x_m, x_n) < \varepsilon.$$

Every convergent sequence is Cauchy (by the triangle inequality). The converse need not hold.

**Example 3.8.** The sequence $x_n = \sum_{k=1}^n \frac{1}{k^2}$ is Cauchy in $\mathbb{R}$ (its terms get close to each other since the partial sums of a convergent series form a Cauchy sequence). This sequence converges to $\pi^2/6$.

**Example 3.9.** In $\mathbb{Q}$ with the usual metric, the sequence $3, 3.1, 3.14, 3.141, \ldots$ (rational approximations to $\pi$) is Cauchy but does not converge — there is no rational limit.

### 2.2 Completeness

**Definition 3.10.** A metric space is *complete* if every Cauchy sequence converges.

**Theorem 3.11.** $\mathbb{R}$ is complete.

This is one of the fundamental properties of the real numbers, equivalent to several other characterizations:
- Every Cauchy sequence converges.
- Every bounded non-empty subset has a supremum (*least upper bound property*).
- Every non-empty nested sequence of closed bounded intervals has non-empty intersection.

**Theorem 3.12 (Baire Category Theorem).** A complete metric space is *Baire*: it cannot be written as a countable union of nowhere-dense sets.

This theorem, though seemingly technical, has powerful applications: it shows that $\mathbb{R}$ cannot be written as a countable union of "thin" sets, and it proves (among other things) that the set of continuous nowhere-differentiable functions is "most" of $C([0,1])$.

### 2.3 Completion

**Theorem 3.13.** Every metric space $(X, d)$ has a *completion* $(\hat{X}, \hat{d})$: a complete metric space into which $X$ embeds isometrically, with the image of $X$ dense in $\hat{X}$.

*Construction sketch.* Define $\hat{X}$ as the set of Cauchy sequences in $X$, modulo the equivalence $(x_n) \sim (y_n)$ iff $d(x_n, y_n) \to 0$. Define $\hat{d}([(x_n)], [(y_n)]) = \lim d(x_n, y_n)$. $\square$

**Example 3.14.** The completion of $\mathbb{Q}$ (under the standard metric) is $\mathbb{R}$. This is one clean construction of the real numbers.

---

## 3. Continuity

### 3.1 Continuous Functions

**Definition 3.15.** A function $f : (X, d_X) \to (Y, d_Y)$ is *continuous at $x \in X$* if:
$$\forall \varepsilon > 0,\ \exists \delta > 0,\ d_X(x, x') < \delta \Rightarrow d_Y(f(x), f(x')) < \varepsilon.$$

$f$ is *continuous* if it is continuous at every point. $f$ is *uniformly continuous* if $\delta$ can be chosen independently of $x$.

**Theorem 3.16 (Topological Characterization of Continuity).** $f : X \to Y$ is continuous if and only if the preimage of every open set in $Y$ is open in $X$.

*Proof.* ($\Rightarrow$): Let $V \subseteq Y$ be open, $x \in f^{-1}(V)$. Since $f(x) \in V$ and $V$ is open, there exists $\varepsilon > 0$ with $B(f(x), \varepsilon) \subseteq V$. By continuity, there exists $\delta > 0$ with $f(B(x,\delta)) \subseteq B(f(x), \varepsilon) \subseteq V$. So $B(x,\delta) \subseteq f^{-1}(V)$, showing $f^{-1}(V)$ is open.

($\Leftarrow$): Let $x \in X$ and $\varepsilon > 0$. The ball $B(f(x), \varepsilon)$ is open in $Y$, so $f^{-1}(B(f(x), \varepsilon))$ is open in $X$. Since $x$ is in this set, there exists $\delta > 0$ with $B(x, \delta) \subseteq f^{-1}(B(f(x), \varepsilon))$, i.e., $d_X(x, x') < \delta \Rightarrow d_Y(f(x), f(x')) < \varepsilon$. $\square$

This theorem is what makes continuity a *topological* concept: it only depends on the open sets, not on the specific metric. This is the bridge to Chapter 13.

### 3.2 Homeomorphisms

**Definition 3.17.** A *homeomorphism* is a continuous bijection whose inverse is also continuous. Metric spaces (or topological spaces) are *homeomorphic* if a homeomorphism between them exists.

Homeomorphic spaces are "topologically identical" — they have the same topological structure even if they look geometrically different.

**Example 3.18.** The open interval $(0,1)$ is homeomorphic to $\mathbb{R}$ via $f(x) = \tan(\pi(x - 1/2))$. A circle and a square boundary are homeomorphic. A coffee cup and a donut are homeomorphic (both have one "handle").

Homeomorphism is the correct notion of "sameness" for topological spaces — just as isomorphism is for groups. Homotopy equivalence (Chapter 14) is a coarser notion: homotopy equivalent spaces have the same homotopy groups but may not be homeomorphic.

---

## 4. Compactness

Compactness is the property that makes many important theorems provable. Intuitively, a compact space "behaves like a finite set" for many analytical purposes.

### 4.1 Open Cover Definition

**Definition 3.19.** A subset $K$ of a metric space is *compact* if every *open cover* of $K$ — a collection of open sets $\{U_\alpha\}$ with $K \subseteq \bigcup_\alpha U_\alpha$ — has a finite *subcover*.

**Theorem 3.20 (Heine-Borel).** A subset of $\mathbb{R}^n$ is compact if and only if it is closed and bounded.

*Proof sketch.* Closed and bounded subsets of $\mathbb{R}^n$ are compact by the Bolzano-Weierstrass theorem (every bounded sequence has a convergent subsequence) plus the fact that closed sets contain their limit points. Compact sets must be bounded (else $\{B(0, n)\}_{n \in \mathbb{N}}$ has no finite subcover) and closed (else some limit point lies outside). $\square$

**Theorem 3.21.** The continuous image of a compact space is compact.

*Proof.* If $f : K \to Y$ is continuous and $K$ is compact, any open cover $\{V_\alpha\}$ of $f(K)$ pulls back to the open cover $\{f^{-1}(V_\alpha)\}$ of $K$. This has a finite subcover $\{f^{-1}(V_1), \ldots, f^{-1}(V_n)\}$, and $\{V_1, \ldots, V_n\}$ covers $f(K)$. $\square$

**Corollary 3.22.** A continuous real-valued function on a compact metric space attains its maximum and minimum (*Extreme Value Theorem*).

**Theorem 3.23.** A continuous bijection from a compact space to a Hausdorff space is a homeomorphism.

---

## 5. Connectedness

**Definition 3.24.** A metric space $X$ is *connected* if it cannot be written as a disjoint union of two non-empty open sets.

$X$ is *path-connected* if for any $x, y \in X$ there is a continuous path $\gamma : [0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$.

Path-connected implies connected, but not conversely. The *topologist's sine curve* $\{(x, \sin(1/x)) \mid x > 0\} \cup \{0\} \times [-1,1]$ is connected but not path-connected.

**Theorem 3.25 (Intermediate Value Theorem).** If $f : [a,b] \to \mathbb{R}$ is continuous and $f(a) < c < f(b)$, then there exists $x \in (a,b)$ with $f(x) = c$.

*Proof.* The interval $[a,b]$ is connected (cannot be split into two non-empty disjoint open sets). The image $f([a,b])$ is connected (continuous image of a connected space). A connected subset of $\mathbb{R}$ is an interval, and an interval containing $f(a)$ and $f(b)$ must contain every value between them. $\square$

---

## 6. The Real Numbers: A Self-Contained Account

The real numbers $\mathbb{R}$ are characterized (up to isomorphism) as the unique *complete ordered field*:

**Definition 3.26 (Complete Ordered Field).** An ordered field $(F, +, \cdot, \leq)$ is *complete* if every non-empty subset of $F$ that is bounded above has a *least upper bound* (supremum) in $F$.

**Theorem 3.27.** Up to isomorphism, there is exactly one complete ordered field.

**Construction (Dedekind cuts):** A *Dedekind cut* is a pair $(L, R)$ of non-empty sets partitioning $\mathbb{Q}$ such that: every element of $L$ is less than every element of $R$, and $L$ has no maximum element. Real numbers are identified with Dedekind cuts. Addition, multiplication, and order are defined on cuts.

**Alternative construction (Cauchy sequences):** Real numbers are equivalence classes of Cauchy sequences of rationals (as in Theorem 3.13 applied to $\mathbb{Q}$).

Both constructions are standard; Dedekind's is more common in analysis, Cauchy's more common in algebra and number theory. In ZFC, these give different *sets* — but they are canonically isomorphic as ordered fields. In HoTT, by univalence, they are *equal as types* (since equivalent types are equal). This is a concrete instance of why univalence matters for foundations.

---

## 7. Connection to What Comes Next

### 7.1 From Metrics to Topologies

The key insight of this chapter is the *topological characterization of continuity* (Theorem 3.16): continuity depends only on the open sets, not on the metric. This suggests abstracting: define a space by specifying which sets are open, without requiring a metric. This is the definition of a topological space (Chapter 13).

Many "spaces" that arise naturally — function spaces, moduli spaces, spaces of homotopy classes of maps — are not metric spaces (or at least not usefully metrized). The purely topological framework handles them.

### 7.2 Paths as Fundamental Objects

A *path* in a space $X$ from $x$ to $y$ is a continuous function $\gamma : [0,1] \to X$ with $\gamma(0) = x$ and $\gamma(1) = y$. In homotopy theory, paths are the basic morphisms: they record how points are connected. In HoTT, paths become the identity type: an element of the identity type $x =_X y$ is precisely a path from $x$ to $y$ in the "space" $X$.

### 7.3 Homotopy as Deformation

A *homotopy* between paths $\gamma_0$ and $\gamma_1$ from $x$ to $y$ is a continuous function $H : [0,1] \times [0,1] \to X$ with $H(0, t) = x$, $H(1, t) = y$, $H(s, 0) = \gamma_0(s)$, $H(s, 1) = \gamma_1(s)$. Homotopy formalizes "continuous deformation" of one path into another.

In HoTT, a homotopy between paths $p, q : x = y$ is a term $H : p = q$ — an element of the identity type between paths. The "higher identity types" are the homotopy types of path spaces.

---

## Exercises

**3.1.** Verify the triangle inequality for the discrete metric.

**3.2.** Show that the French railway metric (Example 3.2) is a metric.

**3.3.** In $(\mathbb{R}, |{-}|)$, prove that a set is compact if and only if it is closed and bounded (using the Bolzano-Weierstrass theorem, which you may assume).

**3.4.** Show that $C([0,1], \mathbb{R})$ with the uniform metric is a complete metric space.

**3.5.** Give an example of a metric space that is complete but not compact.

**3.6.** Give an example of a compact metric space that is not the same as $\mathbb{R}^n$ for any $n$.

**3.7.** Prove that the composition of continuous functions is continuous.

**3.8.** Show that $(0,1)$ and $\mathbb{R}$ are homeomorphic but not isometric (i.e., there is no bijection preserving distances).

**3.9.** Prove the Intermediate Value Theorem directly (without invoking "connected implies interval") using the completeness of $\mathbb{R}$ and a bisection argument.

**3.10 (Challenge).** A metric space $X$ is *totally bounded* if for every $\varepsilon > 0$ it can be covered by finitely many balls of radius $\varepsilon$. Prove that a metric space is compact if and only if it is complete and totally bounded.
