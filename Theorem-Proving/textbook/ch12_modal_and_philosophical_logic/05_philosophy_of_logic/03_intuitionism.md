# Intuitionism and Constructivism

## Overview
**Intuitionism** (Brouwer 1907) holds that mathematics is a mental construction, and
mathematical objects and truths exist only insofar as they are mentally constructed.
This has radical implications for logic: the law of excluded middle is not universally valid.

## Learning Objectives
- State Brouwer's intuitionist philosophy
- Understand why intuitionists reject LEM
- Connect intuitionism to constructive proof and the BHK interpretation
- Appreciate the connection to Lean/Coq's default logic

## Brouwer's Key Claims
1. Mathematical objects are mental constructions, not discovered Platonic entities
2. A mathematical statement is true iff we have a construction (proof) of it
3. ∀x P(x) means: we have a method to construct P(a) for any given a
4. ∃x P(x) means: we have a specific x and a construction of P(x)
5. ¬P means: we have a method showing P leads to absurdity

## Why Intuitionists Reject LEM
P ∨ ¬P would require, for any P, either a proof of P or a proof of ¬P.
For open problems (like the Goldbach conjecture), we have neither — so LEM fails.
Intuitionists say: "assert only what you can prove; don't assert P ∨ ¬P when you know neither."

## Heyting's Formalization
Arend Heyting (1930) formalized intuitionistic logic, giving us a proof system weaker than
classical logic. Notable:
- The double negation translation (Gödel 1933): every classical theorem translates to an
  intuitionistic theorem — classical math embeds in intuitionistic logic
- Intuitionistic FOL is complete w.r.t. Kripke semantics (not classical truth tables)

## Martin-Löf Type Theory
Per Martin-Löf (1970s) developed intuitionistic type theory as a foundation for
constructive mathematics. This is the basis for Lean 4, Coq, and Agda.

## Exercises
See `problems/ch12_modal_logic/01_modal_logic_exercises.md`
