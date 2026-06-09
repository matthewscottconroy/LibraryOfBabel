# Chapter 9 — Hyperbolic Dynamics

> *Hyperbolic systems are the richest, best-understood class of chaotic systems. The theory is essentially complete — and it was built in the 1960s and 70s by Smale, Anosov, Sinai, Ruelle, Bowen, and their contemporaries.*

**Prerequisites:** Chapters 4, 7, 8 (flows, ergodic theory, stability and Lyapunov exponents).

**What this chapter builds:** The horseshoe construction (the prototype of chaos); Anosov diffeomorphisms and their symbolic coding; the shadowing lemma (connecting numerics to theory); Markov partitions; Axiom A attractors and their SRB measures; and structural stability.

---

## 9.1 The Smale Horseshoe

### 9.1.1 Construction

**Construction 9.1.1 (Smale Horseshoe).** Start with the unit square $Q = [0,1]^2$. The horseshoe map $f: Q \to {\mathbb R}^2$ is constructed as follows:
1. Compress $Q$ vertically by factor $\lambda < 1/2$ and stretch horizontally by $\mu > 2$.
2. Bend the resulting strip into a horseshoe shape and place it so it intersects $Q$ in two vertical strips $V_0$ and $V_1$ (the preimages of the two "legs" of the horseshoe).

More precisely: $f$ maps two horizontal strips $H_0, H_1 \subset Q$ to two vertical strips $V_0, V_1 \subset Q$ via:
$$f(H_0) = V_0, \quad f(H_1) = V_1,$$
with expansion by $\mu$ in the horizontal direction and contraction by $\lambda$ in the vertical.

**Definition 9.1.2.** The *invariant Cantor set* of the horseshoe is:
$$\Lambda = \bigcap_{n \in {\mathbb Z}} f^n(Q).$$

This is the set of points whose entire orbit stays in $Q$ under both forward and backward iteration.

### 9.1.2 Symbolic Description

**Theorem 9.1.3.** The invariant set $\Lambda$ of the horseshoe is homeomorphic to the full two-shift $\{0,1\}^{\mathbb Z}$ via the coding map $\pi: \Lambda \to \{0,1\}^{\mathbb Z}$, $\pi(x)_n = i$ iff $f^n(x) \in H_i$.

The map $\pi$ conjugates $f|_\Lambda$ to the shift $\sigma$ on $\{0,1\}^{\mathbb Z}$.

**Consequences:**
- $\Lambda$ is a Cantor set (compact, perfect, totally disconnected)
- $f|_\Lambda$ has a dense orbit (coded by a sequence visiting all words)
- $f|_\Lambda$ has a dense set of periodic orbits (coded by periodic sequences)
- $f|_\Lambda$ has $2^n$ periodic orbits of period $n$ (one for each binary string)
- The topological entropy of $f|_\Lambda$ is $\log 2$

**Remark 9.1.4.** The horseshoe shows that simple geometric operations (stretch, fold) produce extraordinary complexity in the orbit structure. This construction motivated Smale's abstract theory of hyperbolic sets.

---

## 9.2 Hyperbolic Sets

**Definition 9.2.1.** Let $f: M \to M$ be a $C^1$ diffeomorphism and $\Lambda \subseteq M$ a compact $f$-invariant set. $\Lambda$ is a *hyperbolic set* if there exists a $Df$-invariant splitting $T_xM = E^s(x) \oplus E^u(x)$ for each $x \in \Lambda$, and constants $C > 0$, $0 < \lambda < 1$ such that for all $n \geq 0$ and $x \in \Lambda$:
$$\|Df^n(x) v\| \leq C\lambda^n \|v\| \quad \text{for } v \in E^s(x)$$
$$\|Df^{-n}(x) v\| \leq C\lambda^n \|v\| \quad \text{for } v \in E^u(x).$$

The stable/unstable bundles $E^s, E^u$ vary continuously and are preserved by $Df$.

**Examples:**
- The horseshoe $\Lambda$ is a hyperbolic set.
- Repelling fixed points are hyperbolic sets (with $E^u = T_xM$, $E^s = 0$).
- Uniformly expanding maps (all eigenvalues $> 1$) have hyperbolic invariant sets.
- The invariant set of an Anosov diffeomorphism is $\Lambda = M$ (the whole manifold).

