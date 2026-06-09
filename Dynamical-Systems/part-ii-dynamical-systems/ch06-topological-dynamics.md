# Chapter 6 — Topological Dynamics

> *Topological dynamics is the study of continuous maps on topological spaces. With no metric and no measure, it isolates the purely qualitative: which orbits are dense, which sets are attracting, which systems are equivalent.*

**Prerequisites:** Chapters 1 (metric spaces, Baire theorem), 3 (topology, compact spaces).

**What this chapter builds:** The framework of orbits, limit sets, and recurrence; the key notions of transitivity, minimality, and equicontinuity; topological conjugacy as the isomorphism relation for topological dynamical systems; and the foundational existence theorem for invariant measures (Krylov-Bogoliubov).

---

## 6.1 Discrete Dynamical Systems

### 6.1.1 Setup and Basic Definitions

**Definition 6.1.1.** A *topological dynamical system* (TDS) is a pair $(X, f)$ where $X$ is a compact metrizable space and $f: X \to X$ is continuous. (If $f$ is a homeomorphism, the system is *invertible*.)

The *orbit* of $x \in X$ under $f$ is $\mathcal{O}(x) = \{f^n(x) : n \in {\mathbb N}\}$ (or $n \in {\mathbb Z}$ if $f$ is invertible).

**Definition 6.1.2.** The *omega-limit set* of $x$ is
$$\omega_f(x) = \bigcap_{N \geq 0} \overline{\{f^n(x) : n \geq N\}} = \{y : f^{n_k}(x) \to y \text{ for some } n_k \to \infty\}.$$

**Proposition 6.1.3.** For compact $X$ and continuous $f$:
1. $\omega_f(x)$ is nonempty, closed, and $f$-invariant ($f(\omega_f(x)) \subseteq \omega_f(x)$).
2. If $f$ is a homeomorphism, $f(\omega_f(x)) = \omega_f(x)$ (positively and negatively invariant).
3. $\omega_f(x)$ is connected if $X$ is connected.

*(proof of 1)* Nonempty: the sequence $(f^n(x))_n$ has a convergent subsequence by compactness. Closed: direct from definition as an intersection of closed sets. Invariance: if $y = \lim_k f^{n_k}(x)$, then $f(y) = \lim_k f^{n_k+1}(x) \in \omega_f(x)$.

### 6.1.2 Periodic Points

**Definition 6.1.4.** A point $x$ is *periodic* with (minimal) period $n \geq 1$ if $f^n(x) = x$ and $f^k(x) \neq x$ for $0 < k < n$. A period-1 point is a *fixed point*.

The set of periodic points of $f$ is $\text{Per}(f) = \bigcup_{n \geq 1} \text{Fix}(f^n)$.

**Example 6.1.5 (Quadratic Maps).** For $f_c: {\mathbb R} \to {\mathbb R}$, $f_c(x) = x^2 + c$:
- $c = 0$: $x = 0$ is a fixed point (attracting); $x = 1$ is also fixed (repelling).
- $c = -2$: $f_{-2}$ on $[-2, 2]$ has dense periodic points (topologically conjugate to the tent map).
- $c = -1$: period-2 orbit at $\{0, -1\}$.

---

## 6.2 Recurrence

**Definition 6.2.1.** A point $x \in X$ is:
- *recurrent* if $x \in \omega_f(x)$ (i.e., the orbit of $x$ returns arbitrarily close to $x$)
- *nonwandering* if for every open $U \ni x$ there exists $n \geq 1$ with $f^n(U) \cap U \neq \emptyset$

Every periodic point is recurrent; every recurrent point is nonwandering.

**Definition 6.2.2.** The *nonwandering set* $\Omega(f)$ consists of all nonwandering points. It is a closed $f$-invariant set.

**Theorem 6.2.3 (Poincaré Recurrence Theorem — Topological Version).** Let $f: X \to X$ be a homeomorphism of a compact metric space. For any open set $U \neq \emptyset$, there exists $n \geq 1$ with $f^n(U) \cap U \neq \emptyset$.

*Proof:* Consider the sets $U, f^{-1}(U), f^{-2}(U), \ldots$ If they were pairwise disjoint, they could not all fit in a compact space (since $X$ has finite "covering number"). So some $f^{-m}(U) \cap f^{-n}(U) \neq \emptyset$ for $m < n$, i.e., $f^{n-m}(U) \cap U \neq \emptyset$.

**Corollary 6.2.4.** $\Omega(f) = X$ for any homeomorphism of a compact space preserving a full-support measure. In particular, every point is nonwandering — orbits keep returning.

