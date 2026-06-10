# References and Primary Sources

## Foundational Texts and Papers

This chapter concerns the research frontier, so the "foundational texts" are the papers that established the field and that every active researcher has read. The list below is a curated bibliography — not exhaustive, but the essential reading for someone entering HoTT research from this curriculum.

**The HoTT Book (Univalent Foundations Program, Institute for Advanced Study, 2013)**
The primary text of the field. Chapters 1–6 contain the core theory; Chapters 7–10 are already research content. Every paper in the HoTT literature cites this book, and understanding the standard notations and conventions is essential for reading current papers. Free at homotopytypetheory.org/book.

**Guillaume Brunerie, "On the Homotopy Groups of Spheres in Homotopy Type Theory" (PhD thesis, Université Nice Sophia Antipolis, 2016)**
The proof of $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$ in HoTT. This is both a landmark result — the first non-trivial homotopy group computed entirely in type theory — and a template for synthetic homotopy theory research. The introduction explains the overall strategy; Chapter 2 develops the Hopf fibration; Chapter 3 constructs the Brunerie number. The 2022 simplification by Ljungström-Mörtberg makes the computational argument more tractable but the conceptual structure is clearest in Brunerie's original.

**Cyril Cohen, Thierry Coquand, Simon Huber, and Anders Mörtberg, "Cubical Type Theory: A Constructive Interpretation of the Univalence Axiom" (TYPES 2015/2016)**
The CCHM paper: the cubical type theory that underlies Cubical Agda. Shows that the univalence axiom has a computational interpretation (via the Glue type), solving the longstanding canonicity problem for HoTT with a specific computational rule. Every formalization in Cubical Agda rests on this foundation.

**Emily Riehl and Michael Shulman, "A Synthetic Theory of ∞-Categories in Homotopy Type Theory" (2017, arXiv:1705.07442)**
The founding paper of simplicial type theory. Introduces the two-interval framework and proves the synthetic Yoneda lemma. The open problems it leaves — directed univalence, canonicity for STT — are among the central open problems in current research.

**Mathieu Anel, Georg Biedermann, Eric Finster, and André Joyal, "A Generalized Blakers-Massey Theorem" (Journal of Topology, 13(4):1521–1553, 2020)**
Proves the Blakers-Massey theorem in an arbitrary ∞-topos, using a synthetic argument about pushouts. The proof technique — excisive functors, the "descent" condition — is now standard in synthetic homotopy theory and has been formalized in Cubical Agda.

**Mike Shulman, "Brouwer's Fixed-Point Theorem in Real-Cohesive Homotopy Type Theory" (MSCS, 2018)**
The flagship application of cohesive HoTT. Proves the Brouwer theorem from the cohesion axioms. The paper is also a model of how to write a cohesive HoTT paper: axioms stated clearly, proof self-contained, mathematical content at the forefront.

---

## Seminal Papers

**Martin Hofmann and Thomas Streicher, "The Groupoid Interpretation of Type Theory" (1998, in: Twenty-Five Years of Constructive Type Theory)**
The paper that started it all: the groupoid model of type theory, showing that UIP (uniqueness of identity proofs) is not provable in MLTT. Without this paper, the discovery that types have homotopy-theoretic content would have been blocked. Short (27 pages) and essential as historical context.

**Steve Awodey and Michael A. Warren, "Homotopy-Theoretic Models of Identity Types" (Mathematical Proceedings of the Cambridge Philosophical Society, 2009)**
Generalizes the Hofmann-Streicher groupoid model to show that identity types can be modeled by any Quillen model category (via the path objects). This is the paper that makes "types are homotopy types" precise and gives the semantic foundation for all of HoTT.

**Vladimir Voevodsky, "Univalent Foundations of Mathematics" (IAS lecture, 2010; notes available)**
The foundational talk in which Voevodsky first publicly presented the univalence axiom and the homotopy interpretation of type theory. Historical document, but also conceptually important: Voevodsky explains why he found classical foundations (ZFC) inadequate and why univalence resolves the foundational issue.

**Axel Ljungström and Anders Mörtberg, "Symmetric Monoidal Smash Products and the Brunerie Number" (LICS 2023)**
The 2023 paper that significantly simplified the computation of the Brunerie number $\pi_4(S^3) = \mathbb{Z}/2\mathbb{Z}$. Reduces the proof to a computation that can actually be checked by a computer (the original proof was too large for the Agda type-checker). This is the current state of the art on Brunerie's problem.

**Eric Finster and Samuel Mimram, "A Type-Theoretical Definition of Weak ω-Categories" (LICS 2017)**
Introduces "globular type theory," a type theory for weak ω-categories using a type-theoretic analogue of the Batanin tree structure. Related to the problem of giving a general syntax for higher inductive types and to the question of how HoTT relates to computad-based definitions of ω-categories.

