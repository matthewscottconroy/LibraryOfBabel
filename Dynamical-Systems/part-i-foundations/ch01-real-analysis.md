# Chapter 1 — Real Analysis

> *The language of dynamical systems is analysis. Before orbits, attractors, and entropy, there must be limits, compactness, and continuity.*

**Prerequisites:** Calculus, linear algebra, naive set theory.

**What this chapter builds:** The metric space framework within which all of dynamical systems lives; the Banach and Hilbert space theory underlying transfer operators and spectral methods; and the Baire category theorem, which is the primary tool for proving that "generic" dynamical systems have certain properties.

---

## 1.1 Metric Spaces

### 1.1.1 Definition and First Examples

**Definition 1.1.1 (Metric Space).** A *metric space* is a pair $(X, d)$ where $X$ is a set and $d: X \times X \to [0, \infty)$ satisfies:
1. $d(x, y) = 0$ if and only if $x = y$ (identity of indiscernibles)
2. $d(x, y) = d(y, x)$ for all $x, y \in X$ (symmetry)
3. $d(x, z) \leq d(x, y) + d(y, z)$ for all $x, y, z \in X$ (triangle inequality)

**Examples 1.1.2.**
- $({\mathbb R}^n, d_2)$ with $d_2(x, y) = \|x - y\|_2 = \sqrt{\sum_i (x_i - y_i)^2}$ (Euclidean metric)
- $({\mathbb R}^n, d_\infty)$ with $d_\infty(x, y) = \max_i |x_i - y_i|$ (sup metric)
- $(C([0,1]), d_\infty)$ with $d_\infty(f, g) = \sup_{t \in [0,1]} |f(t) - g(t)|$ — the space of continuous functions with the uniform metric
- The *discrete metric* on any set $X$: $d(x, y) = 1$ if $x \neq y$ and $d(x, x) = 0$

**Remark 1.1.3.** The choice of metric matters: the same set can carry very different metric space structures. The continuous function space $(C([0,1]), d_2)$ with $d_2(f,g) = \sqrt{\int_0^1 |f-g|^2}$ is a very different metric space from $(C([0,1]), d_\infty)$ — the first is not complete, the second is.

### 1.1.2 Open and Closed Sets

**Definition 1.1.4.** Let $(X, d)$ be a metric space and $x \in X$, $r > 0$.
- The *open ball* of radius $r$ centered at $x$: $B(x, r) = \{y \in X : d(x, y) < r\}$
- The *closed ball*: $\bar{B}(x, r) = \{y \in X : d(x, y) \leq r\}$
- A set $U \subseteq X$ is *open* if for every $x \in U$ there exists $r > 0$ with $B(x, r) \subseteq U$
- A set $F \subseteq X$ is *closed* if its complement $X \setminus F$ is open

**Proposition 1.1.5.**
1. Arbitrary unions of open sets are open.
2. Finite intersections of open sets are open.
3. $\emptyset$ and $X$ are both open and closed.

**Definition 1.1.6.** The *interior* of $A \subseteq X$ is $\text{int}(A) = \{x \in A : \exists r > 0, B(x,r) \subseteq A\}$. The *closure* is $\bar{A} = \{x \in X : B(x, r) \cap A \neq \emptyset \text{ for all } r > 0\}$. The *boundary* is $\partial A = \bar{A} \setminus \text{int}(A)$.

### 1.1.3 Sequences and Convergence

**Definition 1.1.7.** A sequence $(x_n)$ in $(X, d)$ *converges* to $x \in X$ if $d(x_n, x) \to 0$ as $n \to \infty$. Equivalently: for all $\varepsilon > 0$ there exists $N$ such that $n \geq N$ implies $d(x_n, x) < \varepsilon$.

**Definition 1.1.8.** A sequence $(x_n)$ is *Cauchy* if for all $\varepsilon > 0$ there exists $N$ such that $m, n \geq N$ implies $d(x_m, x_n) < \varepsilon$.

