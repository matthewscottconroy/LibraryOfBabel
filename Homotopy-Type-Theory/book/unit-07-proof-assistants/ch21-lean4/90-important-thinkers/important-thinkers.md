# Important Thinkers: Lean 4 and the Formalization of Mathematics

## Leonardo de Moura (1971–present)
*Principal architect of Lean and co-creator of the Z3 SMT solver; the central figure of modern interactive theorem proving.*

Leonardo de Moura received his PhD from the Pontifical Catholic University of Rio de Janeiro and subsequently joined SRI International and then Microsoft Research. At Microsoft Research he led the development of Z3, which became — and remains — the most widely used satisfiability modulo theories (SMT) solver in both academia and industry. Z3 is embedded in program verification tools, compilers, hardware verification frameworks, and security analysis systems worldwide. Its design reflects de Moura's commitment to combining theoretical rigor with engineering pragmatism: correctness and speed are not in tension, they are the same goal stated differently.

Lean emerged from de Moura's conviction that the historical gap between interactive proof assistants and automated solvers was artificial. Lean 1, Lean 2, and Lean 3 explored increasingly sophisticated elaboration algorithms — the mechanisms by which user-written terms and tactics are resolved into fully explicit proof terms for the kernel to check. Lean 3's elaborator was capable enough to support Mathlib's growth to tens of thousands of theorems. But de Moura recognized that the system had hit fundamental architectural limits.

Lean 4 was designed from scratch. The key innovation: the language is metacircular. Tactics, macros, and the proof assistant infrastructure are all written in Lean 4 itself, using the same dependent type theory that users write proofs in. This collapses the historically sharp boundary between "user" and "system," makes Lean 4 uniquely extensible, and enables a new generation of tools — automated tactic generators, LLM-powered proof search, machine learning systems trained on Lean proofs — to be built within the system rather than outside it.

De Moura's influence on this chapter's content is pervasive. The `Prop`/`Type` distinction, the universe polymorphism system, the typeclass elaboration, the `rfl`/`exact`/`apply` tactic pipeline, and the definitional equality algorithm are all traceable to his design decisions. His insight that a theorem prover should also be a full-featured programming language — a platform for computation, automation, and eventually machine intelligence in mathematics — shapes the direction of the entire field.

---

## Jeremy Avigad (1968–present)
*Mathematical logician; leader of the Mathlib community; author of the primary Lean 4 learning resources.*

Jeremy Avigad is a professor of philosophy and mathematical sciences at Carnegie Mellon University, with research spanning proof theory, the history and philosophy of mathematics, formal verification, and the mathematics of analysis and number theory. He is the primary author of *Theorem Proving in Lean 4* and *Mathematics in Lean* — the two resources that most learners of Lean 4 encounter first.

Avigad's specific contribution to the topics in this chapter is pedagogical architecture: the way the `Prop`/`Type` distinction is explained to mathematicians, the sequencing of tactics from the elementary (`intro`, `exact`, `apply`) to the automated (`simp`, `ring`, `linarith`, `omega`), and the way formalization is connected to classical mathematical practice rather than type-theoretic foundations. *Mathematics in Lean* does not assume knowledge of type theory — it assumes you know mathematics and teaches you to express it in Lean. This choice has brought a large cohort of research mathematicians into the ecosystem.

Within Mathlib, Avigad has contributed to the formalization of number theory (analytic and algebraic), ergodic theory, combinatorics, and probability. He is an organizer of the Lean Forward project and has been influential in establishing the community's norms around documentation, naming conventions, and the balance between tactic automation and explicit proof structure. His work represents the clearest articulation of what it means for a proof assistant to serve mathematicians rather than logicians.

---

## Kevin Buzzard (1969–present)
*Number theorist and algebraic geometer; leader of the Lean formalization of Fermat's Last Theorem; the mathematician who brought Lean to pure mathematics.*

Kevin Buzzard is a professor of pure mathematics at Imperial College London, specializing in the Langlands program, automorphic forms, and the arithmetic of elliptic curves. His mathematical work is at the frontier of algebraic number theory — precisely the kind of content that formalization was long thought to be decades away from touching. Around 2017, Buzzard began working in Lean and rapidly concluded that its capabilities were being systematically underestimated by the pure mathematics community.

He began using Lean in his teaching, created the annual *Formalising Mathematics* course at Imperial (freely available online and widely used), and launched the project of formalizing the proof of Fermat's Last Theorem. This is not a distant goal but a structured, ongoing multi-year project with a Leanblueprint dependency graph and a growing international community of contributors.

