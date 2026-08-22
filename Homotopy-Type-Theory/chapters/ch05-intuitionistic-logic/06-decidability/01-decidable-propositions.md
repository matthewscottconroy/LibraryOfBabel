# 6.1 Decidability: Constructive and Type-Theoretic Perspectives

## What Decidability Means Constructively

In classical logic, every proposition is either true or false. This binary view means "decidability" is trivial classically — every proposition is decided.

In constructive logic, things are different. A proposition $P$ is **decidable** if there is a proof of $P \vee \neg P$ — i.e., if we have a construction that either proves $P$ or proves $\neg P$. This is not automatic!

**Definition.** A proposition $P$ is *decidable* if there exists a proof of $P \vee \neg P$.

A predicate $P : A \to \text{Prop}$ is *decidable* if $\forall x : A, P(x) \vee \neg P(x)$.

A type $A$ has *decidable equality* if $\forall x, y : A, (x = y) \vee (x \neq y)$.

**Examples of decidable propositions:**
- $n = m$ for natural numbers $n, m : \mathbb{N}$ (we can compute and compare)
- $n < m$ for natural numbers
- $P(n)$ for any primitive recursive predicate $P$
- "This finite group satisfies property $Q$" for explicitly enumerable $Q$

**Examples of propositions whose decidability is unknown or false:**
- "The Riemann Hypothesis holds"
- "This Turing machine halts on input $n$" (the halting problem is undecidable)
- $x = y$ for arbitrary real numbers given by Cauchy sequences
- "$f = g$" for arbitrary continuous functions (function equality is undecidable in general)

## Decidable Types and H-Levels

In HoTT, decidability connects deeply with h-level theory (Chapter 17).

**Definition.** A type $A$ is a *mere proposition* (h-proposition, or h-level $-1$) if all its elements are equal: $\forall x, y : A, x = y$.

**Theorem.** A mere proposition $P$ is decidable iff $P$ is either contractible ($P \simeq \mathbf{1}$) or empty ($P \simeq \mathbf{0}$).

*Proof.* If $P$ is decidable, then either we have $p : P$ (making $P$ contractible — it's a mere proposition with an element) or we have $f : P \to \mathbf{0}$ (making $P$ empty). $\square$

Decidable mere propositions are called *Boolean*: they're either true (contractible) or false (empty).

**Definition.** A type $A$ is a *set* (h-level 0) if all its identity types are mere propositions: for all $x, y : A$, the type $\text{Id}_A(x, y)$ is a mere proposition.

This means: if two elements of $A$ are equal, there's at most one proof of their equality.

**Examples of sets:**
- $\mathbb{N}$: natural numbers. Any two proofs of $n = m$ are equal (equality of naturals is a decidable mere proposition, hence a set).
- $\mathbb{Z}$, $\mathbb{Q}$: same reasoning.
- Any group, ring, field (as types) — their underlying type is a set.
- The type of all sets is not a set (it's at least a groupoid).

**Non-sets (higher h-levels):**
- $S^1$ (the circle): the identity type $\text{Id}_{S^1}(\text{base}, \text{base})$ is equivalent to $\mathbb{Z}$ (the winding number). Not a mere proposition.
- The universe $\mathcal{U}$ of all types: $\text{Id}_{\mathcal{U}}(A, B)$ is equivalent to $A \simeq B$ (type equivalences), by Univalence. Not a mere proposition in general.

## Decidable Equality for Sets

For a set $A$, decidable equality means: for all $x, y : A$, we can decide whether $x = y$ or $x \neq y$. This is:
$$\text{DecEq}(A) \;:=\; \prod_{x, y : A} (x = y) + (x \neq y)$$

Note: since $A$ is a set, $(x = y)$ is a mere proposition. So $(x = y) + (x \neq y)$ is deciding between two mere propositions — this is a "Boolean" decision.

**Examples:**
- $\mathbb{N}$ has decidable equality: check if the numerals are the same.
- $\mathbb{Z}$, $\mathbb{Q}$, $\mathbb{Z}/n\mathbb{Z}$: decidable equality.
- Any finitely presented group: whether two elements are equal (i.e., whether a word represents the identity) is in general undecidable (the word problem).
- $\mathbb{R}$: equality is *not* decidable constructively.

**Why $\mathbb{R}$ fails:** Given two real numbers as Cauchy sequences $(a_n)$ and $(b_n)$, deciding whether $a = b$ (i.e., $|a_n - b_n| \to 0$) would require knowing the limit — which might not be computable. Specifically, for the Cauchy sequence that is 0 except possibly at step $n$ if a certain Turing machine halts by step $n$, deciding $= 0$ would solve the halting problem.

## Decidability and the Law of Excluded Middle

**LEM implies all propositions are decidable.** This is essentially the definition of LEM: $\forall P, P \vee \neg P$.

**Countable choice + the structure of $\mathbb{N}$ implies $\mathbb{N}$ has decidable equality.** (This is provable constructively: just compare the numerals.)

**Key theorem (Decidability is not equivalent to LEM in HoTT).** LEM says *all* propositions are decidable. But many specific propositions are decidable constructively without needing LEM. The gap between "decidable" and "all propositions decidable" is the gap between constructive and classical mathematics.

## Decidability in Practice

In Lean 4, decidable propositions are instances of the `Decidable` typeclass:

```lean
class Decidable (p : Prop) where
  decide : p ∨ ¬p

instance : Decidable (n = m) := ...  -- for natural numbers n, m

-- Using decidability:
if h : P then ... else ...  -- requires Decidable P
```

When `Decidable P` is available, you can branch on `P` computationally. When it's not (for general propositions), you need to either provide a `Decidable` instance or use `Classical.propDecidable` (which uses LEM).

This distinction is practically important in verified software: for a certified algorithm to run, its conditions must be *computably decidable*, not just classically true-or-false.

## Decidability and the Halting Problem

The undecidability of the halting problem is intimately connected to the non-decidability of equality for computable functions.

**Theorem (Rice's Theorem).** For any non-trivial semantic property $P$ of Turing machines, it is undecidable whether a given Turing machine has property $P$.

(A property is "semantic" if it depends only on the function computed, not on the program. "Non-trivial" means neither all machines have it nor no machines have it.)

In type-theoretic terms: equality of functions $f, g : \mathbb{N} \to \mathbb{N}$ (function extensionality) is not decidable for arbitrary programs. We can check equality on finitely many inputs, but not universally in finite time.

This is why type theory uses *definitional equality* (syntactic equality of normal forms) as a decidable notion of equality, and *propositional equality* ($=$, the identity type) as a separate, possibly undecidable notion. Definitional equality is checkable by the type checker; propositional equality requires proof terms.

## Decidability in HoTT's h-Level Hierarchy

The h-levels give a way to measure "how much structure" a type has:
- h-level $-2$: contractible (one point up to homotopy)
- h-level $-1$: mere proposition (at most one element)
- h-level 0: set (equality is a mere proposition)
- h-level 1: groupoid (equalities can be composed)
- h-level $n$: $n$-groupoid (equalities up to level $n$)

Decidability is most natural at h-level $-1$ (for mere propositions) and h-level 0 (for sets with decidable equality). At higher h-levels, the notion becomes more subtle:

For a type $A$ at h-level 1 (a groupoid), decidable equality would mean deciding whether any two elements are connected by a morphism — this is the *isomorphism problem* for groupoids. For groups (as pointed groupoids), this is the conjugacy problem.

As we move up h-levels, "decidability" becomes increasingly subtle and is related to deep questions in group theory, algebraic topology, and computability.

The precise relationship between decidability, h-levels, and computation is one of the central research themes in HoTT.
