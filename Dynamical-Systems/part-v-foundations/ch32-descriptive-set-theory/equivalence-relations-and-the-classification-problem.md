# 32.3 Equivalence Relations and the Classification Problem

Here is the central question of this chapter, stated precisely: given a class of dynamical systems, when can we classify them up to isomorphism by Borel invariants?

"Classify by Borel invariants" means: find a Borel-measurable function $\phi$ from the space of systems to some standard space (the reals, the natural numbers, countable graphs, etc.) such that two systems are isomorphic if and only if $\phi$ assigns them the same value. This is the mathematical version of "find a complete invariant."

The theory of Borel equivalence relations, developed by Kechris, Louveau, Hjorth, and others in the 1990s and 2000s, gives a systematic framework for asking when this is possible.

## 32.3.1 Borel Equivalence Relations

**Definition 32.3.1.** A *Borel equivalence relation* on a Polish space $X$ is an equivalence relation $E \subseteq X \times X$ that is a Borel subset of $X \times X$.

Isomorphism of dynamical systems is an equivalence relation on the Polish space of dynamical systems. The key question is: is it a Borel equivalence relation? And if so, where does it sit in the complexity hierarchy?

**The Classification Problem:** Given a class of dynamical systems, can we classify them up to isomorphism? This requires finding a *complete invariant* — a Borel map $\phi: X \to Y$ such that $x \sim y \iff \phi(x) = \phi(y)$.

**Definition 32.3.2 (Borel Reducibility).** $E$ is *Borel reducible* to $F$ ($E \leq_B F$) if there is a Borel map $f: X \to Y$ such that $x \mathrel{E} y \iff f(x) \mathrel{F} f(y)$. Intuitively, classifying by $E$ is "no harder" than classifying by $F$.

Borel reducibility gives a partial order on the complexity of equivalence relations. If $E \leq_B F$, then any classification scheme for $F$ can be "pulled back" to a classification scheme for $E$. If $E \not\leq_B F$, then classifying by $E$ is genuinely harder.

**The Hierarchy:**
$$\text{id}(\mathbb{R}) \leq_B E_0 \leq_B E_\infty \leq_B \ldots$$

where $\text{id}(\mathbb{R})$: real numbers up to equality; $E_0$: eventually equal binary sequences.

$\text{id}(\mathbb{R})$ is the simplest nontrivial equivalence relation: equality of real numbers. Any invariant that is a real number (like entropy) is a Borel reduction of the isomorphism relation to $\text{id}(\mathbb{R})$. $E_0$ is the next level: two binary sequences are $E_0$-equivalent if they agree from some point on. A system is classifiable by $E_0$ if and only if it has countably many isomorphism classes, or some similar countable structure.

## 32.3.2 Turbulence and Non-Classification

The obstruction to smooth classification is *turbulence*, introduced by Greg Hjorth in 2000.

**Definition 32.3.3 (Hjorth, 2000).** A continuous action of a Polish group $G$ on a Polish space $X$ is *turbulent* if every orbit is dense in its local closure and every orbit is meager in this local closure. Informally: the orbits are "wildly irregular."

Think of turbulence as a topological obstruction. In a turbulent action, the orbits are simultaneously dense everywhere locally (you can approximate any point from any orbit) and meager everywhere locally (the orbit itself takes up "no room"). This incompatibility prevents any Borel function from distinguishing the orbits.

**Theorem 32.3.4 (Hjorth's Turbulence Theorem).** If a continuous Polish group action is turbulent, then the orbit equivalence relation is not Borel reducible to any orbit equivalence relation induced by a continuous action of $S_\infty$ (the Polish group of permutations). In particular, there is no "smooth" (Borel) classification.

$S_\infty$ is the group of permutations of $\mathbb{N}$ — its orbit equivalence relations are exactly the "classification by countable structures" (graphs, groups, orders, etc.). Hjorth's theorem says: turbulent group actions produce equivalence relations that are too complex to be classified by any countable structure, however elaborate.

**Application 32.3.5 (Unclassifiability of Measure-Preserving Systems).** The isomorphism relation for ergodic measure-preserving transformations:
- *Cannot* be classified by countable structures (Foreman-Rudolph-Weiss, 2011)
- The isomorphism relation is complete analytic ($\Sigma^1_1$-complete)
- There is no Borel function $\phi$ assigning complete invariants to ergodic MPTs

This is a foundational result: ergodic measure-preserving systems are inherently unclassifiable.

The Foreman-Rudolph-Weiss theorem (2011) is one of the deepest results in ergodic theory. It says not just that we haven't found complete invariants — it says that complete Borel invariants *cannot exist*. The isomorphism problem for ergodic systems is more complex than any classification by countable structures.

The proof goes via Hjorth's turbulence theory: the conjugacy action of $\text{Aut}(X, \mu)$ on $\text{MPT}([0,1])$ is turbulent. Since turbulence obstructs Borel reducibility to $S_\infty$-orbits, and classification by countable structures corresponds to exactly such orbits, the isomorphism problem is unclassifiable.

This result is philosophically important. It means that the search for "complete invariants" for ergodic systems — the program of Ornstein theory — cannot be continued beyond certain limits. Bernoulli shifts are classified by entropy. Some broader classes are classified by additional isomorphism invariants. But for the full class of ergodic systems, no finite or countable collection of invariants can do the job. The systems are simply too varied.
