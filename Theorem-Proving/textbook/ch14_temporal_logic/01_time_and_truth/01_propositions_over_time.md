# Propositions Over Time

Classical logic treats propositions as eternally true or false. Temporal logic asks a richer question: what is true *now*, what *will* be true, what *has always been* true?

## The Motivation

Consider specifying the behavior of a traffic light controller:
- "The light is never red and green simultaneously." — a *safety* property
- "Every red phase is eventually followed by a green phase." — a *liveness* property
- "Between every two red phases, there is at least one green phase." — a *fairness* property

Classical propositional logic cannot express these: they describe behavior across *sequences of states*, not individual states. Temporal logic provides the operators to do so.

## Time as a Mathematical Structure

Temporal logic models time as a *Kripke structure* where:
- **States** represent snapshots of a system
- **Transitions** connect successive states
- **Atomic propositions** hold or fail at each state

Different choices of time structure yield different logics:
- *Linear time*: at each moment, there is exactly one future (a sequence)
- *Branching time*: at each moment, multiple futures are possible (a tree)

## Key Operators

Given a set of states S with a transition relation → and labeling L:

| Operator | Read as | Meaning |
|----------|---------|---------|
| **G φ** | "Globally φ" | φ holds at all future states |
| **F φ** | "Finally φ" | φ holds at some future state |
| **X φ** | "neXt φ" | φ holds at the next state |
| **φ U ψ** | "φ Until ψ" | φ holds until ψ holds (and ψ eventually holds) |
| **φ W ψ** | "φ Weak-until ψ" | φ holds until ψ, or φ holds forever |
| **φ R ψ** | "φ Release ψ" | ψ holds until (and including when) φ holds, or ψ holds forever |

The operators G and F are dual: G φ = ¬F ¬φ ("not eventually not φ" = "always φ").

## Safety vs. Liveness

Every temporal property is a *safety* property, a *liveness* property, or a combination (Alpern-Schneider classification, 1985):

- **Safety**: "something bad never happens." Example: G ¬(red ∧ green)
  - A safety property is violated by a *finite* prefix — you can witness the violation.
- **Liveness**: "something good eventually happens." Example: G(request → F granted)
  - A liveness property is not violated by any finite prefix — you must see the infinite behavior.

This distinction is crucial for verification: safety can often be checked by bounded model checking, while liveness requires reasoning about infinite paths.

## Historical Note

Temporal logic was introduced by Arthur Prior in the 1950s as a formalization of *tense logic* in philosophy — a way to rigorously reason about sentences like "it was the case that..." and "it will be the case that...". Amir Pnueli (1977) recognized its application to program verification and received the Turing Award in 1996 for this insight. The connection between temporal specification and automatic verification (model checking) transformed how we build reliable software and hardware.
