# Geometry in Proof Assistants

The chapter's arc — Euclid's diagram-dependent system ([Section 1](../01_euclid/01_euclid_axioms.md)), made rigorous by Hilbert ([Section 3](../03_hilbert/01_hilbert_axioms.md)), shown complete and decidable by Tarski ([Section 4](../04_tarski/01_tarski_geometry.md)) — closes in the proof assistant, where the axioms are machine-checked and, remarkably, where the *decision procedure* can sometimes replace proof altogether. Geometry is the one classical theory in which Hilbert's dream of a complete, consistent, mechanically verifiable axiomatization is fully realized.

## Why Formalize Geometry?

The motivation is exactly the gap of Section 1: diagrams silently supply premises. The notorious "proof that every triangle is isosceles" goes through on a carefully mis-drawn figure because a betweenness fact is read off the picture that no axiom guarantees. A proof assistant admits no picture: every incidence, betweenness, and congruence must be a cited hypothesis or a derived lemma. Formalization is thus the ultimate audit of Euclidean reasoning, catching the co-exact assumptions Manders located in the drawing rather than the text.

## GeoCoq: Tarski's Axioms in Coq

The most complete synthetic formalization is **GeoCoq** (Narboux, Braun, Boutry, and collaborators), a large Coq library built on **Tarski's axioms**. Tarski's system is ideal for a proof assistant: one sort (points), two relations ($B$ and $\equiv$), no set or line sorts to encode, and a purely first-order axiom list. The primitives become a typeclass and the axioms its fields:

```coq
Class Tarski_neutral :=
{
  Tpoint : Type;
  Bet  : Tpoint -> Tpoint -> Tpoint -> Prop;          (* betweenness B(a,b,c) *)
  Cong : Tpoint -> Tpoint -> Tpoint -> Tpoint -> Prop; (* congruence ab ≡ cd *)
  cong_pseudo_reflexivity : forall a b, Cong a b b a;
  cong_identity           : forall a b c, Cong a b c c -> a = b;
  cong_inner_transitivity : forall a b c d e f,
      Cong a b c d -> Cong a b e f -> Cong c d e f;
  segment_construction    : forall a b c d,
      exists e, Bet a b e /\ Cong b e c d;
  five_segment            : forall a a' b b' c c' d d', (* SAS, coded *)
      Cong a b a' b' -> Cong b c b' c' -> Cong a d a' d' -> Cong b d b' d' ->
      Bet a b c -> Bet a' b' c' -> a <> b -> Cong c d c' d'
  (* ... Pasch, dimension, Euclid, continuity schema ... *)
}.
```

GeoCoq formalizes Wanda Szmielew's development (from Schwabhäuser, Szmielew, and Tarski, *Metamathematische Methoden in der Geometrie*, 1983) chapter by chapter: the ordering of points, midpoints, perpendiculars, the **arithmetization of geometry** (constructing the coordinate real-closed field from the axioms, recovering the Pythagorean theorem), and the continuity variants. A headline result is a fully mechanized **study of the parallel postulate**: GeoCoq proves the equivalence, over neutral geometry, of more than thirty formulations — Playfair, the triangle angle sum, the existence of a rectangle, Bachmann's *Lotschnittaxiom*, Proclus's axiom — closing by machine the informal "it is well known that these are equivalent" of the textbooks.

## The Tarski–Hilbert Equivalence, Mechanized

Sections 3 and 4 present two axiomatizations of the same geometry. GeoCoq *proves they coincide*: it derives all of **Hilbert's axioms as theorems** from Tarski's, and conversely interprets Tarski's primitives in Hilbert's, giving a machine-verified bi-interpretation. This is the strongest possible vindication that "Euclidean plane geometry" is a single well-defined object approachable from either the betweenness-and-congruence side or the incidence-order-congruence side.

## Synthetic vs. Analytic: Lean's Mathlib

Lean 4's **Mathlib** takes the complementary, **analytic** route sanctioned by the coordinatization theorem ([Section 4](../04_tarski/01_tarski_geometry.md)): rather than posit axioms, it *defines* Euclidean geometry over the real inner-product space `EuclideanSpace ℝ (Fin n)` and derives everything from the theory of $\mathbb{R}$. Angles are `InnerProductGeometry.angle`; there are spheres, the law of cosines, the existence and uniqueness of the circumcenter, Ptolemy's inequality, and the concurrence theorems. A synthetic lemma and its analytic counterpart look different but describe the *same* structure — and the coordinatization theorem is precisely the guarantee that the two formalizations, GeoCoq's and Mathlib's, are talking about one geometry. A small Lean rendering of Tarski's signature as a typeclass, in the GeoCoq spirit:

```lean
class TarskiPlane (Point : Type*) where
  Bet  : Point → Point → Point → Prop
  Cong : Point → Point → Point → Point → Prop
  cong_pseudo_refl : ∀ a b, Cong a b b a
  cong_identity    : ∀ a b c, Cong a b c c → a = b
  seg_construct    : ∀ a b c d, ∃ e, Bet a b e ∧ Cong b e c d
  -- five-segment, Pasch, dimension, Euclid, continuity …

-- a first genuine lemma, provable from the axioms alone:
example {Point} [TarskiPlane Point] (a b : Point) :
    TarskiPlane.Cong a b a b := by
  -- from pseudo-reflexivity + inner transitivity
  sorry
```

## Deciding Instead of Proving

Because elementary geometry is *decidable* (Section 4), the proof assistant has a second mode unavailable elsewhere in mathematics: it can **decide** a statement rather than search for a human-style proof. In practice three layers coexist:

- **The algebraic (equality-type) fragment** — concurrency, collinearity, and other equational theorems — reduces to polynomial-ideal membership and is dispatched *fast* by **Wu's method** or **Gröbner bases** (the Chou–Gao–Zhang area method has mechanically proved hundreds of such theorems). Coq's `nsatz` tactic and Lean's `polyrith` implement the Nullstellensatz search directly.
- **The full ordered theory** — anything involving betweenness or inequalities — needs real quantifier elimination: **CAD** (QEPCAD, Redlog) or SMT (`nlsat` in Z3), doubly exponential but complete. Lean's `nlinarith` and `positivity` handle the nonlinear-arithmetic goals that arise, and heavier obligations can be discharged by an external RCF oracle.
- **Ordinary interactive proof** — the human-guided derivations of GeoCoq and Mathlib, for readability and for the metatheory (the decision procedures themselves are justified by the results of Sections 3–4).

The three layers embody the chapter's thesis operationally. Euclid needed a diagram; Hilbert needed a longer list of axioms; Tarski proved the whole theory decidable; and the proof assistant now *checks the axioms, derives the theorems, and — when we wish — computes the answer*. No other branch of classical mathematics permits all three at once, because no other is at once first-order, complete, and decidable. Geometry, the oldest formal system, is also the most completely conquered.

## Exercises
See [problems/ch20_geometry_and_logic/](../../../problems/ch20_geometry_and_logic/)
