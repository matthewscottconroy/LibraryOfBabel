# References and Primary Sources

## Primary Documentation and Tools

**Lean 4 itself** — the theorem prover and programming language. The canonical source is:
- Repository: [github.com/leanprover/lean4](https://github.com/leanprover/lean4)
- Official documentation site: [leanprover.github.io](https://leanprover.github.io)
- Release notes and language reference: [leanprover.github.io/lean4/doc](https://leanprover.github.io/lean4/doc/)

**Mathlib4** — the main mathematical library, with hundreds of thousands of theorems spanning algebra, analysis, topology, number theory, category theory, and more.
- Repository: [github.com/leanprover-community/mathlib4](https://github.com/leanprover-community/mathlib4)
- Interactive API docs: [leanprover-community.github.io/mathlib4_docs](https://leanprover-community.github.io/mathlib4_docs/)
- Community wiki and guides: [leanprover-community.github.io](https://leanprover-community.github.io/)

**Lean 4 VS Code Extension** — the standard development environment. Provides real-time type-checking, goal display, `#check`/`#eval`, and tactic state. Install `leanprover.lean4` from the VS Code marketplace.

**Loogle** — a semantic search tool for Lean 4 / Mathlib by type signature. Available at [loogle.lean-lang.org](https://loogle.lean-lang.org/). Query by pattern, e.g., `List.length _ = List.length _`.

**Moogle** — a natural-language search tool for Mathlib theorems, powered by language models. Available at [www.moogle.ai](https://www.moogle.ai/). Useful when you know the mathematical content but not the Lean name.

---

## Foundational Papers

**Leonardo de Moura and Sebastian Ullrich.** "The Lean 4 Theorem Prover and Programming Language." *Proceedings of the 28th International Conference on Automated Deduction (CADE-28)*, LNAI 12699, pp. 625–635. Springer, 2021.
The primary reference for Lean 4. Describes the language design, the elaboration algorithm, the macro/metaprogramming system, and the rationale for the divergence from Lean 3. This is the paper to cite when referring to Lean 4 as a system.

**Jeremy Avigad, Leonardo de Moura, Soonho Kong, and Sebastian Ullrich.** "Theorem Proving in Lean 4." Online textbook, continuously updated. [leanprover.github.io/theorem_proving_in_lean4](https://leanprover.github.io/theorem_proving_in_lean4/)
The official tutorial/reference for Lean 4 proof writing. Covers dependent types, tactics, the `Prop`/`Type` distinction, and the standard library. The most comprehensive introduction available.

**The Mathlib Community.** "The Lean Mathematical Library." *Proceedings of the 9th ACM SIGPLAN International Conference on Certified Programs and Proofs (CPP 2020)*, pp. 367–381. ACM, 2020.
Describes the design principles behind Mathlib — the typeclass system, the naming conventions, the structure of the library, and the social infrastructure of the Mathlib community. Essential background for anyone contributing to or using Mathlib.

**Thierry Coquand and Gérard Huet.** "The Calculus of Constructions." *Information and Computation* 76(2–3):95–120, 1988.
The foundational paper for the type theory underlying both Lean 4 and Coq. The Calculus of Constructions (CoC) is the core type theory; Lean 4's CIC extends it with inductive types and universes. Understanding CoC illuminates why `Prop` works the way it does.

**Peter Dybjer.** "Inductive Families." *Formal Aspects of Computing* 6(4):440–465, 1994.
The theoretical basis for Lean 4's `inductive` declarations. Dybjer's inductive families are precisely what Lean 4 calls dependent inductive types — the mechanism by which `Fin n`, `Vec α n`, and indexed type families are defined.

**Thierry Coquand.** "Metamathematical Investigations of a Calculus of Constructions." In P. Odifreddi, ed., *Logic and Computer Science*, pp. 91–122. Academic Press, 1990.
A technical treatment of the meta-theory of CoC: normalization, consistency, the impredicative Prop universe. Useful for understanding why Lean's `Prop` is sound and what it means for proof irrelevance to hold.

**Mario Carneiro.** "The Type Theory of Lean." Master's thesis, Carnegie Mellon University, 2019.
Describes the precise type theory of Lean (Lean 3, but closely related to Lean 4): the kernel, the definitional equality algorithm, and the logical foundations. This is the reference for Lean's meta-theory at a level of rigor beyond the official documentation.

---

## Textbooks and Learning Resources

**Jeremy Avigad.** *Mathematics in Lean.* Free online textbook, available at [leanprover-community.github.io/mathematics_in_lean](https://leanprover-community.github.io/mathematics_in_lean/).
The primary learning resource for mathematical formalization in Lean 4. Covers number theory, algebra, analysis, and topology with extensive Lean 4 code. Structured as a tutorial with exercises. Suitable for anyone with a mathematics background (undergraduate level and above).

**Jeremy Avigad, Leonardo de Moura, Soonho Kong, Sebastian Ullrich.** *Theorem Proving in Lean 4.* Available at [leanprover.github.io/theorem_proving_in_lean4](https://leanprover.github.io/theorem_proving_in_lean4/).
More type-theoretically oriented than *Mathematics in Lean*. Covers the dependent type system from first principles, inductive definitions, the tactic system, and the meta-theoretic foundations. Best for readers coming from a type theory or programming languages background.

**Kevin Buzzard.** *Formalising Mathematics.* Imperial College London course notes and Lean 4 files, available at [github.com/ImperialCollegeLondon/formalising-mathematics-2024](https://github.com/ImperialCollegeLondon/formalising-mathematics-2024).
Buzzard's annual graduate course at Imperial College, progressively formalizing serious algebraic geometry, number theory, and algebra in Lean 4. Available as a repository of `.lean` files with commentary. Invaluable for seeing how a research mathematician works with Lean on hard content.

**Anne Baanen, Alexander Bentkamp, Jasmin Blanchette, Johannes Hölzl, Jannis Limperg.** *The Hitchhiker's Guide to Logical Verification.* 2023. Available at [github.com/blanchette/logical_verification_2023](https://github.com/blanchette/logical_verification_2023).
A graduate textbook on formal verification using Lean 4, covering program verification, model checking, and theorem proving. More CS-oriented than Avigad's books; good for anyone interested in using Lean for software verification rather than pure mathematics.

**Daniel Selsam and Leonardo de Moura.** *Lean 4 Programming Language.* Internal reference documentation. Available through the Lean 4 repository.
The terse programmer-oriented reference. Not the best first introduction, but the authoritative source on language features: syntax, elaboration, `do`-notation, macros, and the runtime system.

---

## Key Libraries and Online Resources

**Mathlib4** — [github.com/leanprover-community/mathlib4](https://github.com/leanprover-community/mathlib4)
The core library. Everything from basic algebra to perfectoid spaces. When working on mathematical formalization, this is the primary resource. The `Mathlib.Algebra`, `Mathlib.Topology`, `Mathlib.Analysis`, and `Mathlib.CategoryTheory` namespaces are especially relevant to HoTT-adjacent content.

**Lean 4 Community Zulip** — [leanprover.zulipchat.com](https://leanprover.zulipchat.com/)
The primary community forum. The `#mathlib4`, `#new members`, and `#Is there code for X?` streams are particularly useful. Fastest place to get answers on Lean 4 syntax, Mathlib naming, and formalization strategies.

**Reservoir** — [reservoir.lean-lang.org](https://reservoir.lean-lang.org/)
The Lean 4 package registry. Lists community libraries beyond Mathlib, including verification tools, category theory libraries, and domain-specific formalizations.

**Leanblueprint** — [github.com/PatrickMassot/leanblueprint](https://github.com/PatrickMassot/leanblueprint)
A tool (by Patrick Massot) for creating dependency graphs of mathematical formalizations, linking LaTeX blueprints to Lean 4 proofs. Used in several large-scale formalization projects, including the Fermat's Last Theorem project.

**Lean Formalization of the Liquid Tensor Experiment** — [github.com/leanprover-community/lean4-project](https://github.com/leanprover-community/mathlib4) (via Mathlib)
The formalization of Clausen-Scholze's liquid tensor experiment, led by Johan Commelin. A landmark achievement in mathematical formalization; the first major result in "condensed mathematics" to be verified. Good to study for how large-scale formalization projects are organized.

---

## Historical Context

Lean was originally developed by Leonardo de Moura at Microsoft Research, with the first public version (Lean 1) appearing around 2013. Lean 2 and Lean 3 followed, each refining the type theory, the elaboration algorithm, and the user interface. Lean 3 was the version that attracted a critical mass of mathematicians, culminating in a formalization of the perfectoid spaces definition in 2019 (by Kevin Buzzard, Johan Commelin, and Patrick Massot) — a result that demonstrated that cutting-edge research mathematics was within reach of a proof assistant. Lean 3's Mathlib library grew rapidly throughout 2018–2021, but the system had fundamental limitations: its metaprogramming was cumbersome, its compilation was slow, and the underlying runtime was not designed for large developments.

Lean 4 represents a full redesign, announced by de Moura and Ullrich in 2021. The key architectural changes were: (1) Lean 4 is itself written in Lean 4 (metacircular), making the metaprogramming system the same language as the object language; (2) a new incremental compilation system (the `lake` build tool) makes large developments tractable; (3) the `do`-notation and effect system make Lean 4 usable as a general-purpose functional programming language, not merely a proof assistant. The migration of Mathlib from Lean 3 to Lean 4 was completed in 2023, a major community effort coordinated through Zulip. As of 2024–2025, Lean 4 / Mathlib4 is the world's largest formalized mathematics library, and active projects include formalizations of the Langlands program, class field theory, and a Lean 4 implementation of the Fermat's Last Theorem proof (led by Kevin Buzzard). The gap between what can be formalized in Lean 4 and what research mathematicians actually do is closing faster than anyone predicted in 2015.