---

## 9.3 Anosov Diffeomorphisms

**Definition 9.3.1.** A $C^1$ diffeomorphism $f: M \to M$ of a compact manifold is an *Anosov diffeomorphism* if $M$ itself is a hyperbolic set: for all $x \in M$, $T_xM = E^s(x) \oplus E^u(x)$ with uniform expansion in $E^u$ and contraction in $E^s$.

**Examples 9.3.2.**
- *Linear toral automorphisms*: $f_A: {\mathbb T}^n \to {\mathbb T}^n$ for $A \in GL(n, {\mathbb Z})$ with no eigenvalue on the unit circle. The splitting is constant: $E^s = $ span of eigenvectors with $|\lambda| < 1$, $E^u = $ span with $|\lambda| > 1$.
- The Arnold cat map $A = \begin{pmatrix} 2 & 1 \\ 1 & 1 \end{pmatrix}$ on ${\mathbb T}^2$ with eigenvalues $\lambda_+ = (3+\sqrt{5})/2 > 1$ and $\lambda_- = (3-\sqrt{5})/2 < 1$.
- It is unknown whether Anosov diffeomorphisms exist on manifolds other than infranilmanifolds (a major open problem).

**Theorem 9.3.3 (Anosov).** Every Anosov diffeomorphism is topologically transitive (has a dense orbit) and has a dense set of periodic points.

**Theorem 9.3.4.** For an Anosov diffeomorphism $f$:
1. The stable and unstable foliations $\mathcal{W}^s$, $\mathcal{W}^u$ are well-defined, $f$-invariant, and continuous (though generally not smooth as foliations).
2. $f$ is *structurally stable*: any $C^1$-perturbation $g$ of $f$ is topologically conjugate to $f$.

---

## 9.4 The Shadowing Lemma

The shadowing lemma is the rigorous bridge between numerical simulations and theoretical analysis. It says: approximate orbits (with small errors) are approximated by true orbits.

**Definition 9.4.1.** A *$\delta$-pseudo-orbit* of $f$ is a sequence $(x_n)_{n \in {\mathbb Z}}$ with $d(f(x_n), x_{n+1}) \leq \delta$ for all $n$.

**Theorem 9.4.2 (Shadowing Lemma / Anosov-Bowen).** Let $\Lambda$ be a hyperbolic set of $f$. For every $\varepsilon > 0$ there exists $\delta > 0$ such that: every $\delta$-pseudo-orbit $(x_n)$ in $\Lambda$ is $\varepsilon$-shadowed by a true orbit: there exists $y \in M$ with $d(f^n(y), x_n) \leq \varepsilon$ for all $n$.

If $\Lambda = M$ (Anosov), the true orbit is unique and the shadowing point depends continuously on the pseudo-orbit.

*(proof sketch)* The proof reduces to finding a fixed point of a certain operator in the space of sequences $(y_n)$ with the property that $y_{n+1} = f(y_n) + \text{small error}$. The hyperbolic splitting ensures this operator is a contraction.

**Numerical Consequence:** Computer simulations of hyperbolic systems produce pseudo-orbits (due to floating-point errors $\approx 10^{-16}$). The shadowing lemma guarantees these pseudo-orbits are close to true orbits — so numerical simulations of Anosov systems are *valid*.

---

## 9.5 Markov Partitions

**Definition 9.5.1.** A *Markov partition* of a hyperbolic set $\Lambda$ is a finite cover $\mathcal{R} = \{R_1, \ldots, R_k\}$ by "rectangles" (sets that are products of stable and unstable manifold pieces) with:
1. $R_i = \overline{\text{int}(R_i)}$ and $\text{int}(R_i) \cap \text{int}(R_j) = \emptyset$ for $i \neq j$
2. The Markov property: if $x \in \text{int}(R_i)$ and $f(x) \in \text{int}(R_j)$, then $f(W^u_{\text{loc}}(x) \cap R_i) \supseteq W^u_{\text{loc}}(f(x)) \cap R_j$

**Theorem 9.5.2 (Sinai, Bowen).** Every Anosov diffeomorphism (and every hyperbolic attractor) has a Markov partition.

