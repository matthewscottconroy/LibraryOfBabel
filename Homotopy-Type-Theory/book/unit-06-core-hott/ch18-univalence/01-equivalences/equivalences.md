# Equivalences

## The Problem with Naive Bijections

In set theory, a bijection between sets A and B is a function f : A → B with a two-sided inverse: there exists g : B → A with g∘f = id_A and f∘g = id_B.

In HoTT, we can state the same thing. A *quasi-equivalence* (or quasi-inverse) is a function f : A → B equipped with:
```
g : B → A
η : g ∘ f ~ id_A     (left homotopy)
ε : f ∘ g ~ id_B     (right homotopy)
```

The type of quasi-inverses for f is: `qinv(f) := Σ(g:B→A). (g∘f ~ id_A) × (f∘g ~ id_B)`

**The problem:** `qinv(f)` is *not* a proposition in general. A function can have multiple distinct quasi-inverses.

**Concrete example.** For f = id_A, the pair (id_A, refl, refl) is a quasi-inverse. But if A has a non-trivial self-homotopy H : id_A ~ id_A (a homotopy from the identity to itself that is not the trivial homotopy), then (id_A, H, refl) is also a quasi-inverse.

Why does this matter? For the Univalence Axiom to work correctly — for paths in the universe to correspond bijectively to equivalences — we need the type of equivalences to be a proposition. "Either f is an equivalence or it isn't, with no additional data." If `qinv(f)` is not a proposition, we would have "f is an equivalence in multiple different ways," and the correspondence would fail.

We need a notion of equivalence that is propositional.

## Definition 1: Bi-Invertible Maps

The simplest fix: require a left inverse and a right inverse separately.

**Definition.** f : A → B is *bi-invertible* if:
```
isBiInv(f) := (Σ(g:B→A). g∘f ~ id_A) × (Σ(h:B→A). f∘h ~ id_B)
```

The left inverse g and right inverse h need not be equal.

**Theorem.** `isBiInv(f)` is a proposition.

*Key argument.* Suppose f has a right inverse h (so f∘h ~ id_B). For any left inverse g: g = g∘id_B = g∘(f∘h) = (g∘f)∘h = id_A∘h = h. So the left inverse is uniquely determined by the right inverse. Therefore the type of left inverses, given a right inverse, is a proposition (in fact, contractible). Symmetric argument for right inverses given a left inverse. So isBiInv(f) is a product of two contractible types (once both inverses exist), hence contractible, hence a proposition. ∎

## Definition 2: Half-Adjoint Equivalences

An alternative: keep one inverse but add a coherence condition.

**Definition.** f : A → B is a *half-adjoint equivalence* (HAE) if:
```
isHAE(f) := Σ(g:B→A). Σ(η: g∘f ~ id_A). Σ(ε: f∘g ~ id_B). Π(x:A). ap_f(η(x)) = ε(f(x))
```

The coherence `τ : Π(x:A). ap_f(η(x)) = ε(f(x))` says: the two ways to show f∘g∘f ~ f agree. Path 1: apply f to the left homotopy η_x (getting ap_f(η(x)) : f(g(f(x))) = f(x)). Path 2: evaluate the right homotopy ε at f(x) (getting ε(f(x)) : f(g(f(x))) = f(x)).

**Theorem.** `isHAE(f)` is a proposition.

The coherence condition pins down the relationship between η and ε. Without it, different (η, ε) pairs can be non-equal. With it, the coherence τ is uniquely determined by g, η, and ε.

## Definition 3: Contractible Fibers

The most elegant definition:

**Definition.** The *fiber* of f : A → B over y : B is:
```
fib_f(y) := Σ(x:A). f(x) = y
```

**Definition.** f is an *equivalence* if all fibers are contractible:
```
isEquiv(f) := Π(y:B). isContr(fib_f(y))
```

**Intuition.** f is an equivalence iff every y : B has exactly one preimage. "Exactly one" = contractible fiber.

**Theorem.** `isEquiv(f)` is a proposition.

*Proof.* `isEquiv(f) = Π(y:B). isContr(fib_f(y))`. This is a product of propositions (isContr is a proposition by Theorem 1.9 of Chapter 17). A product of propositions is a proposition. ∎

