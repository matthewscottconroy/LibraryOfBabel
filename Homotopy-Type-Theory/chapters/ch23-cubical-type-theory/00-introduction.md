# Chapter 23: Cubical Type Theory — Computational HoTT

## The Canonicity Problem

The HoTT Book presents a beautiful vision: homotopy type theory as a foundation for mathematics where types are spaces, functions are continuous maps, and equality is homotopy. But it comes with a subtle foundational problem that the Book explicitly leaves open: *canonicity*.

In any type theory worth calling a foundation for mathematics, we expect **canonicity**: every closed term of type $\mathbb{N}$ reduces to a numeral $0, 1, 2, \ldots$. This is the computational completeness of the theory — proofs aren't just formal objects, they *run* and produce concrete answers.

In standard HoTT (Martin-Löf type theory + univalence as an axiom), canonicity fails. The problem is `ua`:

```
ua(e) : A = B
```

is an axiom. It has no reduction rule. So any term that uses `ua` in a computation is *stuck* — it can't be evaluated further. A term like `transport (ua succ-equiv) 3` should evaluate to `4`, but there's no computation rule telling the type checker how to reduce it.

This isn't just an aesthetic problem. It means:
- Proofs using univalence have no computational content
- You can't extract programs from univalence-based proofs
- The "propositions are types, proofs are programs" slogan breaks down

## The Cubical Solution

The Cohen-Coquand-Huber-Mörtberg (CCHM) cubical type theory (2015) solves this by fundamentally rethinking what a path *is*. Instead of an axiom, univalence becomes a *theorem* with a genuine computation rule.

The core idea: replace the identity type $a =_A b$ with a *path type* defined as

$$a =_A b \;\;\equiv\;\; \text{functions } f : \mathbb{I} \to A \text{ with } f(0) = a \text{ and } f(1) = b \text{ (definitionally)}$$

where $\mathbb{I}$ is a new primitive *interval* type. With this definition:
- Transport is computed by evaluating a function at a point
- Composition fills open cubes, with definitional computation rules
- Univalence follows from the `Glue` type constructor
- Function extensionality follows immediately by rearranging arguments
- *Every closed term normalizes* — canonicity holds as a theorem

This chapter develops the mathematical theory behind Cubical Agda: the interval, face formulas, partial elements, composition, the Kan condition, the Glue type, and the proof of univalence.

## Chapter Roadmap

**Section 1: The Interval and Paths** — The interval $\mathbb{I}$, dimension variables, face formulas, path types as functions.

**Section 2: Composition and Transport** — The `hcomp` operation, the Kan filling condition, composition for each type former, transport.

**Section 3: The Glue Type and Univalence** — Partial elements, the Glue type constructor, proving univalence, the computation rule.

**Section 4: Variations** — Cartesian cubical type theory, XTT, canonicity and normalization, connections to simplicial models.
