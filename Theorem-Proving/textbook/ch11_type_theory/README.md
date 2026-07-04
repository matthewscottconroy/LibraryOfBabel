# Chapter 11: Type Theory

What if propositions were types and proofs were programs? Type theory takes this identification — the Curry–Howard correspondence — literally, and it is the foundation on which Lean 4, Coq, and Agda are built.

## Overview

The chapter begins with the **untyped lambda calculus**: three constructs ($e ::= x \mid \lambda x.\, e \mid e\ e$), free vs. bound variables, and the three reduction rules — $\alpha$ (renaming bound variables), $\beta$ ($(\lambda x.\, e)\, t \to e[t/x]$, the computation step, with capture-avoiding substitution), and $\eta$ (extensionality). Key results: the **Church–Rosser theorem** (confluence, hence unique normal forms), the divergent term $\Omega$, **Church encodings** of booleans, numerals, and pairs, the **Y fixed-point combinator** for recursion, and reduction strategies (normal vs. applicative order, call-by-name/value/need). The untyped calculus computes exactly the Turing-computable functions.

The **Curry–Howard correspondence** then supplies the dictionary: proposition $\leftrightarrow$ type, proof $\leftrightarrow$ term, $P \land Q \leftrightarrow$ product, $P \lor Q \leftrightarrow$ sum, $P \to Q \leftrightarrow$ function type, $\bot \leftrightarrow$ empty type, $\forall \leftrightarrow \Pi$, $\exists \leftrightarrow \Sigma$; cut elimination *is* $\beta$-reduction, and constructive proofs extract to certified programs (CompCert), while classical LEM lacks direct computational content. The **simply typed lambda calculus** (Church 1940) disciplines the untyped system with judgments $\Gamma \vdash e : \tau$ and three typing rules (variable, abstraction, application); the **strong normalization theorem** makes every typed term terminate ($\Omega$ and Y become untypable — the price is Turing-completeness), and STLC corresponds exactly to minimal propositional logic. **Hindley–Milner type inference** recovers types automatically via constraint generation, Robinson unification with the occurs check, and let-polymorphism (Algorithm W); System F (Girard–Reynolds) and dependent types push inference past decidability.

**Dependent types** let types mention values: the **$\Pi$-type** $\prod_{x:A} B(x)$ is the dependent function type and, via Curry–Howard, the universal quantifier; the **$\Sigma$-type** $\sum_{x:A} B(x)$ is the dependent pair and the existential (witness plus evidence), with subtypes $\{x : A \mathbin{/\!/} P\, x\}$ as the special case. Each type former is specified by formation, introduction, elimination, and computation rules (`Vector α n` is the running example); Martin-Löf type theory and the Calculus of Inductive Constructions (Lean 4, Coq) add inductive types and a universe hierarchy with decidable type checking. **Homotopy type theory** closes the chapter: **identity types** $a =_A b$ with `refl` and the **J-eliminator** (path induction), paths and $n$-types, the circle $S^1$ with $\pi_1(S^1) = \mathbb{Z}$, Voevodsky's **univalence axiom** $(A \simeq B) \simeq (A = B)$ (yielding transport and function extensionality), and **higher inductive types** with path constructors — quotients, propositional truncation $\lVert A \rVert$, suspensions and spheres.

## Why It Matters

This is the theory behind the tools used throughout the book: a Lean or Coq proof *is* a lambda term, type checking *is* proof verification, and Chapter 5's strategies become tactics that elaborate proof terms. Chapter 4's intuitionistic natural deduction is revealed as STLC in disguise, Chapter 7's recursion becomes inductive types, and Chapter 13's formal verification applies all of it.

## Chapter Roadmap

1. [Lambda Calculus](01_lambda_calculus/01_untyped_lambda_calculus.md) — syntax, reduction, Church encodings, and the Y combinator; further files treat $\alpha/\beta/\eta$ with Church–Rosser and reduction strategies, and give a Haskell implementation.
2. [The Curry–Howard Correspondence](02_curry_howard_correspondence/01_propositions_as_types.md) — the propositions-as-types dictionary; a second file on proofs-as-programs (cut elimination as $\beta$-reduction, extraction), plus a Lean 4 companion.
3. [Simple Type Theory](02_simple_type_theory/01_simply_typed_lambda.md) — STLC typing rules and strong normalization; then Hindley–Milner inference, unification, and let-polymorphism.
4. [Dependent Types](03_dependent_types/01_pi_types.md) — $\Pi$-types as universal quantifiers; $\Sigma$-types as existentials and subtypes; Lean 4 and Coq companion files.
5. [Homotopy Type Theory](04_homotopy_type_theory/01_identity_types.md) — identity types and J-elimination; the univalence axiom; higher inductive types.

## Prerequisites

Natural deduction and the intuitionistic/classical distinction (Chapter 4), proof strategies (Chapter 5), and induction and recursion (Chapter 7). Prior exposure to a functional language (Haskell, ML) helps but is not required.
