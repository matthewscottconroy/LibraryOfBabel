# 91 — References

## How to Use This Bibliography

This is not a comprehensive bibliography of HoTT. It is a curated reading list: the papers and texts that matter most for someone entering research-level HoTT from this curriculum. Each entry is annotated to explain what you get from it and when to read it.

The list is organized from foundational to frontier. Start at the top; come back to the later entries as your research matures.

---

## Primary Texts

**Univalent Foundations Program. *Homotopy Type Theory: Univalent Foundations of Mathematics.* Institute for Advanced Study, 2013. Free at homotopytypetheory.org/book.**

The foundational text of the field. Chapters 1–6 are essential: identity types, h-levels, the fundamental theorem of identity types, equivalences, univalence, and HITs. Chapters 7–10 are research content: homotopy theory (π₁(S¹), van Kampen, covering spaces), set theory (ordinals, cardinal arithmetic), real numbers (Cauchy and Dedekind completions), and formal topology.

Do not read this linearly. Read Chapters 1–6 carefully, then use the later chapters as reference. Every subsequent paper cites this book for standard results; knowing the notation and the conventions is essential.

**Guillaume Brunerie. "On the Homotopy Groups of Spheres in Homotopy Type Theory." PhD thesis, Université Nice Sophia Antipolis, 2016. arXiv:1606.05916.**

The proof of π₄(S³) = ℤ/2ℤ in HoTT. This is both a mathematical landmark and a guide to what hard synthetic homotopy theory looks like. The introduction (pages 1–20) is essential reading for anyone interested in the Brunerie problem. Chapter 2 develops the Hopf fibration. Chapter 3 introduces the Brunerie number. The full proof is over 100 pages of dense mathematics.

Read strategy: introduction carefully, then Chapter 2 (Hopf fibration), then return to later chapters as needed. Do not try to read it all at once.

---

## Cubical Type Theory

**Cyril Cohen, Thierry Coquand, Simon Huber, and Anders Mörtberg. "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom." *Logical Methods in Computer Science*, 2020. (First appeared at TYPES 2015/2016.)**

The CCHM paper: the foundation of Cubical Agda. Introduces the De Morgan interval, cubical path types, the Glue type (which gives univalence its computation rule), and proves canonicity (every closed natural number reduces to a numeral). Essential for understanding why Cubical Agda can compute things that Book HoTT cannot.

Read this after understanding Book HoTT. The contrast between "univalence as axiom" and "univalence as theorem from Glue" is the central conceptual move.

**Thierry Coquand. "Canonicity and Normalization for Dependent Type Theory." arXiv:1810.09367, 2019.**

Proves normalization (not just canonicity) for a version of cubical type theory. A technical paper, but important for anyone interested in the computational theory of HoTT — particularly the open problem of canonicity for simplicial type theory, which this paper illuminates by contrast.

**Simon Huber. "Canonicity for Cubical Type Theory." *Journal of Automated Reasoning*, 2018.**

The original proof of canonicity for cubical type theory. More accessible than Coquand's 2019 paper; start here for the canonicity story.

---

## Simplicial Type Theory

**Emily Riehl and Michael Shulman. "A Synthetic Theory of ∞-Categories in Homotopy Type Theory." arXiv:1705.07442, 2017.**

The founding paper of simplicial type theory. Introduces the two-interval setup, Segal types, Rezk types, and proves the synthetic Yoneda lemma. The paper is long (100+ pages); the introduction and Sections 1–5 give the essential ideas. Read this after Chapter 24 of this curriculum.

The directed univalence problem is explicitly stated in this paper; reading the paper makes clear exactly what the problem is asking and why it is hard.

**Nikolai Kudasov, Emily Riehl, and Jonathan Weinberger. "Formalizing the ∞-Categorical Yoneda Lemma." *Proceedings of CPP 2024*, 2024.**

The formalization of the synthetic Yoneda lemma in Rzk. The first machine-verified proof of a central theorem in synthetic ∞-category theory. Both a mathematical paper (the Yoneda lemma in STT) and a formalization paper (how to encode STT arguments in Rzk). A model of how formalization papers should be written.

---

## Semantics and Foundations

**Steve Awodey and Michael A. Warren. "Homotopy-Theoretic Models of Identity Types." *Mathematical Proceedings of the Cambridge Philosophical Society*, 2009.**

The paper that made "types are homotopy types" precise: identity types can be modeled by the path objects of any Quillen model category. This gives the semantic foundation for HoTT. Short (15 pages) and essential for understanding why HoTT works.

**Martin Hofmann and Thomas Streicher. "The Groupoid Interpretation of Type Theory." In *Twenty-Five Years of Constructive Type Theory*, Oxford University Press, 1998.**

The paper that started it all: the groupoid model shows that UIP (uniqueness of identity proofs) is not provable in MLTT. Without this, the discovery that types have homotopy-theoretic content would have been blocked by the assumption that all proofs of a = b are equal. Short and readable as historical context.

**Peter Lumsdaine and Michael Shulman. "Semantics of Higher Inductive Types." *Mathematical Proceedings of the Cambridge Philosophical Society*, 2020.**

