# 90 — Important Thinkers

## The People Behind the Problems

A field is its people. The theorems in this curriculum were proved by specific people at specific times, motivated by specific questions. Understanding who they are and what they are working on now is not just biographical interest — it is research intelligence. The person who proved a theorem is often the best guide to what comes next.

What follows is a map of the contemporary HoTT community, organized by the connections their work has to the open problems of this chapter.

---

## Vladimir Voevodsky (1966–2017) — Founder

Voevodsky received the Fields Medal in 2002 for his proof of the Milnor conjecture, a thirty-year-old problem in algebraic K-theory relating K-theory to Galois cohomology. The proof required developing motivic cohomology from scratch. This work was celebrated as one of the great mathematical achievements of the late twentieth century.

Then Voevodsky found an error in an earlier paper of his own. The error was not caught by referees; it went undetected for years. He began to worry about the reliability of human proof-checking at the scale required by modern mathematics.

His solution was formalization, but the existing tools (Coq, Isabelle, HOL) were built on foundations (set theory, classical logic) that made it nearly impossible to formalize the kind of mathematics Voevodsky cared about: algebraic topology, algebraic geometry, category theory. He needed a foundation where "two groups are equal if and only if they are isomorphic" was literally true — not just a convenient abuse of language.

The insight: types have homotopy-theoretic content, and by adding the univalence axiom (equivalent types are equal), you get a foundation where isomorphism is identity. He presented this in a 2010 lecture at IAS that electrified the type theory community. The subsequent Special Year at IAS (2012–2013), which produced the HoTT Book, was his organization.

Voevodsky died suddenly in September 2017 at age 51. The UniMath library (github.com/UniMath/UniMath), which he initiated in Coq, continues to be developed by his collaborators. Every formalization library in HoTT, every paper that uses univalence, every proof that identifies isomorphic structures — all of it is the continuation of the program he started.

---

## Guillaume Brunerie — The Homotopy Groups

Brunerie's 2016 PhD thesis at Université Nice Sophia Antipolis proved π₄(S³) = ℤ/2ℤ entirely within HoTT. The proof constructed the Hopf fibration synthetically, defined the Hopf invariant, set up the EHP long exact sequence, and computed the "Brunerie number" n such that π₄(S³) = ℤ/nℤ. The computation of n required running the Agda type-checker on a term too large for the checker to reduce in practice — a famous anomaly that the field has been working to resolve since.

Brunerie is currently a researcher at Stockholm University. His ongoing work includes the relationship between synthetic and classical homotopy theory, and the project (with Mörtberg and Ljungström) of finding cleaner proofs of π₄(S³) that make the computation transparent. His thesis remains the canonical reference for the hard problems of synthetic homotopy theory.

**Connection to open problems:** Problems 1 (Brunerie number), 5 (π₅(S⁴)).

---

## Anders Mörtberg — Cubical Type Theory

Mörtberg is the "M" in CCHM — the Cohen-Coquand-Huber-Mörtberg paper that introduced cubical type theory and gave univalence a computation rule. Before CCHM, univalence was an axiom with no computational content; after CCHM, it is a theorem derivable from the Glue type constructor, with explicit computation rules that make transport along equivalences computable.

Mörtberg is currently a professor at Stockholm University. He is one of the primary maintainers of the Cubical Agda library and has supervised or collaborated on many of the significant recent formalizations: the Brunerie number computation (with Ljungström), the Hopf fibration, the Eilenberg-MacLane spaces. His ongoing research focuses on improving the computational performance of Cubical Agda (making large computations like the Brunerie number more tractable) and extending the library.

**Connection to open problems:** Problems 1 (computational proof of π₄(S³)), 2 (canonicity, via the cubical model).

---

## Axel Ljungström — Computation and Formalization

Ljungström is a PhD student at Stockholm University (advisor: Mörtberg). His most significant result to date is the 2023 LICS paper with Mörtberg on the Brunerie number, which simplified the computation enough to make it machine-verifiable in Cubical Agda. The key innovation — symmetric monoidal smash products — is now a standard tool in the Cubical Agda library.

Ljungström represents the new generation of HoTT researchers: trained with the tools (Cubical Agda), able to work at the intersection of formalization and mathematics, and already producing results that extend the frontier. His ongoing work includes further simplifications of synthetic homotopy theory proofs and the development of new library infrastructure for computing homotopy groups.

