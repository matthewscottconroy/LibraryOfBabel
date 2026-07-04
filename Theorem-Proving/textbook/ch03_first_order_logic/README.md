# Chapter 3: First-Order Logic

Propositional logic can say "it is raining" but not "all humans are mortal." First-order logic (FOL) adds objects, relations, and quantifiers — the language in which virtually all of modern mathematics is written.

## Overview

Syntax first: a **signature** $\sigma$ supplies constant, function, and relation symbols, each with a fixed **arity**; **terms** (variables, constants, function applications) denote individuals in a **domain of discourse**, and **atomic sentences** $R(t_1, \ldots, t_n)$ — plus the identity $t_1 = t_2$ — make the simplest claims. The **quantifiers** carry the expressive power. $\forall x\, P(x)$ behaves like an infinite conjunction: falsified by a single counterexample, vacuously true over an empty extension, and almost always paired with a conditional ("Every $F$ is $G$" is $\forall x(F(x) \to G(x))$, *not* $\forall x(F(x) \land G(x))$). $\exists x\, P(x)$ is proved by producing a **witness**, generalizes to unique existence $\exists!$, and exposes the classical/constructive divide through non-constructive existence proofs (the $\sqrt{2}^{\sqrt{2}}$ argument). **Nested quantifiers** are where the power and the danger live: $\forall x\, \exists y\, R(x,y)$ and $\exists y\, \forall x\, R(x,y)$ differ exactly as pointwise differs from uniform continuity — a distinction readable as a two-player game — and the **quantifier negation laws** $\neg\forall x\, P \equiv \exists x\, \neg P$ and $\neg\exists x\, P \equiv \forall x\, \neg P$ flip quantifiers mechanically. A variable occurrence is **free** or **bound** according to quantifier scope; a **sentence** has no free variables; substitution must avoid **variable capture** via $\alpha$-renaming.

The semantics is Tarski's: a **structure** $\mathcal{M}$ fixes a non-empty domain and interprets every symbol of $\sigma$; the **satisfaction relation** $\mathcal{M}, s \vDash \phi$ is defined recursively over variable assignments $s$, yielding truth in a model, validity, and logical consequence $\Gamma \vDash \phi$. The same sentence can be true in $(\mathbb{Z}, +, 0)$ and false in $(\mathbb{N}, +, 0)$ — truth is model-relative. The chapter overview supplies the metatheory by name: **Herbrand's theorem**, **unification** and most general unifiers (Robinson 1965) with the occurs check, and the Church–Turing **undecidability** of FOL — validity is only semi-decidable, with decidable fragments (monadic, two-variable, guarded).

## Why It Matters

FOL is the lingua franca of formal mathematics: Peano arithmetic and ZFC set theory are first-order theories. Translation between English and FOL is the daily craft of writing specifications and theorem statements; structures and satisfaction found model theory (Chapter 9); Herbrand's theorem plus unification found automated theorem proving; and the quantifier rules become natural deduction rules in Chapter 4 and everyday Lean 4 / Coq idioms (`intro`, `obtain`, anonymous constructors) thereafter.

## Chapter Roadmap

1. [Terms, Predicates, and Atomic Sentences](01_terms_predicates_atomic_sentences/01_individuals_and_domain.md) — Constants, variables, predicates, functions, and arity; then atomic sentences with identity, and an introduction to the Tarski's World / Carnap blocks-world software.
2. [Quantifiers](02_quantifiers/01_universal_quantification.md) — Universal quantification (semantics, vacuous truth, instantiation and introduction); companion files cover existential quantification and witnesses, nested quantifiers and quantifier order, and evaluating quantified sentences in Tarski's World.
3. [Free and Bound Variables](03_free_and_bound_variables/01_scope_and_binding.md) — Scope, binding, sentences vs. open formulas, and capture-avoiding substitution.
4. [Translation](04_translation/01_english_to_fol.md) — Systematic English-to-FOL patterns ("every," "some," "no," "only," "exactly one"); the reverse FOL-to-English direction; and a pattern library including Aristotle's four categorical forms, injectivity, and $\varepsilon$-$\delta$ continuity.
5. [Models and Interpretations](05_models_and_interpretations/01_structures.md) — Signatures, structures, term evaluation, and Tarski's truth definition with the T-schema; plus a Python blocks-world model checker implementing the satisfaction relation.
6. [FOL in Lean and Coq](06_fol_in_lean_and_coq/01_fol_in_lean.lean) — Quantifier rules as function application and anonymous constructors in Lean 4, with the quantifier negation lemmas; the Coq counterpart adds classical quantifier negation and the Peano axioms on `nat`.

## Prerequisites

Chapters 1–2: well-formed formulas, the syntax/semantics distinction, the truth-functional connectives, and the notions of tautology and logical consequence. FOL retains all of the propositional machinery and adds terms and quantifiers on top of it.