**The Transition Matrix:** Define $A_{ij} = 1$ if $f(\text{int}(R_i)) \cap \text{int}(R_j) \neq \emptyset$, else $0$. The associated *subshift of finite type* $\Sigma_A \subseteq \{1,\ldots,k\}^{\mathbb Z}$ codes the dynamics: the coding map $\pi: \Lambda \to \Sigma_A$ defined by $\pi(x)_n = i$ iff $f^n(x) \in R_i$ is almost surjective (bijective on a residual set).

**Theorem 9.5.3.** For an Anosov diffeomorphism $f$ with Markov partition $\mathcal{R}$ and transition matrix $A$:
$$h_{\text{top}}(f) = \log \lambda_{\text{PF}}(A)$$
where $\lambda_{\text{PF}}(A)$ is the Perron-Frobenius eigenvalue of $A$.

---

## 9.6 SRB Measures

Not all invariant measures for a hyperbolic system are "physically relevant." The SRB measures are the ones seen by Lebesgue-typical initial conditions.

**Definition 9.6.1.** For a diffeomorphism $f$ with a hyperbolic attractor $\Lambda$, a measure $\mu$ is an *SRB measure* (Sinai-Ruelle-Bowen, or *physical measure*) if for Lebesgue-a.e. $x$ in the basin of attraction:
$$\frac{1}{N} \sum_{n=0}^{N-1} \varphi(f^n(x)) \to \int \varphi\,d\mu \quad \text{for all continuous } \varphi.$$

**Theorem 9.6.2 (Sinai-Ruelle-Bowen).** Every Axiom A attractor has a unique SRB measure. The SRB measure:
- Is ergodic
- Is absolutely continuous on unstable manifolds (but may be singular w.r.t. Lebesgue)
- Satisfies Pesin's formula: $h_\mu(f) = \sum_{\lambda_i > 0} \lambda_i$

*SRB measure characterization:* $\mu$ is SRB iff it satisfies Pesin's formula AND it has absolutely continuous conditional measures on unstable manifolds.

**Example 9.6.3.** For linear toral automorphisms: the SRB measure is Lebesgue measure (since the system preserves Lebesgue measure and is ergodic, Lebesgue is both SRB and the unique ergodic measure).

---

## 9.7 Axiom A and Structural Stability

