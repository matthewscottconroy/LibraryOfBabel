# Chapter 22: Cubical Agda — HoTT with Computational Content

## Why Cubical Agda?

The previous chapter covered Lean 4 and Mathlib — the best tool for formalizing classical mathematics. But Lean 4 has a fundamental limitation for HoTT: it treats equality as proof-irrelevant (`Prop`-valued), which forces all types to behave like sets. You can't define a HIT like the circle $S^1$ with a non-trivial loop, because in Lean 4, any proof that `base = base` is definitionally equal to `refl`.

Cubical Agda is different. It gives HoTT *computational content* — the ability to actually compute with paths, univalence, and higher inductive types.

The key words here are "computational content." In ordinary HoTT (as in the HoTT Book), univalence is an *axiom* — you can assert it and use it to prove things, but you can't compute with it. A term like `transport (ua e) x` is stuck: it doesn't reduce to anything. This breaks canonicity (the property that every closed term of a base type evaluates to a concrete value).

Cubical type theory, introduced by Bezem, Coquand, and Huber (and refined by Cohen, Coquand, Huber, Mörtberg), solves this by replacing the *identity type* `a = b` with a *path type* `a ≡ b` defined as functions out of the interval. The interval is a primitive type `I` with two endpoints `i0` and `i1`, and a path from `a` to `b` is literally a function `I → A` sending `i0` to `a` and `i1` to `b`.

This change makes everything computable:
- Transport has a definition in terms of a primitive `transp` operation
- Univalence is a *theorem* provable from the `Glue` type constructor
- Higher inductive types are definable with actual computation rules
- The resulting theory has *canonicity*: every closed natural number computes to a numeral

Agda is the proof assistant that implements this theory. Cubical Agda is Agda with the `--cubical` pragma enabled.

## Cubical Agda vs. HoTT Book vs. Lean 4

| Feature | HoTT Book | Lean 4 | Cubical Agda |
|---------|-----------|--------|--------------|
| Foundation | MLTT + Univalence (axiom) | CIC + `propext` + `funext` | Cubical Type Theory (CTT) |
| Univalence | Axiom | Consequence of axioms | Theorem (provable!) |
| HITs | Axioms | Not available | First-class |
| Canonicity | No (blocked on `ua`) | Yes (for CIC) | Yes |
| Classical math | With extra axioms | Full Mathlib | Limited |
| HoTT-specific | Full theory | Not available | Full theory |
| Library | n/a | Mathlib (massive) | Cubical library (growing) |

Cubical Agda is the tool for anyone who wants to work with HoTT *computationally* — to run proofs, to verify that computed results match theoretical expectations, to extract programs from proofs. It's the closest thing to a *running implementation* of the HoTT Book's mathematics.

## Chapter Roadmap

**Section 1: Agda Basics** — Agda's syntax, universe levels, the `--without-K` pragma.

**Section 2: Cubical Mode** — The interval type `I`, path types as functions `I → A`, transport, `hcomp`, the `Glue` type, and the proof of univalence.

**Section 3: HITs in Agda** — Defining and using higher inductive types: the circle, suspension, pushouts, and truncations. The circle computation $\pi_1(S^1) = \mathbb{Z}$.

**Section 4: HoTT in Agda** — The Cubical Agda library, h-levels, equivalences, and a tour of what's formalized.

This chapter is the computational culmination of the entire curriculum — where all the abstract theory becomes running code.
