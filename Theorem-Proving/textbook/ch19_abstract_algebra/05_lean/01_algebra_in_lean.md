# Algebra in Lean

Everything in this chapter — axioms as theories, theorems as equational derivations, decision procedures for equational fragments — becomes concrete inside a proof assistant. Lean 4's mathematical library, Mathlib, formalizes the entire algebraic hierarchy, and its automation tactics are working implementations of the decidability results we have met.

## The Hierarchy as Typeclasses

Mathlib encodes "a structure satisfying axioms" as a **typeclass**: a record bundling operations with proofs of the axioms. Structures extend one another exactly as the mathematical hierarchy suggests:

```lean
-- Slightly simplified from Mathlib, which interposes more layers
-- (MulOneClass, DivInvMonoid, ...) and extra data fields.
class Semigroup (α : Type u) extends Mul α where
  mul_assoc : ∀ a b c : α, a * b * c = a * (b * c)

class Monoid (M : Type u) extends Semigroup M, One M where
  one_mul : ∀ a : M, 1 * a = a
  mul_one : ∀ a : M, a * 1 = a

class Group (G : Type u) extends Monoid G, Inv G where
  inv_mul_cancel : ∀ a : G, a⁻¹ * a = 1

class CommRing (R : Type u) extends Ring R where
  mul_comm : ∀ a b : R, a * b = b * a

class Field (K : Type u) extends CommRing K, Inv K where
  exists_pair_ne : ∃ x y : K, x ≠ y
  mul_inv_cancel : ∀ a : K, a ≠ 0 → a * a⁻¹ = 1
  inv_zero : (0 : K)⁻¹ = 0
```

Compare `Group` with `Field`: the group axioms are bare equations, but `mul_inv_cancel` for fields carries the hypothesis `a ≠ 0` — the non-equational axiom that the HSP theorem (Section 4) told us was unavoidable. (Mathlib makes `⁻¹` total by the junk-value convention `0⁻¹ = 0`; the *theory* does not change, only the signature.)

A theorem proved once for `Monoid` is instantly available for every group, ring, and field, because **instances** (`ℤ : CommRing`, `ℚ : Field`, `Equiv.Perm α : Group`, ...) are found by typeclass resolution and inheritance follows the `extends` edges.

**Diamonds.** The inheritance graph is not a tree: `CommRing` reaches `Monoid` both through its multiplicative structure and through other intermediate classes, so one type can acquire the "same" ancestor instance along two paths. Lean 4 handles this with *flat structures* — parent fields are inlined — but the two paths must then agree **definitionally**, or rewriting breaks. This is why Mathlib classes carry seemingly redundant data fields (such as `npow`, the power operation, with a default value): fixing the data in the class, rather than deriving it differently along each path, keeps diamond instances definitionally equal.

## A Worked Proof: Cancellation

Section 1 proved left cancellation by a chain of labeled axiom applications. Here is the same derivation in Lean, one axiom per line:

```lean
example {G : Type*} [Group G] {a b c : G} (h : a * b = a * c) : b = c :=
  calc b = 1 * b         := (one_mul b).symm
    _ = a⁻¹ * a * b      := by rw [inv_mul_cancel]
    _ = a⁻¹ * (a * b)    := mul_assoc a⁻¹ a b
    _ = a⁻¹ * (a * c)    := by rw [h]
    _ = a⁻¹ * a * c      := (mul_assoc a⁻¹ a c).symm
    _ = 1 * c            := by rw [inv_mul_cancel]
    _ = c                := one_mul c
```

The formal proof *is* the equational derivation — G2, G3, G1, hypothesis, G1, G3, G2. Idiomatically one writes either of:

```lean
example {G : Type*} [Group G] {a b c : G} (h : a * b = a * c) : b = c :=
  mul_left_cancel h

example {G : Type*} [Group G] {a b c : G} (h : a * b = a * c) : b = c := by
  rw [← inv_mul_cancel_left a b, h, inv_mul_cancel_left]
```

where `inv_mul_cancel_left : a⁻¹ * (a * b) = b` is exactly rule (4) that Knuth–Bendix completion discovered in Section 4.

## Tactics as Decision Procedures

Several Mathlib tactics are implementations of this chapter's decidability theorems, each deciding an *equational fragment*:

- **`ring`** proves every identity valid in all commutative (semi)rings by normalizing both sides to a canonical polynomial form. Since the free commutative ring on $x_1, \dots, x_n$ is $\mathbb{Z}[x_1, \dots, x_n]$ (Section 4), comparing normal forms decides the word problem for commutative rings — Birkhoff completeness plus confluent rewriting, packaged as a tactic.
- **`abel`** does the same for (additive) commutative groups, and **`group`** normalizes words in free groups — the convergent rewrite system from Section 4 in action. **`noncomm_ring`** handles rings without commutativity.
- **`omega`** decides linear integer arithmetic — a working Presburger-fragment procedure, on the decidable side of the boundary from Section 2, while no tactic can decide full nonlinear integer arithmetic (Matiyasevich).
- **`decide`** evaluates any proposition with a `Decidable` instance, e.g. checking by brute force that a small Cayley table is associative — decision by finiteness.

The pattern to internalize: full first-order theories of classes of structures are usually undecidable (Section 1), but the *universally quantified equations* are often decidable by normalization, and those are precisely the goals these tactics close.

## What Formalized Algebra Achieves

Mathlib today contains the full undergraduate algebra curriculum and much beyond: subgroups, quotients and the isomorphism theorems, Lagrange, the Sylow theorems, polynomial rings and ideals, Galois theory, and the theory of fields — all machine-checked, all stated at typeclass generality so each theorem applies across the hierarchy.

The landmark that proved large-scale algebra formalizable is the **odd-order theorem**. Feit and Thompson's 1963 theorem — every finite group of odd order is solvable, a cornerstone of the classification of finite simple groups (Section 1) — occupied a 255-page journal issue. Georges Gonthier and his team formalized it completely in Coq (2006–2012) using the Mathematical Components/SSReflect libraries: on the order of 170,000 lines of proof script, more than 4,000 theorems and 15,000 definitions. Every step, down to the equational manipulations of Section 1, was checked by a kernel a few thousand lines long. For proofs at the scale of the classification — too long for any referee — this is the credible path to certainty, and it is the argument of this book in miniature: logic is not merely *about* algebra; it is the technology by which algebra is known.

## Exercises
See [problems/ch19_abstract_algebra/](../../../problems/ch19_abstract_algebra/)
