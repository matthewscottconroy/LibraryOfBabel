# Transport and ap

## The Problem of Dependent Types

Equality in simple type theory is easy. If f : A → B and a = b in A, then f(a) = f(b) in B. Leibniz's law. Done.

Equality in dependent type theory is harder. Suppose we have a type family P : A → Type. If a = b in A, then P(a) and P(b) are different types. They are equal types — the path a = b induces an identification of P(a) with P(b). But an element x : P(a) lives in P(a), not in P(b). If we want to compare x with an element y : P(b), we must first move x from P(a) to P(b), or move y from P(b) to P(a).

Transport is the operation that does this moving.

## Transport Along Paths

**Definition.** For any type family P : A → Type and any path p : a =_A b, the *transport function* is:

```
transport^P(p) : P(a) → P(b)
```

It moves elements of P(a) to elements of P(b), using the path p as a guide.

**Construction by J.** Apply J to p with:

```
Motive: C(b, p) = (P(a) → P(b))
Base case: C(a, refl_a) = (P(a) → P(a)), supplied by id_{P(a)}
```

So: `transport^P(refl_a, x) ≡ x` for all x : P(a) — definitional equality.

Transport along the trivial path is the identity. The path a = a introduces no change.

**Transport is an equivalence.** The function transport^P(p) : P(a) → P(b) has an inverse: transport^P(p⁻¹) : P(b) → P(a). The round trips:

```
transport^P(p⁻¹) ∘ transport^P(p) ~ id_{P(a)}
transport^P(p) ∘ transport^P(p⁻¹) ~ id_{P(b)}
```

Both proved by J on p (reducing to the reflexivity case where everything is the identity). So transport is an equivalence — an isomorphism of types.

## Transport as Parallel Transport

The name "transport" is not arbitrary. In differential geometry, *parallel transport* along a curve γ : [0,1] → M moves tangent vectors from T_{γ(0)}M to T_{γ(1)}M along the curve, preserving the connection structure.

In HoTT, a type family P : A → Type is a fibration — a family of "fibers" over the base space A. A path p : a = b in the base lifts to an isomorphism of fibers transport^P(p) : P(a) → P(b). This is parallel transport: a canonical way to identify fibers that lie over connected points.

The analogy is exact in the simplicial set model. A type family over A is literally a fibration of simplicial sets, and transport is the path-lifting function that every fibration provides.

## Computing Transport

Transport in specific type families can be computed explicitly.

**Constant family.** If P(x) = B for all x (constant family), then transport^P(p, b) = b — transport does nothing, since the fiber doesn't change.

**Path family.** If P(x) = (a = x) for fixed a, then:

```
transport^{a=(-)}(p, q) = q · p
```

Transport in the right-hand path family concatenates on the right. Verification: at p = refl_b, we get q · refl_b = q, which matches the computation rule.

**Product family.** If P(x) = Q(x) × R(x), then:

```
transport^{Q×R}(p, (q, r)) = (transport^Q(p, q), transport^R(p, r))
```

Transport in a product is transport in each component.

**Function family.** If P(x) = (Q(x) → R(x)), then:

```
transport^{Q→R}(p, f) = λy. transport^R(p, f(transport^Q(p⁻¹, y)))
```

To transport a function: go backward in Q, apply f, go forward in R. This is the covariant-contravariant transport: functions are contravariant in their domain and covariant in their codomain.

**The Sigma family.** If P(x) = Σ(y : Q(x)). R(x, y), then transport is more complex — it involves transporting both the first and second components, with the transport of the second component depending on the transport of the first.

## The Functorial Action: ap

For a non-dependent function f : A → B and a path p : a = b in A, we want a path from f(a) to f(b) in B. This is the *action on paths*, written ap_f(p).

**Definition.** For f : A → B and p : a =_A b:

```
ap_f(p) : f(a) =_B f(b)
```

**Construction by J.** Motive: C(b, p) = (f(a) = f(b)). Base case: f(a) = f(a), supplied by refl_{f(a)}.

**Computation rule:** `ap_f(refl_a) ≡ refl_{f(a)}`.

## ap is a Functor

The map ap_f : (a = b) → (f(a) = f(b)) is not just a function — it respects the groupoid structure. That is, ap_f is a groupoid homomorphism:

**ap preserves composition.** `ap_f(p · q) = ap_f(p) · ap_f(q)`

