# Chapter 4: Proof Theory

## What This Chapter Is About

Mathematics is built from proofs. But what *is* a proof, precisely?

In practice, a proof is a convincing argument — a sequence of logical steps that leads from known facts to a new conclusion. But this description is far too loose for foundational purposes. Different mathematicians have different standards for what counts as "convincing." Steps that one mathematician considers obvious another might require justification. And when we try to formalize mathematics in a computer (as in Lean or Coq), "convincing argument" won't do — we need an exact, mechanical criterion for proof-checking.

Proof theory is the mathematical study of proofs as formal objects. It asks:
- What is the minimal, explicit structure of a valid proof?
- Are there normal forms — canonical "simplified" versions of every proof?
- What is the relationship between a proof and the thing it proves?
- What can't be proved, and why?

The answers to these questions are not just technical curiosities. They are the foundation of type theory and, ultimately, of Homotopy Type Theory.

## The Central Theme: Proofs as Structures

The main insight of proof theory is that proofs have *internal structure* that matters.

In classical logic, all that matters about a proof is whether it's valid — whether it really establishes its conclusion. Two different proofs of the same theorem are "the same proof" from the classical point of view.

In constructive logic, proof theory, and type theory, different proofs of the same theorem can be fundamentally different objects. A proof of "there exists a natural number $n$ with property $P(n)$" not only establishes that such an $n$ exists but contains within it an actual witness — a specific $n$ and a proof that $P(n)$ holds. Two proofs might exhibit different witnesses, making them different proofs in a meaningful sense.

This shift — from proofs as certificates of truth to proofs as computational objects — is what the Curry-Howard correspondence formalizes. It's the conceptual bridge from logic to type theory.

## The Roadmap

This chapter develops proof theory from scratch.

**Section 1: Judgments.** A proof operates within a *context* that specifies what is known. A *judgment* is a formal declaration of the form "in this context, this proposition holds." We study the structure of judgments and what it means for a judgment to be derivable.

**Section 2: Natural Deduction.** The most natural way to formalize proofs is *natural deduction*, a system of formal rules for constructing proofs by "introducing" and "eliminating" logical connectives. We develop the full system for propositional and predicate logic, including the classical/intuitionistic distinction.

**Section 3: Normalization.** Raw proofs in natural deduction can contain redundancy — "detours" where you introduce a connective only to immediately eliminate it. Normalization is the process of simplifying proofs by removing detours. The normalization theorem says every proof can be put into a canonical "normal form" — no detours. This has profound consequences for the structure of provable formulas.

**Section 4: Sequent Calculus.** An alternative to natural deduction, developed by Gentzen in the same 1935 paper. Sequent calculus is more symmetric and better suited for certain theoretical investigations (in particular, for proving consistency). The main theorem — Gentzen's *Hauptsatz* (main theorem), also called cut elimination — is one of the great results of 20th-century logic.

Throughout, we'll emphasize the connections to type theory: judgments become typing judgments, proof rules become type formation rules, and normalization becomes computation.

## What You Should Know Coming In

This chapter assumes:
- Chapter 0 (Logic and Proof): comfort with logical connectives, quantifiers, and proof by induction.
- Chapter 1 (Set Theory): basic familiarity with sets and functions.

The material here is more abstract and technical than the previous chapters. Take your time with the formal systems — they're dense but precise, and precision is the whole point.

## A Historical Note

The formal systems in this chapter were developed in the 1930s by Gerhard Gentzen, who was 25 years old when he published his landmark 1935 paper *Untersuchungen über das logische Schließen* (Investigations into Logical Deduction). In it, Gentzen introduced both natural deduction and sequent calculus, proved normalization, and proved the consistency of Peano Arithmetic (using transfinite induction up to $\varepsilon_0$).

Gentzen's work was the beginning of proof theory as a discipline. Alan Turing was developing computability theory at the same time. When Haskell Curry and William Howard later (1934 and 1969 respectively) observed that proof rules and type-formation rules are the same formal structure, the pieces fell into place: logic, computation, and type theory are aspects of the same underlying phenomenon.

This chapter ends with the observation that sets the stage for the rest of the curriculum.
