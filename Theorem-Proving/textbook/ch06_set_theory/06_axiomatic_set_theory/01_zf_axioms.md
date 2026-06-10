# Zermelo-Fraenkel Set Theory

> "We wish to rebuild the edifice of mathematics on a foundation that is secure against the paradoxes."
> — Ernst Zermelo, 1908

## From Intuition to Axioms

The collapse of naive set theory after Russell's paradox left mathematicians with a crisis and an opportunity. The crisis: our most natural, intuitive way of forming sets — "the set of all things satisfying P" — was demonstrably inconsistent. The opportunity: start over, carefully, with explicit axioms chosen to be:

1. **Consistent** (no paradoxes)
2. **Sufficient** (strong enough to recover all of standard mathematics)
3. **Minimal** (no more than necessary)

Ernst Zermelo (1908), later refined by Abraham Fraenkel and Thoralf Skolem, produced what became the standard: **ZF** (Zermelo-Fraenkel) set theory, or **ZFC** when the Axiom of Choice is included.

ZFC has proven remarkably successful. Every theorem in standard mathematics — calculus, algebra, topology, number theory, combinatorics — can be formalized within ZFC. It is, in a precise sense, *universal mathematical infrastructure*.

## Reading the Axioms

Each axiom of ZF is a sentence in first-order logic with the single binary relation symbol $\in$. The only things that exist in this universe are **sets** — no urelements (non-set individuals). Numbers, functions, ordered pairs, and everything else are encoded as sets.

### Axiom 1: Extensionality

$$\forall A\, \forall B\, (\forall x\, (x \in A \leftrightarrow x \in B) \to A = B)$$

**Meaning**: Two sets with exactly the same members are identical. There is only one empty set, only one set $\{1, 2, 3\}$, and so on. Sets are "extensional" objects — their identity is completely determined by their content.

**Why needed**: Without this, we might have many "different" empty sets. The axiom collapses these to one.

### Axiom 2: Empty Set

$$\exists A\, \forall x\, (x \notin A)$$

**Meaning**: Something exists with no elements — the empty set $\emptyset$. By Extensionality, there is exactly one such thing.

**Why needed**: Without this, ZF would have models with no sets at all. We need at least one set to get started.

### Axiom 3: Pairing

$$\forall a\, \forall b\, \exists A\, \forall x\, (x \in A \leftrightarrow x = a \vee x = b)$$

**Meaning**: For any two sets $a$ and $b$, the set $\{a, b\}$ exists. This gives us $\{a\}$ (by pairing $a$ with itself) and $\{a, b\}$ for distinct $a, b$.

**Why needed**: To build larger sets, we need to be able to group things together.

### Axiom 4: Union

$$\forall \mathcal{F}\, \exists A\, \forall x\, (x \in A \leftrightarrow \exists B\, (B \in \mathcal{F} \wedge x \in B))$$

**Meaning**: For any set $\mathcal{F}$ of sets, the union $\bigcup \mathcal{F}$ — containing all elements of all members of $\mathcal{F}$ — exists.

**Example**: $\bigcup \{\{1,2\}, \{3,4\}, \{2,5\}\} = \{1, 2, 3, 4, 5\}$.

In particular, $A \cup B = \bigcup \{A, B\}$, and $A \cup B$ exists by Pairing plus Union.

### Axiom 5: Power Set

$$\forall A\, \exists P\, \forall x\, (x \in P \leftrightarrow x \subseteq A)$$

**Meaning**: For any set $A$, the set $\mathcal{P}(A)$ of all subsets of $A$ exists.

**Example**: $\mathcal{P}(\{1,2\}) = \{\emptyset, \{1\}, \{2\}, \{1,2\}\}$ — four elements.

**Why surprising**: Starting from finite sets, iterated power sets grow extremely quickly: $|\mathcal{P}(A)| = 2^{|A|}$. This is how we get uncountably infinite sets from countable ones: $|\mathcal{P}(\mathbb{N})| = 2^{\aleph_0} = |\mathbb{R}|$.

### Axiom 6: Separation (Restricted Comprehension)

$$\forall A\, \exists B\, \forall x\, (x \in B \leftrightarrow x \in A \wedge \varphi(x))$$

for any formula $\varphi$ not containing $B$.

**Meaning**: Given an existing set $A$ and a property $\varphi$, the subset $\{x \in A \mid \varphi(x)\}$ exists. We can *filter* sets, not create them from scratch.

**Key difference from naive comprehension**: You cannot collect *all* $x$ satisfying $\varphi$ — only $x$ from within an existing set $A$. This blocks Russell's paradox because to form $\{x \mid x \notin x\}$, you would need an "all sets" container, which ZF never provides.

### Axiom 7: Infinity

$$\exists A\, (\emptyset \in A \wedge \forall x \in A\, (x \cup \{x\} \in A))$$

**Meaning**: An infinite set exists. Specifically, there is a set containing $\emptyset$, and if it contains $x$ then it contains $x \cup \{x\}$.

**This builds $\mathbb{N}$**:
- $0 = \emptyset$
- $1 = \{0\} = \{\emptyset\}$
- $2 = \{0, 1\} = \{\emptyset, \{\emptyset\}\}$
- $3 = \{0, 1, 2\} = \{\emptyset, \{\emptyset\}, \{\emptyset, \{\emptyset\}\}\}$

