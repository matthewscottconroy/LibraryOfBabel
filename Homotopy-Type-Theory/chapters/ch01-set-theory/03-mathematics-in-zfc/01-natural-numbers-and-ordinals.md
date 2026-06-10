# 3.1 Building Mathematics in ZFC: Natural Numbers and Ordinals

## The Von Neumann Encoding

To do mathematics in ZFC, we need to represent mathematical objects as sets. The first and most fundamental: natural numbers.

The *von Neumann encoding* represents each natural number as the set of all smaller natural numbers:
$$0 = \emptyset, \quad 1 = \{0\} = \{\emptyset\}, \quad 2 = \{0, 1\} = \{\emptyset, \{\emptyset\}\}, \quad n+1 = n \cup \{n\}$$

So:
- $0 = \emptyset$ (zero members — fits "zero is nothing")
- $1 = \{\emptyset\}$ (one member, namely $\emptyset$)
- $2 = \{\emptyset, \{\emptyset\}\}$ (two members: 0 and 1)
- $3 = \{\emptyset, \{\emptyset\}, \{\emptyset, \{\emptyset\}\}\}$ (three members: 0, 1, and 2)

A key property: $m < n$ iff $m \in n$. The membership relation $\in$ *is* the order relation on natural numbers in this encoding.

## The Axiom of Infinity

The axioms so far (empty set, pairing, union, power set, separation, replacement, foundation) allow building any finite set, but cannot produce an infinite set. We need another axiom:

**Axiom of Infinity:**
$$\exists I\, [\emptyset \in I \wedge \forall x \in I,\, (x \cup \{x\}) \in I]$$

There exists an *inductive set*: a set containing $\emptyset$ and closed under the successor operation $x \mapsto x \cup \{x\}$.

Any inductive set contains $0, 1, 2, 3, \ldots$ as elements. Define:
$$\mathbb{N} = \bigcap \{I \mid I \text{ is inductive}\}$$

(The intersection exists by Separation applied to any inductive set.) $\mathbb{N}$ is the *smallest* inductive set — the least element of the "lattice" of inductive sets.

**Theorem (Peano Axioms).** With these definitions:
1. $0 \in \mathbb{N}$
2. If $n \in \mathbb{N}$, then $S(n) = n \cup \{n\} \in \mathbb{N}$
3. $S(n) \neq 0$ for all $n \in \mathbb{N}$
4. $S$ is injective: $S(m) = S(n) \Rightarrow m = n$
5. Induction: if $P(0)$ and $\forall n \in \mathbb{N}, P(n) \to P(S(n))$, then $\forall n \in \mathbb{N}, P(n)$.

*Proof sketch of (3):* $S(n) = n \cup \{n\}$, which contains $n$, so $S(n) \neq \emptyset = 0$.

*Proof sketch of (4):* If $S(m) = S(n)$ then $m \cup \{m\} = n \cup \{n\}$. Checking membership in both sides gives $m = n$.

*Proof of (5) (Induction principle):* Let $P$ satisfy the hypotheses. Define $K = \{n \in \mathbb{N} \mid P(n)\}$. Then $K$ is inductive (by the hypotheses), so $\mathbb{N} \subseteq K$. Therefore $K = \mathbb{N}$ and $P$ holds everywhere. $\square$

This is a beautiful circularity: induction on $\mathbb{N}$ is provable from the definition of $\mathbb{N}$ as the *smallest* inductive set.

## Arithmetic From Sets

Once we have $\mathbb{N}$ and the Peano axioms, we can define arithmetic:
- $m + 0 = m$; $m + S(n) = S(m + n)$ (recursive definition)
- $m \cdot 0 = 0$; $m \cdot S(n) = m \cdot n + m$
- $m^0 = 1$; $m^{S(n)} = m^n \cdot m$

By Replacement (applied to functional formulas defining these operations), each of $+$, $\cdot$, $\exp$ is a well-defined function $\mathbb{N} \times \mathbb{N} \to \mathbb{N}$.

From $\mathbb{N}$, we construct:
- $\mathbb{Z}$ as equivalence classes of pairs: $(m, n) \sim (m', n') \iff m + n' = m' + n$ (thinking of $(m, n)$ as "$m - n$").
- $\mathbb{Q}$ as equivalence classes of pairs $(p, q)$ with $q \neq 0$: thinking of $(p, q)$ as "$p/q$."
- $\mathbb{R}$ via Dedekind cuts (subsets of $\mathbb{Q}$ with certain properties) or via equivalence classes of Cauchy sequences.
- $\mathbb{C} = \mathbb{R} \times \mathbb{R}$ with suitable operations.