*Proof.* By J on p, then J on q. Reduces to `ap_f(refl · refl) = ap_f(refl) · ap_f(refl)`, i.e., `refl = refl · refl`. True by right unit. ∎

**ap preserves inverses.** `ap_f(p⁻¹) = (ap_f(p))⁻¹`

**ap of identity.** `ap_{id_A}(p) = p`

**ap of composition.** `ap_{g∘f}(p) = ap_g(ap_f(p))`

These four laws say exactly that ap_f is a morphism of groupoids. Every function in HoTT automatically preserves path structure. There is no need to prove that a function is "continuous" — the type theory enforces it. Every function acts on paths, inverses, and compositions in the correct way.

This is a remarkable consequence of the J rule. The classicial theorem that "every continuous function between topological spaces induces a homomorphism of fundamental groups" becomes, in HoTT, a trivial consequence of the rules: ap_{π₁(f)} = π₁(ap_f).

## The Dependent Action: apd

For a *dependent* function f : Π(x : A). P(x) and a path p : a = b in A, we cannot simply state `f(a) = f(b)` — these live in different fibers P(a) and P(b). We need to transport f(a) to P(b) first.

**Definition.** For f : Π(x : A). P(x) and p : a = b:

```
apd_f(p) : transport^P(p, f(a)) =_{P(b)} f(b)
```

This says: the transported value of f at a equals the value of f at b.

**Construction by J.** Motive: C(b, p) = (transport^P(p, f(a)) = f(b)). Base case: transport^P(refl_a, f(a)) = f(a), which holds by the computation rule for transport.

**Computation rule:** `apd_f(refl_a) ≡ refl_{f(a)}`.

**Interpretation.** For a section f of a fibration P → A, the fact that f is a section means it is compatible with parallel transport: transporting f(a) along any path p gives f(b). The dependent action apd_f(p) is the specific path witnessing this compatibility.

In the simplicial model: a section of a fibration commutes with path-lifting. The apd of a section is the coherence between the two ways to go from the fiber over a to the fiber over b — either use the section then note it's constant, or transport directly.

## Homotopies are Natural Transformations

A homotopy between two functions f, g : A → B is a family of paths:

```
H : Π(a : A). f(a) =_B g(a)
```

This is the type-theoretic definition of "pointwise equality." If f = g (the functions are equal), then certainly f ~ g (they are pointwise equal) — this direction follows by applying ap to the equality.

But homotopies carry more structure than mere pointwise equality. They are *natural transformations* in the categorical sense.

**Theorem (Naturality of homotopies).** For H : f ~ g and p : a = b:

```
ap_g(p) · H(b) = H(a) · ap_f(p)
```

This commutation says: it does not matter whether you first apply the homotopy at a and then follow g's action on p, or first follow f's action on p and then apply the homotopy at b. The two paths from f(a) to g(b) agree.

*Proof.* By J on p. Reduces to `ap_g(refl) · H(a) = H(a) · ap_f(refl)`, i.e., `refl · H(a) = H(a) · refl`. Both sides equal H(a) by the unit laws. ∎

This naturality square is the type-theoretic version of the standard categorical naturality condition. Every homotopy is natural — automatically. There is no need to verify naturality; it follows from the J rule.

## The Two Fundamental Operations

Transport and ap are the two fundamental ways paths interact with the rest of type theory.

**Transport** moves elements along paths in dependent types. It is the "vertical" operation — it moves along the fiber direction of a fibration.

**ap** acts on paths by applying a function. It is the "horizontal" operation — it moves along the base of a fibration.

Together, they ensure that the entire type-theoretic universe is homotopy-coherent. Every type family behaves like a fibration. Every function is continuous. Every proof is homotopy-invariant.

This coherence is the formal content of what Voevodsky meant when he said that homotopy type theory provides a "foundation based on the principle that all identity proofs are paths." Transport and ap are the mechanisms that make this principle operative everywhere in the theory.

## Summary

| Concept | Type | Key property |
|---|---|---|
| transport^P(p) | P(a) → P(b) | Equivalence; computes on refl as id |
| ap_f(p) | f(a) = f(b) | Functor; computes on refl as refl |
| apd_f(p) | transport(p, f(a)) = f(b) | Dependent ap; witnesses section condition |
| Homotopy H | Π(a). f(a) = g(a) | Natural transformation |

The path lifting property — that transport always exists — is the type-theoretic Kan condition. The functoriality of ap is automatic continuity. The naturality of homotopies is the fundamental coherence that makes mathematics in HoTT well-behaved.