Every convergent sequence is Cauchy. The converse fails in general but holds in complete metric spaces.

**Definition 1.1.9.** A metric space $(X, d)$ is *complete* if every Cauchy sequence converges.

**Examples 1.1.10.**
- ${\mathbb R}^n$ with any norm-induced metric is complete.
- $(C([0,1]), d_\infty)$ is complete (proof: a uniform Cauchy sequence of continuous functions has a continuous limit).
- $({\mathbb Q}, |\cdot|)$ is *not* complete — $(\sqrt{2}$ approximations are Cauchy in ${\mathbb Q}$ but converge outside ${\mathbb Q}$).

---

## 1.2 Compactness

Compactness is one of the most powerful properties a metric space can have, and most of the existence theorems in dynamical systems rely on it.

### 1.2.1 Equivalent Definitions

**Definition 1.2.1.** A subset $K \subseteq X$ of a metric space is *compact* if every open cover has a finite subcover: whenever $K \subseteq \bigcup_{\alpha \in I} U_\alpha$ for open sets $U_\alpha$, there exist finitely many $\alpha_1, \ldots, \alpha_n$ with $K \subseteq U_{\alpha_1} \cup \cdots \cup U_{\alpha_n}$.

**Theorem 1.2.2 (Equivalent Characterizations of Compactness in Metric Spaces).** For a subset $K$ of a metric space, the following are equivalent:
1. $K$ is compact (every open cover has a finite subcover)
2. $K$ is *sequentially compact* (every sequence in $K$ has a convergent subsequence with limit in $K$)
3. $K$ is *complete and totally bounded* (for every $\varepsilon > 0$, $K$ is covered by finitely many balls of radius $\varepsilon$)

*(proof sketch)* The equivalence $(1) \Leftrightarrow (3)$ is the key step. Total boundedness gives a way to extract convergent subsequences via a diagonal argument.

**Theorem 1.2.3 (Heine-Borel Theorem).** A subset $K \subseteq {\mathbb R}^n$ is compact if and only if it is closed and bounded.

**Remark 1.2.4.** Heine-Borel fails in infinite-dimensional spaces. The closed unit ball in an infinite-dimensional Banach space is never compact. This distinction is crucial when dynamical systems act on function spaces.

### 1.2.2 Properties of Compact Spaces

**Proposition 1.2.5.** Let $K$ be a compact metric space.
1. $K$ is complete and bounded.
2. Every closed subset of $K$ is compact.
3. If $f: K \to Y$ is continuous, then $f(K)$ is compact.
4. If $f: K \to {\mathbb R}$ is continuous, then $f$ attains its maximum and minimum.
5. If $f: K \to Y$ is continuous and bijective with $Y$ Hausdorff, then $f^{-1}$ is continuous.

---

## 1.3 Continuous Functions

### 1.3.1 Continuity and Uniform Continuity

**Definition 1.3.1.** A function $f: (X, d_X) \to (Y, d_Y)$ is *continuous at $x_0$* if for all $\varepsilon > 0$ there exists $\delta > 0$ such that $d_X(x, x_0) < \delta$ implies $d_Y(f(x), f(x_0)) < \varepsilon$.

$f$ is *continuous* if it is continuous at every point, and *uniformly continuous* if $\delta$ can be chosen independently of $x_0$.

**Theorem 1.3.2.** A continuous function on a compact metric space is uniformly continuous.

