# References: Lean 4 and Mathlib

## Primary References

**Avigad, Jeremy, et al. *Theorem Proving in Lean 4*.**
The official interactive tutorial for Lean 4, freely available at `leanprover.github.io/theorem_proving_in_lean4`. Covers the type theory foundations, term mode proofs, tactic mode, and the elaboration system. The authoritative starting point for anyone learning Lean 4.

**Avigad, Jeremy, et al. *Mathematics in Lean*.**
A sequel tutorial focused specifically on mathematics, available at `leanprover-community.github.io/mathematics_in_lean`. Organized around mathematical areas (number theory, group theory, analysis, topology), with exercises and worked examples. Designed for mathematicians rather than logicians.

**The Lean 4 Documentation.** `leanprover.github.io/lean4/doc`. The official language reference, covering syntax, the elaboration system, the macro and metaprogramming infrastructure, and the Lake build system.

---

## Mathlib

**The Mathlib4 Library.** `github.com/leanprover-community/mathlib4`. The source code and documentation for the entire library. Every theorem statement, proof, and docstring is available here.

**Mathlib4 Documentation.** `leanprover-community.github.io/mathlib4_docs`. The rendered API documentation, searchable by name. Essential for finding theorems.

**Loogle.** `loogle.lean-lang.org`. A web-based search engine for Mathlib theorems, accepting theorem-shape queries (patterns in Lean 4 syntax). Finds theorems by conclusion structure, not just by name.

**Moogle.** `moogle.ai`. A natural-language search for Mathlib theorems using a neural language model. Useful for exploration when you don't know the Lean 4 terminology.

**The Lean Zulip.** `leanprover.zulipchat.com`. The primary community forum. The `#mathlib4`, `#new members`, and `#general` channels are active and responsive. If you can't find a theorem, ask here.

---

## Key Papers and Results

**Gonthier, Georges. "Formal Proof — the Four-Color Theorem." *Notices of the AMS*, 2008.**
The classic paper describing the formalization of the four-color theorem in Coq. Establishes the model: formalization as mathematical examination, revealing errors that informal review cannot. Required reading for anyone interested in why formalization matters.

**Hales, Thomas, et al. "A Formal Proof of the Kepler Conjecture." *Forum of Mathematics, Pi*, 2017.**
The journal publication describing the Flyspeck project's machine-verified proof of the Kepler conjecture. Demonstrates that formalization can achieve what informal review cannot — certainty — in the presence of large-scale computer calculations.

**Buzzard, Kevin, Commelin, Johan, and Massot, Patrick. "Formalising Perfectoid Spaces." *Proceedings of CPP*, 2020.**
The paper announcing the formalization of perfectoid spaces in Lean 3. Shows that research-level algebraic geometry is within reach of proof assistants.

**Gonthier, Georges, et al. "A Machine-Checked Proof of the Odd Order Theorem." *Proceedings of ITP*, 2013.**
The formalization of the Feit-Thompson theorem (every finite group of odd order is solvable) — 150,000 lines of Coq code, fifteen years of work. The most complex formalization of classical group theory to date.

---

## Foundational Theory

**Coquand, Thierry, and Huet, Gérard. "The Calculus of Constructions." *Information and Computation*, 1988.**
The foundational paper for the type theory underlying Lean 4 (and Coq). Defines the Calculus of Constructions, the dependent type theory that unifies logic and computation.

**Barendregt, Henk. "Lambda Calculi with Types." In *Handbook of Logic in Computer Science*, 1992.**
The authoritative reference for typed lambda calculi, covering the Curry-Howard correspondence, System F, and the Calculus of Constructions. More technical than the tutorials but gives the full mathematical picture.

**Martin-Löf, Per. "Intuitionistic Type Theory." *Bibliotheca di Matematica*, 1984.**
The original presentation of dependent type theory, the foundation underlying all modern proof assistants. Philosophically rich and mathematically precise.

---

## Tools and Infrastructure

**The elan Lean Version Manager.** `github.com/leanprover/elan`. Install and manage Lean 4 versions.

**The Lake Build System.** `github.com/leanprover/lake`. Lean 4's package manager and build system. Documentation at `leanprover.github.io/lake`.

**VS Code Lean 4 Extension.** Available in the VS Code marketplace as `lean4`. Provides real-time infoview, error checking, and the interactive proof state display.

**leanblueprint.** `github.com/PatrickMassot/leanblueprint`. Patrick Massot's tool for connecting LaTeX proof blueprints to Lean 4 formalizations. Used by large-scale formalization projects including the Fermat's Last Theorem project.

---

## For Further Study

**de Moura, Leonardo, and Ullrich, Sebastian. "The Lean 4 Theorem Prover and Programming Language." *Proceedings of CADE*, 2021.**
The paper describing Lean 4's design and implementation. Covers the elaboration algorithm, the macro system, and the metacircular architecture.

**The Fermat's Last Theorem Formalization.** `github.com/ImperialCollegeLondon/FLT`. Kevin Buzzard's ongoing project. The leanblueprint at `imperialcollegelondon.github.io/FLT` shows the dependency graph of the proof and the current state of formalization.

**Buzzard, Kevin. *Formalising Mathematics* course materials.** `github.com/ImperialCollegeLondon/formalising-mathematics-2024`. The annually updated Imperial course on Lean 4 formalization, aimed at research mathematicians. Exercises are organized by mathematical area.