The paper that resolves the consistency and semantics of a large class of HITs. Shows that HITs specifiable as "cell monads" have models as left adjoints in ∞-toposes. Essential for anyone working on the general coherence problem for HITs (Problem 3).

---

## Synthetic Homotopy Theory

**Axel Ljungström and Anders Mörtberg. "Symmetric Monoidal Smash Products in HoTT." *Proceedings of LICS 2023*, 2023.**

The current state of the art on Brunerie's problem. Reformulates the computation of the Brunerie number using symmetric monoidal smash products, reducing the proof term to a size that Cubical Agda can check. Both a mathematical paper (new results about smash products) and a computational achievement (machine verification of π₄(S³)).

**Mathieu Anel, Georg Biedermann, Eric Finster, and André Joyal. "A Generalized Blakers-Massey Theorem." *Journal of Topology*, 13(4):1521–1553, 2020.**

Proves the Blakers-Massey theorem in an arbitrary ∞-topos, using a synthetic argument about pushouts. The technique — excisive functors, the orthogonal factorization system for n-connected and n-truncated maps — is now standard in synthetic homotopy theory and has been formalized in Cubical Agda. This paper is the state of the art in synthetic homotopy theory.

---

## Cohesive and Modal HoTT

**Michael Shulman. "Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory." *Mathematical Structures in Computer Science*, 28(6):856–941, 2018.**

The flagship application of cohesive HoTT. Proves the Brouwer fixed-point theorem from the cohesion axioms using the shape modality — no coordinates, no analysis, just type-theoretic axioms about geometry. A model of how to write a cohesive HoTT paper: axioms clearly stated, proof self-contained, mathematical content at the forefront.

**David Jaz Myers. "Simplicial, Divisorial, and Orientable Cohesion." arXiv:2102.05848, 2021.**

Unifies multiple kinds of cohesion (smooth, algebraic, combinatorial) into a single framework. Shows that theorems proved in real-cohesive HoTT hold in a much more general setting. Important for anyone interested in the connection between cohesive HoTT and condensed mathematics.

---

## Recent Frontier Papers (2020–2025)

**Ulrik Buchholtz, Floris van Doorn, and Egbert Rijke. "Higher Groups in Homotopy Type Theory." *LICS 2018*, 2018; extended as "Central H-Spaces and Banded Types." arXiv:2301.02636, 2023.**

Studies H-spaces and their centralizers using synthetic homotopy theory. Proves new results about the homotopy structure of classifying spaces that have no simpler classical proof. Demonstrates the power of HoTT methods for problems that classical algebraic topology handles more awkwardly.

**Tom de Jong and Martín Hötzel Escardó. "On Small Types in Univalent Foundations." *LICS 2023*, 2023.**

Studies the set-theoretic aspects of HoTT: universe levels, the "type of small types," and how the stratification by universe level affects the mathematics. Important for anyone who has wondered whether HoTT's universe structure is a feature or a limitation.

**Thorsten Altenkirch, Ambrus Kaposi, et al. "Quotient Inductive-Inductive Types." *Formal Structures in Computation and Deduction*, 2018; extended 2022.**

Develops a general framework for quotient inductive-inductive types (QIITs) — a class of HITs that includes many practically important examples (the integers, the Cauchy reals, many syntax types). Directly relevant to Problem 3 (general syntax for HITs).

**Jonathan Weinberger. "Strict Stability of Extension Types." arXiv:2203.07194, 2022.**

Proves that extension types in simplicial type theory satisfy a strict stability condition, giving them good computational properties. A technical foundational result for the Rzk program.

**Egbert Rijke. "Introduction to Homotopy Type Theory." arXiv:2212.11082, 2022.**

An ongoing textbook project developing synthetic homotopy theory from first principles. Currently (2025) covers h-levels, pushouts, van Kampen, Freudenthal, and the James construction. A self-contained treatment of the synthetic approach in the HoTT Book style, without cubical computation rules.

---

## Formalization Resources

**The Cubical Agda Library. github.com/agda/cubical**

The primary HoTT formalization library. Contains: the circle, Hopf fibration, Freudenthal, Blakers-Massey, pushouts, truncations, Eilenberg-MacLane spaces, π₁(S¹) = ℤ, π₄(S³) = ℤ/2ℤ (Brunerie number), and much more. The issues list is the most accurate real-time map of formalization gaps.

**The sHoTT Library. github.com/rzk-lang/sHoTT**

The formalization library for simplicial type theory in Rzk. Contains: Segal types, Rezk types, the Yoneda lemma, adjunction theory (partial). The primary library for anyone working in the Riehl-Shulman direction.

**The UniMath Library. github.com/UniMath/UniMath**

Voevodsky's formalization library in Coq. Contains: foundations of HoTT (in Book HoTT style), category theory, algebra. Uses axiomatic univalence rather than cubical computation rules, giving a more foundationally pure but less computationally powerful system.

**HoTTEST Seminar Archive. uwo.ca/math/faculty/kapulkin/seminars/hottest.html**

Archive of all HoTTEST talks since 2018. The talks from 2022–2025 give a current picture of the research frontier. Essential viewing for anyone entering the field.

**HoTT Zulip. hott.zulipchat.com**

The central online community. The fastest way to get answers to specific technical questions and to understand what the community is currently working on.
