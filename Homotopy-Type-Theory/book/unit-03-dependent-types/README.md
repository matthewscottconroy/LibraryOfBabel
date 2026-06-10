# Unit 03: Dependent Types — The Bridge to HoTT

## What Changes When Types Depend on Values

There is a specific moment when a type system stops being a convenient bookkeeping device and starts being a language for mathematics. That moment is when you allow a type to depend on a value.

In every type system you have encountered before this unit — the simply typed lambda calculus, System F, even System Fω — there is a strict border between the world of types and the world of terms. Types classify terms. Terms inhabit types. Neither crosses into the other's territory. The type `List Int` does not know how many elements are in any particular list. The term `[1, 2, 3]` does not carry its length in its type.

Dependent types erase this border. In a dependently typed system, the type `Vec 3` means "a list of exactly three elements," and `Vec n` is a type that changes as `n` changes. The type is a function of the value. This is a small syntactic extension. Its consequences are enormous.

## Why This Unit Exists

We have built up to this point carefully. The untyped lambda calculus gave us computation. The simply typed lambda calculus gave us basic safety guarantees — functions cannot be applied to arguments of the wrong type. System F gave us polymorphism — one function that works at every type. System Fω gave us type operators — functions at the type level.

But none of these systems can state a theorem about natural numbers inside the type system. None of them can express "the output list has the same length as the input list" or "this function is a bijection" or "this proof is a proof of Fermat's Last Theorem." For that, you need types that talk about specific values. You need dependent types.

This unit develops dependent type theory in two chapters.

**Chapter 8: Dependent Types.** We introduce the machinery ground-up: type families (types indexed by values), Π types (dependent functions), Σ types (dependent pairs), universes (types of types), inductive types (the natural numbers, lists, vectors, and more), and the full Curry-Howard correspondence that emerges when all of these combine. By the end of Chapter 8, you can write formally verified programs with types that encode precise mathematical specifications.

**Chapter 9: Martin-Löf Type Theory.** We consolidate dependent type theory into a single formal system — MLTT, developed by Per Martin-Löf in lectures and papers during the 1970s and 1980s. MLTT adds the ingredient that makes HoTT possible: the **identity type**, the type whose elements are proofs that two things are equal. We examine the four forms of judgment that constitute MLTT, the full set of type-forming rules, the J elimination rule for identity types, transport and ap (the two operations built from J), and the crucial distinction between intensional and extensional MLTT.

## The HoTT Connection

Every concept in this unit is load-bearing for what comes later. The Π type and Σ type are the workhorses of all HoTT constructions. The universe hierarchy is required just to state Univalence — the axiom that equivalent types are identical. The identity type, once you look at it carefully, turns out to encode not just equality but paths. A proof that `a = b` is not merely a certificate; it is a path through the type `A` from `a` to `b`, and paths can have higher structure — homotopies, homotopies of homotopies, all the way up.

This is the discovery that makes HoTT surprising and powerful: if you take MLTT's identity type seriously and interpret it geometrically, types become spaces. Points become elements. Paths become identity proofs. The fact that identity proofs can fail to be unique — that Uniqueness of Identity Proofs is not derivable — means that these spaces can have non-trivial topology. The circle, the sphere, the torus — all definable as types. Their homotopy groups — all computable in the type theory.

But that is Unit 05. Here we build the foundation.

## How to Read This Unit

Each chapter has a consistent structure. The content sections (01 through 06) develop the mathematics in sequence, with each section approximately building on the last. At the end of each chapter you will find:

- **Important Thinkers** — the people who created these ideas, with historical context
- **References** — primary sources and recommended secondary literature
- **Thought Experiments** — extended conceptual puzzles to deepen intuition
- **Exercises** — 25–35 problems ranging from direct computation to open-ended proof
- **Applications** — specific deployments of these ideas in real systems

We recommend reading the content sections once through without stopping to check all the details, then returning with pencil and paper for the exercises. The ideas here are genuinely new — they do not reduce to anything you have seen before — and the only way to absorb them is to work with them.

Let us begin.
