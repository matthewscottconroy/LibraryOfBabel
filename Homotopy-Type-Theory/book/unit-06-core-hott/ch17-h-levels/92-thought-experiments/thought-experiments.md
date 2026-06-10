# Thought Experiments: H-Levels and Truncations

## Thought Experiment 1: The Complexity Meter

Imagine a device that measures the "complexity of equality" for a type. For the natural numbers, the meter reads 0 — equality is fully discrete, a proposition. For the circle, the meter reads ∞ — there are infinitely many distinct loops, and loops between loops, and so on.

*Question:* Where would you expect the following types to fall on this complexity meter?

(a) The integers Z.
(b) The group of symmetries of the square (the dihedral group D₄).
(c) The type of all groups (as a type in HoTT, using the Univalence Axiom to identify isomorphic groups).
(d) The type of all continuous functions [0,1] → R.
(e) The type of all topological spaces (up to homeomorphism).

For each, consider: is the type a proposition? A set? A groupoid? Something higher?

## Thought Experiment 2: The Archivist's Dilemma

An archivist is cataloguing mathematical proofs. She has two proofs that 2 + 2 = 4: one by Peano induction, one by direct calculation. She has two proofs that π is irrational: one by Niven's argument, one by Hermite's original method.

In classical mathematics, she catalogs these as "two proofs of the same theorem" — same theorem, different proofs, but the theorem is what matters.

In HoTT, the situation is nuanced:
- For 2 + 2 = 4 (a statement about natural numbers, which form a set): the two proofs are equal as elements of the identity type. The type N is a set, so there is at most one proof of any equality. The archivist's two "different proofs" are actually the same element of the identity type.
- For a statement about types (say, "the circle is equivalent to...something"): the proofs may be genuinely different elements of the equivalence type. The type of types is not a set — it's at least a groupoid.

*Question:* What does it mean for two mathematical proofs to be "the same proof" in HoTT? Give a precise condition. (Hint: they are the same iff they are equal as elements of the relevant identity type, which depends on the h-level of the type involved.)

## Thought Experiment 3: The Proposition/Set Boundary

Consider the following types and determine whether each is a proposition (at most one element up to equality) or requires being a set (many elements, but each path type is a proposition):

(a) `isPrime(n)` for a specific natural number n.
(b) `Σ(n : N). isPrime(n)` — the type of prime numbers.
(c) `n < m` for specific n, m : N (where `<` is defined as Σ(k:N). n + S(k) = m).
(d) `f is injective` for f : A → B.
(e) `G is abelian` for a group G.

For each, also ask: if the type is a proposition, could it have been defined in a way that makes it proof-relevant? And if so, what would the additional proof-data mean?

## Thought Experiment 4: Collapsing Information

Propositional truncation ‖A‖ "forgets" the specific element of A and only remembers that A is inhabited. But what does this mean computationally?

Consider A = Z (the integers). The propositional truncation ‖Z‖ = 1 (contractible — Z is inhabited). Now suppose you are given an element of ‖Z‖ and you want to define a function ‖Z‖ → B. By the universal property, you can only define this if B is a proposition. You cannot extract the specific integer.

*Question:* Here is a function that seems to be defined on ‖Z‖: "given that Z is inhabited, Z is inhabited." This is the identity function on ‖Z‖, which is a proposition. But can you define: "given that Z is inhabited, the element +3 : Z"? Why or why not? What precisely goes wrong?

## Thought Experiment 5: The Non-Set Universe

The universe Type is not a set, by Univalence: the path type Bool =_{Type} Bool has exactly two elements (the identity equivalence and the swap equivalence). So Bool =_{Type} Bool is not a proposition — it has two distinct elements.

*Question:* Does this mean we cannot reason about equality of types? Or does it mean we must reason more carefully? In particular:

(a) Can we say "Bool = Bool" in HoTT? Yes. But what does this statement mean? It means we have a path Bool = Bool — and there are two such paths.
(b) Can we say "Bool = Bool is a proposition"? No — it has two distinct elements.
(c) What is the "correct" statement of "Bool and Bool are the same type"? (Hint: it should be a proposition. Think about what information you would lose if you forgot which of the two paths you used.)

## Thought Experiment 6: Mere Existence vs. Witnessed Existence

In analysis, the Intermediate Value Theorem states: if f : [0,1] → R is continuous, f(0) < 0, and f(1) > 0, then there exists x ∈ [0,1] with f(x) = 0.

In classical analysis, this gives us a zero of f. In constructive analysis (without choice), this only gives us the *mere existence* of a zero — we may not be able to find it.

In HoTT, the IVT (if formalized) would give: `‖Σ(x:[0,1]). f(x) = 0‖` — the propositional truncation of the type of zeros.

*Question:* Under what additional assumptions can we "extract" a specific zero from this truncated existence statement? (Hint: the law of excluded middle, or some form of choice, might be relevant. Alternatively, if f has finitely many zeros, we can perhaps find the leftmost one constructively.)
