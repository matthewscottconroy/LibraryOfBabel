# Chapter 9: Model Theory

The study of the gap between what axioms say and which structures satisfy them — where syntax ($\vdash$) meets semantics ($\vDash$), and where the limits of first-order axiomatization come sharply into view.

## Overview

The chapter's central question: what is the relationship between a theory (a set of sentences) and its models (the structures satisfying them)? Can axioms pin down a structure uniquely? We begin with **first-order structures**: a non-empty domain plus interpretations of every constant, function, and relation symbol, with truth $\mathcal{M} \vDash \phi$ defined by Tarski's inductive satisfaction clauses. Structures are compared by **homomorphisms**, **isomorphisms** (bijective, structure-preserving in both directions), **embeddings**, and **elementary embeddings** (preserving all first-order formulas); isomorphic structures are **elementarily equivalent** — they satisfy exactly the same sentences.

The twin pillars follow. **Soundness** — $\Gamma \vdash \phi \Rightarrow \Gamma \vDash \phi$, proved by induction on derivations — guarantees proofs never lie. **Gödel's completeness theorem** (1929) is the converse: $\Gamma \vDash \phi \Rightarrow \Gamma \vdash \phi$, proved by the **Henkin construction** (extend to a maximal consistent set, add witness constants, read off a model). Together, $\vdash$ and $\vDash$ coincide for first-order logic — a result carefully distinguished from Gödel's *incompleteness* theorems (Chapter 10). Because proofs are finite, completeness yields the **compactness theorem**: $\Gamma$ is satisfiable iff every finite subset is. Its applications are the chapter's showpieces: **non-standard models of arithmetic** (add a constant $c$ with $c \neq \bar{n}$ for every numeral; compactness delivers a model of PA with infinite elements), the inexpressibility of finiteness in first-order logic, and the De Bruijn-Erdős theorem on graph coloring.

The final section studies theories themselves. The **Löwenheim-Skolem theorems** (downward: a countable theory with an infinite model has a countable one; upward: models of every infinite cardinality exist) show first-order logic cannot control cardinality — the source of **Skolem's paradox**, a countable model of ZFC that internally believes in uncountable sets. **Complete theories** ($T \vdash \phi$ or $T \vdash \neg\phi$ for every $\phi$) include DLO, ACF, and RCF, while PA and ZFC are incomplete; **Vaught's test** derives completeness from $\kappa$-categoricity, with Morley's categoricity theorem as coda. Non-standard models culminate in Robinson's **non-standard analysis**: the hyperreals ${}^*\mathbb{R}$ with genuine infinitesimals, the standard part function $\mathrm{st}$, and the **transfer principle**.

## Why It Matters

Model theory delimits what any first-order axiomatization — hence any proof assistant working over such axioms — can determine about its intended structure: PA has models with infinite naturals, and no first-order theory distinguishes $\mathbb{N}$ from them. Soundness is precisely the property that makes a Lean 4 or Coq kernel trustworthy: a checked proof is true in every model. The chapter builds directly on the syntax and semantics of Chapter 3 and the proof systems of Chapter 4, uses Chapter 6's cardinality machinery for Löwenheim-Skolem, and sets the stage for Chapter 10, where truth in the standard model and provability in PA finally come apart.

## Chapter Roadmap

1. [Structures](01_structures/01_first_order_structures.md) — first-order structures, standard examples ($\mathbb{N}$ with $0, S, +, \times$; ordered fields; Boolean algebras), and Tarski's truth definition; the second file covers homomorphisms, isomorphisms, and elementary embeddings.
2. [Completeness](02_completeness/01_goedel_completeness.md) — Gödel's completeness theorem via the Henkin construction and its contrast with incompleteness; the second file proves soundness, yielding $\Gamma \vdash \phi \iff \Gamma \vDash \phi$.
3. [Compactness](03_compactness/01_compactness_theorem.md) — the compactness theorem (with its topological analogy), non-standard models of PA, why finiteness is not first-order expressible, graph coloring, and the Löwenheim-Skolem theorems with Skolem's paradox.
4. [Theories](04_theories/01_complete_theories.md) — complete versus incomplete theories and Vaught's categoricity test; the second file develops non-standard models in depth: $\mathbb{Z}$-blocks ordered like $\mathbb{Q}$, hyperreals and the transfer principle, and the philosophical stakes for mathematical realism.

## Prerequisites

Chapter 3 (first-order syntax and semantics) and Chapter 4 (proof systems, the meaning of $\vdash$) are essential; Chapter 6 supplies the cardinality notions — countable versus uncountable, bijections — on which compactness and Löwenheim-Skolem arguments rely.
