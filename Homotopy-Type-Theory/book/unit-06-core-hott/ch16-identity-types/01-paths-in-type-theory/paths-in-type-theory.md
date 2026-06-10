# Paths in Type Theory

## The Shape of Equality

Begin with a question that sounds simple: what is it to be equal?

In ordinary logic, equality is a relation. Given elements a and b of some domain, the statement a = b is either true or false. If it is true, we say a and b are equal, and we treat them interchangeably. We do not ask: in what way are they equal? By which argument? Following which path?

This simplicity is not a virtue. It is an impoverishment.

Here is what we miss when we treat equality as a proposition rather than a type. Suppose we have two proofs that a group G is abelian. Both proofs establish the same fact. But one proof works by noting that G is a quotient of a free abelian group; the other works by exhibiting an explicit commutativity witness for each pair of elements. These proofs are different. They carry different information. They connect to different future arguments in different ways. In classical logic, we are forced to identify them — both are "the proof that G is abelian." In HoTT, they are different elements of the type `G-is-abelian`, related by a 2-path that witnesses the specific way they are equivalent.

In HoTT, the identity type `a =_A b` is a genuine type. It may be empty (if a and b are not equal in any sense). It may have one element (if the type A is a set and a = b). It may have infinitely many elements (as in the loop space of the circle, where `base =_{S^1} base` has one element for each integer). An element `p : a =_A b` is not just a verdict — it is a witness, a path, a specific reason.

## Formation, Introduction, and Elimination

The identity type is governed by four rules: formation (how to form the type), introduction (how to construct elements), elimination (how to use elements), and computation (how elimination interacts with introduction).

**Formation.** Given a type A and two terms a : A and b : A, we may form the identity type:

```
  A : Type    a : A    b : A
  ──────────────────────────
       a =_A b : Type
```

Equivalently written Id_A(a, b). This is a type family over A × A: it assigns to each pair (a, b) the type of paths from a to b.

**Introduction.** For any term a : A, there is a canonical element:

```
      a : A
  ──────────────
  refl_a : a =_A a
```

The reflexivity path is the only axiomatically given element of any identity type. It is the constant path at a. We cannot axiomatically give a path from a to b for `a ≠ b` — such paths must be built from the structure of A itself.

This is a subtle and important point. The existence of a path `p : a =_A b` for `a ≠ b` is not forbidden by the rules — it simply cannot be postulated from outside. It must arise from the constructors of A. The only given path is the trivial one.

**Elimination (the J rule).** This is the heart of the identity type. Suppose we fix a : A and we want to prove something `C(b, p)` for all b : A and all paths p : a =_A b. It suffices to prove `C(a, refl_a)` — the case where b is a and p is the constant path.

Formally:

```
  b : A, p : a =_A b ⊢ C(b, p) : Type
  d : C(a, refl_a)
  ─────────────────────────────────────
  J(C, d, b, p) : C(b, p)
```

**Computation.** The J eliminator computes as: `J(C, d, a, refl_a) ≡ d`. When we apply J with the trivial path, we recover the base case.

## The Homotopy Interpretation

Why does the J rule work? The deep reason is that the space of based paths is contractible.

Define the based path space rooted at a : A to be:

```
P_a A  :=  Σ(b : A), (a =_A b)
```

This is the type of all paths starting at a, varying over their endpoint. The claim is: P_a A is contractible. There is a center — the pair (a, refl_a) — and every other pair (b, p) is connected to it by a canonical path, built from p itself.

This is precisely what the J rule says. To define something for all (b, p) in P_a A, it suffices to define it at the contractible center (a, refl_a). Contractibility means "there is only one thing here, up to the structure we're considering," and that one thing is the trivial based path.

In the simplicial set model that underlies HoTT:
- A type is interpreted as a Kan complex — a combinatorial space.
- `a : A` is a 0-simplex (vertex).
- `a =_A b` is the path space — the Kan complex of 1-simplices from a to b.
- `refl_a` is the degenerate 1-simplex `σ₀(a)` — the constant path, going nowhere.
- The J rule corresponds to the fact that the based path fibration over a Kan complex is a fibration with contractible fibers.

