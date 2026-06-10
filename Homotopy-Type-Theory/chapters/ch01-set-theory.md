# Chapter 1: Set Theory — The Classical Foundation and Its Discontents

## Introduction

For most of the twentieth century, set theory — specifically Zermelo-Fraenkel set theory with the Axiom of Choice (ZFC) — served as the official foundation of mathematics. Every mathematical object was (in principle) a set; every mathematical theorem could (in principle) be expressed as a statement about sets; every mathematical proof could (in principle) be verified against the ZFC axioms.

This chapter has two purposes. First, we will understand this foundation — the ZFC axioms, what they give us, and how ordinary mathematics is built from them. This is genuine and important knowledge; the mathematics in later chapters (group theory, topology, category theory) is built in this framework.

Second, we will see why set theory, despite its success, is not entirely satisfying as a foundation — and why type theory offers something genuinely different. The problems are not merely aesthetic. They concern the nature of identity (when are two things "the same"?), the computational content of proofs, and the relationship between logic and mathematics.

---

## 1. The Naive Approach and Its Failure

### 1.1 Cantor's Intuition

Georg Cantor's naïve set theory began with a simple idea: a *set* is any collection of objects sharing a common property. Given a property $P$, we can form the set $\{x \mid P(x)\}$ of all objects satisfying $P$. This is called *unrestricted comprehension*.

This is beautifully simple and matches intuition. Unfortunately, it is inconsistent.

### 1.2 Russell's Paradox

**Paradox 1.1 (Russell, 1901).** Let $R = \{x \mid x \notin x\}$ — the set of all sets that are not members of themselves. Is $R \in R$?

- If $R \in R$: then by the defining property of $R$, we need $R \notin R$. Contradiction.
- If $R \notin R$: then $R$ satisfies the property of $R$, so $R \in R$. Contradiction.

In either case we get a contradiction. Therefore unrestricted comprehension is *inconsistent* — it allows us to derive a contradiction from the axioms alone.

**Remark 1.2.** Russell's paradox is not a curiosity. It showed that informal mathematical reasoning, even by the greatest mathematicians, had been operating on an inconsistent foundation. The crisis this caused led directly to the axiomatic programs of Hilbert, Zermelo, and others.

Other paradoxes confirm the problem:
- **Burali-Forti paradox:** The "set of all ordinals" would have to be an ordinal larger than all ordinals — a contradiction.
- **Cantor's paradox:** The "set of all sets" would have a power set larger than itself — but it is the largest set. Contradiction.

The solution is *restriction*: we must carefully axiomatize what sets exist, rather than permitting unrestricted formation.

---

## 2. The Zermelo-Fraenkel Axioms

ZFC consists of nine axioms (or nine axiom schemes). We state each one, explain its motivation, and give examples.

Throughout, $\in$ denotes set membership.

### Axiom 1: Extensionality

$$\forall A\, \forall B\, [\forall x\, (x \in A \leftrightarrow x \in B) \to A = B]$$

Two sets are equal if and only if they have exactly the same members. This axiom defines what set *equality* means: sets have no structure beyond their elements. The set $\{1, 2, 3\}$ and the set $\{3, 1, 2\}$ are identical because they have the same members.

**Consequence:** A set is entirely determined by its members. There is no additional "intensional" information — no notion of how a set was defined or constructed, only what it contains. This will become a key contrast with type theory.

### Axiom 2: Empty Set

$$\exists A\, \forall x\, (x \notin A)$$

There exists a set with no members. By extensionality, this set is unique; we call it $\emptyset$ or $\varnothing$.

### Axiom 3: Pairing

$$\forall a\, \forall b\, \exists C\, \forall x\, (x \in C \leftrightarrow x = a \vee x = b)$$

For any two sets $a$ and $b$, there is a set $\{a, b\}$ containing exactly $a$ and $b$. Combined with the empty set axiom: $\{a\} = \{a, a\}$ exists by pairing.

### Axiom 4: Union

$$\forall \mathcal{F}\, \exists U\, \forall x\, (x \in U \leftrightarrow \exists A \in \mathcal{F},\, x \in A)$$

For any family of sets $\mathcal{F}$, there is a set $\bigcup \mathcal{F}$ containing exactly the members of members of $\mathcal{F}$.

**Example 1.3.** $\bigcup \{\{1,2\}, \{2,3\}, \{4\}\} = \{1,2,3,4\}$.

