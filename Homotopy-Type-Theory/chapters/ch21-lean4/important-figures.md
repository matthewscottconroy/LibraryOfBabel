# Important Figures

## Leonardo de Moura (1971–present)
*Principal designer of Lean and co-creator of the Z3 SMT solver; the central architect of modern interactive theorem proving.*

Leonardo de Moura received his PhD from the Pontifical Catholic University of Rio de Janeiro and subsequently joined SRI International and then Microsoft Research. At Microsoft Research he led the development of Z3, which became (and remains) the most widely used satisfiability modulo theories (SMT) solver in both academia and industry — used as a backend by program verification tools, compilers, and security analysis frameworks worldwide. Z3's design reflected de Moura's commitment to combining theoretical rigor with engineering pragmatism: a tool that should be fast, predictable, and correct in equal measure.

Lean emerged from de Moura's conviction that interactive proof assistants and automated solvers need not be fundamentally separate. Lean 1, Lean 2, and Lean 3 explored increasingly sophisticated elaboration algorithms — the system by which user-written terms and tactics are resolved to fully explicit proof terms that the kernel can check. The elaboration algorithm in Lean 3 was capable enough to support Mathlib's growth to tens of thousands of theorems. But de Moura recognized that the system was hitting fundamental limits, and Lean 4 was designed from scratch. The key innovation of Lean 4 is that the language is metacircular — tactics, macros, and the proof assistant infrastructure are all written in Lean 4 itself, using the same dependent type theory that users write proofs in. This collapses the historically sharp boundary between "user" and "system" and makes Lean 4 uniquely extensible.

De Moura's influence on the chapter's content is pervasive: the `Prop`/`Type` distinction, the universe polymorphism system, the typeclass elaboration, the `rfl`/`exact`/`apply` tactic pipeline, and the definitional equality algorithm are all directly traceable to his design decisions. His insight that a theorem prover should also be a full-featured programming language — capable of serving as a platform for machine learning research in theorem proving — shapes the direction of the field.

---

## Sebastian Ullrich (1991–present)
*Lead implementer of Lean 4; architect of its compiler, build system, and metaprogramming infrastructure.*

Sebastian Ullrich completed his PhD at KIT (Karlsruhe Institute of Technology) under the supervision of Peter Thiemann, working on programming language theory and verification. He joined de Moura at Microsoft Research as a research engineer and became the principal implementer of Lean 4 — responsible for the compiler backend (LLVM-based native code generation), the `lake` build system, the incremental compilation infrastructure, and the macro/`syntax` system that makes Lean 4's extensible notation work.

Ullrich's most technically significant contribution to Lean 4 is the elaboration system for macros and `syntax` declarations. In Lean 4, new notations, tactics, and DSLs are introduced by `syntax` and `macro` declarations that are themselves Lean 4 code processed during elaboration. This is a genuine macro-by-reflection system: the object language and the meta-language are the same. Ullrich's design ensures that macro expansion is hygienic (no variable capture), compositional (macros can call other macros), and extensible without modification to the core kernel. The tactic language (`intro`, `exact`, `apply`, `simp`, `ring`, `linarith`) is entirely built from these macros and elaboration hooks.

For users of this chapter, Ullrich's work underlies every interaction with the VS Code Lean 4 extension: the real-time elaboration that shows you the goal state as you type, the incremental checking that reruns only the changed portions of a file, and the precise error messages that report unification failures. The `#check`, `#eval`, and `#print` commands that learners use constantly are part of Ullrich's elaboration infrastructure.

---

## Jeremy Avigad (1968–present)
*Mathematical logician; leader of the Mathlib community effort; author of the primary Lean 4 learning resources.*

Jeremy Avigad is a professor of philosophy and mathematical sciences at Carnegie Mellon University, with a research focus on proof theory, the history and philosophy of mathematics, and formal verification. His academic background spans mathematical logic (proof mining, reverse mathematics, proof complexity) and the practical mathematics of analysis and number theory. He is a longstanding contributor to both Isabelle/HOL and Lean, and is the primary author of *Theorem Proving in Lean 4* and *Mathematics in Lean* — the two resources that most learners of Lean 4 encounter first.

Avigad's specific contribution to the topics in this chapter is the pedagogical architecture: the way the `Prop`/`Type` distinction is explained to mathematicians, the way tactics are sequenced from `intro`/`exact`/`apply` to automation (`simp`, `ring`, `linarith`, `omega`), and the way formalization is connected to classical mathematical practice rather than type-theoretic foundations. His *Mathematics in Lean* does not assume knowledge of type theory — it assumes you know mathematics and teaches you to write it in Lean. This is a deliberate pedagogical choice that has brought a large number of research mathematicians into the Lean ecosystem.