All of mathematics can be encoded this way — but the encodings are somewhat arbitrary (e.g., $\frac{1}{2}$ as a Dedekind cut is a very different set from $\frac{1}{2}$ as a Cauchy sequence equivalence class). This arbitrariness is one of the "discontents" of the set-theoretic foundation.

## Ordinals

The von Neumann construction generalizes naturally to *transfinite* numbers — the ordinals.

**Definition.** A set $\alpha$ is an *ordinal* if:
- $\alpha$ is *transitive*: $x \in \alpha \Rightarrow x \subseteq \alpha$ (members of $\alpha$ are subsets of $\alpha$)
- $\alpha$ is well-ordered by $\in$: $\in$ is a strict total order on $\alpha$ with no infinite descending chains

The natural numbers are all ordinals (under the von Neumann encoding). The *first infinite ordinal* is:
$$\omega = \mathbb{N} = \{0, 1, 2, 3, \ldots\}$$

Then:
$$\omega + 1 = \omega \cup \{\omega\} = \{0, 1, 2, \ldots, \omega\}$$
$$\omega + 2 = \{0, 1, 2, \ldots, \omega, \omega + 1\}$$
$$\omega \cdot 2 = \omega + \omega = \{0, 1, \ldots, \omega, \omega+1, \omega+2, \ldots\}$$
$$\omega^2, \omega^\omega, \omega^{\omega^\omega}, \ldots, \epsilon_0, \ldots$$

The ordinals keep going — there's no end to the transfinite.

**Key properties of ordinals:**
- Every well-ordered set is isomorphic to a unique ordinal.
- The ordinals are totally ordered by $\in$ (which is the same as $<$ for ordinals).
- Every set of ordinals has a supremum (least upper bound).
- The class of all ordinals is a proper class (not a set — Burali-Forti).

## Cardinals

Two sets $A$ and $B$ have the same *cardinality* if there is a bijection $f : A \to B$. We write $|A| = |B|$.

The *cardinality* of a set is its isomorphism class under bijection. In ZFC with AC, every set's cardinality is represented by a unique *cardinal number* (a special kind of ordinal).

**Finite cardinals:** $0, 1, 2, 3, \ldots$ (the natural numbers).

**Infinite cardinals (alephs):**
- $\aleph_0 = |\mathbb{N}|$ (countable infinity)
- $\aleph_1$ = the first uncountable cardinal
- $\aleph_2, \aleph_3, \ldots$
- $\aleph_\omega, \ldots$ (transfinite sequence of cardinals)

**Cantor's theorem:** $|\mathcal{P}(A)| > |A|$ for all $A$. Applied to $\mathbb{N}$: $|\mathcal{P}(\mathbb{N})| > \aleph_0$. What is $|\mathcal{P}(\mathbb{N})|$?

**Continuum Hypothesis (CH):** $|\mathcal{P}(\mathbb{N})| = |\mathbb{R}| = \aleph_1$ (the first uncountable cardinal).

Gödel (1938) proved CH is consistent with ZFC. Cohen (1963) proved CH is independent of ZFC. The Continuum Hypothesis is *undecidable* in ZFC — it can neither be proved nor disproved. This is the most famous independence result in set theory.

## The Cumulative Hierarchy

The *von Neumann cumulative hierarchy* organizes all sets into levels:
$$V_0 = \emptyset$$
$$V_{\alpha+1} = \mathcal{P}(V_\alpha) \quad (\text{power set of the previous level})$$
$$V_\lambda = \bigcup_{\alpha < \lambda} V_\alpha \quad (\text{for limit ordinals } \lambda)$$
$$V = \bigcup_{\alpha \in \text{Ord}} V_\alpha \quad (\text{the universe of all sets})$$

**Facts:**
- Every set appears in some $V_\alpha$.
- The *rank* of a set $x$ is the smallest $\alpha$ with $x \in V_{\alpha+1}$.
- The Axiom of Foundation is equivalent to every set having a rank.
- $V_\omega$ = all hereditarily finite sets (no infinite sets in the hierarchy below $\omega$).

The hierarchy is a *model* of ZFC (assuming ZFC is consistent), which justifies the consistency of the theory.

**Analogy with type theory:** The levels $V_\alpha$ in the cumulative hierarchy correspond to the universe levels $\mathsf{Type}_0, \mathsf{Type}_1, \mathsf{Type}_2, \ldots$ in Martin-Löf Type Theory. Just as each $V_\alpha$ contains all "small enough" sets, each $\mathsf{Type}_n$ contains all "small enough" types. The hierarchy prevents self-referential paradoxes in both frameworks.