From Union and Pairing, we can form $A \cup B = \bigcup \{A, B\}$.

### Axiom 5: Power Set

$$\forall A\, \exists P\, \forall B\, (B \in P \leftrightarrow B \subseteq A)$$

For every set $A$, there is a set $\mathcal{P}(A)$ of all subsets of $A$.

**Example 1.4.** $\mathcal{P}(\{0,1\}) = \{\emptyset, \{0\}, \{1\}, \{0,1\}\}$.

The power set axiom is the most "generous" axiom of ZFC in terms of the sets it guarantees to exist. It is responsible for much of the richness (and difficulty) of set-theoretic mathematics.

### Axiom 6: Separation (Restricted Comprehension)

$$\forall A\, \forall \varphi\, \exists B\, \forall x\, (x \in B \leftrightarrow x \in A \wedge \varphi(x))$$

(Here $\varphi$ ranges over formulas in the language of set theory with $x$ free.) Given an *existing* set $A$ and a property $\varphi$, we can form the *subset* $\{x \in A \mid \varphi(x)\}$ of elements of $A$ satisfying $\varphi$.

This is comprehension with a crucial restriction: we can only *separate* a subset of an already-existing set, not create sets from nothing. This blocks Russell's paradox: to form $R = \{x \mid x \notin x\}$ we would need a set of *all* sets to separate from — but no such set exists in ZFC.

**Example 1.5.** The set of even natural numbers: $\{n \in \mathbb{N} \mid \exists k \in \mathbb{N}, n = 2k\}$.

### Axiom 7: Replacement

If $\varphi(x, y)$ is a functional relation (for each $x$ there is at most one $y$ with $\varphi(x,y)$), then the image of any set under $\varphi$ is a set.

$$\forall A\, [\forall x \in A\, \exists! y\, \varphi(x,y)] \to \exists B\, \forall y\, [y \in B \leftrightarrow \exists x \in A, \varphi(x,y)]$$

Replacement says: the "image" of any set under any definable function is again a set. It is needed to construct transfinite sequences and to ensure that certain ordinals exist.

### Axiom 8: Foundation (Regularity)

$$\forall A\, [A \neq \emptyset \to \exists x \in A,\, x \cap A = \emptyset]$$

Every non-empty set has an $\in$-minimal element — an element that shares no members with the set. This rules out:
- Sets that contain themselves ($x \in x$, ruled out since $\{x\}$ would have no $\in$-minimal element)
- Infinite descending chains $\cdots \in x_2 \in x_1 \in x_0$

Foundation ensures the *well-foundedness* of $\in$, which underpins induction in set theory.

### Axiom 9: Axiom of Choice (AC)

$$\forall \mathcal{F}\, [\emptyset \notin \mathcal{F} \to \exists f\, (f : \mathcal{F} \to \bigcup\mathcal{F}\ \wedge\ \forall A \in \mathcal{F},\, f(A) \in A)]$$

For any collection of non-empty sets, there is a *choice function* that selects one element from each. When the collection is finite, this is trivial. When infinite, it is a genuine assumption.

**Equivalent forms of AC (all provably equivalent in ZF):**
- *Zorn's Lemma:* Every non-empty partially ordered set where every chain has an upper bound has a maximal element.
- *Well-Ordering Theorem:* Every set can be well-ordered.
- *Tychonoff's Theorem:* An arbitrary product of compact topological spaces is compact.

**Independence:** Gödel (1938) showed AC is *consistent* with ZF (it cannot be disproved). Cohen (1963) showed AC is *independent* of ZF (it cannot be proved). So ZF + AC (= ZFC) and ZF + ¬AC are both consistent theories — assuming ZF is consistent.

---

## 3. Building Mathematics in ZFC

### 3.1 The Natural Numbers

We construct $\mathbb{N}$ using the von Neumann encoding:
$$0 = \emptyset, \quad 1 = \{0\} = \{\emptyset\}, \quad 2 = \{0, 1\} = \{\emptyset, \{\emptyset\}\}, \quad n+1 = n \cup \{n\}$$

The existence of $\mathbb{N}$ as a completed set requires a further axiom (the *Axiom of Infinity*): there exists an *inductive set* — a set $I$ with $\emptyset \in I$ and $(\forall x \in I,\, x \cup \{x\} \in I)$. We then define $\mathbb{N}$ as the smallest inductive set (using Separation).

