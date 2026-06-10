# Constructive and Intuitionistic Logic: Mathematics Under a Microscope

> *"A mathematical theorem, once proved, exists in a realm apart from physical reality, and no future discovery can falsify it. But: which theorems have we actually proved? That depends on which rules of inference are legitimate."*

---

Imagine mathematics as a building. Classical mathematics, built on ZFC set theory and classical logic, is a cathedral — enormous, richly detailed, with flying buttresses of non-constructive argument supporting grand structures that could not stand without them. Constructive mathematics, built on intuitionistic logic and type theory, is a different kind of structure — smaller in some dimensions, but with load-bearing walls at every level: every theorem comes with an algorithm, every existence proof comes with a witness.

The choice between these approaches is not just philosophical. It has concrete consequences for what can be computed, what can be verified, and what counts as a proof. It is encoded in the design of Lean and Coq, which are constructive by default. And it is one of the most fascinating debates in the foundations of mathematics.

## Brouwer's Challenge

L.E.J. Brouwer (1881–1966) was one of the twentieth century's most brilliant and contrarian mathematicians. He founded the topology of manifolds (Brouwer Fixed-Point Theorem, Invariance of Domain) and simultaneously launched a foundational revolution that his contemporaries found infuriating.

Brouwer's **intuitionism** rests on a simple but radical premise: *mathematics is a mental activity*. Mathematical objects are not discovered in a Platonic realm of abstract entities; they are constructed by the human mind. A mathematical statement is not true because it corresponds to some mind-independent fact; it is true because we have constructed a proof of it.

This view has an immediate consequence for the Law of Excluded Middle (LEM): P ∨ ¬P. To assert P ∨ ¬P is to claim that either P has been proved or ¬P has been proved. But there are propositions — the Goldbach conjecture, the Riemann hypothesis, countless others — for which neither proof exists. We cannot assert their truth or their falsehood.

Brouwer's conclusion: LEM is not a logical law. It is, at best, a methodological maxim ("keep looking for a proof or a disproof") but not something we are entitled to assert in an argument before such a proof has been found.

## The Brouwer-Heyting-Kolmogorov (BHK) Interpretation

What, then, is the meaning of logical operators from an intuitionistic perspective? Arend Heyting (Brouwer's student) and Andrei Kolmogorov (independently) formulated the **BHK interpretation**, which gives a proof-theoretic meaning to each logical operator:

| Statement | Intuitionistic Meaning |
|-----------|----------------------|
| P ∧ Q | A pair: a proof of P and a proof of Q |
| P ∨ Q | Either a proof of P, or a proof of Q (with a label indicating which) |
| P → Q | A method (function) that converts any proof of P into a proof of Q |
| ¬P | A method that derives a contradiction from any proof of P (i.e., P → ⊥) |
| ∀x P(x) | A method that, given any x, constructs a proof of P(x) |
| ∃x P(x) | A pair: a specific witness t, and a proof that P(t) holds |
| ⊤ | Trivially proved |
| ⊥ | Cannot be proved (no proof exists) |

The BHK interpretation makes the computational content of proofs explicit. A proof of P ∨ Q is not just "one of them is true" — it is a *labeled certificate* indicating which one. A proof of ∃x P(x) is not just "something satisfying P exists" — it is an *explicit witness* plus a *proof about that witness*.

This is exactly the Curry-Howard correspondence (Chapter 11): propositions are types, and proofs are programs. The BHK interpretation is the intuitionistic semantics; the Curry-Howard correspondence is its type-theoretic implementation.

## What LEM Buys You (and What It Costs)

The Law of Excluded Middle enables three non-constructive proof techniques:

**1. Classical proof by contradiction** (RAA): To prove P, assume ¬P and derive ⊥. This requires LEM to conclude P from ¬¬P (double negation elimination), which is not valid intuitionistically.

**2. Non-constructive existence**: Prove ∃x P(x) by showing the assumption ∀x ¬P(x) leads to contradiction. This proves existence without identifying a witness.

**3. Case-splitting on arbitrary propositions**: `if P then A else B` is valid classically for any P; intuitionistically, you need a *decision procedure* for P.

These techniques are extraordinarily powerful. Many of the deepest theorems in analysis, algebra, and combinatorics use them. The Bolzano-Weierstrass theorem (every bounded sequence has a convergent subsequence), the Hahn-Banach theorem, and much of modern measure theory rely on non-constructive arguments.

The cost: proofs using LEM do not carry computational content. A non-constructive existence proof does not tell you how to find the witness; it only tells you the witness exists. This is fine if your goal is to establish mathematical facts. It is a problem if your goal is to *extract programs* from proofs.

## The Gödel Translation: Classical in Intuitionistic Disguise

A remarkable result by Gödel (1933) shows that classical logic and intuitionistic logic are not as far apart as they seem. Gödel defined a translation (•)° from classical to intuitionistic logic such that:

- φ is classically provable if and only if φ° is intuitionistically provable

The key ingredient: φ° replaces every subformula ψ with ¬¬ψ. Double negation translates classical truths into constructively assertable truths.

This shows that classical mathematics *embeds* into intuitionistic mathematics — there is no classical theorem that is inconsistent with intuitionistic mathematics. The difference is not about what is true but about what *follows from what* in a single deductive step.

## Lean 4 and Coq: The Design Choice

Lean 4 includes classical axioms by default (via `import Mathlib` or `open Classical`). This means you can use `Classical.em : ∀ p : Prop, p ∨ ¬p` and `Classical.byContradiction` without restriction. Most of Mathlib relies on classical logic.

Coq is constructive by default but includes classical axioms in a library (`Require Import Classical`). The core calculus (CIC) is constructive; classical reasoning is opt-in. This design allows Coq to extract verified programs from proofs — a program extracted from a constructive proof is guaranteed to compute the right answer; extracting from a classical proof may not give a useful program.

Agda is constructive with no classical axioms in the standard library. Proofs in Agda are automatically computational.

> **A Design Exercise**: Suppose you want to write a library of sorting algorithms with correctness proofs. If you use classical logic, you can prove "the sorted output satisfies the specification" without the proof giving you the algorithm. If you use constructive logic, the proof *is* the algorithm — the sorting function and its correctness proof are the same thing, extracted automatically. Which would you prefer? Under what circumstances?

## Mathematics Without LEM: What Survives?

A surprisingly large amount of mathematics is constructively valid. All of:
- Propositional logic (without LEM)
- Arithmetic (with induction and primitive recursion)
- Algebra (groups, rings, fields, modules — all the structural theory)
- Constructive analysis (Bishop's constructive analysis, developed in the 1960s)
- Combinatorics
- Category theory (most of it)

The cases where classical reasoning is genuinely necessary — where no constructive proof exists — are more localized: certain theorems about real numbers, the Axiom of Choice and its consequences, some theorems in set theory.

The constructive program has another advantage: it is *safer*. A constructive proof assistant with no classical axioms will never silently accept a proof of `False`. A classical system must be carefully designed to ensure that the classical axioms are consistent (which ZFC is, assuming large cardinals, but this consistency cannot be proved within ZFC — see Gödel's second incompleteness theorem, Chapter 10).

---

*Next: Chapter 6 — Set Theory: the language and the paradoxes.*
