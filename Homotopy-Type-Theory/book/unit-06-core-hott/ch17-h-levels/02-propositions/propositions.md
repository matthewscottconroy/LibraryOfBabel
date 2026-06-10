# Mere Propositions

## Truth Without Witnesses

In constructive mathematics, there is a distinction that classical logic ignores: the distinction between *knowing that P is true* and *knowing which element of P you have*. In classical logic, "P is true" and "P is provable" and "P has a proof" are all equivalent. In constructive mathematics, these come apart.

A *mere proposition* is a type where this distinction collapses. If a type A is a proposition, then any two elements of A are equal. Once you know A is inhabited, you know everything there is to know about it — not because the elements are identical, but because they are all equal to each other. The specific element you have carries no additional information; it only tells you that A is inhabited.

This is the type-theoretic formalization of the classical notion of a truth value. Classical logic has two truth values: true and false. In HoTT, "true" corresponds to a contractible proposition (inhabited, uniquely), and "false" corresponds to the empty proposition (empty). But propositions as a class include both, and the class of propositions is closed under the standard logical operations.

## The Definition

**Definition.** A type A is a *mere proposition* (or *h-prop* or *(-1)-type*) if any two of its elements are equal:

```
isProp(A) := Π(a b : A). (a = b)
```

This is weaker than contractibility: we do not require a specific center of contraction, only that any two elements be equal. An inhabited proposition is contractible (any element serves as the center), but the empty type is also a proposition (vacuously: there are no two elements to compare).

## Basic Examples

**The empty type ∅** is a proposition. There are no elements, so the condition `Π(a b : ∅). a = b` is vacuously true. (This is the proposition *false*.)

**The unit type 1** is a proposition — in fact, it is contractible. The only element is *, and * = * by refl. (This is the proposition *true*.)

**The type `a = b`** (for any fixed a, b in a set A) is a proposition when A is a set. (We prove this in the next section.)

**Decidable propositions** are propositions P such that P + ¬P is inhabited. The booleans Bool are *not* propositions (they have two distinct elements), but a type like "n is even or not" expressed as a Sigma-type does give a proposition.

## Propositions vs. Proof-Relevant Types

The key contrast is between propositions and *proof-relevant* types.

Consider the type of prime factorizations of a natural number n. In classical mathematics, we say "n has a unique prime factorization" — there is one. In HoTT, the type of prime factorizations is:

```
Σ(k : N). Σ(p : Vec(Prime, k)). product(p) = n ∧ isSorted(p)
```

Is this a proposition? For n ≥ 2, the Fundamental Theorem of Arithmetic says this type is contractible (essentially one element). So it *is* a proposition, and a proof-relevant type would be unnecessary.

But consider the type of *all paths* from x to y in a graph. This is not a proposition — two different paths are genuinely different, and the specific path matters for further computations (e.g., it has a length, a sequence of edges).

The general principle: use propositions when you want to express *existence without choice* of a specific witness; use proof-relevant types when the specific witness matters.

## Propositional Truncation Preview

Sometimes you have a proof-relevant type A but you want to "forget" the specific element and only remember the fact of inhabitation. The *propositional truncation* ‖A‖ does this: it is a proposition equivalent to "A is inhabited."

We define ‖A‖ precisely in Section 5 (as a higher inductive type). Here we note: the map A → ‖A‖ sends every element of A to its equivalence class, and anything proved from ‖A‖ only uses the propositional content of A.

This is the type-theoretic analog of the classical logical operation of "existential closure" — from a proof of A we get "A exists," and from "A exists" we can prove things using only the existence, not the specific witness.

## Propositions and Logic

Propositions in HoTT correspond to logical propositions in the traditional sense. The logical connectives act on propositions as follows:

| Logic | Type Theory |
|---|---|
| True | 1 (unit type) |
| False | ∅ (empty type) |
| P ∧ Q | P × Q (product) |
| P ∨ Q | ‖P + Q‖ (truncated coproduct) |
| P → Q | P → Q (function type) |
| ¬P | P → ∅ |
| ∀(x:A).P(x) | Π(x:A).P(x) |
| ∃(x:A).P(x) | ‖Σ(x:A).P(x)‖ (truncated sigma) |

The *truncations* for ∨ and ∃ are crucial: without truncation, `P + Q` is not a proposition (even if P and Q are propositions, there are two constructors inl and inr), and `Σ(x:A).P(x)` is not a proposition (it carries the specific witness x). Truncation forces these types to be propositions.

This distinction — between proof-relevant existence `Σ(x:A).P(x)` and proof-irrelevant existence `‖Σ(x:A).P(x)‖` — is one of the most important and subtle aspects of HoTT. It is the formal expression of the difference between "there exists a witness, and I can give you one" versus "there exists a witness, but I cannot necessarily give you one."

## isProp is a Proposition

A key lemma:

**Theorem.** isProp(isProp(A)).

*Proof.* Suppose f, g : isProp(A). We need f = g (as elements of Π(a b:A).(a=b)). By funext (twice, for the two arguments), it suffices to show f(a)(b) = g(a)(b) for all a, b : A.

Since isProp(A) is assumed (either from f or g — they both say A has at most one element), the type a = b is contractible whenever a = b holds, or empty otherwise. Either way, f(a)(b) and g(a)(b) are elements of the same type, and if that type is a proposition (which it is, since propositions form a sub-hierarchy), they are equal. ∎

More carefully: since A is a proposition (f or g witnesses this), for any a, b : A, the path type a = b is either empty or contractible. In the contractible case, f(a)(b) = g(a)(b) because both are elements of a contractible type. ∎

## Subsingleton Elimination

**Theorem (Subsingleton elimination).** If A is a proposition and B : A → Type with each B(a) a proposition, then `Π(a:A).B(a)` is a proposition.

*Proof.* Given f, g : Π(a:A).B(a). By funext, need f(a) = g(a) for all a : A. Since B(a) is a proposition, any two elements are equal. ∎

This means: if you are proving a proposition from a proposition, you can freely use the hypothesis and the proof is automatically valid. This is the type-theoretic version of the logical principle that proofs of propositions don't carry information.

## Hedberg's Theorem Preview

A type A has *decidable equality* if `Π(a b:A). (a=b) + ¬(a=b)`. Hedberg's Theorem (proved in Section 3) says:

**Theorem.** Decidable equality implies the type is a set.

For propositions in particular: a proposition with decidable membership (either inhabited or not) is a set (in fact, either contractible or empty). This means all decidable propositions live at the bottom of the h-level hierarchy.

## Summary

| Property | Definition | Key examples |
|---|---|---|
| isProp(A) | Π(a b : A). a = b | ∅, 1, a=b in sets |
| In hierarchy | h-level -1 | "truth values" |
| Logic | Classical propositions | ∧, ∨ (truncated), → |
| isProp(isProp(A)) | Yes | Being a proposition is propositional |
| Closure | Π, →, ∅, 1 | Product, function types |

Propositions are the h-level -1 types. They are the types where proof does not matter — only existence. They form the "logical" layer of HoTT, the layer where the type theory connects to classical logic. Every statement in ordinary mathematics, when made precise in HoTT, lives at h-level -1 or is a set-level structure (h-level 0). Propositions are where formal proof meets informal mathematical truth.