**Definition 1.3.3.** $f: X \to Y$ is *Lipschitz* with constant $L$ if $d_Y(f(x), f(x')) \leq L \cdot d_X(x, x')$ for all $x, x' \in X$.

Lipschitz continuity implies uniform continuity. The Lipschitz constant of a $C^1$ map is bounded by the supremum of $\|Df\|$.

### 1.3.2 The Arzelà-Ascoli Theorem

This theorem characterizes compact sets in function spaces and is the key tool for proving existence of invariant objects in dynamics.

**Definition 1.3.4.** A family $\mathcal{F}$ of functions $f: K \to {\mathbb R}$ (with $K$ compact) is *equicontinuous* at $x_0$ if for all $\varepsilon > 0$ there exists $\delta > 0$ such that $|x - x_0| < \delta$ implies $|f(x) - f(x_0)| < \varepsilon$ for all $f \in \mathcal{F}$ simultaneously.

$\mathcal{F}$ is *uniformly equicontinuous* if $\delta$ can be chosen independently of $x_0$.

**Theorem 1.3.5 (Arzelà-Ascoli).** Let $K$ be a compact metric space and $\mathcal{F} \subseteq C(K, {\mathbb R})$. Then $\mathcal{F}$ has compact closure in $(C(K, {\mathbb R}), d_\infty)$ if and only if $\mathcal{F}$ is pointwise bounded and equicontinuous.

*(proof sketch)* Given a sequence $(f_n)$ that is equicontinuous and pointwise bounded, choose a countable dense set $\{x_1, x_2, \ldots\} \subseteq K$ and apply a diagonal argument: first extract a subsequence converging at $x_1$, then a further subsequence converging at $x_2$, and so on. The diagonal subsequence converges pointwise on the dense set, and equicontinuity promotes this to uniform convergence.

**Application in Dynamics:** Given a sequence of maps $f_n: X \to X$ with $\sup_n \text{Lip}(f_n) < \infty$ on a compact space, Arzelà-Ascoli extracts a uniformly convergent subsequence. This is the standard argument for constructing invariant measures and proving fixed-point theorems.

---

## 1.4 Completeness and the Contraction Mapping Theorem

### 1.4.1 The Banach Fixed Point Theorem

**Definition 1.4.1.** A map $f: X \to X$ is a *contraction* if there exists $\lambda \in [0, 1)$ such that $d(f(x), f(y)) \leq \lambda \cdot d(x, y)$ for all $x, y \in X$.

**Theorem 1.4.2 (Banach Fixed Point Theorem / Contraction Mapping Theorem).** Let $(X, d)$ be a complete metric space and $f: X \to X$ a contraction with constant $\lambda$. Then:
1. $f$ has a unique fixed point $x^* \in X$.
2. For any $x_0 \in X$, the iterates $f^n(x_0) \to x^*$ as $n \to \infty$.
3. The rate of convergence is $d(f^n(x_0), x^*) \leq \lambda^n \cdot d(x_0, x^*)$.

*(proof)* The sequence $(f^n(x_0))$ is Cauchy: $d(f^m(x_0), f^n(x_0)) \leq \lambda^{\min(m,n)} d(x_0, f(x_0)) / (1-\lambda)$. By completeness it converges to some $x^*$, and continuity of $f$ gives $f(x^*) = x^*$. Uniqueness: if $x^*, y^*$ are both fixed, then $d(x^*, y^*) = d(f(x^*), f(y^*)) \leq \lambda d(x^*, y^*)$, so $d(x^*, y^*) = 0$.

**Application in Dynamics:** The contraction mapping theorem is used to prove: (1) existence and uniqueness for ODEs (Picard-Lindelöf), (2) existence of stable manifolds (the graph transform is a contraction), (3) construction of fractal attractors as fixed points of iterated function systems.

---

## 1.5 Differentiation in Banach Spaces

### 1.5.1 The Fréchet Derivative

For dynamical systems on manifolds and function spaces, we need differentiation in abstract spaces.

**Definition 1.5.1 (Fréchet Derivative).** Let $X, Y$ be Banach spaces and $f: U \subseteq X \to Y$ where $U$ is open. $f$ is *Fréchet differentiable* at $x_0 \in U$ if there exists a bounded linear map $Df(x_0): X \to Y$ such that
$$\lim_{\|h\| \to 0} \frac{\|f(x_0 + h) - f(x_0) - Df(x_0)h\|}{\|h\|} = 0.$$

The map $Df(x_0)$ is the *Fréchet derivative* of $f$ at $x_0$, also written $f'(x_0)$.

**Theorem 1.5.2 (Chain Rule).** If $f: U \to V$ and $g: V \to W$ are Fréchet differentiable, then $g \circ f$ is differentiable and $D(g \circ f)(x) = Dg(f(x)) \circ Df(x)$.

### 1.5.2 The Inverse and Implicit Function Theorems

**Theorem 1.5.3 (Inverse Function Theorem).** Let $f: U \subseteq X \to Y$ be $C^1$ and suppose $Df(x_0): X \to Y$ is a linear isomorphism (bounded with bounded inverse). Then there exist open sets $U' \ni x_0$ and $V' \ni f(x_0)$ such that $f|_{U'}: U' \to V'$ is a diffeomorphism.

**Theorem 1.5.4 (Implicit Function Theorem).** Let $F: U \subseteq X \times Y \to Z$ be $C^1$ with $F(x_0, y_0) = 0$ and $D_y F(x_0, y_0): Y \to Z$ a linear isomorphism. Then there exist neighborhoods $U' \ni x_0$ and a unique $C^1$ map $g: U' \to Y$ with $g(x_0) = y_0$ and $F(x, g(x)) = 0$ for all $x \in U'$.

---

## 1.6 Banach and Hilbert Spaces

### 1.6.1 Normed Spaces and Banach Spaces

**Definition 1.6.1.** A *normed space* $(X, \|\cdot\|)$ is a vector space $X$ over ${\mathbb R}$ (or ${\mathbb C}$) equipped with a norm satisfying: (i) $\|x\| \geq 0$ with equality iff $x = 0$; (ii) $\|\alpha x\| = |\alpha| \|x\|$; (iii) $\|x + y\| \leq \|x\| + \|y\|$. A *Banach space* is a complete normed space.

**Examples 1.6.2.**
- $\ell^p = \{(a_n)_{n \geq 1} : \sum |a_n|^p < \infty\}$ with $\|(a_n)\|_p = (\sum |a_n|^p)^{1/p}$ for $1 \leq p < \infty$
- $\ell^\infty = \{(a_n) : \sup_n |a_n| < \infty\}$ with the sup norm
- $C(K)$ for compact $K$, with the uniform norm
- $L^p(\mu)$ for a measure space $(\Omega, \mathcal{F}, \mu)$ — functions with finite $p$-th moment

### 1.6.2 Hilbert Spaces

**Definition 1.6.3.** A *Hilbert space* $(H, \langle \cdot, \cdot \rangle)$ is a Banach space whose norm is induced by an inner product: $\|x\|^2 = \langle x, x \rangle$, where $\langle \cdot, \cdot \rangle: H \times H \to {\mathbb R}$ is bilinear, symmetric, and positive definite.

**Theorem 1.6.4 (Cauchy-Schwarz Inequality).** For any $x, y \in H$: $|\langle x, y \rangle| \leq \|x\| \cdot \|y\|$.

**Theorem 1.6.5 (Projection Theorem).** Let $H$ be a Hilbert space and $K \subseteq H$ a closed convex subset. For any $x \in H$, there exists a unique $\hat{x} \in K$ minimizing $\|x - k\|$ over $k \in K$. The map $x \mapsto \hat{x}$ is the *orthogonal projection* onto $K$.

**Definition 1.6.6.** A *Schauder basis* for a Banach space $X$ is a sequence $(e_n)$ such that every $x \in X$ has a unique expansion $x = \sum_{n=1}^\infty a_n e_n$. An *orthonormal basis* for a Hilbert space $H$ is a Schauder basis with $\langle e_m, e_n \rangle = \delta_{mn}$.

**Examples 1.6.7.** 
- $L^2([0,1])$ has ONB $\{e^{2\pi i n t}\}_{n \in {\mathbb Z}}$ (Fourier series).
- Every separable Hilbert space is isometrically isomorphic to $\ell^2$.

**Application in Dynamics:** The Koopman operator of a measure-preserving transformation acts on $L^2(\mu)$ as a unitary operator. Its spectral theory (eigenvalues, spectral measures, cyclic vectors) governs mixing, ergodicity, and the recurrence structure of the system.

---

## 1.7 The Baire Category Theorem

The Baire Category Theorem is the cornerstone of topological genericity arguments in dynamics. Most "pathological" examples and most "typical" properties are proved via Baire's theorem.

### 1.7.1 Statement and Proof

**Definition 1.7.1.** A subset $A$ of a metric space $X$ is:
- *nowhere dense* if $\text{int}(\bar{A}) = \emptyset$ (its closure has empty interior)
- *meager* (or *of first category*) if it is a countable union of nowhere dense sets
- *residual* (or *comeager*) if its complement is meager

**Theorem 1.7.2 (Baire Category Theorem).** Let $(X, d)$ be a complete metric space. Then:
1. Every residual set is dense.
2. $X$ is not meager.
3. Equivalently: a countable intersection of open dense sets is dense.

*(proof)* Let $U_1, U_2, \ldots$ be open dense sets. We build a nested sequence of open balls: start with any open ball $B_0$. Since $U_1$ is dense, $U_1 \cap B_0 \neq \emptyset$; find a closed ball $B_1 \subseteq U_1 \cap B_0$ of radius $\leq 1/2$. Since $U_2$ is dense, find $B_2 \subseteq U_2 \cap \text{int}(B_1)$ of radius $\leq 1/4$. Continue: $B_n \subseteq U_n \cap \text{int}(B_{n-1})$ with radius $\leq 2^{-n}$. The centers form a Cauchy sequence, converging to some $x^* \in \bigcap_n B_n \subseteq \bigcap_n U_n$.

### 1.7.2 Generic Properties in Dynamics

**Definition 1.7.3.** A property $\mathcal{P}$ is *generic* in a complete metric space $X$ if the set $\{x \in X : \mathcal{P}(x) \text{ holds}\}$ is residual.

**Interpretation:** Generic = "typical" in the topological sense. A generic property holds for "most" elements. This is the standard notion of typicality in topological dynamics, distinct from the measure-theoretic notion of "almost everywhere."

**Theorem 1.7.4 (Generic Continuity — Baire).** Let $f: X \to {\mathbb R}$ be a pointwise limit of continuous functions on a complete metric space $X$. Then $f$ is continuous on a residual set.

**Example 1.7.5 (Generic Dynamics).** In the space $\text{Homeo}(X)$ of homeomorphisms of a compact metric space, many properties are generic. For instance, generic homeomorphisms of the Cantor set are minimal (every orbit is dense). Generic continuous maps of $[0,1]$ are nowhere differentiable.

---

## 1.8 The Stone-Weierstrass Theorem

**Theorem 1.8.1 (Stone-Weierstrass).** Let $K$ be a compact Hausdorff space and $\mathcal{A} \subseteq C(K, {\mathbb R})$ a subalgebra that separates points (for $x \neq y$, some $f \in \mathcal{A}$ has $f(x) \neq f(y)$) and contains the constant functions. Then $\mathcal{A}$ is dense in $C(K, {\mathbb R})$ under the uniform norm.

**Corollary 1.8.2.** Polynomials are dense in $C([a,b])$. Trigonometric polynomials are dense in $C({\mathbb T})$ where ${\mathbb T} = {\mathbb R}/{\mathbb Z}$.

**Application in Dynamics:** Stone-Weierstrass implies that to specify a measure on a compact space, it suffices to specify its integrals against polynomials or trigonometric polynomials. This is the basis for the moment problem and for approximating invariant measures.

---

## Exercises

**Exercise 1.1.** Let $(X, d)$ be a metric space. Show that $d': X \times X \to [0,\infty)$ defined by $d'(x,y) = d(x,y)/(1+d(x,y))$ is also a metric on $X$, and that $(X,d)$ and $(X,d')$ have the same open sets (i.e., the same topology).

**Exercise 1.2.** Prove that the intersection of a compact set and a closed set is compact.

**Exercise 1.3.** Let $f: X \to Y$ be a continuous bijection with $X$ compact and $Y$ Hausdorff. Prove that $f^{-1}$ is continuous.

**Exercise 1.4.** Show that $C([0,1])$ with the $L^1$ norm $\|f\|_1 = \int_0^1 |f(t)|\,dt$ is not complete. (*Hint:* Find a Cauchy sequence whose pointwise limit is not continuous.)

**Exercise 1.5.** (Baire) Let $X = {\mathbb R}$ with the usual metric. Show that ${\mathbb Q}$ is meager in ${\mathbb R}$ but that ${\mathbb R} \setminus {\mathbb Q}$ (the irrationals) is residual. Conclude that the irrationals are dense in ${\mathbb R}$.

**Exercise 1.6.** Let $H$ be a Hilbert space with orthonormal basis $\{e_n\}_{n=1}^\infty$. Define the *shift operator* $S: H \to H$ by $S(e_n) = e_{n+1}$. Show that $S$ is an isometry (preserves the norm) but is not unitary (not surjective). The *adjoint* $S^*$ satisfies $S^*(e_1) = 0$ and $S^*(e_n) = e_{n-1}$ for $n \geq 2$. Show that $S^*S = I$ but $SS^* \neq I$.

**Exercise 1.7.** (Contraction Mapping Theorem) Let $\lambda \in (0,1)$ and define $f: {\mathbb R} \to {\mathbb R}$ by $f(x) = \lambda x + (1-\lambda) x_0$ for a fixed $x_0$. Show that $f$ is a contraction with fixed point $x_0$. Generalize: show that if $g: [a,b] \to [a,b]$ is differentiable with $|g'(x)| \leq \lambda < 1$ for all $x \in [a,b]$, then $g$ has a unique fixed point.

**Exercise 1.8.** Prove the Arzelà-Ascoli Theorem for $K = [0,1]$: a sequence $(f_n)$ in $C([0,1])$ with $\sup_n \|f_n\|_\infty \leq M$ and uniform equicontinuity has a uniformly convergent subsequence.

**Exercise 1.9.** (Research Connection) The Collatz map $C: {\mathbb N} \to {\mathbb N}$ defined by $C(n) = 3n+1$ if $n$ odd and $C(n) = n/2$ if $n$ even extends to a map on the *2-adic integers* ${\mathbb Z}_2$ (the completion of ${\mathbb Z}$ under the 2-adic metric $d_2(m,n) = 2^{-v_2(m-n)}$ where $v_2(k)$ is the largest power of 2 dividing $k$). Show that ${\mathbb Z}_2$ is complete under $d_2$. What does the Collatz conjecture become in this 2-adic setting?

---

## Chapter Notes

The material in this chapter is covered thoroughly in Rudin's *Principles of Mathematical Analysis* (Chapters 2, 3, 4, 7) and *Real and Complex Analysis* (Chapters 1-5). The functional analysis material (Sections 1.6) is in Rudin's *Functional Analysis*. For the Baire Category Theorem and its applications, see Oxtoby's *Measure and Category* — a beautiful short book that develops the precise analogy between measure-zero sets (null sets) and meager sets (first-category sets) and their respective notions of "almost all."

The Arzelà-Ascoli theorem is particularly important for what follows: it reappears in the proof of existence of invariant measures (Chapter 6), in the construction of stable manifolds (Chapter 9), and in the theory of attractors for infinite-dimensional systems (Chapter 15).
