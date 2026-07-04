# Chapter 1: Language and Logic Foundations

What separates a *rigorous* argument from a merely persuasive one? Answering that question forces us out of natural language and into formal languages — the medium of every proof assistant.

## Overview

Natural language fails the standards of rigorous reasoning in two distinct ways: **ambiguity** — lexical (the equivocation on "nothing" in the ham-sandwich syllogism), syntactic ("Every student read a book": $\forall s\, \exists b$ vs. $\exists b\, \forall s$), and pragmatic — and **vagueness**, exemplified by the Sorites paradox. The failures are not hypothetical: Cauchy's long-accepted "proof" that a convergent sequence of continuous functions has a continuous limit rests on a quantifier-order confusion. A **formal language** eliminates both problems by fiat: an alphabet $\Sigma$, a recursive (BNF) grammar generating exactly the **well-formed formulas** (wffs), and a compositional semantics. Two structural results anchor the approach: the **Unique Readability Theorem** (every wff has exactly one parse tree, so meaning is a determinate function of syntax) and Tarski's **object language / metalanguage** stratification with its **T-schema** — $\mathrm{True}(\ulcorner\phi\urcorner) \iff \phi$ — which blocks the Liar paradox.

The chapter then fixes logic's subject matter and standards. A **proposition** is a truth-apt content (Frege's sense vs. reference, Russell's structured propositions, `Prop` in Lean 4); sentences, utterances, statements, and beliefs compete as **truth-bearers**; and **logical form** (Russell's analysis of "the present king of France") is extracted by regimentation. An **argument** is premises $P_1, \ldots, P_n$ plus a conclusion $C$: it is **valid** iff the conclusion is true in every model in which all premises are true, and **sound** iff it is valid and all premises are true. Validity is a property of form, not content — a single **counterexample** refutes — and the classic **fallacies** (affirming the consequent, denying the antecedent, undistributed middle; ad hominem, strawman, false dichotomy, begging the question) are diagnosed structurally.

Finally, **proof**: a formal proof is a finite sequence of formulas, each an axiom or obtained from earlier lines by an explicit inference rule, witnessing $\Gamma \vdash \phi$. The chapter states the bridge results proved later — **soundness** (if $\Gamma \vdash \phi$ then $\Gamma \vDash \phi$) and **completeness** (the converse) — and traces the spectrum from informal argument through Wiles's repaired proof of Fermat's Last Theorem to machine-checked objects such as Gonthier's Coq proof of the four-color theorem.

## Why It Matters

Every proof assistant — Lean 4, Coq, Isabelle — is built on this chapter's thesis: natural language is too ambiguous for verified reasoning, and syntactic checking (does the proof term typecheck?) can certify semantic truth. The concepts introduced here — wff, validity, soundness, $\vdash$ vs. $\vDash$, proof vs. persuasion — are presupposed everywhere: Chapter 2 instantiates them for propositional logic, Chapter 3 for first-order logic, Chapter 4 for proof systems, and Chapter 10 returns to Hilbert's program and its Gödelian limits.

## Chapter Roadmap

1. [Informal vs. Formal Language](01_informal_vs_formal_language/01_natural_language_and_ambiguity.md) — The three faces of ambiguity plus vagueness and the Sorites paradox; the companion files define formal languages (alphabet, BNF grammar, unique readability, object vs. metalanguage) and develop the syntax/semantics distinction with valuations, entailment, and Tarski's hierarchy.
2. [Propositions and Statements](02_propositions_and_statements/01_what_is_a_proposition.md) — What a proposition is, from Frege's Thoughts to Lean's `Prop`; further files survey candidate truth-bearers and the indexicality problem, then logical form and regimentation.
3. [Arguments, Validity, and Soundness](03_arguments_validity_soundness/01_anatomy_of_an_argument.md) — Premises, conclusions, indicator words, and standard form; then the validity/soundness distinction with the counterexample method, and the catalogue of formal and informal fallacies.
4. [Introduction to Proof](04_introduction_to_proof/01_what_is_a_mathematical_proof.md) — Proof as convincing argument, as symbol manipulation, and as machine-checked object; an overview of the book's tools (Lean 4, Coq, Tarski's World/Carnap, Python, Haskell); and proof vs. persuasion, where authority and intuition carry no weight.

## Prerequisites

None. This chapter is self-contained — it is the entry point of the book, assuming only general mathematical maturity.
