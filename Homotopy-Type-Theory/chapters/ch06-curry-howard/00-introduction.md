# Chapter 6: The Curry-Howard Correspondence

## The Discovery

In 1934, Haskell Curry noticed something remarkable: the types of the basic combinators in combinatory logic correspond to axioms of implicational logic. The identity combinator $I = \lambda x. x$ has type $A \to A$, corresponding to the axiom "A implies A." The constant combinator $K = \lambda x. \lambda y. x$ has type $A \to B \to A$, corresponding to the axiom "if A, then B implies A."

This was not a coincidence. Curry noticed a systematic correspondence between the structure of typed terms and the structure of logical derivations.

In 1969, William Howard extended this observation to full intuitionistic natural deduction and the simply typed lambda calculus. His manuscript, widely circulated but unpublished until 1980, showed that every natural deduction proof corresponds to a typed lambda term, and that proof normalization corresponds to beta reduction.

Meanwhile, Per Martin-Löf extended the correspondence to dependent types and predicate logic in the 1970s, creating the type-theoretic foundation that underlies modern proof assistants.

The result is one of the deepest and most surprising discoveries in foundations: **the structure of mathematical proofs and the structure of computer programs are the same thing.**

## What the Correspondence Says

The Curry-Howard correspondence is a *precise isomorphism* — not a loose analogy — between:

- **Propositions** and **types**: every proposition in intuitionistic logic corresponds to a type in a type-theoretic system, and vice versa.
- **Proofs** and **terms**: every proof of a proposition corresponds to a term (program) of the corresponding type.
- **Proof normalization** and **computation**: reducing a proof to normal form (removing detours) corresponds to evaluating a program (reducing redexes).
- **Proof checking** and **type checking**: verifying that a derivation is a valid proof corresponds to verifying that a term has the claimed type.

This is not just a dictionary. The structural rules are *identical*: the introduction rules for logical connectives are the same as the type-formation rules for type constructors, and the elimination rules are the same as the eliminators.

## Why This Matters

**For logic:** The Curry-Howard correspondence gives a computational interpretation of proofs. Every intuitionistic proof carries algorithmic content: it's not just a certificate of truth but an executable program. Proof normalization is computation.

**For computer science:** The correspondence gives a logical interpretation of types and programs. A type is not just a specification of what values a variable can hold — it's a proposition that the program proves. Typing a program is proving it correct.

**For proof assistants:** The correspondence is the reason proof assistants exist. Lean 4, Coq, Agda, and Isabelle are all implementations of the Curry-Howard correspondence: types are propositions, terms are proofs, the type checker verifies proofs mechanically.

**For HoTT:** The correspondence extends to identity types, where proofs of equality are paths. The whole homotopy-theoretic interpretation of types builds on this extension.

## The Roadmap

This chapter develops the Curry-Howard correspondence carefully.

**Section 1: The Central Dictionary.** We lay out the precise correspondence between logical connectives and type constructors, between proof rules and typing rules, and between proof normalization and computation.

**Section 2: Simply Typed Lambda Calculus.** We introduce the formal system on the programming side of the correspondence: STLC, with its typing rules, reduction rules, and normal forms.

**Section 3: Normalization and Consistency.** We prove strong normalization for STLC (using the reducibility method), and derive the consistency of intuitionistic propositional logic as a corollary.

**Section 4: Extending to Dependent Types.** We preview how the correspondence extends from propositional to predicate logic via dependent types ($\Pi$ and $\Sigma$ types), setting up the fuller treatment in Chapter 8.

**Section 5: Proof Assistants.** We see the correspondence in action in Lean 4 and Agda, and examine what it means to write a proof as a program.

## Prerequisites

This chapter requires:
- Chapter 4 (Proof Theory): natural deduction rules, β-reduction, normalization.
- Chapter 5 (Intuitionistic Logic): BHK interpretation, IPC, why classical axioms don't have computational content.
- Basic familiarity with the λ-calculus is helpful but not required (we develop it here).
