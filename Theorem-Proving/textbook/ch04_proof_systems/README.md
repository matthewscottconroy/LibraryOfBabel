# Chapter 4: Proof Systems

What, precisely, is a proof? This chapter makes the question mathematical — defining four proof systems and asking how we know each proves all and only the right things ($\Gamma \vdash \phi$ iff $\Gamma \vDash \phi$).

## Overview

The heart of the chapter is **natural deduction** (Gentzen 1935): no axioms, only rules — one **introduction rule** and one **elimination rule** per connective, balanced by the principle of **harmony**. The catalogue: $\land I$ and $\land E_{1,2}$; $\lor I_{1,2}$ and $\lor E$ (proof by cases); $\to I$ (assume $\phi$, derive $\psi$, **discharge** the hypothesis) and $\to E$ (modus ponens); $\neg I$, $\neg E$, and **explosion** ($\bot E$, ex falso quodlibet); plus the quantifier rules $\forall I$ (arbitrary variable, with its freshness side condition), $\forall E$ (instantiation), $\exists I$ (witness), and $\exists E$ (fresh witness variable). These rules alone yield **intuitionistic logic**; adding **double negation elimination** — equivalently the law of excluded middle $\phi \lor \neg\phi$, or reductio ad absurdum — yields classical logic. Introduce-then-eliminate detours reduce away (local reduction), the germ of the **normalization theorem** and of Curry–Howard: $\to I$ is lambda abstraction, $\to E$ is function application.

**Sequent calculus** (Gentzen's LK) reformulates proof symmetrically: sequents $\Gamma \vdash \Delta$, left and right rules for each connective, and the **structural rules** — weakening, contraction, exchange — whose selective removal generates the substructural logics (linear, affine, relevant, ordered). Its central metatheorem is **cut elimination** (Gentzen's Hauptsatz): every proof using the cut rule can be transformed into a cut-free proof, giving the **subformula property**, a consistency proof, and terminating propositional proof search. Two refutation-based systems complete the picture: **resolution** (Robinson 1965) — derive the empty clause $\square$ from CNF clauses, lifted to FOL by **unification** and most general unifiers — and **semantic tableaux**, a mechanical decision procedure in which a closed tree proves validity and an open branch yields a countermodel. The chapter overview also covers **Hilbert systems** (three axiom schemas plus modus ponens) and the metatheorems **soundness** and **completeness**, the latter via the Henkin construction (Gödel's completeness theorem, 1930).

## Why It Matters

The choice of proof system is not stylistic: natural deduction corresponds to lambda calculus and is the skeleton of Lean 4 and Coq; cut-free sequent proofs drive proof search; resolution powers SAT solvers and Prolog; tableaux underlie DPLL. Chapter 5's proof strategies are structural patterns inside these systems, Chapter 11's Curry–Howard correspondence turns these proofs into programs, and Chapter 13's verification tools automate the systems defined here.

## Chapter Roadmap

1. [Natural Deduction](01_natural_deduction/01_introduction_and_elimination_rules.md) — The introduction/elimination paradigm and harmony, then one file per rule family: conjunction (local reduction and normalization), disjunction (case analysis, sum types), implication (hypothesis discharge, the lambda calculus connection), negation (explosion, DNE, classical vs. intuitionistic), and the quantifier rules with their freshness conditions.
2. [Sequent Calculus](02_sequent_calculus/01_sequents_and_rules.md) — Sequents $\Gamma \vdash \Delta$, left/right rules, the cut rule, and cut elimination with its consequences; the second file develops weakening, contraction, and exchange, and the substructural logics obtained by dropping them.
3. [Resolution](03_resolution/01_resolution_principle.md) — The resolution rule on clauses, refutation completeness, and first-order resolution via unification and MGUs; `resolution.py` implements a working propositional refutation prover.
4. [Tableaux](04_tableaux/01_semantic_tableaux.md) — Semantic tableaux as a decision procedure: branching and non-branching rules, closed tableaux, countermodel extraction from open branches, and a compact Python implementation.
5. [Proof Assistants Workflow](05_proof_assistants_workflow/01_lean4_workflow.lean) — The Lean 4 proving cycle (stating theorems, term vs. tactic mode, reading goal states, `sorry` stubs); the companion Coq file shows `Proof`/`Qed`, bullet-structured proofs, `Search`, and `Admitted`.

## Prerequisites

Chapters 2 and 3: the propositional connectives with their truth-table semantics, CNF, and the quantifiers with the free/bound variable discipline. Chapter 1's distinction between derivability ($\vdash$) and semantic consequence ($\vDash$) frames the soundness and completeness results proved about these systems.