The Fermat's Last Theorem project has already produced formalizations of the theory of modular forms, Galois representations, Serre's $\varepsilon$-conjecture, and substantial parts of the Taylor-Wiles method. Buzzard's contributions to Mathlib include work on p-adic numbers, group cohomology, and the algebraic geometry needed for the Langlands program. His Imperial course has introduced dozens of research mathematicians to Lean 4.

Buzzard represents the model user this chapter aims at: a mathematician who knows what they want to prove, who has learned a proof assistant to verify it with full rigor, and who contributes back to the library infrastructure as they go.

---

## Patrick Massot (present)
*Differential geometer and Lean formalization pioneer; author of leanblueprint; co-formalizer of perfectoid spaces.*

Patrick Massot is a professor at Université Paris-Saclay (Orsay), specializing in contact topology and symplectic geometry. He became involved in Lean formalization after observing Mathlib's analysis library grow to maturity, and is now one of Mathlib's most active contributors outside the core team.

His most widely reported achievement (joint with Kevin Buzzard and Johan Commelin) is the formalization of *perfectoid spaces* in Lean 3 — announced in 2019, widely described as the first piece of research-level algebraic geometry to be machine-verified. Perfectoid spaces, invented by Peter Scholze in 2011 (work for which Scholze received a Fields Medal), are technical objects in the arithmetic geometry of p-adic numbers. Their formalization required hundreds of definitions and lemmas before the central definition could even be stated.

For this chapter, Massot's most directly relevant contribution is the `leanblueprint` tool: a system for writing a structured LaTeX proof outline (a "blueprint") with dependency information, linking each node to its Lean 4 formalization. This operationalizes the workflow of large-scale formalization: write the mathematics first, formalize each piece in order of dependencies, track completeness visually. The Fermat's Last Theorem project uses leanblueprint.

Massot has also contributed extensively to Mathlib's topology library (topological groups, uniform spaces, completions) and to the analysis library (metric spaces, real analysis, measure theory). His work exemplifies the possibility of doing active research mathematics simultaneously with its formalization — not as separate activities but as a unified practice.

---

## Sebastien Gouezel (present)
*Analyst and geometer; Lean 4 formalization contributor; specialist in the mathematics of dynamical systems and real analysis.*

Sebastien Gouezel is a professor at Nantes (and later Rennes), specializing in ergodic theory, hyperbolic dynamics, and the geometry of metric spaces. He is one of the mathematicians who has most successfully combined active research with systematic Lean formalization, contributing to Mathlib in the areas of measure theory, integration, topology, and analysis.

His most technically significant formalization achievement is the proof of the Morse lemma (in the context of hyperbolic geometry) in Lean, which required developing substantial infrastructure around metric spaces and geodesics. Gouezel's contributions to Mathlib's measure theory library — in particular the `MeasureTheory` and `Analysis.SpecialFunctions` modules — have enabled later formalizations of probability theory and harmonic analysis.

For the topics in this chapter, Gouezel's work matters because it demonstrates that real-analysis infrastructure — the kind needed for classical topology and its connection to HoTT via homotopy theory — is not only formalizable but has been formalized. The classical mathematical backbone that HoTT rests on (metric spaces, continuous functions, covering spaces) is available in Mathlib because of the cumulative work of contributors like Gouezel.

---

## Thomas Hales (1958–present)
*Mathematician and proof assistant pioneer; director of the Flyspeck project; formalizer of the Kepler conjecture.*

Thomas Hales is a professor at the University of Pittsburgh, known primarily for his 1998 proof of the Kepler conjecture — the claim that the face-centered cubic packing is the densest arrangement of equal spheres in three-dimensional space. The proof, roughly 300 pages plus extensive computer calculations, was initially met with skepticism: the referees reported being 99% confident of its correctness after four years of review, which is not the kind of certainty mathematics usually demands.

Hales's response was to formalize the proof in a proof assistant, undertaking what became the Flyspeck project (Formal Proof of the Kepler conjecture). After a decade of work, largely using a combination of HOL Light and Isabelle/HOL, the formalization was completed in 2014. The result: a machine-verified proof that leaves no room for the 1% doubt.

Hales's significance for this chapter is not technical (Flyspeck used HOL Light, not Lean 4) but conceptual. He established the model that the mathematics community has since adopted: when a proof is too complex for informal review to achieve full confidence, formalization is the solution. The four-color theorem (Gonthier), the Kepler conjecture (Hales), the odd-order theorem (Gonthier and collaborators), and the proof of Fermat's Last Theorem (in progress) follow this model. Lean 4 and Mathlib are the current state-of-the-art platform for this enterprise.
