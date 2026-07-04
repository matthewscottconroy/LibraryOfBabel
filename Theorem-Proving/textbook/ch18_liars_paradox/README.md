# Chapter 18: The Liar's Paradox and Self-Reference

"This sentence is false." The sentence that breaks naive theories of truth — and launched a thousand foundational investigations.

## Overview

The Liar's paradox arises from two innocent-looking commitments: the **T-schema** ($T(\ulcorner\phi\urcorner) \leftrightarrow \phi$ for every sentence $\phi$) and the availability of self-reference. This chapter shows that self-reference is not an eliminable trick: the **diagonal lemma** guarantees that any theory representing its own syntax contains, for every formula $\psi(x)$, a sentence $\lambda$ provably equivalent to $\psi(\ulcorner\lambda\urcorner)$. Applied to "$x$ is not true," diagonalization yields the formal Liar and with it **Tarski's undefinability theorem** (1933): no consistent theory extending Robinson arithmetic can define its own truth predicate. Alongside the Liar we study its cousins — the underdetermined Truthteller, **Curry's paradox** (which derives absurdity from a detachable conditional and contraction alone, with no negation), Yablo's non-self-referential infinite variant, and Kripke's empirically contingent liars.

The classical response is Tarski's **object language / metalanguage hierarchy** $L_0 \subset L_1 \subset L_2 \subset \cdots$ with typed truth predicates $T_n$, governed by **Convention T** (material adequacy); the Liar's fixed-point equation becomes unsolvable, at the price of a universal truth predicate. Kripke's *Outline of a Theory of Truth* (1975) instead builds a single language with a partial, self-applicable truth predicate: using the three-valued **Strong Kleene** scheme, the monotone jump operator on partial interpretations $(E, A)$ has **fixed points** (via transfinite iteration and the Knaster–Tarski argument), and **groundedness** — having a value in the least fixed point — separates healthy truth-talk from pathology: the Liar is valueless in every fixed point, the Truthteller merely undetermined across them.

But gap theories face the **strengthened Liar** ("this sentence is not true") and the general **revenge** phenomenon: each theory's own classifications ("gappy," "ungrounded," "indeterminate") feed a new diagonal sentence, so consistency is bought with expressive incompleteness — Kripke's "ghost of the Tarski hierarchy." The final section examines the paraconsistent escape: Priest's **LP** (Logic of Paradox), where the Liar takes the glutty value $\mathbf{B}$, explosion and material modus ponens fail (worked countermodels), yet exactly the classical tautologies remain valid. **Dialetheism** — the thesis that some contradictions are true — claims full expressive completeness, though Curry's paradox still forces a choice between detachment and contraction.

## Why It Matters

The Liar is Gödel's engine: the diagonal lemma that generates it is the same construction behind the incompleteness theorems of Chapter 10 — Gödel's sentence is the Liar with the *definable* predicate "provable" in place of the *undefinable* "true," and Tarski's theorem is the cleanest explanation of why truth outruns proof. Tarski's positive construction — the compositional definition of satisfaction — is the foundation of truth in a model (Chapters 3 and 9). And the hierarchy survives as engineering practice in proof assistants: object-level syntax manipulated by metaprograms, reflection principles, and the universe hierarchy that blocks Girard's paradox, type theory's own Liar.

## Chapter Roadmap

1. [The Paradox](01_paradox/01_the_liar.md) — the Liar informally and formally; Gödel numbering, the diagonal lemma, Tarski's undefinability theorem; Truthteller, Curry, Yablo, contingent liars.
2. [Tarski's Hierarchy](02_tarski/01_tarski_hierarchy.md) — object vs metalanguage, Convention T, the typed tower $T_n$, the satisfaction definition, costs of stratification.
3. [Kripke's Fixed-Point Construction](03_kripke/01_truth_gaps.md) — Strong Kleene tables, the jump operator, monotonicity, least and intrinsic fixed points, groundedness.
4. [Revenge Paradoxes](04_revenge/01_strengthened_liar.md) — the strengthened Liar, the revenge recipe, Field's determinacy hierarchy, contextualism, a formal criterion.
5. [Dialetheism and Paraconsistency](05_paraconsistency/01_dialetheism.md) — LP truth tables, failure of explosion and modus ponens, classical recapture, Curry's constraint, paraconsistent automated reasoning.

## Prerequisites

Chapters 2–3 (propositional and first-order logic) are assumed throughout. Chapter 10 (computability, Gödel numbering, incompleteness) is strongly recommended for the diagonal lemma and undefinability theorem; Chapter 9 (model theory) illuminates the satisfaction definition.