**Thierry Coquand, "Canonicity and Normalization for Dependent Type Theory" (2019, arXiv:1810.09367)**
Proves normalization for a version of cubical type theory, a key step toward understanding the computational content of HoTT. Relevant to the open problem of canonicity for simplicial type theory (Problem 5 in Chapter 26).

---

## The Current Frontier: Papers from 2020–2025

These papers represent active areas of research; they are not yet "canonical" but are among the most important current work.

**Thorsten Altenkirch, Ambrus Kaposi, et al., "Quotient Inductive-Inductive Types" (2018–2022)**
The formal syntax for a class of HITs beyond the 1-dimensional ones. Establishes a general grammar for "quotient inductive-inductive types" (QIITs) and their semantics in ∞-toposes. Directly relevant to Problem 3 (general syntax for HITs).

**David Jaz Myers, "Simplicial, Divisorial, and Orientable Cohesion" (2022)**
Develops a unified framework for different kinds of cohesion (smooth, algebraic, combinatorial). Shows that many theorems proved in real-cohesive HoTT hold in other cohesive settings as well.

**Nikolai Kudasov, Emily Riehl, and Jonathan Weinberger, "Formalizing the ∞-Categorical Yoneda Lemma" (CPP 2024)**
The formalization of the synthetic Yoneda lemma in Rzk. First machine-verified proof of this central STT theorem.

**Tom de Jong and Martín Hötzel Escardó, "On Small Types in Univalent Foundations" (LICS 2023)**
Studies the set-theoretic aspects of HoTT: what is the "set of small types," how does the universe stratification interact with large mathematical structures. Relevant to set-theoretic aspects of formalization and to the relationship between HoTT and classical foundations.

**Ulrik Buchholtz, et al., "Central H-Spaces and Banded Types" (2023)**
Studies "central" H-spaces (spaces where the multiplication is central) using HoTT methods, proving new results about the homotopy structure of classifying spaces. Represents the ongoing development of synthetic algebraic topology.

---

## Online Resources and Formalization

**HoTT Zulip: hott.zulipchat.com**
The central online community for HoTT research. Streams include general discussion, announcements of new papers and results, job listings, and dedicated streams for specific tools (Agda, Lean). Reading a few weeks of the "general" stream is the fastest way to understand what the community is currently working on. Asking well-formulated questions here is the most direct way to get help from active researchers.

**Cubical Agda Library: github.com/agda/cubical**
The primary formalization library for HoTT in Cubical Agda. Contains the circle, the Hopf fibration, Freudenthal, pushouts, Blakers-Massey, and much more. The issues list identifies specific formalization tasks that the maintainers consider valuable. This is the best place to find an approachable first formalization project.

**Rzk Library (sHoTT): github.com/rzk-lang/sHoTT**
The formalization library for simplicial type theory in Rzk. Actively developed and directly connected to the Riehl-Shulman research program. Contributing here is contributing to the active research frontier of STT.

**The HoTTEST Seminar Archive: uwo.ca/math/faculty/kapulkin/seminars/hottest.html**
Archive of all HoTTEST seminar talks since 2018. These talks cover active research, open problems, and recent results. Watching the talks from the past two or three years gives a current picture of the field that no textbook can provide.

**arxiv.org: cs.LO and math.LO categories**
The primary venue for new HoTT preprints. Setting up email alerts for "homotopy type theory," "cubical type theory," and "simplicial type theory" is the standard way to track new papers. Most papers appear on arXiv before publication, often months to years before.

---

## Historical Context

The HoTT field is young enough that its entire history is traceable. The homotopy interpretation of type theory was developed in three phases: the logical phase (1998–2008, starting with Hofmann-Streicher and running through the groupoid model work), the foundational phase (2009–2013, with Voevodsky's univalence axiom and the formation of the Univalent Foundations Program that produced the HoTT Book), and the technical expansion phase (2013–present, producing cubical type theory, simplicial type theory, cohesive HoTT, and the first serious formalization libraries).

The HoTT Book itself (2013) was an extraordinary event in mathematical history: over fifty mathematicians, type theorists, and computer scientists collaborated in a single year-long program at the IAS to write a definitive reference for a new field. The list of authors reads like a who's who of the community, and the collaborative production process — unusual for mathematics — reflected the community's values: openness, collective authorship, and the conviction that foundations should be publicly accessible. The book remains the central text of the field over a decade later.

What distinguishes the current period (2020–2026) from the previous decade is the maturation of the formalization tools. Cubical Agda has made large-scale formalization practical; the Brunerie number computation, the Blakers-Massey theorem, and the Freudenthal suspension theorem are all machine-verified. The Rzk proof assistant has made STT results machine-verifiable for the first time. Lean 4 and Mathlib are increasingly able to handle HoTT content alongside classical mathematics. The frontier is no longer "can we formalize X?" but "what is the right way to formalize X, and what new mathematics does the formalization reveal?" — a much more interesting question.
