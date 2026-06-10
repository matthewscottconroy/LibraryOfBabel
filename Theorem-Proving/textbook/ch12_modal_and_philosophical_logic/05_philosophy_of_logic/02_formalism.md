# Formalism in Mathematics

## Hilbert's Program

**Mathematical formalism** holds that mathematics is the manipulation of meaningless symbols according to explicit rules. Mathematical objects do not exist independently; a "set" or "number" is just a formal symbol in a game. What matters is that the rules are consistent and useful.

The most famous formalist was **David Hilbert**, whose **Hilbert Program** (1920s) aimed to:
1. Formalize all of mathematics in a complete, consistent axiomatic system
2. Prove the consistency of this system using only "finitary" (safe, finitely checkable) means

**The Formalist's Slogan**: "Mathematics is the science of infinite formal systems."

## The Appeal of Formalism

Formalism avoids the metaphysical mysteries of Platonism. You do not need to explain how humans have epistemic access to abstract mathematical objects — there are no such objects. Mathematics is just formal symbol manipulation, which is a perfectly concrete activity (humans or machines do it).

Formalism also fits naturally with the development of computer science: programs are formal systems manipulating symbols. The connection between mathematical proof and computation (the Curry-Howard correspondence) is very natural from a formalist perspective.

## Gödel's Refutation of Hilbert's Program

Gödel's First Incompleteness Theorem (1931) showed that no consistent, computably axiomatizable formal system containing arithmetic is **complete** — there will always be true-in-the-standard-model sentences that are formally unprovable.

Gödel's Second Incompleteness Theorem added the devastating blow: the consistency of any such system cannot be proved within the system itself. Hilbert's finitary consistency proof is impossible.

This does not eliminate formalism entirely — mathematicians can still work in formal systems and produce machine-checkable proofs. But it shows that formal systems are incomplete snapshots of mathematical truth, not its complete capture.

## Modern Formalism: Proof Assistants

In a sense, proof assistants (Lean, Coq, Isabelle) realize Hilbert's dream, minus the completeness and consistency-from-within. We do have:
- Fully formalized mathematical proofs (four-color theorem, Kepler conjecture, etc.)
- Machine-checkable verification down to foundational axioms
- A growing body of formally verified mathematics (Mathlib)

What we cannot have (by Gödel): a guarantee from within the system that the system is consistent. We trust the foundations (ZFC, CIC) based on their long track record and the coherence of mathematics built on them — a pragmatic, not absolute, confidence.

## Exercises
See [problems/ch12_modal_logic/05_philosophy_exercises.md](../../../problems/ch12_modal_logic/05_philosophy_exercises.md)
