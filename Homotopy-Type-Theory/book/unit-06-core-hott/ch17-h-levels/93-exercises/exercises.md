# Exercises: H-Levels and Truncations

## Contractibility Exercises

**Exercise 1.** Show that if A and B are both contractible, then A × B is contractible. Find the center of contraction and the contracting homotopy explicitly.

**Exercise 2.** Show that if A is contractible and P : A → Type, then `Σ(a:A).P(a) ≃ P(c)` where c is the center of contraction of A. Give an explicit equivalence.

**Exercise 3.** Prove that if P : A → Type and each P(a) is contractible, then `Π(a:A).P(a)` is contractible. What is its center?

**Exercise 4.** Let f : A → B be a function. Show: if B is contractible and all fibers `fib_f(b) = Σ(a:A).(f(a)=b)` are contractible, then A is contractible.

**Exercise 5.** Show that `isContr(A)` is a proposition (any two elements of `isContr(A)` are equal). This requires knowing that path types in a contractible type are contractible.

## Proposition Exercises

**Exercise 6.** Show that `isProp(A)` implies that A is contractible iff A is inhabited.

**Exercise 7.** Prove that if A and B are propositions, then `A × B`, `A → B`, and `A → ∅` are propositions.

**Exercise 8.** Is `A + B` a proposition if A and B are propositions? What is the answer, and why? (Hint: consider A = B = 1.)

**Exercise 9.** Show that `isProp(‖A‖)` for any type A (the propositional truncation is always a proposition). You may use the path constructor `squash` directly.

**Exercise 10.** The type `‖A‖` satisfies: `‖A‖ ≃ isContr(‖A‖) + ¬‖A‖`... actually, let's be careful. Show that `‖A‖` is equivalent to: "A is merely inhabited," and that this is a proposition.

## Set Exercises

**Exercise 11.** Show that N is a set using the code family approach: define `code : N → N → Type` recursively as described in the section, show `(m = n) ≃ code(m,n)`, and conclude isSet(N).

**Exercise 12.** Show that `Bool` is a set. (Hint: true ≠ false by defining a function Bool → Type that sends true to 1 and false to ∅, then transporting.)

**Exercise 13 (Proof-Level).** Prove Hedberg's Theorem: if A has decidable equality (a function `dec : Π(a b:A). (a=b) + ¬(a=b)`), then A is a set. The key step: use `dec` to define a function `canon : Π(a b:A). (a=b) → (a=b)` that "canonicalizes" paths, then show all paths have the same canonical form.

**Exercise 14.** Show that if A is a set and B : A → Type is a family of sets, then `Σ(a:A).B(a)` is a set.

**Exercise 15.** Show that if B is a set, then `A → B` is a set (using funext). What about `Π(a:A).B(a)` when B : A → Type is a family of sets?

## H-Level Exercises

**Exercise 16.** Show that every proposition is a set. (The hierarchy is cumulative.) Prove it using the definition of set: all path types are propositions.

**Exercise 17.** Show that if A is an n-type, then A is an (n+1)-type. Prove this for all n ≥ -2.

**Exercise 18.** Determine the h-level of each of the following types:
(a) The empty type ∅
(b) Fin(n) for n ≥ 2
(c) Z (the integers)
(d) The type of permutations of Fin(n) — i.e., Aut(Fin(n)) ≃ Sₙ
(e) The universe Prop = Σ(A:Type).isProp(A)

**Exercise 19.** Show that the universe Prop (the type of propositions) is a set. (Two propositions are equal iff they are logically equivalent, and any two proofs of equivalence between propositions are equal.)

**Exercise 20.** Show that `isSet(A)` is a proposition. (The property of being a set is itself a proposition.)

## Truncation Exercises

**Exercise 21.** Prove the universal property of propositional truncation: for any proposition P, the map `(‖A‖ → P) → (A → P)` (given by precomposing with `|−| : A → ‖A‖`) is an equivalence.

**Exercise 22.** Prove that `‖A‖ = ‖‖A‖‖` — the propositional truncation is idempotent (applying it twice gives the same result as applying it once).

**Exercise 23 (Proof-Level).** Show that from `Π(a:A). P(a)` (a proof for all elements) we can derive `‖A‖ → ‖P‖` (if A is merely inhabited, then P is merely satisfied), where P : A → Prop. What does this say about the interaction of truncation and universal quantification?

**Exercise 24 (Proof-Level).** Show that the axiom of choice `AC = Π(A:Set). Π(B:A→Set). Π(P:Π(a:A).B(a)→Prop). (Π(a:A). ‖Σ(b:B(a)).P(a,b)‖) → ‖Σ(f:Π(a:A).B(a)). Π(a:A).P(a,f(a))‖` is *not* provable from the HoTT axioms alone (without additional axioms). (Hint: identify a specific type-theoretic example where choice would fail in a constructive model.)

## Proof-Level Exercises

**Exercise 25 (Proof-Level).** Prove that `S^1` is NOT a set. (Hint: show that the loop space `base =_{S^1} base` has at least two distinct elements, namely `refl_base` and `loop`. To show `loop ≠ refl_base`, transport along a family that assigns different values to 0 and 1 in Z, using the code family from Chapter 20.)

**Exercise 26 (Proof-Level).** Let G be a group. Define the *delooping* BG as the HIT with one point pt and for each g : G a path g-loop : pt = pt, with the group operation encoded as path composition. Show that BG is a 1-type (groupoid): all path types are sets.

**Exercise 27 (Proof-Level).** Prove that S^n is not a set for n ≥ 1. (Use the fact that π₁(S^n) = 0 for n ≥ 2 and π₁(S^1) = Z ≠ 0, showing that the loop space of S^n is nontrivial.)

**Exercise 28 (Challenge).** Show that the set of n-types in the universe, `Type_n := Σ(A:Type). is-n-type(A)`, is an (n+1)-type. The key step: show that paths between n-types correspond to equivalences between them, and equivalences between n-types form n-types.

**Exercise 29 (Challenge).** Prove that `‖S^1‖_0 = 1` — the set-truncation of the circle is contractible (one connected component). (Hint: show that all elements of ‖S^1‖_0 are equal by using the path constructor of S^1 to generate the needed equalities in the truncation.)

**Exercise 30 (Challenge).** Prove that propositional truncation preserves coproducts up to propositional truncation: `‖A + B‖ ≃ ‖A‖ + ‖B‖`... wait, is this actually true? Investigate whether `‖A + B‖` is equivalent to `‖A‖ ∨ ‖B‖` (the propositional disjunction). (The answer depends on whether we mean propositional or proof-relevant disjunction, and the answer is subtle.)