**Connection to open problems:** Problem 1 (Brunerie number), Problem 5 (π₅(S⁴), as the next target for synthetic computation).

---

## Emily Riehl — Directed Type Theory

Riehl is a professor at Johns Hopkins University. Her mathematical work spans ∞-category theory, algebraic topology, and category theory, with major books on both (Categorical Homotopy Theory, 2014; Elements of ∞-Category Theory, with Verity, 2022). Her HoTT work is the development, with Michael Shulman, of simplicial type theory — a type theory where types are ∞-categories rather than ∞-groupoids.

The 2017 Riehl-Shulman paper (arXiv:1705.07442) established the foundations of simplicial type theory: the two-interval setup (the simplicial interval 2 as an object of the type theory, separate from the cubical interval), Segal types, Rezk types, and the synthetic Yoneda lemma. The paper explicitly states directed univalence as the central open problem.

Riehl's ongoing work includes the Rzk formalization program (formalizing simplicial type theory results in the Rzk proof assistant), the development of (co)cartesian fibration theory synthetically, and the question of how simplicial type theory relates to the model structures on simplicial sets and ∞-categories.

**Connection to open problems:** Problem 4 (directed univalence), the entire simplicial type theory program.

---

## Michael Shulman — Cohesive and Modal HoTT

Shulman is a professor at the University of San Diego and one of the most prolific and wide-ranging contributors to HoTT. His contributions span the foundations (he proved that every Grothendieck ∞-topos models HoTT), the semantics of HITs (Lumsdaine-Shulman 2020), modal type theory (he developed the framework for cohesive HoTT), and the applications to physics (the Brouwer fixed-point theorem paper).

Shulman's real cohesion paper (MSCS 2018) is the flagship demonstration that cohesive HoTT can do serious synthetic mathematics without coordinates. The paper proves Brouwer's fixed-point theorem using the shape modality ʃ and the crisp induction principle — a proof that no coordinates appear anywhere and that the result follows from purely type-theoretic axioms about the cohesive structure.

His current work includes the foundations of cohesive HoTT (working out which theorems hold in which versions of cohesion), the connection between cohesive HoTT and condensed mathematics (Clausen-Scholze), and the ongoing development of simplicial type theory with Riehl.

**Connection to open problems:** Problems 2 (homotopy canonicity), 4 (directed univalence), and the connections to condensed mathematics.

---

## Peter Lumsdaine — Semantics of HITs

Lumsdaine is a professor at Stockholm University. His most significant HoTT contribution (with Shulman) is the 2020 paper on the semantics of higher inductive types — showing that a large class of HITs can be given semantics as left adjoints in ∞-toposes, via the notion of a "cell monad." This resolves the consistency and semantics questions for most HITs that appear in practice.

Lumsdaine's work is primarily foundational: he is concerned with whether the type theories we use are correct (coherent, consistent, and well-modeled), rather than with proving new mathematical theorems within them. This foundational work is essential for the entire HoTT enterprise.

**Connection to open problems:** Problem 3 (coherence for HITs).

---

## Steve Awodey — Foundations and Models

Awodey is a professor at Carnegie Mellon University. His 2009 paper with Warren (Homotopy-Theoretic Models of Identity Types) was one of the first to make precise the connection between identity types and path spaces: it showed that any Quillen model category can model Martin-Löf type theory with the identity type as path objects. This is the semantic foundation for the claim that "types are homotopy types."

Awodey's ongoing work includes the semantics of HoTT in general model-categorical settings (not just the simplicial set model), the philosophical foundations of univalent mathematics, and the connection between HoTT and topos theory. He is one of the clearest expositors of the foundational ideas.

---

## Thierry Coquand — Cubical Type Theory and Computation

Coquand is a professor at the University of Gothenburg and one of the "C"s in CCHM. He is one of the founders of the Coq proof assistant and has been a central figure in constructive type theory for decades. His contribution to CCHM was the De Morgan interval structure that makes cubical type theory constructively valid — without the interval having decidable equality, the type theory would not be constructive.

His ongoing work includes canonicity and normalization for cubical type theory (his 2019 paper proves normalization for a version of cubical type theory, a key technical result), and the development of "agda-flat" (a proof assistant for modal type theory with a crisp ♭ modality).

**Connection to open problems:** Problem 2 (canonicity for Book HoTT — the cubical solution gives the contrast).