---

## 6.3 Topological Transitivity and Mixing

**Definition 6.3.1.** $(X, f)$ is *topologically transitive* if there exists a point $x \in X$ with dense orbit: $\overline{\mathcal{O}(x)} = X$.

Equivalently (for compact metric spaces with countable basis): for every pair of nonempty open sets $U, V \subseteq X$, there exists $n \geq 0$ with $f^n(U) \cap V \neq \emptyset$.

**Definition 6.3.2.** $(X, f)$ is *topologically mixing* if for every nonempty open $U, V \subseteq X$, there exists $N$ such that $f^n(U) \cap V \neq \emptyset$ for all $n \geq N$ (not just for some $n$).

Mixing implies transitivity but not vice versa (an irrational rotation is transitive but not mixing).

**Theorem 6.3.3 (Baire Category and Transitivity).** For a compact metric space without isolated points and a homeomorphism $f$: $(X, f)$ is transitive iff the set of points with dense orbits is a dense $G_\delta$ (residual) set.

*(proof sketch)* The set $\{x : f^n(x) \in V\}$ is open and dense for each open $V$ (by transitivity). The set of points with dense orbits is $\bigcap_{V \in \mathcal{V}} \bigcup_{n \geq 0} f^{-n}(V)$ over a countable basis $\mathcal{V}$ — a countable intersection of open dense sets, hence residual by Baire.

---

## 6.4 Minimality

**Definition 6.4.1.** A TDS $(X, f)$ is *minimal* if every orbit is dense: $\overline{\mathcal{O}(x)} = X$ for all $x \in X$. Equivalently, $X$ has no proper closed $f$-invariant subset.

**Examples 6.4.2.**
- *Irrational rotations*: $R_\alpha: {\mathbb T} \to {\mathbb T}$, $R_\alpha(x) = x + \alpha \pmod{1}$ for $\alpha \notin {\mathbb Q}$ is minimal. (Proof: orbit of any point equidistributes — see Weyl, Chapter 31.)
- *Minimal subshifts*: Sturmian sequences, Thue-Morse, other combinatorially defined systems.
- *Subshifts of Finite Type* are never minimal (they have periodic points).

**Theorem 6.4.3 (Existence of Minimal Subsystems).** Every compact TDS $(X, f)$ contains a minimal subset (a closed $f$-invariant set on which $f$ is minimal).

*(proof)* Apply Zorn's lemma to the family of nonempty closed $f$-invariant subsets of $X$ (ordered by reverse inclusion). Any chain has a lower bound (the intersection); a minimal element in the ordering is a minimal subset.

**Remark 6.4.4.** The existence of minimal subsystems is a fundamental compactness argument. It is the topological analogue of the existence of ergodic components in measure theory.

---

## 6.5 Equicontinuity

**Definition 6.5.1.** A TDS $(X, f)$ is *equicontinuous* if the family of iterates $\{f^n : n \geq 0\}$ is equicontinuous at every point: for all $\varepsilon > 0$ and $x \in X$, there exists $\delta > 0$ such that $d(x, y) < \delta$ implies $d(f^n(x), f^n(y)) < \varepsilon$ for all $n \geq 0$.

**Theorem 6.5.2.** An equicontinuous minimal TDS is conjugate to a group rotation on a compact group.

**Definition 6.5.3 (Distal and Proximal).** Two points $x, y \in X$ are *proximal* if $\inf_n d(f^n(x), f^n(y)) = 0$ (orbits can approach each other). They are *distal* if $\inf_n d(f^n(x), f^n(y)) > 0$ (orbits stay bounded away). A system is *distal* if all pairs of distinct points are distal.

Equicontinuous $\Rightarrow$ distal $\Rightarrow$ nonproximal-nontrivial.

**Remark 6.5.4.** Equicontinuous systems are the "opposite" of chaotic ones. They have no sensitive dependence. The dichotomy between equicontinuous and sensitive systems is Auslander-Yorke's theorem.

---

## 6.6 Topological Conjugacy

**Definition 6.6.1.** Two TDSs $(X, f)$ and $(Y, g)$ are *topologically conjugate* if there exists a homeomorphism $h: X \to Y$ with $h \circ f = g \circ h$. If $h$ is only continuous (not necessarily a homeomorphism), it is a *factor map* and $(Y, g)$ is a *topological factor* of $(X, f)$.

