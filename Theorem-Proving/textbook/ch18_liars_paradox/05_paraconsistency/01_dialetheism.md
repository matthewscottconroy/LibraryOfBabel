# Dialetheism and Paraconsistency

What if the Liar is *both true and false*? This is the dialetheist position: some contradictions are true.

## Dialetheism

*Dialetheism* (Graham Priest, 1987) holds that some sentences are *dialetheia* — both true and false. The Liar sentence L, far from being paradoxical, is simply true and false simultaneously.

This is not a contradiction in the pejorative sense, for the dialetheist — it is a *true contradiction*, a *dialetheia*. Not all contradictions are dialetheias (most propositions are simply true or simply false), but some genuinely are.

Motivations:
- The Liar and Curry paradoxes seem irreducible — every proposed solution generates "revenge paradoxes"
- Set-theoretic paradoxes (Russell) may similarly resist consistent treatment
- Gödel sentences are provable-and-unprovable (in some sense), suggesting contradictions at the heart of mathematics

## Paraconsistent Logic

If some contradictions are true, we need a logic that can tolerate them without *explosion* — the classical rule ex contradictione quodlibet: from P ∧ ¬P, derive Q (for any Q).

*Paraconsistent logics* block explosion while preserving most of classical reasoning:

**LP (Logic of Paradox)**: Three truth values: True (T), False (F), Both (B).
- T and B make a sentence "designated" (assertable)
- Conjunction, disjunction work as usual; negation maps T→F, F→T, B→B
- Modus ponens is valid; explosion fails because B ∧ ¬B = B (not F)

**Relevance logic**: Requires that premises and conclusions share propositional content — avoiding irrelevant logical connections.

**Priest's LP**: The Liar sentence L has truth value B (both). T(⌈L⌉) ↔ L holds (T-schema is retained), and L ↔ ¬L holds — but this is not explosive because LP tolerates true contradictions.

## The Revenge Problem

Every proposed solution to the Liar faces *revenge paradoxes* — stronger versions that reassert the problem:

- **Against truth-gaps (Kripke)**: "This sentence is false or gappy." If true: it's false or gappy — so either false (paradox) or gappy. If gappy: it's true by the second disjunct. Contradiction.
- **Against dialetheism**: "This sentence is only false." If true: it's only false — contradiction with being true. If both: it's only false — contradiction with being true. If false: it says "only false," which is satisfied — so it's true.
- **Against hierarchy (Tarski)**: "This sentence is false at every level."

The revenge paradoxes suggest no simple theory of truth can solve the Liar once and for all. The paradox may be *ineliminable from any sufficiently expressive language*, pointing to a deep limit on the concept of truth itself.