---

## Egbert Rijke — Synthetic Homotopy Theory

Rijke is a professor at the University of Ljubljana. His work is primarily in synthetic homotopy theory in the HoTT Book style: proving classical homotopy-theoretic results using the language of HoTT, without cubical computation rules. His ongoing project is a new textbook "Introduction to Homotopy Type Theory" that systematically develops synthetic homotopy theory from first principles.

Rijke's contribution to the community is both mathematical (new synthetic proofs of classical results) and pedagogical (developing the language and notation for writing synthetic homotopy theory clearly). His arXiv preprints (2022–2025) develop the theory of truncated types, connected types, and pushouts in a self-contained way.

---

## Ulrik Buchholtz — Higher Algebra and Modal HoTT

Buchholtz is a professor at the University of Nottingham. His work spans higher algebra in HoTT (H-spaces, classifying spaces, banded types) and modal type theory. His 2023 paper (with Floris van Doorn and Egbert Rijke) on central H-spaces and banded types proves new results about the homotopy structure of classifying spaces using HoTT methods — results that have no classical proof that is simpler.

Buchholtz is also one of the developers of the "homotopy type theory" community infrastructure and has contributed to the HoTT Book, the Cubical Agda library, and several foundational papers.

---

## Evan Cavallo — Cubical Type Theory Internals

Cavallo is a professor at the University of Gothenburg. His PhD thesis (CMU, 2021) developed the internal language of cubical type theory — the theory of how cubical type theory works "from the inside," including the computational rules for the Glue type and the relationship between cubical type theory and the simplicial set model.

His ongoing work includes improving the computational behavior of Cubical Agda (making the type-checker faster and more predictable) and the theoretical foundations of cubical type theory (coherence, normalization, and the relationship to other computational type theories).

---

## David Jaz Myers — Cohesion and Physical Mathematics

Myers is a postdoctoral researcher whose work develops cohesive HoTT in the direction of physics and differential geometry. His paper "Simplicial, Divisorial, and Orientable Cohesion" (2022) unifies different kinds of cohesion (smooth, algebraic, combinatorial) into a single framework and shows that many theorems proved in real-cohesive HoTT hold in this more general setting.

Myers is also developing an approach to string theory using cohesive HoTT, following the program initiated by Urs Schreiber. His ongoing work connects the physics of M-theory (strings, branes, fluxes) to the mathematics of higher gauge theory formulated in cohesive type theory.

**Connection to applications:** The M-theory formalization program (Section 94).

---

## Nikolai Kudasov — The Rzk Proof Assistant

Kudasov is the primary developer of the Rzk proof assistant — the tool that implements simplicial type theory and makes the Riehl-Shulman program machine-verifiable. His technical contributions to Rzk include the implementation of the extension types (the Rzk analogue of the Agda record type, but for simplicial shapes) and the infrastructure for the sHoTT library.

His 2024 paper with Riehl and Weinberger (CPP 2024) on the formalization of the Yoneda lemma in Rzk is the first machine-verification of a central theorem in synthetic ∞-category theory. This paper defines the current state of the art for what Rzk can do and points at the next targets.

---

## The Broader Community

The researchers above are not exhaustive. The community also includes:

- **Jonathan Weinberger** (Johns Hopkins): sHoTT formalization, cartesian fibrations in STT
- **Kris Kapulkin** (Western University): Models of HoTT, the HoTTEST seminar organizer
- **Tom de Jong** (University of Edinburgh): Domain theory in HoTT, order theory, large cardinal principles in univalent foundations
- **Martín Escardó** (University of Birmingham): Compact types, searchability, constructive mathematics in HoTT
- **Mathieu Anel** (Carnegie Mellon University): ∞-topos theory, connections between HoTT and homotopy theory
- **Eric Finster** (University of Birmingham): Higher algebra, globular type theory, the theory of operads in HoTT
- **Marc Bezem** (University of Bergen): Normalization for type theory, connections to proof theory
- **Benedikt Ahrens** (University of Birmingham): Category theory in HoTT, UniMath library

This is a community in which the people at the frontier are reachable: they post on Zulip, they give talks at HoTTEST, they respond to questions about their papers. The social distance between a student finishing this curriculum and a researcher proving theorems at the frontier is small. The intellectual distance requires work to close — but the work is specified, the tools are available, and the community is welcoming.