Topological conjugacy is the correct notion of isomorphism for TDSs: conjugate systems have identical orbit structures, periodic points, entropy, and all topological invariants.

**Example 6.6.2 (Conjugacy of Quadratic and Tent Maps).** The tent map $T: [0,1] \to [0,1]$, $T(x) = 1 - |2x-1|$, is topologically conjugate to $f_{-2}: [-2,2] \to [-2,2]$, $f_{-2}(x) = x^2 - 2$. The conjugacy is $h(x) = -2\cos(\pi x)$ (or $x = (1/\pi)\arccos(-y/2)$).

**Theorem 6.6.3.** Topological conjugacy preserves:
- Minimality and topological transitivity
- Topological entropy (see Chapter 22)
- The set of periods of periodic orbits (by period)
- Equicontinuity, distality

---

## 6.7 Sensitivity and Chaos

**Definition 6.7.1 (Devaney's Chaos).** A TDS $(X, f)$ is *chaotic in the sense of Devaney* if:
1. $f$ is topologically transitive
2. Periodic points are dense in $X$
3. $f$ has *sensitive dependence on initial conditions*: there exists $\delta > 0$ such that for any $x \in X$ and $\varepsilon > 0$, there exist $y \in B(x, \varepsilon)$ and $n \geq 0$ with $d(f^n(x), f^n(y)) > \delta$.

**Theorem 6.7.2 (Banks et al., 1992).** If $(X, f)$ is transitive and periodic points are dense, then $f$ is sensitive. Hence sensitive dependence is implied by the first two conditions.

*Proof idea:* Take any $x$ and $\varepsilon > 0$. Find a periodic orbit $\mathcal{O}(p)$ and consider the distance between $\mathcal{O}(p)$ and $\mathcal{O}(q)$ for another periodic orbit. Transitivity allows one to find nearby points that shadow different periodic orbits and hence diverge.

**Definition 6.7.3 (Li-Yorke Chaos).** $(X, f)$ is *Li-Yorke chaotic* if there exists an uncountable set $S \subseteq X$ such that for all $x \neq y$ in $S$:
$$\limsup_n d(f^n(x), f^n(y)) > 0 \quad \text{and} \quad \liminf_n d(f^n(x), f^n(y)) = 0.$$

Such a set $S$ is a *scrambled set*.

**Theorem 6.7.4 (Li-Yorke, 1975).** If $f: [a,b] \to [a,b]$ has a period-3 orbit, then $f$ has orbits of every period and is Li-Yorke chaotic. More generally: "Period 3 implies chaos."

---

## 6.8 Invariant Measures — Existence

### 6.8.1 The Krylov-Bogoliubov Theorem

**Definition 6.8.1.** A Borel probability measure $\mu$ on $X$ is *$f$-invariant* if $\mu(f^{-1}(A)) = \mu(A)$ for all Borel $A$, equivalently $\int \varphi \circ f\,d\mu = \int \varphi\,d\mu$ for all $\varphi \in C(X)$.

**Theorem 6.8.2 (Krylov-Bogoliubov).** Every continuous map $f: X \to X$ on a compact metrizable space $X$ has at least one invariant Borel probability measure.

*(proof)* Fix any $\mu_0$ (e.g., a Dirac mass $\delta_{x_0}$). Consider the Cesàro averages $\mu_N = \frac{1}{N}\sum_{n=0}^{N-1} f^n_* \mu_0$. By the Prokhorov / Arzelà-Ascoli argument, the sequence $(\mu_N)$ is tight (since $X$ is compact), so a subsequence converges weakly to some $\mu$. One checks: for $\varphi \in C(X)$:
$$\int \varphi \circ f\,d\mu_N - \int \varphi\,d\mu_N = \frac{1}{N}(\int \varphi \circ f^N\,d\mu_0 - \int \varphi\,d\mu_0) \to 0.$$
Taking the limit, $\int \varphi \circ f\,d\mu = \int \varphi\,d\mu$, so $\mu$ is $f$-invariant.

**Remark 6.8.3.** The Krylov-Bogoliubov theorem guarantees existence but not uniqueness. Multiple invariant measures can coexist. A system with a *unique* invariant measure is called *uniquely ergodic*.

### 6.8.2 Unique Ergodicity

**Definition 6.8.4.** $f: X \to X$ is *uniquely ergodic* if it has a unique invariant probability measure.

**Theorem 6.8.5 (Weyl, Oxtoby).** $f$ is uniquely ergodic if and only if for every $\varphi \in C(X)$, the averages $\frac{1}{N}\sum_{n=0}^{N-1} \varphi(f^n(x))$ converge uniformly in $x \in X$ to the constant $\int \varphi\,d\mu$.

**Example 6.8.6.** Every irrational rotation $R_\alpha$ is uniquely ergodic (with Lebesgue measure as the unique invariant measure). The convergence $\frac{1}{N}\sum_{n=0}^{N-1} e^{2\pi i k (x + n\alpha)} \to 0$ for $k \neq 0$ (Weyl's theorem) establishes equidistribution.

---

## 6.9 The Ellis Semigroup

**Definition 6.9.1.** For a TDS $(X, f)$, the *Ellis semigroup* $E(X, f)$ is the closure of $\{f^n : n \in {\mathbb N}\}$ in $X^X$ (with the product topology), with the operation of composition.

$E(X, f)$ is a compact Hausdorff semigroup. It encodes the asymptotic behavior of all orbits.

**Theorem 6.9.2.** $(X, f)$ is equicontinuous iff $E(X, f)$ is a group of homeomorphisms (equivalently, is a compact group acting on $X$ continuously).

The Ellis semigroup is a powerful algebraic tool for studying recurrence and the structure of topological dynamical systems beyond what orbit analysis alone provides.

---

## Exercises

**Exercise 6.1.** Let $f: X \to X$ be minimal. Show that $f$ has no proper closed invariant subsets. Conversely, show that if $X$ is compact and every closed invariant subset equals $X$ or $\emptyset$, then $f$ is minimal.

**Exercise 6.2.** Classify the omega-limit sets of the logistic map $f_4(x) = 4x(1-x)$ on $[0,1]$. (*Hint:* $f_4$ is conjugate to the tent map via $x = \sin^2(\pi\theta/2)$.)

**Exercise 6.3.** Prove that the doubling map $f: x \mapsto 2x \pmod{1}$ on $[0,1]$ is topologically mixing but not minimal.

**Exercise 6.4.** Let $R_\alpha: {\mathbb T} \to {\mathbb T}$ be irrational rotation. Show $R_\alpha$ is minimal using the following: if $F \subseteq {\mathbb T}$ is closed and $R_\alpha$-invariant, then $F$ is closed under translation by $\alpha$, hence by $n\alpha$ for all $n$, hence $F$ must be all of ${\mathbb T}$ (density of $\{n\alpha \pmod{1}\}$).

**Exercise 6.5.** Prove the Poincaré Recurrence Theorem for topological systems: show that if $f: X \to X$ is a homeomorphism of a compact metric space, every open set $U$ satisfies $f^n(U) \cap U \neq \emptyset$ for some $n \geq 1$.

**Exercise 6.6.** Let $f: X \to X$ be uniquely ergodic with invariant measure $\mu$. For $\mu$-a.e. $x$, the orbit of $x$ equidistributes: for every continuous $\varphi$, $\frac{1}{N}\sum_{n<N} \varphi(f^n(x)) \to \int \varphi\,d\mu$. By unique ergodicity, this convergence is *uniform* in $x$. Verify this for the rotation $R_\alpha$ using Fourier analysis.

**Exercise 6.7.** (Li-Yorke) The map $f(x) = 4x(1-x)$ on $[0,1]$ has a period-3 orbit. (a) Find it numerically. (b) Conclude by the Li-Yorke theorem that $f$ is chaotic in the Li-Yorke sense.

**Exercise 6.8.** (Research Connection) The Collatz map $T$ on ${\mathbb N}$ does not have a compact phase space, so the Krylov-Bogoliubov theorem does not directly apply. Describe the obstacles to finding an invariant probability measure for $T$ with respect to counting measure. What would such a measure look like?

---

## Chapter Notes

The foundations of topological dynamics are in Auslander's *Minimal Flows and Their Extensions*, Ellis's *Lectures on Topological Dynamics*, and the accessible introduction in Walters' *An Introduction to Ergodic Theory* (Chapter 1). For the modern perspective, Glasner's *Proximal Flows* and Auslander-Glasner's work are essential.

Devaney's definition of chaos (Section 6.7) and the theorem of Banks et al. that the third condition is redundant are from the 1990s and remain standard references. For Li-Yorke chaos, the original paper "Period Three Implies Chaos" (Li-Yorke, 1975, *American Mathematical Monthly*) is the right starting point.

The Krylov-Bogoliubov theorem (Section 6.8) is where topological and measure-theoretic dynamics first meet. In the next chapter, we add the measure and study what invariant measures reveal about long-term averages.