Within Mathlib, Avigad has contributed to the formalization of number theory (analytic and algebraic), the ergodic theory library, and the library of results on combinatorics and probability. He is an organizer of the Lean Forward project (connecting the Lean community to mathematicians) and has been influential in the community's norms around documentation, naming, and the balance between automation and explicit proof structure.

---

## Patrick Massot (present)
*Differential geometer and Lean formalization pioneer; author of key Mathlib tools; perfectoid spaces co-formalizer.*

Patrick Massot is a professor of mathematics at Université Paris-Saclay (Orsay), specializing in contact topology and symplectic geometry. He became involved in Lean formalization after observing the growth of Mathlib's analysis library, and he is now one of the most active Mathlib contributors outside of the core team. His most widely known formalization achievement (joint with Kevin Buzzard and Johan Commelin) is the definition of *perfectoid spaces* in Lean 3 — a result announced in 2019 and widely reported as the first piece of research-level algebraic geometry to be formalized in a proof assistant.

For this chapter, Massot's most directly relevant contribution is the `leanblueprint` tool: a system for writing a LaTeX "blueprint" of a mathematical argument (a structured proof outline with dependency information) and linking each node of the blueprint to a Lean 4 formalization. This tool is now used by several large formalization projects, including Kevin Buzzard's Fermat's Last Theorem project. It operationalizes a workflow — "write the math first, then formalize each piece" — that is essential for attacking substantial mathematical results.

Massot has also contributed extensively to Mathlib's topology library (topological groups, uniform spaces, completions) and to the analysis library (metric spaces, real analysis, measure theory). His blog posts and course materials (in French and English) on Lean formalization are widely read in the community. He is a consistent voice for maintaining the library's mathematical quality alongside its technical growth.

---

## Kevin Buzzard (1969–present)
*Number theorist and algebraic geometer; leader of the Lean formalization of Fermat's Last Theorem; evangelizer of Lean in pure mathematics.*

Kevin Buzzard is a professor of pure mathematics at Imperial College London, specializing in the Langlands program, automorphic forms, and the arithmetic of elliptic curves. His mathematical work is at the frontier of algebraic number theory — exactly the kind of content that formalization was long thought to be decades away from touching. Around 2017, Buzzard began working in Lean and rapidly concluded that its capabilities were being underestimated by the pure mathematics community. He began using Lean in his teaching, created the annual "Formalising Mathematics" course at Imperial (whose materials are freely available and widely used), and launched the ambitious project of formalizing the proof of Fermat's Last Theorem — not as a distant goal, but as a structured multi-year project with a Leanblueprint dependency graph and a community of contributors.

The Fermat's Last Theorem project (still ongoing as of 2025) has already produced formalizations of the theory of modular forms, the theory of Galois representations, Serre's `ε`-conjecture result, and substantial parts of the Taylor-Wiles method. Buzzard's specific contributions to Lean/Mathlib include work on p-adic numbers, group cohomology, and the algebraic geometry needed for the Langlands program. His annual Imperial course has introduced dozens of research mathematicians to Lean 4, producing a generation of formalization-literate number theorists.

From a pedagogical standpoint, Buzzard represents the model user this chapter is aimed at: a mathematician who knows what they want to prove, who is learning a new tool to prove it rigorously, and who is contributing back to the community's infrastructure as they go.

---

## Anne Baanen (present)
*Algebraist and Lean 4 contributor; specialist in algebraic number theory formalization and typeclass design.*

Anne Baanen is a researcher in the formal methods and verification community, with a focus on the formalization of algebraic number theory and ring theory. Baanen completed a PhD at Vrije Universiteit Amsterdam with work on formalization of algebraic number theory in Lean, and has been a prolific contributor to Mathlib in the areas of ring theory, module theory, and algebraic number theory.

Baanen's most technically significant contributions to Mathlib are in the area of typeclass design for algebraic hierarchies. The challenge of representing mathematical hierarchies (groups, rings, modules, algebras, ...) in a dependent type system is non-trivial: the typeclass graph must be carefully organized to avoid diamonds, ensure instance resolution terminates, and support both definitional and propositional equality of structure components. Baanen has contributed key typeclass instances and design patterns in `Mathlib.Algebra.Module`, `Mathlib.RingTheory`, and related areas, including the formalization of Dedekind domains and ideals in rings of integers.

For this chapter, Baanen's work is most relevant in the `ring` and `linarith` sections: the algebraic automation tactics that fire on `CommRing`, `LinearOrderedField`, and similar typeclasses are backed by the carefully designed typeclass instances that Baanen and the Mathlib community have developed. When `ring` proves a polynomial identity automatically, it is using the `CommRing` typeclass whose design reflects years of collective work — Baanen's contributions among the most substantial.
