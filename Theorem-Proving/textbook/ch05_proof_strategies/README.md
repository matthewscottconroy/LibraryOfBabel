# Chapter 5: Proof Strategies

The art of *finding* proofs — not as mystical inspiration but as a catalogue of strategies, each corresponding to a precise structural pattern in natural deduction and to a tactic in a modern proof assistant.

## Overview

Formal proof systems (Chapter 4) tell us what counts as a valid proof; they do not tell us how to discover one. This chapter bridges that gap. Its central question: given a goal $\phi$, which strategy should you reach for, and why does that strategy constitute a valid argument? We treat each major strategy in turn: **direct proof** (to prove $P \to Q$, assume $P$ and derive $Q$ — the $\to I$ rule), **proof by contradiction** (*reductio ad absurdum*, RAA: to prove $P$, assume $\neg P$ and derive $\bot$), **proof by contrapositive** (use the classical equivalence $(P \to Q) \equiv (\neg Q \to \neg P)$), **proof by cases** (exhaustive case analysis via the $\lor E$ rule, typically justified by the law of excluded middle), **existence and uniqueness proofs** (constructive witnesses via $\exists I$, uniqueness via "assume two, show equal," and their combination $\exists!x\, P(x)$), and **induction** as a strategy (choosing the induction variable, strengthening the inductive hypothesis).

The chapter's showcase proofs are classics: the irrationality of $\sqrt{2}$ and Euclid's infinitude of primes (both by contradiction), "if $n^2$ is odd then $n$ is odd" (by contrapositive), $n(n+1)$ is even (by cases), the uniqueness of a group's identity element, and the non-constructive proof that irrational $a, b$ with $a^b$ rational exist (via $\sqrt{2}^{\sqrt{2}}$).

A dedicated section examines the fault line between **constructive (intuitionistic) and classical logic**: Brouwer's intuitionism, the **BHK interpretation** (a proof of $P \lor Q$ is a labeled certificate; a proof of $\exists x\, P(x)$ is a witness plus evidence), why double negation elimination and LEM ($P \lor \neg P$) fail intuitionistically, and **Gödel's double-negation translation** embedding classical logic into intuitionistic logic. Key heuristic: a proof by contradiction that never uses its negation hypothesis is a direct proof in disguise.

## Why It Matters

Proof strategies are the bridge between formal rules and mathematical practice, and every strategy here is directly executable: Lean 4's `intro`, `apply`, `by_contra`, `contrapose!`, `rcases`, `use`, and `induction` tactics implement exactly these patterns, as do Coq's counterparts. The chapter equips you to write the informal proofs of Chapters 6–9 (set theory, induction, number theory) and to understand why Lean and Coq are constructive by default — proofs without LEM carry computational content, enabling program extraction (Chapter 11's Curry-Howard correspondence).

## Chapter Roadmap

1. [Direct Proof](01_direct_proof/01_method_structure_examples.md) — assume $P$, derive $Q$; worked examples on parity and transitivity of divisibility, plus a companion Lean 4 file.
2. [Proof by Contradiction](02_proof_by_contradiction/01_reductio_ad_absurdum.md) — the RAA rule, irrationality of $\sqrt{2}$, infinitude of primes, and when contradiction is classically necessary; companion Coq file.
3. [Proof by Contrapositive](03_proof_by_contrapositive/01_the_contrapositive.md) — the equivalence $(P \to Q) \equiv (\neg Q \to \neg P)$, contrapositive versus contradiction, and Lean's `contrapose!`.
4. [Proof by Cases](04_proof_by_cases/01_exhaustive_case_analysis.md) — exhaustive case analysis via $\lor E$ and LEM, with `rcases`/`by_cases` in a companion Lean file.
5. [Existence and Uniqueness](05_existence_and_uniqueness/01_existence_proofs.md) — constructive versus non-constructive existence; the second file develops uniqueness proofs and $\exists!$.
6. [Constructive vs Classical](06_constructive_vs_classical/01_intuitionistic_logic.md) — Brouwer, the BHK interpretation, what LEM buys and costs, the Gödel translation, and design choices in Lean, Coq, and Agda.
7. [Proof by Induction](07_proof_by_induction/01_induction_as_strategy.md) — induction as a strategy: picking the variable, strengthening the IH, common pitfalls; developed fully in Chapter 7.

## Prerequisites

Chapters 2–4: propositional and first-order logic, and the natural deduction rules ($\to I$, $\lor E$, $\exists I$, RAA) that each strategy formalizes.
