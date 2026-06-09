# Chapter 32 — Descriptive Set Theory and Dynamical Systems

> *Borel sets, analytic sets, and the hierarchy of definable sets form the foundation of modern analysis. The Borel hierarchy classifies the complexity of dynamical properties. Turbulence is the obstruction to smooth classification — some equivalence relations are so complex that no assignment of invariants can tame them.*

**Prerequisites:** Chapter 1 (metric spaces, Baire category), Chapter 2 (measure theory), Chapter 6 (topological dynamics).

---

## 32.1 The Borel Hierarchy

### 32.1.1 Borel Sets and Their Complexity

**Definition 32.1.1.** In a Polish space (completely metrizable separable topological space) $X$:
- $\Sigma^0_1$: open sets
- $\Pi^0_1$: closed sets (complements of open sets)
- $\Sigma^0_{\alpha+1}$: countable unions of $\Pi^0_\alpha$ sets
- $\Pi^0_{\alpha+1}$: countable intersections of $\Sigma^0_\alpha$ sets
- $\Delta^0_\alpha = \Sigma^0_\alpha \cap \Pi^0_\alpha$: the ambiguous class

The *Borel sets* are $\bigcup_{\alpha < \omega_1} \Sigma^0_\alpha$ (the union over all countable ordinals).

**Example 32.1.2.**
- $F_\sigma$ sets: countable unions of closed sets = $\Sigma^0_2$
- $G_\delta$ sets: countable intersections of open sets = $\Pi^0_2$
- $F_{\sigma\delta}$ = $\Pi^0_3$, $G_{\delta\sigma}$ = $\Sigma^0_3$, etc.

**Theorem 32.1.3 (Hierarchy is Strict).** Each class in the Borel hierarchy is strictly larger than the previous. No Borel set is missing from the hierarchy; every Borel set has a well-defined Borel rank.

### 32.1.2 Analytic and Coanalytic Sets

**Definition 32.1.4.** A set $A \subseteq X$ is *analytic* ($\Sigma^1_1$) if it is the continuous image of a Borel set: $A = f(B)$ for $f$ continuous and $B$ Borel.

Equivalently, $A$ is analytic iff it is the projection of a Borel set in $X \times Y$.

**Definition 32.1.5.** A set is *coanalytic* ($\Pi^1_1$) if its complement is analytic.

**Theorem 32.1.6 (Luzin Separation Theorem).** Two disjoint analytic sets can be separated by a Borel set.

**Theorem 32.1.7 (Luzin-Suslin).** Every Borel set is analytic. Not every analytic set is Borel (there exist $\Sigma^1_1$ sets that are not Borel).

---

## 32.2 Polish Spaces in Dynamics

**Theorem 32.2.1 (The Space of Dynamical Systems is Polish).** The following spaces are Polish:
- $\text{MPT}([0,1], \text{Lebesgue})$: measure-preserving transformations, with the weak topology
- $\text{Homeo}(K)$: homeomorphisms of a compact metric space $K$, with the $\sup$ topology
- $\text{Aut}(X, \mu)$: measure-preserving automorphisms, with the weak operator topology

This means: the space of all dynamical systems is itself a dynamical system (acted on by conjugacy).

**Theorem 32.2.2 (Generic Dynamical Properties).** In $\text{MPT}([0,1])$ (the space of all measure-preserving transformations with the weak topology), the generic (comeager) transformation:
1. Is ergodic (ergodic MPTs form a dense $G_\delta$)
2. Is weakly mixing but not strongly mixing (strongly mixing is meager)
3. Has zero entropy (positive entropy MPTs are meager)

**Remark 32.2.3.** "Generic" in Baire category sense ≠ "almost all" in measure-theoretic sense. The generic transformation is weakly mixing, but most natural examples (Bernoulli shifts, rotations) are in the non-generic classes.

---

## 32.3 Equivalence Relations and the Classification Problem

### 32.3.1 Borel Equivalence Relations

**Definition 32.3.1.** A *Borel equivalence relation* on a Polish space $X$ is an equivalence relation $E \subseteq X \times X$ that is a Borel subset of $X \times X$.

**The Classification Problem:** Given a class of dynamical systems, can we classify them up to isomorphism? This requires finding a *complete invariant* — a Borel map $\phi: X \to Y$ such that $x \sim y \iff \phi(x) = \phi(y)$.

**Definition 32.3.2 (Borel Reducibility).** $E$ is *Borel reducible* to $F$ ($E \leq_B F$) if there is a Borel map $f: X \to Y$ such that $x \mathrel{E} y \iff f(x) \mathrel{F} f(y)$. Intuitively, classifying by $E$ is "no harder" than classifying by $F$.