**Definition 9.7.1 (Smale's Axiom A).** A diffeomorphism $f: M \to M$ satisfies *Axiom A* if:
1. The nonwandering set $\Omega(f)$ is hyperbolic
2. Periodic points are dense in $\Omega(f)$

**Theorem 9.7.2 (Smale's Spectral Decomposition).** For an Axiom A diffeomorphism, $\Omega(f) = \Lambda_1 \cup \cdots \cup \Lambda_k$ where each $\Lambda_i$ is a closed, $f$-invariant, topologically transitive set (a *basic set*). The basic sets are ordered: there is no cycle among them.

**Definition 9.7.3.** $f$ is *structurally stable* if every $g$ sufficiently $C^1$-close to $f$ is topologically conjugate to $f$.

**Theorem 9.7.4 (Robbin-Robinson).** Axiom A + strong transversality (stable and unstable manifolds intersect transversally) implies structural stability.

**Theorem 9.7.5 (Mañé).** Structural stability implies Axiom A + strong transversality. (So Axiom A + ST $\Leftrightarrow$ structural stability.)

---

## 9.8 Homoclinic Orbits and the Genesis of Chaos

**Definition 9.8.1.** Let $p$ be a hyperbolic fixed point. A *homoclinic orbit* is an orbit in $W^s(p) \cap W^u(p)$ (other than $p$ itself). A *transverse homoclinic orbit* has a transverse intersection of $W^s$ and $W^u$.

**Theorem 9.8.2 (Smale-Birkhoff Homoclinic Theorem).** If $f$ has a transverse homoclinic point, then some iterate $f^n$ contains a horseshoe in its dynamics near the homoclinic orbit. In particular, $f$ has periodic orbits of every large period and positive topological entropy.

*(proof sketch)* The transverse intersection of $W^s$ and $W^u$ forces the manifolds to intersect again and again (the Lambda Lemma / Inclination Lemma), creating a geometric situation equivalent to Smale's horseshoe construction.

**Historical Note.** Poincaré discovered homoclinic orbits in the 1890s while studying the three-body problem. He recognized they implied "extreme complexity" but could not formalize it. Smale's horseshoe (1960s) finally captured this complexity mathematically.

---

## 9.9 Partial Hyperbolicity

Not all interesting systems are fully hyperbolic. *Partial hyperbolicity* relaxes the requirement.

**Definition 9.9.1.** $f: M \to M$ is *partially hyperbolic* if there exists a $Df$-invariant splitting $TM = E^s \oplus E^c \oplus E^u$ (stable, center, unstable) with uniform expansion in $E^u$, contraction in $E^s$, and the center $E^c$ being "dominated" — weaker contraction/expansion than the extreme bundles.

**Examples:** Frame flows, geodesic flows on non-constant curvature manifolds, certain algebraic systems.

**Open Problems:** Does partial hyperbolicity imply ergodicity? (Pugh-Shub conjecture, partially resolved.) Does every center-bunched partially hyperbolic system have finitely many ergodic measures?

---

## Exercises

**Exercise 9.1.** Verify that the invariant Cantor set of the horseshoe map has Hausdorff dimension $< 2$. (Estimate the dimension using the contraction and expansion rates $\lambda, \mu$.)

**Exercise 9.2.** (Shadowing) For the doubling map $f(x) = 2x \pmod 1$, show that any $\delta$-pseudo-orbit is $\delta/(2-1)$-shadowed by a true orbit. (*Hint:* Solve the "shadow" equation $2x_{n+1} - x_{n+2} = 2e_n$ where $e_n$ are errors.)

**Exercise 9.3.** Compute a Markov partition for the Arnold cat map $f_A$ on ${\mathbb T}^2$ with $A = \begin{pmatrix} 2 & 1 \\ 1 & 1\end{pmatrix}$. (*Hint:* The partition consists of two rectangles aligned with the stable/unstable eigendirections of $A$.) Write down the transition matrix and compute the topological entropy.

**Exercise 9.4.** For the baker's map $B: [0,1]^2 \to [0,1]^2$ defined by $B(x,y) = (2x, y/2)$ for $x < 1/2$ and $B(x,y) = (2x-1, (y+1)/2)$ for $x \geq 1/2$: show it is Anosov, find its stable/unstable foliations, construct a Markov partition, and compute its entropy.

**Exercise 9.5.** (Structural Stability) Let $f: {\mathbb T}^2 \to {\mathbb T}^2$ be the Arnold cat map. Suppose $g$ is a small $C^1$-perturbation. Show that the periodic orbit structure of $g$ is the same as that of $f$ (same number of periodic orbits of each period) using structural stability.

**Exercise 9.6.** (Research Connection) The logistic map $f_\mu(x) = \mu x(1-x)$ for $\mu = 4$ is topologically conjugate to the tent map on $[0,1]$. The tent map has a Markov partition into $\{[0,1/2], [1/2,1]\}$. Construct the symbolic coding and compute the entropy. Is $f_4$ an Anosov map on a compact manifold? (What goes wrong?)

---

## Chapter Notes

Smale's foundational paper *Differentiable Dynamical Systems* (1967) is required reading. It introduces Axiom A, structural stability, the spectral decomposition theorem, and provides the theoretical framework for the entire subject.

The theory of Markov partitions (Sinai 1968, Bowen 1975) and SRB measures (Sinai 1972, Ruelle 1976, Bowen-Ruelle 1975) are in:
- Bowen, *Equilibrium States and the Ergodic Theory of Anosov Diffeomorphisms* (1975) — short and essential
- Katok-Hasselblatt, *Introduction to the Modern Theory of Dynamical Systems* — the comprehensive reference

The shadowing lemma is in Bowen's *Symbolic Dynamics for Hyperbolic Flows* and Pilyugin's *Shadowing in Dynamical Systems*.

For Smale's horseshoe and homoclinic orbits, see Guckenheimer-Holmes' *Nonlinear Oscillations, Dynamical Systems, and Bifurcations of Vector Fields* (Chapters 3, 5).
