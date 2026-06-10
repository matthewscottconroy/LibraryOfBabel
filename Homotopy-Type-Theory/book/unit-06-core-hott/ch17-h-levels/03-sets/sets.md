# Sets

## Where Classical Mathematics Lives

Most of everyday mathematics happens in sets. The natural numbers form a set. The real numbers form a set. A group is a set with extra structure. A topological space is a set with a topology. Linear algebra, combinatorics, number theory, analysis — all of these work primarily with sets.

In HoTT, a *set* is a type at h-level 0: a type where equality is propositional. Any two elements may be equal or not, but when they are equal, there is at most one proof of their equality. The path type `a = b` is either empty (a and b are unequal) or contractible (a and b are equal in the unique way they can be).

This is exactly the behavior of equality in ordinary mathematics. Two natural numbers are either equal or not. Two sets are either equal or not. The proof of equality carries no additional information — we care only whether equality holds, not in which of potentially many ways it holds.

## The Definition

**Definition.** A type A is a *set* (or *h-set* or *0-type*) if all path types are propositions:

```
isSet(A) := Π(a b : A). isProp(a = b)
```

Equivalently: for all a, b : A and all paths p, q : a = b, we have p = q. Paths between the same two points are all equal. There is at most one path.

This property is also known as *Uniqueness of Identity Proofs (UIP)*. In classical type theory, UIP was sometimes taken as an axiom (Axiom K). In HoTT, UIP is neither provable (because there are types where it fails) nor an axiom (because HoTT wants non-trivial path structure). Instead, UIP is the defining property of sets, and most types of ordinary mathematics happen to be sets.

## Natural Numbers are a Set

The most important example:

**Theorem.** isSet(N).

The proof proceeds via Hedberg's theorem (below). Natural numbers have decidable equality: for any m, n : N, we can decide whether m = n. This decidability, plus a technical argument, implies isSet(N).

Alternatively, one can prove directly that N is a set by showing that the type `m = n` (for natural numbers m, n) is either empty or a proposition. The key steps:

1. Define `code : N → N → Type` by:
   - code(0, 0) = 1
   - code(S(m), S(n)) = code(m, n)
   - code(0, S(n)) = ∅
   - code(S(m), 0) = ∅

2. Show there is an equivalence `(m = n) ≃ code(m, n)`.

3. Since code(m, n) is either 1 (contractible, when m = n numerically) or ∅ (empty, when m ≠ n), the path type m = n is either contractible or empty — hence a proposition.

4. Since all path types are propositions, N is a set. ∎

## Hedberg's Theorem

The key theorem connecting decidability to being a set:

**Theorem (Hedberg).** If A has decidable equality — that is, `Π(a b : A). (a = b) + ¬(a = b)` — then A is a set.

*Proof sketch.* The key idea: decidable equality provides a function `eq? : A → A → Bool` (or more precisely, a function to the coproduct (a=b) + ¬(a=b)). From this, one can construct a "canonical" representative of each path: a function f that takes any path p : a = b to a *specific* path `f(p) : a = b` such that f(p) = f(q) for any two paths p, q : a = b.

Once we have such a function f with f(p) = f(q) for all p, q, we can derive p = q:

```
p = refl · p = f(refl)⁻¹ · f(refl) · p = f(refl)⁻¹ · f(p) = f(refl)⁻¹ · f(q) = q
```

(with appropriate uses of naturality). ∎

**Corollary.** N, Z, Q, Bool, Fin(n) for any n — all have decidable equality, hence all are sets.

**Corollary.** Any type with a decidable equality function (like types with a `decEq` instance in Agda or Lean) is a set.

## Sets Form a Category

The collection of sets in HoTT — the "category of sets" — behaves exactly like the category Set of classical mathematics.

- **Morphisms:** Functions f : A → B between sets.
- **Isomorphisms:** Bijections — functions with set-theoretic inverses. (By Hedberg, these coincide with equivalences of the underlying types.)
- **Limits:** Products A × B, equalizers Σ(a:A).f(a)=g(a), pullbacks.
- **Colimits:** Coproducts A + B (already sets if A, B are sets), coequalizers (as set truncations of pushouts).
- **Subobjects:** For f : A → B with B a set, the fibers fib_f(b) are sets, and the image is a set.

**Key theorem:** The collection of sets in HoTT satisfies the axioms of an elementary topos (with the propositional truncation playing the role of the power object). This means classical mathematics, formalized in HoTT, works exactly as expected.

## isSet is a Proposition

**Theorem.** isProp(isSet(A)).

*Proof.* isSet(A) = Π(a b : A). isProp(a = b). A product of propositions is a proposition (since isProp(isProp(-)) from Chapter 17, Section 2). ∎

## UIP and its Independence

The *Uniqueness of Identity Proofs* axiom (UIP, also called Axiom K in Martin-Lof's original system) states:

```
Π(A : Type). Π(a : A). Π(p : a = a). p = refl_a
```

That is, the only loop at any point is the trivial loop. This implies every type is a set.

**Theorem.** UIP is independent of the Martin-Lof type theory without it. There are models where UIP holds (set-theoretic models) and models where it fails (simplicial set model, where S^1 has non-trivial loops).

This independence is the content of the Hofmann-Streicher groupoid model. The key example: in the simplicial set model, the circle S^1 (as a HIT, Chapter 19) has `base = base ≃ Z`. The type `loop = refl_base` is empty — the loop is not equal to the trivial loop. So UIP fails for S^1.

In HoTT without UIP (the standard setting), being a set is a *condition* on a type, not the default. Most types of ordinary mathematics happen to satisfy this condition, but higher types (circles, spheres, the universe) do not.

## Examples: Sets and Non-Sets

**Sets** (isSet holds):
- All inductive types: N, Bool, Fin(n), lists, trees (with decidable equality on elements)
- Z, Q, R (the last two require care — R is a set but R-as-Cauchy-sequences requires quotient)
- Any type with decidable equality (by Hedberg)
- Propositions (props are sets — in fact, they are at lower h-level)

**Non-sets** (isSet fails):
- The circle S^1: `base = base ≃ Z` has infinitely many elements
- Any sphere S^n for n ≥ 1
- The universe Type: `A = B ≃ A ≃ B` can have many elements
- The interval I (as a HIT): `0 = 1` has exactly one element (seg), but the type is contractible so technically it is a set — wait, a contractible type is a set (all path types are contractible hence propositions). So I is a set. But I is interesting for other reasons.

The general rule: any type defined by ordinary inductive types (without higher path constructors) is a set. Any type with higher inductive constructors (path constructors, homotopy constructors) may have non-trivial path structure and may fail to be a set.

## Summary

| Property | Definition | Key examples |
|---|---|---|
| isSet(A) | Π(a b:A). isProp(a=b) | N, Z, Bool, finite types |
| Equivalent to | UIP holds for A | — |
| In hierarchy | h-level 0 | "mathematical sets" |
| Hedberg | Decidable eq. ⇒ set | All discrete types |
| Non-examples | Circles, spheres, Type | Higher homotopy types |

Sets are the home of classical mathematics in HoTT. The h-level 0 condition — that equality is a proposition — is exactly the condition that makes a type behave like a classical set. At this level, HoTT agrees with classical mathematics. Above this level, it transcends it.