**The Hierarchy:**
$$\text{id}({\mathbb R}) \leq_B E_0 \leq_B E_\infty \leq_B \ldots$$

where $\text{id}({\mathbb R})$: real numbers up to equality; $E_0$: eventually equal binary sequences.

### 32.3.2 Turbulence and Non-Classification

**Definition 32.3.3 (Hjorth, 2000).** A continuous action of a Polish group $G$ on a Polish space $X$ is *turbulent* if every orbit is dense in its local closure and every orbit is meager in this local closure. Informally: the orbits are "wildly irregular."

**Theorem 32.3.4 (Hjorth's Turbulence Theorem).** If a continuous Polish group action is turbulent, then the orbit equivalence relation is not Borel reducible to any orbit equivalence relation induced by a continuous action of $S_\infty$ (the Polish group of permutations). In particular, there is no "smooth" (Borel) classification.

**Application 32.3.5 (Unclassifiability of Measure-Preserving Systems).** The isomorphism relation for ergodic measure-preserving transformations:
- *Cannot* be classified by countable structures (Foreman-Rudolph-Weiss, 2011)
- The isomorphism relation is complete analytic ($\Sigma^1_1$-complete)
- There is no Borel function $\phi$ assigning complete invariants to ergodic MPTs

This is a foundational result: ergodic measure-preserving systems are inherently unclassifiable.

---

## 32.4 Entropy as a Borel Invariant

**Theorem 32.4.1.** The KS entropy $h: \text{Erg}(X,\mu) \to [0,\infty]$ is a Borel function on the space of ergodic MPTs.

**Theorem 32.4.2 (Ornstein's Theorem — Classification by Entropy for Bernoulli Shifts).** The isomorphism relation restricted to Bernoulli shifts is smooth (classifiable): two Bernoulli shifts are isomorphic iff they have the same entropy. The invariant $h$ is a *complete invariant* for Bernoulli shifts.

**Contrast:** For all ergodic MPTs, isomorphism is not classifiable. For Bernoulli shifts (a special subclass), it is. This illustrates the power of Ornstein theory (Chapter 7) and the limitations of general classification.

---

## 32.5 Definability and the Baire Property

**Definition 32.5.1.** A set $A \subseteq X$ has the *Baire property* if there is an open set $U$ such that $A \triangle U$ is meager (i.e., $A$ and $U$ differ by a set of first category).

**Theorem 32.5.2 (Every Analytic Set has the Baire Property).** Every $\Sigma^1_1$ set has the Baire property (and is Lebesgue measurable). This is the Luzin-Sierpiński theorem.

**Theorem 32.5.3 (Regularity and Descriptive Set Theory).** Under the axiom of determinacy (AD), *every* subset of a Polish space has the Baire property and is Lebesgue measurable. Under the axiom of choice (AC), there exist Bernstein sets and Vitali sets without these properties.

**Application to Dynamics:** Topological properties of dynamical systems that are definable in the Borel hierarchy (e.g., "has a fixed point," "is minimal," "has positive entropy") are well-behaved. Pathological dynamical systems (e.g., those arising from AC) are not definable and cannot arise in practice.

---

## Exercises

**Exercise 32.1.** Show that the set of normal numbers is a $G_\delta$-set in $[0,1]$ (hence $\Pi^0_2$). Is it $\Pi^0_1$ (closed)? Why or why not?

**Exercise 32.2.** Show that the set of ergodic MPTs is a $G_\delta$ in the space of all MPTs with the weak topology. What does this say about the "genericity" of ergodicity?

**Exercise 32.3.** (Borel Reducibility) Show that equality of real numbers $\text{id}({\mathbb R})$ is Borel reducible to eventual equality of sequences $E_0$ (binary sequences equal from some point on). Find the explicit reduction map.

**Exercise 32.4.** (Research) The Foreman-Rudolph-Weiss theorem says ergodic MPTs cannot be classified by countable structures. Look up what "countable structure" means in this context. What does this say about the existence of complete invariants for ergodic systems beyond entropy?

---

## Chapter Notes

The standard reference is Kechris's *Classical Descriptive Set Theory* (Springer, 1995). Gao's *Invariant Descriptive Set Theory* (CRC Press, 2009) covers the Borel equivalence relation theory and classification problems.

Hjorth's turbulence theory is in *Classification and Orbit Equivalence Relations* (AMS, 2000). The Foreman-Rudolph-Weiss unclassifiability theorem is in *The conjugacy problem in ergodic theory* (Annals of Math., 2011).

The connection between descriptive set theory and ergodic theory is surveyed in Kechris's *Global Aspects of Ergodic Group Actions* (AMS, 2010). Foreman and Weiss have a survey *An Anti-Classification Theorem for Ergodic Measure Preserving Transformations* (J. European Math. Soc., 2004).
