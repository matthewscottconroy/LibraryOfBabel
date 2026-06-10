# Chapter 21: Lean 4 and Mathlib — Formal Proof at Scale

In 2005, Georges Gonthier completed a machine-verified proof of the four-color theorem. Not a sketch. Not a paper proof that could, in principle, be formalized. A proof in which every logical step was checked by a computer program — 100,000 lines of Coq code. His reaction surprised people: he said the formalization revealed errors in the informal proof that no human reviewer had caught in thirty years.

The lesson was not that humans are fallible. We knew that. The lesson was that formalization finds errors that informal review cannot. Informal review is social: it relies on shared assumptions, shared vocabulary, shared intuitions about what "obviously" follows from what. Formalization is antisocial in the best sense. The proof assistant does not share your intuitions. It does not know that the step is "routine." It requires you to state every premise, justify every inference, and account for every case.

Gonthier's proof was in Coq. Today, in 2025, the state of the art has moved. Lean 4 and Mathlib are the dominant tools for large-scale mathematical formalization, and they have achieved something Coq never managed: a library — Mathlib — that has become a genuine research resource for mathematicians working at the frontier.

## Why Lean 4?

Lean 4 is simultaneously a proof assistant, a functional programming language, and a platform for metaprogramming. This triple identity is not accidental. Leonardo de Moura, Lean's creator, understood that the historical gap between proof assistants (correct but hard to use) and programming languages (easy to use but unverified) was closing, and he designed Lean 4 to straddle it.

The foundational bet is the Curry-Howard correspondence: propositions are types, proofs are programs. A proof of $P \Rightarrow Q$ is a function from $P$ to $Q$. A proof of $\forall n : \mathbb{N}, P(n)$ is a function from natural numbers to proofs of $P$. The type checker that checks your programs is the same system that checks your proofs. There is no separate proof checker — the type checker is the proof checker.

This means Lean 4 can be both a programming language and a proof assistant without cheating. When you define a sorting algorithm in Lean 4, you can also prove it's correct, using the same system, in the same file. The program is the proof; the proof is the program.

## Mathlib: Mathematics at Machine Scale

Mathlib4 is the largest formalized mathematics library in existence. As of 2025, it contains over 150,000 theorems — a number that continues to grow. Its coverage includes:

- **Number theory:** from basic divisibility to the prime number theorem to perfectoid spaces
- **Algebra:** from monoids and groups to commutative algebra and algebraic geometry
- **Analysis:** real and complex analysis, measure theory, functional analysis
- **Topology:** point-set topology, manifolds, fiber bundles
- **Category theory:** categories, functors, natural transformations, adjunctions, limits, toposes
- **Combinatorics:** graph theory, combinatorial game theory, discrete probability

For our purposes — working in and around HoTT — Mathlib provides the classical mathematical background. Group theory for the fundamental group. Topological spaces as the classical setting. Category theory for the homotopy hypothesis. Homological algebra for computing cohomology. All of this, machine-checked, is available in Mathlib.

## What Lean 4 Cannot Do

It is equally important to understand Lean 4's limitations for HoTT.

Lean 4's foundational treatment of equality places it in `Prop`, the proof-irrelevant universe. Two proofs of the same proposition are definitionally equal. This is the K axiom for propositions, and it means that every identity type in Lean 4 satisfies UIP: there is at most one proof of $a = b$ up to definitional equality.

This is exactly what HoTT rejects. In HoTT, the identity type $a = b$ is a type whose elements are paths, and there can be genuinely different paths between the same endpoints. The circle $S^1$ is defined precisely by having a non-trivial loop $\text{loop} : \text{base} = \text{base}$ that is not equal to $\text{refl}$. This is impossible to state in Lean 4's `Prop`-based equality.

Lean 4 is for classical mathematics. For HoTT-specific content — HITs, univalence as a computation rule, synthetic homotopy theory — Chapter 22 on Cubical Agda is the tool.

## Chapter Roadmap

**Section 1: Lean 4 Basics** — Installation, project setup, core syntax, the universe hierarchy, dependent types, and what `#check`, `#eval`, and `#reduce` tell you. This section gives you the vocabulary to read and write Lean 4 code.

**Section 2: Tactics and Proofs** — The tactic mode, the proof state, the twenty core tactics, and the automation tools (`simp`, `ring`, `linarith`, `omega`). This section gives you the tools to prove things.

**Section 3: Mathlib** — The library's structure, naming conventions, key areas, and how to find theorems. This section gives you access to 150,000 proven results.

**Section 4: Formalization in Practice** — A worked formalization example from start to finish: how to take a mathematical theorem, identify what Mathlib already has, fill the gaps, and arrive at a machine-checked proof. This section gives you the workflow.

After these four sections, the exercises, thought experiments, and applications complete the chapter. The goal is not that you memorize every tactic, but that you can sit down, open a Lean 4 file, and make progress on a real formalization problem.
