# Chapter 10: Computability and Incompleteness

The two great limitative results of the 1930s — Turing's undecidability of the halting problem and Gödel's incompleteness theorems — mark the outer boundary of what algorithms can compute and what formal systems can prove, and turn out to be two faces of a single phenomenon.

## Overview

The chapter opens with the question behind Hilbert's *Entscheidungsproblem*: what can a purely mechanical procedure compute? A **Turing machine** is defined as a 7-tuple $(Q, \Sigma, \Gamma, \delta, q_0, q_{\text{accept}}, q_{\text{reject}})$ operating on an infinite tape; from it we get partial and total **computable functions** and the narrower class of **primitive recursive functions** (built from base functions by composition and primitive recursion — the Ackermann function separates the two). The **Church–Turing thesis** asserts that every effectively computable function is Turing-computable. It is a thesis, not a theorem, but the convergence of Turing machines, Church's lambda calculus, and the Gödel–Kleene recursive functions on one class of functions is its decisive evidence.

Next, decidability. A set is **decidable** (recursive) if a machine always halts with the correct verdict, and **semi-decidable** (recursively enumerable, r.e.) if a machine accepts exactly its members but may diverge; $L$ is decidable iff both $L$ and $\bar{L}$ are semi-decidable. The **halting problem** $\mathrm{HALT} = \{\langle P, x \rangle : P \text{ halts on } x\}$ is semi-decidable but undecidable — proved by Cantor-style diagonalization on a self-applied "diagonalizer" program. **Many-one reductions** ($A \leq_m B$) propagate undecidability outward (to emptiness, regularity), culminating in **Rice's theorem**: every non-trivial property of program behavior is undecidable. The **arithmetical hierarchy** ($\Sigma^0_1, \Pi^0_1, \Sigma^0_2, \ldots$) then stratifies the undecidable, while Presburger arithmetic and real closed fields (Tarski) show that decidable theories do exist.

Finally, incompleteness. **Gödel numbering** encodes formulas and proofs as numbers, making provability an arithmetic predicate $\mathrm{Provable}(n)$; the **diagonal lemma** yields for any $\phi(x)$ a sentence $\psi$ with $T \vdash \psi \leftrightarrow \phi(\ulcorner\psi\urcorner)$, and applied to $\neg\mathrm{Provable}(x)$ it produces the Gödel sentence $G_T$ ("I am not provable"). **Gödel's first incompleteness theorem** (1931): any consistent, effectively axiomatizable theory containing basic arithmetic satisfies $T \nvdash G_T$ and $T \nvdash \neg G_T$ (Rosser removed the original $\omega$-consistency assumption), yet $G_T$ is true in $\mathbb{N}$. The **second incompleteness theorem** — $T \nvdash \mathrm{Con}(T)$ — ends Hilbert's finitary consistency program. The chapter closes by sorting what the theorems show (truth outruns provability) from what they do not (the contested Penrose–Lucas argument; no claim that mathematics is unreliable).

## Why It Matters

This chapter fixes the boundary inside which the whole book operates. Completeness (Chapter 4) says $\vdash$ matches $\vDash$; incompleteness says the theorems of arithmetic are only c.e., never decidable — so proof search can never be a decision procedure, and Rice's theorem explains why the verification tools of Chapter 13 must approximate. Provability logic in Chapter 12 (GL) modalizes this chapter's $\mathrm{Provable}$ predicate, and proof assistants exist precisely to work rigorously within these limits.

## Chapter Roadmap

1. [Computability](01_computability/01_effective_computability.md) — effective computability and primitive recursion; further files give the formal Turing machine definition and the Church–Turing thesis with its evidence.
2. [Decidability](02_decidability/01_decidable_problems.md) — decidable vs. semi-decidable problems; then the halting problem via diagonalization plus Rice's theorem, and many-one reductions with the arithmetical hierarchy.
3. [Incompleteness](03_incompleteness/01_first_incompleteness_theorem.md) — Gödel numbering, the diagonal lemma, both incompleteness theorems; a companion file on philosophical implications.
4. [Computability in Python](04_computability_in_python/01_turing_simulator.py) — an executable Turing machine simulator, plus demonstrations of the halting paradox and undecidability.

## Prerequisites

First-order logic and its proof systems, including the completeness theorem (Chapters 3–4); induction and recursion (Chapter 7). Cantor's diagonal argument (Chapter 6) and Peano arithmetic (Chapter 8) are assumed for the incompleteness material.
