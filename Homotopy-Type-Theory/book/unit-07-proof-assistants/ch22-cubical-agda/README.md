# Chapter 22: Cubical Agda — HoTT with Computational Content

Book HoTT has a problem: the univalence axiom is an axiom. You can assert it, and derive consequences, but you cannot compute with it. If you have a function `f : Bool ≡ Bool → Bool` and you apply it to `ua(swap)`, you cannot reduce the result to a normal form. The proof is stuck. The type says the answer is a `Bool`, but the type theory cannot actually produce the boolean. Canonicity — the property that every closed term of a basic type reduces to a canonical value — is broken.

Anders Mörtberg and his collaborators solved this by building a type theory — Cubical Agda — where univalence is not an axiom but a theorem, provable from the structure of the interval. In Cubical Agda, `ua` computes. Paths are functions. The abstract becomes concrete. This chapter teaches you to use it.

## The Problem, Stated Precisely

In Martin-Löf type theory with the univalence axiom (the HoTT Book's foundation), the term

```
transport (ua (isoToEquiv swapIso)) true : Bool
```

should evaluate to `false`. After all, transporting `true` across the equivalence that swaps `true` and `false` should give `false`. But in axiomatic HoTT, this term does not reduce. `ua swapIso` is an axiom, a black box — the type theory knows it has type `Bool ≡ Bool` but has no computation rule that opens it.

This is not just a practical inconvenience. It means proofs are not programs: you can prove `transport (ua e) a ≡ e .fst a` (that transport-along-ua equals applying-the-equivalence), but you cannot *compute* it. The proof of this equation is itself an axiom, with no computational content.

This violates the spirit of the Curry-Howard correspondence. If propositions are types and proofs are programs, programs should run. A foundation for mathematics whose proofs don't run is incomplete.

## The Cubical Solution

The cubical type theory of Cohen, Coquand, Huber, and Mörtberg (2016) solves this by replacing the identity type with a *path type*: the type `a ≡ b` is defined as the type of functions `I → A` from a formal interval `I` to `A`, satisfying `p i0 = a` and `p i1 = b` definitionally. The endpoints are not just propositionally equal to `a` and `b` — they *are* `a` and `b`, by definition.

This one change — paths as interval-parametrized functions with definitional endpoints — restores canonicity. Transport acquires a computation rule: `transport (ua e) a` reduces to `e .fst a`, definitionally. Univalence becomes a theorem, proved from the `Glue` type constructor. Higher inductive types get genuine computation rules on their path constructors.

Agda is the proof assistant implementing this theory. Cubical Agda is Agda with the `--cubical` pragma.

## Why This Matters for HoTT

Every HoTT proof, under the HoTT Book's axiomatic framework, is potentially a "stuck computation" — a term that the type theory cannot reduce to a canonical form. The π₁(S¹) = ℤ proof constructs an equivalence, but if you try to compute the winding number of a specific loop using that equivalence, you may get stuck. The proof is an abstract object, not a computable function.

In Cubical Agda, the proof *is* the computation. The function `encode : (base ≡ base) → ℤ` is a running program. You can apply it to a specific loop — say, `loop ∙ loop ∙ loop` — and Agda will normalize the result to `3 : ℤ`. The theorem and the algorithm are the same thing.

This is what "computational content" means: not just that the proof is logically valid, but that the proof *runs*. Cubical Agda makes HoTT computationally real.

## Chapter Roadmap

**Section 1: Agda Basics** — Agda's syntax, module system, inductive types, pattern matching, universe levels, the `--cubical` pragma, and the basics of working with proof obligations interactively. Working Agda code throughout.

**Section 2: Cubical Mode** — The interval `I`, paths as functions `I → A`, path inversion via complement `~`, path concatenation via `hcomp`, transport via `transp`, the `Glue` type, and the proof of univalence. Every operation explained with working code.

**Section 3: HITs in Cubical Agda** — Higher inductive types as data declarations with path constructors. The circle `S¹`, the suspension, the torus, propositional and set truncations, the pushout. The fundamental theorem: $\pi_1(S^1) = \mathbb{Z}$.

**Section 4: HoTT in Agda** — The Cubical Agda library structure, h-levels, equivalences, the `ua` function and its computation rule, the full homotopy-group infrastructure, and what computable HoTT makes possible. The Brunerie number and the computation of $\pi_4(S^3)$.

After these sections, the exercises, thought experiments, and applications complete the picture. The goal is not memorization of Agda syntax but intuition: when you see a cubical proof, you should understand what it is computing and why.

For anyone serious about HoTT as a foundation for mathematics — not just as a theoretical framework but as a computational tool — Cubical Agda is where the action is.
