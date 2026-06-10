# Exercises: Univalence

## Equivalence Exercises

**Exercise 1.** Show that the identity function id_A : A → A is an equivalence. Give an explicit proof using the contractible-fibers definition: show that each fiber fib_{id}(a) = Σ(x:A).(x=a) is contractible.

**Exercise 2.** Show that equivalences are closed under composition: if e₁ : A ≃ B and e₂ : B ≃ C, then e₂ ∘ e₁ : A ≃ C. Use the contractible-fibers definition.

**Exercise 3.** Show that any equivalence e : A ≃ B has an inverse equivalence e⁻¹ : B ≃ A. Construct the inverse explicitly from the contractible-fibers structure of e.

**Exercise 4.** Prove the two-out-of-three property: in a composable triple A → B → C, if any two of the three maps are equivalences, so is the third.

**Exercise 5.** Show that the three definitions of equivalence (bi-invertible, half-adjoint, contractible fibers) are logically equivalent. At least show one direction: isEquiv(f) → isBiInv(f).

**Exercise 6.** Show that isBiInv(f) is a proposition (any two proofs that f is bi-invertible are equal). Follow the argument given in the section: use the existence of a right inverse to show the type of left inverses is contractible.

## Univalence Exercises

**Exercise 7.** Define `idToEquiv : (A = B) → (A ≃ B)` explicitly using the J rule. Show the computation rule: `idToEquiv(refl_A) = (id_A, isEquiv-id_A)`.

**Exercise 8.** Using the Univalence Axiom and the computation rule for ua, show that:
```
transport^{id}(ua(e), x) = fun(e)(x)
```
for any equivalence e : A ≃ B and x : A. (This is direct from the definitions — trace through the definitions carefully.)

**Exercise 9.** Use Univalence to prove propositional extensionality: for propositions P and Q, `(P ↔ Q) → (P = Q)`. Give the explicit argument showing P ≃ Q from P ↔ Q when P and Q are propositions.

**Exercise 10.** Show that the Univalence Axiom implies function extensionality, using the interval I (defined as the HIT with points 0, 1 and path seg : 0=1). The key steps: construct k : A → I → B from the homotopy H : f ~ g, curry to get k̃ : I → (A→B), then apply ap_{k̃}(seg).

## Computing with Univalence

**Exercise 11.** Show that `(Bool = Bool)` has exactly two elements: `refl_{Bool}` and `ua(swap)`. The key step: show that `Bool ≃ Bool` has exactly two elements (id and swap).

**Exercise 12.** Show that transport along `ua(swap) : Bool = Bool` is the swap function: `transport^{id}(ua(swap))(true) = false`.

**Exercise 13.** Compute `Aut(Fin(2))` = the type of self-equivalences of Fin(2) = {0,1}. How many elements does it have? What group does it form?

**Exercise 14.** Show that `(N = N) ≃ Aut(N)` (the type of bijections from N to itself). Is this type a set? What is its cardinality?

**Exercise 15.** For propositions P and Q, show that `(P = Q) ≃ (P ↔ Q)`. This is propositional extensionality stated as an equivalence.

## Structure Invariance Exercises

**Exercise 16.** Define the type of groups: `Group := Σ(G:Type). isGroup(G)` where `isGroup` records the group operations and axioms. Show that an equivalence of groups (group isomorphism) corresponds to a path in Group, using the Sigma-path characterization and Univalence.

**Exercise 17 (Proof-Level).** Use structure invariance to show: if A is a set and B ≃ A, then B is a set. (This should be trivial using transport, but trace through the argument explicitly.)

**Exercise 18 (Proof-Level).** Prove the "Univalence Principle": for any type-theoretically definable predicate P : Type → Type and any equivalence e : A ≃ B, the function `transport^P(ua(e)) : P(A) → P(B)` is an equivalence.

**Exercise 19 (Proof-Level).** Show that the universe of propositions `Prop = Σ(A:Type).isProp(A)` is a set, using propositional extensionality and the fact that equivalences between propositions are propositions.

**Exercise 20 (Proof-Level).** Show that the universe of sets `Set = Σ(A:Type).isSet(A)` is a groupoid (h-level 1). The key step: show that paths between sets correspond to bijections, and the type of bijections between two sets is itself a set.

## Proof-Level and Challenge Exercises

**Exercise 21 (Proof-Level).** Show that `(A ≃ B) ≃ (B ≃ A)` — the type of equivalences is symmetric. Give an explicit construction of the inverse equivalence and verify it is indeed an inverse.

**Exercise 22 (Proof-Level).** The *univalence for propositions*: for propositions P and Q, `(P = Q) ≃ (P ↔ Q)`. Prove this as an equivalence, with explicit functions in both directions and their composition laws.

**Exercise 23 (Proof-Level).** Using Univalence, prove that there is no path between Bool and N in the universe (i.e., Bool ≠ N). The key: Bool has exactly 2 elements, N has infinitely many, and an equivalence would give a bijection, contradicting finiteness/infiniteness. Make this argument precise using the fact that Fin(2) ≃ Bool but Fin(2) is not equivalent to N.

**Exercise 24 (Proof-Level).** Show that quasi-inverses are *not* propositions: find a type A and a function f : A → A such that `qinv(f)` has more than one element. (Hint: use A = S^1 and f = id_{S^1}, noting that id_{S^1} has the trivial quasi-inverse (id, refl, refl) but also the quasi-inverse (id, loop, refl) or some similar variant using the loop.)

**Exercise 25 (Challenge).** Prove that `Aut(Fin(n)) ≃ Sₙ` — the automorphism group of Fin(n) is the symmetric group on n elements. Give an explicit bijection between self-equivalences of Fin(n) and permutations of {1,...,n}.

**Exercise 26 (Challenge).** The *univalence for small types*: suppose U is a universe closed under all type formers (products, sums, pi-types, sigma-types) and satisfying the univalence axiom. Show that for any two types A, B : U, the type `A =_U B` is equivalent to `A ≃ B`, even when A and B are large (in a larger universe). (This requires careful universe-polymorphism arguments.)

**Exercise 27 (Challenge).** Show that in cubical type theory, the Univalence Axiom follows from the Kan operations and the computation rules for the interval. Specifically: describe how `ua(e)` would be defined using a path in the universe I → Type, and how its computation rule would follow from the Kan filling operation.