The set guaranteed by the Axiom of Infinity (or a subset of it isolated by Separation) serves as the natural numbers. **Every natural number is the set of all smaller natural numbers** — a brilliant encoding.

### Axiom 8: Replacement

If $\varphi(x, y)$ is a formula that is functional (for each $x \in A$, there is exactly one $y$ satisfying $\varphi$), then $\{y \mid \exists x \in A\, \varphi(x, y)\}$ exists.

**Meaning**: The image of any set under a "definable function" is itself a set. This makes transfinite arithmetic possible.

**Why needed**: Power set and Union alone cannot build all the sets needed for ordinal arithmetic. Replacement allows sets to "go as far" as definable functions can reach.

### Axiom 9: Foundation (Regularity)

$$\forall A\, (A \neq \emptyset \to \exists x \in A\, (x \cap A = \emptyset))$$

**Meaning**: Every non-empty set has an $\in$-minimal element — an element sharing no members with the set.

**Key consequence**: No set can be a member of itself ($A \notin A$). No infinite descending chains $\ldots \in A_2 \in A_1 \in A_0$ exist. Sets form a *well-founded* hierarchy.

**The cumulative hierarchy** ($V$):
- $V_0 = \emptyset$
- $V_{\alpha+1} = \mathcal{P}(V_\alpha)$
- $V_\lambda = \bigcup_{\alpha < \lambda} V_\alpha$ (for limit ordinals $\lambda$)

Every set appears in some $V_\alpha$ in this hierarchy. Foundation is what makes this structure work.

### Axiom 10: Choice (The Axiom of Choice)

$$\forall \mathcal{F}\, (\emptyset \notin \mathcal{F} \to \exists f\colon \mathcal{F} \to \bigcup \mathcal{F}\, \forall B \in \mathcal{F}\, (f(B) \in B))$$

**Meaning**: Given a family of non-empty sets, there is a "choice function" that picks one element from each set simultaneously.

**Why this is deep**: For finite families, choice is trivial — just enumerate and pick. For *infinite* families of non-empty sets, there may be no *definable* rule for picking; the axiom asserts a choice function *exists* even if it cannot be explicitly constructed.

**Equivalent statements** (all provable equivalent to AC in ZF):
- Zorn's Lemma: every chain-bounded partial order has a maximal element
- Well-Ordering Theorem: every set can be well-ordered
- Tychonoff's theorem: an arbitrary product of compact spaces is compact
- Every vector space has a basis
- Every surjection has a right inverse

**The independence of AC**: Both ZF + AC (giving ZFC) and ZF + ¬AC are consistent (assuming ZF is). Gödel showed ZFC is consistent in 1938; Cohen showed ZF + ¬AC is consistent in 1963 using the technique of *forcing*. You can do mathematics with or without the Axiom of Choice — most mathematicians accept it (using ZFC) because it makes the mathematics cleaner.

**Strange consequences of AC**:
- Banach-Tarski paradox: A solid ball can be decomposed into finitely many pieces and reassembled into two balls of the same size as the original
- Non-measurable sets: There exist subsets of $[0,1]$ with no well-defined "length"
- These consequences are theoretically valid but cannot be physically realized — they use AC to make arbitrary choices across uncountably many sets simultaneously

## The Landscape Beyond ZFC

ZFC is powerful but not omnipotent. Several important questions are **independent of ZFC** — neither provable nor disprovable:

| Statement | Status |
|-----------|--------|
| Continuum Hypothesis (CH) | Independent (Gödel 1938, Cohen 1963) |
| Generalized CH | Independent |
| Large cardinal axioms (e.g., "inaccessible cardinals exist") | Consistent extensions of ZFC |
| Projective Determinacy (PD) | Independent; follows from large cardinals |

This independence is not a flaw — it reveals that ZFC describes a *family* of possible mathematical universes, not a single determined one. Adding further axioms (like large cardinal axioms) allows us to answer more questions, at the cost of stronger consistency assumptions.

## ZF in Lean 4

Lean 4 is not based on ZF but on **Calculus of Inductive Constructions (CIC)**, a dependent type theory. Nevertheless, ZF-like reasoning is available:

```lean
import Mathlib.Order.SetNotation

-- Separation: filter a set by a predicate
example (A : Set ℕ) : {x ∈ A | x > 5} ⊆ A := by
  intro x hx
  exact hx.1

-- Power set: Set (Set α) in Lean
example (A : Set ℕ) (B : Set ℕ) (h : B ⊆ A) : B ∈ 𝒫 A := by
  exact h
```

In Lean's universe hierarchy, there is no "set of all sets" in any given universe — this is the type-theoretic equivalent of ZF's No Universal Set principle.

## Stop and Think

*Why are there 9 or 10 axioms rather than one?*

Because no single simple principle avoids paradox while recovering all of mathematics. Naive comprehension (*one* principle — every property defines a set) is the simplest possible set theory and it is inconsistent. ZF's multiple axioms are chosen to be individually "obviously true" about sets while collectively giving just enough power to do mathematics without granting enough power to form paradoxical constructions.

This multiplicity is a sign of the difficulty: set theory is doing something hard — talking about all possible infinite structures — and it needs careful constraints to stay consistent.

## Exercises
See [problems/ch06_set_theory/05_axiomatic_exercises.md](../../../problems/ch06_set_theory/05_axiomatic_exercises.md)
