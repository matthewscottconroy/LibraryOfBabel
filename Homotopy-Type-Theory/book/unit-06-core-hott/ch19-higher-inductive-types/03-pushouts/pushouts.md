# Pushouts

## Gluing Types Together

In topology, a pushout is a "gluing" construction: given two spaces B and A with a common subspace C (and inclusions f : C → A and g : C → B), the pushout A ⊔_C B is the space obtained by gluing A and B along C. If C is a circle and A and B are discs, the pushout is a sphere. If A and B are each a line segment and C = {0,1} (two points), the pushout is a figure-eight.

Pushouts are ubiquitous in topology and category theory. They are the categorical colimit of a span A ← C → B. In HoTT, pushouts are HITs.

## The Pushout

**Definition.** The pushout of a diagram A ←^f C →^g B is the HIT P with:
- Point constructor: `inl(a) : P` for a : A
- Point constructor: `inr(b) : P` for b : B
- Path constructor: `glue(c) : inl(f(c)) = inr(g(c))` for c : C

The path constructor glue says: for each c in C, the image of c under f (in A, mapped to P via inl) is identified with the image of c under g (in B, mapped to P via inr).

**The eliminator.** To define h : P → D, specify:
- `h(inl(a)) = hA(a) : D` for a : A
- `h(inr(b)) = hB(b) : D` for b : B
- `ap_h(glue(c)) = hglue(c) : hA(f(c)) = hB(g(c))` for c : C

The condition `hglue` says: the path hA(f(c)) = hB(g(c)) in D is compatible with the gluing.

**Universal property.** The pushout P is the "universal" type receiving maps from A and B that agree on C. That is:

```
(P → D)  ≃  Σ(hA : A→D). Σ(hB : B→D). Π(c:C). hA(f(c)) = hB(g(c))
```

Maps out of the pushout are exactly pairs of maps from A and B that agree on the images of C.

## The Circle as a Pushout

S^1 is the pushout of the diagram 1 ←^{0} 1 →^{1} 1 — two copies of the unit type glued at two "points" of a single third copy.

More precisely: take A = B = 1 and C = 1, with f = λ*.* and g = λ*.* (both constant at the single element *). The pushout has:
- inl(*) : P
- inr(*) : P
- glue(*) : inl(*) = inr(*)

So P has two points and one path between them — that is the interval I, not S^1.

For S^1, use C = Bool: A = B = 1, f = λ b. * (constant), g = λ b. * (constant). Then:
- inl(*) = inr(*) =: base
- glue(true) : base = base
- glue(false) : base = base

The two gluing paths give loop₁ = glue(true) and loop₂ = glue(false). For S^1 we want one loop; we can take the pushout with C = {*} (unit type) and A = B = 1:
- C = 1, A = 1, B = 1
- Both maps f, g : 1 → 1 are the identity
- Pushout: two points (inl and inr) and one path (glue) → this is I.

For S^1 more cleanly: take the coequalizer of the two maps f, g : 1 → 1 (identity and identity) viewed as maps into the same point. Actually, S^1 is the pushout of:
```
1 ←^{id} 1 →^{id} 1   (with two distinct inclusions)
```
where the two inclusions both go to the single point but are identified as different maps. This gives one base point and one glue path (the loop).

**Cleaner construction:** S^1 is the *coequalizer* of the two inclusion maps i₀, i₁ : 1 → I (sending * to 0_I and 1_I respectively). The coequalizer identifies i₀(*) = 0_I with i₁(*) = 1_I, turning the interval's endpoints into a single point with the segment becoming a loop.

## The Torus

The torus T^2 is the pushout of two cylinders glued along their boundaries, or equivalently (by the square representation), it is the type with:
- One point: base
- Two loops: p : base = base and q : base = base
- One 2-cell: t : p · q = q · p (the relation that p and q commute)

In terms of pushouts: T^2 is the pushout of two copies of the cylinder S^1 × I along two copies of S^1.

The fundamental group of T^2 is Z × Z — computed in Chapter 20 using the van Kampen theorem applied to the pushout.

## The van Kampen Theorem

The Seifert-van Kampen Theorem is the classical theorem computing the fundamental group of a union of spaces. In HoTT, it follows directly from the universal property of pushouts.

**Theorem (van Kampen in HoTT).** Let P be the pushout of A ←^f C →^g B. If C, A, B are all connected, then:

```
π₁(P, p₀)  ≃  π₁(A, a₀) *_{π₁(C, c₀)} π₁(B, b₀)
```

The fundamental group of the pushout is the *amalgamated free product* of the fundamental groups of A and B over the fundamental group of C.

*Proof (sketch).* By the universal property of pushouts and the universal property of the amalgamated free product, both satisfy the same universal property: a group homomorphism from each side that agree on C gives a unique group homomorphism from the amalgamated product. The identification of the fundamental group of the pushout with the amalgamated product follows from showing they satisfy the same universal property. ∎

**Key examples:**
- π₁(S^1) = Z via van Kampen applied to the circle as a pushout.
- π₁(T^2) = Z × Z via van Kampen applied to the torus.
- π₁(figure-eight) = Z * Z (free product) via van Kampen applied to two circles meeting at a point.

## Coequalizers and Quotients

Coequalizers are special pushouts: given two maps f, g : A → B, the coequalizer is the pushout of B ←^f A →^g B. It identifies `f(a) = g(a)` for all a : A.

**Set quotients.** For a type A and an equivalence relation R : A → A → Prop, the *quotient type* A/R is the coequalizer of the two projections from Σ(a b:A).R(a,b) to A. The path constructor `quot-r : a = b` for each proof r : R(a,b) forces the identified elements to be literally equal in A/R.

**Examples:**
- Z/nZ: the integers Z with the relation `a ~_n b ↔ n | (a-b)`. The quotient Z/nZ has one element for each residue class, and equality between elements holds iff they are in the same class.
- The reals R as Cauchy sequences modulo the equivalence "same limit."

Quotient types as pushouts show that HITs subsume ordinary quotient types as a special case.

## Summary

| Construction | Definition | Key property |
|---|---|---|
| Pushout A ⊔_C B | inl, inr, glue | Universal property; van Kampen |
| Coequalizer | Pushout of f, g : A → B | Quotient types |
| S^1 as pushout | Coequalizer of i₀, i₁ : 1 → I | π₁ = Z |
| T^2 as pushout | Double pushout of cylinders | π₁ = Z × Z |

Pushouts are the most general HIT construction, subsuming suspension (as a pushout of B ← A → B), circle (as a coequalizer), and all other colimits. The van Kampen theorem is a direct consequence of the pushout universal property, giving HoTT a clean proof of a classical theorem.
