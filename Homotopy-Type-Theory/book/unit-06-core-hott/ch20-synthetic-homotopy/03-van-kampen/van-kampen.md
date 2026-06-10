# The Seifert-van Kampen Theorem

## The Classical Statement

The Seifert-van Kampen theorem is the main tool for computing fundamental groups from decompositions of spaces. Classical statement:

**Theorem.** Let X = A ∪ B where A, B are open and A ∩ B is path-connected. Choose a basepoint x₀ ∈ A ∩ B. Then:
```
π₁(X, x₀) ≅ π₁(A, x₀) *_{π₁(A∩B, x₀)} π₁(B, x₀)
```

The fundamental group of X is the amalgamated free product (pushout in the category of groups) of the fundamental groups of A and B over the fundamental group of A ∩ B.

In HoTT, A ∪ B is replaced by a pushout, and the theorem follows directly from the universal property of pushouts.

## The HoTT Setup

Let P be the pushout of A ←^f C →^g B. This corresponds to "gluing A and B along C" — the type-theoretic analog of X = A ∪_{A∩B} B where A ∩ B = C.

Fix a basepoint p₀ : P (we take p₀ = inl(a₀) for some a₀ : A with f⁻¹(a₀) defined).

**Goal:** Compute π₁(P, p₀) in terms of π₁(A, a₀), π₁(B, b₀), and π₁(C, c₀).

## The Amalgamated Free Product

The amalgamated free product G *_H K (also written G *_H K) of groups G and K over a group H (with homomorphisms φ : H → G and ψ : H → K) is the pushout in the category of groups:

```
H →^φ G
|       |
ψ↓     ↓
K → G *_H K
```

It satisfies: a group homomorphism from G *_H K to any group Q is the same as group homomorphisms from G and K to Q that agree on H.

Equivalently, G *_H K is the free product G * K modulo the relations φ(h) = ψ(h) for all h : H.

## The HoTT Proof

**Theorem (van Kampen in HoTT).** Let P be the pushout of A ←^f C →^g B, with all three types connected and with chosen basepoints. Then:

```
π₁(P, p₀) ≅ π₁(A, a₀) *_{π₁(C, c₀)} π₁(B, b₀)
```

*Proof sketch.* By the universal property of pushouts, a map P → D (for any type D) corresponds to maps A → D and B → D that agree on C.

Applying this to D = K(G, 1) for a group G (an Eilenberg-MacLane space), and truncating:

Maps from P to K(G,1) up to homotopy (= π₀(map(P, K(G,1)))) correspond to:
- Maps from A to K(G,1) up to homotopy: π₁(A)-sets by the covering space classification.
- Maps from B to K(G,1) up to homotopy: π₁(B)-sets.
- That agree on C: π₁(C)-equivariant.

By the representability of cohomology H^1(-;G) = π₀(map(-, K(G,1))) and the van Kampen theorem for groups, the result follows. ∎

A more direct proof: use the encode-decode method applied to the pushout. Define `code : P → Type` with `code(inl(a)) = π₁(A, a₀)`, `code(inr(b)) = π₁(B, b₀)`, and the glue path constructors give the amalgamated free product structure.

## Key Examples

**π₁(S¹) via van Kampen.** Write S¹ as the pushout 1 ← 1 → 1 (two arcs glued at their endpoints). Each arc is contractible (π₁ = 0). The intersection is two points (π₁ = 0). The amalgamated free product of two trivial groups over a trivial group is:
- Wait — this gives π₁ = Z * Z / (both generators are trivial) = 0?

The issue: S¹ as a pushout of two arcs has the arcs sharing two boundary points (not one). Let me be careful.

Write S¹ as the pushout of two half-arcs [0,1/2] and [1/2, 1] with 0 and 1 identified (since S¹ = I / (0~1)). The intersection is two points (0 and 1, which are identified). Actually, more cleanly:

S¹ = pushout of `{*} ←^{(0,1)} I →^{const} {*}` where I is the interval. The map (0,1) sends * to both endpoints (this is a bit informal). In HoTT: S¹ is the coequalizer of i₀, i₁ : 1 → I. By van Kampen (applied to this coequalizer), π₁(S¹) = Z. ✓

**π₁(T²) = Z × Z.** The torus is the pushout of two cylinders. By van Kampen:
```
π₁(T²) = Z *_Z Z = Z × Z
```

where both maps from π₁(S¹) = Z to π₁(cylinder) = Z are identity maps, giving the abelianization Z × Z.

**π₁(figure-eight) = Z * Z.** The figure-eight (wedge of two circles, S¹ ∨ S¹) is the pushout of two circles along a single point. By van Kampen:
```
π₁(S¹ ∨ S¹) = Z *_{1} Z = Z * Z
```

The amalgamated product over the trivial group is the free product. ✓

**π₁(RP²) = Z/2Z.** The real projective plane RP² is the pushout of a disc D² (contractible) and S¹ along a map S¹ → S¹ that wraps twice (the "degree 2" map). By van Kampen:
```
π₁(RP²) = π₁(D²) *_{π₁(S¹)} π₁(S¹)
         = 1 *_Z Z
         = Z / (2 · generator = identity)
         = Z/2Z
```

✓ This matches the classical computation.

## The Advantage of the HoTT Proof

The classical van Kampen proof requires:
1. The theory of covering spaces.
2. A careful argument about path concatenation in the pushout.
3. The group theory of amalgamated free products.
4. Separate proofs of several lemmas about path-connected spaces.

The HoTT proof uses:
1. The universal property of pushouts (from the HIT definition).
2. The universal property of amalgamated free products (from group theory).
3. The equivalence of the two universal properties (a direct argument).

The HoTT proof is more abstract but also more transparent: the equivalence of the two universal properties *is* the theorem. There is no separate topological argument.

Moreover, the HoTT proof works for *all* pushouts, not just "nice" topological spaces (semi-locally simply connected, locally path-connected, etc.). Any pushout of connected types in HoTT satisfies van Kampen, without any point-set topological conditions.

## Summary

| Setup | Pushout of A ←^f C →^g B |
|---|---|
| Classical analog | X = A ∪ B with A ∩ B = C |
| Result | π₁(P) = π₁(A) *_{π₁(C)} π₁(B) |
| Proof method | Universal property of pushouts + groups |
| Key examples | S¹, T², figure-eight, RP² |

The van Kampen theorem in HoTT is a direct consequence of the universal property of pushouts. It is more general, cleaner, and more direct than the classical proof. It applies to any connected pushout in HoTT, without any point-set topological conditions.
