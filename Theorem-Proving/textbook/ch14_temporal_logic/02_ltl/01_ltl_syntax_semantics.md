# Linear Temporal Logic (LTL)

In LTL, we reason about a single infinite execution trace — the unique future of a deterministic (or nondeterministically resolved) system.

## Syntax

Given a set AP of atomic propositions, LTL formulas are built by:
```
φ ::= p          (atomic proposition, p ∈ AP)
    | ¬φ         (negation)
    | φ ∧ ψ      (conjunction)
    | X φ        (next)
    | φ U ψ      (until)
```

All other operators are derived:
- F φ = ⊤ U φ  ("eventually": at some point, φ)
- G φ = ¬F ¬φ  ("globally": at all points, φ)
- φ R ψ = ¬(¬φ U ¬ψ)  (release)
- φ W ψ = (φ U ψ) ∨ G φ  (weak until)

## Semantics

An LTL formula is evaluated on an *infinite trace* π = s₀ s₁ s₂ ... where each sᵢ ⊆ AP is the set of propositions true at step i.

We write π, i ⊨ φ to mean "φ holds at position i of trace π":

```
π, i ⊨ p           iff  p ∈ sᵢ
π, i ⊨ ¬φ          iff  π, i ⊭ φ
π, i ⊨ φ ∧ ψ       iff  π, i ⊨ φ and π, i ⊨ ψ
π, i ⊨ X φ         iff  π, i+1 ⊨ φ
π, i ⊨ φ U ψ       iff  ∃j ≥ i. π, j ⊨ ψ and ∀k ∈ [i,j). π, k ⊨ φ
```

A formula φ is satisfied by trace π if π, 0 ⊨ φ.

## Common Specification Patterns

| Pattern | LTL Formula | Meaning |
|---------|-------------|---------|
| Safety | G ¬bad | bad never occurs |
| Reachability | F goal | goal is eventually reached |
| Response | G(req → F resp) | every request is eventually responded to |
| Precedence | ¬p W q | p doesn't occur before q |
| Invariant | G(P → Q) | whenever P, then Q |
| Absence | G ¬p | p never occurs |
| Recurrence | G F p | p occurs infinitely often |
| Persistence | F G p | p holds from some point on forever |

## Expressiveness

LTL is expressively equivalent to the *star-free regular languages* over infinite words (the ω-regular languages without counting). It cannot express:
- "p holds at every even time step" (requires CTL or counting)
- Properties involving the number of times an event occurs

LTL is **PSPACE-complete** to model check (verify a system against an LTL spec). Despite this worst-case complexity, practical tools (SPIN, nuSMV, NuXMV) handle industrial-scale systems.

## Example: Mutual Exclusion

The Peterson algorithm ensures mutual exclusion. Key properties:
- **Safety**: G ¬(cs₁ ∧ cs₂)  — both processes not in critical section simultaneously
- **Liveness**: G(want₁ → F cs₁)  — process 1 eventually enters its critical section

These can be verified with SPIN using a model of the algorithm.