**Theorem 1.6 (Peano Axioms from ZFC).** With this definition:
- $0 \in \mathbb{N}$
- If $n \in \mathbb{N}$, then $S(n) = n \cup \{n\} \in \mathbb{N}$
- $S(n) \neq 0$ for all $n$
- $S$ is injective
- Induction holds: if $P(0)$ and $\forall n \in \mathbb{N},\, P(n) \to P(S(n))$, then $\forall n \in \mathbb{N},\, P(n)$.

### 3.2 Ordered Pairs and Functions

The *Kuratowski pair*: $\langle a, b \rangle = \{\{a\}, \{a, b\}\}$.

**Theorem 1.7.** $\langle a, b \rangle = \langle c, d \rangle \Leftrightarrow a = c \wedge b = d$.

A *relation* from $A$ to $B$ is a subset of $A \times B = \{\langle a, b\rangle \mid a \in A, b \in B\}$. A *function* $f : A \to B$ is a relation such that for every $a \in A$ there is exactly one $b \in B$ with $\langle a, b \rangle \in f$.

### 3.3 Ordinals and Cardinals

**Definition 1.8.** A set $\alpha$ is an *ordinal* if it is transitive ($x \in \alpha \to x \subseteq \alpha$) and well-ordered by $\in$.

The ordinals are: $0, 1, 2, \ldots, \omega, \omega+1, \omega+2, \ldots, \omega \cdot 2, \ldots, \omega^2, \ldots, \omega^\omega, \ldots, \epsilon_0, \ldots$

Every well-ordered set is isomorphic to a unique ordinal. Ordinals serve as canonical representatives of order types.

**Definition 1.9.** Two sets $A$ and $B$ have the same *cardinality* (written $|A| = |B|$) if there is a bijection $f : A \to B$.

A set is *finite* if it has the same cardinality as some natural number. A set is *countable* if it has the same cardinality as $\mathbb{N}$ (or is finite). A set is *uncountable* if it is infinite and not countable.

**Theorem 1.10 (Cantor).** For any set $A$, $|A| < |\mathcal{P}(A)|$. In particular, $\mathbb{R}$ is uncountable.

*Proof sketch.* There is an injection $A \hookrightarrow \mathcal{P}(A)$ (send each $a$ to $\{a\}$), but no surjection. Given any function $f : A \to \mathcal{P}(A)$, the set $D = \{x \in A \mid x \notin f(x)\}$ (a diagonal construction) is not in the image of $f$. $\square$

---

## 4. The Axiom of Choice: Deeper

The Axiom of Choice is philosophically contentious in constructive mathematics.

**Why constructivists object:** AC asserts the *existence* of a choice function without giving any means to construct it. From a constructive standpoint (BHK interpretation, Chapter 5), to prove $\exists f, P(f)$ you must *exhibit* an $f$. AC claims existence without exhibition.

**Consequences of ¬AC:** If we work in ZF without AC, some things break:
- Not every vector space has a basis
- Not every ring has a maximal ideal
- There exist non-measurable sets — wait, actually those require AC to construct. Without AC, it's consistent that every subset of $\mathbb{R}$ is Lebesgue measurable.

**In HoTT:** The axiom of choice is *not* assumed globally. Instead, there is a hierarchy of choice principles, some of which are theorems in HoTT and some of which are independent. The precise relationship between choice and the univalence axiom is subtle and interesting.

---

## 5. Why Set Theory Is Not Enough

Despite its success, set theory has serious limitations as a foundation for mathematics as it is actually practiced.

### 5.1 The Problem of Identity

In ZFC, two mathematical objects are equal if and only if they are the same set. But mathematicians routinely identify objects that are "canonically isomorphic" but not literally equal. For example:
- The real numbers can be constructed as Dedekind cuts or as Cauchy sequences of rationals. These constructions give different sets, but mathematicians treat them as "the same."
- Any two groups of order $p$ (prime) are isomorphic — we say "the cyclic group of order $p$," not "a particular one of the many isomorphic copies."

ZFC has no native way to express "equal up to isomorphism" as a form of equality. Working mathematicians solve this by convention and informal identification, but this is extra-logical.

**HoTT's answer:** The *univalence axiom* makes "equivalent types are equal" a theorem (for types, not sets). This directly formalizes the mathematical practice of identifying isomorphic structures.

### 5.2 The Computational Content Problem