## All Three Definitions Are Equivalent

**Theorem.** For any f : A → B:
```
isBiInv(f)  ≃  isHAE(f)  ≃  isEquiv(f)
```

Since all three are propositions, any logical implication gives an equivalence. The proof consists of showing:

1. **isEquiv ⇒ isBiInv:** From contractible fibers, extract the center (g(y), ε_y) of each fib_f(y) to get a right inverse g with homotopy ε. For the left inverse: the center of fib_f(f(x)) is (g(f(x)), ε(f(x))), but (x, refl) is also in fib_f(f(x)), so by contractibility g(f(x)) = x (up to a specific path η(x)). This gives g as a left inverse with homotopy η.

2. **isBiInv ⇒ qinv ⇒ isHAE:** A left and right inverse can be merged: given left inverse g with η and right inverse h with ε, then g = g∘f∘h ~ id_B∘h = h. So g = h, and we have a single quasi-inverse. The coherence τ is constructed by the zig-zag: ap_f(η(x)) and ε(f(x)) are two paths in fib_{f∘g∘f}(f(x)), and since the fiber is contractible, they are equal.

3. **isHAE ⇒ isEquiv:** Given (g, η, ε, τ), show the fiber fib_f(y) is contractible. The center is (g(y), ε(y)). For any (x, p) : fib_f(y), construct the contracting path using η and the coherence τ.

## The Type of Equivalences

**Definition.** The *type of equivalences* from A to B is:
```
A ≃ B := Σ(f:A→B). isEquiv(f)
```

An element (f, e) : A ≃ B is a function together with a proof that it is an equivalence.

**Key properties:**
- **Reflexivity:** The identity id_A : A ≃ A (its fibers fib_{id}(a) = Σ(x:A).(x=a) are contractible, as we proved in Chapter 17).
- **Symmetry:** If e : A ≃ B, then e⁻¹ : B ≃ A (using the inverse function from the equivalence structure).
- **Transitivity:** If e₁ : A ≃ B and e₂ : B ≃ C, then e₂ ∘ e₁ : A ≃ C.
- **Two-out-of-three:** In a composable triple A → B → C, if any two of the three maps are equivalences, so is the third.

## Why Propositionality Matters for Univalence

The Univalence Axiom says that the map `idToEquiv : (A=B) → (A≃B)` is an equivalence. For this to make sense — for the map to be an equivalence and not merely a quasi-equivalence — the type A≃B must have the right size.

If `isEquiv(f)` were not a proposition, then A≃B would have "too many elements" — different elements with the same underlying function but different equivalence data. The map idToEquiv would then be many-to-one, not one-to-one, and the correspondence would fail.

The propositionality of isEquiv is the technical fact that makes the Univalence Axiom coherent.

## Automorphisms and Symmetry Groups

The *automorphism type* of A is:
```
Aut(A) := A ≃ A
```

This is the type of self-equivalences of A. As a group under composition, it is the symmetry group of A.

By Univalence (next section), `Aut(A) ≃ (A = A)` (the loop space of the universe at A). So the symmetry group of A is the loop space of the universe at A.

Examples:
- Aut(0) = 1 (only the identity)
- Aut(1) = 1 (only the identity)
- Aut(Bool) = Z/2Z (identity and swap)
- Aut(Fin(n)) ≃ Sₙ (the symmetric group on n elements)
- Aut(Z) is countably infinite (one self-equivalence for each bijection Z → Z)

## Summary

| Definition | Type | Proposition? | Key property |
|---|---|---|---|
| qinv(f) | Σ(g:B→A).(g∘f~id)×(f∘g~id) | No | Simple but wrong |
| isBiInv(f) | (left inv) × (right inv) | Yes | Separate inverses |
| isHAE(f) | Σ(g,η,ε). coherence | Yes | One inv + triangle |
| isEquiv(f) | Π(y:B). isContr(fib_f(y)) | Yes | Contractible fibers |

All three correct definitions are logically equivalent and propositions. We prefer `isEquiv` for its elegance and its direct connection to contractibility (the foundational notion of Chapter 17).