So "path induction" is not an arbitrary axiom. It is the type-theoretic expression of the fundamental fact that the based path space is contractible.

## Why Identity Types Can Have Many Elements

In set-theoretic mathematics, equality between elements of a set is a discrete relation: either a = b (one proof, trivially) or not. This is because sets are *discrete spaces* — homotopically trivial.

Types in HoTT are not required to be discrete. They are spaces. And in a space, the path type from a to b can be arbitrarily complex:

- In the circle `S^1`: the type `base =_{S^1} base` of loops at the base point is equivalent to Z. One loop for each integer — one for each winding number. The loops `loop`, `loop·loop`, `loop^n` are all distinct elements of the same identity type.

- In the universe `Type`: the type `A =_{Type} B` of paths between types is equivalent (by the Univalence Axiom, Chapter 18) to the type `A ≃ B` of equivalences. If there are many non-isomorphic ways for A and B to be equivalent, then `A =_{Type} B` has many elements.

- In the 2-sphere `S^2`: paths between paths — 2-paths — are nontrivial. The type `refl_{base} =_{base =_{S^2} base} refl_{base}` of 2-loops at the base is equivalent to Z.

The richness of identity types reflects the richness of the space. And this richness is not a bug — it is the entire point of homotopy type theory. By refusing to flatten identity into a proposition, we preserve all the homotopy-theoretic information that classical mathematics threw away.

## The Type-Theoretic Approach vs. Classical Logic

Let us be explicit about the difference.

In classical predicate logic, equality is a relation E(x, y) satisfying:
- Reflexivity: E(x, x)
- Substitution: if E(x, y) and P(x), then P(y)

These are axioms. Equality is an unanalyzed primitive.

In HoTT, equality is a *type*, and every aspect of its behavior follows from the rules for that type:

| Classical | HoTT |
|---|---|
| E(a, a) is a truth value | `refl_a : a = a` is a term |
| Substitution is a rule schema | Transport is a function |
| Symmetry is an axiom | `p⁻¹ : b = a` is defined by J |
| Transitivity is an axiom | `p · q : a = c` is defined by J |
| Leibniz: E(a,b) and P(a) implies P(b) | `transport^P(p) : P(a) → P(b)` |

Each classical axiom becomes a derived result in HoTT, obtained by applying the J eliminator. The difference is not that HoTT is weaker — it proves everything classical logic proves about equality. The difference is that HoTT is *more structured*: it keeps track of the witnesses, the paths, the reasons. It refuses to collapse all proofs into a binary verdict.

This extra structure is the source of all the novelty in the chapters ahead.

## Reflexivity as the Constant Path

One final point deserves attention. The reflexivity term `refl_a : a =_A a` is not just "the proof that a equals itself." In the homotopy interpretation, it is the *constant path at a*.

What does "constant path" mean? Topologically, a path from x to y in a space X is a continuous function `γ : [0,1] → X` with `γ(0) = x` and `γ(1) = y`. The constant path at x is the function `γ(t) = x` for all t — it never moves.

In the simplicial model, `refl_a` is the degenerate 1-simplex at a — the unique 1-simplex that is "collapsed" to the vertex a. Degenerate simplices are the combinatorial analog of constant paths.

The J rule says: every path is "connected" to the constant path. Not that every path *is* the constant path — that would make all types discrete. But that every path is reachable from the constant path, in the sense that to prove something about all paths, you need only prove it for the constant path and the rest follows.

This is a profound statement. It means that the identity type, despite potentially having complex structure, is in a precise sense *generated* by a single element: refl. The J rule is the formal expression of this generativity.

## Summary

The identity type `a =_A b` is:

1. A type — not a proposition, not a relation, but a full type with its own elements and structure.
2. Interpreted as a path space — elements are paths, higher elements are homotopies.
3. Governed by the J rule — path induction from the reflexivity path.
4. Potentially complex — may have many distinct elements depending on the type A.
5. Foundational — all properties of equality (symmetry, transitivity, substitution) are derived from J.

Every chapter in this unit builds on this foundation. The J rule is the axiom. Everything else is mathematics.
