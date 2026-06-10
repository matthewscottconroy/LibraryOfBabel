# 2.1 The ZFC Axioms: Extensionality and Basic Existence

## The Strategy of ZFC

ZFC's response to the paradoxes is pragmatic: don't try to axiomatize all of logic (which led to Frege's inconsistency); instead, axiomatize just what you need.

Specifically, ZFC:
1. Assumes the existence of a few basic sets (empty set, infinity).
2. Provides *construction axioms* for building new sets from old (pairing, union, power set).
3. Allows *restricted comprehension* (separation) to carve out subsets.
4. Adds *replacement* to handle transfinite constructions.
5. Restricts what sets can "look like" via *foundation*.
6. Asserts the existence of *choice functions* for non-empty collections.

The axioms are designed to be just strong enough for all of mathematics, but careful enough to avoid contradictions.

## Axiom of Extensionality

**Axiom (Extensionality):**
$$\forall A\, \forall B\, [\forall x\, (x \in A \leftrightarrow x \in B) \to A = B]$$

Two sets are equal if and only if they have the same members. Period. A set has no other identifying information beyond its elements.

**What this axiom establishes:** Set equality means *same elements*. Two sets built by different means, from different materials, in different orders — if they end up containing the same things, they are the same set.

**Consequences:**
- $\{1, 2, 3\} = \{3, 1, 2\} = \{1, 1, 2, 3\}$ (sets have no order or repetition — the last one works because the element $1$ is either in the set or not, regardless of being "listed twice").
- The empty set $\emptyset$ is unique: if $A$ and $B$ are both empty, then $\forall x, (x \in A \leftrightarrow x \in B)$ vacuously, so $A = B$.
- There is no "inner structure" to a set beyond its elements.

**The contrast with type theory:** In type theory, *types* can have intensional identity: two types might be definitionally equal (identical in structure) or propositionally equal (connected by a proof of equality), and these can differ. In HoTT, the Univalence Axiom says types are equal iff they are equivalent (have a bijection), which is an *extensional* principle — but at the level of types, not at the level of sets. The set-theoretic and type-theoretic notions of equality are genuinely different.

## Axiom of Empty Set

**Axiom (Empty Set):**
$$\exists A\, \forall x\, (x \notin A)$$

There exists a set with no members. By extensionality, this set is unique. We call it $\emptyset$ or $\varnothing$.

**Why we need this:** Without asserting the existence of at least one set, we might have a model where no sets exist at all — technically consistent but useless. The empty set is the starting point from which everything is built.

**Using the empty set:** The empty set is a subset of every set: $\emptyset \subseteq A$ for all $A$ (vacuously, since there's nothing in $\emptyset$ that could fail to be in $A$). It plays the role of "zero" in cardinal arithmetic.

## Axiom of Pairing

**Axiom (Pairing):**
$$\forall a\, \forall b\, \exists C\, \forall x\, (x \in C \leftrightarrow x = a \vee x = b)$$

For any two sets $a$ and $b$, there is a set $\{a, b\}$ containing exactly $a$ and $b$. Note: if $a = b$, this gives $\{a\}$ (a singleton).

**Derived sets:** Using pairing and union, we can build:
- Singletons: $\{a\} = \{a, a\}$
- Ordered pairs: using the Kuratowski encoding $\langle a, b \rangle = \{\{a\}, \{a, b\}\}$ (see Section 3.2)
- Finite sets of any size: $\{a, b, c\} = \{a, b\} \cup \{c\}$ (once union is available)

## Axiom of Union

**Axiom (Union):**
$$\forall \mathcal{F}\, \exists U\, \forall x\, (x \in U \leftrightarrow \exists A \in \mathcal{F},\, x \in A)$$

For any set $\mathcal{F}$ (thought of as a family of sets), the *union* $\bigcup \mathcal{F}$ exists: the set of all things that are members of some member of $\mathcal{F}$.

**Familiar operations:** 
- Binary union: $A \cup B = \bigcup \{A, B\}$ (the union of the family $\{A, B\}$)
- $\bigcup_{i \in I} A_i = \bigcup \{A_i \mid i \in I\}$ (arbitrary union)

**Example:** $\bigcup \{\{1,2\}, \{2,3\}, \{4\}\} = \{1, 2, 3, 4\}$.

**Intersection:** There is no separate intersection axiom; intersection is defined via Separation:
$$A \cap B = \{x \in A \mid x \in B\}$$

## Axiom of Power Set

**Axiom (Power Set):**
$$\forall A\, \exists P\, \forall B\, (B \in P \leftrightarrow B \subseteq A)$$

For every set $A$, there is a set $\mathcal{P}(A)$ (the *power set* of $A$) whose members are exactly the subsets of $A$.

**Cardinality implications:** If $|A| = n$ (finite), then $|\mathcal{P}(A)| = 2^n$. For infinite $A$, Cantor's theorem guarantees $|\mathcal{P}(A)| > |A|$.

This axiom is crucial for defining the real numbers (as subsets of $\mathbb{Q}$ via Dedekind cuts, or as equivalence classes of Cauchy sequences via the quotient of a subset of $\mathbb{Q}^{\mathbb{N}}$).

**The power set axiom is generous.** It asserts the existence of all subsets of a set, even ones with no explicit description. For infinite sets, this produces very large collections. The power set of $\mathbb{N}$ is uncountable; the power set of the reals is even larger.

This generosity is both a feature (it gives us all the sets we need) and a philosophical challenge (it produces sets with no constructive description). Constructive mathematics either rejects or significantly weakens the power set axiom.

## Axiom of Separation (Restricted Comprehension)

**Axiom Scheme (Separation):** For each formula $\varphi(x)$ (possibly with parameters),
$$\forall A\, \exists B\, \forall x\, (x \in B \leftrightarrow x \in A \wedge \varphi(x))$$

Given an *existing set* $A$ and a *definable property* $\varphi$, we can form the *subset* $\{x \in A \mid \varphi(x)\}$.

**Why this blocks Russell's paradox:** To form $\{x \mid x \notin x\}$, we would need a set $A$ of all sets to separate from. But no such $A$ exists in ZFC (the paradoxes showed it cannot). With restricted comprehension, we can only form subsets of existing sets, so the paradox never arises.

**What Separation gives us:** Almost all subsets we care about:
- Even natural numbers: $\{n \in \mathbb{N} \mid \exists k \in \mathbb{N}, n = 2k\}$
- Primes: $\{n \in \mathbb{N} \mid n > 1 \wedge \forall d \in \mathbb{N}, d \mid n \to d = 1 \vee d = n\}$
- Functions with a given property: $\{f \in A^B \mid P(f)\}$

**Separation is an axiom scheme:** There is one axiom for each formula $\varphi$. Since there are infinitely many formulas, ZFC is technically an infinite theory. This is standard; it doesn't prevent ZFC from being effective (there's an algorithm to decide if a given sentence is an axiom).

## Axiom of Replacement

**Axiom Scheme (Replacement):** If $\varphi(x, y)$ is a functional formula (each $x$ has at most one $y$ with $\varphi(x, y)$), then for any set $A$:
$$[\forall x \in A, \exists! y, \varphi(x, y)] \to \exists B, \forall y [y \in B \leftrightarrow \exists x \in A, \varphi(x, y)]$$

If we have a function defined by a formula that maps each element of $A$ to a unique $y$, the image of $A$ under this function is a set.

**Why Separation isn't enough:** Separation can only produce subsets of existing sets. But sometimes we need to build sets that are "as large as" some existing set $A$ but consist of different objects. For example, the set $\{V_0, V_1, V_2, \ldots\}$ (cumulative hierarchy levels indexed by $\mathbb{N}$) has the same size as $\mathbb{N}$ but consists of cumulative hierarchy levels — objects that aren't subsets of $\mathbb{N}$.

Replacement says: if you can *function* from $A$ to various sets, the collection of those sets is itself a set.

**Replacement is essential for ordinal arithmetic** and for constructing transfinite sequences.

## Axiom of Foundation (Regularity)

**Axiom (Foundation):**
$$\forall A\, [A \neq \emptyset \to \exists x \in A,\, x \cap A = \emptyset]$$

Every non-empty set has an *$\in$-minimal element*: an element $x$ that shares no members with $A$.

**What Foundation prevents:**
- Self-membership: if $A \in A$, then $\{A\}$ has no minimal element (since $A \cap \{A\} = \{A\} \neq \emptyset$). So $A \in A$ is impossible.
- Membership cycles: $A \in B \in A$ is impossible by the same argument.
- Infinite descending chains: $\cdots \in A_2 \in A_1 \in A_0$ would violate Foundation.

**Why Foundation is useful:** It ensures the *well-foundedness of $\in$*, which is the basis for induction in set theory. Every set has a "rank" (a stage in the cumulative hierarchy where it first appears), and you can do induction on this rank.

Foundation has no mathematical content for "normal" mathematics — all the mathematical sets we care about satisfy it automatically. It's a tidying-up axiom that ensures the universe of sets is well-structured.

In *non-well-founded set theory* (developed by Aczel and others), Foundation is replaced by an anti-foundation axiom allowing $A \in A$ and other cycles. This is useful for modeling circular data structures and coalgebraic processes, but we don't need it here.
