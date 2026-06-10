# References and Primary Sources

## Primary Documentation and Tools

**Agda** — the proof assistant that implements Cubical Agda. The main repository and release downloads are at:
- Repository: [github.com/agda/agda](https://github.com/agda/agda)
- Official documentation: [agda.readthedocs.io](https://agda.readthedocs.io/)
- Releases and binaries: [github.com/agda/agda/releases](https://github.com/agda/agda/releases)

To use Cubical Agda, enable the `--cubical` pragma (or `--erased-cubical` for a version with erased interval). The cubical mode is built into Agda core — it is not a separate tool.

**The Cubical Agda Library** — the primary library for cubical type theory in Agda.
- Repository: [github.com/agda/cubical](https://github.com/agda/cubical)
- Documentation: generated from source; browse the `.agda` files directly.
- Key namespaces: `Cubical.Core`, `Cubical.Foundations`, `Cubical.HITs`, `Cubical.Homotopy`, `Cubical.Algebra`.

**Agda Standard Library** — the non-cubical standard library, based on `--without-K` (Hedberg-safe, but not cubical).
- Repository: [github.com/agda/agda-stdlib](https://github.com/agda/agda-stdlib)
- Note: the standard library is not compatible with `--cubical` mode. The cubical library is the correct choice for cubical Agda work.

**HoTT-Agda** — an older library for HoTT in Agda, written before cubical mode existed, using axioms for univalence and funext. Primarily of historical interest, but some content has not yet been ported to the cubical library.
- Repository: [github.com/HoTT/HoTT-Agda](https://github.com/HoTT/HoTT-Agda)

---

## Foundational Papers

**Andrej Bauer, Jason Gross, Peter LeFanu Lumsdaine, Mike Shulman, Matthieu Sozeau, Bas Spitters.** "The HoTT Library: A Formalization of Homotopy Type Theory in Coq." *Proceedings of CPP 2017*, pp. 164–172. ACM, 2017.
The foundational formalization paper for axiomatic HoTT. Documents the design choices made when encoding univalence and HITs axiomatically, before cubical mode was available. Provides the clearest account of what it means to formalize HoTT *without* computational content — and thereby motivates why Cubical Agda is necessary.

**Thierry Coquand, Simon Huber, Anders Mörtberg.** "On Higher Inductive Types in Cubical Type Theory." *Proceedings of LICS 2018*. IEEE, 2018.
The paper that gives the precise typing rules for HITs in cubical type theory, including the computation rules on path constructors. This is the theoretical foundation for `S¹`, `Susp`, `Trunc`, and `Pushout` in the cubical Agda library. Shows how path constructors have genuine computational content (they are not axioms).

**Cyril Cohen, Thierry Coquand, Simon Huber, Anders Mörtberg.** "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom." *IfCoLoG Journal of Logics and Their Applications* 4(10):3127–3169, 2017. (Preliminary version at TYPES 2015.)
The primary theoretical reference for the type theory underlying Cubical Agda. Establishes the interval, face formulas, the composition operation (hcomp), transport, and the Glue type, and proves univalence as a theorem. This paper is the theoretical foundation for the entire chapter.

**Andrea Vezzosi, Anders Mörtberg, Andreas Abel.** "Cubical Agda: A Dependently Typed Programming Language with Univalence and Higher Inductive Types." *Proceedings of ICFP 2019*, Vol. 3. ACM, 2019.
The primary paper for Cubical Agda as a tool. Describes the implementation of the cubical primitives in Agda, the `--cubical` pragma, the interval operations, and HITs. Documents the `transp` and `hcomp` primitives as they appear to Agda users, with examples. This is the paper to read alongside the chapter text.

**Ulf Norell.** "Towards a Practical Programming Language Based on Dependent Type Theory." PhD thesis, Chalmers University of Technology, 2007.
The original Agda thesis. Norell's design of Agda's implicit argument system, universe polymorphism, pattern matching, and `with`-clauses established the foundations on which Cubical Agda is built. Understanding Agda's core elaboration (how implicit arguments are solved, how pattern matching compiles) requires this thesis.

**Martin Hofmann and Thomas Streicher.** "The groupoid interpretation of type theory." *Twenty-Five Years of Constructive Type Theory*, Oxford Logic Guides 36, pp. 83–111. Oxford University Press, 1998.
The groupoid model paper: the first semantics showing that identity types can be interpreted as morphisms in a groupoid, not just proof-irrelevant propositions. This is the conceptual precursor to the cubical model — it showed that `a = b` could have multiple distinct inhabitants and that this is consistent. Every path-theoretic concept in this chapter traces back here.

---

## Textbooks and Learning Resources

**The Univalent Foundations Program.** *Homotopy Type Theory: Univalence, Higher Inductive Types, and Their Applications.* Institute for Advanced Study, 2013. Available free at [homotopytypetheory.org/book](https://homotopytypetheory.org/book/).
The HoTT Book. Not specific to Cubical Agda, but provides the mathematical content that Cubical Agda is implementing. Chapters 1–3 (type theory), Chapter 4 (equivalences), Chapter 6 (higher inductive types), and Chapter 8 (homotopy groups) are most relevant to this chapter. Essential background reading.

**Anders Mörtberg.** *Introduction to Cubical Type Theory.* Lecture notes, CMU and Stockholm University, 2019–2022. Available at [staff.math.su.se/anders.mortberg/papers/cubicaltt-lecture.pdf](https://staff.math.su.se/anders.mortberg/papers/cubicaltt-lecture.pdf) (and through the author's webpage).
Lecture notes specifically targeted at learning cubical type theory and Cubical Agda. Covers the interval, paths, transport, hcomp, the Glue type, HITs, and the π₁(S¹) = ℤ computation. Written by the lead Cubical Agda library maintainer — the closest thing to an official textbook for this chapter's content.

**Martín Hötzel Escardó.** *Introduction to Univalent Foundations of Mathematics with Agda.* Available at [github.com/martinescardo/TypeTopology](https://github.com/martinescardo/TypeTopology) (as a literate Agda source).
A comprehensive development of univalent foundations in Agda (using `--without-K` but not full cubical). Covers propositions, sets, h-levels, the K axiom, Hedberg's theorem, and much more — all in directly runnable Agda. The TypeTopology library is a self-contained alternative to the cubical library for mathematicians interested in univalent foundations.

**Egbert Rijke.** *Introduction to Homotopy Type Theory.* Available at [github.com/EgbertRijke/HoTT-Intro](https://github.com/EgbertRijke/HoTT-Intro) (book manuscript + Agda formalization).
A newer HoTT textbook aimed at graduate mathematicians, with parallel Agda formalizations in a non-cubical style. Excellent for building mathematical intuition for what Cubical Agda is formalizing. The Agda code is clean and well-commented.

**Simon Boulier, Egbert Rijke, Nicolas Tabareau.** "The J rule is equivalent to the circle." *Preprint*, 2022.
A technical paper showing that the J elimination rule for identity types is equivalent (in a precise sense) to the existence of a non-trivial loop at the base of the circle. Relevant to understanding why the circle is so fundamental to path type theory.

---

## Key Libraries and Online Resources

**The Cubical Agda Library** — [github.com/agda/cubical](https://github.com/agda/cubical)
The primary library. Browse `Cubical/Core/` for the primitives, `Cubical/Foundations/` for path algebra and equivalences, `Cubical/HITs/` for the circle, suspension, and truncations, and `Cubical/Homotopy/` for homotopy groups. The library is well-commented and serves as both a reference and a learning resource.

**agda.readthedocs.io** — [agda.readthedocs.io](https://agda.readthedocs.io/)
The official Agda documentation. The section on *Cubical Agda* (under Language Reference) describes the `--cubical` pragma, the interval primitives, the `Glue` type, and HITs. Essential for understanding what the language provides versus what the library adds on top.

**The Agda Zulip** — [agda.zulipchat.com](https://agda.zulipchat.com/)
The primary Agda community forum. The `#cubical` and `#general` streams are active. For questions about cubical-specific behavior (why a `transp` gets stuck, how to use `hcomp` for a specific filling problem), the Zulip is the fastest route to an answer.

**cubical.readthedocs.io** — (hosted documentation generated from the cubical library, linked from the github repo)
The auto-generated documentation for the cubical library. Useful for browsing type signatures without reading raw Agda.

**Agda Mode for Emacs / VS Code** — the standard development environment. Emacs with `agda2-mode` is the traditional setup; the `agda-mode` VS Code extension provides interactive hole-filling and goal display. Type `C-c C-l` (load file), `C-c C-,` (goal type), `C-c C-.` (goal type and inferred type), `C-c C-a` (auto-solve hole) — the interactive workflow is essential for cubical Agda development.

---

## Historical Context

Agda was first developed by Catarina Coquand at Chalmers University of Technology in the late 1990s as a proof assistant based on Martin-Löf type theory. Ulf Norell's 2007 PhD thesis essentially redesigned Agda from the ground up — the "Agda 2" that Norell implemented introduced the elaboration algorithm, implicit argument solving, universe polymorphism, and the interactive proof development model (holes, goal display, auto-search) that define the Agda experience today. Norell's Agda became widely used in the type theory and programming languages communities throughout the 2010s, particularly for formalization of type-theoretic results (logical relations proofs, normalization proofs, mechanized type theories).

The cubical extension of Agda grew directly out of the theoretical work by Bezem, Coquand, and Huber on cubical sets models of type theory (2013–2015), and the subsequent CCHM paper by Cohen, Coquand, Huber, and Mörtberg (2015/2017). The implementation of Cubical Agda by Simon Huber, Andrea Vezzosi, and Anders Mörtberg in 2018–2019 brought this theory to a working proof assistant. The `--cubical` flag was merged into Agda proper (not a fork) around 2018, and the Vezzosi-Mörtberg-Abel ICFP 2019 paper documented the resulting system. The cubical Agda library has grown continuously since then, with major contributions from Mörtberg (path algebra, the Glue type, HITs), Vezzosi (guarded cubical, implementation), and a wider community including Evan Cavallo, Licata, Brunerie, and others. As of 2024–2025, the library contains formalizations of $\pi_1(S^1) = \mathbb{Z}$ (the encode-decode proof and the Brunerie number computation), a synthetic homotopy theory library, and growing algebra and topology content. It remains smaller than Lean's Mathlib but is the definitive formalization environment for HoTT-specific mathematics.