A proof in ZFC can prove the existence of an object without giving any algorithm for computing it. For example: the axiom of choice proves that every surjection $A \twoheadrightarrow B$ has a right inverse, but gives no algorithm to find it.

In the foundations of computer science, we want proofs to have *computational content* — a proof of $\exists n, P(n)$ should yield an actual $n$. ZFC makes no such guarantee.

**Type theory's answer:** In dependent type theory, every proof of $\exists n, P(n)$ (written $\Sigma_{n:\mathbb{N}} P(n)$) comes with a witness $n$ and a proof of $P(n)$. Proofs are programs; theorems are specifications.

### 5.3 Sethood as a Concept, Not a Given

In ZFC, everything is a set. The number $2$, the ordered pair $\langle 0, 1\rangle$, the function $\sin : \mathbb{R} \to \mathbb{R}$, the group $\mathbb{Z}/2\mathbb{Z}$ — all are sets. But asking "is $2 \in \sin$?" is a well-formed question in ZFC (the answer is "no," because $2$ is not in the graph of $\sin$, but the question itself is silly).

Type theory imposes *types*: you cannot even *form* the question "$2 \in \sin$" because $2$ and $\sin$ have different types. Types are a discipline that prevents category errors.

---

## 6. The Cumulative Hierarchy

The universe of set theory is the *cumulative hierarchy* $V = \bigcup_{\alpha \in \text{Ord}} V_\alpha$, defined by:
$$V_0 = \emptyset, \quad V_{\alpha+1} = \mathcal{P}(V_\alpha), \quad V_\lambda = \bigcup_{\alpha < \lambda} V_\alpha \text{ (for limit ordinals } \lambda\text{)}.$$

Every set is in some $V_\alpha$, and the *rank* of a set is the least $\alpha$ with $\text{set} \in V_{\alpha+1}$.

This hierarchy provides the "big picture" structure of set theory and is analogous to the *universe hierarchy* in type theory: $\mathsf{Type}_0 : \mathsf{Type}_1 : \mathsf{Type}_2 : \cdots$

---

## Exercises

**1.1.** Show that the following are pairwise equivalent (in ZF), assuming you have access to the other axioms:
  - Axiom of Choice
  - Every surjection has a right inverse
  - Every set can be well-ordered

**1.2.** Using only the ZFC axioms stated above (plus the Axiom of Infinity), construct the integers $\mathbb{Z}$ as a set. (*Hint:* construct $\mathbb{Z}$ as equivalence classes of pairs of natural numbers under the relation $(a, b) \sim (c, d)$ iff $a + d = b + c$.)

**1.3.** Prove Cantor's theorem: for any set $A$, there is no surjection from $A$ to $\mathcal{P}(A)$. Use the diagonal argument as in the proof sketch.

**1.4.** The *Burali-Forti paradox:* Show that if there were a "set of all ordinals" $\Omega$, then $\Omega$ would itself be an ordinal, and hence $\Omega \in \Omega$, contradicting Foundation.

**1.5.** The *Schröder-Bernstein theorem:* If there are injections $f : A \to B$ and $g : B \to A$, then there is a bijection $A \to B$. Prove this. (*Hint:* Define $C_0 = A \setminus g(B)$ and $C_{n+1} = g(f(C_n))$; consider the partition of $A$ into $\bigcup_n C_n$ and its complement.)

**1.6.** Show that the Axiom of Foundation implies there is no set $x$ with $x \in x$. Show it also rules out $x \in y \in x$.

**1.7.** Explain in your own words why the following argument is not a proof of the Axiom of Choice: "Given non-empty sets $A_i$, for each $i$ pick any element of $A_i$; the collection of chosen elements is the choice function." (What goes wrong when the collection of sets is infinite?)

**1.8 (Research).** Look up and explain Cohen's forcing method at a high level. What does it mean for a statement to be "independent of ZFC"? How is this different from the statement being false?

**1.9.** In ZFC, every real number is a specific set (via the Dedekind cut construction). Write down explicitly what specific set the number $\frac{1}{2}$ is in this construction.

**1.10 (Conceptual).** In ZFC, given the group $(\mathbb{Z}/2\mathbb{Z}, +)$ and the group $(\{0,1\}, \oplus)$ where $\oplus$ is XOR (which is literally the same group, just with different notation), are these the same set or different sets? What does this tell you about the adequacy of ZFC for capturing mathematical identity?
