# Chapter 7: Simply Typed Lambda Calculus and System F

## The Problem of Typing

Church's lambda calculus, invented in the 1930s as a foundation for mathematics, is deceptively simple. Its three constructs — variables, function formation ($\lambda x. t$), and function application ($t\, s$) — can encode everything: natural numbers, booleans, lists, pairs, and even the Y combinator, which implements recursion. The untyped lambda calculus is Turing-complete.

But this power comes at a cost. The untyped lambda calculus has no notion of "type error": you can apply any term to any other term. The term $\mathsf{true}\,\mathsf{true}$ (applying a boolean to a boolean) is syntactically valid. The term $(\lambda x. x\, x)(\lambda x. x\, x)$ is valid and doesn't terminate — it's the prototypical infinite loop.

Worse, from the foundational perspective: the untyped lambda calculus cannot serve as a logic. Under Curry-Howard, terms would be proofs and types would be propositions. But without types, there's no distinction between proofs and non-proofs. And the existence of non-terminating terms would mean "proofs" could run forever — the type-theoretic equivalent of inconsistency.

*Types solve these problems.* By adding a type discipline to the lambda calculus, we rule out self-application, guarantee termination, and make the Curry-Howard correspondence into a genuine logic-programming isomorphism.

## Two Typed Systems

This chapter develops two typed lambda calculi:

**Simply Typed Lambda Calculus (STLC):** The foundational system. Types are built from base types and function types. Every well-typed term terminates. Corresponds to intuitionistic propositional logic. Cannot express polymorphic functions (functions that work for any type) or self-referential types.

**System F:** Extends STLC with *universal quantification over types* — polymorphism. A function can have type $\forall \alpha. \alpha \to \alpha$, meaning "for any type $\alpha$, take an $\alpha$ and return an $\alpha$." This is the polymorphic identity function. System F is much more expressive: it encodes all basic data types (booleans, natural numbers, lists) as lambda terms. Corresponds to second-order intuitionistic propositional logic.

Both are important:
- STLC is the conceptually clearest, the one where Curry-Howard is most transparent.
- System F is the practical foundation for functional programming languages (Haskell, ML) and an essential stepping stone to dependent types.

## What Comes After

System F is a precursor to dependent types (Chapter 8). The key difference:
- In System F, types can depend on *type variables* (universally quantified types).
- In dependent type theory, types can depend on *term variables* (values).

This single extension — allowing types to depend on values — unlocks the full expressiveness of formal mathematics. We build toward it in this chapter.

## The Lambda Calculus as a Notation

One important preliminary: the lambda calculus is not just a programming language. It's a *notation* for functions and function application, and it's used throughout mathematics and logic. When a mathematician writes "the function $x \mapsto x^2$," they're writing a lambda abstraction: $\lambda x. x^2$.

In type theory, this notation becomes formal: $\lambda x : A. t$ is a term with type $A \to B$ (where $t : B$ under the assumption $x : A$). The Curry-Howard correspondence turns the notation for proofs and the notation for programs into the same formal system.

## Chapter Organization

We begin with the untyped lambda calculus (briefly) to see what types rule out. Then we develop STLC fully: syntax, typing rules, computation, strong normalization. Then System F: polymorphism, Church encodings, the Girard-Reynolds parametricity theorem. We close with System F$_\omega$ (type operators), which is the foundation of Haskell's type system and Coq's Calculus of Constructions.
